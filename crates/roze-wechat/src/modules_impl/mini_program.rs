use bytes::Bytes;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    config::Platform,
    crypto,
    error::Result,
    modules::{DomainModule, PlatformClient},
    types::{StableAccessTokenRequest, StableAccessTokenResponse},
    Client,
};

#[derive(Debug, Clone)]
pub struct MiniProgram {
    inner: PlatformClient,
}

impl MiniProgram {
    pub fn new(client: Client, platform: Platform) -> Self {
        Self {
            inner: PlatformClient::new(client, platform),
        }
    }

    pub fn auth(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "mini_program.auth")
    }

    pub async fn code2_session(
        &self,
        request: Code2SessionRequest,
    ) -> Result<Code2SessionResponse> {
        self.inner
            .get_with_query(
                "sns/jscode2session",
                None,
                vec![
                    ("appid".to_string(), request.app_id),
                    ("secret".to_string(), request.secret),
                    ("js_code".to_string(), request.js_code),
                    ("grant_type".to_string(), "authorization_code".to_string()),
                ],
            )
            .await
    }

    pub async fn check_session(
        &self,
        app_id: impl Into<String>,
        open_id: impl Into<String>,
        session_key: impl AsRef<[u8]>,
    ) -> Result<WechatStatusResponse> {
        let signature = crypto::hmac_sha256_hex(session_key.as_ref(), b"")?;
        self.inner
            .get_with_query(
                "wxa/checksession",
                None,
                vec![
                    ("appid".to_string(), app_id.into()),
                    ("openid".to_string(), open_id.into()),
                    ("signature".to_string(), signature),
                    ("sig_method".to_string(), "hmac_sha256".to_string()),
                ],
            )
            .await
    }

    pub async fn reset_user_session_key(
        &self,
        app_id: impl Into<String>,
        open_id: impl Into<String>,
        session_key: impl AsRef<[u8]>,
    ) -> Result<WechatStatusResponse> {
        let signature = crypto::hmac_sha256_hex(session_key.as_ref(), b"")?;
        self.inner
            .get_with_query(
                "wxa/resetusersessionkey",
                None,
                vec![
                    ("appid".to_string(), app_id.into()),
                    ("openid".to_string(), open_id.into()),
                    ("signature".to_string(), signature),
                    ("sig_method".to_string(), "hmac_sha256".to_string()),
                ],
            )
            .await
    }

    pub fn base(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "mini_program.base")
    }

    pub async fn stable_access_token(
        &self,
        request: StableAccessTokenRequest,
    ) -> Result<StableAccessTokenResponse> {
        self.inner.post("cgi-bin/stable_token", None, request).await
    }

    pub fn customer_service_message(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "mini_program.customer_service_message")
    }

    pub async fn send_customer_service_message(
        &self,
        access_token: impl Into<String>,
        request: CustomerServiceMessage,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/message/custom/send",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn set_customer_typing(
        &self,
        access_token: impl Into<String>,
        openid: impl Into<String>,
        typing: bool,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/message/custom/typing",
                Some(access_token.into()),
                json!({
                    "touser": openid.into(),
                    "command": if typing { "Typing" } else { "CancelTyping" },
                }),
            )
            .await
    }

    pub fn data_cube(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "mini_program.data_cube")
    }

    pub async fn daily_visit_trend(
        &self,
        access_token: impl Into<String>,
        request: DataCubeDateRange,
    ) -> Result<Value> {
        self.inner
            .post(
                "datacube/getweanalysisappiddailyvisittrend",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn weekly_visit_trend(
        &self,
        access_token: impl Into<String>,
        request: DataCubeDateRange,
    ) -> Result<Value> {
        self.inner
            .post(
                "datacube/getweanalysisappidweeklyvisittrend",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn monthly_visit_trend(
        &self,
        access_token: impl Into<String>,
        request: DataCubeDateRange,
    ) -> Result<Value> {
        self.inner
            .post(
                "datacube/getweanalysisappidmonthlyvisittrend",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn daily_retain_info(
        &self,
        access_token: impl Into<String>,
        request: DataCubeDateRange,
    ) -> Result<Value> {
        self.inner
            .post(
                "datacube/getweanalysisappiddailyretaininfo",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn visit_page(
        &self,
        access_token: impl Into<String>,
        request: DataCubeDateRange,
    ) -> Result<Value> {
        self.inner
            .post(
                "datacube/getweanalysisappidvisitpage",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn user_portrait(
        &self,
        access_token: impl Into<String>,
        request: DataCubeDateRange,
    ) -> Result<Value> {
        self.inner
            .post(
                "datacube/getweanalysisappiduserportrait",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub fn live_broadcast(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "mini_program.live_broadcast")
    }

    pub async fn create_live_room(
        &self,
        access_token: impl Into<String>,
        request: LiveRoomRequest,
    ) -> Result<Value> {
        self.inner
            .post(
                "wxaapi/broadcast/room/create",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn live_info(
        &self,
        access_token: impl Into<String>,
        request: LiveInfoRequest,
    ) -> Result<Value> {
        self.inner
            .post(
                "wxa/business/getliveinfo",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn live_replay(
        &self,
        access_token: impl Into<String>,
        room_id: i64,
        start: i64,
        limit: i64,
    ) -> Result<Value> {
        self.inner
            .post(
                "wxa/business/getliveinfo",
                Some(access_token.into()),
                json!({
                    "action": "get_replay",
                    "room_id": room_id,
                    "start": start,
                    "limit": limit,
                }),
            )
            .await
    }

    pub async fn delete_live_room(
        &self,
        access_token: impl Into<String>,
        room_id: i64,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "wxaapi/broadcast/room/deleteroom",
                Some(access_token.into()),
                json!({ "id": room_id }),
            )
            .await
    }

    pub fn phone_number(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "mini_program.phone_number")
    }

    pub async fn get_user_phone_number(
        &self,
        access_token: impl Into<String>,
        code: impl Into<String>,
    ) -> Result<PhoneNumberResponse> {
        self.inner
            .post(
                "wxa/business/getuserphonenumber",
                Some(access_token.into()),
                json!({ "code": code.into() }),
            )
            .await
    }

    pub fn ocr(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "mini_program.ocr")
    }

    pub async fn ocr_bankcard_by_url(
        &self,
        access_token: impl Into<String>,
        img_url: impl Into<String>,
    ) -> Result<OcrBankcardResponse> {
        self.ocr_by_url("cv/ocr/bankcard", access_token, img_url, Vec::new())
            .await
    }

    pub async fn ocr_bankcard_from_bytes(
        &self,
        access_token: impl Into<String>,
        file_name: impl Into<String>,
        data: Vec<u8>,
    ) -> Result<OcrBankcardResponse> {
        self.ocr_from_bytes("cv/ocr/bankcard", access_token, file_name, data, Vec::new())
            .await
    }

    pub async fn ocr_business_license_by_url(
        &self,
        access_token: impl Into<String>,
        img_url: impl Into<String>,
    ) -> Result<OcrBusinessLicenseResponse> {
        self.ocr_by_url("cv/ocr/bizlicense", access_token, img_url, Vec::new())
            .await
    }

    pub async fn ocr_business_license_from_bytes(
        &self,
        access_token: impl Into<String>,
        file_name: impl Into<String>,
        data: Vec<u8>,
    ) -> Result<OcrBusinessLicenseResponse> {
        self.ocr_from_bytes(
            "cv/ocr/bizlicense",
            access_token,
            file_name,
            data,
            Vec::new(),
        )
        .await
    }

    pub async fn ocr_driving_license_by_url(
        &self,
        access_token: impl Into<String>,
        img_url: impl Into<String>,
    ) -> Result<OcrDrivingLicenseResponse> {
        self.ocr_by_url("cv/ocr/drivinglicense", access_token, img_url, Vec::new())
            .await
    }

    pub async fn ocr_driving_license_from_bytes(
        &self,
        access_token: impl Into<String>,
        file_name: impl Into<String>,
        data: Vec<u8>,
    ) -> Result<OcrDrivingLicenseResponse> {
        self.ocr_from_bytes(
            "cv/ocr/drivinglicense",
            access_token,
            file_name,
            data,
            Vec::new(),
        )
        .await
    }

    pub async fn ocr_id_card_by_url(
        &self,
        access_token: impl Into<String>,
        img_url: impl Into<String>,
    ) -> Result<OcrIdCardResponse> {
        self.ocr_by_url("cv/ocr/idcard", access_token, img_url, Vec::new())
            .await
    }

    pub async fn ocr_id_card_from_bytes(
        &self,
        access_token: impl Into<String>,
        file_name: impl Into<String>,
        data: Vec<u8>,
    ) -> Result<OcrIdCardResponse> {
        self.ocr_from_bytes("cv/ocr/idcard", access_token, file_name, data, Vec::new())
            .await
    }

    pub async fn ocr_printed_text_by_url(
        &self,
        access_token: impl Into<String>,
        img_url: impl Into<String>,
    ) -> Result<OcrPrintedTextResponse> {
        self.ocr_by_url("cv/ocr/comm", access_token, img_url, Vec::new())
            .await
    }

    pub async fn ocr_printed_text_from_bytes(
        &self,
        access_token: impl Into<String>,
        file_name: impl Into<String>,
        data: Vec<u8>,
    ) -> Result<OcrPrintedTextResponse> {
        self.ocr_from_bytes("cv/ocr/comm", access_token, file_name, data, Vec::new())
            .await
    }

    pub async fn ocr_vehicle_license_by_url(
        &self,
        access_token: impl Into<String>,
        mode: impl Into<String>,
        img_url: impl Into<String>,
    ) -> Result<OcrVehicleLicenseResponse> {
        self.ocr_by_url(
            "cv/ocr/driving",
            access_token,
            img_url,
            vec![("type".to_string(), mode.into())],
        )
        .await
    }

    pub async fn ocr_vehicle_license_from_bytes(
        &self,
        access_token: impl Into<String>,
        mode: impl Into<String>,
        file_name: impl Into<String>,
        data: Vec<u8>,
    ) -> Result<OcrVehicleLicenseResponse> {
        self.ocr_from_bytes(
            "cv/ocr/driving",
            access_token,
            file_name,
            data,
            vec![("type".to_string(), mode.into())],
        )
        .await
    }

    async fn ocr_by_url<R>(
        &self,
        path: &'static str,
        access_token: impl Into<String>,
        img_url: impl Into<String>,
        mut query: Vec<(String, String)>,
    ) -> Result<R>
    where
        R: for<'de> Deserialize<'de>,
    {
        query.push(("img_url".to_string(), img_url.into()));
        self.inner
            .post_multipart(
                path,
                Some(access_token.into()),
                query,
                reqwest::multipart::Form::new(),
            )
            .await
    }

    async fn ocr_from_bytes<R>(
        &self,
        path: &'static str,
        access_token: impl Into<String>,
        file_name: impl Into<String>,
        data: Vec<u8>,
        query: Vec<(String, String)>,
    ) -> Result<R>
    where
        R: for<'de> Deserialize<'de>,
    {
        let form = reqwest::multipart::Form::new().part(
            "img",
            reqwest::multipart::Part::bytes(data).file_name(file_name.into()),
        );
        self.inner
            .post_multipart(path, Some(access_token.into()), query, form)
            .await
    }

    pub fn security(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "mini_program.security")
    }

    pub async fn security_msg_sec_check(
        &self,
        access_token: impl Into<String>,
        request: SecurityMsgSecCheckRequest,
    ) -> Result<Value> {
        self.inner
            .post("wxa/msg_sec_check", Some(access_token.into()), request)
            .await
    }

    pub async fn security_media_check_async(
        &self,
        access_token: impl Into<String>,
        request: SecurityMediaCheckAsyncRequest,
    ) -> Result<Value> {
        self.inner
            .post("wxa/media_check_async", Some(access_token.into()), request)
            .await
    }

    pub fn risk_control(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "mini_program.risk_control")
    }

    pub async fn get_user_risk_rank(
        &self,
        access_token: impl Into<String>,
        request: RiskControlGetUserRiskRankRequest,
    ) -> Result<RiskControlGetUserRiskRankResponse> {
        self.inner
            .post("wxa/getuserriskrank", Some(access_token.into()), request)
            .await
    }

    pub fn wxa_sec_order(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "mini_program.wxa_sec_order")
    }

    pub async fn upload_shipping_info(
        &self,
        access_token: impl Into<String>,
        request: WxaSecUploadShippingInfoRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "wxa/sec/order/upload_shipping_info",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn upload_combined_shipping_info(
        &self,
        access_token: impl Into<String>,
        request: WxaSecUploadCombinedShippingInfoRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "wxa/sec/order/upload_combined_shipping_info",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_wxa_sec_order(
        &self,
        access_token: impl Into<String>,
        request: WxaSecOrderQuery,
    ) -> Result<WxaSecOrderResponse> {
        self.inner
            .post(
                "wxa/sec/order/get_order",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_wxa_sec_order_list(
        &self,
        access_token: impl Into<String>,
        request: WxaSecOrderListRequest,
    ) -> Result<WxaSecOrderListResponse> {
        self.inner
            .post(
                "wxa/sec/order/get_order_list",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn notify_wxa_sec_confirm_receive(
        &self,
        access_token: impl Into<String>,
        request: WxaSecConfirmReceiveRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "wxa/sec/order/notify_confirm_receive",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn set_wxa_sec_msg_jump_path(
        &self,
        access_token: impl Into<String>,
        path: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "wxa/sec/order/set_msg_jump_path",
                Some(access_token.into()),
                json!({ "path": path.into() }),
            )
            .await
    }

    pub async fn is_wxa_sec_trade_managed(
        &self,
        access_token: impl Into<String>,
        app_id: impl Into<String>,
    ) -> Result<WxaSecTradeManagedResponse> {
        self.inner
            .post(
                "wxa/sec/order/is_trade_managed",
                Some(access_token.into()),
                json!({ "appid": app_id.into() }),
            )
            .await
    }

    pub async fn is_wxa_sec_trade_management_confirmation_completed(
        &self,
        access_token: impl Into<String>,
        app_id: impl Into<String>,
    ) -> Result<WxaSecTradeManagementConfirmationResponse> {
        self.inner
            .post(
                "wxa/sec/order/is_trade_management_confirmation_completed",
                Some(access_token.into()),
                json!({ "appid": app_id.into() }),
            )
            .await
    }

    pub async fn operate_wxa_sec_special_order(
        &self,
        access_token: impl Into<String>,
        request: WxaSecSpecialOrderRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "wxa/sec/order/opspecialorder",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub fn messages(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "mini_program.messages")
    }

    pub async fn send_subscribe_message(
        &self,
        access_token: impl Into<String>,
        request: SubscribeMessageRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/message/subscribe/send",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub fn url(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "mini_program.url")
    }

    pub async fn generate_url_scheme(
        &self,
        access_token: impl Into<String>,
        request: UrlSchemeGenerateRequest,
    ) -> Result<UrlSchemeResponse> {
        self.inner
            .post("wxa/generatescheme", Some(access_token.into()), request)
            .await
    }

    pub async fn generate_url_link(
        &self,
        access_token: impl Into<String>,
        request: UrlLinkGenerateRequest,
    ) -> Result<UrlLinkResponse> {
        self.inner
            .post("wxa/generate_urllink", Some(access_token.into()), request)
            .await
    }

    pub async fn generate_short_link(
        &self,
        access_token: impl Into<String>,
        page_url: impl Into<String>,
        page_title: impl Into<String>,
        is_permanent: bool,
    ) -> Result<ShortLinkResponse> {
        self.inner
            .post(
                "wxa/genwxashortlink",
                Some(access_token.into()),
                json!({
                    "page_url": page_url.into(),
                    "page_title": page_title.into(),
                    "is_permanent": is_permanent,
                }),
            )
            .await
    }

    pub fn wxa_code(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "mini_program.wxa_code")
    }

    pub async fn create_qr_code(
        &self,
        access_token: impl Into<String>,
        request: CreateQrCodeRequest,
    ) -> Result<Value> {
        self.inner
            .post(
                "cgi-bin/wxaapp/createwxaqrcode",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn create_qr_code_bytes(
        &self,
        access_token: impl Into<String>,
        request: CreateQrCodeRequest,
    ) -> Result<Bytes> {
        self.inner
            .post_json_bytes(
                "cgi-bin/wxaapp/createwxaqrcode",
                Some(access_token.into()),
                serde_json::to_value(request)?,
            )
            .await
    }

    pub async fn get_wxa_code(
        &self,
        access_token: impl Into<String>,
        request: GetWxaCodeRequest,
    ) -> Result<Value> {
        self.inner
            .post("wxa/getwxacode", Some(access_token.into()), request)
            .await
    }

    pub async fn get_wxa_code_bytes(
        &self,
        access_token: impl Into<String>,
        request: GetWxaCodeRequest,
    ) -> Result<Bytes> {
        self.inner
            .post_json_bytes(
                "wxa/getwxacode",
                Some(access_token.into()),
                serde_json::to_value(request)?,
            )
            .await
    }

    pub async fn get_unlimited_wxa_code(
        &self,
        access_token: impl Into<String>,
        request: GetUnlimitedWxaCodeRequest,
    ) -> Result<Value> {
        self.inner
            .post("wxa/getwxacodeunlimit", Some(access_token.into()), request)
            .await
    }

    pub async fn get_unlimited_wxa_code_bytes(
        &self,
        access_token: impl Into<String>,
        request: GetUnlimitedWxaCodeRequest,
    ) -> Result<Bytes> {
        self.inner
            .post_json_bytes(
                "wxa/getwxacodeunlimit",
                Some(access_token.into()),
                serde_json::to_value(request)?,
            )
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Code2SessionRequest {
    pub app_id: String,
    pub secret: String,
    pub js_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Code2SessionResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub openid: Option<String>,
    #[serde(default)]
    pub session_key: Option<String>,
    #[serde(default)]
    pub unionid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatStatusResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhoneNumberResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub phone_info: Option<PhoneInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhoneInfo {
    pub phone_number: String,
    pub pure_phone_number: String,
    pub country_code: String,
    #[serde(default)]
    pub watermark: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrBankcardResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub number: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrBusinessLicenseResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub reg_num: Option<String>,
    #[serde(default)]
    pub serial: Option<String>,
    #[serde(default)]
    pub legal_representative: Option<String>,
    #[serde(default)]
    pub enterprise_name: Option<String>,
    #[serde(default)]
    pub type_of_organization: Option<String>,
    #[serde(default)]
    pub address: Option<String>,
    #[serde(default)]
    pub type_of_enterprise: Option<String>,
    #[serde(default)]
    pub business_scope: Option<String>,
    #[serde(default)]
    pub registered_capital: Option<String>,
    #[serde(default)]
    pub paid_in_capital: Option<String>,
    #[serde(default)]
    pub valid_period: Option<String>,
    #[serde(default)]
    pub registered_date: Option<String>,
    #[serde(default)]
    pub cert_position: Option<Value>,
    #[serde(default)]
    pub img_size: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrDrivingLicenseResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub id_num: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub sex: Option<String>,
    #[serde(default)]
    pub nationality: Option<String>,
    #[serde(default)]
    pub address: Option<String>,
    #[serde(default)]
    pub birth_date: Option<String>,
    #[serde(default)]
    pub issue_date: Option<String>,
    #[serde(default)]
    pub car_class: Option<String>,
    #[serde(default)]
    pub valid_from: Option<String>,
    #[serde(default)]
    pub valid_to: Option<String>,
    #[serde(default)]
    pub official_seal: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrIdCardResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub addr: Option<String>,
    #[serde(default)]
    pub gender: Option<String>,
    #[serde(default)]
    pub nationality: Option<String>,
    #[serde(default)]
    pub valid_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrPrintedTextResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub items: Vec<Value>,
    #[serde(default)]
    pub img_size: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrVehicleLicenseResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub vhicle_type: Option<String>,
    #[serde(default)]
    pub owner: Option<String>,
    #[serde(default)]
    pub addr: Option<String>,
    #[serde(default)]
    pub use_character: Option<String>,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub vin: Option<String>,
    #[serde(default)]
    pub engine_num: Option<String>,
    #[serde(default)]
    pub register_date: Option<String>,
    #[serde(default)]
    pub issue_date: Option<String>,
    #[serde(default)]
    pub plate_num_b: Option<String>,
    #[serde(default)]
    pub record: Option<String>,
    #[serde(default)]
    pub passengers_num: Option<String>,
    #[serde(default)]
    pub total_quality: Option<String>,
    #[serde(default)]
    pub prepare_quality: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerServiceMessage {
    pub touser: String,
    pub msgtype: String,
    #[serde(flatten)]
    pub payload: Value,
}

impl CustomerServiceMessage {
    pub fn text(touser: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            touser: touser.into(),
            msgtype: "text".to_string(),
            payload: json!({ "text": { "content": content.into() } }),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataCubeDateRange {
    pub begin_date: String,
    pub end_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMsgSecCheckRequest {
    pub content: String,
    #[serde(default = "default_security_scene")]
    pub scene: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openid: Option<String>,
}

fn default_security_scene() -> i64 {
    1
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMediaCheckAsyncRequest {
    pub media_url: String,
    pub media_type: i64,
    #[serde(default = "default_security_scene")]
    pub scene: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskControlGetUserRiskRankRequest {
    pub appid: String,
    pub openid: String,
    pub scene: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_no: Option<String>,
    pub bank_card_no: String,
    pub cert_no: String,
    pub client_ip: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_info: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskControlGetUserRiskRankResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub risk_rank: Option<i64>,
    #[serde(default)]
    pub unoin_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WxaSecOrderKey {
    pub order_number_type: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "mchid")]
    pub mch_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_trade_no: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WxaSecPayer {
    pub openid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WxaSecShippingContact {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consignor_contact: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver_contact: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WxaSecShippingInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub express_company: Option<String>,
    pub item_desc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<WxaSecShippingContact>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WxaSecUploadShippingInfoRequest {
    pub order_key: WxaSecOrderKey,
    pub logistics_type: i64,
    pub delivery_mode: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_all_delivered: Option<bool>,
    pub shipping_list: Vec<WxaSecShippingInfo>,
    pub upload_time: String,
    pub payer: WxaSecPayer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WxaSecSubOrderShippingInfo {
    pub order_key: WxaSecOrderKey,
    pub logistics_type: i64,
    pub delivery_mode: i64,
    pub is_all_delivered: bool,
    pub shipping_list: Vec<WxaSecShippingInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WxaSecUploadCombinedShippingInfoRequest {
    pub order_key: WxaSecOrderKey,
    pub sub_orders: Vec<WxaSecSubOrderShippingInfo>,
    pub upload_time: String,
    pub payer: WxaSecPayer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WxaSecOrderQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_merchant_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_trade_no: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WxaSecOrderListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_time_range: Option<WxaSecPayTimeRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_state: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_index: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WxaSecPayTimeRange {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WxaSecConfirmReceiveRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_merchant_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_trade_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WxaSecSpecialOrderRequest {
    pub order_id: String,
    #[serde(rename = "type")]
    pub operation_type: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay_to: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WxaSecOrderResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub order: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WxaSecOrderListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub last_index: Option<String>,
    #[serde(default)]
    pub has_more: Option<bool>,
    #[serde(default)]
    pub order_list: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WxaSecTradeManagedResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub is_trade_managed: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WxaSecTradeManagementConfirmationResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub completed: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeMessageRequest {
    pub touser: String,
    pub template_id: String,
    pub data: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miniprogram_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveRoomRequest {
    pub name: String,
    #[serde(rename = "coverImg")]
    pub cover_img: String,
    #[serde(rename = "startTime")]
    pub start_time: i64,
    #[serde(rename = "endTime")]
    pub end_time: i64,
    #[serde(rename = "anchorName")]
    pub anchor_name: String,
    #[serde(rename = "anchorWechat")]
    pub anchor_wechat: String,
    #[serde(rename = "shareImg")]
    pub share_img: String,
    #[serde(rename = "type")]
    pub type_id: i64,
    #[serde(rename = "screenType")]
    pub screen_type: i64,
    #[serde(rename = "closeLike")]
    pub close_like: i64,
    #[serde(rename = "closeGoods")]
    pub close_goods: i64,
    #[serde(rename = "closeComment")]
    pub close_comment: i64,
    #[serde(rename = "feedsImg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feeds_img: Option<String>,
    #[serde(rename = "closeReplay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_replay: Option<i64>,
    #[serde(rename = "closeShare")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_share: Option<i64>,
    #[serde(rename = "closeKf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_kf: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveInfoRequest {
    pub start: i64,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlSchemeGenerateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jump_wxa: Option<JumpWxa>,
    #[serde(default)]
    pub is_expire: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_type: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_interval: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlLinkGenerateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_version: Option<String>,
    #[serde(default)]
    pub is_expire: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_type: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_interval: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_base: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpWxa {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlSchemeResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub openlink: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlLinkResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub url_link: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortLinkResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub link: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateQrCodeRequest {
    pub path: String,
    #[serde(default = "default_wxa_code_width")]
    pub width: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWxaCodeRequest {
    pub path: String,
    #[serde(default = "default_wxa_code_width")]
    pub width: i64,
    #[serde(default)]
    pub auto_color: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_color: Option<Value>,
    #[serde(default)]
    pub is_hyaline: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUnlimitedWxaCodeRequest {
    pub scene: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    #[serde(default)]
    pub check_path: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_version: Option<String>,
    #[serde(default = "default_wxa_code_width")]
    pub width: i64,
    #[serde(default)]
    pub auto_color: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_color: Option<Value>,
    #[serde(default)]
    pub is_hyaline: bool,
}

fn default_wxa_code_width() -> i64 {
    430
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{
        Code2SessionResponse, CreateQrCodeRequest, CustomerServiceMessage, DataCubeDateRange,
        JumpWxa, LiveInfoRequest, LiveRoomRequest, OcrBankcardResponse, OcrBusinessLicenseResponse,
        OcrDrivingLicenseResponse, OcrIdCardResponse, OcrPrintedTextResponse,
        OcrVehicleLicenseResponse, PhoneNumberResponse, RiskControlGetUserRiskRankRequest,
        RiskControlGetUserRiskRankResponse, SecurityMsgSecCheckRequest, SubscribeMessageRequest,
        UrlSchemeGenerateRequest, WxaSecConfirmReceiveRequest, WxaSecOrderKey,
        WxaSecOrderListRequest, WxaSecOrderListResponse, WxaSecOrderQuery, WxaSecOrderResponse,
        WxaSecPayTimeRange, WxaSecPayer, WxaSecShippingContact, WxaSecShippingInfo,
        WxaSecSpecialOrderRequest, WxaSecSubOrderShippingInfo, WxaSecTradeManagedResponse,
        WxaSecTradeManagementConfirmationResponse, WxaSecUploadCombinedShippingInfoRequest,
        WxaSecUploadShippingInfoRequest,
    };

    #[test]
    fn serializes_subscribe_message_request() {
        let value = serde_json::to_value(SubscribeMessageRequest {
            touser: "openid".to_string(),
            template_id: "tpl".to_string(),
            data: json!({ "thing1": { "value": "hello" } }),
            page: Some("pages/index".to_string()),
            miniprogram_state: Some("formal".to_string()),
            lang: Some("zh_CN".to_string()),
        })
        .unwrap();

        assert_eq!(value["template_id"], "tpl");
        assert_eq!(value["data"]["thing1"]["value"], "hello");
    }

    #[test]
    fn serializes_qr_code_default_width() {
        let value = serde_json::to_value(CreateQrCodeRequest {
            path: "pages/index".to_string(),
            width: 430,
        })
        .unwrap();

        assert_eq!(value, json!({ "path": "pages/index", "width": 430 }));
    }

    #[test]
    fn serializes_url_scheme_request() {
        let value = serde_json::to_value(UrlSchemeGenerateRequest {
            jump_wxa: Some(JumpWxa {
                path: Some("pages/index".to_string()),
                query: Some("a=1".to_string()),
                env_version: Some("release".to_string()),
            }),
            is_expire: true,
            expire_time: Some(1_800_000_000),
            expire_type: None,
            expire_interval: None,
        })
        .unwrap();

        assert_eq!(value["jump_wxa"]["path"], "pages/index");
        assert_eq!(value["is_expire"], true);
    }

    #[test]
    fn serializes_customer_service_text_message() {
        let value = serde_json::to_value(CustomerServiceMessage::text("openid", "hello")).unwrap();

        assert_eq!(value["touser"], "openid");
        assert_eq!(value["msgtype"], "text");
        assert_eq!(value["text"]["content"], "hello");
    }

    #[test]
    fn serializes_data_cube_date_range() {
        let value = serde_json::to_value(DataCubeDateRange {
            begin_date: "20260704".to_string(),
            end_date: "20260704".to_string(),
        })
        .unwrap();

        assert_eq!(
            value,
            json!({ "begin_date": "20260704", "end_date": "20260704" })
        );
    }

    #[test]
    fn serializes_security_msg_check_default_scene() {
        let value = serde_json::to_value(SecurityMsgSecCheckRequest {
            content: "hello".to_string(),
            scene: 1,
            version: Some(2),
            openid: Some("openid".to_string()),
        })
        .unwrap();

        assert_eq!(value["scene"], 1);
        assert_eq!(value["version"], 2);
        assert_eq!(value["openid"], "openid");
    }

    #[test]
    fn serializes_risk_control_user_risk_rank_request() {
        let value = serde_json::to_value(RiskControlGetUserRiskRankRequest {
            appid: "wxappid".to_string(),
            openid: "openid".to_string(),
            scene: 1,
            mobile_no: Some("13800000000".to_string()),
            bank_card_no: "6222000000000000".to_string(),
            cert_no: "110101199001010000".to_string(),
            client_ip: "127.0.0.1".to_string(),
            email_address: None,
            extended_info: Some("{\"device\":\"ios\"}".to_string()),
        })
        .unwrap();

        assert_eq!(value["appid"], "wxappid");
        assert_eq!(value["openid"], "openid");
        assert_eq!(value["scene"], 1);
        assert_eq!(value["mobile_no"], "13800000000");
        assert_eq!(value["bank_card_no"], "6222000000000000");
        assert_eq!(value["cert_no"], "110101199001010000");
        assert_eq!(value["client_ip"], "127.0.0.1");
        assert!(value.get("email_address").is_none());
        assert_eq!(value["extended_info"], "{\"device\":\"ios\"}");
    }

    #[test]
    fn deserializes_risk_control_user_risk_rank_response() {
        let response: RiskControlGetUserRiskRankResponse = serde_json::from_value(json!({
            "errcode": 0,
            "risk_rank": 2,
            "unoin_id": 123456
        }))
        .unwrap();

        assert_eq!(response.errcode, Some(0));
        assert_eq!(response.risk_rank, Some(2));
        assert_eq!(response.unoin_id, Some(123456));
    }

    #[test]
    fn serializes_wxa_sec_shipping_info_request() {
        let value = serde_json::to_value(WxaSecUploadShippingInfoRequest {
            order_key: WxaSecOrderKey {
                order_number_type: 2,
                transaction_id: Some("tx".to_string()),
                mch_id: Some("mchid".to_string()),
                out_trade_no: None,
            },
            logistics_type: 1,
            delivery_mode: 1,
            is_all_delivered: Some(true),
            shipping_list: vec![WxaSecShippingInfo {
                tracking_no: Some("tracking".to_string()),
                express_company: Some("SF".to_string()),
                item_desc: "book".to_string(),
                contact: Some(WxaSecShippingContact {
                    consignor_contact: None,
                    receiver_contact: Some("13800000000".to_string()),
                }),
            }],
            upload_time: "2026-07-07T00:00:00+08:00".to_string(),
            payer: WxaSecPayer {
                openid: "openid".to_string(),
            },
        })
        .unwrap();

        assert_eq!(value["order_key"]["mchid"], "mchid");
        assert!(value["order_key"].get("out_trade_no").is_none());
        assert_eq!(value["is_all_delivered"], true);
        assert_eq!(value["shipping_list"][0]["item_desc"], "book");
        assert_eq!(
            value["shipping_list"][0]["contact"]["receiver_contact"],
            "13800000000"
        );
        assert_eq!(value["payer"]["openid"], "openid");
    }

    #[test]
    fn serializes_wxa_sec_combined_shipping_info_request() {
        let value = serde_json::to_value(WxaSecUploadCombinedShippingInfoRequest {
            order_key: WxaSecOrderKey {
                order_number_type: 1,
                transaction_id: Some("tx-parent".to_string()),
                mch_id: None,
                out_trade_no: None,
            },
            sub_orders: vec![WxaSecSubOrderShippingInfo {
                order_key: WxaSecOrderKey {
                    order_number_type: 2,
                    transaction_id: None,
                    mch_id: Some("mchid".to_string()),
                    out_trade_no: Some("out-trade-no".to_string()),
                },
                logistics_type: 1,
                delivery_mode: 1,
                is_all_delivered: true,
                shipping_list: vec![WxaSecShippingInfo {
                    tracking_no: Some("tracking".to_string()),
                    express_company: None,
                    item_desc: "coffee".to_string(),
                    contact: None,
                }],
            }],
            upload_time: "2026-07-07T00:00:00+08:00".to_string(),
            payer: WxaSecPayer {
                openid: "openid".to_string(),
            },
        })
        .unwrap();

        assert_eq!(value["order_key"]["transaction_id"], "tx-parent");
        assert_eq!(
            value["sub_orders"][0]["order_key"]["out_trade_no"],
            "out-trade-no"
        );
        assert_eq!(value["sub_orders"][0]["is_all_delivered"], true);
        assert_eq!(value["payer"]["openid"], "openid");
    }

    #[test]
    fn serializes_wxa_sec_order_queries() {
        let order_query = serde_json::to_value(WxaSecOrderQuery {
            transaction_id: None,
            merchant_id: None,
            sub_merchant_id: None,
            merchant_trade_no: Some("trade-no".to_string()),
        })
        .unwrap();
        assert_eq!(order_query["merchant_trade_no"], "trade-no");
        assert!(order_query.get("transaction_id").is_none());

        let list_query = serde_json::to_value(WxaSecOrderListRequest {
            pay_time_range: Some(WxaSecPayTimeRange {
                begin_time: Some(1_800_000_000),
                end_time: Some(1_800_003_600),
            }),
            order_state: Some(2),
            openid: Some("openid".to_string()),
            last_index: Some("cursor".to_string()),
            page_size: Some(20),
        })
        .unwrap();
        assert_eq!(list_query["pay_time_range"]["begin_time"], 1_800_000_000);
        assert_eq!(list_query["page_size"], 20);

        let confirm = serde_json::to_value(WxaSecConfirmReceiveRequest {
            transaction_id: Some("tx".to_string()),
            merchant_id: None,
            sub_merchant_id: None,
            merchant_trade_no: None,
            received_time: Some(1_800_003_600),
        })
        .unwrap();
        assert_eq!(confirm["transaction_id"], "tx");
        assert_eq!(confirm["received_time"], 1_800_003_600);

        let special = serde_json::to_value(WxaSecSpecialOrderRequest {
            order_id: "order-id".to_string(),
            operation_type: 1,
            delay_to: Some(1_800_010_000),
        })
        .unwrap();
        assert_eq!(special["order_id"], "order-id");
        assert_eq!(special["type"], 1);
        assert_eq!(special["delay_to"], 1_800_010_000);
    }

    #[test]
    fn deserializes_wxa_sec_order_responses() {
        let order: WxaSecOrderResponse = serde_json::from_value(json!({
            "errcode": 0,
            "order": {
                "transaction_id": "tx",
                "order_state": 2
            }
        }))
        .unwrap();
        assert_eq!(order.errcode, Some(0));
        assert_eq!(order.order.expect("order")["transaction_id"], "tx");

        let list: WxaSecOrderListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "last_index": "cursor",
            "has_more": true,
            "order_list": [{ "transaction_id": "tx" }]
        }))
        .unwrap();
        assert_eq!(list.last_index.as_deref(), Some("cursor"));
        assert_eq!(list.has_more, Some(true));
        assert_eq!(list.order_list[0]["transaction_id"], "tx");

        let managed: WxaSecTradeManagedResponse = serde_json::from_value(json!({
            "errcode": 0,
            "is_trade_managed": true
        }))
        .unwrap();
        assert_eq!(managed.is_trade_managed, Some(true));

        let confirmation: WxaSecTradeManagementConfirmationResponse =
            serde_json::from_value(json!({
                "errcode": 0,
                "completed": true
            }))
            .unwrap();
        assert_eq!(confirmation.completed, Some(true));
    }

    #[test]
    fn deserializes_code2_session_response() {
        let response: Code2SessionResponse = serde_json::from_value(json!({
            "openid": "openid",
            "session_key": "session-key",
            "unionid": "unionid"
        }))
        .unwrap();

        assert_eq!(response.openid.as_deref(), Some("openid"));
        assert_eq!(response.session_key.as_deref(), Some("session-key"));
        assert_eq!(response.unionid.as_deref(), Some("unionid"));
    }

    #[test]
    fn deserializes_phone_number_response() {
        let response: PhoneNumberResponse = serde_json::from_value(json!({
            "phone_info": {
                "phone_number": "+8613800000000",
                "pure_phone_number": "13800000000",
                "country_code": "86",
                "watermark": { "appid": "wxappid" }
            }
        }))
        .unwrap();
        let phone_info = response.phone_info.expect("phone_info");

        assert_eq!(phone_info.phone_number, "+8613800000000");
        assert_eq!(phone_info.pure_phone_number, "13800000000");
        assert_eq!(phone_info.country_code, "86");
        assert_eq!(phone_info.watermark.expect("watermark")["appid"], "wxappid");
    }

    #[test]
    fn deserializes_ocr_responses() {
        let bankcard: OcrBankcardResponse = serde_json::from_value(json!({
            "errcode": 0,
            "number": "6222000000000000"
        }))
        .unwrap();
        assert_eq!(bankcard.number.as_deref(), Some("6222000000000000"));

        let business_license: OcrBusinessLicenseResponse = serde_json::from_value(json!({
            "errcode": 0,
            "reg_num": "91440000",
            "legal_representative": "Alice",
            "enterprise_name": "Example Ltd",
            "cert_position": { "pos": { "left_top": { "x": 1, "y": 2 } } },
            "img_size": { "w": 100, "h": 80 }
        }))
        .unwrap();
        assert_eq!(business_license.reg_num.as_deref(), Some("91440000"));
        assert_eq!(
            business_license.legal_representative.as_deref(),
            Some("Alice")
        );
        assert_eq!(
            business_license.cert_position.expect("cert_position")["pos"]["left_top"]["x"],
            1
        );

        let driving_license: OcrDrivingLicenseResponse = serde_json::from_value(json!({
            "errcode": 0,
            "id_num": "110101199001010000",
            "name": "Alice",
            "car_class": "C1",
            "official_seal": "seal"
        }))
        .unwrap();
        assert_eq!(
            driving_license.id_num.as_deref(),
            Some("110101199001010000")
        );
        assert_eq!(driving_license.car_class.as_deref(), Some("C1"));

        let id_card: OcrIdCardResponse = serde_json::from_value(json!({
            "errcode": 0,
            "type": "Front",
            "name": "Alice",
            "id": "110101199001010000",
            "addr": "Beijing"
        }))
        .unwrap();
        assert_eq!(id_card.r#type.as_deref(), Some("Front"));
        assert_eq!(id_card.id.as_deref(), Some("110101199001010000"));

        let printed_text: OcrPrintedTextResponse = serde_json::from_value(json!({
            "errcode": 0,
            "items": [{ "text": "hello", "pos": { "x": 1 } }],
            "img_size": { "w": 100 }
        }))
        .unwrap();
        assert_eq!(printed_text.items[0]["text"], "hello");
        assert_eq!(printed_text.img_size.expect("img_size")["w"], 100);

        let vehicle_license: OcrVehicleLicenseResponse = serde_json::from_value(json!({
            "errcode": 0,
            "vhicle_type": "small car",
            "owner": "Alice",
            "vin": "VIN123",
            "plate_num_b": "YUEB00000"
        }))
        .unwrap();
        assert_eq!(vehicle_license.vhicle_type.as_deref(), Some("small car"));
        assert_eq!(vehicle_license.plate_num_b.as_deref(), Some("YUEB00000"));
    }

    #[test]
    fn serializes_live_room_request() {
        let value = serde_json::to_value(LiveRoomRequest {
            name: "launch".to_string(),
            cover_img: "media-cover".to_string(),
            start_time: 1_800_000_000,
            end_time: 1_800_003_600,
            anchor_name: "host".to_string(),
            anchor_wechat: "host-wechat".to_string(),
            share_img: "media-share".to_string(),
            type_id: 1,
            screen_type: 0,
            close_like: 0,
            close_goods: 0,
            close_comment: 0,
            feeds_img: None,
            close_replay: Some(1),
            close_share: None,
            close_kf: None,
        })
        .unwrap();

        assert_eq!(value["name"], "launch");
        assert_eq!(value["coverImg"], "media-cover");
        assert_eq!(value["startTime"], 1_800_000_000);
        assert_eq!(value["endTime"], 1_800_003_600);
        assert_eq!(value["anchorName"], "host");
        assert_eq!(value["anchorWechat"], "host-wechat");
        assert_eq!(value["shareImg"], "media-share");
        assert_eq!(value["type"], 1);
        assert_eq!(value["screenType"], 0);
        assert_eq!(value["closeLike"], 0);
        assert_eq!(value["closeGoods"], 0);
        assert_eq!(value["closeComment"], 0);
        assert_eq!(value["closeReplay"], 1);
        assert!(value.get("feedsImg").is_none());
    }

    #[test]
    fn serializes_live_info_request() {
        let value = serde_json::to_value(LiveInfoRequest {
            start: 0,
            limit: 20,
        })
        .unwrap();

        assert_eq!(value, json!({ "start": 0, "limit": 20 }));
    }
}
