use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::{Result, WechatError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatApiEnvelope<T> {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(flatten)]
    pub data: T,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Empty {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawResponse {
    #[serde(flatten)]
    pub value: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StableAccessTokenRequest {
    pub grant_type: String,
    pub appid: String,
    pub secret: String,
    #[serde(default)]
    pub force_refresh: bool,
}

impl StableAccessTokenRequest {
    pub fn client_credential(
        appid: impl Into<String>,
        secret: impl Into<String>,
        force_refresh: bool,
    ) -> Self {
        Self {
            grant_type: "client_credential".to_string(),
            appid: appid.into(),
            secret: secret.into(),
            force_refresh,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StableAccessTokenResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub access_token: Option<String>,
    #[serde(default)]
    pub expires_in: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallbackQuery {
    pub signature: Option<String>,
    pub msg_signature: Option<String>,
    pub timestamp: String,
    pub nonce: String,
    pub echostr: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallbackMessage {
    #[serde(default, rename = "ToUserName")]
    pub to_user_name: Option<String>,
    #[serde(default, rename = "FromUserName")]
    pub from_user_name: Option<String>,
    #[serde(default, rename = "CreateTime")]
    pub create_time: Option<i64>,
    #[serde(default, rename = "MsgType")]
    pub msg_type: Option<String>,
    #[serde(default, rename = "Event")]
    pub event: Option<String>,
    #[serde(default, rename = "EventKey")]
    pub event_key: Option<String>,
    #[serde(default, rename = "MsgId")]
    pub msg_id: Option<i64>,
    #[serde(default, rename = "AgentID")]
    pub agent_id: Option<i64>,
    #[serde(default, rename = "Content")]
    pub content: Option<String>,
    #[serde(default, rename = "PicUrl")]
    pub pic_url: Option<String>,
    #[serde(default, rename = "MediaId")]
    pub media_id: Option<String>,
    #[serde(default, rename = "Format")]
    pub format: Option<String>,
    #[serde(default, rename = "Recognition")]
    pub recognition: Option<String>,
    #[serde(default, rename = "ThumbMediaId")]
    pub thumb_media_id: Option<String>,
    #[serde(default, rename = "Location_X")]
    pub location_x: Option<f64>,
    #[serde(default, rename = "Location_Y")]
    pub location_y: Option<f64>,
    #[serde(default, rename = "Scale")]
    pub scale: Option<i64>,
    #[serde(default, rename = "Label")]
    pub label: Option<String>,
    #[serde(default, rename = "AppType")]
    pub app_type: Option<String>,
    #[serde(default, rename = "Title")]
    pub title: Option<String>,
    #[serde(default, rename = "Description")]
    pub description: Option<String>,
    #[serde(default, rename = "Url")]
    pub url: Option<String>,
    #[serde(default, rename = "Encrypt")]
    pub encrypt: Option<String>,
    #[serde(default, rename = "AppId")]
    pub app_id: Option<String>,
    #[serde(default, rename = "InfoType")]
    pub info_type: Option<String>,
    #[serde(default, rename = "ComponentVerifyTicket")]
    pub component_verify_ticket: Option<String>,
    #[serde(default, rename = "AuthorizerAppid")]
    pub authorizer_appid: Option<String>,
    #[serde(default, rename = "AuthorizationCode")]
    pub authorization_code: Option<String>,
    #[serde(default, rename = "JobId")]
    pub job_id: Option<String>,
    #[serde(default, rename = "SuiteId")]
    pub suite_id: Option<String>,
    #[serde(default, rename = "SuiteTicket")]
    pub suite_ticket: Option<String>,
    #[serde(default, rename = "AuthCorpId")]
    pub auth_corp_id: Option<String>,
    #[serde(default, rename = "appid")]
    pub appid: Option<String>,
    #[serde(default, rename = "trace_id")]
    pub trace_id: Option<String>,
    #[serde(default, rename = "version")]
    pub version: Option<i64>,
    #[serde(default, rename = "detail")]
    pub detail: Vec<MediaCheckDetail>,
    #[serde(default, rename = "errCode")]
    pub err_code: Option<i64>,
    #[serde(default, rename = "errMsg")]
    pub err_msg: Option<String>,
    #[serde(default, rename = "result")]
    pub result: Option<MediaCheckResult>,
}

impl CallbackMessage {
    pub fn parse_xml(xml: &str) -> Result<Self> {
        quick_xml::de::from_str(xml)
            .map_err(|err| WechatError::Xml(format!("invalid callback xml: {err}")))
    }

    pub fn is_encrypted(&self) -> bool {
        self.encrypt
            .as_deref()
            .is_some_and(|value| !value.is_empty())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaCheckDetail {
    #[serde(default)]
    pub strategy: Option<String>,
    #[serde(default, rename = "errCode")]
    pub err_code: Option<i64>,
    #[serde(default)]
    pub suggest: Option<String>,
    #[serde(default)]
    pub label: Option<i64>,
    #[serde(default)]
    pub prob: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaCheckResult {
    #[serde(default)]
    pub suggest: Option<String>,
    #[serde(default)]
    pub label: Option<i64>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{CallbackMessage, StableAccessTokenRequest, StableAccessTokenResponse};

    #[test]
    fn serializes_stable_access_token_request() {
        let value = serde_json::to_value(StableAccessTokenRequest::client_credential(
            "appid", "secret", true,
        ))
        .unwrap();

        assert_eq!(
            value,
            json!({
                "grant_type": "client_credential",
                "appid": "appid",
                "secret": "secret",
                "force_refresh": true
            })
        );
    }

    #[test]
    fn deserializes_stable_access_token_response() {
        let response: StableAccessTokenResponse =
            serde_json::from_value(json!({ "access_token": "token", "expires_in": 7200 })).unwrap();

        assert_eq!(response.access_token.as_deref(), Some("token"));
        assert_eq!(response.expires_in, Some(7200));
    }

    #[test]
    fn parses_text_callback_message_xml() {
        let message = CallbackMessage::parse_xml(
            r#"<xml>
                <ToUserName><![CDATA[to]]></ToUserName>
                <FromUserName><![CDATA[from]]></FromUserName>
                <CreateTime>1710000000</CreateTime>
                <MsgType><![CDATA[text]]></MsgType>
                <Content><![CDATA[hello]]></Content>
                <MsgId>123</MsgId>
            </xml>"#,
        )
        .unwrap();

        assert_eq!(message.to_user_name.as_deref(), Some("to"));
        assert_eq!(message.from_user_name.as_deref(), Some("from"));
        assert_eq!(message.msg_type.as_deref(), Some("text"));
        assert_eq!(message.content.as_deref(), Some("hello"));
        assert_eq!(message.msg_id, Some(123));
    }

    #[test]
    fn parses_event_callback_message_xml() {
        let message = CallbackMessage::parse_xml(
            r#"<xml>
                <ToUserName><![CDATA[to]]></ToUserName>
                <FromUserName><![CDATA[from]]></FromUserName>
                <CreateTime>1710000000</CreateTime>
                <MsgType><![CDATA[event]]></MsgType>
                <Event><![CDATA[CLICK]]></Event>
                <EventKey><![CDATA[MENU_KEY]]></EventKey>
                <Encrypt><![CDATA[ciphertext]]></Encrypt>
            </xml>"#,
        )
        .unwrap();

        assert_eq!(message.msg_type.as_deref(), Some("event"));
        assert_eq!(message.event.as_deref(), Some("CLICK"));
        assert_eq!(message.event_key.as_deref(), Some("MENU_KEY"));
        assert!(message.is_encrypted());
    }

    #[test]
    fn parses_work_media_upload_job_finish_callback_xml() {
        let message = CallbackMessage::parse_xml(
            r#"<xml>
                <ToUserName><![CDATA[corp-id]]></ToUserName>
                <FromUserName><![CDATA[sys]]></FromUserName>
                <CreateTime>1710000000</CreateTime>
                <MsgType><![CDATA[event]]></MsgType>
                <Event><![CDATA[upload_media_job_finish]]></Event>
                <JobId><![CDATA[job-id]]></JobId>
            </xml>"#,
        )
        .unwrap();

        assert_eq!(message.from_user_name.as_deref(), Some("sys"));
        assert_eq!(message.event.as_deref(), Some("upload_media_job_finish"));
        assert_eq!(message.job_id.as_deref(), Some("job-id"));
    }

    #[test]
    fn parses_mini_program_server_message_xml() {
        let image = CallbackMessage::parse_xml(
            r#"<xml>
                <ToUserName><![CDATA[to]]></ToUserName>
                <FromUserName><![CDATA[from]]></FromUserName>
                <CreateTime>1710000000</CreateTime>
                <MsgType><![CDATA[image]]></MsgType>
                <PicUrl><![CDATA[https://example.com/a.jpg]]></PicUrl>
                <MediaId><![CDATA[media-id]]></MediaId>
                <MsgId>123</MsgId>
                <AgentID>42</AgentID>
            </xml>"#,
        )
        .unwrap();
        assert_eq!(image.msg_type.as_deref(), Some("image"));
        assert_eq!(image.pic_url.as_deref(), Some("https://example.com/a.jpg"));
        assert_eq!(image.media_id.as_deref(), Some("media-id"));
        assert_eq!(image.agent_id, Some(42));

        let location = CallbackMessage::parse_xml(
            r#"<xml>
                <ToUserName><![CDATA[to]]></ToUserName>
                <FromUserName><![CDATA[from]]></FromUserName>
                <CreateTime>1710000000</CreateTime>
                <MsgType><![CDATA[location]]></MsgType>
                <Location_X>23.134521</Location_X>
                <Location_Y>113.358803</Location_Y>
                <Scale>20</Scale>
                <Label><![CDATA[label]]></Label>
                <AppType><![CDATA[wxamini]]></AppType>
                <MsgId>456</MsgId>
            </xml>"#,
        )
        .unwrap();
        assert_eq!(location.location_x, Some(23.134521));
        assert_eq!(location.location_y, Some(113.358803));
        assert_eq!(location.scale, Some(20));
        assert_eq!(location.app_type.as_deref(), Some("wxamini"));

        let link = CallbackMessage::parse_xml(
            r#"<xml>
                <ToUserName><![CDATA[to]]></ToUserName>
                <FromUserName><![CDATA[from]]></FromUserName>
                <CreateTime>1710000000</CreateTime>
                <MsgType><![CDATA[link]]></MsgType>
                <Title><![CDATA[title]]></Title>
                <Description><![CDATA[desc]]></Description>
                <Url><![CDATA[https://example.com]]></Url>
                <PicUrl><![CDATA[https://example.com/pic.jpg]]></PicUrl>
            </xml>"#,
        )
        .unwrap();
        assert_eq!(link.title.as_deref(), Some("title"));
        assert_eq!(link.description.as_deref(), Some("desc"));
        assert_eq!(link.url.as_deref(), Some("https://example.com"));
    }

    #[test]
    fn parses_mini_program_media_check_async_xml() {
        let message = CallbackMessage::parse_xml(
            r#"<xml>
                <ToUserName><![CDATA[to]]></ToUserName>
                <FromUserName><![CDATA[from]]></FromUserName>
                <CreateTime>1710000000</CreateTime>
                <MsgType><![CDATA[event]]></MsgType>
                <Event><![CDATA[wxa_media_check]]></Event>
                <appid><![CDATA[wxappid]]></appid>
                <trace_id><![CDATA[trace-id]]></trace_id>
                <version>2</version>
                <detail>
                    <strategy><![CDATA[content_model]]></strategy>
                    <errCode>0</errCode>
                    <suggest><![CDATA[pass]]></suggest>
                    <label>100</label>
                    <prob>90</prob>
                </detail>
                <errCode>0</errCode>
                <errMsg><![CDATA[ok]]></errMsg>
                <result>
                    <suggest><![CDATA[pass]]></suggest>
                    <label>100</label>
                </result>
            </xml>"#,
        )
        .unwrap();

        assert_eq!(message.event.as_deref(), Some("wxa_media_check"));
        assert_eq!(message.appid.as_deref(), Some("wxappid"));
        assert_eq!(message.trace_id.as_deref(), Some("trace-id"));
        assert_eq!(message.version, Some(2));
        assert_eq!(message.err_code, Some(0));
        assert_eq!(message.detail[0].strategy.as_deref(), Some("content_model"));
        assert_eq!(message.detail[0].prob, Some(90));
        assert_eq!(
            message.result.expect("result").suggest.as_deref(),
            Some("pass")
        );
    }

    #[test]
    fn parses_open_platform_ticket_callback_xml() {
        let message = CallbackMessage::parse_xml(
            r#"<xml>
                <AppId><![CDATA[component-appid]]></AppId>
                <CreateTime>1710000000</CreateTime>
                <InfoType><![CDATA[component_verify_ticket]]></InfoType>
                <ComponentVerifyTicket><![CDATA[ticket]]></ComponentVerifyTicket>
            </xml>"#,
        )
        .unwrap();

        assert_eq!(
            message.info_type.as_deref(),
            Some("component_verify_ticket")
        );
        assert_eq!(message.app_id.as_deref(), Some("component-appid"));
        assert_eq!(message.component_verify_ticket.as_deref(), Some("ticket"));
    }
}
