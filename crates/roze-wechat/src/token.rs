use std::{collections::HashMap, sync::Arc};

use chrono::{DateTime, Duration, Utc};
use roze_singleflight::SingleFlightGroup;
use tokio::sync::RwLock;

use crate::error::{Result, WechatError};

#[derive(Debug, Clone)]
pub struct AccessToken {
    pub value: String,
    pub expires_at: DateTime<Utc>,
}

impl AccessToken {
    pub fn is_stale(&self, skew_seconds: i64) -> bool {
        Utc::now() + Duration::seconds(skew_seconds) >= self.expires_at
    }
}

#[derive(Debug, Clone, Default)]
pub struct TokenStore {
    inner: Arc<RwLock<HashMap<String, AccessToken>>>,
}

impl TokenStore {
    pub async fn get(&self, key: &str) -> Option<AccessToken> {
        self.inner.read().await.get(key).cloned()
    }

    pub async fn set(&self, key: impl Into<String>, token: AccessToken) {
        self.inner.write().await.insert(key.into(), token);
    }
}

#[derive(Debug, Clone)]
pub struct TokenManager {
    store: TokenStore,
    singleflight: SingleFlightGroup,
    refresh_skew_seconds: i64,
}

impl TokenManager {
    pub fn new(store: TokenStore, refresh_skew_seconds: i64) -> Self {
        Self {
            store,
            singleflight: SingleFlightGroup::new(),
            refresh_skew_seconds,
        }
    }

    pub async fn get_or_refresh<F, Fut>(&self, key: &str, refresh: F) -> Result<AccessToken>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<AccessToken>>,
    {
        if let Some(token) = self.store.get(key).await {
            if !token.is_stale(self.refresh_skew_seconds) {
                return Ok(token);
            }
        }

        let store = self.store.clone();
        let refresh_skew_seconds = self.refresh_skew_seconds;
        let key_owned = key.to_string();
        let flight_key = format!("wechat-token:{key}");
        let refreshed = self
            .singleflight
            .do_call(&flight_key, || async move {
                if let Some(token) = store.get(&key_owned).await {
                    if !token.is_stale(refresh_skew_seconds) {
                        return Ok(token);
                    }
                }

                let refreshed = refresh().await.map_err(|err| err.to_string())?;
                if refreshed.value.is_empty() {
                    return Err("empty token from refresh".to_string());
                }
                store.set(key_owned, refreshed.clone()).await;
                Ok(refreshed)
            })
            .await;
        self.singleflight.reset(&flight_key).await;
        let refreshed = refreshed.map_err(WechatError::Token)?;
        Ok(refreshed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };

    #[tokio::test]
    async fn caches_fresh_token() {
        let manager = TokenManager::new(TokenStore::default(), 300);
        let token = manager
            .get_or_refresh("oa:app", || async {
                Ok(AccessToken {
                    value: "token-a".to_string(),
                    expires_at: Utc::now() + Duration::seconds(7200),
                })
            })
            .await
            .unwrap();

        let cached = manager
            .get_or_refresh("oa:app", || async {
                Ok(AccessToken {
                    value: "token-b".to_string(),
                    expires_at: Utc::now() + Duration::seconds(7200),
                })
            })
            .await
            .unwrap();

        assert_eq!(token.value, "token-a");
        assert_eq!(cached.value, "token-a");
    }

    #[tokio::test]
    async fn coalesces_concurrent_refreshes() {
        let manager = Arc::new(TokenManager::new(TokenStore::default(), 300));
        let calls = Arc::new(AtomicUsize::new(0));

        let first = {
            let manager = manager.clone();
            let calls = calls.clone();
            tokio::spawn(async move {
                manager
                    .get_or_refresh("oa:app", || async move {
                        calls.fetch_add(1, Ordering::SeqCst);
                        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                        Ok(AccessToken {
                            value: "token-a".to_string(),
                            expires_at: Utc::now() + Duration::seconds(7200),
                        })
                    })
                    .await
                    .unwrap()
            })
        };
        let second = {
            let manager = manager.clone();
            let calls = calls.clone();
            tokio::spawn(async move {
                manager
                    .get_or_refresh("oa:app", || async move {
                        calls.fetch_add(1, Ordering::SeqCst);
                        Ok(AccessToken {
                            value: "token-b".to_string(),
                            expires_at: Utc::now() + Duration::seconds(7200),
                        })
                    })
                    .await
                    .unwrap()
            })
        };

        assert_eq!(first.await.unwrap().value, "token-a");
        assert_eq!(second.await.unwrap().value, "token-a");
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
}
