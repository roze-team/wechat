use std::{collections::HashMap, sync::Arc};

use chrono::{DateTime, Duration, Utc};
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
    refresh_skew_seconds: i64,
}

impl TokenManager {
    pub fn new(store: TokenStore, refresh_skew_seconds: i64) -> Self {
        Self {
            store,
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

        let refreshed = refresh().await?;
        if refreshed.value.is_empty() {
            return Err(WechatError::Token("empty token from refresh".to_string()));
        }
        self.store.set(key.to_string(), refreshed.clone()).await;
        Ok(refreshed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
