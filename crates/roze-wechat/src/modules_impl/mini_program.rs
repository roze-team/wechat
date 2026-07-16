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

    pub async fn access_token(
        &self,
        app_id: impl Into<String>,
        secret: impl Into<String>,
    ) -> Result<StableAccessTokenResponse> {
        self.inner
            .get_with_query(
                "cgi-bin/token",
                None,
                vec![
                    ("grant_type".to_string(), "client_credential".to_string()),
                    ("appid".to_string(), app_id.into()),
                    ("secret".to_string(), secret.into()),
                ],
            )
            .await
    }

    pub async fn get_paid_union_id(
        &self,
        access_token: impl Into<String>,
        request: MiniProgramPaidUnionIdRequest,
    ) -> Result<MiniProgramPaidUnionIdResponse> {
        let mut query = vec![("openid".to_string(), request.openid)];
        if let Some(transaction_id) = request.transaction_id {
            query.push(("transaction_id".to_string(), transaction_id));
        }
        if let Some(mch_id) = request.mch_id {
            query.push(("mch_id".to_string(), mch_id));
        }
        if let Some(out_trade_no) = request.out_trade_no {
            query.push(("out_trade_no".to_string(), out_trade_no));
        }
        self.inner
            .get_with_query("wxa/getpaidunionid", Some(access_token.into()), query)
            .await
    }

    pub async fn check_encrypted_data(
        &self,
        access_token: impl Into<String>,
        encrypted_msg_hash: impl Into<String>,
    ) -> Result<MiniProgramCheckEncryptedDataResponse> {
        self.inner
            .post(
                "wxa/business/checkencryptedmsg",
                Some(access_token.into()),
                json!({ "encrypted_msg_hash": encrypted_msg_hash.into() }),
            )
            .await
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

    pub async fn get_customer_service_temp_media_bytes(
        &self,
        access_token: impl Into<String>,
        media_id: impl Into<String>,
    ) -> Result<Bytes> {
        self.inner
            .get_bytes(
                "cgi-bin/media/get",
                Some(access_token.into()),
                vec![("media_id".to_string(), media_id.into())],
            )
            .await
    }

    pub async fn upload_customer_service_temp_media_from_bytes(
        &self,
        access_token: impl Into<String>,
        media_type: impl Into<String>,
        file_name: impl Into<String>,
        data: Vec<u8>,
    ) -> Result<MiniProgramUploadMediaResponse> {
        let form = reqwest::multipart::Form::new().part(
            "media",
            reqwest::multipart::Part::bytes(data).file_name(file_name.into()),
        );
        self.inner
            .post_multipart(
                "cgi-bin/media/upload",
                Some(access_token.into()),
                vec![("type".to_string(), media_type.into())],
                form,
            )
            .await
    }

    pub fn uniform_message(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "mini_program.uniform_message")
    }

    pub async fn send_uniform_message(
        &self,
        access_token: impl Into<String>,
        request: Value,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/message/wxopen/template/uniform_send",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub fn updatable_message(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "mini_program.updatable_message")
    }

    pub async fn create_updatable_message_activity_id(
        &self,
        access_token: impl Into<String>,
        union_id: impl Into<String>,
        open_id: impl Into<String>,
    ) -> Result<MiniProgramActivityIdResponse> {
        self.inner
            .get_with_query(
                "cgi-bin/message/wxopen/activityid/create",
                Some(access_token.into()),
                vec![
                    ("unionid".to_string(), union_id.into()),
                    ("openid".to_string(), open_id.into()),
                ],
            )
            .await
    }

    pub async fn send_updatable_message(
        &self,
        access_token: impl Into<String>,
        request: Value,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/message/wxopen/updatablemsg/send",
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

    pub async fn performance_data(
        &self,
        access_token: impl Into<String>,
        request: Value,
    ) -> Result<Value> {
        self.inner
            .post(
                "wxa/business/performance/boot",
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

    pub async fn get_live_goods_warehouse(
        &self,
        access_token: impl Into<String>,
        goods_ids: Vec<i64>,
    ) -> Result<MiniProgramLiveGoodsWarehouseResponse> {
        self.inner
            .post(
                "wxa/business/getgoodswarehouse",
                Some(access_token.into()),
                json!({ "goods_ids": goods_ids }),
            )
            .await
    }

    pub async fn get_live_followers(
        &self,
        access_token: impl Into<String>,
        limit: i64,
        page_break: i64,
    ) -> Result<MiniProgramLiveFollowersResponse> {
        self.inner
            .post(
                "wxa/business/get_wxa_followers",
                Some(access_token.into()),
                json!({ "limit": limit, "page_break": page_break }),
            )
            .await
    }

    pub async fn push_live_message(
        &self,
        access_token: impl Into<String>,
        room_id: i64,
        user_openid: Vec<String>,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "wxa/business/push_message",
                Some(access_token.into()),
                json!({ "room_id": room_id, "user_openid": user_openid }),
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

    pub fn image(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "mini_program.image")
    }

    pub async fn image_ai_crop_by_url(
        &self,
        access_token: impl Into<String>,
        img_url: impl Into<String>,
    ) -> Result<ImageAiCropResponse> {
        self.ocr_by_url("cv/img/aicrop", access_token, img_url, Vec::new())
            .await
    }

    pub async fn image_ai_crop_from_bytes(
        &self,
        access_token: impl Into<String>,
        file_name: impl Into<String>,
        data: Vec<u8>,
    ) -> Result<ImageAiCropResponse> {
        self.ocr_from_bytes("cv/img/aicrop", access_token, file_name, data, Vec::new())
            .await
    }

    pub async fn image_scan_qrcode_by_url(
        &self,
        access_token: impl Into<String>,
        img_url: impl Into<String>,
    ) -> Result<ImageScanQrCodeResponse> {
        self.ocr_by_url("cv/img/qrcode", access_token, img_url, Vec::new())
            .await
    }

    pub async fn image_scan_qrcode_from_bytes(
        &self,
        access_token: impl Into<String>,
        file_name: impl Into<String>,
        data: Vec<u8>,
    ) -> Result<ImageScanQrCodeResponse> {
        self.ocr_from_bytes("cv/img/qrcode", access_token, file_name, data, Vec::new())
            .await
    }

    pub async fn image_super_resolution_by_url(
        &self,
        access_token: impl Into<String>,
        img_url: impl Into<String>,
    ) -> Result<ImageSuperResolutionResponse> {
        self.ocr_by_url("cv/img/superresolution", access_token, img_url, Vec::new())
            .await
    }

    pub async fn image_super_resolution_from_bytes(
        &self,
        access_token: impl Into<String>,
        file_name: impl Into<String>,
        data: Vec<u8>,
    ) -> Result<ImageSuperResolutionResponse> {
        self.ocr_from_bytes(
            "cv/img/superresolution",
            access_token,
            file_name,
            data,
            Vec::new(),
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

    pub async fn image_security_check_from_bytes(
        &self,
        access_token: impl Into<String>,
        file_name: impl Into<String>,
        data: Vec<u8>,
    ) -> Result<WechatStatusResponse> {
        let form = reqwest::multipart::Form::new().part(
            "media",
            reqwest::multipart::Part::bytes(data).file_name(file_name.into()),
        );
        self.inner
            .post_multipart(
                "wxa/img_sec_check",
                Some(access_token.into()),
                Vec::new(),
                form,
            )
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

    pub fn soter(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "mini_program.soter")
    }

    pub async fn verify_soter_signature(
        &self,
        access_token: impl Into<String>,
        request: SoterVerifySignatureRequest,
    ) -> Result<SoterVerifySignatureResponse> {
        self.inner
            .post(
                "cgi-bin/soter/verify_signature",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub fn service_market(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "mini_program.service_market")
    }

    pub async fn invoke_service_market(
        &self,
        access_token: impl Into<String>,
        request: ServiceMarketInvokeRequest,
    ) -> Result<ServiceMarketInvokeResponse> {
        self.inner
            .post("wxa/servicemarket", Some(access_token.into()), request)
            .await
    }

    pub fn internet(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "mini_program.internet")
    }

    pub async fn get_user_encrypt_key(
        &self,
        access_token: impl Into<String>,
        request: InternetGetUserEncryptKeyRequest,
    ) -> Result<InternetGetUserEncryptKeyResponse> {
        self.inner
            .post_json_with_access_token_query(
                "wxa/business/getuserencryptkey",
                Some(access_token.into()),
                vec![
                    ("openid".to_string(), request.openid),
                    ("signature".to_string(), request.signature),
                    ("sig_method".to_string(), request.sig_method),
                ],
                json!({}),
                Vec::new(),
            )
            .await
    }

    pub fn device(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "mini_program.device")
    }

    pub async fn send_hardware_device_message(
        &self,
        access_token: impl Into<String>,
        request: DeviceSubscribeMessageRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/message/device/subscribe/send",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_device_sn_ticket(
        &self,
        access_token: impl Into<String>,
        request: DeviceSnTicketRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post("wxa/getsnticket", Some(access_token.into()), request)
            .await
    }

    pub async fn create_iot_group_id(
        &self,
        access_token: impl Into<String>,
        request: DeviceCreateIotGroupRequest,
    ) -> Result<DeviceCreateIotGroupResponse> {
        self.inner
            .post(
                "wxa/business/group/createid",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_iot_group_info(
        &self,
        access_token: impl Into<String>,
        request: DeviceGetIotGroupInfoRequest,
    ) -> Result<DeviceGetIotGroupInfoResponse> {
        self.inner
            .post(
                "wxa/business/group/getinfo",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn add_iot_group_device(
        &self,
        access_token: impl Into<String>,
        request: DeviceGroupDeviceListRequest,
    ) -> Result<DeviceGroupOperationResponse> {
        self.inner
            .post(
                "wxa/business/group/adddevice",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn remove_iot_group_device(
        &self,
        access_token: impl Into<String>,
        request: DeviceGroupDeviceListRequest,
    ) -> Result<DeviceGroupOperationResponse> {
        self.inner
            .post(
                "wxa/business/group/removedevice",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_device_license_pkg_list(
        &self,
        access_token: impl Into<String>,
        pkg_type: i64,
    ) -> Result<DeviceLicensePkgListResponse> {
        self.inner
            .post(
                "wxa/business/license/getpkglist",
                Some(access_token.into()),
                json!({ "pkg_type": pkg_type }),
            )
            .await
    }

    pub async fn active_device_license(
        &self,
        access_token: impl Into<String>,
        request: DeviceActiveLicenseRequest,
    ) -> Result<DeviceActiveLicenseResponse> {
        self.inner
            .post(
                "wxa/business/license/activedevice",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_device_license_info(
        &self,
        access_token: impl Into<String>,
        device_list: Vec<DeviceInfo>,
    ) -> Result<DeviceLicenseInfoResponse> {
        self.inner
            .post(
                "wxa/business/license/getdeviceinfo",
                Some(access_token.into()),
                json!({ "device_list": device_list }),
            )
            .await
    }

    pub fn operation(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "mini_program.operation")
    }

    pub async fn get_operation_domain_info(
        &self,
        access_token: impl Into<String>,
        action: impl Into<String>,
    ) -> Result<OperationDomainInfoResponse> {
        self.inner
            .post(
                "wxa/getwxadevinfo",
                Some(access_token.into()),
                json!({ "action": action.into() }),
            )
            .await
    }

    pub async fn get_operation_feedback(
        &self,
        access_token: impl Into<String>,
        feedback_type: i64,
        page: i64,
        num: i64,
    ) -> Result<OperationFeedbackResponse> {
        self.inner
            .get_with_query(
                "wxaapi/feedback/list",
                Some(access_token.into()),
                vec![
                    ("type".to_string(), feedback_type.to_string()),
                    ("page".to_string(), page.to_string()),
                    ("num".to_string(), num.to_string()),
                ],
            )
            .await
    }

    pub async fn get_operation_feedback_media(
        &self,
        access_token: impl Into<String>,
        record_id: i64,
        media_id: impl Into<String>,
    ) -> Result<Bytes> {
        self.inner
            .post_form_bytes(
                "cgi-bin/media/getfeedbackmedia",
                Some(access_token.into()),
                vec![
                    ("record_id".to_string(), record_id.to_string()),
                    ("media_id".to_string(), media_id.into()),
                ],
            )
            .await
    }

    pub async fn get_operation_gray_release_plan(
        &self,
        access_token: impl Into<String>,
    ) -> Result<OperationGrayReleasePlanResponse> {
        self.inner
            .get("wxa/getgrayreleaseplan", Some(access_token.into()))
            .await
    }

    pub async fn get_operation_js_err_detail(
        &self,
        access_token: impl Into<String>,
        request: OperationRequest,
    ) -> Result<OperationJsErrDetailResponse> {
        self.inner
            .post(
                "wxaapi/log/jserr_detail",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_operation_js_err_list(
        &self,
        access_token: impl Into<String>,
        request: OperationRequest,
    ) -> Result<OperationJsErrListResponse> {
        self.inner
            .post("wxaapi/log/jserr_list", Some(access_token.into()), request)
            .await
    }

    pub async fn search_operation_js_err(
        &self,
        access_token: impl Into<String>,
        request: OperationRequest,
    ) -> Result<OperationJsErrSearchResponse> {
        self.inner
            .post(
                "wxaapi/log/jserr_search",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_operation_performance(
        &self,
        access_token: impl Into<String>,
        request: OperationRequest,
    ) -> Result<OperationPerformanceResponse> {
        self.inner
            .post(
                "wxaapi/log/get_performance",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_operation_scene_list(
        &self,
        access_token: impl Into<String>,
    ) -> Result<OperationSceneListResponse> {
        self.inner
            .get("wxa/log/get_scene", Some(access_token.into()))
            .await
    }

    pub async fn get_operation_version_list(
        &self,
        access_token: impl Into<String>,
    ) -> Result<OperationVersionListResponse> {
        self.inner
            .get("wxaapi/log/get_client_version", Some(access_token.into()))
            .await
    }

    pub async fn search_operation_real_time_log(
        &self,
        access_token: impl Into<String>,
        request: OperationRequest,
    ) -> Result<OperationRealTimeLogSearchResponse> {
        self.inner
            .post(
                "wxaapi/userlog/userlog_search",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub fn server(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "mini_program.server")
    }

    pub fn search(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "mini_program.search")
    }

    pub async fn image_search(
        &self,
        access_token: impl Into<String>,
        request: SearchImageSearchRequest,
    ) -> Result<SearchImageSearchResponse> {
        self.inner
            .post("wxa/imagesearch", Some(access_token.into()), request)
            .await
    }

    pub async fn site_search(
        &self,
        access_token: impl Into<String>,
        request: SearchSiteSearchRequest,
    ) -> Result<SearchSiteSearchResponse> {
        self.inner
            .post("wxa/sitesearch", Some(access_token.into()), request)
            .await
    }

    pub async fn submit_search_pages(
        &self,
        access_token: impl Into<String>,
        request: SearchSubmitPagesRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "wxa/search/wxaapi_submitpages",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub fn nearby_poi(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "mini_program.nearby_poi")
    }

    pub async fn add_nearby_poi(
        &self,
        access_token: impl Into<String>,
        request: NearbyPoiAddRequest,
    ) -> Result<NearbyPoiAddResponse> {
        self.inner
            .post("wxa/addnearbypoi", Some(access_token.into()), request)
            .await
    }

    pub async fn delete_nearby_poi(
        &self,
        access_token: impl Into<String>,
        poi_id: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "wxa/delnearbypoi",
                Some(access_token.into()),
                json!({ "poi_id": poi_id.into() }),
            )
            .await
    }

    pub async fn get_nearby_poi_list(
        &self,
        access_token: impl Into<String>,
        page: i64,
        page_rows: i64,
    ) -> Result<NearbyPoiListResponse> {
        self.inner
            .get_with_query(
                "wxa/getnearbypoilist",
                Some(access_token.into()),
                vec![
                    ("page".to_string(), page.to_string()),
                    ("page_rows".to_string(), page_rows.to_string()),
                ],
            )
            .await
    }

    pub async fn set_nearby_poi_show_status(
        &self,
        access_token: impl Into<String>,
        request: NearbyPoiShowStatusRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "wxa/setnearbypoishowstatus",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub fn plugin(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "mini_program.plugin")
    }

    pub async fn apply_plugin(
        &self,
        access_token: impl Into<String>,
        plugin_app_id: impl Into<String>,
        reason: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/wxa/plugin",
                Some(access_token.into()),
                PluginActionRequest::apply(plugin_app_id, reason),
            )
            .await
    }

    pub async fn get_plugin_list(
        &self,
        access_token: impl Into<String>,
    ) -> Result<PluginListResponse> {
        self.inner
            .post(
                "cgi-bin/wxa/plugin",
                Some(access_token.into()),
                json!({ "action": "list" }),
            )
            .await
    }

    pub async fn unbind_plugin(
        &self,
        access_token: impl Into<String>,
        plugin_app_id: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/wxa/plugin",
                Some(access_token.into()),
                PluginActionRequest::unbind(plugin_app_id),
            )
            .await
    }

    pub async fn get_plugin_dev_apply_list(
        &self,
        access_token: impl Into<String>,
        request: PluginDevApplyListRequest,
    ) -> Result<PluginDevApplyListResponse> {
        self.inner
            .post("cgi-bin/wxa/devplugin", Some(access_token.into()), request)
            .await
    }

    pub async fn set_plugin_dev_apply_status(
        &self,
        access_token: impl Into<String>,
        request: PluginDevApplyStatusRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post("cgi-bin/wxa/devplugin", Some(access_token.into()), request)
            .await
    }

    pub fn virtual_payment(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "mini_program.virtual_payment")
    }

    pub fn build_virtual_payment_order(
        &self,
        offer_id: impl Into<String>,
        app_key: impl AsRef<[u8]>,
        request: VirtualPaymentOrderRequest,
    ) -> Result<VirtualPaymentOrderResponse> {
        let post_body = virtual_payment_order_post_body(&offer_id.into(), &request);
        Ok(VirtualPaymentOrderResponse {
            pay_sign: crypto::hmac_sha256_hex(
                app_key.as_ref(),
                format!("requestVirtualPayment&{post_body}").as_bytes(),
            )?,
            signature: crypto::hmac_sha256_hex(
                request.session_key.as_bytes(),
                post_body.as_bytes(),
            )?,
            post_body,
        })
    }

    pub async fn start_upload_virtual_payment_goods(
        &self,
        app_key: impl AsRef<[u8]>,
        request: VirtualPaymentUploadProductsRequest,
    ) -> Result<VirtualPaymentUploadGoodsResponse> {
        let post_body = serde_json::to_string(&request)?;
        let pay_sig = crypto::hmac_sha256_hex(
            app_key.as_ref(),
            format!("/xpay/start_upload_goods&{post_body}").as_bytes(),
        )?;
        let body = serde_json::from_str(&post_body)?;

        self.inner
            .post_json_with_query(
                "/xpay/start_upload_goods",
                vec![("pay_sig".to_string(), pay_sig)],
                body,
                Vec::new(),
            )
            .await
    }

    pub fn b2b(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "mini_program.b2b")
    }

    pub fn build_b2b_payment(
        &self,
        session_key: impl AsRef<[u8]>,
        app_key: impl AsRef<[u8]>,
        data: Value,
    ) -> Result<B2bPaymentResponse> {
        let sign_data = serde_json::to_string(&data)?;
        Ok(B2bPaymentResponse {
            sign_data: sign_data.clone(),
            mode: "retail_pay_goods".to_string(),
            signature: crypto::hmac_sha256_hex(session_key.as_ref(), sign_data.as_bytes())?,
            pay_sig: crypto::hmac_sha256_hex(
                app_key.as_ref(),
                format!("requestCommonPayment&{sign_data}").as_bytes(),
            )?,
        })
    }

    pub async fn get_b2b_order(
        &self,
        app_key: impl AsRef<[u8]>,
        request: B2bGetOrderRequest,
    ) -> Result<B2bGetOrderResponse> {
        self.post_b2b("/retail/B2b/getorder", app_key, request)
            .await
    }

    pub async fn refund_b2b_order(
        &self,
        app_key: impl AsRef<[u8]>,
        request: B2bRefundRequest,
    ) -> Result<B2bRefundResponse> {
        self.post_b2b("/retail/B2b/refund", app_key, request).await
    }

    pub async fn get_b2b_refund(
        &self,
        app_key: impl AsRef<[u8]>,
        request: B2bGetRefundRequest,
    ) -> Result<B2bGetRefundResponse> {
        self.post_b2b("/retail/B2b/getrefund", app_key, request)
            .await
    }

    pub async fn add_b2b_profit_sharing_account(
        &self,
        app_key: impl AsRef<[u8]>,
        request: B2bAddProfitSharingAccountRequest,
    ) -> Result<B2bStatusResponse> {
        self.post_b2b("/retail/B2b/addprofitsharingaccount", app_key, request)
            .await
    }

    pub async fn delete_b2b_profit_sharing_account(
        &self,
        app_key: impl AsRef<[u8]>,
        request: B2bDeleteProfitSharingAccountRequest,
    ) -> Result<B2bStatusResponse> {
        self.post_b2b("/retail/B2b/delprofitsharingaccount", app_key, request)
            .await
    }

    pub async fn query_b2b_profit_sharing_account(
        &self,
        app_key: impl AsRef<[u8]>,
        request: B2bQueryProfitSharingAccountRequest,
    ) -> Result<B2bQueryProfitSharingAccountResponse> {
        self.post_b2b("/retail/B2b/queryprofitsharingaccount", app_key, request)
            .await
    }

    pub async fn create_b2b_profit_sharing_order(
        &self,
        app_key: impl AsRef<[u8]>,
        request: B2bCreateProfitSharingOrderRequest,
    ) -> Result<B2bStatusResponse> {
        self.post_b2b("/retail/B2b/createprofitsharingorder", app_key, request)
            .await
    }

    pub async fn query_b2b_profit_sharing_order(
        &self,
        app_key: impl AsRef<[u8]>,
        request: B2bQueryProfitSharingOrderRequest,
    ) -> Result<B2bProfitSharingOrderResponse> {
        self.post_b2b("/retail/B2b/queryprofitsharingorder", app_key, request)
            .await
    }

    pub async fn query_b2b_profit_sharing_remain_amount(
        &self,
        app_key: impl AsRef<[u8]>,
        request: B2bQueryProfitSharingRemainAmountRequest,
    ) -> Result<B2bProfitSharingRemainAmountResponse> {
        self.post_b2b("/retail/B2b/queryprofitsharingremainamt", app_key, request)
            .await
    }

    pub async fn finish_b2b_profit_sharing_order(
        &self,
        app_key: impl AsRef<[u8]>,
        request: B2bFinishProfitSharingOrderRequest,
    ) -> Result<B2bStatusResponse> {
        self.post_b2b("/retail/B2b/finishprofitsharingorder", app_key, request)
            .await
    }

    pub async fn refund_b2b_profit_sharing(
        &self,
        app_key: impl AsRef<[u8]>,
        request: B2bRefundProfitSharingRequest,
    ) -> Result<B2bStatusResponse> {
        self.post_b2b("/retail/B2b/refundprofitsharing", app_key, request)
            .await
    }

    pub async fn query_b2b_refund_profit_sharing_order(
        &self,
        app_key: impl AsRef<[u8]>,
        request: B2bQueryRefundProfitSharingOrderRequest,
    ) -> Result<B2bProfitSharingOrderResponse> {
        self.post_b2b(
            "/retail/B2b/queryrefundprofitsharingorder",
            app_key,
            request,
        )
        .await
    }

    pub async fn download_b2b_bill(
        &self,
        app_key: impl AsRef<[u8]>,
        request: B2bDownloadBillRequest,
    ) -> Result<B2bDownloadBillResponse> {
        self.post_b2b("/retail/B2b/downloadbill", app_key, request)
            .await
    }

    async fn post_b2b<B, R>(
        &self,
        path: &'static str,
        app_key: impl AsRef<[u8]>,
        request: B,
    ) -> Result<R>
    where
        B: Serialize,
        R: for<'de> Deserialize<'de>,
    {
        let body = serde_json::to_string(&request)?;
        let pay_sig =
            crypto::hmac_sha256_hex(app_key.as_ref(), format!("{path}&{body}").as_bytes())?;
        self.inner
            .post_raw_json(
                path,
                vec![("pay_sig".to_string(), pay_sig)],
                "application/json".to_string(),
                body.into_bytes(),
                Vec::new(),
            )
            .await
    }

    pub fn industry_mini_drama_vod(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "mini_program.industry_mini_drama_vod")
    }

    pub async fn upload_mini_drama_video_by_url(
        &self,
        access_token: impl Into<String>,
        request: MiniDramaVideoMediaUploadByUrlRequest,
    ) -> Result<MiniDramaVideoMediaUploadByUrlResponse> {
        self.inner
            .post("wxa/sec/vod/pullupload", Some(access_token.into()), request)
            .await
    }

    pub async fn upload_mini_drama_video_by_file_from_bytes(
        &self,
        access_token: impl Into<String>,
        request: MiniDramaVideoMediaUploadByFileRequest,
    ) -> Result<MiniDramaVideoMediaUploadByFileResponse> {
        let form = reqwest::multipart::Form::new()
            .part(
                "media_data",
                reqwest::multipart::Part::bytes(request.media_data)
                    .file_name(request.media_name.clone()),
            )
            .part(
                "cover_data",
                reqwest::multipart::Part::bytes(request.cover_data.unwrap_or_default())
                    .file_name("cover"),
            );
        self.inner
            .post_multipart_with_headers(
                "wxa/sec/vod/singlefileupload",
                Some(access_token.into()),
                Vec::new(),
                form,
                vec![
                    ("media_type".to_string(), request.media_type),
                    ("cover_type".to_string(), request.cover_type),
                    ("media_name".to_string(), request.media_name),
                ],
            )
            .await
    }

    pub async fn get_mini_drama_upload_task(
        &self,
        access_token: impl Into<String>,
        task_id: i64,
    ) -> Result<MiniDramaVideoMediaTaskResponse> {
        self.inner
            .post(
                "wxa/sec/vod/gettask",
                Some(access_token.into()),
                json!({ "task_id": task_id }),
            )
            .await
    }

    pub async fn apply_mini_drama_chunk_upload(
        &self,
        access_token: impl Into<String>,
        request: MiniDramaVideoApplyChunkUploadRequest,
    ) -> Result<MiniDramaVideoApplyChunkUploadResponse> {
        self.inner
            .post(
                "wxa/sec/vod/applyupload",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn upload_mini_drama_chunk_from_bytes(
        &self,
        access_token: impl Into<String>,
        request: MiniDramaVideoChunkUploadRequest,
    ) -> Result<MiniDramaVideoChunkUploadResponse> {
        let form = reqwest::multipart::Form::new().part(
            "data",
            reqwest::multipart::Part::bytes(request.data).file_name("chunk"),
        );
        self.inner
            .post_multipart_with_headers(
                "wxa/sec/vod/singlefileupload",
                Some(access_token.into()),
                Vec::new(),
                form,
                vec![
                    (
                        "resource_type".to_string(),
                        request.resource_type.to_string(),
                    ),
                    ("part_number".to_string(), request.part_number.to_string()),
                    ("upload_id".to_string(), request.upload_id),
                ],
            )
            .await
    }

    pub async fn complete_mini_drama_chunk_upload(
        &self,
        access_token: impl Into<String>,
        request: MiniDramaVideoChunkUploadCompleteRequest,
    ) -> Result<MiniDramaVideoChunkUploadCompleteResponse> {
        self.inner
            .post(
                "wxa/sec/vod/commitupload",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_mini_drama_media_list(
        &self,
        access_token: impl Into<String>,
        request: MiniDramaMediaListRequest,
    ) -> Result<MiniDramaMediaListResponse> {
        self.inner
            .post(
                "wxa/sec/vod/listmedia",
                Some(access_token.into()),
                request.normalized(),
            )
            .await
    }

    pub async fn get_mini_drama_media_info(
        &self,
        access_token: impl Into<String>,
        media_id: i64,
    ) -> Result<MiniDramaMediaInfoResponse> {
        self.inner
            .post(
                "wxa/sec/vod/getmedia",
                Some(access_token.into()),
                json!({ "media_id": media_id }),
            )
            .await
    }

    pub async fn get_mini_drama_media_link(
        &self,
        access_token: impl Into<String>,
        request: MiniDramaMediaLinkRequest,
    ) -> Result<MiniDramaMediaLinkResponse> {
        self.inner
            .post(
                "wxa/sec/vod/getmedialink",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn delete_mini_drama_media(
        &self,
        access_token: impl Into<String>,
        media_id: i64,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "wxa/sec/vod/deletemedia",
                Some(access_token.into()),
                json!({ "media_id": media_id }),
            )
            .await
    }

    pub async fn submit_mini_drama_audit(
        &self,
        access_token: impl Into<String>,
        request: MiniDramaSubmitAuditRequest,
    ) -> Result<MiniDramaSubmitAuditResponse> {
        self.inner
            .post("wxa/sec/vod/auditdrama", Some(access_token.into()), request)
            .await
    }

    pub async fn get_mini_drama_list(
        &self,
        access_token: impl Into<String>,
        request: MiniDramaListRequest,
    ) -> Result<MiniDramaListResponse> {
        self.inner
            .post(
                "wxa/sec/vod/listdramas",
                Some(access_token.into()),
                request.normalized(),
            )
            .await
    }

    pub async fn get_mini_drama_info(
        &self,
        access_token: impl Into<String>,
        drama_id: i64,
    ) -> Result<MiniDramaInfoResponse> {
        self.inner
            .post(
                "wxa/sec/vod/getdrama",
                Some(access_token.into()),
                json!({ "drama_id": drama_id }),
            )
            .await
    }

    pub async fn submit_replace_mini_drama_audit(
        &self,
        access_token: impl Into<String>,
        request: MiniDramaSubmitReplaceAuditRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "wxa/sec/vod/submitreplacedramamedias",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn replace_audited_mini_drama_media(
        &self,
        access_token: impl Into<String>,
        request: MiniDramaReplaceAuditedMediaRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "wxa/sec/vod/replacedramamedia",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn update_mini_drama_info(
        &self,
        access_token: impl Into<String>,
        request: MiniDramaUpdateInfoRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "wxa/sec/vod/modifydramabasicinfo",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_mini_drama_latest_audit_info(
        &self,
        access_token: impl Into<String>,
        request: MiniDramaAuditInfoRequest,
    ) -> Result<MiniDramaAuditInfoResponse> {
        self.inner
            .post(
                "wxa/sec/vod/getdramalatestauditinfo",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_mini_drama_cdn_usage(
        &self,
        access_token: impl Into<String>,
        request: MiniDramaCdnInfoRequest,
    ) -> Result<MiniDramaCdnUsageResponse> {
        self.inner
            .post(
                "wxa/sec/vod/getcdnusagedata",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_mini_drama_cdn_logs(
        &self,
        access_token: impl Into<String>,
        request: MiniDramaCdnInfoRequest,
    ) -> Result<MiniDramaCdnLogsResponse> {
        self.inner
            .post("wxa/sec/vod/getcdnlogs", Some(access_token.into()), request)
            .await
    }

    pub async fn list_mini_drama_packages(
        &self,
        access_token: impl Into<String>,
        request: MiniDramaListRequest,
    ) -> Result<MiniDramaPackageListResponse> {
        self.inner
            .post(
                "wxa/sec/vod/listpackages",
                Some(access_token.into()),
                request.normalized(),
            )
            .await
    }

    pub async fn add_mini_drama_authorization(
        &self,
        access_token: impl Into<String>,
        request: MiniDramaAuthorizationRequest,
    ) -> Result<MiniDramaAuthorizationResponse> {
        self.inner
            .post(
                "wxa/sec/vod/authorizedrama",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn remove_mini_drama_authorization(
        &self,
        access_token: impl Into<String>,
        request: MiniDramaAuthorizationRequest,
    ) -> Result<MiniDramaAuthorizationResponse> {
        self.inner
            .post(
                "wxa/sec/vod/deauthorizedrama",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn search_mini_drama_authorization(
        &self,
        access_token: impl Into<String>,
        request: MiniDramaAuthorizationSearchRequest,
    ) -> Result<MiniDramaAuthorizationSearchResponse> {
        self.inner
            .post(
                "wxa/sec/vod/getauthorizeobjects",
                Some(access_token.into()),
                request.normalized(),
            )
            .await
    }

    pub async fn search_mini_drama_authorized_by(
        &self,
        access_token: impl Into<String>,
        request: MiniDramaAuthorizedBySearchRequest,
    ) -> Result<MiniDramaAuthorizationSearchResponse> {
        self.inner
            .post(
                "wxa/sec/vod/getauthorizedobjects",
                Some(access_token.into()),
                request.normalized(),
            )
            .await
    }

    pub async fn add_mini_drama_account_authorization(
        &self,
        access_token: impl Into<String>,
        request: MiniDramaAccountAuthorizationRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "wxa/sec/vod/authorizeapp",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn remove_mini_drama_account_authorization(
        &self,
        access_token: impl Into<String>,
        request: MiniDramaAccountAuthorizationRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "wxa/sec/vod/deauthorizeapp",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn search_mini_drama_account_authorization(
        &self,
        access_token: impl Into<String>,
    ) -> Result<MiniDramaAccountAuthorizationSearchResponse> {
        self.inner
            .post(
                "wxa/sec/vod/getauthorizeapps",
                Some(access_token.into()),
                json!({}),
            )
            .await
    }

    pub async fn search_mini_drama_account_authorized_by(
        &self,
        access_token: impl Into<String>,
    ) -> Result<MiniDramaAccountAuthorizationSearchResponse> {
        self.inner
            .post(
                "wxa/sec/vod/getauthorizedapps",
                Some(access_token.into()),
                json!({}),
            )
            .await
    }

    pub async fn set_flush_drama(
        &self,
        access_token: impl Into<String>,
        request: MiniDramaSetFlushDramaRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "wxadrama/developersetflushdrama",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub fn immediate_delivery(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "mini_program.immediate_delivery")
    }

    pub async fn abnormal_confirm_immediate_delivery(
        &self,
        access_token: impl Into<String>,
        request: ImmediateDeliveryRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/express/local/business/order/confirm_return",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn add_immediate_delivery_order(
        &self,
        access_token: impl Into<String>,
        request: ImmediateDeliveryRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/immediateDelivery/local/business/order/add",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn add_immediate_delivery_tip(
        &self,
        access_token: impl Into<String>,
        request: ImmediateDeliveryRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/immediateDelivery/local/business/order/addtips",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn bind_immediate_delivery_account(
        &self,
        access_token: impl Into<String>,
        delivery_id: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/express/local/business/shop/add",
                Some(access_token.into()),
                json!({ "delivery_id": delivery_id.into() }),
            )
            .await
    }

    pub async fn cancel_immediate_delivery_order(
        &self,
        access_token: impl Into<String>,
        request: ImmediateDeliveryRequest,
    ) -> Result<ImmediateDeliveryCancelOrderResponse> {
        self.inner
            .post(
                "cgi-bin/immediateDelivery/local/business/order/cancel",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_all_immediate_delivery(
        &self,
        access_token: impl Into<String>,
    ) -> Result<ImmediateDeliveryDeliveryListResponse> {
        self.inner
            .post(
                "cgi-bin/immediateDelivery/local/business/delivery/getall",
                Some(access_token.into()),
                json!({}),
            )
            .await
    }

    pub async fn get_immediate_delivery_bind_account(
        &self,
        access_token: impl Into<String>,
    ) -> Result<ImmediateDeliveryBindAccountResponse> {
        self.inner
            .post(
                "cgi-bin/express/local/business/shop/get",
                Some(access_token.into()),
                json!({}),
            )
            .await
    }

    pub async fn get_immediate_delivery_order(
        &self,
        access_token: impl Into<String>,
        request: ImmediateDeliveryGetOrderRequest,
    ) -> Result<ImmediateDeliveryGetOrderResponse> {
        self.inner
            .post(
                "cgi-bin/immediateDelivery/local/business/order/get",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn mock_update_immediate_delivery_order(
        &self,
        access_token: impl Into<String>,
        request: ImmediateDeliveryRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/immediateDelivery/local/business/test_update_order",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn open_immediate_delivery(
        &self,
        access_token: impl Into<String>,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/immediateDelivery/local/business/open",
                Some(access_token.into()),
                json!({}),
            )
            .await
    }

    pub async fn pre_add_immediate_delivery_order(
        &self,
        access_token: impl Into<String>,
        request: ImmediateDeliveryRequest,
    ) -> Result<ImmediateDeliveryPreAddOrderResponse> {
        self.inner
            .post(
                "cgi-bin/immediateDelivery/local/business/order/pre_add",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn pre_cancel_immediate_delivery_order(
        &self,
        access_token: impl Into<String>,
        request: ImmediateDeliveryRequest,
    ) -> Result<ImmediateDeliveryPreCancelOrderResponse> {
        self.inner
            .post(
                "cgi-bin/immediateDelivery/local/business/order/precancel",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn real_mock_update_immediate_delivery_order(
        &self,
        access_token: impl Into<String>,
        request: ImmediateDeliveryRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/immediateDelivery/local/business/realmock_update_order",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn re_add_immediate_delivery_order(
        &self,
        access_token: impl Into<String>,
        request: ImmediateDeliveryRequest,
    ) -> Result<ImmediateDeliveryReOrderResponse> {
        self.inner
            .post(
                "cgi-bin/immediateDelivery/local/business/order/readd",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub fn express(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "mini_program.express")
    }

    pub async fn add_express_order(
        &self,
        access_token: impl Into<String>,
        request: ExpressRequest,
    ) -> Result<ExpressAddOrderResponse> {
        self.inner
            .post(
                "cgi-bin/express/business/order/add",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn batch_get_express_order(
        &self,
        access_token: impl Into<String>,
        order_list: Vec<Value>,
    ) -> Result<ExpressBatchOrderListResponse> {
        self.inner
            .post(
                "cgi-bin/express/business/order/batchget",
                Some(access_token.into()),
                json!({ "order_list": order_list }),
            )
            .await
    }

    pub async fn bind_express_account(
        &self,
        access_token: impl Into<String>,
        request: ExpressBindAccountRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/express/business/accountService/bind",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn cancel_express_order(
        &self,
        access_token: impl Into<String>,
        request: ExpressOrderRequest,
    ) -> Result<ExpressCancelOrderResponse> {
        self.inner
            .post(
                "cgi-bin/express/business/order/cancel",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_all_express_account(
        &self,
        access_token: impl Into<String>,
    ) -> Result<ExpressAccountListResponse> {
        self.inner
            .get(
                "cgi-bin/express/business/accountService/getall",
                Some(access_token.into()),
            )
            .await
    }

    pub async fn get_all_express_delivery(
        &self,
        access_token: impl Into<String>,
    ) -> Result<ExpressDeliveryListResponse> {
        self.inner
            .get(
                "cgi-bin/express/business/delivery/getall",
                Some(access_token.into()),
            )
            .await
    }

    pub async fn get_express_order(
        &self,
        access_token: impl Into<String>,
        request: ExpressGetOrderRequest,
    ) -> Result<ExpressGetOrderResponse> {
        self.inner
            .post(
                "cgi-bin/express/business/order/get",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_express_path(
        &self,
        access_token: impl Into<String>,
        request: ExpressOrderRequest,
    ) -> Result<ExpressGetPathResponse> {
        self.inner
            .post(
                "cgi-bin/express/business/path/get",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_express_printer(
        &self,
        access_token: impl Into<String>,
    ) -> Result<ExpressGetPrinterResponse> {
        self.inner
            .get(
                "cgi-bin/express/business/printer/getall",
                Some(access_token.into()),
            )
            .await
    }

    pub async fn get_express_quota(
        &self,
        access_token: impl Into<String>,
        delivery_id: impl Into<String>,
        biz_id: impl Into<String>,
    ) -> Result<ExpressGetQuotaResponse> {
        self.inner
            .post(
                "cgi-bin/express/business/quota/get",
                Some(access_token.into()),
                json!({
                    "delivery_id": delivery_id.into(),
                    "biz_id": biz_id.into(),
                }),
            )
            .await
    }

    pub async fn test_update_express_order(
        &self,
        access_token: impl Into<String>,
        request: ExpressTestUpdateOrderRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/express/business/test_update_order",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn update_express_printer(
        &self,
        access_token: impl Into<String>,
        request: ExpressUpdatePrinterRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/express/business/printer/update",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_express_contact(
        &self,
        access_token: impl Into<String>,
        token: impl Into<String>,
        waybill_id: impl Into<String>,
    ) -> Result<ExpressGetContactResponse> {
        self.inner
            .post(
                "cgi-bin/express/delivery/contact/get",
                Some(access_token.into()),
                json!({
                    "token": token.into(),
                    "waybill_id": waybill_id.into(),
                }),
            )
            .await
    }

    pub async fn preview_express_template(
        &self,
        access_token: impl Into<String>,
        request: ExpressRequest,
    ) -> Result<ExpressPreviewTemplateResponse> {
        self.inner
            .post(
                "cgi-bin/express/delivery/template/preview",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn update_express_business(
        &self,
        access_token: impl Into<String>,
        request: ExpressRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/express/delivery/service/business/update",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn update_express_path(
        &self,
        access_token: impl Into<String>,
        request: ExpressRequest,
    ) -> Result<WechatStatusResponse> {
        self.inner
            .post(
                "cgi-bin/express/delivery/path/update",
                Some(access_token.into()),
                request,
            )
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
pub struct MiniProgramPaidUnionIdRequest {
    pub openid: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mch_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub out_trade_no: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgramPaidUnionIdResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub unionid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgramCheckEncryptedDataResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub vaild: Option<bool>,
    #[serde(default)]
    pub create_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgramUploadMediaResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default, rename = "type")]
    pub media_type: Option<String>,
    #[serde(default)]
    pub media_id: Option<String>,
    #[serde(default)]
    pub created_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgramActivityIdResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub activity_id: Option<String>,
    #[serde(default)]
    pub expiration_time: Option<i64>,
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
pub struct ImageAiCropResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub results: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageScanQrCodeResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub code_results: Vec<Value>,
    #[serde(default)]
    pub img_size: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageSuperResolutionResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub media_id: Option<String>,
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
pub struct SoterVerifySignatureRequest {
    pub openid: String,
    pub json_string: String,
    pub json_signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoterVerifySignatureResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub is_ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMarketInvokeRequest {
    pub service: String,
    pub api: String,
    pub client_msg_id: String,
    pub data: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMarketInvokeResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub data: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternetGetUserEncryptKeyRequest {
    pub openid: String,
    pub signature: String,
    pub sig_method: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternetGetUserEncryptKeyResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub key_info_list: Vec<InternetUserEncryptKeyInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternetUserEncryptKeyInfo {
    #[serde(default)]
    pub encrypt_key: Option<String>,
    #[serde(default)]
    pub version: Option<i64>,
    #[serde(default)]
    pub expire_in: Option<i64>,
    #[serde(default)]
    pub iv: Option<String>,
    #[serde(default)]
    pub create_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceSubscribeMessageRequest {
    pub to_openid_list: Vec<String>,
    pub sn: String,
    pub template_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miniprogram_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    pub data: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceSnTicketRequest {
    pub model_id: String,
    pub sn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCreateIotGroupRequest {
    pub group_name: String,
    pub model_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceGetIotGroupInfoRequest {
    pub group_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub model_id: String,
    pub sn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceGroupDeviceListRequest {
    pub group_id: String,
    pub device_list: Vec<DeviceInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceActiveLicenseRequest {
    pub pkg_type: i64,
    pub device_list: Vec<DeviceInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCreateIotGroupResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub group_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceGetIotGroupInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub group_name: Option<String>,
    #[serde(default)]
    pub model_id: Option<String>,
    #[serde(default)]
    pub model_type: Option<String>,
    #[serde(default)]
    pub device_list: Vec<DeviceInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceOperationItem {
    #[serde(default)]
    pub model_id: Option<String>,
    #[serde(default)]
    pub sn: Option<String>,
    #[serde(default)]
    pub errcode: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceGroupOperationResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub device_list: Vec<DeviceOperationItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceLicensePkgListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub pkg_list: Vec<DeviceLicensePkg>,
    #[serde(default)]
    pub max_active_number: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceLicensePkg {
    #[serde(default)]
    pub pkg_id: Option<String>,
    #[serde(default)]
    pub pkg_type: Option<i64>,
    #[serde(default)]
    pub start_time: Option<i64>,
    #[serde(default)]
    pub end_time: Option<i64>,
    #[serde(default)]
    pub pkg_status: Option<i64>,
    #[serde(default)]
    pub used: Option<i64>,
    #[serde(default)]
    pub all: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceActiveLicenseResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub device_list: Vec<DeviceInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceLicenseInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub device_list: Vec<DeviceLicenseInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceLicenseInfo {
    #[serde(default)]
    pub model_id: Option<String>,
    #[serde(default)]
    pub sn: Option<String>,
    #[serde(default)]
    pub expire_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationRequest {
    #[serde(flatten)]
    pub payload: Value,
}

impl OperationRequest {
    pub fn new(payload: Value) -> Self {
        Self { payload }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationDomainInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub requestdomain: Vec<String>,
    #[serde(default)]
    pub wsrequestdomain: Vec<String>,
    #[serde(default)]
    pub uploaddomain: Vec<String>,
    #[serde(default)]
    pub downloaddomain: Vec<String>,
    #[serde(default)]
    pub udpdomain: Vec<String>,
    #[serde(default)]
    pub bizdomain: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationFeedbackResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub list: Vec<Value>,
    #[serde(default)]
    pub total_num: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationGrayReleasePlanResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub gray_release_plan: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationJsErrDetailResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub success: Option<bool>,
    #[serde(default)]
    pub openid: Option<String>,
    #[serde(default)]
    pub data: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationJsErrListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub success: Option<bool>,
    #[serde(default)]
    pub openid: Option<String>,
    #[serde(default)]
    pub data: Vec<Value>,
    #[serde(default, rename = "totalCount")]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationJsErrSearchResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub results: Option<Value>,
    #[serde(default)]
    pub total: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationPerformanceResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub default_time_data: Option<String>,
    #[serde(default)]
    pub compare_time_data: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationSceneListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub scene: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationVersionListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub cvlist: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationRealTimeLogSearchResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub data: Option<Value>,
    #[serde(default)]
    pub list: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchImageSearchRequest {
    pub img: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchImageSearchResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub items: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSiteSearchRequest {
    pub keyword: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_info: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSiteSearchResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub items: Vec<Value>,
    #[serde(default)]
    pub has_next_page: Option<i64>,
    #[serde(default)]
    pub hit_count: Option<i64>,
    #[serde(default)]
    pub next_page_info: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSubmitPagesRequest {
    pub pages: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearbyPoiAddRequest {
    #[serde(flatten)]
    pub payload: Value,
}

impl NearbyPoiAddRequest {
    pub fn new(payload: Value) -> Self {
        Self { payload }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearbyPoiAddResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub data: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearbyPoiListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub data: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearbyPoiShowStatusRequest {
    pub poi_id: String,
    pub status: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginActionRequest {
    pub action: String,
    pub plugin_appid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl PluginActionRequest {
    pub fn apply(plugin_app_id: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            action: "apply".to_string(),
            plugin_appid: plugin_app_id.into(),
            reason: Some(reason.into()),
        }
    }

    pub fn unbind(plugin_app_id: impl Into<String>) -> Self {
        Self {
            action: "unbind".to_string(),
            plugin_appid: plugin_app_id.into(),
            reason: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub plugin_list: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDevApplyListRequest {
    pub action: String,
    pub page: i64,
    pub num: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDevApplyListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub apply_list: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDevApplyStatusRequest {
    pub action: String,
    pub appid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualPaymentOrderRequest {
    pub session_key: String,
    pub product_id: String,
    pub price: i64,
    pub out_trade_no: String,
    pub attach: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualPaymentOrderResponse {
    pub post_body: String,
    pub pay_sign: String,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct B2bPaymentResponse {
    pub sign_data: String,
    pub mode: String,
    pub signature: String,
    pub pay_sig: String,
}

fn virtual_payment_order_post_body(offer_id: &str, request: &VirtualPaymentOrderRequest) -> String {
    format!(
        r#"{{"offerId":"{}","buyQuantity":1,"env":0,"currencyType":"CNY","platform":"android","productId":"{}","goodsPrice":{},"outTradeNo":"{}","attach":"{}"}}"#,
        offer_id, request.product_id, request.price, request.out_trade_no, request.attach
    )
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualPaymentUploadProductsRequest {
    pub env: i64,
    pub upload_item: Vec<VirtualPaymentGoodItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualPaymentGoodItem {
    pub id: String,
    pub name: String,
    pub price: i64,
    #[serde(rename = "remake")]
    pub remark: String,
    pub item_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualPaymentUploadGoodsResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub base_resp: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualPaymentUploadProductSearchResponse {
    #[serde(default)]
    pub cost: Option<i64>,
    #[serde(default)]
    pub end: Option<i64>,
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub progress: Option<i64>,
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default)]
    pub total: Option<i64>,
    #[serde(default)]
    pub upload_item: Vec<VirtualPaymentUploadItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualPaymentUploadItem {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub item_url: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub price: Option<i64>,
    #[serde(default)]
    pub upload_status: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct B2bGetOrderRequest {
    pub mchid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_trade_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct B2bRefundRequest {
    pub mchid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_trade_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    pub out_refund_no: String,
    pub refund_amount: i64,
    pub refund_from: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_reason: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct B2bGetRefundRequest {
    pub mchid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_refund_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct B2bAddProfitSharingAccountRequest {
    pub profit_sharing_relation_type: String,
    pub payee_type: String,
    pub payee_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payee_name: Option<String>,
    pub env: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct B2bDeleteProfitSharingAccountRequest {
    pub payee_type: String,
    pub payee_id: String,
    pub env: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct B2bQueryProfitSharingAccountRequest {
    pub offset: i64,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct B2bCreateProfitSharingOrderRequest {
    pub mchid: String,
    pub out_trade_no: String,
    pub profit_fee: i64,
    pub receiver_type: String,
    pub receiver_account: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct B2bQueryProfitSharingOrderRequest {
    pub mchid: String,
    pub out_trade_no: String,
    pub receiver_type: String,
    pub receiver_account: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct B2bQueryProfitSharingRemainAmountRequest {
    pub mchid: String,
    pub out_trade_no: String,
    pub env: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct B2bFinishProfitSharingOrderRequest {
    pub mchid: String,
    pub out_trade_no: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct B2bRefundProfitSharingRequest {
    pub mchid: String,
    pub out_trade_no: String,
    pub out_refund_no: String,
    pub payee_type: String,
    pub payee_id: String,
    pub refund_amt: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct B2bQueryRefundProfitSharingOrderRequest {
    pub mchid: String,
    pub out_trade_no: String,
    pub out_refund_no: String,
    pub payee_type: String,
    pub payee_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct B2bDownloadBillRequest {
    pub mchid: String,
    pub bill_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct B2bStatusResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct B2bAmount {
    #[serde(default)]
    pub order_amount: Option<i64>,
    #[serde(default)]
    pub payer_amount: Option<i64>,
    #[serde(default)]
    pub refund_amount: Option<i64>,
    #[serde(default)]
    pub currency: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct B2bGetOrderResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub appid: Option<String>,
    #[serde(default)]
    pub mchid: Option<String>,
    #[serde(default)]
    pub out_trade_no: Option<String>,
    #[serde(default)]
    pub order_id: Option<String>,
    #[serde(default)]
    pub pay_status: Option<String>,
    #[serde(default)]
    pub pay_time: Option<String>,
    #[serde(default)]
    pub attach: Option<String>,
    #[serde(default)]
    pub payer_openid: Option<String>,
    #[serde(default)]
    pub amount: Option<B2bAmount>,
    #[serde(default)]
    pub wxpay_transaction_id: Option<String>,
    #[serde(default)]
    pub env: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct B2bRefundResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub refund_id: Option<String>,
    #[serde(default)]
    pub out_refund_no: Option<String>,
    #[serde(default)]
    pub order_id: Option<String>,
    #[serde(default)]
    pub out_trade_no: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct B2bGetRefundResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub refund_id: Option<String>,
    #[serde(default)]
    pub out_refund_no: Option<String>,
    #[serde(default)]
    pub order_id: Option<String>,
    #[serde(default)]
    pub out_trade_no: Option<String>,
    #[serde(default)]
    pub create_time: Option<String>,
    #[serde(default)]
    pub refund_time: Option<String>,
    #[serde(default)]
    pub refund_status: Option<String>,
    #[serde(default)]
    pub refund_desc: Option<String>,
    #[serde(default)]
    pub amount: Option<B2bAmount>,
    #[serde(default)]
    pub wxpay_refund_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct B2bProfitSharingAccountInfo {
    #[serde(default)]
    pub sharing_account_type: Option<String>,
    #[serde(default)]
    pub sharing_account: Option<String>,
    #[serde(default)]
    pub add_time: Option<String>,
    #[serde(default)]
    pub update_time: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct B2bQueryProfitSharingAccountResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub account_list: Vec<B2bProfitSharingAccountInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct B2bProfitSharingOrderResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub order_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct B2bProfitSharingRemainAmountResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub remain_amt: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct B2bDownloadBillResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub success_bill_url: Option<String>,
    #[serde(default)]
    pub refund_bill_url: Option<String>,
    #[serde(default)]
    pub all_bill_url: Option<String>,
    #[serde(default)]
    pub fund_bill_url: Option<String>,
    #[serde(default)]
    pub ended_day_avail_amt: Option<i64>,
    #[serde(default)]
    pub ended_day_frozen_amt: Option<i64>,
    #[serde(default)]
    pub ended_day_total_amt: Option<i64>,
    #[serde(default)]
    pub profit_sharing_bill_url: Option<String>,
    #[serde(default)]
    pub profit_refund_bill_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaVideoMediaUploadByUrlRequest {
    pub media_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_url: Option<String>,
    pub media_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_context: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MiniDramaVideoMediaUploadByFileRequest {
    pub media_name: String,
    pub media_type: String,
    pub media_data: Vec<u8>,
    pub cover_type: String,
    pub cover_data: Option<Vec<u8>>,
}

impl MiniDramaVideoMediaUploadByFileRequest {
    pub fn new(media_name: impl Into<String>, media_data: Vec<u8>) -> Self {
        Self {
            media_name: media_name.into(),
            media_type: "MP4".to_string(),
            media_data,
            cover_type: "JPEG".to_string(),
            cover_data: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MiniDramaVideoChunkUploadRequest {
    pub upload_id: String,
    pub part_number: i64,
    pub resource_type: i64,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaVideoApplyChunkUploadRequest {
    pub media_name: String,
    pub media_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_context: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaPartInfo {
    pub part_number: i64,
    pub etag: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaVideoChunkUploadCompleteRequest {
    pub upload_id: String,
    pub media_part_infos: Vec<MiniDramaPartInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub cover_part_infos: Vec<MiniDramaPartInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MiniDramaMediaListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drama_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}

impl MiniDramaMediaListRequest {
    fn normalized(mut self) -> Self {
        if let Some(limit) = self.limit {
            if limit > 100 {
                self.limit = Some(100);
            }
        }
        if let Some(offset) = self.offset {
            if offset < 0 {
                self.offset = Some(0);
            }
        }
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaMediaLinkRequest {
    pub media_id: i64,
    pub t: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expr: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rlimit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "whref")]
    pub href: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bkref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaSubmitAuditRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drama_id: Option<i64>,
    pub name: String,
    pub media_count: i64,
    pub media_id_list: Vec<i64>,
    pub description: String,
    pub recommendations: String,
    pub cover_material_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_poster_material_id: Option<String>,
    pub producer: String,
    pub authorized_material_id: String,
    pub qualification_type: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualification_certificate_material_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_commitment_letter_material_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_of_production: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expedited: Option<i64>,
    pub actor_list: MiniDramaActorList,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub replace_media_list: Vec<MiniDramaReplaceInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaReplaceInfo {
    pub old: i64,
    pub new: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MiniDramaListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}

impl MiniDramaListRequest {
    fn normalized(mut self) -> Self {
        if let Some(limit) = self.limit {
            if limit > 100 {
                self.limit = Some(100);
            }
        }
        if let Some(offset) = self.offset {
            if offset < 0 {
                self.offset = Some(0);
            }
        }
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaSubmitReplaceAuditRequest {
    pub drama_id: i64,
    pub replace_media_list: Vec<MiniDramaReplaceInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaReplaceAuditedMediaRequest {
    pub drama_id: i64,
    pub old_media_id: i64,
    pub new_media_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaUpdateInfoRequest {
    pub drama_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_material_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendations: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_poster_material_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternate_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_list: Option<MiniDramaActorList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualification_type: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualification_certificate_material_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_of_production: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_commitment_letter_material_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaActorList {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub actor: Vec<MiniDramaActor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaActor {
    pub name: String,
    pub photo_material_id: String,
    pub role: String,
    pub profile: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaAuditInfoRequest {
    pub drama_id: i64,
    pub audit_type: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaCdnInfoRequest {
    pub start_time: i64,
    pub end_time: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_interval: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaAuthorizationRequest {
    pub authorized_appid: String,
    pub drama_id: Vec<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authz_expire_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaAuthorizationSearchRequest {
    pub drama_id: i64,
    #[serde(flatten)]
    pub page: MiniDramaListRequest,
}

impl MiniDramaAuthorizationSearchRequest {
    fn normalized(mut self) -> Self {
        self.page = self.page.normalized();
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaAuthorizedBySearchRequest {
    pub authorizer_appid: String,
    #[serde(flatten)]
    pub page: MiniDramaListRequest,
}

impl MiniDramaAuthorizedBySearchRequest {
    fn normalized(mut self) -> Self {
        self.page = self.page.normalized();
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaAccountAuthorizationRequest {
    pub authorized_appid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authz_expire_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaSetFlushDramaRequest {
    pub list: Vec<MiniDramaFlushDramaInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaFlushDramaInfo {
    pub src_appid: String,
    pub drama_id: String,
    pub drama_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaVideoMediaUploadByUrlResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub task_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaVideoMediaUploadByFileResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub media_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaVideoMediaTaskResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub task_info: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaVideoApplyChunkUploadResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub upload_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaVideoChunkUploadResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub etag: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaVideoChunkUploadCompleteResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub media_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaMediaListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub media_info_list: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaMediaInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub media_info: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaMediaLinkResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub media_info: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaSubmitAuditResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub drama_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub drama_info_list: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub drama_info: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaAuditInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub audit_detail: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaCdnUsageResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub data_interval: Option<i64>,
    #[serde(default)]
    pub item_list: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaCdnLogsResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub domestic_cdn_logs: Vec<Value>,
    #[serde(default)]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaPackageListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub package_list: Vec<Value>,
    #[serde(default)]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaAuthorizationResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub result: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaAuthorizationSearchResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub objects: Vec<Value>,
    #[serde(default)]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaAccountAuthorizationSearchResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub objects: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmediateDeliveryRequest {
    #[serde(flatten)]
    pub payload: Value,
}

impl ImmediateDeliveryRequest {
    pub fn new(payload: Value) -> Self {
        Self { payload }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmediateDeliveryGetOrderRequest {
    pub shopid: String,
    pub shop_order_id: String,
    pub shop_no: String,
    pub delivery_sign: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmediateDeliveryBindAccountResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub shop_list: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmediateDeliveryDeliveryListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub list: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmediateDeliveryCancelOrderResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub deduct_fee: Option<i64>,
    #[serde(default)]
    pub desc: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmediateDeliveryGetOrderResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub order_status: Option<i64>,
    #[serde(default)]
    pub waybill_id: Option<String>,
    #[serde(default)]
    pub rider_name: Option<String>,
    #[serde(default)]
    pub rider_phone: Option<String>,
    #[serde(default)]
    pub rider_lng: Option<f64>,
    #[serde(default)]
    pub rider_lat: Option<f64>,
    #[serde(default)]
    pub reach_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmediateDeliveryPreAddOrderResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub fee: Option<i64>,
    #[serde(default)]
    pub deliverfee: Option<String>,
    #[serde(default)]
    pub couponfee: Option<String>,
    #[serde(default)]
    pub tips: Option<String>,
    #[serde(default)]
    pub insurancfee: Option<f64>,
    #[serde(default)]
    pub distance: Option<f64>,
    #[serde(default)]
    pub dispatch_duration: Option<i64>,
    #[serde(default)]
    pub delivery_token: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmediateDeliveryPreCancelOrderResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub deduct_fee: Option<i64>,
    #[serde(default)]
    pub desc: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmediateDeliveryReOrderResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub fee: Option<i64>,
    #[serde(default)]
    pub deliverfee: Option<String>,
    #[serde(default)]
    pub couponfee: Option<String>,
    #[serde(default)]
    pub tips: Option<String>,
    #[serde(default, rename = "insurancfee")]
    pub insurance_fee: Option<f64>,
    #[serde(default)]
    pub distance: Option<f64>,
    #[serde(default)]
    pub waybill_id: Option<i64>,
    #[serde(default)]
    pub order_status: Option<i64>,
    #[serde(default)]
    pub finish_code: Option<i64>,
    #[serde(default)]
    pub pickup_code: Option<i64>,
    #[serde(default)]
    pub dispatch_duration: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressRequest {
    #[serde(flatten)]
    pub payload: Value,
}

impl ExpressRequest {
    pub fn new(payload: Value) -> Self {
        Self { payload }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressBindAccountRequest {
    #[serde(rename = "type")]
    pub action_type: String,
    pub biz_id: String,
    pub delivery_id: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressOrderRequest {
    pub order_id: String,
    pub openid: String,
    pub delivery_id: String,
    pub waybill_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressGetOrderRequest {
    pub order_id: String,
    pub openid: String,
    pub delivery_id: String,
    pub waybill_id: String,
    pub print_type: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressTestUpdateOrderRequest {
    pub biz_id: String,
    pub order_id: String,
    pub delivery_id: String,
    pub waybill_id: String,
    pub action_time: i64,
    pub action_type: i64,
    pub action_msg: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressUpdatePrinterRequest {
    pub openid: String,
    pub update_type: String,
    pub tagid_list: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressAddOrderResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub delivery_resultcode: Option<i64>,
    #[serde(default)]
    pub delivery_resultmsg: Option<String>,
    #[serde(default)]
    pub order_id: Option<String>,
    #[serde(default)]
    pub waybill_id: Option<String>,
    #[serde(default)]
    pub waybill_data: Vec<ExpressWaybillData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressWaybillData {
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressOrderSummary {
    #[serde(default)]
    pub order_id: Option<String>,
    #[serde(default)]
    pub waybill_id: Option<String>,
    #[serde(default)]
    pub delivery_id: Option<String>,
    #[serde(default)]
    pub order_status: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressBatchOrderListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub order_list: Vec<ExpressOrderSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressCancelOrderResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub delivery_resultcode: Option<i64>,
    #[serde(default)]
    pub delivery_resultmsg: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressAccountListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub count: Option<i64>,
    #[serde(default)]
    pub list: Vec<ExpressAccount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressAccount {
    #[serde(default)]
    pub biz_id: Option<String>,
    #[serde(default)]
    pub delivery_id: Option<String>,
    #[serde(default)]
    pub delivery_name: Option<String>,
    #[serde(default)]
    pub account_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressDeliveryListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub count: Option<i64>,
    #[serde(default)]
    pub data: Vec<ExpressDelivery>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressDelivery {
    #[serde(default)]
    pub delivery_id: Option<String>,
    #[serde(default)]
    pub delivery_name: Option<String>,
    #[serde(default)]
    pub can_use_cash: Option<i64>,
    #[serde(default)]
    pub can_get_quota: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressGetOrderResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub print_html: Option<String>,
    #[serde(default)]
    pub waybill_data: Vec<ExpressWaybillData>,
    #[serde(default)]
    pub delivery_id: Option<String>,
    #[serde(default)]
    pub waybill_id: Option<String>,
    #[serde(default)]
    pub order_id: Option<String>,
    #[serde(default)]
    pub order_status: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressGetPathResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub openid: Option<String>,
    #[serde(default)]
    pub delivery_id: Option<String>,
    #[serde(default)]
    pub waybill_id: Option<String>,
    #[serde(default)]
    pub path_item_num: Option<i64>,
    #[serde(default)]
    pub path_item_list: Vec<ExpressPathItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressPathItem {
    #[serde(default)]
    pub action_time: Option<i64>,
    #[serde(default)]
    pub action_type: Option<i64>,
    #[serde(default)]
    pub action_msg: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressGetPrinterResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub count: Option<Value>,
    #[serde(default)]
    pub openid: Vec<String>,
    #[serde(default)]
    pub tagid_list: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressGetQuotaResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub quota_num: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressGetContactResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub waybill_id: Option<String>,
    #[serde(default)]
    pub sender: Vec<ExpressContact>,
    #[serde(default)]
    pub receiver: Vec<ExpressContact>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressContact {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub tel: Option<String>,
    #[serde(default)]
    pub mobile: Option<String>,
    #[serde(default)]
    pub company: Option<String>,
    #[serde(default)]
    pub post_code: Option<String>,
    #[serde(default)]
    pub country: Option<String>,
    #[serde(default)]
    pub province: Option<String>,
    #[serde(default)]
    pub city: Option<String>,
    #[serde(default)]
    pub area: Option<String>,
    #[serde(default)]
    pub address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressPreviewTemplateResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub waybill_id: Option<String>,
    #[serde(default)]
    pub rendered_waybill_template: Option<String>,
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
pub struct MiniProgramLiveGoodsWarehouseResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub goods: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgramLiveFollowersResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub followers: Vec<Value>,
    #[serde(default)]
    pub page_break: Option<Value>,
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

    use super::*;
    use super::{
        virtual_payment_order_post_body, B2bAddProfitSharingAccountRequest, B2bDownloadBillRequest,
        B2bDownloadBillResponse, B2bGetOrderRequest, B2bGetOrderResponse, B2bGetRefundRequest,
        B2bGetRefundResponse, B2bProfitSharingOrderResponse, B2bProfitSharingRemainAmountResponse,
        B2bQueryProfitSharingAccountRequest, B2bQueryProfitSharingAccountResponse,
        B2bRefundRequest, B2bRefundResponse, Code2SessionResponse, CreateQrCodeRequest,
        CustomerServiceMessage, DataCubeDateRange, DeviceActiveLicenseRequest,
        DeviceActiveLicenseResponse, DeviceCreateIotGroupRequest, DeviceCreateIotGroupResponse,
        DeviceGetIotGroupInfoRequest, DeviceGetIotGroupInfoResponse, DeviceGroupDeviceListRequest,
        DeviceGroupOperationResponse, DeviceInfo, DeviceLicenseInfoResponse,
        DeviceLicensePkgListResponse, DeviceSnTicketRequest, DeviceSubscribeMessageRequest,
        ExpressAccountListResponse, ExpressAddOrderResponse, ExpressBatchOrderListResponse,
        ExpressBindAccountRequest, ExpressCancelOrderResponse, ExpressDeliveryListResponse,
        ExpressGetContactResponse, ExpressGetOrderRequest, ExpressGetOrderResponse,
        ExpressGetPathResponse, ExpressGetPrinterResponse, ExpressGetQuotaResponse,
        ExpressOrderRequest, ExpressPreviewTemplateResponse, ExpressRequest,
        ExpressTestUpdateOrderRequest, ExpressUpdatePrinterRequest, ImageAiCropResponse,
        ImageScanQrCodeResponse, ImageSuperResolutionResponse,
        ImmediateDeliveryBindAccountResponse, ImmediateDeliveryCancelOrderResponse,
        ImmediateDeliveryDeliveryListResponse, ImmediateDeliveryGetOrderRequest,
        ImmediateDeliveryGetOrderResponse, ImmediateDeliveryPreAddOrderResponse,
        ImmediateDeliveryPreCancelOrderResponse, ImmediateDeliveryReOrderResponse,
        ImmediateDeliveryRequest, InternetGetUserEncryptKeyRequest,
        InternetGetUserEncryptKeyResponse, JumpWxa, LiveInfoRequest, LiveRoomRequest,
        NearbyPoiAddRequest, NearbyPoiAddResponse, NearbyPoiListResponse,
        NearbyPoiShowStatusRequest, OcrBankcardResponse, OcrBusinessLicenseResponse,
        OcrDrivingLicenseResponse, OcrIdCardResponse, OcrPrintedTextResponse,
        OcrVehicleLicenseResponse, OperationDomainInfoResponse, OperationFeedbackResponse,
        OperationGrayReleasePlanResponse, OperationJsErrDetailResponse, OperationJsErrListResponse,
        OperationJsErrSearchResponse, OperationPerformanceResponse,
        OperationRealTimeLogSearchResponse, OperationRequest, OperationSceneListResponse,
        OperationVersionListResponse, PhoneNumberResponse, PluginActionRequest,
        PluginDevApplyListRequest, PluginDevApplyListResponse, PluginDevApplyStatusRequest,
        PluginListResponse, RiskControlGetUserRiskRankRequest, RiskControlGetUserRiskRankResponse,
        SearchImageSearchRequest, SearchImageSearchResponse, SearchSiteSearchRequest,
        SearchSiteSearchResponse, SearchSubmitPagesRequest, SecurityMsgSecCheckRequest,
        ServiceMarketInvokeRequest, ServiceMarketInvokeResponse, SoterVerifySignatureRequest,
        SoterVerifySignatureResponse, SubscribeMessageRequest, UrlSchemeGenerateRequest,
        VirtualPaymentGoodItem, VirtualPaymentOrderRequest,
        VirtualPaymentUploadProductSearchResponse, VirtualPaymentUploadProductsRequest,
        WxaSecConfirmReceiveRequest, WxaSecOrderKey, WxaSecOrderListRequest,
        WxaSecOrderListResponse, WxaSecOrderQuery, WxaSecOrderResponse, WxaSecPayTimeRange,
        WxaSecPayer, WxaSecShippingContact, WxaSecShippingInfo, WxaSecSpecialOrderRequest,
        WxaSecSubOrderShippingInfo, WxaSecTradeManagedResponse,
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

        let upload: MiniProgramUploadMediaResponse = serde_json::from_value(json!({
            "type": "image",
            "media_id": "media",
            "created_at": 1800000000
        }))
        .unwrap();
        assert_eq!(upload.media_type.as_deref(), Some("image"));
        assert_eq!(upload.media_id.as_deref(), Some("media"));
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

        let performance = json!({
            "cost_time_type": 1,
            "default_start_time": 1,
            "default_end_time": 2,
            "device": 1,
            "networktype": 1
        });
        assert_eq!(performance["cost_time_type"], 1);
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
    fn serializes_soter_service_market_and_internet_requests() {
        let soter = serde_json::to_value(SoterVerifySignatureRequest {
            openid: "openid".to_string(),
            json_string: "{\"challenge\":\"abc\"}".to_string(),
            json_signature: "signature".to_string(),
        })
        .unwrap();
        assert_eq!(soter["openid"], "openid");
        assert_eq!(soter["json_string"], "{\"challenge\":\"abc\"}");
        assert_eq!(soter["json_signature"], "signature");

        let service = serde_json::to_value(ServiceMarketInvokeRequest {
            service: "wx79ac3de8be320b71".to_string(),
            api: "OcrAllInOne".to_string(),
            client_msg_id: "client-msg-id".to_string(),
            data: json!({
                "img_url": "https://example.com/id-card.jpg",
                "data_type": 3
            }),
        })
        .unwrap();
        assert_eq!(service["service"], "wx79ac3de8be320b71");
        assert_eq!(service["api"], "OcrAllInOne");
        assert_eq!(service["client_msg_id"], "client-msg-id");
        assert_eq!(service["data"]["data_type"], 3);

        let internet = serde_json::to_value(InternetGetUserEncryptKeyRequest {
            openid: "openid".to_string(),
            signature: "signature".to_string(),
            sig_method: "hmac_sha256".to_string(),
        })
        .unwrap();
        assert_eq!(internet["openid"], "openid");
        assert_eq!(internet["signature"], "signature");
        assert_eq!(internet["sig_method"], "hmac_sha256");
    }

    #[test]
    fn deserializes_soter_service_market_and_internet_responses() {
        let soter: SoterVerifySignatureResponse = serde_json::from_value(json!({
            "errcode": 0,
            "is_ok": true
        }))
        .unwrap();
        assert_eq!(soter.errcode, Some(0));
        assert_eq!(soter.is_ok, Some(true));

        let service: ServiceMarketInvokeResponse = serde_json::from_value(json!({
            "errcode": 0,
            "data": {
                "result": "ok",
                "score": 99
            }
        }))
        .unwrap();
        assert_eq!(service.data.expect("data")["score"], 99);

        let service_string: ServiceMarketInvokeResponse = serde_json::from_value(json!({
            "errcode": 0,
            "data": "{\"result\":\"ok\"}"
        }))
        .unwrap();
        assert_eq!(service_string.data, Some(json!("{\"result\":\"ok\"}")));

        let encrypt_key: InternetGetUserEncryptKeyResponse = serde_json::from_value(json!({
            "errcode": 0,
            "key_info_list": [{
                "encrypt_key": "encrypt-key",
                "version": 1,
                "expire_in": 7200,
                "iv": "iv",
                "create_time": 1800000000
            }]
        }))
        .unwrap();
        assert_eq!(
            encrypt_key.key_info_list[0].encrypt_key.as_deref(),
            Some("encrypt-key")
        );
        assert_eq!(encrypt_key.key_info_list[0].version, Some(1));
        assert_eq!(encrypt_key.key_info_list[0].expire_in, Some(7200));
    }

    #[test]
    fn serializes_device_requests() {
        let message = serde_json::to_value(DeviceSubscribeMessageRequest {
            to_openid_list: vec!["openid".to_string()],
            sn: "sn".to_string(),
            template_id: "template-id".to_string(),
            page: Some("pages/device".to_string()),
            miniprogram_state: Some("formal".to_string()),
            lang: Some("zh_CN".to_string()),
            data: json!({
                "thing1": { "value": "online" }
            }),
        })
        .unwrap();
        assert_eq!(message["to_openid_list"][0], "openid");
        assert_eq!(message["template_id"], "template-id");
        assert_eq!(message["data"]["thing1"]["value"], "online");

        let ticket = serde_json::to_value(DeviceSnTicketRequest {
            model_id: "model-id".to_string(),
            sn: "sn".to_string(),
        })
        .unwrap();
        assert_eq!(ticket["model_id"], "model-id");
        assert_eq!(ticket["sn"], "sn");

        let create_group = serde_json::to_value(DeviceCreateIotGroupRequest {
            group_name: "Living Room".to_string(),
            model_id: "model-id".to_string(),
        })
        .unwrap();
        assert_eq!(create_group["group_name"], "Living Room");

        let get_group = serde_json::to_value(DeviceGetIotGroupInfoRequest {
            group_id: "group-id".to_string(),
        })
        .unwrap();
        assert_eq!(get_group["group_id"], "group-id");

        let device = DeviceInfo {
            model_id: "model-id".to_string(),
            sn: "sn".to_string(),
        };
        let group_devices = serde_json::to_value(DeviceGroupDeviceListRequest {
            group_id: "group-id".to_string(),
            device_list: vec![device.clone()],
        })
        .unwrap();
        assert_eq!(group_devices["device_list"][0]["model_id"], "model-id");

        let active_license = serde_json::to_value(DeviceActiveLicenseRequest {
            pkg_type: 1,
            device_list: vec![device],
        })
        .unwrap();
        assert_eq!(active_license["pkg_type"], 1);
        assert_eq!(active_license["device_list"][0]["sn"], "sn");
    }

    #[test]
    fn deserializes_device_responses() {
        let create_group: DeviceCreateIotGroupResponse = serde_json::from_value(json!({
            "errcode": 0,
            "group_id": "group-id"
        }))
        .unwrap();
        assert_eq!(create_group.group_id.as_deref(), Some("group-id"));

        let group_info: DeviceGetIotGroupInfoResponse = serde_json::from_value(json!({
            "errcode": 0,
            "group_name": "Living Room",
            "model_id": "model-id",
            "model_type": "device",
            "device_list": [{ "model_id": "model-id", "sn": "sn" }]
        }))
        .unwrap();
        assert_eq!(group_info.group_name.as_deref(), Some("Living Room"));
        assert_eq!(group_info.device_list[0].sn, "sn");

        let add: DeviceGroupOperationResponse = serde_json::from_value(json!({
            "errcode": 0,
            "device_list": [{ "model_id": "model-id", "sn": "sn", "errcode": 0 }]
        }))
        .unwrap();
        assert_eq!(add.device_list[0].errcode, Some(0));

        let pkg: DeviceLicensePkgListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "max_active_number": 100,
            "pkg_list": [{
                "pkg_id": "pkg-id",
                "pkg_type": 1,
                "start_time": 1800000000,
                "end_time": 1800100000,
                "pkg_status": 1,
                "used": 2,
                "all": 10
            }]
        }))
        .unwrap();
        assert_eq!(pkg.max_active_number, Some(100));
        assert_eq!(pkg.pkg_list[0].pkg_id.as_deref(), Some("pkg-id"));
        assert_eq!(pkg.pkg_list[0].all, Some(10));

        let active: DeviceActiveLicenseResponse = serde_json::from_value(json!({
            "errcode": 0,
            "device_list": [{ "model_id": "model-id", "sn": "sn" }]
        }))
        .unwrap();
        assert_eq!(active.device_list[0].model_id, "model-id");

        let info: DeviceLicenseInfoResponse = serde_json::from_value(json!({
            "errcode": 0,
            "device_list": [{
                "model_id": "model-id",
                "sn": "sn",
                "expire_time": 1800100000
            }]
        }))
        .unwrap();
        assert_eq!(info.device_list[0].expire_time, Some(1_800_100_000));
    }

    #[test]
    fn serializes_operation_request() {
        let request = serde_json::to_value(OperationRequest::new(json!({
            "start_time": 1_800_000_000,
            "end_time": 1_800_003_600,
            "errmsg_keyword": "TypeError",
            "openid": "openid"
        })))
        .unwrap();

        assert_eq!(request["start_time"], 1_800_000_000);
        assert_eq!(request["errmsg_keyword"], "TypeError");
        assert_eq!(request["openid"], "openid");
    }

    #[test]
    fn deserializes_operation_responses() {
        let domain: OperationDomainInfoResponse = serde_json::from_value(json!({
            "errcode": 0,
            "requestdomain": ["https://api.example.com"],
            "wsrequestdomain": ["wss://ws.example.com"],
            "uploaddomain": ["https://upload.example.com"],
            "downloaddomain": ["https://download.example.com"],
            "udpdomain": ["udp.example.com"],
            "bizdomain": ["https://biz.example.com"]
        }))
        .unwrap();
        assert_eq!(domain.requestdomain[0], "https://api.example.com");
        assert_eq!(domain.wsrequestdomain[0], "wss://ws.example.com");

        let feedback: OperationFeedbackResponse = serde_json::from_value(json!({
            "errcode": 0,
            "list": [{ "record_id": 1, "content": "feedback" }],
            "total_num": 1
        }))
        .unwrap();
        assert_eq!(feedback.total_num, Some(1));
        assert_eq!(feedback.list[0]["content"], "feedback");

        let gray: OperationGrayReleasePlanResponse = serde_json::from_value(json!({
            "errcode": 0,
            "gray_release_plan": { "status": 1, "gray_percentage": 30 }
        }))
        .unwrap();
        assert_eq!(
            gray.gray_release_plan.expect("gray_release_plan")["gray_percentage"],
            30
        );

        let detail: OperationJsErrDetailResponse = serde_json::from_value(json!({
            "errcode": 0,
            "success": true,
            "openid": "openid",
            "data": [{ "message": "TypeError" }]
        }))
        .unwrap();
        assert_eq!(detail.success, Some(true));
        assert_eq!(detail.data[0]["message"], "TypeError");

        let list: OperationJsErrListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "success": true,
            "openid": "openid",
            "data": [{ "count": 3 }],
            "totalCount": 3
        }))
        .unwrap();
        assert_eq!(list.total_count, Some(3));
        assert_eq!(list.data[0]["count"], 3);

        let search: OperationJsErrSearchResponse = serde_json::from_value(json!({
            "errcode": 0,
            "results": { "items": [{ "message": "TypeError" }] },
            "total": 1
        }))
        .unwrap();
        assert_eq!(search.total, Some(1));
        assert_eq!(
            search.results.expect("results")["items"][0]["message"],
            "TypeError"
        );

        let performance: OperationPerformanceResponse = serde_json::from_value(json!({
            "errcode": 0,
            "default_time_data": "[1,2]",
            "compare_time_data": "[2,3]"
        }))
        .unwrap();
        assert_eq!(performance.default_time_data.as_deref(), Some("[1,2]"));

        let scene: OperationSceneListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "scene": [{ "name": "chat", "value": 1007 }]
        }))
        .unwrap();
        assert_eq!(scene.scene[0]["value"], 1007);

        let version: OperationVersionListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "cvlist": [{ "version": "8.0.0", "percentage": 50 }]
        }))
        .unwrap();
        assert_eq!(version.cvlist[0]["version"], "8.0.0");

        let real_time: OperationRealTimeLogSearchResponse = serde_json::from_value(json!({
            "errcode": 0,
            "data": { "total": 1 },
            "list": [{ "level": "error", "message": "failed" }]
        }))
        .unwrap();
        assert_eq!(real_time.data.expect("data")["total"], 1);
        assert_eq!(real_time.list[0]["level"], "error");
    }

    #[test]
    fn serializes_search_requests() {
        let image_search = serde_json::to_value(SearchImageSearchRequest {
            img: vec![json!({
                "name": "goods.jpg",
                "value": "base64-image"
            })],
        })
        .unwrap();
        assert_eq!(image_search["img"][0]["name"], "goods.jpg");
        assert_eq!(image_search["img"][0]["value"], "base64-image");

        let site_search = serde_json::to_value(SearchSiteSearchRequest {
            keyword: "coffee".to_string(),
            next_page_info: None,
        })
        .unwrap();
        assert_eq!(site_search["keyword"], "coffee");
        assert!(site_search.get("next_page_info").is_none());

        let submit_pages = serde_json::to_value(SearchSubmitPagesRequest {
            pages: vec![json!({
                "path": "pages/goods/detail",
                "query": "id=1"
            })],
        })
        .unwrap();
        assert_eq!(submit_pages["pages"][0]["path"], "pages/goods/detail");
        assert_eq!(submit_pages["pages"][0]["query"], "id=1");
    }

    #[test]
    fn deserializes_search_responses() {
        let image_search: SearchImageSearchResponse = serde_json::from_value(json!({
            "errcode": 0,
            "items": [{ "img_url": "https://example.com/goods.jpg", "score": 98 }]
        }))
        .unwrap();
        assert_eq!(image_search.errcode, Some(0));
        assert_eq!(image_search.items[0]["score"], 98);

        let site_search: SearchSiteSearchResponse = serde_json::from_value(json!({
            "errcode": 0,
            "items": [{ "title": "Coffee", "path": "pages/goods/detail" }],
            "has_next_page": 1,
            "hit_count": 10,
            "next_page_info": "cursor"
        }))
        .unwrap();
        assert_eq!(site_search.items[0]["title"], "Coffee");
        assert_eq!(site_search.has_next_page, Some(1));
        assert_eq!(site_search.hit_count, Some(10));
        assert_eq!(site_search.next_page_info.as_deref(), Some("cursor"));
    }

    #[test]
    fn serializes_nearby_poi_requests() {
        let add = serde_json::to_value(NearbyPoiAddRequest::new(json!({
            "store_name": "Roze Coffee",
            "address": "1 Rust Road",
            "pic_list": ["media-id"],
            "service_infos": [{ "id": 1, "type": 2 }]
        })))
        .unwrap();
        assert_eq!(add["store_name"], "Roze Coffee");
        assert_eq!(add["pic_list"][0], "media-id");
        assert_eq!(add["service_infos"][0]["type"], 2);

        let show_status = serde_json::to_value(NearbyPoiShowStatusRequest {
            poi_id: "poi-id".to_string(),
            status: 1,
        })
        .unwrap();
        assert_eq!(show_status, json!({ "poi_id": "poi-id", "status": 1 }));
    }

    #[test]
    fn deserializes_nearby_poi_responses() {
        let add: NearbyPoiAddResponse = serde_json::from_value(json!({
            "errcode": 0,
            "data": [{ "poi_id": "poi-id", "audit_id": "audit-id" }]
        }))
        .unwrap();
        assert_eq!(add.errcode, Some(0));
        assert_eq!(add.data[0]["poi_id"], "poi-id");

        let list: NearbyPoiListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "data": {
                "left_apply_num": 9,
                "max_apply_num": 10,
                "data": [{ "poi_id": "poi-id", "store_name": "Roze Coffee" }]
            }
        }))
        .unwrap();
        let data = list.data.expect("data");
        assert_eq!(data["left_apply_num"], 9);
        assert_eq!(data["data"][0]["store_name"], "Roze Coffee");
    }

    #[test]
    fn serializes_plugin_requests() {
        let apply = serde_json::to_value(PluginActionRequest::apply(
            "plugin-appid",
            "needed for checkout",
        ))
        .unwrap();
        assert_eq!(apply["action"], "apply");
        assert_eq!(apply["plugin_appid"], "plugin-appid");
        assert_eq!(apply["reason"], "needed for checkout");

        let unbind = serde_json::to_value(PluginActionRequest::unbind("plugin-appid")).unwrap();
        assert_eq!(unbind["action"], "unbind");
        assert_eq!(unbind["plugin_appid"], "plugin-appid");
        assert!(unbind.get("reason").is_none());

        let list = serde_json::to_value(PluginDevApplyListRequest {
            action: "dev_apply_list".to_string(),
            page: 1,
            num: 20,
        })
        .unwrap();
        assert_eq!(list["action"], "dev_apply_list");
        assert_eq!(list["page"], 1);
        assert_eq!(list["num"], 20);

        let status = serde_json::to_value(PluginDevApplyStatusRequest {
            action: "dev_agree".to_string(),
            appid: "wxappid".to_string(),
            reason: None,
        })
        .unwrap();
        assert_eq!(status["action"], "dev_agree");
        assert_eq!(status["appid"], "wxappid");
        assert!(status.get("reason").is_none());
    }

    #[test]
    fn deserializes_plugin_responses() {
        let list: PluginListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "plugin_list": [{ "appid": "plugin-appid", "status": 1 }]
        }))
        .unwrap();
        assert_eq!(list.errcode, Some(0));
        assert_eq!(list.plugin_list[0]["appid"], "plugin-appid");

        let apply_list: PluginDevApplyListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "apply_list": [{ "appid": "wxappid", "reason": "need plugin" }]
        }))
        .unwrap();
        assert_eq!(apply_list.apply_list[0]["reason"], "need plugin");
    }

    #[test]
    fn builds_virtual_payment_order_post_body_and_signatures() {
        let request = VirtualPaymentOrderRequest {
            session_key: "session-key".to_string(),
            product_id: "coins_100".to_string(),
            price: 100,
            out_trade_no: "out-trade-no".to_string(),
            attach: "metadata".to_string(),
        };
        let post_body = virtual_payment_order_post_body("offer-id", &request);

        assert_eq!(
            post_body,
            r#"{"offerId":"offer-id","buyQuantity":1,"env":0,"currencyType":"CNY","platform":"android","productId":"coins_100","goodsPrice":100,"outTradeNo":"out-trade-no","attach":"metadata"}"#
        );
        assert_eq!(
            crate::crypto::hmac_sha256_hex(
                b"app-key",
                format!("requestVirtualPayment&{post_body}").as_bytes()
            )
            .unwrap(),
            "b49d01ba290228d068311a5f8f86d7f1c0ed09532d5acbcf0be1007b9927085a"
        );
        assert_eq!(
            crate::crypto::hmac_sha256_hex(b"session-key", post_body.as_bytes()).unwrap(),
            "548bded6ea02f3e30d23a7ae30e9883645d5f73e552322ccfb86af61d068f819"
        );
    }

    #[test]
    fn serializes_virtual_payment_upload_products_request() {
        let value = serde_json::to_value(VirtualPaymentUploadProductsRequest {
            env: 0,
            upload_item: vec![VirtualPaymentGoodItem {
                id: "coins_100".to_string(),
                name: "100 coins".to_string(),
                price: 100,
                remark: "coin pack".to_string(),
                item_url: "https://example.com/coin.png".to_string(),
            }],
        })
        .unwrap();

        assert_eq!(value["env"], 0);
        assert_eq!(value["upload_item"][0]["id"], "coins_100");
        assert_eq!(value["upload_item"][0]["remake"], "coin pack");
        assert_eq!(
            value["upload_item"][0]["item_url"],
            "https://example.com/coin.png"
        );
    }

    #[test]
    fn deserializes_virtual_payment_upload_search_response() {
        let response: VirtualPaymentUploadProductSearchResponse = serde_json::from_value(json!({
            "errcode": 0,
            "errmsg": "ok",
            "status": 2,
            "progress": 100,
            "upload_item": [{
                "id": "coins_100",
                "item_url": "https://example.com/coin.png",
                "name": "100 coins",
                "price": 100,
                "upload_status": 0,
                "errmsg": ""
            }]
        }))
        .unwrap();

        assert_eq!(response.errcode, Some(0));
        assert_eq!(response.status, Some(2));
        assert_eq!(response.upload_item[0].id.as_deref(), Some("coins_100"));
        assert_eq!(response.upload_item[0].upload_status, Some(0));
    }

    #[test]
    fn builds_b2b_payment_and_signatures() {
        let client = crate::Client::new(crate::WechatConfig::default()).unwrap();
        let mini_program = super::MiniProgram::new(client, crate::config::Platform::MiniProgram);
        let data = json!({
            "mchid": "mchid",
            "out_trade_no": "trade-no",
            "amount": 100
        });

        let payment = mini_program
            .build_b2b_payment(b"session-key", b"app-key", data.clone())
            .unwrap();

        assert_eq!(payment.mode, "retail_pay_goods");
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&payment.sign_data).unwrap(),
            data
        );
        assert_eq!(
            payment.signature,
            crate::crypto::hmac_sha256_hex(b"session-key", payment.sign_data.as_bytes()).unwrap()
        );
        assert_eq!(
            payment.pay_sig,
            crate::crypto::hmac_sha256_hex(
                b"app-key",
                format!("requestCommonPayment&{}", payment.sign_data).as_bytes()
            )
            .unwrap()
        );
    }

    #[test]
    fn serializes_b2b_requests() {
        let get_order = serde_json::to_value(B2bGetOrderRequest {
            mchid: "mchid".to_string(),
            out_trade_no: Some("trade-no".to_string()),
            order_id: None,
        })
        .unwrap();
        assert_eq!(get_order["mchid"], "mchid");
        assert_eq!(get_order["out_trade_no"], "trade-no");
        assert!(get_order.get("order_id").is_none());

        let refund = serde_json::to_value(B2bRefundRequest {
            mchid: "mchid".to_string(),
            out_trade_no: Some("trade-no".to_string()),
            order_id: None,
            out_refund_no: "refund-no".to_string(),
            refund_amount: 50,
            refund_from: "UNSETTLED".to_string(),
            refund_reason: None,
        })
        .unwrap();
        assert_eq!(refund["refund_amount"], 50);
        assert_eq!(refund["refund_from"], "UNSETTLED");
        assert!(refund.get("refund_reason").is_none());

        let get_refund = serde_json::to_value(B2bGetRefundRequest {
            mchid: "mchid".to_string(),
            out_refund_no: Some("refund-no".to_string()),
            refund_id: None,
        })
        .unwrap();
        assert_eq!(get_refund["out_refund_no"], "refund-no");
        assert!(get_refund.get("refund_id").is_none());

        let add_account = serde_json::to_value(B2bAddProfitSharingAccountRequest {
            profit_sharing_relation_type: "SERVICE_PROVIDER".to_string(),
            payee_type: "PERSONAL_OPENID".to_string(),
            payee_id: "openid".to_string(),
            payee_name: None,
            env: 0,
        })
        .unwrap();
        assert_eq!(
            add_account["profit_sharing_relation_type"],
            "SERVICE_PROVIDER"
        );
        assert!(add_account.get("payee_name").is_none());

        let query_account = serde_json::to_value(B2bQueryProfitSharingAccountRequest {
            offset: 0,
            limit: 20,
        })
        .unwrap();
        assert_eq!(query_account["limit"], 20);

        let bill = serde_json::to_value(B2bDownloadBillRequest {
            mchid: "mchid".to_string(),
            bill_date: "20260709".to_string(),
        })
        .unwrap();
        assert_eq!(bill["bill_date"], "20260709");
    }

    #[test]
    fn deserializes_b2b_responses() {
        let order: B2bGetOrderResponse = serde_json::from_value(json!({
            "appid": "wxappid",
            "mchid": "mchid",
            "out_trade_no": "trade-no",
            "order_id": "order-id",
            "pay_status": "SUCCESS",
            "pay_time": "2026-07-09T12:00:00+08:00",
            "attach": "attach",
            "payer_openid": "openid",
            "amount": { "order_amount": 100, "payer_amount": 100, "currency": "CNY" },
            "wxpay_transaction_id": "wxpay-id",
            "env": 0
        }))
        .unwrap();
        assert_eq!(order.pay_status.as_deref(), Some("SUCCESS"));
        assert_eq!(order.amount.unwrap().order_amount, Some(100));

        let refund: B2bRefundResponse = serde_json::from_value(json!({
            "refund_id": "refund-id",
            "out_refund_no": "refund-no",
            "order_id": "order-id",
            "out_trade_no": "trade-no"
        }))
        .unwrap();
        assert_eq!(refund.refund_id.as_deref(), Some("refund-id"));

        let refund_detail: B2bGetRefundResponse = serde_json::from_value(json!({
            "refund_id": "refund-id",
            "out_refund_no": "refund-no",
            "order_id": "order-id",
            "out_trade_no": "trade-no",
            "create_time": "2026-07-09T12:00:00+08:00",
            "refund_time": "2026-07-09T12:05:00+08:00",
            "refund_status": "SUCCESS",
            "refund_desc": "ok",
            "amount": { "order_amount": 100, "refund_amount": 50, "currency": "CNY" },
            "wxpay_refund_id": "wx-refund-id"
        }))
        .unwrap();
        assert_eq!(refund_detail.refund_status.as_deref(), Some("SUCCESS"));
        assert_eq!(refund_detail.amount.unwrap().refund_amount, Some(50));

        let accounts: B2bQueryProfitSharingAccountResponse = serde_json::from_value(json!({
            "account_list": [{
                "sharing_account_type": "PERSONAL_OPENID",
                "sharing_account": "openid",
                "add_time": "2026-07-09T12:00:00+08:00",
                "update_time": "2026-07-09T12:00:00+08:00",
                "name": "receiver"
            }]
        }))
        .unwrap();
        assert_eq!(
            accounts.account_list[0].sharing_account.as_deref(),
            Some("openid")
        );

        let sharing: B2bProfitSharingOrderResponse =
            serde_json::from_value(json!({ "order_status": "FINISHED" })).unwrap();
        assert_eq!(sharing.order_status.as_deref(), Some("FINISHED"));

        let remain: B2bProfitSharingRemainAmountResponse =
            serde_json::from_value(json!({ "remain_amt": 70 })).unwrap();
        assert_eq!(remain.remain_amt, Some(70));

        let bill: B2bDownloadBillResponse = serde_json::from_value(json!({
            "success_bill_url": "https://example.com/success.csv",
            "refund_bill_url": "https://example.com/refund.csv",
            "all_bill_url": "https://example.com/all.csv",
            "fund_bill_url": "https://example.com/fund.csv",
            "ended_day_avail_amt": 100,
            "ended_day_frozen_amt": 10,
            "ended_day_total_amt": 110,
            "profit_sharing_bill_url": "https://example.com/profit.csv",
            "profit_refund_bill_url": "https://example.com/profit-refund.csv"
        }))
        .unwrap();
        assert_eq!(bill.ended_day_total_amt, Some(110));
        assert_eq!(
            bill.profit_refund_bill_url.as_deref(),
            Some("https://example.com/profit-refund.csv")
        );
    }

    #[test]
    fn serializes_mini_drama_vod_requests() {
        let upload = serde_json::to_value(MiniDramaVideoMediaUploadByUrlRequest {
            media_url: "https://example.com/video.mp4".to_string(),
            cover_url: None,
            media_name: "Drama - EP01".to_string(),
            source_context: Some("trace-1".to_string()),
        })
        .unwrap();
        assert_eq!(upload["media_url"], "https://example.com/video.mp4");
        assert!(upload.get("cover_url").is_none());
        assert_eq!(upload["source_context"], "trace-1");

        let apply = serde_json::to_value(MiniDramaVideoApplyChunkUploadRequest {
            media_name: "Drama - EP01".to_string(),
            media_type: "MP4".to_string(),
            cover_type: Some("JPEG".to_string()),
            source_context: None,
        })
        .unwrap();
        assert_eq!(apply["media_type"], "MP4");
        assert!(apply.get("source_context").is_none());

        let complete = serde_json::to_value(MiniDramaVideoChunkUploadCompleteRequest {
            upload_id: "upload-id".to_string(),
            media_part_infos: vec![MiniDramaPartInfo {
                part_number: 1,
                etag: "etag".to_string(),
            }],
            cover_part_infos: Vec::new(),
        })
        .unwrap();
        assert_eq!(complete["upload_id"], "upload-id");
        assert_eq!(complete["media_part_infos"][0]["part_number"], 1);
        assert!(complete.get("cover_part_infos").is_none());

        let media_list = serde_json::to_value(
            MiniDramaMediaListRequest {
                drama_id: Some(100),
                media_name: Some("Drama%".to_string()),
                start_time: Some(1_800_000_000),
                end_time: Some(1_800_086_400),
                limit: Some(200),
                offset: Some(-1),
            }
            .normalized(),
        )
        .unwrap();
        assert_eq!(media_list["limit"], 100);
        assert_eq!(media_list["offset"], 0);

        let link = serde_json::to_value(MiniDramaMediaLinkRequest {
            media_id: 123,
            t: 1_800_086_400,
            us: Some("channel".to_string()),
            expr: Some(60),
            rlimit: Some(9),
            href: Some("example.com".to_string()),
            bkref: None,
        })
        .unwrap();
        assert_eq!(link["media_id"], 123);
        assert_eq!(link["whref"], "example.com");
        assert!(link.get("bkref").is_none());

        let flush = serde_json::to_value(MiniDramaSetFlushDramaRequest {
            list: vec![MiniDramaFlushDramaInfo {
                src_appid: "wxsource".to_string(),
                drama_id: "100".to_string(),
                drama_name: "Drama".to_string(),
            }],
        })
        .unwrap();
        assert_eq!(flush["list"][0]["src_appid"], "wxsource");
    }

    #[test]
    fn serializes_mini_drama_audit_and_authorization_requests() {
        let actor = MiniDramaActor {
            name: "Actor".to_string(),
            photo_material_id: "photo-id".to_string(),
            role: "Hero".to_string(),
            profile: "profile with enough details".to_string(),
        };
        let audit = serde_json::to_value(MiniDramaSubmitAuditRequest {
            drama_id: None,
            name: "Drama".to_string(),
            media_count: 1,
            media_id_list: vec![10],
            description: "description".to_string(),
            recommendations: "recommend".to_string(),
            cover_material_id: "cover-id".to_string(),
            promotion_poster_material_id: None,
            producer: "producer".to_string(),
            authorized_material_id: "auth-id".to_string(),
            qualification_type: 2,
            registration_number: None,
            qualification_certificate_material_id: None,
            cost_commitment_letter_material_id: Some("cost-id".to_string()),
            cost_of_production: Some(20),
            expedited: Some(1),
            actor_list: MiniDramaActorList { actor: vec![actor] },
            replace_media_list: vec![MiniDramaReplaceInfo { old: 1, new: 2 }],
        })
        .unwrap();
        assert!(audit.get("drama_id").is_none());
        assert_eq!(audit["media_id_list"][0], 10);
        assert_eq!(
            audit["actor_list"]["actor"][0]["photo_material_id"],
            "photo-id"
        );
        assert_eq!(audit["replace_media_list"][0]["new"], 2);

        let replace = serde_json::to_value(MiniDramaReplaceAuditedMediaRequest {
            drama_id: 100,
            old_media_id: 1,
            new_media_id: 2,
        })
        .unwrap();
        assert_eq!(replace["old_media_id"], 1);

        let update = serde_json::to_value(MiniDramaUpdateInfoRequest {
            drama_id: 100,
            description: Some("new description".to_string()),
            cover_material_id: None,
            recommendations: None,
            promotion_poster_material_id: None,
            alternate_name: Some("Alias".to_string()),
            actor_list: None,
            qualification_type: None,
            qualification_certificate_material_id: None,
            registration_number: None,
            cost_of_production: None,
            cost_commitment_letter_material_id: None,
        })
        .unwrap();
        assert_eq!(update["alternate_name"], "Alias");
        assert!(update.get("cover_material_id").is_none());

        let search = serde_json::to_value(
            MiniDramaAuthorizationSearchRequest {
                drama_id: 100,
                page: MiniDramaListRequest {
                    limit: Some(150),
                    offset: Some(-3),
                },
            }
            .normalized(),
        )
        .unwrap();
        assert_eq!(search["limit"], 100);
        assert_eq!(search["offset"], 0);

        let account = serde_json::to_value(MiniDramaAccountAuthorizationRequest {
            authorized_appid: "wxauth".to_string(),
            authz_expire_time: None,
        })
        .unwrap();
        assert_eq!(account["authorized_appid"], "wxauth");
        assert!(account.get("authz_expire_time").is_none());
    }

    #[test]
    fn deserializes_mini_drama_vod_responses() {
        let upload: MiniDramaVideoMediaUploadByUrlResponse = serde_json::from_value(json!({
            "errcode": 0,
            "task_id": 123
        }))
        .unwrap();
        assert_eq!(upload.task_id, Some(123));

        let task: MiniDramaVideoMediaTaskResponse = serde_json::from_value(json!({
            "task_info": { "id": 123, "status": 3, "media_id": 456 }
        }))
        .unwrap();
        assert_eq!(task.task_info.unwrap()["media_id"], 456);

        let file: MiniDramaVideoMediaUploadByFileResponse =
            serde_json::from_value(json!({ "media_id": 456 })).unwrap();
        assert_eq!(file.media_id, Some(456));

        let apply: MiniDramaVideoApplyChunkUploadResponse =
            serde_json::from_value(json!({ "upload_id": "upload-id" })).unwrap();
        assert_eq!(apply.upload_id.as_deref(), Some("upload-id"));

        let chunk: MiniDramaVideoChunkUploadResponse =
            serde_json::from_value(json!({ "etag": "etag" })).unwrap();
        assert_eq!(chunk.etag.as_deref(), Some("etag"));

        let media_list: MiniDramaMediaListResponse = serde_json::from_value(json!({
            "media_info_list": [{ "media_id": 1, "name": "Drama - EP01" }]
        }))
        .unwrap();
        assert_eq!(media_list.media_info_list[0]["media_id"], 1);

        let drama: MiniDramaInfoResponse = serde_json::from_value(json!({
            "drama_info": { "drama_id": 100, "name": "Drama" }
        }))
        .unwrap();
        assert_eq!(drama.drama_info.unwrap()["name"], "Drama");

        let cdn: MiniDramaCdnUsageResponse = serde_json::from_value(json!({
            "data_interval": 3600,
            "item_list": [{ "time": 1800000000, "value": 1024 }]
        }))
        .unwrap();
        assert_eq!(cdn.data_interval, Some(3600));
        assert_eq!(cdn.item_list[0]["value"], 1024);

        let auth: MiniDramaAuthorizationSearchResponse = serde_json::from_value(json!({
            "objects": [{ "drama_id": 100, "authorized_appid": "wxauth" }],
            "total_count": 1
        }))
        .unwrap();
        assert_eq!(auth.total_count, Some(1));
        assert_eq!(auth.objects[0]["authorized_appid"], "wxauth");

        let account: MiniDramaAccountAuthorizationSearchResponse = serde_json::from_value(json!({
            "objects": [{ "authorized_appid": "wxauth", "authorized_time": 1800000000 }]
        }))
        .unwrap();
        assert_eq!(account.objects[0]["authorized_appid"], "wxauth");
    }

    #[test]
    fn serializes_immediate_delivery_requests() {
        let request = serde_json::to_value(ImmediateDeliveryRequest::new(json!({
            "shopid": "shop-id",
            "shop_order_id": "order-id",
            "delivery_id": "delivery-id",
            "delivery_sign": "sign"
        })))
        .unwrap();
        assert_eq!(request["shopid"], "shop-id");
        assert_eq!(request["delivery_sign"], "sign");

        let get_order = serde_json::to_value(ImmediateDeliveryGetOrderRequest {
            shopid: "shop-id".to_string(),
            shop_order_id: "order-id".to_string(),
            shop_no: "shop-no".to_string(),
            delivery_sign: "sign".to_string(),
        })
        .unwrap();
        assert_eq!(get_order["shopid"], "shop-id");
        assert_eq!(get_order["shop_order_id"], "order-id");
        assert_eq!(get_order["shop_no"], "shop-no");
        assert_eq!(get_order["delivery_sign"], "sign");
    }

    #[test]
    fn deserializes_immediate_delivery_responses() {
        let shops: ImmediateDeliveryBindAccountResponse = serde_json::from_value(json!({
            "errcode": 0,
            "shop_list": [{ "delivery_id": "delivery-id", "shopid": "shop-id" }]
        }))
        .unwrap();
        assert_eq!(shops.shop_list[0]["delivery_id"], "delivery-id");

        let delivery_list: ImmediateDeliveryDeliveryListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "list": [{ "delivery_id": "delivery-id", "delivery_name": "Fast Delivery" }]
        }))
        .unwrap();
        assert_eq!(delivery_list.list[0]["delivery_name"], "Fast Delivery");

        let cancel: ImmediateDeliveryCancelOrderResponse = serde_json::from_value(json!({
            "errcode": 0,
            "deduct_fee": 5,
            "desc": "cancelled"
        }))
        .unwrap();
        assert_eq!(cancel.deduct_fee, Some(5));
        assert_eq!(cancel.desc.as_deref(), Some("cancelled"));

        let order: ImmediateDeliveryGetOrderResponse = serde_json::from_value(json!({
            "errcode": 0,
            "order_status": 101,
            "waybill_id": "waybill-id",
            "rider_name": "Alex",
            "rider_phone": "13800000000",
            "rider_lng": 120.1,
            "rider_lat": 30.2,
            "reach_time": 300
        }))
        .unwrap();
        assert_eq!(order.order_status, Some(101));
        assert_eq!(order.waybill_id.as_deref(), Some("waybill-id"));
        assert_eq!(order.rider_lng, Some(120.1));

        let pre_add: ImmediateDeliveryPreAddOrderResponse = serde_json::from_value(json!({
            "errcode": 0,
            "fee": 10,
            "deliverfee": "10",
            "couponfee": "0",
            "tips": "0",
            "insurancfee": 0.0,
            "distance": 1000.0,
            "dispatch_duration": 300,
            "delivery_token": 1111111
        }))
        .unwrap();
        assert_eq!(pre_add.fee, Some(10));
        assert_eq!(pre_add.delivery_token, Some(1111111));

        let pre_cancel: ImmediateDeliveryPreCancelOrderResponse = serde_json::from_value(json!({
            "errcode": 0,
            "deduct_fee": 5,
            "desc": "fee"
        }))
        .unwrap();
        assert_eq!(pre_cancel.deduct_fee, Some(5));

        let reorder: ImmediateDeliveryReOrderResponse = serde_json::from_value(json!({
            "errcode": 0,
            "fee": 10,
            "deliverfee": "10",
            "couponfee": "0",
            "tips": "0",
            "insurancfee": 0.0,
            "distance": 1000.0,
            "waybill_id": 123456789,
            "order_status": 101,
            "finish_code": 1024,
            "pickup_code": 2048,
            "dispatch_duration": 300
        }))
        .unwrap();
        assert_eq!(reorder.insurance_fee, Some(0.0));
        assert_eq!(reorder.waybill_id, Some(123456789));
        assert_eq!(reorder.pickup_code, Some(2048));
    }

    #[test]
    fn serializes_express_requests() {
        let add = serde_json::to_value(ExpressRequest::new(json!({
            "order_id": "order-id",
            "openid": "openid",
            "delivery_id": "delivery-id",
            "biz_id": "biz-id"
        })))
        .unwrap();
        assert_eq!(add["order_id"], "order-id");
        assert_eq!(add["biz_id"], "biz-id");

        let bind = serde_json::to_value(ExpressBindAccountRequest {
            action_type: "bind".to_string(),
            biz_id: "biz-id".to_string(),
            delivery_id: "delivery-id".to_string(),
            password: "secret".to_string(),
        })
        .unwrap();
        assert_eq!(bind["type"], "bind");
        assert_eq!(bind["biz_id"], "biz-id");
        assert_eq!(bind["delivery_id"], "delivery-id");
        assert_eq!(bind["password"], "secret");

        let order = serde_json::to_value(ExpressOrderRequest {
            order_id: "order-id".to_string(),
            openid: "openid".to_string(),
            delivery_id: "delivery-id".to_string(),
            waybill_id: "waybill-id".to_string(),
        })
        .unwrap();
        assert_eq!(order["order_id"], "order-id");
        assert_eq!(order["openid"], "openid");

        let get_order = serde_json::to_value(ExpressGetOrderRequest {
            order_id: "order-id".to_string(),
            openid: "openid".to_string(),
            delivery_id: "delivery-id".to_string(),
            waybill_id: "waybill-id".to_string(),
            print_type: 1,
        })
        .unwrap();
        assert_eq!(get_order["print_type"], 1);

        let update = serde_json::to_value(ExpressTestUpdateOrderRequest {
            biz_id: "biz-id".to_string(),
            order_id: "order-id".to_string(),
            delivery_id: "delivery-id".to_string(),
            waybill_id: "waybill-id".to_string(),
            action_time: 1_800_000_000,
            action_type: 100001,
            action_msg: "picked up".to_string(),
        })
        .unwrap();
        assert_eq!(update["action_time"], 1_800_000_000);
        assert_eq!(update["action_type"], 100001);
        assert_eq!(update["action_msg"], "picked up");

        let printer = serde_json::to_value(ExpressUpdatePrinterRequest {
            openid: "openid".to_string(),
            update_type: "bind".to_string(),
            tagid_list: "tag-a,tag-b".to_string(),
        })
        .unwrap();
        assert_eq!(printer["update_type"], "bind");
        assert_eq!(printer["tagid_list"], "tag-a,tag-b");
    }

    #[test]
    fn deserializes_express_responses() {
        let add: ExpressAddOrderResponse = serde_json::from_value(json!({
            "errcode": 0,
            "delivery_resultcode": 0,
            "delivery_resultmsg": "ok",
            "order_id": "order-id",
            "waybill_id": "waybill-id",
            "waybill_data": [{ "key": "tracking_no", "value": "waybill-id" }]
        }))
        .unwrap();
        assert_eq!(add.delivery_resultcode, Some(0));
        assert_eq!(add.order_id.as_deref(), Some("order-id"));
        assert_eq!(add.waybill_data[0].key.as_deref(), Some("tracking_no"));
        assert_eq!(add.waybill_data[0].value.as_deref(), Some("waybill-id"));

        let batch: ExpressBatchOrderListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "order_list": [{ "order_id": "order-id", "waybill_id": "waybill-id" }]
        }))
        .unwrap();
        assert_eq!(batch.order_list[0].order_id.as_deref(), Some("order-id"));
        assert_eq!(
            batch.order_list[0].waybill_id.as_deref(),
            Some("waybill-id")
        );

        let cancel: ExpressCancelOrderResponse = serde_json::from_value(json!({
            "errcode": 0,
            "delivery_resultcode": 0,
            "delivery_resultmsg": ""
        }))
        .unwrap();
        assert_eq!(cancel.delivery_resultcode, Some(0));

        let accounts: ExpressAccountListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "count": 1,
            "list": [{ "biz_id": "biz-id", "delivery_id": "delivery-id" }]
        }))
        .unwrap();
        assert_eq!(accounts.count, Some(1));
        assert_eq!(accounts.list[0].biz_id.as_deref(), Some("biz-id"));
        assert_eq!(accounts.list[0].delivery_id.as_deref(), Some("delivery-id"));

        let deliveries: ExpressDeliveryListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "count": 1,
            "data": [{ "delivery_id": "delivery-id", "delivery_name": "Fast Delivery" }]
        }))
        .unwrap();
        assert_eq!(
            deliveries.data[0].delivery_name.as_deref(),
            Some("Fast Delivery")
        );

        let order: ExpressGetOrderResponse = serde_json::from_value(json!({
            "errcode": 0,
            "print_html": "<html></html>",
            "waybill_data": [{ "key": "tracking_no", "value": "waybill-id" }],
            "delivery_id": "delivery-id",
            "waybill_id": "waybill-id",
            "order_id": "order-id",
            "order_status": 1
        }))
        .unwrap();
        assert_eq!(order.order_status, Some(1));
        assert_eq!(order.print_html.as_deref(), Some("<html></html>"));
        assert_eq!(order.waybill_data[0].key.as_deref(), Some("tracking_no"));

        let path: ExpressGetPathResponse = serde_json::from_value(json!({
            "errcode": 0,
            "openid": "openid",
            "delivery_id": "delivery-id",
            "waybill_id": "waybill-id",
            "path_item_num": 1,
            "path_item_list": [{
                "action_time": 1_800_000_000,
                "action_type": 100001,
                "action_msg": "picked up"
            }]
        }))
        .unwrap();
        assert_eq!(path.path_item_num, Some(1));
        assert_eq!(path.path_item_list[0].action_type, Some(100001));
        assert_eq!(
            path.path_item_list[0].action_msg.as_deref(),
            Some("picked up")
        );

        let printer: ExpressGetPrinterResponse = serde_json::from_value(json!({
            "errcode": 0,
            "count": "1",
            "openid": ["openid"],
            "tagid_list": ["tag-a"]
        }))
        .unwrap();
        assert_eq!(printer.count, Some(json!("1")));
        assert_eq!(printer.openid[0], "openid");

        let quota: ExpressGetQuotaResponse = serde_json::from_value(json!({
            "errcode": 0,
            "quota_num": "100"
        }))
        .unwrap();
        assert_eq!(quota.quota_num, Some(json!("100")));

        let contact: ExpressGetContactResponse = serde_json::from_value(json!({
            "errcode": 0,
            "waybill_id": "waybill-id",
            "sender": [{ "name": "sender", "mobile": "13800000000" }],
            "receiver": [{ "name": "receiver", "address": "street" }]
        }))
        .unwrap();
        assert_eq!(contact.waybill_id.as_deref(), Some("waybill-id"));
        assert_eq!(contact.sender[0].mobile.as_deref(), Some("13800000000"));
        assert_eq!(contact.receiver[0].name.as_deref(), Some("receiver"));

        let preview: ExpressPreviewTemplateResponse = serde_json::from_value(json!({
            "errcode": 0,
            "waybill_id": "waybill-id",
            "rendered_waybill_template": "template"
        }))
        .unwrap();
        assert_eq!(preview.waybill_id.as_deref(), Some("waybill-id"));
        assert_eq!(
            preview.rendered_waybill_template.as_deref(),
            Some("template")
        );
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

        let paid = serde_json::to_value(MiniProgramPaidUnionIdRequest {
            openid: "openid".to_string(),
            transaction_id: Some("transaction".to_string()),
            mch_id: None,
            out_trade_no: None,
        })
        .unwrap();
        assert_eq!(paid["openid"], "openid");
        assert_eq!(paid["transaction_id"], "transaction");
        assert!(paid.get("mch_id").is_none());

        let paid_response: MiniProgramPaidUnionIdResponse =
            serde_json::from_value(json!({ "unionid": "unionid" })).unwrap();
        assert_eq!(paid_response.unionid.as_deref(), Some("unionid"));

        let encrypted: MiniProgramCheckEncryptedDataResponse = serde_json::from_value(json!({
            "vaild": true,
            "create_time": 1800000000
        }))
        .unwrap();
        assert_eq!(encrypted.vaild, Some(true));

        let activity: MiniProgramActivityIdResponse = serde_json::from_value(json!({
            "activity_id": "activity",
            "expiration_time": 1800000000
        }))
        .unwrap();
        assert_eq!(activity.activity_id.as_deref(), Some("activity"));
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
    fn deserializes_image_responses() {
        let crop: ImageAiCropResponse = serde_json::from_value(json!({
            "errcode": 0,
            "results": [{
                "crop_left": 10,
                "crop_top": 20,
                "crop_right": 300,
                "crop_bottom": 240
            }]
        }))
        .unwrap();
        assert_eq!(crop.errcode, Some(0));
        assert_eq!(crop.results[0]["crop_left"], 10);

        let qrcode: ImageScanQrCodeResponse = serde_json::from_value(json!({
            "errcode": 0,
            "code_results": [{
                "type_name": "QR_CODE",
                "data": "https://example.com"
            }],
            "img_size": {
                "w": 640,
                "h": 480
            }
        }))
        .unwrap();
        assert_eq!(qrcode.code_results[0]["type_name"], "QR_CODE");
        assert_eq!(qrcode.img_size.expect("img_size")["w"], 640);

        let super_resolution: ImageSuperResolutionResponse = serde_json::from_value(json!({
            "errcode": 0,
            "media_id": "media-id"
        }))
        .unwrap();
        assert_eq!(super_resolution.media_id.as_deref(), Some("media-id"));
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

        let goods: MiniProgramLiveGoodsWarehouseResponse = serde_json::from_value(json!({
            "goods": [{ "goodsId": 100, "name": "item" }]
        }))
        .unwrap();
        assert_eq!(goods.goods[0]["goodsId"], 100);

        let followers: MiniProgramLiveFollowersResponse = serde_json::from_value(json!({
            "followers": [{ "openid": "openid" }],
            "page_break": 10
        }))
        .unwrap();
        assert_eq!(followers.followers[0]["openid"], "openid");
        assert_eq!(followers.page_break.unwrap(), 10);
    }
}
