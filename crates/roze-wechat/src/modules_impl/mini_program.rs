use bytes::Bytes;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    config::Platform,
    crypto,
    error::{Result, WechatError},
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
        request: UniformMessageRequest,
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
        request: UpdatableMessageRequest,
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
    ) -> Result<DataCubeVisitTrendResponse> {
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
    ) -> Result<DataCubeVisitTrendResponse> {
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
    ) -> Result<DataCubeVisitTrendResponse> {
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
    ) -> Result<DataCubeRetainInfoResponse> {
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
    ) -> Result<DataCubeVisitPageResponse> {
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
    ) -> Result<DataCubeUserPortraitResponse> {
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
        request: DataCubePerformanceDataRequest,
    ) -> Result<DataCubePerformanceDataResponse> {
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
    ) -> Result<MiniProgramCreateLiveRoomResponse> {
        request.validate()?;
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
    ) -> Result<MiniProgramLiveInfoResponse> {
        request.validate()?;
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
    ) -> Result<MiniProgramLiveReplayResponse> {
        validate_live_positive("room id", room_id)?;
        validate_live_page(start, limit, 100)?;
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
        validate_live_positive("room id", room_id)?;
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
        validate_live_positive_ids("goods id", &goods_ids, 100)?;
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
        validate_live_page(page_break, limit, 1_000)?;
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
        validate_live_positive("room id", room_id)?;
        validate_live_unique_strings("follower openid", &user_openid, 200)?;
        self.inner
            .post(
                "wxa/business/push_message",
                Some(access_token.into()),
                json!({ "room_id": room_id, "user_openid": user_openid }),
            )
            .await
    }

    pub async fn add_live_room_goods(
        &self,
        access_token: impl Into<String>,
        request: MiniProgramLiveRoomAddGoodsRequest,
    ) -> Result<WechatStatusResponse> {
        request.validate()?;
        self.inner
            .post(
                "wxaapi/broadcast/room/addgoods",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn edit_live_room(
        &self,
        access_token: impl Into<String>,
        request: MiniProgramLiveRoomEditRequest,
    ) -> Result<WechatStatusResponse> {
        request.validate()?;
        self.inner
            .post(
                "wxaapi/broadcast/room/editroom",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_live_push_url(
        &self,
        access_token: impl Into<String>,
        room_id: i64,
    ) -> Result<MiniProgramLivePushUrlResponse> {
        validate_live_positive("room id", room_id)?;
        self.inner
            .get_with_query(
                "wxaapi/broadcast/room/getpushurl",
                Some(access_token.into()),
                vec![("roomId".to_string(), room_id.to_string())],
            )
            .await
    }

    pub async fn get_live_shared_code(
        &self,
        access_token: impl Into<String>,
        room_id: i64,
        params: impl Into<String>,
    ) -> Result<MiniProgramLiveSharedCodeResponse> {
        validate_live_positive("room id", room_id)?;
        let params = params.into();
        self.inner
            .get_with_query(
                "wxaapi/broadcast/room/getsharedcode",
                Some(access_token.into()),
                vec![
                    ("roomId".to_string(), room_id.to_string()),
                    ("params".to_string(), params),
                ],
            )
            .await
    }

    pub async fn add_live_assistants(
        &self,
        access_token: impl Into<String>,
        request: MiniProgramLiveAssistantAddRequest,
    ) -> Result<WechatStatusResponse> {
        request.validate()?;
        self.inner
            .post(
                "wxaapi/broadcast/room/addassistant",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn modify_live_assistant(
        &self,
        access_token: impl Into<String>,
        request: MiniProgramLiveAssistantModifyRequest,
    ) -> Result<WechatStatusResponse> {
        request.validate()?;
        self.inner
            .post(
                "wxaapi/broadcast/room/modifyassistant",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn remove_live_assistant(
        &self,
        access_token: impl Into<String>,
        request: MiniProgramLiveAssistantRemoveRequest,
    ) -> Result<WechatStatusResponse> {
        request.validate()?;
        self.inner
            .post(
                "wxaapi/broadcast/room/removeassistant",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_live_assistants(
        &self,
        access_token: impl Into<String>,
        room_id: i64,
    ) -> Result<MiniProgramLiveAssistantListResponse> {
        validate_live_positive("room id", room_id)?;
        self.inner
            .get_with_query(
                "wxaapi/broadcast/room/getassistantlist",
                Some(access_token.into()),
                vec![("roomId".to_string(), room_id.to_string())],
            )
            .await
    }

    pub async fn add_live_sub_anchor(
        &self,
        access_token: impl Into<String>,
        request: MiniProgramLiveSubAnchorRequest,
    ) -> Result<WechatStatusResponse> {
        request.validate()?;
        self.inner
            .post(
                "wxaapi/broadcast/room/addsubanchor",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn modify_live_sub_anchor(
        &self,
        access_token: impl Into<String>,
        request: MiniProgramLiveSubAnchorRequest,
    ) -> Result<WechatStatusResponse> {
        request.validate()?;
        self.inner
            .post(
                "wxaapi/broadcast/room/modifysubanchor",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn delete_live_sub_anchor(
        &self,
        access_token: impl Into<String>,
        room_id: i64,
    ) -> Result<WechatStatusResponse> {
        validate_live_positive("room id", room_id)?;
        self.inner
            .post(
                "wxaapi/broadcast/room/deletesubanchor",
                Some(access_token.into()),
                json!({ "roomId": room_id }),
            )
            .await
    }

    pub async fn get_live_sub_anchor(
        &self,
        access_token: impl Into<String>,
        room_id: i64,
    ) -> Result<MiniProgramLiveSubAnchorResponse> {
        validate_live_positive("room id", room_id)?;
        self.inner
            .get_with_query(
                "wxaapi/broadcast/room/getsubanchor",
                Some(access_token.into()),
                vec![("roomId".to_string(), room_id.to_string())],
            )
            .await
    }

    pub async fn update_live_feed_public(
        &self,
        access_token: impl Into<String>,
        room_id: i64,
        is_feeds_public: i64,
    ) -> Result<WechatStatusResponse> {
        validate_live_room_flag("feed-public", room_id, is_feeds_public)?;
        self.inner
            .post(
                "wxaapi/broadcast/room/updatefeedpublic",
                Some(access_token.into()),
                json!({ "roomId": room_id, "isFeedsPublic": is_feeds_public }),
            )
            .await
    }

    pub async fn update_live_replay(
        &self,
        access_token: impl Into<String>,
        room_id: i64,
        close_replay: i64,
    ) -> Result<WechatStatusResponse> {
        validate_live_room_flag("close-replay", room_id, close_replay)?;
        self.inner
            .post(
                "wxaapi/broadcast/room/updatereplay",
                Some(access_token.into()),
                json!({ "roomId": room_id, "closeReplay": close_replay }),
            )
            .await
    }

    pub async fn update_live_customer_service(
        &self,
        access_token: impl Into<String>,
        room_id: i64,
        close_kf: i64,
    ) -> Result<WechatStatusResponse> {
        validate_live_room_flag("close-customer-service", room_id, close_kf)?;
        self.inner
            .post(
                "wxaapi/broadcast/room/updatekf",
                Some(access_token.into()),
                json!({ "roomId": room_id, "closeKf": close_kf }),
            )
            .await
    }

    pub async fn update_live_comment(
        &self,
        access_token: impl Into<String>,
        room_id: i64,
        ban_comment: i64,
    ) -> Result<WechatStatusResponse> {
        validate_live_room_flag("ban-comment", room_id, ban_comment)?;
        self.inner
            .post(
                "wxaapi/broadcast/room/updatecomment",
                Some(access_token.into()),
                json!({ "roomId": room_id, "banComment": ban_comment }),
            )
            .await
    }

    pub async fn update_live_goods_sale(
        &self,
        access_token: impl Into<String>,
        room_id: i64,
        goods_id: i64,
        on_sale: i64,
    ) -> Result<WechatStatusResponse> {
        validate_live_positive("room id", room_id)?;
        validate_live_positive("goods id", goods_id)?;
        validate_live_flag("goods on-sale", on_sale)?;
        self.inner
            .post(
                "wxaapi/broadcast/goods/onsale",
                Some(access_token.into()),
                json!({ "roomId": room_id, "goodsId": goods_id, "onSale": on_sale }),
            )
            .await
    }

    pub async fn delete_live_goods_from_room(
        &self,
        access_token: impl Into<String>,
        room_id: i64,
        goods_id: i64,
    ) -> Result<WechatStatusResponse> {
        validate_live_room_goods_ids(room_id, goods_id)?;
        self.inner
            .post(
                "wxaapi/broadcast/goods/deleteInRoom",
                Some(access_token.into()),
                json!({ "roomId": room_id, "goodsId": goods_id }),
            )
            .await
    }

    pub async fn push_live_goods(
        &self,
        access_token: impl Into<String>,
        room_id: i64,
        goods_id: i64,
    ) -> Result<WechatStatusResponse> {
        validate_live_room_goods_ids(room_id, goods_id)?;
        self.inner
            .post(
                "wxaapi/broadcast/goods/push",
                Some(access_token.into()),
                json!({ "roomId": room_id, "goodsId": goods_id }),
            )
            .await
    }

    pub async fn sort_live_goods(
        &self,
        access_token: impl Into<String>,
        room_id: i64,
        goods_ids: Vec<i64>,
    ) -> Result<WechatStatusResponse> {
        validate_live_positive("room id", room_id)?;
        validate_live_positive_ids("goods id", &goods_ids, 100)?;
        let goods = goods_ids
            .into_iter()
            .map(|goods_id| json!({ "goodsId": goods_id }))
            .collect::<Vec<_>>();
        self.inner
            .post(
                "wxaapi/broadcast/goods/sort",
                Some(access_token.into()),
                json!({ "roomId": room_id, "goods": goods }),
            )
            .await
    }

    pub async fn get_live_goods_video(
        &self,
        access_token: impl Into<String>,
        room_id: i64,
        goods_id: i64,
    ) -> Result<MiniProgramLiveGoodsVideoResponse> {
        validate_live_room_goods_ids(room_id, goods_id)?;
        self.inner
            .post(
                "wxaapi/broadcast/goods/getVideo",
                Some(access_token.into()),
                json!({ "roomId": room_id, "goodsId": goods_id }),
            )
            .await
    }

    pub async fn add_live_goods(
        &self,
        access_token: impl Into<String>,
        request: MiniProgramLiveGoodsMutationRequest,
    ) -> Result<MiniProgramLiveGoodsMutationResponse> {
        request.validate_create()?;
        self.inner
            .post(
                "wxaapi/broadcast/goods/add",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn reset_live_goods_audit(
        &self,
        access_token: impl Into<String>,
        audit_id: i64,
        goods_id: i64,
    ) -> Result<WechatStatusResponse> {
        validate_live_positive("audit id", audit_id)?;
        validate_live_positive("goods id", goods_id)?;
        self.inner
            .post(
                "wxaapi/broadcast/goods/resetaudit",
                Some(access_token.into()),
                json!({ "auditId": audit_id, "goodsId": goods_id }),
            )
            .await
    }

    pub async fn audit_live_goods(
        &self,
        access_token: impl Into<String>,
        goods_id: i64,
    ) -> Result<MiniProgramLiveGoodsAuditResponse> {
        validate_live_positive("goods id", goods_id)?;
        self.inner
            .post(
                "wxaapi/broadcast/goods/audit",
                Some(access_token.into()),
                json!({ "goodsId": goods_id }),
            )
            .await
    }

    pub async fn delete_live_goods(
        &self,
        access_token: impl Into<String>,
        goods_id: i64,
    ) -> Result<WechatStatusResponse> {
        validate_live_positive("goods id", goods_id)?;
        self.inner
            .post(
                "wxaapi/broadcast/goods/delete",
                Some(access_token.into()),
                json!({ "goodsId": goods_id }),
            )
            .await
    }

    pub async fn update_live_goods(
        &self,
        access_token: impl Into<String>,
        request: MiniProgramLiveGoodsMutationRequest,
    ) -> Result<WechatStatusResponse> {
        request.validate_update()?;
        self.inner
            .post(
                "wxaapi/broadcast/goods/update",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn list_live_goods(
        &self,
        access_token: impl Into<String>,
        request: MiniProgramLiveGoodsListRequest,
    ) -> Result<MiniProgramLiveGoodsListResponse> {
        request.validate()?;
        self.inner
            .get_with_query(
                "wxaapi/broadcast/goods/getapproved",
                Some(access_token.into()),
                request.query(),
            )
            .await
    }

    pub async fn add_live_role(
        &self,
        access_token: impl Into<String>,
        username: impl Into<String>,
        role: i64,
    ) -> Result<WechatStatusResponse> {
        let username = username.into();
        validate_live_role(&username, role)?;
        self.inner
            .post(
                "wxaapi/broadcast/role/addrole",
                Some(access_token.into()),
                json!({ "username": username, "role": role }),
            )
            .await
    }

    pub async fn delete_live_role(
        &self,
        access_token: impl Into<String>,
        username: impl Into<String>,
        role: i64,
    ) -> Result<WechatStatusResponse> {
        let username = username.into();
        validate_live_role(&username, role)?;
        self.inner
            .post(
                "wxaapi/broadcast/role/deleterole",
                Some(access_token.into()),
                json!({ "username": username, "role": role }),
            )
            .await
    }

    pub async fn list_live_roles(
        &self,
        access_token: impl Into<String>,
        request: MiniProgramLiveRoleListRequest,
    ) -> Result<MiniProgramLiveRoleListResponse> {
        request.validate()?;
        self.inner
            .post(
                "wxaapi/broadcast/role/getrolelist",
                Some(access_token.into()),
                request,
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
    ) -> Result<SecurityMsgSecCheckResponse> {
        self.inner
            .post("wxa/msg_sec_check", Some(access_token.into()), request)
            .await
    }

    pub async fn security_media_check_async(
        &self,
        access_token: impl Into<String>,
        request: SecurityMediaCheckAsyncRequest,
    ) -> Result<SecurityMediaCheckAsyncResponse> {
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
        let action = action.into();
        validate_operation_domain_action(&action)?;
        self.inner
            .post(
                "wxa/getwxadevinfo",
                Some(access_token.into()),
                json!({ "action": action }),
            )
            .await
    }

    pub async fn get_operation_domain_info_typed(
        &self,
        access_token: impl Into<String>,
        action: OperationDomainAction,
    ) -> Result<OperationDomainInfoResponse> {
        self.inner
            .post(
                "wxa/getwxadevinfo",
                Some(access_token.into()),
                json!({ "action": action.as_str() }),
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
        validate_operation_feedback_query(feedback_type, page, num)?;
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
        if record_id <= 0 {
            return Err(WechatError::Config(
                "mini-program operation feedback record id must be positive".to_string(),
            ));
        }
        let media_id = media_id.into();
        validate_operation_required("feedback media id", &media_id)?;
        self.inner
            .post_form_bytes(
                "cgi-bin/media/getfeedbackmedia",
                Some(access_token.into()),
                vec![
                    ("record_id".to_string(), record_id.to_string()),
                    ("media_id".to_string(), media_id),
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
        request.validate()?;
        self.inner
            .post(
                "wxaapi/log/jserr_detail",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_operation_js_err_detail_typed(
        &self,
        access_token: impl Into<String>,
        request: OperationJsErrDetailRequest,
    ) -> Result<OperationJsErrDetailResponse> {
        request.validate()?;
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
        request.validate()?;
        self.inner
            .post("wxaapi/log/jserr_list", Some(access_token.into()), request)
            .await
    }

    pub async fn get_operation_js_err_list_typed(
        &self,
        access_token: impl Into<String>,
        request: OperationJsErrListRequest,
    ) -> Result<OperationJsErrListResponse> {
        request.validate()?;
        self.inner
            .post("wxaapi/log/jserr_list", Some(access_token.into()), request)
            .await
    }

    pub async fn search_operation_js_err(
        &self,
        access_token: impl Into<String>,
        request: OperationRequest,
    ) -> Result<OperationJsErrSearchResponse> {
        request.validate()?;
        self.inner
            .post(
                "wxaapi/log/jserr_search",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn search_operation_js_err_typed(
        &self,
        access_token: impl Into<String>,
        request: OperationJsErrSearchRequest,
    ) -> Result<OperationJsErrSearchResponse> {
        request.validate()?;
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
        request.validate()?;
        self.inner
            .post(
                "wxaapi/log/get_performance",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_operation_performance_typed(
        &self,
        access_token: impl Into<String>,
        request: OperationPerformanceRequest,
    ) -> Result<OperationPerformanceResponse> {
        request.validate()?;
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
        request.validate()?;
        self.inner
            .post(
                "wxaapi/userlog/userlog_search",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn search_operation_real_time_log_typed(
        &self,
        access_token: impl Into<String>,
        request: OperationRealTimeLogSearchRequest,
    ) -> Result<OperationRealTimeLogSearchResponse> {
        request.validate()?;
        self.inner
            .get_with_query(
                "wxaapi/userlog/userlog_search",
                Some(access_token.into()),
                request.to_query(),
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
        request.validate()?;
        self.inner
            .post(
                "cgi-bin/express/local/business/order/confirm_return",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn abnormal_confirm_immediate_delivery_typed(
        &self,
        access_token: impl Into<String>,
        request: ImmediateDeliveryAbnormalConfirmRequest,
    ) -> Result<ImmediateDeliveryStatusResponse> {
        request.validate()?;
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
        request.validate()?;
        self.inner
            .post(
                "cgi-bin/immediateDelivery/local/business/order/add",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn add_immediate_delivery_order_typed(
        &self,
        access_token: impl Into<String>,
        request: ImmediateDeliveryAddOrderRequest,
    ) -> Result<ImmediateDeliveryOrderResponse> {
        request.validate()?;
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
        request.validate()?;
        self.inner
            .post(
                "cgi-bin/immediateDelivery/local/business/order/addtips",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn add_immediate_delivery_tip_typed(
        &self,
        access_token: impl Into<String>,
        request: ImmediateDeliveryAddTipRequest,
    ) -> Result<ImmediateDeliveryStatusResponse> {
        request.validate()?;
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
        let delivery_id = delivery_id.into();
        validate_immediate_delivery_required("delivery id", &delivery_id)?;
        self.inner
            .post(
                "cgi-bin/express/local/business/shop/add",
                Some(access_token.into()),
                json!({ "delivery_id": delivery_id }),
            )
            .await
    }

    pub async fn cancel_immediate_delivery_order(
        &self,
        access_token: impl Into<String>,
        request: ImmediateDeliveryRequest,
    ) -> Result<ImmediateDeliveryCancelOrderResponse> {
        request.validate()?;
        self.inner
            .post(
                "cgi-bin/immediateDelivery/local/business/order/cancel",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn cancel_immediate_delivery_order_typed(
        &self,
        access_token: impl Into<String>,
        request: ImmediateDeliveryCancelOrderRequest,
    ) -> Result<ImmediateDeliveryCancelOrderResponse> {
        request.validate()?;
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
        request.validate()?;
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
        request.validate()?;
        self.inner
            .post(
                "cgi-bin/immediateDelivery/local/business/test_update_order",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn mock_update_immediate_delivery_order_typed(
        &self,
        access_token: impl Into<String>,
        request: ImmediateDeliveryMockUpdateOrderRequest,
    ) -> Result<ImmediateDeliveryStatusResponse> {
        request.validate(false)?;
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
        request.validate()?;
        self.inner
            .post(
                "cgi-bin/immediateDelivery/local/business/order/pre_add",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn pre_add_immediate_delivery_order_typed(
        &self,
        access_token: impl Into<String>,
        request: ImmediateDeliveryAddOrderRequest,
    ) -> Result<ImmediateDeliveryPreAddOrderResponse> {
        request.validate()?;
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
        request.validate()?;
        self.inner
            .post(
                "cgi-bin/immediateDelivery/local/business/order/precancel",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn pre_cancel_immediate_delivery_order_typed(
        &self,
        access_token: impl Into<String>,
        request: ImmediateDeliveryCancelOrderRequest,
    ) -> Result<ImmediateDeliveryPreCancelOrderResponse> {
        request.validate()?;
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
        request.validate()?;
        self.inner
            .post(
                "cgi-bin/immediateDelivery/local/business/realmock_update_order",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn real_mock_update_immediate_delivery_order_typed(
        &self,
        access_token: impl Into<String>,
        request: ImmediateDeliveryMockUpdateOrderRequest,
    ) -> Result<ImmediateDeliveryStatusResponse> {
        request.validate(true)?;
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
        request.validate()?;
        self.inner
            .post(
                "cgi-bin/immediateDelivery/local/business/order/readd",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn re_add_immediate_delivery_order_typed(
        &self,
        access_token: impl Into<String>,
        request: ImmediateDeliveryReOrderRequest,
    ) -> Result<ImmediateDeliveryReOrderResponse> {
        request.validate()?;
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
        request.validate()?;
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
        order_list: Vec<ExpressBatchGetOrderItem>,
    ) -> Result<ExpressBatchOrderListResponse> {
        validate_express_batch_orders(&order_list)?;
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
        request.validate()?;
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
        request.validate()?;
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
        request.validate()?;
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
        request.validate()?;
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
        let delivery_id = delivery_id.into();
        let biz_id = biz_id.into();
        validate_express_required("delivery id", &delivery_id)?;
        validate_express_required("business id", &biz_id)?;
        self.inner
            .post(
                "cgi-bin/express/business/quota/get",
                Some(access_token.into()),
                json!({
                    "delivery_id": delivery_id,
                    "biz_id": biz_id,
                }),
            )
            .await
    }

    pub async fn test_update_express_order(
        &self,
        access_token: impl Into<String>,
        request: ExpressTestUpdateOrderRequest,
    ) -> Result<WechatStatusResponse> {
        request.validate()?;
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
        request.validate()?;
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
        let token = token.into();
        let waybill_id = waybill_id.into();
        validate_express_required("contact token", &token)?;
        validate_express_required("waybill id", &waybill_id)?;
        self.inner
            .post(
                "cgi-bin/express/delivery/contact/get",
                Some(access_token.into()),
                json!({
                    "token": token,
                    "waybill_id": waybill_id,
                }),
            )
            .await
    }

    pub async fn preview_express_template(
        &self,
        access_token: impl Into<String>,
        request: ExpressRequest,
    ) -> Result<ExpressPreviewTemplateResponse> {
        request.validate()?;
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
        request.validate()?;
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
        request.validate()?;
        self.inner
            .post(
                "cgi-bin/express/delivery/path/update",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn add_express_order_typed(
        &self,
        access_token: impl Into<String>,
        request: ExpressAddOrderRequest,
    ) -> Result<ExpressAddOrderResponse> {
        request.validate()?;
        self.inner
            .post(
                "cgi-bin/express/business/order/add",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn preview_express_template_typed(
        &self,
        access_token: impl Into<String>,
        request: ExpressPreviewTemplateRequest,
    ) -> Result<ExpressPreviewTemplateResponse> {
        request.validate()?;
        self.inner
            .post(
                "cgi-bin/express/delivery/template/preview",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn update_express_business_typed(
        &self,
        access_token: impl Into<String>,
        request: ExpressUpdateBusinessRequest,
    ) -> Result<WechatStatusResponse> {
        request.validate()?;
        self.inner
            .post(
                "cgi-bin/express/delivery/service/business/update",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn update_express_path_typed(
        &self,
        access_token: impl Into<String>,
        request: ExpressUpdatePathRequest,
    ) -> Result<WechatStatusResponse> {
        request.validate()?;
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
    ) -> Result<WxaCodeResponse> {
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
    ) -> Result<WxaCodeResponse> {
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
    ) -> Result<WxaCodeResponse> {
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
pub struct UniformMessageRequest {
    pub touser: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weapp_template_msg: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mp_template_msg: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatableMessageRequest {
    pub activity_id: String,
    pub target_state: i64,
    pub template_info: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataCubePerformanceDataRequest {
    pub cost_time_type: i64,
    pub default_start_time: i64,
    pub default_end_time: i64,
    pub device: i64,
    pub networktype: i64,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataCubeVisitTrendResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub list: Vec<DataCubeVisitTrendItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataCubeVisitTrendItem {
    #[serde(default)]
    pub ref_date: Option<String>,
    #[serde(default)]
    pub session_cnt: Option<i64>,
    #[serde(default)]
    pub visit_pv: Option<i64>,
    #[serde(default)]
    pub visit_uv: Option<i64>,
    #[serde(default)]
    pub visit_uv_new: Option<i64>,
    #[serde(default)]
    pub stay_time_uv: Option<f64>,
    #[serde(default)]
    pub stay_time_session: Option<f64>,
    #[serde(default)]
    pub visit_depth: Option<f64>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataCubeRetainInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub visit_uv_new: Vec<DataCubeRetainInfoItem>,
    #[serde(default)]
    pub visit_uv: Vec<DataCubeRetainInfoItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataCubeRetainInfoItem {
    #[serde(default)]
    pub key: Option<i64>,
    #[serde(default)]
    pub value: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataCubeVisitPageResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub list: Vec<DataCubeVisitPageItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataCubeVisitPageItem {
    #[serde(default)]
    pub page_path: Option<String>,
    #[serde(default)]
    pub page_visit_pv: Option<i64>,
    #[serde(default)]
    pub page_visit_uv: Option<i64>,
    #[serde(default)]
    pub page_staytime_pv: Option<f64>,
    #[serde(default)]
    pub entrypage_pv: Option<i64>,
    #[serde(default)]
    pub exitpage_pv: Option<i64>,
    #[serde(default)]
    pub page_share_pv: Option<i64>,
    #[serde(default)]
    pub page_share_uv: Option<i64>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataCubeUserPortraitResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub visit_uv_new: Option<Value>,
    #[serde(default)]
    pub visit_uv: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataCubePerformanceDataResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub data: Vec<Value>,
    #[serde(flatten)]
    pub extra: Value,
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
pub struct SecurityMsgSecCheckResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub result: Option<SecurityCheckResult>,
    #[serde(default)]
    pub detail: Vec<SecurityCheckDetail>,
    #[serde(default)]
    pub trace_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMediaCheckAsyncResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub trace_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityCheckResult {
    #[serde(default)]
    pub suggest: Option<String>,
    #[serde(default)]
    pub label: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityCheckDetail {
    #[serde(default)]
    pub strategy: Option<String>,
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub suggest: Option<String>,
    #[serde(default)]
    pub label: Option<i64>,
    #[serde(default)]
    pub prob: Option<f64>,
    #[serde(default)]
    pub keyword: Option<String>,
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

    pub fn validate(&self) -> Result<()> {
        match &self.payload {
            Value::Object(payload) if !payload.is_empty() => Ok(()),
            _ => Err(WechatError::Config(
                "mini-program operation payload must be a nonempty JSON object".to_string(),
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationDomainAction {
    Business,
    Server,
}

impl OperationDomainAction {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Business => "getbizdomain",
            Self::Server => "getserverdomain",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationJsErrListRequest {
    #[serde(rename = "appVersion")]
    pub app_version: String,
    #[serde(rename = "errType")]
    pub error_type: String,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "endTime")]
    pub end_time: String,
    pub keyword: String,
    pub openid: String,
    pub orderby: String,
    pub desc: String,
    pub offset: i64,
    pub limit: i64,
}

impl OperationJsErrListRequest {
    pub fn validate(&self) -> Result<()> {
        validate_operation_required("JS-error app version", &self.app_version)?;
        validate_operation_date_range(&self.start_time, &self.end_time)?;
        if !matches!(self.error_type.as_str(), "0" | "1" | "2" | "3") {
            return Err(WechatError::Config(
                "mini-program operation JS-error type must be 0, 1, 2, or 3".to_string(),
            ));
        }
        if !matches!(self.orderby.as_str(), "uv" | "pv") {
            return Err(WechatError::Config(
                "mini-program operation JS-error orderby must be uv or pv".to_string(),
            ));
        }
        if !matches!(self.desc.as_str(), "1" | "2") {
            return Err(WechatError::Config(
                "mini-program operation JS-error list desc must be 1 or 2".to_string(),
            ));
        }
        validate_operation_page(self.offset, self.limit, 30)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationJsErrDetailRequest {
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "endTime")]
    pub end_time: String,
    #[serde(rename = "errorMsgMd5")]
    pub error_message_md5: String,
    #[serde(rename = "errorStackMd5")]
    pub error_stack_md5: String,
    #[serde(rename = "appVersion")]
    pub app_version: String,
    #[serde(rename = "sdkVersion")]
    pub sdk_version: String,
    #[serde(rename = "osName")]
    pub os_name: String,
    #[serde(rename = "clientVersion")]
    pub client_version: String,
    pub openid: String,
    pub desc: String,
    pub offset: i64,
    pub limit: i64,
}

impl OperationJsErrDetailRequest {
    pub fn validate(&self) -> Result<()> {
        validate_operation_date_range(&self.start_time, &self.end_time)?;
        validate_operation_md5("error message MD5", &self.error_message_md5)?;
        validate_operation_md5("error stack MD5", &self.error_stack_md5)?;
        for (kind, value) in [
            ("app version", self.app_version.as_str()),
            ("SDK version", self.sdk_version.as_str()),
            ("client version", self.client_version.as_str()),
        ] {
            validate_operation_required(kind, value)?;
        }
        if !matches!(self.os_name.as_str(), "0" | "1" | "2" | "3") {
            return Err(WechatError::Config(
                "mini-program operation JS-error OS name must be 0, 1, 2, or 3".to_string(),
            ));
        }
        if !matches!(self.desc.as_str(), "0" | "1") {
            return Err(WechatError::Config(
                "mini-program operation JS-error detail desc must be 0 or 1".to_string(),
            ));
        }
        validate_operation_page(self.offset, self.limit, 30)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationJsErrSearchRequest {
    pub errmsg_keyword: String,
    #[serde(rename = "type")]
    pub query_type: i64,
    pub client_version: String,
    pub start_time: i64,
    pub end_time: i64,
    pub start: i64,
    pub limit: i64,
}

impl OperationJsErrSearchRequest {
    pub fn validate(&self) -> Result<()> {
        if !matches!(self.query_type, 1 | 2) {
            return Err(WechatError::Config(
                "mini-program operation legacy JS-error query type must be 1 or 2".to_string(),
            ));
        }
        validate_operation_timestamp_range(self.start_time, self.end_time)?;
        validate_operation_page(self.start, self.limit, 30)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationPerformanceRequest {
    pub cost_time_type: i64,
    pub default_start_time: i64,
    pub default_end_time: i64,
    pub device: String,
    pub is_download_code: String,
    pub scene: String,
    pub networktype: String,
}

impl OperationPerformanceRequest {
    pub fn validate(&self) -> Result<()> {
        if !matches!(self.cost_time_type, 1..=3) {
            return Err(WechatError::Config(
                "mini-program operation performance cost-time type must be 1, 2, or 3".to_string(),
            ));
        }
        validate_operation_timestamp_range(self.default_start_time, self.default_end_time)?;
        if !matches!(self.device.as_str(), "@_all:" | "1" | "2") {
            return Err(WechatError::Config(
                "mini-program operation performance device must be @_all:, 1, or 2".to_string(),
            ));
        }
        if !matches!(self.is_download_code.as_str(), "@_all:" | "1" | "2") {
            return Err(WechatError::Config(
                "mini-program operation download-code filter must be @_all:, 1, or 2".to_string(),
            ));
        }
        if !matches!(
            self.networktype.as_str(),
            "@_all:" | "wifi" | "4g" | "3g" | "2g"
        ) {
            return Err(WechatError::Config(
                "mini-program operation network type is unsupported".to_string(),
            ));
        }
        validate_operation_required("performance scene", &self.scene)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationRealTimeLogSearchRequest {
    pub date: String,
    pub begintime: i64,
    pub endtime: i64,
    #[serde(default)]
    pub start: i64,
    #[serde(default = "operation_default_log_limit")]
    pub limit: i64,
    #[serde(default, rename = "traceId", skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, rename = "filterMsg", skip_serializing_if = "Option::is_none")]
    pub filter_message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<i64>,
}

impl OperationRealTimeLogSearchRequest {
    pub fn validate(&self) -> Result<()> {
        let date = chrono::NaiveDate::parse_from_str(&self.date, "%Y%m%d").map_err(|err| {
            WechatError::Config(format!(
                "mini-program operation real-time-log date must be YYYYMMDD: {err}"
            ))
        })?;
        validate_operation_timestamp_range(self.begintime, self.endtime)?;
        for (kind, timestamp) in [("begin time", self.begintime), ("end time", self.endtime)] {
            let china_standard_timestamp = timestamp.checked_add(8 * 60 * 60).ok_or_else(|| {
                WechatError::Config(format!(
                    "mini-program operation real-time-log {kind} overflows China Standard Time"
                ))
            })?;
            let timestamp_date = chrono::DateTime::from_timestamp(china_standard_timestamp, 0)
                .ok_or_else(|| {
                    WechatError::Config(format!(
                        "mini-program operation real-time-log {kind} is outside the Unix timestamp range"
                    ))
                })?
                .date_naive();
            if timestamp_date != date {
                return Err(WechatError::Config(format!(
                    "mini-program operation real-time-log {kind} must fall on date {}",
                    self.date
                )));
            }
        }
        validate_operation_page(self.start, self.limit, 100)?;
        if !matches!(self.level, None | Some(2 | 4 | 8)) {
            return Err(WechatError::Config(
                "mini-program operation real-time-log level must be 2, 4, or 8".to_string(),
            ));
        }
        for (kind, value) in [
            ("trace id", self.trace_id.as_deref()),
            ("page URL", self.url.as_deref()),
            ("user id", self.id.as_deref()),
            ("filter message", self.filter_message.as_deref()),
        ] {
            if let Some(value) = value {
                validate_operation_required(kind, value)?;
            }
        }
        Ok(())
    }

    pub fn to_query(&self) -> Vec<(String, String)> {
        let mut query = vec![
            ("date".to_string(), self.date.clone()),
            ("begintime".to_string(), self.begintime.to_string()),
            ("endtime".to_string(), self.endtime.to_string()),
            ("start".to_string(), self.start.to_string()),
            ("limit".to_string(), self.limit.to_string()),
        ];
        for (name, value) in [
            ("traceId", self.trace_id.as_deref()),
            ("url", self.url.as_deref()),
            ("id", self.id.as_deref()),
            ("filterMsg", self.filter_message.as_deref()),
        ] {
            if let Some(value) = value {
                query.push((name.to_string(), value.to_string()));
            }
        }
        if let Some(level) = self.level {
            query.push(("level".to_string(), level.to_string()));
        }
        query
    }
}

fn operation_default_log_limit() -> i64 {
    20
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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationFeedbackResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub list: Vec<OperationFeedbackItem>,
    #[serde(default)]
    pub total_num: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationFeedbackItem {
    #[serde(default)]
    pub record_id: Option<i64>,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub phone: Option<String>,
    #[serde(default)]
    pub openid: Option<String>,
    #[serde(default)]
    pub create_time: Option<i64>,
    #[serde(default)]
    pub system_info: Option<String>,
    #[serde(default)]
    pub media_id: Vec<String>,
    #[serde(default, rename = "type")]
    pub feedback_type: Option<i64>,
    #[serde(default)]
    pub app_version: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationGrayReleasePlanResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub gray_release_plan: Option<OperationGrayReleasePlan>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationGrayReleasePlan {
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default)]
    pub gray_percentage: Option<i64>,
    #[serde(default)]
    pub create_timestamp: Option<i64>,
    #[serde(default)]
    pub default_finish_timestamp: Option<i64>,
    #[serde(default)]
    pub support_experiencer_first: Option<bool>,
    #[serde(default)]
    pub support_debuger_first: Option<bool>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
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
    pub data: Vec<OperationJsErrDetail>,
    #[serde(default, rename = "totalCount")]
    pub total_count: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationJsErrDetail {
    #[serde(default, rename = "errorMsg", alias = "message")]
    pub message: Option<String>,
    #[serde(default, rename = "errorStack", alias = "stack")]
    pub stack: Option<String>,
    #[serde(
        default,
        rename = "TimeStamp",
        alias = "time",
        alias = "timestamp",
        deserialize_with = "deserialize_optional_i64"
    )]
    pub time: Option<i64>,
    #[serde(default, rename = "appVersion", alias = "app_version")]
    pub app_version: Option<String>,
    #[serde(default, rename = "sdkVersion", alias = "sdk_version")]
    pub sdk_version: Option<String>,
    #[serde(default, rename = "clientVersion", alias = "client_version")]
    pub client_version: Option<String>,
    #[serde(default, rename = "DeviceModel", alias = "device")]
    pub device: Option<String>,
    #[serde(default, rename = "errorMsgMd5")]
    pub error_message_md5: Option<String>,
    #[serde(default, rename = "errorStackMd5")]
    pub error_stack_md5: Option<String>,
    #[serde(default, rename = "Count")]
    pub count: Option<i64>,
    #[serde(default, rename = "Ds")]
    pub ds: Option<String>,
    #[serde(default, rename = "osName")]
    pub os_name: Option<String>,
    #[serde(default)]
    pub openid: Option<String>,
    #[serde(default)]
    pub pluginversion: Option<String>,
    #[serde(default, rename = "appId")]
    pub app_id: Option<String>,
    #[serde(default)]
    pub source: Option<String>,
    #[serde(default)]
    pub route: Option<String>,
    #[serde(default, rename = "Uin")]
    pub uin: Option<String>,
    #[serde(default)]
    pub nickname: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
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
    pub data: Vec<OperationJsErrSummary>,
    #[serde(default, rename = "totalCount")]
    pub total_count: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationJsErrSummary {
    #[serde(default, rename = "errorMsg", alias = "message")]
    pub message: Option<String>,
    #[serde(default)]
    pub count: Option<i64>,
    #[serde(default, rename = "appVersion", alias = "app_version")]
    pub app_version: Option<String>,
    #[serde(default, rename = "sdkVersion", alias = "sdk_version")]
    pub sdk_version: Option<String>,
    #[serde(default, rename = "clientVersion", alias = "client_version")]
    pub client_version: Option<String>,
    #[serde(default)]
    pub first_time: Option<i64>,
    #[serde(default)]
    pub last_time: Option<i64>,
    #[serde(default, rename = "errorMsgMd5")]
    pub error_message_md5: Option<String>,
    #[serde(default, rename = "errorStackMd5")]
    pub error_stack_md5: Option<String>,
    #[serde(default)]
    pub uv: Option<i64>,
    #[serde(default)]
    pub pv: Option<i64>,
    #[serde(default, rename = "errorStack")]
    pub error_stack: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationJsErrSearchResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub results: Option<OperationJsErrSearchResults>,
    #[serde(default)]
    pub total: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationJsErrSearchResults {
    #[serde(default)]
    pub items: Vec<OperationJsErrDetail>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationSceneListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub scene: Vec<OperationScene>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationScene {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub value: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationVersionListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub cvlist: Vec<OperationClientVersion>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationClientVersion {
    #[serde(default, rename = "type")]
    pub version_type: Option<i64>,
    #[serde(default)]
    pub client_version_list: Vec<String>,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub percentage: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationRealTimeLogSearchResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub data: Option<OperationRealTimeLogData>,
    #[serde(default)]
    pub list: Vec<OperationRealTimeLogItem>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationRealTimeLogData {
    #[serde(default)]
    pub total: Option<i64>,
    #[serde(default)]
    pub has_next_page: Option<bool>,
    #[serde(default)]
    pub page: Option<i64>,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub list: Vec<OperationRealTimeLogItem>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationRealTimeLogItem {
    #[serde(default, deserialize_with = "deserialize_optional_string")]
    pub level: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub timestamp: Option<i64>,
    #[serde(default, rename = "traceid", alias = "trace_id")]
    pub trace_id: Option<String>,
    #[serde(default)]
    pub platform: Option<i64>,
    #[serde(default, rename = "libraryVersion")]
    pub library_version: Option<String>,
    #[serde(default, rename = "clientVersion")]
    pub client_version: Option<String>,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub msg: Vec<OperationRealTimeLogMessage>,
    #[serde(default, rename = "filterMsg")]
    pub filter_message: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationRealTimeLogMessage {
    #[serde(default)]
    pub time: Option<i64>,
    #[serde(default)]
    pub msg: Vec<Value>,
    #[serde(default)]
    pub level: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationGrayReleaseStatusKind {
    Initial,
    Running,
    Paused,
    Finished,
    Deleted,
    Other(i64),
}

impl From<i64> for OperationGrayReleaseStatusKind {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::Initial,
            1 => Self::Running,
            2 => Self::Paused,
            3 => Self::Finished,
            4 => Self::Deleted,
            other => Self::Other(other),
        }
    }
}

impl OperationDomainInfoResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_operation_response_success(self.errcode, self.errmsg.as_deref())?;
        let mut domains = std::collections::HashSet::new();
        for (kind, values) in [
            ("request", self.requestdomain.as_slice()),
            ("WebSocket request", self.wsrequestdomain.as_slice()),
            ("upload", self.uploaddomain.as_slice()),
            ("download", self.downloaddomain.as_slice()),
            ("UDP", self.udpdomain.as_slice()),
            ("business", self.bizdomain.as_slice()),
        ] {
            for value in values {
                validate_operation_required(&format!("{kind} domain"), value)?;
                if !domains.insert((kind, value)) {
                    return Err(WechatError::Config(format!(
                        "mini-program operation {kind} domain list contains duplicates"
                    )));
                }
            }
        }
        Ok(())
    }
}

impl OperationFeedbackResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_operation_response_success(self.errcode, self.errmsg.as_deref())?;
        let total = self.total_num.unwrap_or(self.list.len() as i64);
        if total < 0
            || usize::try_from(total)
                .ok()
                .is_some_and(|total| total < self.list.len())
        {
            return Err(WechatError::Config(
                "mini-program operation feedback total is inconsistent with the returned list"
                    .to_string(),
            ));
        }
        let mut record_ids = std::collections::HashSet::with_capacity(self.list.len());
        for item in &self.list {
            let record_id = item.record_id.ok_or_else(|| {
                WechatError::Config(
                    "mini-program operation feedback item is missing record id".to_string(),
                )
            })?;
            if record_id <= 0 || !record_ids.insert(record_id) {
                return Err(WechatError::Config(
                    "mini-program operation feedback record ids must be positive and unique"
                        .to_string(),
                ));
            }
            if item.create_time.is_some_and(|time| time < 0) {
                return Err(WechatError::Config(
                    "mini-program operation feedback create time cannot be negative".to_string(),
                ));
            }
            let mut media_ids = std::collections::HashSet::with_capacity(item.media_id.len());
            for media_id in &item.media_id {
                validate_operation_required("feedback media id", media_id)?;
                if !media_ids.insert(media_id) {
                    return Err(WechatError::Config(
                        "mini-program operation feedback item contains duplicate media ids"
                            .to_string(),
                    ));
                }
            }
        }
        Ok(())
    }

    pub fn next_page(&self, page: i64, num: i64) -> Result<Option<i64>> {
        validate_operation_feedback_query(0, page, num)?;
        self.validate()?;
        let consumed = page.checked_mul(num).ok_or_else(|| {
            WechatError::Config("mini-program operation feedback pagination overflowed".to_string())
        })?;
        if consumed >= self.total_num.unwrap_or(self.list.len() as i64) {
            Ok(None)
        } else {
            page.checked_add(1).map(Some).ok_or_else(|| {
                WechatError::Config(
                    "mini-program operation feedback next page overflowed".to_string(),
                )
            })
        }
    }
}

impl OperationGrayReleasePlan {
    pub fn status_kind(&self) -> Option<OperationGrayReleaseStatusKind> {
        self.status.map(OperationGrayReleaseStatusKind::from)
    }

    pub fn validate(&self) -> Result<()> {
        if self
            .gray_percentage
            .is_some_and(|value| !(0..=100).contains(&value))
        {
            return Err(WechatError::Config(
                "mini-program operation gray percentage must be between 0 and 100".to_string(),
            ));
        }
        for (kind, value) in [
            ("create timestamp", self.create_timestamp),
            ("default finish timestamp", self.default_finish_timestamp),
        ] {
            if value.is_some_and(|value| value < 0) {
                return Err(WechatError::Config(format!(
                    "mini-program operation gray-release {kind} cannot be negative"
                )));
            }
        }
        if let (Some(created), Some(finished)) =
            (self.create_timestamp, self.default_finish_timestamp)
        {
            if finished < created {
                return Err(WechatError::Config(
                    "mini-program operation gray-release finish time precedes creation".to_string(),
                ));
            }
        }
        Ok(())
    }
}

impl OperationGrayReleasePlanResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_operation_response_success(self.errcode, self.errmsg.as_deref())?;
        if let Some(plan) = &self.gray_release_plan {
            plan.validate()?;
        }
        Ok(())
    }
}

impl OperationJsErrDetailResponse {
    pub fn validate(&self) -> Result<()> {
        validate_operation_js_response(
            self.errcode,
            self.errmsg.as_deref(),
            self.success,
            self.total_count,
            self.data.len(),
        )?;
        for detail in &self.data {
            detail.validate()?;
        }
        Ok(())
    }
}

impl OperationJsErrDetail {
    pub fn validate(&self) -> Result<()> {
        if self.count.is_some_and(|count| count < 0) || self.time.is_some_and(|time| time < 0) {
            return Err(WechatError::Config(
                "mini-program operation JS-error count/time cannot be negative".to_string(),
            ));
        }
        for (kind, digest) in [
            ("error message MD5", self.error_message_md5.as_deref()),
            ("error stack MD5", self.error_stack_md5.as_deref()),
        ] {
            if let Some(digest) = digest {
                validate_operation_md5(kind, digest)?;
            }
        }
        Ok(())
    }
}

impl OperationJsErrListResponse {
    pub fn validate(&self) -> Result<()> {
        validate_operation_js_response(
            self.errcode,
            self.errmsg.as_deref(),
            self.success,
            self.total_count,
            self.data.len(),
        )?;
        for item in &self.data {
            if item.count.is_some_and(|count| count < 0)
                || item.uv.is_some_and(|count| count < 0)
                || item.pv.is_some_and(|count| count < 0)
            {
                return Err(WechatError::Config(
                    "mini-program operation JS-error counters cannot be negative".to_string(),
                ));
            }
            if let (Some(first), Some(last)) = (item.first_time, item.last_time) {
                if last < first {
                    return Err(WechatError::Config(
                        "mini-program operation JS-error last time precedes first time".to_string(),
                    ));
                }
            }
            for (kind, digest) in [
                ("error message MD5", item.error_message_md5.as_deref()),
                ("error stack MD5", item.error_stack_md5.as_deref()),
            ] {
                if let Some(digest) = digest {
                    validate_operation_md5(kind, digest)?;
                }
            }
        }
        Ok(())
    }

    pub fn next_offset(&self, offset: i64, limit: i64) -> Result<Option<i64>> {
        validate_operation_page(offset, limit, 30)?;
        self.validate()?;
        let next = offset.checked_add(self.data.len() as i64).ok_or_else(|| {
            WechatError::Config("mini-program operation JS-error pagination overflowed".to_string())
        })?;
        if next >= self.total_count.unwrap_or(next) {
            Ok(None)
        } else if self.data.is_empty() {
            Err(WechatError::Config(
                "mini-program operation JS-error pagination stalled before total count".to_string(),
            ))
        } else {
            Ok(Some(next))
        }
    }
}

impl OperationJsErrSearchResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_operation_response_success(self.errcode, self.errmsg.as_deref())?;
        let items = self
            .results
            .as_ref()
            .map(|results| results.items.as_slice())
            .unwrap_or_default();
        let total = self.total.unwrap_or(items.len() as i64);
        if total < 0
            || usize::try_from(total)
                .ok()
                .is_some_and(|total| total < items.len())
        {
            return Err(WechatError::Config(
                "mini-program operation legacy JS-error total is inconsistent".to_string(),
            ));
        }
        for item in items {
            item.validate()?;
        }
        Ok(())
    }
}

impl OperationPerformanceResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_operation_response_success(self.errcode, self.errmsg.as_deref())?;
        self.default_time_data_value()?;
        self.compare_time_data_value()?;
        Ok(())
    }

    pub fn default_time_data_value(&self) -> Result<Option<Value>> {
        parse_operation_json_string(
            "default performance data",
            self.default_time_data.as_deref(),
        )
    }

    pub fn compare_time_data_value(&self) -> Result<Option<Value>> {
        parse_operation_json_string(
            "comparison performance data",
            self.compare_time_data.as_deref(),
        )
    }
}

impl OperationSceneListResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_operation_response_success(self.errcode, self.errmsg.as_deref())?;
        let mut values = std::collections::HashSet::with_capacity(self.scene.len());
        for scene in &self.scene {
            validate_operation_required("scene name", scene.name.as_deref().unwrap_or_default())?;
            let value = scene.value.ok_or_else(|| {
                WechatError::Config("mini-program operation scene is missing value".to_string())
            })?;
            if value < 0 || !values.insert(value) {
                return Err(WechatError::Config(
                    "mini-program operation scene values must be non-negative and unique"
                        .to_string(),
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationVersionTypeKind {
    Client,
    ServiceDirect,
    Other(i64),
}

impl From<i64> for OperationVersionTypeKind {
    fn from(value: i64) -> Self {
        match value {
            1 => Self::Client,
            2 => Self::ServiceDirect,
            other => Self::Other(other),
        }
    }
}

impl OperationClientVersion {
    pub fn version_type_kind(&self) -> Option<OperationVersionTypeKind> {
        self.version_type.map(OperationVersionTypeKind::from)
    }
}

impl OperationVersionListResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_operation_response_success(self.errcode, self.errmsg.as_deref())?;
        let mut types = std::collections::HashSet::new();
        for item in &self.cvlist {
            if let Some(kind) = item.version_type {
                if kind <= 0 || !types.insert(kind) {
                    return Err(WechatError::Config(
                        "mini-program operation client-version types must be positive and unique"
                            .to_string(),
                    ));
                }
            }
            if item
                .percentage
                .is_some_and(|value| !(0..=100).contains(&value))
            {
                return Err(WechatError::Config(
                    "mini-program operation client-version percentage must be between 0 and 100"
                        .to_string(),
                ));
            }
            let mut versions = std::collections::HashSet::new();
            for version in &item.client_version_list {
                validate_operation_required("client version", version)?;
                if !versions.insert(version) {
                    return Err(WechatError::Config(
                        "mini-program operation client-version list contains duplicates"
                            .to_string(),
                    ));
                }
            }
        }
        Ok(())
    }
}

impl OperationRealTimeLogItem {
    pub fn level_bits(&self) -> Result<Option<i64>> {
        self.level
            .as_deref()
            .map(|level| {
                level.parse::<i64>().map_err(|err| {
                    WechatError::Config(format!(
                        "mini-program operation real-time-log level is invalid: {err}"
                    ))
                })
            })
            .transpose()
    }

    pub fn validate(&self) -> Result<()> {
        if self.timestamp.is_some_and(|time| time < 0) {
            return Err(WechatError::Config(
                "mini-program operation real-time-log timestamp cannot be negative".to_string(),
            ));
        }
        if self
            .platform
            .is_some_and(|platform| !matches!(platform, 1 | 2))
        {
            return Err(WechatError::Config(
                "mini-program operation real-time-log platform must be 1 or 2".to_string(),
            ));
        }
        let level = self.level_bits()?;
        if level.is_some_and(|level| level < 0 || level & !14 != 0) {
            return Err(WechatError::Config(
                "mini-program operation real-time-log aggregate level contains unknown bits"
                    .to_string(),
            ));
        }
        for message in &self.msg {
            if message.time.is_some_and(|time| time < 0)
                || message
                    .level
                    .is_some_and(|level| !matches!(level, 2 | 4 | 8))
            {
                return Err(WechatError::Config(
                    "mini-program operation real-time-log message has invalid time or level"
                        .to_string(),
                ));
            }
        }
        Ok(())
    }
}

impl OperationRealTimeLogSearchResponse {
    pub fn items(&self) -> &[OperationRealTimeLogItem] {
        self.data
            .as_ref()
            .filter(|data| !data.list.is_empty())
            .map(|data| data.list.as_slice())
            .unwrap_or(self.list.as_slice())
    }

    pub fn validate(&self) -> Result<()> {
        ensure_operation_response_success(self.errcode, self.errmsg.as_deref())?;
        let items = self.items();
        let total = self
            .data
            .as_ref()
            .and_then(|data| data.total)
            .unwrap_or(items.len() as i64);
        if total < 0
            || usize::try_from(total)
                .ok()
                .is_some_and(|total| total < items.len())
        {
            return Err(WechatError::Config(
                "mini-program operation real-time-log total is inconsistent".to_string(),
            ));
        }
        let mut trace_timestamps = std::collections::HashSet::with_capacity(items.len());
        for item in items {
            item.validate()?;
            if let (Some(trace_id), Some(timestamp)) = (item.trace_id.as_deref(), item.timestamp) {
                if !trace_timestamps.insert((trace_id, timestamp)) {
                    return Err(WechatError::Config(
                        "mini-program operation real-time-log response contains duplicate entries"
                            .to_string(),
                    ));
                }
            }
        }
        Ok(())
    }

    pub fn next_start(&self, start: i64, limit: i64) -> Result<Option<i64>> {
        validate_operation_page(start, limit, 100)?;
        self.validate()?;
        let next = start
            .checked_add(self.items().len() as i64)
            .ok_or_else(|| {
                WechatError::Config(
                    "mini-program operation real-time-log pagination overflowed".to_string(),
                )
            })?;
        let total = self
            .data
            .as_ref()
            .and_then(|data| data.total)
            .unwrap_or(next);
        if next >= total {
            Ok(None)
        } else if self.items().is_empty() {
            Err(WechatError::Config(
                "mini-program operation real-time-log pagination stalled before total".to_string(),
            ))
        } else {
            Ok(Some(next))
        }
    }
}

fn validate_operation_required(kind: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(WechatError::Config(format!(
            "mini-program operation {kind} must not be blank"
        )));
    }
    Ok(())
}

fn validate_operation_domain_action(action: &str) -> Result<()> {
    if !matches!(action, "getbizdomain" | "getserverdomain") {
        return Err(WechatError::Config(
            "mini-program operation domain action must be getbizdomain or getserverdomain"
                .to_string(),
        ));
    }
    Ok(())
}

fn validate_operation_feedback_query(feedback_type: i64, page: i64, num: i64) -> Result<()> {
    if !(0..=8).contains(&feedback_type) {
        return Err(WechatError::Config(
            "mini-program operation feedback type must be between 0 and 8".to_string(),
        ));
    }
    if page <= 0 || !(1..=20).contains(&num) {
        return Err(WechatError::Config(
            "mini-program operation feedback page must be positive and num must be 1..=20"
                .to_string(),
        ));
    }
    Ok(())
}

fn validate_operation_page(offset: i64, limit: i64, maximum: i64) -> Result<()> {
    if offset < 0 || !(1..=maximum).contains(&limit) {
        return Err(WechatError::Config(format!(
            "mini-program operation offset must be non-negative and limit must be 1..={maximum}"
        )));
    }
    Ok(())
}

fn validate_operation_date_range(start: &str, end: &str) -> Result<()> {
    let start = chrono::NaiveDate::parse_from_str(start, "%Y-%m-%d").map_err(|err| {
        WechatError::Config(format!(
            "mini-program operation start date must be YYYY-MM-DD: {err}"
        ))
    })?;
    let end = chrono::NaiveDate::parse_from_str(end, "%Y-%m-%d").map_err(|err| {
        WechatError::Config(format!(
            "mini-program operation end date must be YYYY-MM-DD: {err}"
        ))
    })?;
    if end < start {
        return Err(WechatError::Config(
            "mini-program operation end date precedes start date".to_string(),
        ));
    }
    Ok(())
}

fn validate_operation_timestamp_range(start: i64, end: i64) -> Result<()> {
    if start <= 0 || end <= 0 || end < start {
        return Err(WechatError::Config(
            "mini-program operation timestamps must be positive and ordered".to_string(),
        ));
    }
    Ok(())
}

fn validate_operation_md5(kind: &str, digest: &str) -> Result<()> {
    if digest.len() != 32 || !digest.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(WechatError::Config(format!(
            "mini-program operation {kind} must be a 32-character hexadecimal digest"
        )));
    }
    Ok(())
}

fn ensure_operation_response_success(errcode: Option<i64>, errmsg: Option<&str>) -> Result<()> {
    if let Some(code) = errcode.filter(|code| *code != 0) {
        return Err(WechatError::Api {
            code,
            message: errmsg
                .unwrap_or("mini-program operation API error")
                .to_string(),
        });
    }
    Ok(())
}

fn validate_operation_js_response(
    errcode: Option<i64>,
    errmsg: Option<&str>,
    success: Option<bool>,
    total: Option<i64>,
    count: usize,
) -> Result<()> {
    ensure_operation_response_success(errcode, errmsg)?;
    if success == Some(false) {
        return Err(WechatError::Config(
            "mini-program operation JS-error query reported success=false".to_string(),
        ));
    }
    let total = total.unwrap_or(count as i64);
    if total < 0
        || usize::try_from(total)
            .ok()
            .is_some_and(|total| total < count)
    {
        return Err(WechatError::Config(
            "mini-program operation JS-error total is inconsistent with returned data".to_string(),
        ));
    }
    Ok(())
}

fn parse_operation_json_string(kind: &str, value: Option<&str>) -> Result<Option<Value>> {
    value
        .map(|value| {
            serde_json::from_str(value).map_err(|err| {
                WechatError::Config(format!(
                    "mini-program operation {kind} is not valid JSON: {err}"
                ))
            })
        })
        .transpose()
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
pub struct MiniDramaVideoMediaTaskInfo {
    #[serde(default)]
    pub id: Option<i64>,
    #[serde(default)]
    pub task_id: Option<i64>,
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default)]
    pub media_id: Option<i64>,
    #[serde(default)]
    pub err_msg: Option<String>,
    #[serde(default)]
    pub create_time: Option<i64>,
    #[serde(default)]
    pub update_time: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaVideoMediaTaskResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub task_info: Option<MiniDramaVideoMediaTaskInfo>,
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
pub struct MiniDramaMediaInfo {
    #[serde(default)]
    pub media_id: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub media_url: Option<String>,
    #[serde(default)]
    pub cover_url: Option<String>,
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default)]
    pub duration: Option<i64>,
    #[serde(default)]
    pub size: Option<i64>,
    #[serde(default)]
    pub create_time: Option<i64>,
    #[serde(default)]
    pub update_time: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaMediaListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub media_info_list: Vec<MiniDramaMediaInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaMediaInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub media_info: Option<MiniDramaMediaInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaMediaLinkResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub media_info: Option<MiniDramaMediaInfo>,
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
pub struct MiniDramaInfo {
    #[serde(default)]
    pub drama_id: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub media_count: Option<i64>,
    #[serde(default)]
    pub cover_material_id: Option<String>,
    #[serde(default)]
    pub promotion_poster_material_id: Option<String>,
    #[serde(default)]
    pub producer: Option<String>,
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default)]
    pub create_time: Option<i64>,
    #[serde(default)]
    pub update_time: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub drama_info_list: Vec<MiniDramaInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub drama_info: Option<MiniDramaInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaAuditDetail {
    #[serde(default)]
    pub drama_id: Option<i64>,
    #[serde(default)]
    pub audit_id: Option<i64>,
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default)]
    pub reason: Option<String>,
    #[serde(default)]
    pub create_time: Option<i64>,
    #[serde(default)]
    pub update_time: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaAuditInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub audit_detail: Option<MiniDramaAuditDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaCdnUsageItem {
    #[serde(default)]
    pub time: Option<i64>,
    #[serde(default)]
    pub value: Option<i64>,
    #[serde(default)]
    pub flux: Option<i64>,
    #[serde(default)]
    pub bandwidth: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
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
    pub item_list: Vec<MiniDramaCdnUsageItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaCdnLog {
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub start_time: Option<i64>,
    #[serde(default)]
    pub end_time: Option<i64>,
    #[serde(default)]
    pub size: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaCdnLogsResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub domestic_cdn_logs: Vec<MiniDramaCdnLog>,
    #[serde(default)]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaPackageInfo {
    #[serde(default)]
    pub package_id: Option<i64>,
    #[serde(default)]
    pub drama_id: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default)]
    pub create_time: Option<i64>,
    #[serde(default)]
    pub update_time: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaPackageListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub package_list: Vec<MiniDramaPackageInfo>,
    #[serde(default)]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaAuthorizationResult {
    #[serde(default)]
    pub drama_id: Option<i64>,
    #[serde(default)]
    pub authorized_appid: Option<String>,
    #[serde(default)]
    pub result_code: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaAuthorizationResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub result: Vec<MiniDramaAuthorizationResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaAuthorizationObject {
    #[serde(default)]
    pub drama_id: Option<i64>,
    #[serde(default)]
    pub authorized_appid: Option<String>,
    #[serde(default)]
    pub authorized_time: Option<i64>,
    #[serde(default)]
    pub authz_expire_time: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDramaAuthorizationSearchResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub objects: Vec<MiniDramaAuthorizationObject>,
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
    pub objects: Vec<MiniDramaAuthorizationObject>,
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

    pub fn validate(&self) -> Result<()> {
        match &self.payload {
            Value::Object(payload) if !payload.is_empty() => Ok(()),
            _ => Err(WechatError::Config(
                "mini-program immediate-delivery payload must be a nonempty JSON object"
                    .to_string(),
            )),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmediateDeliveryOrderIdentity {
    pub delivery_id: String,
    pub shop_order_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub openid: Option<String>,
    pub shop_no: String,
    pub shopid: String,
    pub delivery_sign: String,
}

impl ImmediateDeliveryOrderIdentity {
    pub fn signed(
        delivery_id: impl Into<String>,
        shop_order_id: impl Into<String>,
        openid: impl Into<String>,
        shop_no: impl Into<String>,
        shopid: impl Into<String>,
        app_secret: impl AsRef<str>,
    ) -> Result<Self> {
        let mut identity = Self {
            delivery_id: delivery_id.into(),
            shop_order_id: shop_order_id.into(),
            openid: Some(openid.into()),
            shop_no: shop_no.into(),
            shopid: shopid.into(),
            delivery_sign: String::new(),
        };
        identity.sign(app_secret.as_ref())?;
        Ok(identity)
    }

    pub fn signed_without_openid(
        delivery_id: impl Into<String>,
        shop_order_id: impl Into<String>,
        shop_no: impl Into<String>,
        shopid: impl Into<String>,
        app_secret: impl AsRef<str>,
    ) -> Result<Self> {
        let mut identity = Self {
            delivery_id: delivery_id.into(),
            shop_order_id: shop_order_id.into(),
            openid: None,
            shop_no: shop_no.into(),
            shopid: shopid.into(),
            delivery_sign: String::new(),
        };
        identity.sign(app_secret.as_ref())?;
        Ok(identity)
    }

    fn sign(&mut self, app_secret: &str) -> Result<()> {
        for (kind, value) in [
            ("delivery id", self.delivery_id.as_str()),
            ("shop order id", self.shop_order_id.as_str()),
            ("shop number", self.shop_no.as_str()),
            ("shop id", self.shopid.as_str()),
            ("app secret", app_secret),
        ] {
            validate_immediate_delivery_required(kind, value)?;
        }
        self.delivery_sign = crypto::sha1_hex(
            format!("{}{}{}", self.shopid, self.shop_order_id, app_secret).as_bytes(),
        );
        self.validate()
    }

    pub fn validate(&self) -> Result<()> {
        for (kind, value) in [
            ("delivery id", self.delivery_id.as_str()),
            ("shop order id", self.shop_order_id.as_str()),
            ("shop number", self.shop_no.as_str()),
            ("shop id", self.shopid.as_str()),
            ("delivery signature", self.delivery_sign.as_str()),
        ] {
            validate_immediate_delivery_required(kind, value)?;
        }
        if let Some(openid) = &self.openid {
            validate_immediate_delivery_required("openid", openid)?;
        }
        validate_immediate_delivery_max_bytes("shop order id", &self.shop_order_id, 128)?;
        validate_immediate_delivery_signature(&self.delivery_sign)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmediateDeliveryContact {
    pub name: String,
    pub city: String,
    pub address: String,
    pub address_detail: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coordinate_type: Option<i64>,
    pub lng: f64,
    pub lat: f64,
    pub phone: String,
}

impl ImmediateDeliveryContact {
    pub fn validate(&self, kind: &str) -> Result<()> {
        for (field, value) in [
            ("name", self.name.as_str()),
            ("city", self.city.as_str()),
            ("address", self.address.as_str()),
            ("address detail", self.address_detail.as_str()),
            ("phone", self.phone.as_str()),
        ] {
            validate_immediate_delivery_required(&format!("{kind} {field}"), value)?;
        }
        validate_immediate_delivery_max_chars(&format!("{kind} name"), &self.name, 256)?;
        validate_immediate_delivery_max_chars(&format!("{kind} phone"), &self.phone, 64)?;
        if !matches!(self.coordinate_type, None | Some(0 | 1)) {
            return Err(WechatError::Config(format!(
                "mini-program immediate-delivery {kind} coordinate type must be 0 or 1"
            )));
        }
        validate_immediate_delivery_number(
            &format!("{kind} longitude"),
            self.lng,
            -180.0,
            180.0,
            true,
        )?;
        validate_immediate_delivery_number(&format!("{kind} latitude"), self.lat, -90.0, 90.0, true)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmediateDeliveryGoods {
    pub good_count: i64,
    pub good_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub good_price: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub good_unit: Option<String>,
}

impl ImmediateDeliveryGoods {
    pub fn validate(&self) -> Result<()> {
        if self.good_count <= 0 {
            return Err(WechatError::Config(
                "mini-program immediate-delivery goods count must be positive".to_string(),
            ));
        }
        validate_immediate_delivery_required("goods name", &self.good_name)?;
        validate_immediate_delivery_max_chars(
            "goods unit",
            self.good_unit.as_deref().unwrap_or(""),
            20,
        )?;
        if let Some(price) = self.good_price {
            validate_immediate_delivery_money("goods price", price, 0.0, f64::MAX)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmediateDeliveryGoodsDetail {
    pub goods: Vec<ImmediateDeliveryGoods>,
}

impl ImmediateDeliveryGoodsDetail {
    pub fn validate(&self) -> Result<()> {
        if self.goods.is_empty() {
            return Err(WechatError::Config(
                "mini-program immediate-delivery goods detail must not be empty".to_string(),
            ));
        }
        for goods in &self.goods {
            goods.validate()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmediateDeliveryCargo {
    pub goods_value: f64,
    pub goods_weight: f64,
    pub cargo_first_class: String,
    pub cargo_second_class: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub goods_height: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub goods_length: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub goods_width: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub goods_detail: Option<ImmediateDeliveryGoodsDetail>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub goods_pickup_info: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub goods_delivery_info: Option<String>,
}

impl ImmediateDeliveryCargo {
    pub fn validate(&self) -> Result<()> {
        validate_immediate_delivery_money("goods value", self.goods_value, 0.0, 5000.0)?;
        validate_immediate_delivery_number("goods weight", self.goods_weight, 0.0, 50.0, false)?;
        for (kind, value, maximum) in [
            ("goods height", self.goods_height, 45.0),
            ("goods length", self.goods_length, 65.0),
            ("goods width", self.goods_width, 50.0),
        ] {
            if let Some(value) = value {
                validate_immediate_delivery_number(kind, value, 0.0, maximum, false)?;
            }
        }
        validate_immediate_delivery_required("cargo first class", &self.cargo_first_class)?;
        validate_immediate_delivery_required("cargo second class", &self.cargo_second_class)?;
        validate_immediate_delivery_max_chars(
            "goods pickup information",
            self.goods_pickup_info.as_deref().unwrap_or(""),
            100,
        )?;
        validate_immediate_delivery_max_chars(
            "goods delivery information",
            self.goods_delivery_info.as_deref().unwrap_or(""),
            100,
        )?;
        if let Some(goods_detail) = &self.goods_detail {
            goods_detail.validate()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ImmediateDeliveryOrderInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivery_service_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_type: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_delivery_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_finish_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_pick_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poi_seq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_insured: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub declared_value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tips: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_direct_delivery: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cash_on_delivery: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cash_on_pickup: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rider_pick_method: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_finish_code_needed: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_pickup_code_needed: Option<i64>,
}

impl ImmediateDeliveryOrderInfo {
    pub fn validate(&self) -> Result<()> {
        if !matches!(self.order_type, None | Some(0 | 1)) {
            return Err(WechatError::Config(
                "mini-program immediate-delivery order type must be 0 or 1".to_string(),
            ));
        }
        if self.order_type == Some(1)
            && self.expected_delivery_time.is_none()
            && self.expected_finish_time.is_none()
            && self.expected_pick_time.is_none()
        {
            return Err(WechatError::Config(
                "mini-program immediate-delivery scheduled order requires an expected time"
                    .to_string(),
            ));
        }
        for (kind, value) in [
            ("expected delivery time", self.expected_delivery_time),
            ("expected finish time", self.expected_finish_time),
            ("expected pickup time", self.expected_pick_time),
            ("order time", self.order_time),
        ] {
            if value.is_some_and(|value| value <= 0) {
                return Err(WechatError::Config(format!(
                    "mini-program immediate-delivery {kind} must be a positive Unix timestamp"
                )));
            }
        }
        validate_immediate_delivery_max_chars(
            "order note",
            self.note.as_deref().unwrap_or(""),
            200,
        )?;
        validate_immediate_delivery_max_chars(
            "point-of-interest sequence",
            self.poi_seq.as_deref().unwrap_or(""),
            32,
        )?;
        for (kind, value) in [
            ("insured flag", self.is_insured),
            ("direct-delivery flag", self.is_direct_delivery),
            ("finish-code flag", self.is_finish_code_needed),
            ("pickup-code flag", self.is_pickup_code_needed),
        ] {
            if !matches!(value, None | Some(0 | 1)) {
                return Err(WechatError::Config(format!(
                    "mini-program immediate-delivery {kind} must be 0 or 1"
                )));
            }
        }
        if self.is_insured == Some(1) && self.declared_value.is_none() {
            return Err(WechatError::Config(
                "mini-program immediate-delivery insured order requires declared value".to_string(),
            ));
        }
        for (kind, value) in [
            ("declared value", self.declared_value),
            ("cash on delivery", self.cash_on_delivery),
            ("cash on pickup", self.cash_on_pickup),
        ] {
            if let Some(value) = value {
                validate_immediate_delivery_money(kind, value, 0.0, f64::MAX)?;
            }
        }
        if self.tips.is_some_and(|tips| tips < 0) {
            return Err(WechatError::Config(
                "mini-program immediate-delivery tips must not be negative".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmediateDeliveryShopInfo {
    pub goods_count: i64,
    pub goods_name: String,
    pub img_url: String,
    pub wxa_path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wxa_appid: Option<String>,
}

impl ImmediateDeliveryShopInfo {
    pub fn validate(&self) -> Result<()> {
        if self.goods_count <= 0 {
            return Err(WechatError::Config(
                "mini-program immediate-delivery shop goods count must be positive".to_string(),
            ));
        }
        for (kind, value) in [
            ("shop goods name", self.goods_name.as_str()),
            ("shop image URL", self.img_url.as_str()),
            ("shop mini-program path", self.wxa_path.as_str()),
        ] {
            validate_immediate_delivery_required(kind, value)?;
        }
        validate_immediate_delivery_https_url("shop image URL", &self.img_url)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmediateDeliveryAddOrderRequest {
    #[serde(flatten)]
    pub identity: ImmediateDeliveryOrderIdentity,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_biz_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender: Option<ImmediateDeliveryContact>,
    pub receiver: ImmediateDeliveryContact,
    pub cargo: ImmediateDeliveryCargo,
    pub order_info: ImmediateDeliveryOrderInfo,
    pub shop: ImmediateDeliveryShopInfo,
}

impl ImmediateDeliveryAddOrderRequest {
    pub fn validate(&self) -> Result<()> {
        self.identity.validate()?;
        validate_immediate_delivery_required(
            "openid",
            self.identity.openid.as_deref().unwrap_or(""),
        )?;
        if let Some(sender) = &self.sender {
            sender.validate("sender")?;
        }
        self.receiver.validate("receiver")?;
        self.cargo.validate()?;
        self.order_info.validate()?;
        self.shop.validate()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmediateDeliveryCancelOrderRequest {
    #[serde(flatten)]
    pub identity: ImmediateDeliveryOrderIdentity,
    pub waybill_id: String,
    pub cancel_reason_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancel_reason: Option<String>,
}

impl ImmediateDeliveryCancelOrderRequest {
    pub fn validate(&self) -> Result<()> {
        self.identity.validate()?;
        validate_immediate_delivery_required("waybill id", &self.waybill_id)?;
        if self.cancel_reason_id <= 0 {
            return Err(WechatError::Config(
                "mini-program immediate-delivery cancel reason id must be positive".to_string(),
            ));
        }
        validate_immediate_delivery_max_chars(
            "cancel reason",
            self.cancel_reason.as_deref().unwrap_or(""),
            200,
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmediateDeliveryAbnormalConfirmRequest {
    #[serde(flatten)]
    pub identity: ImmediateDeliveryOrderIdentity,
    pub waybill_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
}

impl ImmediateDeliveryAbnormalConfirmRequest {
    pub fn validate(&self) -> Result<()> {
        self.identity.validate()?;
        validate_immediate_delivery_required("waybill id", &self.waybill_id)?;
        validate_immediate_delivery_max_chars(
            "abnormal-confirm remark",
            self.remark.as_deref().unwrap_or(""),
            200,
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmediateDeliveryAddTipRequest {
    #[serde(flatten)]
    pub identity: ImmediateDeliveryOrderIdentity,
    pub waybill_id: String,
    pub tips: i64,
}

impl ImmediateDeliveryAddTipRequest {
    pub fn validate(&self) -> Result<()> {
        self.identity.validate()?;
        validate_immediate_delivery_required("waybill id", &self.waybill_id)?;
        if self.tips <= 0 {
            return Err(WechatError::Config(
                "mini-program immediate-delivery added tips must be positive".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmediateDeliveryMockUpdateOrderRequest {
    pub shopid: String,
    pub shop_order_id: String,
    pub action_time: i64,
    pub order_status: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_msg: Option<String>,
}

impl ImmediateDeliveryMockUpdateOrderRequest {
    pub fn validate(&self, real_environment: bool) -> Result<()> {
        validate_immediate_delivery_required("mock shop id", &self.shopid)?;
        validate_immediate_delivery_required("mock shop order id", &self.shop_order_id)?;
        if !real_environment && self.shopid != "test_shop_id" {
            return Err(WechatError::Config(
                "mini-program immediate-delivery sandbox mock shop id must be test_shop_id"
                    .to_string(),
            ));
        }
        if self.action_time <= 0 {
            return Err(WechatError::Config(
                "mini-program immediate-delivery mock action time must be positive".to_string(),
            ));
        }
        if matches!(
            ImmediateDeliveryOrderStatusKind::from(self.order_status),
            ImmediateDeliveryOrderStatusKind::Other(_)
        ) {
            return Err(WechatError::Config(
                "mini-program immediate-delivery mock order status is unsupported".to_string(),
            ));
        }
        validate_immediate_delivery_max_chars(
            "mock action message",
            self.action_msg.as_deref().unwrap_or(""),
            200,
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmediateDeliveryReOrderRequest {
    #[serde(flatten)]
    pub order: ImmediateDeliveryAddOrderRequest,
    pub delivery_token: String,
}

impl ImmediateDeliveryReOrderRequest {
    pub fn validate(&self) -> Result<()> {
        self.order.validate()?;
        validate_immediate_delivery_required("delivery token", &self.delivery_token)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmediateDeliveryGetOrderRequest {
    pub shopid: String,
    pub shop_order_id: String,
    pub shop_no: String,
    pub delivery_sign: String,
}

impl ImmediateDeliveryGetOrderRequest {
    pub fn validate(&self) -> Result<()> {
        for (kind, value) in [
            ("shop id", self.shopid.as_str()),
            ("shop order id", self.shop_order_id.as_str()),
            ("shop number", self.shop_no.as_str()),
            ("delivery signature", self.delivery_sign.as_str()),
        ] {
            validate_immediate_delivery_required(kind, value)?;
        }
        validate_immediate_delivery_max_bytes("shop order id", &self.shop_order_id, 128)?;
        validate_immediate_delivery_signature(&self.delivery_sign)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmediateDeliveryStatusResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub resultcode: Option<i64>,
    #[serde(default)]
    pub resultmsg: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl ImmediateDeliveryStatusResponse {
    pub fn ensure_success(&self) -> Result<()> {
        ensure_immediate_delivery_response_success(
            self.errcode,
            self.errmsg.as_deref(),
            self.resultcode,
            self.resultmsg.as_deref(),
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmediateDeliveryBindAccountResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub resultcode: Option<i64>,
    #[serde(default)]
    pub resultmsg: Option<String>,
    #[serde(default)]
    pub shop_list: Vec<ImmediateDeliveryShop>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmediateDeliveryShop {
    #[serde(default)]
    pub delivery_id: Option<String>,
    #[serde(default)]
    pub delivery_name: Option<String>,
    #[serde(default)]
    pub shopid: Option<String>,
    #[serde(default)]
    pub shop_no: Option<String>,
    #[serde(default)]
    pub shop_name: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmediateDeliveryDeliveryListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub resultcode: Option<i64>,
    #[serde(default)]
    pub resultmsg: Option<String>,
    #[serde(default)]
    pub list: Vec<ImmediateDeliveryProvider>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmediateDeliveryProvider {
    #[serde(default)]
    pub delivery_id: Option<String>,
    #[serde(default)]
    pub delivery_name: Option<String>,
    #[serde(default)]
    pub can_use_cash: Option<bool>,
    #[serde(default)]
    pub can_get_quota: Option<bool>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl ImmediateDeliveryBindAccountResponse {
    pub fn ensure_success(&self) -> Result<()> {
        ensure_immediate_delivery_response_success(
            self.errcode,
            self.errmsg.as_deref(),
            self.resultcode,
            self.resultmsg.as_deref(),
        )
    }

    pub fn find_shop(&self, delivery_id: &str, shop_id: &str) -> Option<&ImmediateDeliveryShop> {
        self.shop_list.iter().find(|shop| {
            shop.delivery_id.as_deref() == Some(delivery_id)
                && shop.shopid.as_deref() == Some(shop_id)
        })
    }
}

impl ImmediateDeliveryDeliveryListResponse {
    pub fn ensure_success(&self) -> Result<()> {
        ensure_immediate_delivery_response_success(
            self.errcode,
            self.errmsg.as_deref(),
            self.resultcode,
            self.resultmsg.as_deref(),
        )
    }

    pub fn find_provider(&self, delivery_id: &str) -> Option<&ImmediateDeliveryProvider> {
        self.list
            .iter()
            .find(|provider| provider.delivery_id.as_deref() == Some(delivery_id))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmediateDeliveryCancelOrderResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub resultcode: Option<i64>,
    #[serde(default)]
    pub resultmsg: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub deduct_fee: Option<i64>,
    #[serde(default)]
    pub desc: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl ImmediateDeliveryCancelOrderResponse {
    pub fn ensure_success(&self) -> Result<()> {
        ensure_immediate_delivery_response_success(
            self.errcode,
            self.errmsg.as_deref(),
            self.resultcode,
            self.resultmsg.as_deref(),
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmediateDeliveryOrderResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub resultcode: Option<i64>,
    #[serde(default)]
    pub resultmsg: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_string")]
    pub fee: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_string")]
    pub deliverfee: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_string")]
    pub couponfee: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_string")]
    pub tips: Option<String>,
    #[serde(
        default,
        rename = "insurancfee",
        deserialize_with = "deserialize_optional_string"
    )]
    pub insurance_fee: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_string")]
    pub distance: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_string")]
    pub waybill_id: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub order_status: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub finish_code: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub pickup_code: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub dispatch_duration: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl ImmediateDeliveryOrderResponse {
    pub fn ensure_success(&self) -> Result<()> {
        ensure_immediate_delivery_response_success(
            self.errcode,
            self.errmsg.as_deref(),
            self.resultcode,
            self.resultmsg.as_deref(),
        )
    }

    pub fn order_status_kind(&self) -> Option<ImmediateDeliveryOrderStatusKind> {
        self.order_status
            .map(ImmediateDeliveryOrderStatusKind::from)
    }

    pub fn parsed_fee(&self) -> Result<Option<f64>> {
        parse_immediate_delivery_decimal("actual fee", self.fee.as_deref())
    }

    pub fn reconcile_fee(&self) -> Result<()> {
        let Some(actual) = self.parsed_fee()? else {
            return Ok(());
        };
        let delivery =
            parse_immediate_delivery_decimal("delivery fee", self.deliverfee.as_deref())?
                .unwrap_or(0.0);
        let coupon = parse_immediate_delivery_decimal("coupon fee", self.couponfee.as_deref())?
            .unwrap_or(0.0);
        if (actual - (delivery - coupon)).abs() > 0.005 {
            return Err(WechatError::Config(
                "mini-program immediate-delivery actual fee does not equal delivery fee minus coupon"
                    .to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmediateDeliveryGetOrderResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub resultcode: Option<i64>,
    #[serde(default)]
    pub resultmsg: Option<String>,
    #[serde(default)]
    pub delivery_id: Option<String>,
    #[serde(default)]
    pub delivery_name: Option<String>,
    #[serde(default)]
    pub shopid: Option<String>,
    #[serde(default)]
    pub shop_order_id: Option<String>,
    #[serde(default)]
    pub shop_no: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub order_status: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_optional_string")]
    pub waybill_id: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub pickup_code: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub finish_code: Option<i64>,
    #[serde(default)]
    pub rider_name: Option<String>,
    #[serde(default)]
    pub rider_phone: Option<String>,
    #[serde(default)]
    pub rider_lng: Option<f64>,
    #[serde(default)]
    pub rider_lat: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub reach_time: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImmediateDeliveryOrderStatusKind {
    WaitingForRiderAssignment,
    RiderAssigned,
    MerchantCancelledBeforePickup,
    RiderArrivedAtShop,
    PickedUp,
    PickupFailedMerchantCancelled,
    PickupFailedRiderCancelled,
    PickupFailedMerchantReasonCancelled,
    Delivering,
    Delivered,
    MerchantCancelledReturning,
    ReceiverUnreachableReturning,
    ReceiverRejectedReturning,
    ReturnedToMerchant,
    CapacitySystemCancelled,
    ForceMajeureCancelled,
    Other(i64),
}

impl From<i64> for ImmediateDeliveryOrderStatusKind {
    fn from(value: i64) -> Self {
        match value {
            101 => Self::WaitingForRiderAssignment,
            102 => Self::RiderAssigned,
            103 => Self::MerchantCancelledBeforePickup,
            201 => Self::RiderArrivedAtShop,
            202 => Self::PickedUp,
            203 => Self::PickupFailedMerchantCancelled,
            204 => Self::PickupFailedRiderCancelled,
            205 => Self::PickupFailedMerchantReasonCancelled,
            301 => Self::Delivering,
            302 => Self::Delivered,
            303 => Self::MerchantCancelledReturning,
            304 => Self::ReceiverUnreachableReturning,
            305 => Self::ReceiverRejectedReturning,
            401 => Self::ReturnedToMerchant,
            501 => Self::CapacitySystemCancelled,
            502 => Self::ForceMajeureCancelled,
            other => Self::Other(other),
        }
    }
}

impl ImmediateDeliveryOrderStatusKind {
    pub fn is_success_terminal(self) -> bool {
        matches!(self, Self::Delivered)
    }

    pub fn is_failure_terminal(self) -> bool {
        matches!(
            self,
            Self::MerchantCancelledBeforePickup
                | Self::PickupFailedMerchantCancelled
                | Self::PickupFailedRiderCancelled
                | Self::PickupFailedMerchantReasonCancelled
                | Self::ReturnedToMerchant
                | Self::CapacitySystemCancelled
                | Self::ForceMajeureCancelled
        )
    }
}

impl ImmediateDeliveryGetOrderResponse {
    pub fn order_status_kind(&self) -> Option<ImmediateDeliveryOrderStatusKind> {
        self.order_status
            .map(ImmediateDeliveryOrderStatusKind::from)
    }

    pub fn ensure_success(&self) -> Result<()> {
        ensure_immediate_delivery_response_success(
            self.errcode,
            self.errmsg.as_deref(),
            self.resultcode,
            self.resultmsg.as_deref(),
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmediateDeliveryPreAddOrderResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub resultcode: Option<i64>,
    #[serde(default)]
    pub resultmsg: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
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
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub dispatch_duration: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub delivery_token: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmediateDeliveryPreCancelOrderResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub resultcode: Option<i64>,
    #[serde(default)]
    pub resultmsg: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub deduct_fee: Option<i64>,
    #[serde(default)]
    pub desc: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl ImmediateDeliveryPreAddOrderResponse {
    pub fn ensure_success(&self) -> Result<()> {
        ensure_immediate_delivery_response_success(
            self.errcode,
            self.errmsg.as_deref(),
            self.resultcode,
            self.resultmsg.as_deref(),
        )
    }
}

impl ImmediateDeliveryPreCancelOrderResponse {
    pub fn ensure_success(&self) -> Result<()> {
        ensure_immediate_delivery_response_success(
            self.errcode,
            self.errmsg.as_deref(),
            self.resultcode,
            self.resultmsg.as_deref(),
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmediateDeliveryReOrderResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub resultcode: Option<i64>,
    #[serde(default)]
    pub resultmsg: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
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
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub waybill_id: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub order_status: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub finish_code: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub pickup_code: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub dispatch_duration: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl ImmediateDeliveryReOrderResponse {
    pub fn order_status_kind(&self) -> Option<ImmediateDeliveryOrderStatusKind> {
        self.order_status
            .map(ImmediateDeliveryOrderStatusKind::from)
    }

    pub fn ensure_success(&self) -> Result<()> {
        ensure_immediate_delivery_response_success(
            self.errcode,
            self.errmsg.as_deref(),
            self.resultcode,
            self.resultmsg.as_deref(),
        )
    }
}

fn validate_immediate_delivery_required(kind: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(WechatError::Config(format!(
            "mini-program immediate-delivery {kind} must not be blank"
        )));
    }
    Ok(())
}

fn validate_immediate_delivery_max_bytes(kind: &str, value: &str, maximum: usize) -> Result<()> {
    if value.len() > maximum {
        return Err(WechatError::Config(format!(
            "mini-program immediate-delivery {kind} must not exceed {maximum} bytes"
        )));
    }
    Ok(())
}

fn validate_immediate_delivery_max_chars(kind: &str, value: &str, maximum: usize) -> Result<()> {
    if value.chars().count() > maximum {
        return Err(WechatError::Config(format!(
            "mini-program immediate-delivery {kind} must not exceed {maximum} characters"
        )));
    }
    Ok(())
}

fn validate_immediate_delivery_signature(signature: &str) -> Result<()> {
    if signature.len() != 40 || !signature.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(WechatError::Config(
            "mini-program immediate-delivery signature must be a 40-character SHA-1 hex digest"
                .to_string(),
        ));
    }
    Ok(())
}

fn validate_immediate_delivery_number(
    kind: &str,
    value: f64,
    minimum: f64,
    maximum: f64,
    inclusive_minimum: bool,
) -> Result<()> {
    let lower_bound_valid = if inclusive_minimum {
        value >= minimum
    } else {
        value > minimum
    };
    if !value.is_finite() || !lower_bound_valid || value > maximum {
        let lower = if inclusive_minimum { "[" } else { "(" };
        return Err(WechatError::Config(format!(
            "mini-program immediate-delivery {kind} must be in {lower}{minimum}, {maximum}]"
        )));
    }
    Ok(())
}

fn validate_immediate_delivery_money(
    kind: &str,
    value: f64,
    minimum: f64,
    maximum: f64,
) -> Result<()> {
    validate_immediate_delivery_number(kind, value, minimum, maximum, true)?;
    let cents = value * 100.0;
    if !cents.is_finite() || (cents - cents.round()).abs() > 0.000_001 {
        return Err(WechatError::Config(format!(
            "mini-program immediate-delivery {kind} must have at most two decimal places"
        )));
    }
    Ok(())
}

fn validate_immediate_delivery_https_url(kind: &str, value: &str) -> Result<()> {
    let parsed = url::Url::parse(value).map_err(|err| {
        WechatError::Config(format!(
            "mini-program immediate-delivery {kind} must be an absolute URL: {err}"
        ))
    })?;
    if parsed.scheme() != "https" || parsed.host_str().is_none() {
        return Err(WechatError::Config(format!(
            "mini-program immediate-delivery {kind} must use HTTPS"
        )));
    }
    Ok(())
}

fn ensure_immediate_delivery_response_success(
    errcode: Option<i64>,
    errmsg: Option<&str>,
    resultcode: Option<i64>,
    resultmsg: Option<&str>,
) -> Result<()> {
    if let Some(code) = errcode.filter(|code| *code != 0) {
        return Err(WechatError::Api {
            code,
            message: errmsg.unwrap_or("immediate-delivery API error").to_string(),
        });
    }
    if let Some(code) = resultcode.filter(|code| *code != 0) {
        return Err(WechatError::Api {
            code,
            message: resultmsg
                .unwrap_or("immediate-delivery provider error")
                .to_string(),
        });
    }
    Ok(())
}

fn parse_immediate_delivery_decimal(kind: &str, value: Option<&str>) -> Result<Option<f64>> {
    let Some(value) = value else {
        return Ok(None);
    };
    let parsed = value.parse::<f64>().map_err(|err| {
        WechatError::Config(format!(
            "mini-program immediate-delivery {kind} is not a decimal: {err}"
        ))
    })?;
    validate_immediate_delivery_money(kind, parsed, 0.0, f64::MAX)?;
    Ok(Some(parsed))
}

fn validate_express_required(kind: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(WechatError::Config(format!(
            "mini-program express {kind} must not be blank"
        )));
    }
    Ok(())
}

fn validate_express_optional(kind: &str, value: Option<&str>) -> Result<()> {
    if let Some(value) = value {
        validate_express_required(kind, value)?;
    }
    Ok(())
}

fn ensure_express_response_success(errcode: Option<i64>, errmsg: Option<&str>) -> Result<()> {
    if let Some(code) = errcode.filter(|code| *code != 0) {
        return Err(WechatError::Api {
            code,
            message: errmsg
                .unwrap_or("mini-program express operation failed")
                .to_string(),
        });
    }
    Ok(())
}

fn ensure_express_delivery_success(
    result_code: Option<i64>,
    result_message: Option<&str>,
) -> Result<()> {
    if let Some(code) = result_code.filter(|code| *code != 0) {
        return Err(WechatError::Api {
            code,
            message: result_message
                .unwrap_or("express carrier operation failed")
                .to_string(),
        });
    }
    Ok(())
}

fn validate_express_order_identity(
    order_id: &str,
    openid: &str,
    delivery_id: &str,
    waybill_id: &str,
) -> Result<()> {
    for (kind, value) in [
        ("order id", order_id),
        ("openid", openid),
        ("delivery id", delivery_id),
        ("waybill id", waybill_id),
    ] {
        validate_express_required(kind, value)?;
    }
    Ok(())
}

fn validate_express_batch_orders(orders: &[ExpressBatchGetOrderItem]) -> Result<()> {
    if orders.is_empty() || orders.len() > 100 {
        return Err(WechatError::Config(
            "mini-program express batch order list must contain between 1 and 100 entries"
                .to_string(),
        ));
    }
    let mut order_ids = std::collections::HashSet::with_capacity(orders.len());
    for order in orders {
        order.validate()?;
        if !order_ids.insert(order.order_id.trim()) {
            return Err(WechatError::Config(
                "mini-program express batch order ids must be unique".to_string(),
            ));
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressAddOrderRequest {
    pub order_id: String,
    pub openid: String,
    pub delivery_id: String,
    pub biz_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_remark: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tagid: Option<i64>,
    pub add_source: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wx_appid: Option<String>,
    pub sender: ExpressAddress,
    pub receiver: ExpressAddress,
    pub shop: ExpressShop,
    pub cargo: ExpressCargo,
    pub insured: ExpressInsured,
    pub service: ExpressService,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expect_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub take_mode: Option<i64>,
}

impl ExpressAddOrderRequest {
    pub fn validate(&self) -> Result<()> {
        for (kind, value) in [
            ("order id", self.order_id.as_str()),
            ("delivery id", self.delivery_id.as_str()),
            ("business id", self.biz_id.as_str()),
        ] {
            validate_express_required(kind, value)?;
        }
        if !matches!(self.add_source, 0 | 2) {
            return Err(WechatError::Config(
                "mini-program express add source must be 0 or 2".to_string(),
            ));
        }
        if self.add_source == 0 {
            validate_express_required("openid", &self.openid)?;
        } else {
            if !self.openid.is_empty() {
                validate_express_required("openid", &self.openid)?;
            }
            let wx_appid = self.wx_appid.as_deref().ok_or_else(|| {
                WechatError::Config(
                    "mini-program express App/H5 order requires wx_appid".to_string(),
                )
            })?;
            validate_express_required("App/H5 appid", wx_appid)?;
        }
        validate_express_optional("custom remark", self.custom_remark.as_deref())?;
        if self.tagid.is_some_and(|tagid| tagid <= 0) {
            return Err(WechatError::Config(
                "mini-program express tag id must be positive".to_string(),
            ));
        }
        self.sender.validate("sender")?;
        self.receiver.validate("receiver")?;
        self.shop.validate()?;
        self.cargo.validate()?;
        self.insured.validate()?;
        self.service.validate()?;
        if self.expect_time.is_some_and(|time| time < 0) {
            return Err(WechatError::Config(
                "mini-program express expected delivery time cannot be negative".to_string(),
            ));
        }
        if self
            .take_mode
            .is_some_and(|take_mode| !matches!(take_mode, 0 | 1))
        {
            return Err(WechatError::Config(
                "mini-program express take mode must be 0 or 1".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressAddress {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tel: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub province: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

impl ExpressAddress {
    fn validate(&self, kind: &str) -> Result<()> {
        if self.tel.is_none() && self.mobile.is_none() {
            return Err(WechatError::Config(format!(
                "mini-program express {kind} requires telephone or mobile"
            )));
        }
        validate_express_optional(&format!("{kind} telephone"), self.tel.as_deref())?;
        validate_express_optional(&format!("{kind} mobile"), self.mobile.as_deref())?;
        validate_express_optional(&format!("{kind} company"), self.company.as_deref())?;
        validate_express_optional(&format!("{kind} post code"), self.post_code.as_deref())?;
        for (field, value) in [
            ("name", self.name.as_deref()),
            ("country", self.country.as_deref()),
            ("province", self.province.as_deref()),
            ("city", self.city.as_deref()),
            ("area", self.area.as_deref()),
            ("address", self.address.as_deref()),
        ] {
            validate_express_optional(&format!("{kind} {field}"), value)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressShop {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wxa_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub img_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub goods_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub goods_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub detail_list: Vec<ExpressShopDetail>,
}

impl ExpressShop {
    fn validate(&self) -> Result<()> {
        validate_express_optional("shop mini-program path", self.wxa_path.as_deref())?;
        validate_express_optional("shop image URL", self.img_url.as_deref())?;
        validate_express_optional("shop goods name", self.goods_name.as_deref())?;
        if self.goods_count.is_some_and(|count| count <= 0) {
            return Err(WechatError::Config(
                "mini-program express shop goods count must be positive".to_string(),
            ));
        }
        if self.detail_list.is_empty()
            && (self.img_url.is_none() || self.goods_name.is_none() || self.goods_count.is_none())
        {
            return Err(WechatError::Config(
                "mini-program express shop requires image, name, and count when detail_list is empty"
                    .to_string(),
            ));
        }
        for detail in &self.detail_list {
            detail.validate()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressShopDetail {
    pub goods_name: String,
    pub goods_img_url: String,
    pub goods_desc: String,
}

impl ExpressShopDetail {
    fn validate(&self) -> Result<()> {
        validate_express_required("shop detail goods name", &self.goods_name)?;
        validate_express_required("shop detail goods image URL", &self.goods_img_url)?;
        validate_express_required("shop detail goods description", &self.goods_desc)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressCargo {
    pub count: i64,
    pub weight: f64,
    pub space_x: f64,
    pub space_y: f64,
    pub space_z: f64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub detail_list: Vec<ExpressCargoDetail>,
}

impl ExpressCargo {
    fn validate(&self) -> Result<()> {
        if self.count <= 0 {
            return Err(WechatError::Config(
                "mini-program express cargo count must be positive".to_string(),
            ));
        }
        for value in [self.weight, self.space_x, self.space_y, self.space_z] {
            if !value.is_finite() || value <= 0.0 {
                return Err(WechatError::Config(
                    "mini-program express cargo measurements must be finite and positive"
                        .to_string(),
                ));
            }
        }
        let mut names = std::collections::HashSet::with_capacity(self.detail_list.len());
        let mut detail_count = 0_i64;
        for detail in &self.detail_list {
            detail.validate()?;
            if !names.insert(detail.name.trim()) {
                return Err(WechatError::Config(
                    "mini-program express cargo detail names must be unique".to_string(),
                ));
            }
            detail_count = detail_count.checked_add(detail.count).ok_or_else(|| {
                WechatError::Config(
                    "mini-program express cargo detail count overflowed".to_string(),
                )
            })?;
        }
        if !self.detail_list.is_empty() && detail_count != self.count {
            return Err(WechatError::Config(
                "mini-program express cargo detail count must equal cargo count".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressCargoDetail {
    pub name: String,
    pub count: i64,
}

impl ExpressCargoDetail {
    fn validate(&self) -> Result<()> {
        validate_express_required("cargo detail name", &self.name)?;
        if self.count <= 0 {
            return Err(WechatError::Config(
                "mini-program express cargo detail count must be positive".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressInsured {
    pub use_insured: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insured_value: Option<i64>,
}

impl ExpressInsured {
    fn validate(&self) -> Result<()> {
        if !matches!(self.use_insured, 0 | 1) {
            return Err(WechatError::Config(
                "mini-program express insured flag must be 0 or 1".to_string(),
            ));
        }
        match (self.use_insured, self.insured_value) {
            (1, Some(value)) if value > 0 => Ok(()),
            (1, _) => Err(WechatError::Config(
                "mini-program express insured shipment requires a positive insured value"
                    .to_string(),
            )),
            (0, Some(_)) => Err(WechatError::Config(
                "mini-program express uninsured shipment must not include insured value"
                    .to_string(),
            )),
            (0, None) => Ok(()),
            _ => unreachable!("insured flag was validated"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressService {
    pub service_type: i64,
    pub service_name: String,
}

impl ExpressService {
    fn validate(&self) -> Result<()> {
        if self.service_type < 0 {
            return Err(WechatError::Config(
                "mini-program express service type cannot be negative".to_string(),
            ));
        }
        validate_express_required("service name", &self.service_name)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressPreviewTemplateRequest {
    pub waybill_id: String,
    pub waybill_template: String,
    pub waybill_data: String,
    pub custom: ExpressAddOrderRequest,
}

impl ExpressPreviewTemplateRequest {
    pub fn validate(&self) -> Result<()> {
        validate_express_required("preview waybill id", &self.waybill_id)?;
        validate_express_required("waybill template", &self.waybill_template)?;
        validate_express_required("waybill data", &self.waybill_data)?;
        self.custom.validate()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressUpdateBusinessRequest {
    pub shop_app_id: String,
    pub biz_id: String,
    pub result_code: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_msg: Option<String>,
}

impl ExpressUpdateBusinessRequest {
    pub fn validate(&self) -> Result<()> {
        validate_express_required("shop appid", &self.shop_app_id)?;
        validate_express_required("business id", &self.biz_id)?;
        if self.result_code != 0 && self.result_msg.is_none() {
            return Err(WechatError::Config(
                "mini-program express failed business audit requires result message".to_string(),
            ));
        }
        validate_express_optional("business audit result message", self.result_msg.as_deref())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressUpdatePathRequest {
    pub token: String,
    pub waybill_id: String,
    pub action_time: i64,
    pub action_type: i64,
    pub action_msg: String,
}

impl ExpressUpdatePathRequest {
    pub fn validate(&self) -> Result<()> {
        validate_express_required("path token", &self.token)?;
        validate_express_required("waybill id", &self.waybill_id)?;
        if self.action_time <= 0 || self.action_type <= 0 {
            return Err(WechatError::Config(
                "mini-program express path action time and type must be positive".to_string(),
            ));
        }
        validate_express_required("path action message", &self.action_msg)
    }
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

    pub fn validate(&self) -> Result<()> {
        match self.payload.as_object() {
            Some(object) if !object.is_empty() => Ok(()),
            _ => Err(WechatError::Config(
                "mini-program express compatibility payload must be a nonempty JSON object"
                    .to_string(),
            )),
        }
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

impl ExpressBindAccountRequest {
    pub fn validate(&self) -> Result<()> {
        if !matches!(self.action_type.as_str(), "bind" | "unbind") {
            return Err(WechatError::Config(
                "mini-program express account action must be bind or unbind".to_string(),
            ));
        }
        validate_express_required("business id", &self.biz_id)?;
        validate_express_required("delivery id", &self.delivery_id)?;
        if self.action_type == "bind" {
            validate_express_required("account password", &self.password)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressOrderRequest {
    pub order_id: String,
    pub openid: String,
    pub delivery_id: String,
    pub waybill_id: String,
}

impl ExpressOrderRequest {
    pub fn validate(&self) -> Result<()> {
        validate_express_order_identity(
            &self.order_id,
            &self.openid,
            &self.delivery_id,
            &self.waybill_id,
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressGetOrderRequest {
    pub order_id: String,
    pub openid: String,
    pub delivery_id: String,
    pub waybill_id: String,
    pub print_type: i64,
}

impl ExpressGetOrderRequest {
    pub fn validate(&self) -> Result<()> {
        validate_express_order_identity(
            &self.order_id,
            &self.openid,
            &self.delivery_id,
            &self.waybill_id,
        )?;
        if !matches!(self.print_type, 0 | 1) {
            return Err(WechatError::Config(
                "mini-program express print type must be 0 or 1".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressBatchGetOrderItem {
    pub order_id: String,
    pub openid: String,
    pub delivery_id: String,
    pub waybill_id: String,
}

impl ExpressBatchGetOrderItem {
    pub fn validate(&self) -> Result<()> {
        validate_express_order_identity(
            &self.order_id,
            &self.openid,
            &self.delivery_id,
            &self.waybill_id,
        )
    }
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

impl ExpressTestUpdateOrderRequest {
    pub fn validate(&self) -> Result<()> {
        for (kind, value) in [
            ("business id", self.biz_id.as_str()),
            ("order id", self.order_id.as_str()),
            ("delivery id", self.delivery_id.as_str()),
            ("waybill id", self.waybill_id.as_str()),
            ("action message", self.action_msg.as_str()),
        ] {
            validate_express_required(kind, value)?;
        }
        if self.action_time <= 0 || self.action_type <= 0 {
            return Err(WechatError::Config(
                "mini-program express test action time and type must be positive".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressUpdatePrinterRequest {
    pub openid: String,
    pub update_type: String,
    pub tagid_list: String,
}

impl ExpressUpdatePrinterRequest {
    pub fn validate(&self) -> Result<()> {
        validate_express_required("printer openid", &self.openid)?;
        if !matches!(self.update_type.as_str(), "bind" | "unbind") {
            return Err(WechatError::Config(
                "mini-program express printer update type must be bind or unbind".to_string(),
            ));
        }
        validate_express_required("printer tag id list", &self.tagid_list)
    }
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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl ExpressAddOrderResponse {
    pub fn ensure_created(&self) -> Result<(&str, &str)> {
        ensure_express_response_success(self.errcode, self.errmsg.as_deref())?;
        ensure_express_delivery_success(
            self.delivery_resultcode,
            self.delivery_resultmsg.as_deref(),
        )?;
        let order_id = self.order_id.as_deref().ok_or_else(|| {
            WechatError::Config(
                "mini-program express add-order response is missing order id".to_string(),
            )
        })?;
        let waybill_id = self.waybill_id.as_deref().ok_or_else(|| {
            WechatError::Config(
                "mini-program express add-order response is missing waybill id".to_string(),
            )
        })?;
        validate_express_required("response order id", order_id)?;
        validate_express_required("response waybill id", waybill_id)?;
        validate_express_waybill_data(&self.waybill_data)?;
        Ok((order_id, waybill_id))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressWaybillData {
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

fn validate_express_waybill_data(data: &[ExpressWaybillData]) -> Result<()> {
    let mut keys = std::collections::HashSet::with_capacity(data.len());
    for item in data {
        let key = item.key.as_deref().ok_or_else(|| {
            WechatError::Config("mini-program express waybill data item is missing key".to_string())
        })?;
        let value = item.value.as_deref().ok_or_else(|| {
            WechatError::Config(
                "mini-program express waybill data item is missing value".to_string(),
            )
        })?;
        validate_express_required("waybill data key", key)?;
        validate_express_required("waybill data value", value)?;
        if !keys.insert(key.trim()) {
            return Err(WechatError::Config(
                "mini-program express waybill data keys must be unique".to_string(),
            ));
        }
    }
    Ok(())
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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl ExpressOrderSummary {
    pub fn validate(&self) -> Result<()> {
        for (kind, value) in [
            ("summary order id", self.order_id.as_deref()),
            ("summary waybill id", self.waybill_id.as_deref()),
            ("summary delivery id", self.delivery_id.as_deref()),
        ] {
            let value = value.ok_or_else(|| {
                WechatError::Config(format!("mini-program express {kind} is missing"))
            })?;
            validate_express_required(kind, value)?;
        }
        if self.order_status.is_some_and(|status| status < 0) {
            return Err(WechatError::Config(
                "mini-program express order status cannot be negative".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressBatchOrderListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub order_list: Vec<ExpressOrderSummary>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl ExpressBatchOrderListResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_express_response_success(self.errcode, self.errmsg.as_deref())?;
        let mut order_ids = std::collections::HashSet::with_capacity(self.order_list.len());
        let mut waybill_ids = std::collections::HashSet::with_capacity(self.order_list.len());
        for order in &self.order_list {
            order.validate()?;
            let order_id = order
                .order_id
                .as_deref()
                .expect("validated order id exists");
            let waybill_id = order
                .waybill_id
                .as_deref()
                .expect("validated waybill id exists");
            if !order_ids.insert(order_id.trim()) || !waybill_ids.insert(waybill_id.trim()) {
                return Err(WechatError::Config(
                    "mini-program express batch response contains duplicate order or waybill ids"
                        .to_string(),
                ));
            }
        }
        Ok(())
    }

    pub fn find_order(&self, order_id: &str) -> Option<&ExpressOrderSummary> {
        self.order_list
            .iter()
            .find(|order| order.order_id.as_deref() == Some(order_id))
    }
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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl ExpressCancelOrderResponse {
    pub fn ensure_cancelled(&self) -> Result<()> {
        ensure_express_response_success(self.errcode, self.errmsg.as_deref())?;
        ensure_express_delivery_success(
            self.delivery_resultcode,
            self.delivery_resultmsg.as_deref(),
        )
    }
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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl ExpressAccountListResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_express_response_success(self.errcode, self.errmsg.as_deref())?;
        validate_express_decoded_count("account", self.count, self.list.len())?;
        let mut identities = std::collections::HashSet::with_capacity(self.list.len());
        for account in &self.list {
            let biz_id = account.biz_id.as_deref().ok_or_else(|| {
                WechatError::Config(
                    "mini-program express account is missing business id".to_string(),
                )
            })?;
            let delivery_id = account.delivery_id.as_deref().ok_or_else(|| {
                WechatError::Config(
                    "mini-program express account is missing delivery id".to_string(),
                )
            })?;
            validate_express_required("account business id", biz_id)?;
            validate_express_required("account delivery id", delivery_id)?;
            if !identities.insert((biz_id.trim(), delivery_id.trim())) {
                return Err(WechatError::Config(
                    "mini-program express account list contains duplicate identities".to_string(),
                ));
            }
        }
        Ok(())
    }

    pub fn find(&self, biz_id: &str, delivery_id: &str) -> Option<&ExpressAccount> {
        self.list.iter().find(|account| {
            account.biz_id.as_deref() == Some(biz_id)
                && account.delivery_id.as_deref() == Some(delivery_id)
        })
    }
}

fn validate_express_decoded_count(kind: &str, count: Option<i64>, actual: usize) -> Result<()> {
    if count.is_some_and(|count| count < 0) {
        return Err(WechatError::Config(format!(
            "mini-program express {kind} count cannot be negative"
        )));
    }
    let actual = i64::try_from(actual).map_err(|_| {
        WechatError::Config(format!(
            "mini-program express decoded {kind} count exceeds i64"
        ))
    })?;
    if count.is_some_and(|count| count != actual) {
        return Err(WechatError::Config(format!(
            "mini-program express {kind} count does not match decoded list"
        )));
    }
    Ok(())
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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl ExpressDeliveryListResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_express_response_success(self.errcode, self.errmsg.as_deref())?;
        validate_express_decoded_count("delivery", self.count, self.data.len())?;
        let mut delivery_ids = std::collections::HashSet::with_capacity(self.data.len());
        for delivery in &self.data {
            let delivery_id = delivery.delivery_id.as_deref().ok_or_else(|| {
                WechatError::Config(
                    "mini-program express delivery is missing delivery id".to_string(),
                )
            })?;
            validate_express_required("delivery id", delivery_id)?;
            if [delivery.can_use_cash, delivery.can_get_quota]
                .into_iter()
                .flatten()
                .any(|flag| !matches!(flag, 0 | 1))
            {
                return Err(WechatError::Config(
                    "mini-program express delivery capability flags must be 0 or 1".to_string(),
                ));
            }
            if !delivery_ids.insert(delivery_id.trim()) {
                return Err(WechatError::Config(
                    "mini-program express delivery list contains duplicate ids".to_string(),
                ));
            }
        }
        Ok(())
    }

    pub fn supporting_quota(&self) -> Vec<&ExpressDelivery> {
        self.data
            .iter()
            .filter(|delivery| delivery.can_get_quota == Some(1))
            .collect()
    }
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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl ExpressGetOrderResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_express_response_success(self.errcode, self.errmsg.as_deref())?;
        for (kind, value) in [
            ("response delivery id", self.delivery_id.as_deref()),
            ("response waybill id", self.waybill_id.as_deref()),
            ("response order id", self.order_id.as_deref()),
        ] {
            let value = value.ok_or_else(|| {
                WechatError::Config(format!("mini-program express {kind} is missing"))
            })?;
            validate_express_required(kind, value)?;
        }
        if self.order_status.is_some_and(|status| status < 0) {
            return Err(WechatError::Config(
                "mini-program express response order status cannot be negative".to_string(),
            ));
        }
        validate_express_waybill_data(&self.waybill_data)
    }

    pub fn matches(&self, order_id: &str, waybill_id: &str) -> bool {
        self.order_id.as_deref() == Some(order_id) && self.waybill_id.as_deref() == Some(waybill_id)
    }
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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl ExpressGetPathResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_express_response_success(self.errcode, self.errmsg.as_deref())?;
        for (kind, value) in [
            ("path openid", self.openid.as_deref()),
            ("path delivery id", self.delivery_id.as_deref()),
            ("path waybill id", self.waybill_id.as_deref()),
        ] {
            let value = value.ok_or_else(|| {
                WechatError::Config(format!("mini-program express {kind} is missing"))
            })?;
            validate_express_required(kind, value)?;
        }
        validate_express_decoded_count("path item", self.path_item_num, self.path_item_list.len())?;
        let mut previous_time = None;
        for item in &self.path_item_list {
            let action_time = item.action_time.ok_or_else(|| {
                WechatError::Config(
                    "mini-program express path item is missing action time".to_string(),
                )
            })?;
            if action_time <= 0 || item.action_type.is_some_and(|action_type| action_type <= 0) {
                return Err(WechatError::Config(
                    "mini-program express path action time and type must be positive".to_string(),
                ));
            }
            let action_msg = item.action_msg.as_deref().ok_or_else(|| {
                WechatError::Config(
                    "mini-program express path item is missing action message".to_string(),
                )
            })?;
            validate_express_required("path action message", action_msg)?;
            if previous_time.is_some_and(|previous| action_time < previous) {
                return Err(WechatError::Config(
                    "mini-program express path items must be ordered by action time".to_string(),
                ));
            }
            previous_time = Some(action_time);
        }
        Ok(())
    }

    pub fn latest(&self) -> Option<&ExpressPathItem> {
        self.path_item_list.last()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressPathItem {
    #[serde(default)]
    pub action_time: Option<i64>,
    #[serde(default)]
    pub action_type: Option<i64>,
    #[serde(default)]
    pub action_msg: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressGetPrinterResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub count: Option<i64>,
    #[serde(default)]
    pub openid: Vec<String>,
    #[serde(default)]
    pub tagid_list: Vec<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl ExpressGetPrinterResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_express_response_success(self.errcode, self.errmsg.as_deref())?;
        validate_express_decoded_count("printer", self.count, self.openid.len())?;
        let mut openids = std::collections::HashSet::with_capacity(self.openid.len());
        for openid in &self.openid {
            validate_express_required("printer openid", openid)?;
            if !openids.insert(openid.trim()) {
                return Err(WechatError::Config(
                    "mini-program express printer list contains duplicate openids".to_string(),
                ));
            }
        }
        let mut tags = std::collections::HashSet::with_capacity(self.tagid_list.len());
        for tag in &self.tagid_list {
            validate_express_required("printer tag id", tag)?;
            if !tags.insert(tag.trim()) {
                return Err(WechatError::Config(
                    "mini-program express printer tag ids must be unique".to_string(),
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressGetQuotaResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub quota_num: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl ExpressGetQuotaResponse {
    pub fn available_quota(&self) -> Result<i64> {
        ensure_express_response_success(self.errcode, self.errmsg.as_deref())?;
        let quota = self.quota_num.ok_or_else(|| {
            WechatError::Config(
                "mini-program express quota response is missing quota number".to_string(),
            )
        })?;
        if quota < 0 {
            return Err(WechatError::Config(
                "mini-program express quota cannot be negative".to_string(),
            ));
        }
        Ok(quota)
    }

    pub fn has_quota(&self) -> Result<bool> {
        Ok(self.available_quota()? > 0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressGetContactResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_string")]
    pub waybill_id: Option<String>,
    #[serde(default)]
    pub sender: Vec<ExpressContact>,
    #[serde(default)]
    pub receiver: Vec<ExpressContact>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl ExpressGetContactResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_express_response_success(self.errcode, self.errmsg.as_deref())?;
        let waybill_id = self.waybill_id.as_deref().ok_or_else(|| {
            WechatError::Config(
                "mini-program express contact response is missing waybill id".to_string(),
            )
        })?;
        validate_express_required("contact waybill id", waybill_id)?;
        if self.sender.is_empty() || self.receiver.is_empty() {
            return Err(WechatError::Config(
                "mini-program express contact response requires sender and receiver".to_string(),
            ));
        }
        for (kind, contacts) in [("sender", &self.sender), ("receiver", &self.receiver)] {
            for contact in contacts {
                contact.validate(kind)?;
            }
        }
        Ok(())
    }
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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl ExpressContact {
    pub fn validate(&self, kind: &str) -> Result<()> {
        let name = self.name.as_deref().ok_or_else(|| {
            WechatError::Config(format!(
                "mini-program express {kind} contact is missing name"
            ))
        })?;
        validate_express_required(&format!("{kind} contact name"), name)?;
        if self.tel.is_none() && self.mobile.is_none() {
            return Err(WechatError::Config(format!(
                "mini-program express {kind} contact requires telephone or mobile"
            )));
        }
        validate_express_optional(&format!("{kind} contact telephone"), self.tel.as_deref())?;
        validate_express_optional(&format!("{kind} contact mobile"), self.mobile.as_deref())?;
        Ok(())
    }
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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl ExpressPreviewTemplateResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_express_response_success(self.errcode, self.errmsg.as_deref())?;
        let waybill_id = self.waybill_id.as_deref().ok_or_else(|| {
            WechatError::Config(
                "mini-program express preview response is missing waybill id".to_string(),
            )
        })?;
        let rendered = self.rendered_waybill_template.as_deref().ok_or_else(|| {
            WechatError::Config(
                "mini-program express preview response is missing rendered template".to_string(),
            )
        })?;
        validate_express_required("preview waybill id", waybill_id)?;
        validate_express_required("rendered waybill template", rendered)
    }
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
    pub order: Option<WxaSecOrder>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WxaSecOrder {
    #[serde(default)]
    pub transaction_id: Option<String>,
    #[serde(default)]
    pub merchant_id: Option<String>,
    #[serde(default)]
    pub sub_merchant_id: Option<String>,
    #[serde(default)]
    pub merchant_trade_no: Option<String>,
    #[serde(default)]
    pub order_state: Option<i64>,
    #[serde(default)]
    pub order_state_desc: Option<String>,
    #[serde(default)]
    pub openid: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub pay_time: Option<i64>,
    #[serde(default)]
    pub amount: Option<WxaSecOrderAmount>,
    #[serde(default)]
    pub shipping: Option<WxaSecOrderShipping>,
    #[serde(default)]
    pub receive_time: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WxaSecOrderStateKind {
    PendingShipment,
    Shipped,
    ConfirmedReceived,
    Completed,
    Refunded,
    Other(i64),
}

impl From<i64> for WxaSecOrderStateKind {
    fn from(value: i64) -> Self {
        match value {
            1 => Self::PendingShipment,
            2 => Self::Shipped,
            3 => Self::ConfirmedReceived,
            4 => Self::Completed,
            5 => Self::Refunded,
            other => Self::Other(other),
        }
    }
}

impl WxaSecOrder {
    pub fn order_state_kind(&self) -> Option<WxaSecOrderStateKind> {
        self.order_state.map(WxaSecOrderStateKind::from)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WxaSecOrderAmount {
    #[serde(default)]
    pub total: Option<i64>,
    #[serde(default)]
    pub payer_total: Option<i64>,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub payer_currency: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WxaSecOrderShipping {
    #[serde(default)]
    pub logistics_type: Option<i64>,
    #[serde(default)]
    pub delivery_mode: Option<i64>,
    #[serde(default)]
    pub is_all_delivered: Option<bool>,
    #[serde(default)]
    pub shipping_list: Vec<WxaSecOrderShippingItem>,
    #[serde(default)]
    pub upload_time: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WxaSecOrderShippingItem {
    #[serde(default)]
    pub tracking_no: Option<String>,
    #[serde(default)]
    pub express_company: Option<String>,
    #[serde(default)]
    pub item_desc: Option<String>,
    #[serde(default)]
    pub contact: Option<WxaSecShippingContact>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
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
    pub order_list: Vec<WxaSecOrder>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WxaSecTradeManagedResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub is_trade_managed: Option<bool>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WxaSecTradeManagementConfirmationResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub completed: Option<bool>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
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

fn validate_live_required(kind: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(WechatError::Config(format!(
            "mini-program live {kind} must not be blank"
        )));
    }
    Ok(())
}

fn validate_live_optional_non_empty(kind: &str, value: Option<&str>) -> Result<()> {
    if let Some(value) = value {
        validate_live_required(kind, value)?;
    }
    Ok(())
}

fn validate_live_positive(kind: &str, value: i64) -> Result<()> {
    if value <= 0 {
        return Err(WechatError::Config(format!(
            "mini-program live {kind} must be positive"
        )));
    }
    Ok(())
}

fn validate_live_flag(kind: &str, value: i64) -> Result<()> {
    if !matches!(value, 0 | 1) {
        return Err(WechatError::Config(format!(
            "mini-program live {kind} flag must be 0 or 1"
        )));
    }
    Ok(())
}

fn validate_live_room_flag(kind: &str, room_id: i64, value: i64) -> Result<()> {
    validate_live_positive("room id", room_id)?;
    validate_live_flag(kind, value)
}

fn validate_live_page(offset: i64, limit: i64, maximum: i64) -> Result<()> {
    if offset < 0 {
        return Err(WechatError::Config(
            "mini-program live pagination offset cannot be negative".to_string(),
        ));
    }
    if !(1..=maximum).contains(&limit) {
        return Err(WechatError::Config(format!(
            "mini-program live pagination limit must be between 1 and {maximum}"
        )));
    }
    Ok(())
}

fn validate_live_positive_ids(kind: &str, values: &[i64], maximum: usize) -> Result<()> {
    if values.is_empty() || values.len() > maximum {
        return Err(WechatError::Config(format!(
            "mini-program live {kind} list must contain between 1 and {maximum} entries"
        )));
    }
    let mut seen = std::collections::HashSet::with_capacity(values.len());
    for value in values {
        validate_live_positive(kind, *value)?;
        if !seen.insert(*value) {
            return Err(WechatError::Config(format!(
                "mini-program live {kind} values must be unique"
            )));
        }
    }
    Ok(())
}

fn validate_live_unique_strings(kind: &str, values: &[String], maximum: usize) -> Result<()> {
    if values.is_empty() || values.len() > maximum {
        return Err(WechatError::Config(format!(
            "mini-program live {kind} list must contain between 1 and {maximum} entries"
        )));
    }
    let mut seen = std::collections::HashSet::with_capacity(values.len());
    for value in values {
        validate_live_required(kind, value)?;
        if !seen.insert(value.trim()) {
            return Err(WechatError::Config(format!(
                "mini-program live {kind} values must be unique"
            )));
        }
    }
    Ok(())
}

fn validate_live_room_goods_ids(room_id: i64, goods_id: i64) -> Result<()> {
    validate_live_positive("room id", room_id)?;
    validate_live_positive("goods id", goods_id)
}

fn validate_live_role(username: &str, role: i64) -> Result<()> {
    validate_live_required("role username", username)?;
    validate_live_positive("role", role)
}

fn validate_live_http_url(kind: &str, value: &str) -> Result<()> {
    let url = url::Url::parse(value).map_err(|error| {
        WechatError::Config(format!("mini-program live {kind} is invalid: {error}"))
    })?;
    if !matches!(url.scheme(), "http" | "https") || url.host_str().is_none() {
        return Err(WechatError::Config(format!(
            "mini-program live {kind} must be an absolute HTTP(S) URL"
        )));
    }
    Ok(())
}

fn ensure_live_response_success(errcode: Option<i64>, errmsg: Option<&str>) -> Result<()> {
    if let Some(code) = errcode.filter(|code| *code != 0) {
        return Err(WechatError::Api {
            code,
            message: errmsg
                .unwrap_or("mini-program live operation failed")
                .to_string(),
        });
    }
    Ok(())
}

fn deserialize_optional_i64<'de, D>(deserializer: D) -> std::result::Result<Option<i64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    match Option::<Value>::deserialize(deserializer)? {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Number(value)) => value
            .as_i64()
            .ok_or_else(|| serde::de::Error::custom("live numeric value is outside i64"))
            .map(Some),
        Some(Value::String(value)) => value
            .parse::<i64>()
            .map(Some)
            .map_err(serde::de::Error::custom),
        Some(other) => Err(serde::de::Error::custom(format!(
            "live numeric value must be a number or string, got {other}"
        ))),
    }
}

fn deserialize_optional_string<'de, D>(
    deserializer: D,
) -> std::result::Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    match Option::<Value>::deserialize(deserializer)? {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(value)) => Ok(Some(value)),
        Some(Value::Number(value)) => Ok(Some(value.to_string())),
        Some(other) => Err(serde::de::Error::custom(format!(
            "string value must be a string or number, got {other}"
        ))),
    }
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
    #[serde(
        default,
        rename = "subAnchorWechat",
        skip_serializing_if = "Option::is_none"
    )]
    pub sub_anchor_wechat: Option<String>,
    #[serde(
        default,
        rename = "createrWechat",
        skip_serializing_if = "Option::is_none"
    )]
    pub creator_wechat: Option<String>,
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
    #[serde(
        default,
        rename = "isFeedsPublic",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_feeds_public: Option<i64>,
}

impl LiveRoomRequest {
    pub fn validate(&self) -> Result<()> {
        for (kind, value) in [
            ("room name", self.name.as_str()),
            ("cover image media id", self.cover_img.as_str()),
            ("anchor name", self.anchor_name.as_str()),
            ("anchor WeChat id", self.anchor_wechat.as_str()),
            ("share image media id", self.share_img.as_str()),
        ] {
            validate_live_required(kind, value)?;
        }
        validate_live_optional_non_empty(
            "sub-anchor WeChat id",
            self.sub_anchor_wechat.as_deref(),
        )?;
        validate_live_optional_non_empty("creator WeChat id", self.creator_wechat.as_deref())?;
        validate_live_optional_non_empty("feeds image media id", self.feeds_img.as_deref())?;
        if self.start_time <= 0 || self.end_time <= self.start_time {
            return Err(WechatError::Config(
                "mini-program live room requires a positive ordered time range".to_string(),
            ));
        }
        if !matches!(self.type_id, 0 | 1) {
            return Err(WechatError::Config(
                "mini-program live room type must be 0 or 1".to_string(),
            ));
        }
        if !matches!(self.screen_type, 0 | 1) {
            return Err(WechatError::Config(
                "mini-program live room screen type must be 0 or 1".to_string(),
            ));
        }
        for (kind, value) in [
            ("close-like", Some(self.close_like)),
            ("close-goods", Some(self.close_goods)),
            ("close-comment", Some(self.close_comment)),
            ("close-replay", self.close_replay),
            ("close-share", self.close_share),
            ("close-customer-service", self.close_kf),
            ("feed-public", self.is_feeds_public),
        ] {
            if let Some(value) = value {
                validate_live_flag(kind, value)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveInfoRequest {
    pub start: i64,
    pub limit: i64,
}

impl LiveInfoRequest {
    pub fn first_page(limit: i64) -> Result<Self> {
        let request = Self { start: 0, limit };
        request.validate()?;
        Ok(request)
    }

    pub fn validate(&self) -> Result<()> {
        validate_live_page(self.start, self.limit, 100)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgramLiveRoomEditRequest {
    pub id: i64,
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
    #[serde(rename = "closeLike")]
    pub close_like: i64,
    #[serde(rename = "closeGoods")]
    pub close_goods: i64,
    #[serde(rename = "closeComment")]
    pub close_comment: i64,
    #[serde(rename = "isFeedsPublic")]
    pub is_feeds_public: i64,
    #[serde(rename = "closeReplay")]
    pub close_replay: i64,
    #[serde(rename = "closeShare")]
    pub close_share: i64,
    #[serde(rename = "closeKf")]
    pub close_kf: i64,
    #[serde(rename = "feedsImg")]
    pub feeds_img: String,
}

impl MiniProgramLiveRoomEditRequest {
    pub fn validate(&self) -> Result<()> {
        validate_live_positive("room id", self.id)?;
        for (kind, value) in [
            ("room name", self.name.as_str()),
            ("cover image media id", self.cover_img.as_str()),
            ("anchor name", self.anchor_name.as_str()),
            ("anchor WeChat id", self.anchor_wechat.as_str()),
            ("share image media id", self.share_img.as_str()),
            ("feeds image media id", self.feeds_img.as_str()),
        ] {
            validate_live_required(kind, value)?;
        }
        if self.start_time <= 0 || self.end_time <= self.start_time {
            return Err(WechatError::Config(
                "mini-program live room edit requires a positive ordered time range".to_string(),
            ));
        }
        for (kind, value) in [
            ("close-like", self.close_like),
            ("close-goods", self.close_goods),
            ("close-comment", self.close_comment),
            ("feed-public", self.is_feeds_public),
            ("close-replay", self.close_replay),
            ("close-share", self.close_share),
            ("close-customer-service", self.close_kf),
        ] {
            validate_live_flag(kind, value)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgramLiveRoomAddGoodsRequest {
    #[serde(rename = "roomId")]
    pub room_id: i64,
    pub ids: Vec<i64>,
}

impl MiniProgramLiveRoomAddGoodsRequest {
    pub fn validate(&self) -> Result<()> {
        validate_live_positive("room id", self.room_id)?;
        validate_live_positive_ids("goods id", &self.ids, 100)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgramLiveAssistantUser {
    pub username: String,
    pub nickname: String,
}

impl MiniProgramLiveAssistantUser {
    fn validate(&self) -> Result<()> {
        validate_live_required("assistant username", &self.username)?;
        validate_live_required("assistant nickname", &self.nickname)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgramLiveAssistantAddRequest {
    #[serde(rename = "roomId")]
    pub room_id: i64,
    pub users: Vec<MiniProgramLiveAssistantUser>,
}

impl MiniProgramLiveAssistantAddRequest {
    pub fn validate(&self) -> Result<()> {
        validate_live_positive("room id", self.room_id)?;
        if self.users.is_empty() || self.users.len() > 10 {
            return Err(WechatError::Config(
                "mini-program live assistant list must contain between 1 and 10 users".to_string(),
            ));
        }
        let mut usernames = std::collections::HashSet::with_capacity(self.users.len());
        for user in &self.users {
            user.validate()?;
            if !usernames.insert(user.username.trim()) {
                return Err(WechatError::Config(
                    "mini-program live assistant usernames must be unique".to_string(),
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgramLiveAssistantModifyRequest {
    #[serde(rename = "roomId")]
    pub room_id: i64,
    pub username: String,
    pub nickname: String,
}

impl MiniProgramLiveAssistantModifyRequest {
    pub fn validate(&self) -> Result<()> {
        validate_live_positive("room id", self.room_id)?;
        validate_live_required("assistant username", &self.username)?;
        validate_live_required("assistant nickname", &self.nickname)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgramLiveAssistantRemoveRequest {
    #[serde(rename = "roomId")]
    pub room_id: i64,
    pub username: String,
}

impl MiniProgramLiveAssistantRemoveRequest {
    pub fn validate(&self) -> Result<()> {
        validate_live_positive("room id", self.room_id)?;
        validate_live_required("assistant username", &self.username)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgramLiveSubAnchorRequest {
    #[serde(rename = "roomId")]
    pub room_id: i64,
    pub username: String,
}

impl MiniProgramLiveSubAnchorRequest {
    pub fn validate(&self) -> Result<()> {
        validate_live_positive("room id", self.room_id)?;
        validate_live_required("sub-anchor username", &self.username)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgramLiveGoodsMutationRequest {
    #[serde(rename = "goodsInfo")]
    pub goods_info: MiniProgramLiveGoodsMutation,
}

impl MiniProgramLiveGoodsMutationRequest {
    pub fn validate_create(&self) -> Result<()> {
        self.goods_info.validate(true)
    }

    pub fn validate_update(&self) -> Result<()> {
        self.goods_info.validate(false)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgramLiveGoodsMutation {
    #[serde(default, rename = "goodsId", skip_serializing_if = "Option::is_none")]
    pub goods_id: Option<i64>,
    #[serde(
        default,
        rename = "coverImgUrl",
        skip_serializing_if = "Option::is_none"
    )]
    pub cover_img_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, rename = "priceType", skip_serializing_if = "Option::is_none")]
    pub price_type: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price2: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(
        default,
        rename = "thirdPartyAppid",
        skip_serializing_if = "Option::is_none"
    )]
    pub third_party_appid: Option<String>,
}

impl MiniProgramLiveGoodsMutation {
    fn validate(&self, creating: bool) -> Result<()> {
        if creating && self.goods_id.is_some() {
            return Err(WechatError::Config(
                "mini-program live goods creation must not include goods id".to_string(),
            ));
        }
        if creating {
            for (kind, value) in [
                ("goods cover image URL", self.cover_img_url.as_deref()),
                ("goods name", self.name.as_deref()),
                ("goods page path", self.url.as_deref()),
            ] {
                let value = value.ok_or_else(|| {
                    WechatError::Config(format!("mini-program live {kind} is required"))
                })?;
                validate_live_required(kind, value)?;
            }
        } else {
            validate_live_positive(
                "goods id",
                self.goods_id.ok_or_else(|| {
                    WechatError::Config(
                        "mini-program live goods update requires goods id".to_string(),
                    )
                })?,
            )?;
            if self.cover_img_url.is_none()
                && self.name.is_none()
                && self.price_type.is_none()
                && self.price.is_none()
                && self.price2.is_none()
                && self.url.is_none()
                && self.third_party_appid.is_none()
            {
                return Err(WechatError::Config(
                    "mini-program live goods update requires at least one changed field"
                        .to_string(),
                ));
            }
        }
        validate_live_optional_non_empty("goods cover image URL", self.cover_img_url.as_deref())?;
        validate_live_optional_non_empty("goods name", self.name.as_deref())?;
        validate_live_optional_non_empty("goods page path", self.url.as_deref())?;
        validate_live_optional_non_empty(
            "goods third-party appid",
            self.third_party_appid.as_deref(),
        )?;
        if let Some(price_type) = self.price_type {
            if !matches!(price_type, 1..=3) {
                return Err(WechatError::Config(
                    "mini-program live goods price type must be 1, 2, or 3".to_string(),
                ));
            }
        } else if creating {
            return Err(WechatError::Config(
                "mini-program live goods price type is required".to_string(),
            ));
        }
        for price in [self.price, self.price2].into_iter().flatten() {
            if !price.is_finite() || price < 0.0 {
                return Err(WechatError::Config(
                    "mini-program live goods prices must be finite and non-negative".to_string(),
                ));
            }
        }
        if creating && self.price.is_none() {
            return Err(WechatError::Config(
                "mini-program live goods price is required".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgramLiveGoodsListRequest {
    pub offset: i64,
    pub count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

impl MiniProgramLiveGoodsListRequest {
    pub fn validate(&self) -> Result<()> {
        validate_live_page(self.offset, self.count, 100)?;
        if self.status.is_some_and(|status| !(0..=3).contains(&status)) {
            return Err(WechatError::Config(
                "mini-program live goods status must be between 0 and 3".to_string(),
            ));
        }
        Ok(())
    }

    fn query(&self) -> Vec<(String, String)> {
        let mut query = vec![
            ("offset".to_string(), self.offset.to_string()),
            ("count".to_string(), self.count.to_string()),
        ];
        if let Some(status) = self.status {
            query.push(("status".to_string(), status.to_string()));
        }
        query
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgramLiveRoleListRequest {
    pub role: i64,
    pub offset: i64,
    pub limit: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
}

impl MiniProgramLiveRoleListRequest {
    pub fn validate(&self) -> Result<()> {
        validate_live_positive("role", self.role)?;
        validate_live_page(self.offset, self.limit, 100)?;
        validate_live_optional_non_empty("role keyword", self.keyword.as_deref())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgramCreateLiveRoomResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default, rename = "roomId")]
    pub room_id: Option<i64>,
    #[serde(default)]
    pub qrcode_url: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl MiniProgramCreateLiveRoomResponse {
    pub fn require_room_id(&self) -> Result<i64> {
        ensure_live_response_success(self.errcode, self.errmsg.as_deref())?;
        let room_id = self.room_id.ok_or_else(|| {
            WechatError::Config(
                "mini-program live create-room response is missing room id".to_string(),
            )
        })?;
        validate_live_positive("room id", room_id)?;
        if let Some(url) = self.qrcode_url.as_deref() {
            validate_live_http_url("room QR-code URL", url)?;
        }
        Ok(room_id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgramLiveInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub room_info: Vec<MiniProgramLiveRoomInfo>,
    #[serde(default)]
    pub total: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl MiniProgramLiveInfoResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_live_response_success(self.errcode, self.errmsg.as_deref())?;
        if self.total.is_some_and(|total| total < 0) {
            return Err(WechatError::Config(
                "mini-program live room total cannot be negative".to_string(),
            ));
        }
        if self.total.is_some_and(|total| {
            i64::try_from(self.room_info.len()).is_ok_and(|count| count > total)
        }) {
            return Err(WechatError::Config(
                "mini-program live decoded room count cannot exceed total".to_string(),
            ));
        }
        let mut room_ids = std::collections::HashSet::with_capacity(self.room_info.len());
        for room in &self.room_info {
            room.validate()?;
            let room_id = room.require_room_id()?;
            if !room_ids.insert(room_id) {
                return Err(WechatError::Config(
                    "mini-program live room list contains duplicate room ids".to_string(),
                ));
            }
        }
        Ok(())
    }

    pub fn find_room(&self, room_id: i64) -> Option<&MiniProgramLiveRoomInfo> {
        self.room_info
            .iter()
            .find(|room| room.roomid == Some(room_id))
    }

    pub fn next_start(&self, current_start: i64) -> Result<Option<i64>> {
        self.validate()?;
        if current_start < 0 {
            return Err(WechatError::Config(
                "mini-program live current start cannot be negative".to_string(),
            ));
        }
        let count = i64::try_from(self.room_info.len()).map_err(|_| {
            WechatError::Config("mini-program live room page count exceeds i64".to_string())
        })?;
        let next = current_start.checked_add(count).ok_or_else(|| {
            WechatError::Config("mini-program live next room offset overflowed".to_string())
        })?;
        match self.total {
            Some(total) if next < total && count == 0 => Err(WechatError::Config(
                "mini-program live room pagination cannot advance an empty page".to_string(),
            )),
            Some(total) if next < total => Ok(Some(next)),
            _ => Ok(None),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgramLiveRoomInfo {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub roomid: Option<i64>,
    #[serde(default, alias = "coverImg")]
    pub cover_img: Option<String>,
    #[serde(default, alias = "shareImg")]
    pub share_img: Option<String>,
    #[serde(default, alias = "liveStatus")]
    pub live_status: Option<i64>,
    #[serde(default, alias = "startTime")]
    pub start_time: Option<i64>,
    #[serde(default, alias = "endTime")]
    pub end_time: Option<i64>,
    #[serde(default, alias = "anchorName")]
    pub anchor_name: Option<String>,
    #[serde(default, alias = "anchorWechat")]
    pub anchor_wechat: Option<String>,
    #[serde(default)]
    pub goods: Vec<MiniProgramLiveRoomGoods>,
    #[serde(default)]
    pub total: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MiniProgramLiveStatusKind {
    Living,
    NotStarted,
    Ended,
    Forbidden,
    Paused,
    Abnormal,
    Expired,
    Other(i64),
}

impl From<i64> for MiniProgramLiveStatusKind {
    fn from(value: i64) -> Self {
        match value {
            101 => Self::Living,
            102 => Self::NotStarted,
            103 => Self::Ended,
            104 => Self::Forbidden,
            105 => Self::Paused,
            106 => Self::Abnormal,
            107 => Self::Expired,
            other => Self::Other(other),
        }
    }
}

impl MiniProgramLiveRoomInfo {
    pub fn live_status_kind(&self) -> Option<MiniProgramLiveStatusKind> {
        self.live_status.map(MiniProgramLiveStatusKind::from)
    }

    pub fn require_room_id(&self) -> Result<i64> {
        let room_id = self.roomid.ok_or_else(|| {
            WechatError::Config("mini-program live room is missing room id".to_string())
        })?;
        validate_live_positive("room id", room_id)?;
        Ok(room_id)
    }

    pub fn validate(&self) -> Result<()> {
        self.require_room_id()?;
        validate_live_optional_non_empty("room name", self.name.as_deref())?;
        validate_live_optional_non_empty("room anchor name", self.anchor_name.as_deref())?;
        validate_live_optional_non_empty("room anchor WeChat id", self.anchor_wechat.as_deref())?;
        if self.start_time.is_some_and(|time| time < 0)
            || self.end_time.is_some_and(|time| time < 0)
            || self
                .start_time
                .zip(self.end_time)
                .is_some_and(|(start, end)| end < start)
        {
            return Err(WechatError::Config(
                "mini-program live room timestamps are inconsistent".to_string(),
            ));
        }
        if self.total.is_some_and(|total| total < 0) {
            return Err(WechatError::Config(
                "mini-program live room goods total cannot be negative".to_string(),
            ));
        }
        let mut goods_ids = std::collections::HashSet::with_capacity(self.goods.len());
        for goods in &self.goods {
            goods.validate()?;
            let goods_id = goods.require_goods_id()?;
            if !goods_ids.insert(goods_id) {
                return Err(WechatError::Config(
                    "mini-program live room contains duplicate goods ids".to_string(),
                ));
            }
        }
        Ok(())
    }
}

impl MiniProgramLiveStatusKind {
    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Ended | Self::Forbidden | Self::Abnormal | Self::Expired
        )
    }

    pub fn needs_attention(self) -> bool {
        matches!(self, Self::Forbidden | Self::Abnormal | Self::Expired)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgramLiveRoomGoods {
    #[serde(default, alias = "goodsId")]
    pub goods_id: Option<i64>,
    #[serde(default, alias = "coverImg")]
    pub cover_img: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub price: Option<i64>,
    #[serde(default)]
    pub price2: Option<i64>,
    #[serde(default, alias = "priceType")]
    pub price_type: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MiniProgramLiveGoodsPriceType {
    Fixed,
    Range,
    Discount,
    Other(i64),
}

impl From<i64> for MiniProgramLiveGoodsPriceType {
    fn from(value: i64) -> Self {
        match value {
            1 => Self::Fixed,
            2 => Self::Range,
            3 => Self::Discount,
            other => Self::Other(other),
        }
    }
}

impl MiniProgramLiveRoomGoods {
    pub fn price_type_kind(&self) -> Option<MiniProgramLiveGoodsPriceType> {
        self.price_type.map(MiniProgramLiveGoodsPriceType::from)
    }

    pub fn require_goods_id(&self) -> Result<i64> {
        let goods_id = self.goods_id.ok_or_else(|| {
            WechatError::Config("mini-program live room goods is missing goods id".to_string())
        })?;
        validate_live_positive("goods id", goods_id)?;
        Ok(goods_id)
    }

    pub fn validate(&self) -> Result<()> {
        self.require_goods_id()?;
        validate_live_optional_non_empty("room goods name", self.name.as_deref())?;
        validate_live_optional_non_empty("room goods path", self.url.as_deref())?;
        if [self.price, self.price2]
            .into_iter()
            .flatten()
            .any(|price| price < 0)
        {
            return Err(WechatError::Config(
                "mini-program live room goods prices cannot be negative".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgramLiveReplayResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub live_replay: Vec<MiniProgramLiveReplay>,
    #[serde(default)]
    pub total: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgramLiveReplay {
    #[serde(default)]
    pub create_time: Option<String>,
    #[serde(default)]
    pub expire_time: Option<String>,
    #[serde(default)]
    pub media_url: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl MiniProgramLiveReplay {
    pub fn validate(&self) -> Result<()> {
        validate_live_optional_non_empty("replay create time", self.create_time.as_deref())?;
        validate_live_optional_non_empty("replay expire time", self.expire_time.as_deref())?;
        if let Some(url) = self.media_url.as_deref() {
            validate_live_http_url("replay media URL", url)?;
        }
        Ok(())
    }
}

impl MiniProgramLiveReplayResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_live_response_success(self.errcode, self.errmsg.as_deref())?;
        if self.total.is_some_and(|total| total < 0) {
            return Err(WechatError::Config(
                "mini-program live replay total cannot be negative".to_string(),
            ));
        }
        for replay in &self.live_replay {
            replay.validate()?;
        }
        Ok(())
    }

    pub fn next_start(&self, current_start: i64) -> Result<Option<i64>> {
        self.validate()?;
        if current_start < 0 {
            return Err(WechatError::Config(
                "mini-program live replay start cannot be negative".to_string(),
            ));
        }
        let count = i64::try_from(self.live_replay.len()).map_err(|_| {
            WechatError::Config("mini-program live replay page count exceeds i64".to_string())
        })?;
        let next = current_start.checked_add(count).ok_or_else(|| {
            WechatError::Config("mini-program live next replay offset overflowed".to_string())
        })?;
        match self.total {
            Some(total) if next < total && count == 0 => Err(WechatError::Config(
                "mini-program live replay pagination cannot advance an empty page".to_string(),
            )),
            Some(total) if next < total => Ok(Some(next)),
            _ => Ok(None),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgramLiveGoodsWarehouseResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub goods: Vec<MiniProgramLiveWarehouseGoods>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgramLiveWarehouseGoods {
    #[serde(default, alias = "goodsId")]
    pub goods_id: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default, alias = "coverImgUrl")]
    pub cover_img_url: Option<String>,
    #[serde(default)]
    pub price: Option<i64>,
    #[serde(default)]
    pub price2: Option<i64>,
    #[serde(default, alias = "priceType")]
    pub price_type: Option<i64>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default, alias = "auditStatus")]
    pub audit_status: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MiniProgramLiveGoodsAuditStatusKind {
    NotSubmitted,
    Auditing,
    Approved,
    Rejected,
    Other(i64),
}

impl From<i64> for MiniProgramLiveGoodsAuditStatusKind {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::NotSubmitted,
            1 => Self::Auditing,
            2 => Self::Approved,
            3 => Self::Rejected,
            other => Self::Other(other),
        }
    }
}

impl MiniProgramLiveWarehouseGoods {
    pub fn price_type_kind(&self) -> Option<MiniProgramLiveGoodsPriceType> {
        self.price_type.map(MiniProgramLiveGoodsPriceType::from)
    }

    pub fn audit_status_kind(&self) -> Option<MiniProgramLiveGoodsAuditStatusKind> {
        self.audit_status
            .map(MiniProgramLiveGoodsAuditStatusKind::from)
    }

    pub fn require_goods_id(&self) -> Result<i64> {
        let goods_id = self.goods_id.ok_or_else(|| {
            WechatError::Config("mini-program live warehouse goods is missing goods id".to_string())
        })?;
        validate_live_positive("goods id", goods_id)?;
        Ok(goods_id)
    }

    pub fn validate(&self) -> Result<()> {
        self.require_goods_id()?;
        validate_live_optional_non_empty("warehouse goods name", self.name.as_deref())?;
        validate_live_optional_non_empty("warehouse goods path", self.url.as_deref())?;
        if [self.price, self.price2]
            .into_iter()
            .flatten()
            .any(|price| price < 0)
        {
            return Err(WechatError::Config(
                "mini-program live warehouse goods prices cannot be negative".to_string(),
            ));
        }
        Ok(())
    }

    pub fn is_approved(&self) -> bool {
        self.audit_status_kind() == Some(MiniProgramLiveGoodsAuditStatusKind::Approved)
    }
}

impl MiniProgramLiveGoodsWarehouseResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_live_response_success(self.errcode, self.errmsg.as_deref())?;
        let mut goods_ids = std::collections::HashSet::with_capacity(self.goods.len());
        for goods in &self.goods {
            goods.validate()?;
            let goods_id = goods.require_goods_id()?;
            if !goods_ids.insert(goods_id) {
                return Err(WechatError::Config(
                    "mini-program live warehouse contains duplicate goods ids".to_string(),
                ));
            }
        }
        Ok(())
    }

    pub fn find(&self, goods_id: i64) -> Option<&MiniProgramLiveWarehouseGoods> {
        self.goods
            .iter()
            .find(|goods| goods.goods_id == Some(goods_id))
    }

    pub fn approved(&self) -> Vec<&MiniProgramLiveWarehouseGoods> {
        self.goods
            .iter()
            .filter(|goods| goods.is_approved())
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgramLiveFollowersResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub followers: Vec<MiniProgramLiveFollower>,
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub page_break: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgramLiveFollower {
    #[serde(default)]
    pub openid: Option<String>,
    #[serde(default)]
    pub nickname: Option<String>,
    #[serde(default)]
    pub headimg: Option<String>,
    #[serde(default, alias = "subscribeTime")]
    pub subscribe_time: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl MiniProgramLiveFollowersResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_live_response_success(self.errcode, self.errmsg.as_deref())?;
        if self.page_break.is_some_and(|page_break| page_break < 0) {
            return Err(WechatError::Config(
                "mini-program live follower page break cannot be negative".to_string(),
            ));
        }
        let mut openids = std::collections::HashSet::with_capacity(self.followers.len());
        for follower in &self.followers {
            let openid = follower.openid.as_deref().ok_or_else(|| {
                WechatError::Config("mini-program live follower is missing openid".to_string())
            })?;
            validate_live_required("follower openid", openid)?;
            if follower.subscribe_time.is_some_and(|time| time < 0) {
                return Err(WechatError::Config(
                    "mini-program live follower subscribe time cannot be negative".to_string(),
                ));
            }
            if !openids.insert(openid.trim()) {
                return Err(WechatError::Config(
                    "mini-program live follower list contains duplicate openids".to_string(),
                ));
            }
        }
        Ok(())
    }

    pub fn next_page_break(&self) -> Result<Option<i64>> {
        self.validate()?;
        Ok(self.page_break.filter(|page_break| *page_break > 0))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgramLivePushUrlResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default, rename = "pushAddr")]
    pub push_addr: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl MiniProgramLivePushUrlResponse {
    pub fn require_push_address(&self) -> Result<&str> {
        ensure_live_response_success(self.errcode, self.errmsg.as_deref())?;
        let address = self.push_addr.as_deref().ok_or_else(|| {
            WechatError::Config(
                "mini-program live push-url response is missing push address".to_string(),
            )
        })?;
        validate_live_required("push address", address)?;
        Ok(address)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgramLiveSharedCodeResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default, rename = "cdnUrl")]
    pub cdn_url: Option<String>,
    #[serde(default, rename = "pagePath")]
    pub page_path: Option<String>,
    #[serde(default, rename = "posterUrl")]
    pub poster_url: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl MiniProgramLiveSharedCodeResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_live_response_success(self.errcode, self.errmsg.as_deref())?;
        for (kind, value) in [
            ("shared-code CDN URL", self.cdn_url.as_deref()),
            ("shared-code poster URL", self.poster_url.as_deref()),
        ] {
            let value = value.ok_or_else(|| {
                WechatError::Config(format!("mini-program live {kind} is missing"))
            })?;
            validate_live_http_url(kind, value)?;
        }
        let page_path = self.page_path.as_deref().ok_or_else(|| {
            WechatError::Config("mini-program live shared-code page path is missing".to_string())
        })?;
        validate_live_required("shared-code page path", page_path)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgramLiveAssistantListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub list: Vec<MiniProgramLiveAssistant>,
    #[serde(default)]
    pub count: Option<i64>,
    #[serde(default, rename = "maxCount")]
    pub max_count: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgramLiveAssistant {
    #[serde(default)]
    pub timestamp: Option<i64>,
    #[serde(default)]
    pub headimg: Option<String>,
    #[serde(default)]
    pub nickname: Option<String>,
    #[serde(default)]
    pub alias: Option<String>,
    #[serde(default)]
    pub openid: Option<String>,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl MiniProgramLiveAssistantListResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_live_response_success(self.errcode, self.errmsg.as_deref())?;
        if [self.count, self.max_count]
            .into_iter()
            .flatten()
            .any(|count| count < 0)
        {
            return Err(WechatError::Config(
                "mini-program live assistant counts cannot be negative".to_string(),
            ));
        }
        let actual = i64::try_from(self.list.len()).map_err(|_| {
            WechatError::Config("mini-program live assistant count exceeds i64".to_string())
        })?;
        if self.count.is_some_and(|count| count != actual) {
            return Err(WechatError::Config(
                "mini-program live assistant count does not match decoded list".to_string(),
            ));
        }
        if self
            .count
            .zip(self.max_count)
            .is_some_and(|(count, maximum)| count > maximum)
        {
            return Err(WechatError::Config(
                "mini-program live assistant count exceeds maximum".to_string(),
            ));
        }
        for assistant in &self.list {
            if assistant.timestamp.is_some_and(|time| time < 0) {
                return Err(WechatError::Config(
                    "mini-program live assistant timestamp cannot be negative".to_string(),
                ));
            }
            if assistant.openid.is_none() && assistant.username.is_none() {
                return Err(WechatError::Config(
                    "mini-program live assistant requires openid or username".to_string(),
                ));
            }
            validate_live_optional_non_empty("assistant openid", assistant.openid.as_deref())?;
            validate_live_optional_non_empty("assistant username", assistant.username.as_deref())?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgramLiveSubAnchorResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl MiniProgramLiveSubAnchorResponse {
    pub fn require_username(&self) -> Result<&str> {
        ensure_live_response_success(self.errcode, self.errmsg.as_deref())?;
        let username = self.username.as_deref().ok_or_else(|| {
            WechatError::Config(
                "mini-program live sub-anchor response is missing username".to_string(),
            )
        })?;
        validate_live_required("sub-anchor username", username)?;
        Ok(username)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgramLiveGoodsVideoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl MiniProgramLiveGoodsVideoResponse {
    pub fn require_url(&self) -> Result<&str> {
        ensure_live_response_success(self.errcode, self.errmsg.as_deref())?;
        let url = self.url.as_deref().ok_or_else(|| {
            WechatError::Config("mini-program live goods video URL is missing".to_string())
        })?;
        validate_live_http_url("goods video URL", url)?;
        Ok(url)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgramLiveGoodsMutationResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(
        default,
        rename = "goodsId",
        deserialize_with = "deserialize_optional_i64"
    )]
    pub goods_id: Option<i64>,
    #[serde(default, rename = "auditId")]
    pub audit_id: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl MiniProgramLiveGoodsMutationResponse {
    pub fn require_ids(&self) -> Result<(i64, i64)> {
        ensure_live_response_success(self.errcode, self.errmsg.as_deref())?;
        let goods_id = self.goods_id.ok_or_else(|| {
            WechatError::Config("mini-program live goods response is missing goods id".to_string())
        })?;
        let audit_id = self.audit_id.ok_or_else(|| {
            WechatError::Config("mini-program live goods response is missing audit id".to_string())
        })?;
        validate_live_positive("goods id", goods_id)?;
        validate_live_positive("audit id", audit_id)?;
        Ok((goods_id, audit_id))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgramLiveGoodsAuditResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default, rename = "auditId")]
    pub audit_id: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl MiniProgramLiveGoodsAuditResponse {
    pub fn require_audit_id(&self) -> Result<i64> {
        ensure_live_response_success(self.errcode, self.errmsg.as_deref())?;
        let audit_id = self.audit_id.ok_or_else(|| {
            WechatError::Config(
                "mini-program live goods-audit response is missing audit id".to_string(),
            )
        })?;
        validate_live_positive("audit id", audit_id)?;
        Ok(audit_id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgramLiveGoodsListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub total: Option<i64>,
    #[serde(default)]
    pub goods: Vec<MiniProgramLiveWarehouseGoods>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl MiniProgramLiveGoodsListResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_live_response_success(self.errcode, self.errmsg.as_deref())?;
        if self.total.is_some_and(|total| total < 0) {
            return Err(WechatError::Config(
                "mini-program live goods total cannot be negative".to_string(),
            ));
        }
        let warehouse = MiniProgramLiveGoodsWarehouseResponse {
            errcode: Some(0),
            errmsg: None,
            goods: self.goods.clone(),
            extra: Value::Null,
        };
        warehouse.validate()
    }

    pub fn next_offset(&self, current_offset: i64) -> Result<Option<i64>> {
        self.validate()?;
        if current_offset < 0 {
            return Err(WechatError::Config(
                "mini-program live goods offset cannot be negative".to_string(),
            ));
        }
        let count = i64::try_from(self.goods.len()).map_err(|_| {
            WechatError::Config("mini-program live goods page count exceeds i64".to_string())
        })?;
        let next = current_offset.checked_add(count).ok_or_else(|| {
            WechatError::Config("mini-program live goods next offset overflowed".to_string())
        })?;
        match self.total {
            Some(total) if next < total && count == 0 => Err(WechatError::Config(
                "mini-program live goods pagination cannot advance an empty page".to_string(),
            )),
            Some(total) if next < total => Ok(Some(next)),
            _ => Ok(None),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgramLiveRoleListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub total: Option<i64>,
    #[serde(default)]
    pub list: Vec<MiniProgramLiveRoleMember>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgramLiveRoleMember {
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub nickname: Option<String>,
    #[serde(default)]
    pub role: Option<i64>,
    #[serde(default)]
    pub headimg: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl MiniProgramLiveRoleListResponse {
    pub fn validate(&self) -> Result<()> {
        ensure_live_response_success(self.errcode, self.errmsg.as_deref())?;
        if self.total.is_some_and(|total| total < 0) {
            return Err(WechatError::Config(
                "mini-program live role total cannot be negative".to_string(),
            ));
        }
        let mut usernames = std::collections::HashSet::with_capacity(self.list.len());
        for member in &self.list {
            let username = member.username.as_deref().ok_or_else(|| {
                WechatError::Config("mini-program live role member is missing username".to_string())
            })?;
            validate_live_required("role username", username)?;
            if member.role.is_some_and(|role| role <= 0) {
                return Err(WechatError::Config(
                    "mini-program live role member role must be positive".to_string(),
                ));
            }
            if !usernames.insert(username.trim()) {
                return Err(WechatError::Config(
                    "mini-program live role list contains duplicate usernames".to_string(),
                ));
            }
        }
        Ok(())
    }
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WxaCodeResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub ticket: Option<String>,
    #[serde(default)]
    pub expire_seconds: Option<i64>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub buffer: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
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

        let uniform = serde_json::to_value(UniformMessageRequest {
            touser: "openid".to_string(),
            weapp_template_msg: Some(json!({
                "template_id": "mini-tpl",
                "page": "pages/index",
                "data": { "thing1": { "value": "mini" } }
            })),
            mp_template_msg: Some(json!({
                "appid": "wx-official",
                "template_id": "official-tpl",
                "url": "https://example.com",
                "data": { "first": { "value": "official" } }
            })),
        })
        .unwrap();
        assert_eq!(uniform["touser"], "openid");
        assert_eq!(uniform["weapp_template_msg"]["template_id"], "mini-tpl");
        assert_eq!(uniform["mp_template_msg"]["appid"], "wx-official");

        let updatable = serde_json::to_value(UpdatableMessageRequest {
            activity_id: "activity".to_string(),
            target_state: 1,
            template_info: json!({
                "parameter_list": [{
                    "name": "member_count",
                    "value": "2"
                }]
            }),
        })
        .unwrap();
        assert_eq!(updatable["activity_id"], "activity");
        assert_eq!(updatable["target_state"], 1);
        assert_eq!(
            updatable["template_info"]["parameter_list"][0]["name"],
            "member_count"
        );
    }

    #[test]
    fn serializes_qr_code_default_width() {
        let value = serde_json::to_value(CreateQrCodeRequest {
            path: "pages/index".to_string(),
            width: 430,
        })
        .unwrap();

        assert_eq!(value, json!({ "path": "pages/index", "width": 430 }));

        let response: WxaCodeResponse = serde_json::from_value(json!({
            "errcode": 0,
            "ticket": "ticket",
            "expire_seconds": 1800,
            "url": "https://mp.weixin.qq.com/cgi-bin/showqrcode?ticket=ticket",
            "buffer": "base64",
            "extra_field": "kept"
        }))
        .unwrap();
        assert_eq!(response.ticket.as_deref(), Some("ticket"));
        assert_eq!(response.expire_seconds, Some(1800));
        assert_eq!(response.buffer.as_deref(), Some("base64"));
        assert_eq!(response.extra["extra_field"], "kept");
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

        let performance = serde_json::to_value(DataCubePerformanceDataRequest {
            cost_time_type: 1,
            default_start_time: 1,
            default_end_time: 2,
            device: 1,
            networktype: 1,
            extra: serde_json::Value::Null,
        })
        .unwrap();
        assert_eq!(performance["cost_time_type"], 1);
        assert_eq!(performance["networktype"], 1);
        assert!(performance.get("extra").is_none());

        let trend: DataCubeVisitTrendResponse = serde_json::from_value(json!({
            "errcode": 0,
            "list": [{
                "ref_date": "20260704",
                "session_cnt": 10,
                "visit_pv": 20,
                "visit_uv": 5,
                "visit_uv_new": 2,
                "stay_time_uv": 3.5,
                "stay_time_session": 2.5,
                "visit_depth": 1.5,
                "extra_field": "kept"
            }]
        }))
        .unwrap();
        assert_eq!(trend.list[0].ref_date.as_deref(), Some("20260704"));
        assert_eq!(trend.list[0].visit_pv, Some(20));
        assert_eq!(trend.list[0].extra["extra_field"], "kept");

        let retain: DataCubeRetainInfoResponse = serde_json::from_value(json!({
            "visit_uv_new": [{ "key": 0, "value": 10 }],
            "visit_uv": [{ "key": 1, "value": 8 }]
        }))
        .unwrap();
        assert_eq!(retain.visit_uv_new[0].value, Some(10));
        assert_eq!(retain.visit_uv[0].key, Some(1));

        let page: DataCubeVisitPageResponse = serde_json::from_value(json!({
            "list": [{
                "page_path": "pages/index",
                "page_visit_pv": 20,
                "page_visit_uv": 5,
                "page_staytime_pv": 1.5,
                "entrypage_pv": 3,
                "exitpage_pv": 2,
                "page_share_pv": 1,
                "page_share_uv": 1,
                "extra_field": "kept"
            }]
        }))
        .unwrap();
        assert_eq!(page.list[0].page_path.as_deref(), Some("pages/index"));
        assert_eq!(page.list[0].page_share_pv, Some(1));
        assert_eq!(page.list[0].extra["extra_field"], "kept");

        let portrait: DataCubeUserPortraitResponse = serde_json::from_value(json!({
            "visit_uv_new": { "province": [{ "id": 1, "name": "Beijing", "value": 10 }] },
            "visit_uv": { "city": [{ "id": 1, "name": "Beijing", "value": 8 }] }
        }))
        .unwrap();
        assert_eq!(
            portrait.visit_uv_new.unwrap()["province"][0]["name"],
            "Beijing"
        );

        let performance_response: DataCubePerformanceDataResponse = serde_json::from_value(json!({
            "errcode": 0,
            "data": [{ "cost_time": 100, "count": 2 }],
            "extra_field": "kept"
        }))
        .unwrap();
        assert_eq!(performance_response.data[0]["cost_time"], 100);
        assert_eq!(performance_response.extra["extra_field"], "kept");
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

        let msg: SecurityMsgSecCheckResponse = serde_json::from_value(json!({
            "errcode": 0,
            "result": { "suggest": "pass", "label": 100 },
            "detail": [{
                "strategy": "content_model",
                "errcode": 0,
                "suggest": "pass",
                "label": 100,
                "prob": 0.9,
                "keyword": "hello"
            }],
            "trace_id": "trace"
        }))
        .unwrap();
        assert_eq!(msg.result.unwrap().suggest.as_deref(), Some("pass"));
        assert_eq!(msg.detail[0].label, Some(100));
        assert_eq!(msg.trace_id.as_deref(), Some("trace"));

        let media: SecurityMediaCheckAsyncResponse =
            serde_json::from_value(json!({ "errcode": 0, "trace_id": "trace" })).unwrap();
        assert_eq!(media.trace_id.as_deref(), Some("trace"));
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
    fn serializes_and_validates_typed_operation_requests() {
        let list = OperationJsErrListRequest {
            app_version: "0".to_string(),
            error_type: "1".to_string(),
            start_time: "2026-07-01".to_string(),
            end_time: "2026-07-02".to_string(),
            keyword: "TypeError".to_string(),
            openid: String::new(),
            orderby: "pv".to_string(),
            desc: "1".to_string(),
            offset: 0,
            limit: 30,
        };
        list.validate().unwrap();
        let list_value = serde_json::to_value(&list).unwrap();
        assert_eq!(list_value["appVersion"], "0");
        assert_eq!(list_value["errType"], "1");
        assert_eq!(list_value["startTime"], "2026-07-01");

        let detail = OperationJsErrDetailRequest {
            start_time: "2026-07-01".to_string(),
            end_time: "2026-07-02".to_string(),
            error_message_md5: "d41d8cd98f00b204e9800998ecf8427e".to_string(),
            error_stack_md5: "d41d8cd98f00b204e9800998ecf8427e".to_string(),
            app_version: "0".to_string(),
            sdk_version: "0".to_string(),
            os_name: "0".to_string(),
            client_version: "0".to_string(),
            openid: String::new(),
            desc: "1".to_string(),
            offset: 0,
            limit: 20,
        };
        detail.validate().unwrap();
        let detail_value = serde_json::to_value(&detail).unwrap();
        assert_eq!(
            detail_value["errorMsgMd5"],
            "d41d8cd98f00b204e9800998ecf8427e"
        );
        assert_eq!(detail_value["clientVersion"], "0");

        let search = OperationJsErrSearchRequest {
            errmsg_keyword: String::new(),
            query_type: 1,
            client_version: String::new(),
            start_time: 1_783_036_800,
            end_time: 1_783_123_200,
            start: 0,
            limit: 20,
        };
        search.validate().unwrap();
        assert_eq!(serde_json::to_value(&search).unwrap()["type"], 1);

        let performance = OperationPerformanceRequest {
            cost_time_type: 2,
            default_start_time: 1_783_036_800,
            default_end_time: 1_783_123_200,
            device: "@_all:".to_string(),
            is_download_code: "@_all:".to_string(),
            scene: "1007".to_string(),
            networktype: "wifi".to_string(),
        };
        performance.validate().unwrap();

        let real_time = OperationRealTimeLogSearchRequest {
            date: "20260703".to_string(),
            begintime: 1_783_036_800,
            endtime: 1_783_040_400,
            start: 0,
            limit: 20,
            trace_id: Some("trace-id".to_string()),
            url: Some("pages/index/index".to_string()),
            id: None,
            filter_message: Some("checkout".to_string()),
            level: Some(4),
        };
        real_time.validate().unwrap();
        let log_value = serde_json::to_value(&real_time).unwrap();
        assert_eq!(log_value["traceId"], "trace-id");
        assert_eq!(log_value["filterMsg"], "checkout");
        let log_query = real_time.to_query();
        assert!(log_query.contains(&("traceId".to_string(), "trace-id".to_string())));
        assert!(log_query.contains(&("level".to_string(), "4".to_string())));

        let mut invalid_list = list;
        invalid_list.limit = 31;
        assert!(invalid_list.validate().is_err());
        let mut invalid_performance = performance;
        invalid_performance.networktype = "5g".to_string();
        assert!(invalid_performance.validate().is_err());
        assert!(OperationRequest::new(json!({})).validate().is_err());
        assert!(OperationRequest::new(json!([])).validate().is_err());
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
            "list": [{
                "record_id": 1,
                "content": "feedback",
                "phone": "13800000000",
                "openid": "openid",
                "create_time": 1_800_000_000,
                "system_info": "iOS",
                "media_id": ["media-id"]
            }],
            "total_num": 1
        }))
        .unwrap();
        assert_eq!(feedback.total_num, Some(1));
        assert_eq!(feedback.list[0].content.as_deref(), Some("feedback"));
        assert_eq!(feedback.list[0].media_id[0], "media-id");

        let gray: OperationGrayReleasePlanResponse = serde_json::from_value(json!({
            "errcode": 0,
            "gray_release_plan": {
                "status": 1,
                "gray_percentage": 30,
                "support_experiencer_first": true,
                "support_debuger_first": false
            }
        }))
        .unwrap();
        assert_eq!(
            gray.gray_release_plan
                .expect("gray_release_plan")
                .gray_percentage,
            Some(30)
        );

        let detail: OperationJsErrDetailResponse = serde_json::from_value(json!({
            "errcode": 0,
            "success": true,
            "openid": "openid",
            "data": [{
                "message": "TypeError",
                "stack": "stack",
                "time": 1_800_000_000,
                "app_version": "1.0.0",
                "sdk_version": "3.7.0",
                "client_version": "8.0.0",
                "device": "iPhone"
            }]
        }))
        .unwrap();
        assert_eq!(detail.success, Some(true));
        assert_eq!(detail.data[0].message.as_deref(), Some("TypeError"));
        assert_eq!(detail.data[0].device.as_deref(), Some("iPhone"));

        let list: OperationJsErrListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "success": true,
            "openid": "openid",
            "data": [{
                "message": "TypeError",
                "count": 3,
                "app_version": "1.0.0",
                "first_time": 1_800_000_000,
                "last_time": 1_800_000_100
            }],
            "totalCount": 3
        }))
        .unwrap();
        assert_eq!(list.total_count, Some(3));
        assert_eq!(list.data[0].count, Some(3));
        assert_eq!(list.data[0].message.as_deref(), Some("TypeError"));

        let search: OperationJsErrSearchResponse = serde_json::from_value(json!({
            "errcode": 0,
            "results": { "items": [{ "message": "TypeError" }] },
            "total": 1
        }))
        .unwrap();
        assert_eq!(search.total, Some(1));
        assert_eq!(
            search.results.expect("results").items[0].message.as_deref(),
            Some("TypeError")
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
        assert_eq!(scene.scene[0].name.as_deref(), Some("chat"));
        assert_eq!(scene.scene[0].value, Some(1007));

        let version: OperationVersionListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "cvlist": [{ "version": "8.0.0", "percentage": 50 }]
        }))
        .unwrap();
        assert_eq!(version.cvlist[0].version.as_deref(), Some("8.0.0"));
        assert_eq!(version.cvlist[0].percentage, Some(50));

        let real_time: OperationRealTimeLogSearchResponse = serde_json::from_value(json!({
            "errcode": 0,
            "data": {
                "total": 1,
                "has_next_page": false,
                "page": 1,
                "limit": 20
            },
            "list": [{
                "level": "error",
                "message": "failed",
                "timestamp": 1_800_000_000,
                "trace_id": "trace"
            }]
        }))
        .unwrap();
        assert_eq!(real_time.data.expect("data").total, Some(1));
        assert_eq!(real_time.list[0].level.as_deref(), Some("error"));
        assert_eq!(real_time.list[0].trace_id.as_deref(), Some("trace"));
    }

    #[test]
    fn validates_production_operation_responses() {
        let domain: OperationDomainInfoResponse = serde_json::from_value(json!({
            "errcode": 0,
            "requestdomain": ["https://api.example.com"],
            "wsrequestdomain": ["wss://ws.example.com"],
            "request_id": "domain"
        }))
        .unwrap();
        domain.validate().unwrap();
        assert_eq!(domain.extra["request_id"], "domain");

        let feedback: OperationFeedbackResponse = serde_json::from_value(json!({
            "errcode": 0,
            "list": [{
                "record_id": 1,
                "content": "blank screen",
                "create_time": 1_783_036_800,
                "media_id": ["media-1"],
                "item_extension": true
            }],
            "total_num": 21
        }))
        .unwrap();
        feedback.validate().unwrap();
        assert_eq!(feedback.next_page(1, 20).unwrap(), Some(2));
        assert_eq!(feedback.list[0].extra["item_extension"], true);

        let gray: OperationGrayReleasePlanResponse = serde_json::from_value(json!({
            "errcode": 0,
            "gray_release_plan": {
                "status": 1,
                "create_timestamp": 1_783_036_800,
                "default_finish_timestamp": 1_783_123_200,
                "gray_percentage": 30
            }
        }))
        .unwrap();
        gray.validate().unwrap();
        assert_eq!(
            gray.gray_release_plan
                .as_ref()
                .and_then(OperationGrayReleasePlan::status_kind),
            Some(OperationGrayReleaseStatusKind::Running)
        );

        let detail: OperationJsErrDetailResponse = serde_json::from_value(json!({
            "errcode": 0,
            "success": true,
            "totalCount": 1,
            "data": [{
                "Count": 3,
                "sdkVersion": "3.7.0",
                "clientVersion": "8.0.0",
                "errorStackMd5": "d41d8cd98f00b204e9800998ecf8427e",
                "TimeStamp": "1783036800",
                "appVersion": "1.0.0",
                "errorMsgMd5": "d41d8cd98f00b204e9800998ecf8427e",
                "errorMsg": "TypeError",
                "errorStack": "stack",
                "DeviceModel": "iPhone"
            }]
        }))
        .unwrap();
        detail.validate().unwrap();
        assert_eq!(detail.data[0].count, Some(3));
        assert_eq!(detail.data[0].time, Some(1_783_036_800));
        assert_eq!(detail.data[0].device.as_deref(), Some("iPhone"));

        let performance: OperationPerformanceResponse = serde_json::from_value(json!({
            "errcode": 0,
            "default_time_data": "{\"list\":[{\"cost_time\":1533}]}",
            "compare_time_data": "{\"list\":[]}"
        }))
        .unwrap();
        performance.validate().unwrap();
        assert_eq!(
            performance.default_time_data_value().unwrap().unwrap()["list"][0]["cost_time"],
            1533
        );

        let versions: OperationVersionListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "cvlist": [{
                "type": 1,
                "client_version_list": ["8.0.60", "8.0.61"]
            }]
        }))
        .unwrap();
        versions.validate().unwrap();
        assert_eq!(
            versions.cvlist[0].version_type_kind(),
            Some(OperationVersionTypeKind::Client)
        );

        let logs: OperationRealTimeLogSearchResponse = serde_json::from_value(json!({
            "errcode": 0,
            "data": {
                "list": [{
                    "level": 6,
                    "platform": 1,
                    "libraryVersion": "3.7.0",
                    "clientVersion": "8.0.60",
                    "id": "openid",
                    "timestamp": 1783036800,
                    "msg": [{
                        "time": 1783036799,
                        "msg": ["checkout", {"order_id": "order-1"}],
                        "level": 2
                    }],
                    "url": "pages/order/detail",
                    "traceid": "trace-1",
                    "filterMsg": "checkout"
                }],
                "total": 21
            }
        }))
        .unwrap();
        logs.validate().unwrap();
        assert_eq!(logs.items()[0].level_bits().unwrap(), Some(6));
        assert_eq!(logs.items()[0].msg[0].msg[1]["order_id"], "order-1");
        assert_eq!(logs.next_start(0, 20).unwrap(), Some(1));

        let inconsistent: OperationRealTimeLogSearchResponse = serde_json::from_value(json!({
            "errcode": 0,
            "data": { "total": 0, "list": [{ "timestamp": 1 }] }
        }))
        .unwrap();
        assert!(inconsistent.validate().is_err());
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
        assert_eq!(task.task_info.unwrap().media_id, Some(456));

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
            "media_info_list": [{ "media_id": 1, "name": "Drama - EP01", "custom": "kept" }]
        }))
        .unwrap();
        assert_eq!(media_list.media_info_list[0].media_id, Some(1));
        assert_eq!(
            media_list.media_info_list[0].name.as_deref(),
            Some("Drama - EP01")
        );
        assert_eq!(media_list.media_info_list[0].extra["custom"], "kept");

        let media: MiniDramaMediaInfoResponse = serde_json::from_value(json!({
            "media_info": { "media_id": 1, "media_url": "https://example.com/video.mp4" }
        }))
        .unwrap();
        assert_eq!(
            media.media_info.unwrap().media_url.as_deref(),
            Some("https://example.com/video.mp4")
        );

        let link: MiniDramaMediaLinkResponse = serde_json::from_value(json!({
            "media_info": { "media_id": 1, "cover_url": "https://example.com/cover.jpg" }
        }))
        .unwrap();
        assert_eq!(
            link.media_info.unwrap().cover_url.as_deref(),
            Some("https://example.com/cover.jpg")
        );

        let drama: MiniDramaInfoResponse = serde_json::from_value(json!({
            "drama_info": { "drama_id": 100, "name": "Drama" }
        }))
        .unwrap();
        assert_eq!(drama.drama_info.unwrap().name.as_deref(), Some("Drama"));

        let drama_list: MiniDramaListResponse = serde_json::from_value(json!({
            "drama_info_list": [{ "drama_id": 100, "name": "Drama", "status": 2 }]
        }))
        .unwrap();
        assert_eq!(drama_list.drama_info_list[0].status, Some(2));

        let audit: MiniDramaAuditInfoResponse = serde_json::from_value(json!({
            "audit_detail": { "drama_id": 100, "audit_id": 9, "status": 1, "reason": "ok" }
        }))
        .unwrap();
        assert_eq!(audit.audit_detail.unwrap().reason.as_deref(), Some("ok"));

        let cdn: MiniDramaCdnUsageResponse = serde_json::from_value(json!({
            "data_interval": 3600,
            "item_list": [{ "time": 1800000000, "value": 1024 }]
        }))
        .unwrap();
        assert_eq!(cdn.data_interval, Some(3600));
        assert_eq!(cdn.item_list[0].value, Some(1024));

        let logs: MiniDramaCdnLogsResponse = serde_json::from_value(json!({
            "domestic_cdn_logs": [{ "url": "https://example.com/log.gz", "size": 2048 }],
            "total_count": 1
        }))
        .unwrap();
        assert_eq!(logs.total_count, Some(1));
        assert_eq!(
            logs.domestic_cdn_logs[0].url.as_deref(),
            Some("https://example.com/log.gz")
        );

        let packages: MiniDramaPackageListResponse = serde_json::from_value(json!({
            "package_list": [{ "package_id": 7, "drama_id": 100, "name": "Package" }],
            "total_count": 1
        }))
        .unwrap();
        assert_eq!(packages.package_list[0].package_id, Some(7));

        let auth: MiniDramaAuthorizationSearchResponse = serde_json::from_value(json!({
            "objects": [{ "drama_id": 100, "authorized_appid": "wxauth" }],
            "total_count": 1
        }))
        .unwrap();
        assert_eq!(auth.total_count, Some(1));
        assert_eq!(auth.objects[0].authorized_appid.as_deref(), Some("wxauth"));

        let auth_result: MiniDramaAuthorizationResponse = serde_json::from_value(json!({
            "result": [{ "drama_id": 100, "authorized_appid": "wxauth", "result_code": 0 }]
        }))
        .unwrap();
        assert_eq!(auth_result.result[0].result_code, Some(0));

        let account: MiniDramaAccountAuthorizationSearchResponse = serde_json::from_value(json!({
            "objects": [{ "authorized_appid": "wxauth", "authorized_time": 1800000000 }]
        }))
        .unwrap();
        assert_eq!(
            account.objects[0].authorized_appid.as_deref(),
            Some("wxauth")
        );
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
    fn validates_typed_immediate_delivery_workflows() {
        let identity = ImmediateDeliveryOrderIdentity::signed(
            "SFTC",
            "test_shop_order_id",
            "openid",
            "shop-no",
            "test_shop_id",
            "test_app_secrect",
        )
        .unwrap();
        assert_eq!(
            identity.delivery_sign,
            "a93d8d6bae9a9483c1b1d4e8670e7f6226ec94cb"
        );
        let cancellation_identity = ImmediateDeliveryOrderIdentity::signed_without_openid(
            "SFTC",
            "test_shop_order_id",
            "shop-no",
            "test_shop_id",
            "test_app_secrect",
        )
        .unwrap();
        assert!(cancellation_identity.openid.is_none());
        assert_eq!(
            cancellation_identity.delivery_sign,
            "a93d8d6bae9a9483c1b1d4e8670e7f6226ec94cb"
        );

        let request = ImmediateDeliveryAddOrderRequest {
            identity,
            sub_biz_id: Some("sub-merchant".to_string()),
            sender: None,
            receiver: ImmediateDeliveryContact {
                name: "Receiver".to_string(),
                city: "Shanghai".to_string(),
                address: "Pudong".to_string(),
                address_detail: "No. 1".to_string(),
                coordinate_type: Some(0),
                lng: 121.473_701,
                lat: 31.230_416,
                phone: "13800000000".to_string(),
            },
            cargo: ImmediateDeliveryCargo {
                goods_value: 20.0,
                goods_weight: 1.5,
                cargo_first_class: "Retail".to_string(),
                cargo_second_class: "Food".to_string(),
                goods_height: Some(10.0),
                goods_length: Some(20.0),
                goods_width: Some(15.0),
                goods_detail: Some(ImmediateDeliveryGoodsDetail {
                    goods: vec![ImmediateDeliveryGoods {
                        good_count: 2,
                        good_name: "Fruit".to_string(),
                        good_price: Some(10.0),
                        good_unit: Some("box".to_string()),
                    }],
                }),
                goods_pickup_info: Some("front desk".to_string()),
                goods_delivery_info: Some("call receiver".to_string()),
            },
            order_info: ImmediateDeliveryOrderInfo {
                order_type: Some(0),
                order_time: Some(1_800_000_000),
                ..Default::default()
            },
            shop: ImmediateDeliveryShopInfo {
                goods_count: 2,
                goods_name: "Fruit".to_string(),
                img_url: "https://example.com/fruit.png".to_string(),
                wxa_path: "pages/order/detail".to_string(),
                wxa_appid: None,
            },
        };
        request.validate().unwrap();
        let value = serde_json::to_value(&request).unwrap();
        assert_eq!(value["shopid"], "test_shop_id");
        assert_eq!(value["cargo"]["goods_detail"]["goods"][0]["good_count"], 2);
        assert!(value.get("app_secret").is_none());

        let mut invalid = request;
        invalid.receiver.lat = 91.0;
        assert!(invalid.validate().is_err());
        invalid.receiver.lat = 31.0;
        invalid.order_info.order_type = Some(1);
        assert!(invalid.validate().is_err());
        invalid.order_info.expected_finish_time = Some(1_900_000_000);
        invalid.cargo.goods_value = 10.001;
        assert!(invalid.validate().is_err());

        assert!(ImmediateDeliveryRequest::new(json!({})).validate().is_err());
        assert!(ImmediateDeliveryRequest::new(json!([])).validate().is_err());
    }

    #[test]
    fn validates_immediate_delivery_provider_responses() {
        let order: ImmediateDeliveryOrderResponse = serde_json::from_value(json!({
            "errcode": 0,
            "resultcode": "0",
            "fee": "9.00",
            "deliverfee": 10,
            "couponfee": "1.00",
            "tips": 0,
            "insurancfee": "0",
            "distance": 1200,
            "waybill_id": 123456789,
            "order_status": "101",
            "dispatch_duration": "300"
        }))
        .unwrap();
        order.ensure_success().unwrap();
        order.reconcile_fee().unwrap();
        assert_eq!(order.waybill_id.as_deref(), Some("123456789"));
        assert_eq!(
            order.order_status_kind(),
            Some(ImmediateDeliveryOrderStatusKind::WaitingForRiderAssignment)
        );

        let provider_error: ImmediateDeliveryStatusResponse = serde_json::from_value(json!({
            "errcode": 0,
            "resultcode": 42,
            "resultmsg": "provider rejected order"
        }))
        .unwrap();
        assert!(matches!(
            provider_error.ensure_success(),
            Err(WechatError::Api { code: 42, .. })
        ));

        let wechat_error: ImmediateDeliveryStatusResponse = serde_json::from_value(json!({
            "errcode": 930561,
            "errmsg": "invalid request",
            "resultcode": 0
        }))
        .unwrap();
        assert!(matches!(
            wechat_error.ensure_success(),
            Err(WechatError::Api { code: 930561, .. })
        ));
    }

    #[test]
    fn deserializes_immediate_delivery_responses() {
        let shops: ImmediateDeliveryBindAccountResponse = serde_json::from_value(json!({
            "errcode": 0,
            "request_id": "bind-account",
            "shop_list": [{
                "delivery_id": "delivery-id",
                "delivery_name": "Fast Delivery",
                "shopid": "shop-id",
                "shop_no": "shop-no",
                "shop_name": "Roze Shop",
                "shop_extra": "retained"
            }]
        }))
        .unwrap();
        assert_eq!(
            shops.shop_list[0].delivery_id.as_deref(),
            Some("delivery-id")
        );
        assert_eq!(shops.shop_list[0].shopid.as_deref(), Some("shop-id"));
        assert_eq!(shops.shop_list[0].shop_name.as_deref(), Some("Roze Shop"));
        assert_eq!(shops.shop_list[0].extra["shop_extra"], "retained");
        assert_eq!(shops.extra["request_id"], "bind-account");

        let delivery_list: ImmediateDeliveryDeliveryListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "request_id": "delivery-list",
            "list": [{
                "delivery_id": "delivery-id",
                "delivery_name": "Fast Delivery",
                "can_use_cash": true,
                "can_get_quota": false,
                "provider_extra": "retained"
            }]
        }))
        .unwrap();
        assert_eq!(
            delivery_list.list[0].delivery_name.as_deref(),
            Some("Fast Delivery")
        );
        assert_eq!(delivery_list.list[0].can_use_cash, Some(true));
        assert_eq!(delivery_list.list[0].extra["provider_extra"], "retained");
        assert_eq!(delivery_list.extra["request_id"], "delivery-list");

        let cancel: ImmediateDeliveryCancelOrderResponse = serde_json::from_value(json!({
            "errcode": 0,
            "deduct_fee": 5,
            "desc": "cancelled",
            "request_id": "cancel"
        }))
        .unwrap();
        assert_eq!(cancel.deduct_fee, Some(5));
        assert_eq!(cancel.desc.as_deref(), Some("cancelled"));
        assert_eq!(cancel.extra["request_id"], "cancel");

        let order: ImmediateDeliveryGetOrderResponse = serde_json::from_value(json!({
            "errcode": 0,
            "delivery_id": "delivery-id",
            "delivery_name": "Fast Delivery",
            "shopid": "shop-id",
            "shop_order_id": "order-id",
            "shop_no": "shop-no",
            "order_status": 101,
            "waybill_id": "waybill-id",
            "pickup_code": 2048,
            "finish_code": 1024,
            "rider_name": "Alex",
            "rider_phone": "13800000000",
            "rider_lng": 120.1,
            "rider_lat": 30.2,
            "reach_time": 300,
            "order_extra": "retained"
        }))
        .unwrap();
        assert_eq!(order.delivery_id.as_deref(), Some("delivery-id"));
        assert_eq!(order.shop_order_id.as_deref(), Some("order-id"));
        assert_eq!(order.order_status, Some(101));
        assert_eq!(
            order.order_status_kind(),
            Some(ImmediateDeliveryOrderStatusKind::WaitingForRiderAssignment)
        );
        assert_eq!(order.waybill_id.as_deref(), Some("waybill-id"));
        assert_eq!(order.pickup_code, Some(2048));
        assert_eq!(order.rider_lng, Some(120.1));
        assert_eq!(order.extra["order_extra"], "retained");

        let pre_add: ImmediateDeliveryPreAddOrderResponse = serde_json::from_value(json!({
            "errcode": 0,
            "fee": 10,
            "deliverfee": "10",
            "couponfee": "0",
            "tips": "0",
            "insurancfee": 0.0,
            "distance": 1000.0,
            "dispatch_duration": 300,
            "delivery_token": 1111111,
            "request_id": "pre-add"
        }))
        .unwrap();
        assert_eq!(pre_add.fee, Some(10));
        assert_eq!(pre_add.delivery_token, Some(1111111));
        assert_eq!(pre_add.extra["request_id"], "pre-add");

        let pre_cancel: ImmediateDeliveryPreCancelOrderResponse = serde_json::from_value(json!({
            "errcode": 0,
            "deduct_fee": 5,
            "desc": "fee",
            "request_id": "pre-cancel"
        }))
        .unwrap();
        assert_eq!(pre_cancel.deduct_fee, Some(5));
        assert_eq!(pre_cancel.extra["request_id"], "pre-cancel");

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
            "dispatch_duration": 300,
            "request_id": "reorder"
        }))
        .unwrap();
        assert_eq!(reorder.insurance_fee, Some(0.0));
        assert_eq!(reorder.waybill_id, Some(123456789));
        assert_eq!(reorder.pickup_code, Some(2048));
        assert_eq!(
            reorder.order_status_kind(),
            Some(ImmediateDeliveryOrderStatusKind::WaitingForRiderAssignment)
        );
        assert_eq!(reorder.extra["request_id"], "reorder");

        assert!(ImmediateDeliveryOrderStatusKind::Delivered.is_success_terminal());
        assert!(ImmediateDeliveryOrderStatusKind::ReturnedToMerchant.is_failure_terminal());
        assert_eq!(
            ImmediateDeliveryOrderStatusKind::from(999),
            ImmediateDeliveryOrderStatusKind::Other(999)
        );
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

        let batch_item = serde_json::to_value(ExpressBatchGetOrderItem {
            order_id: "order-id".to_string(),
            openid: "openid".to_string(),
            delivery_id: "delivery-id".to_string(),
            waybill_id: "waybill-id".to_string(),
        })
        .unwrap();
        assert_eq!(batch_item["order_id"], "order-id");
        assert_eq!(batch_item["waybill_id"], "waybill-id");

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
    fn serializes_typed_express_provider_and_order_requests() {
        let request = ExpressAddOrderRequest {
            order_id: "order-id".to_string(),
            openid: "openid".to_string(),
            delivery_id: "delivery-id".to_string(),
            biz_id: "biz-id".to_string(),
            custom_remark: Some("Leave at reception".to_string()),
            tagid: Some(10),
            add_source: 0,
            wx_appid: None,
            sender: ExpressAddress {
                name: Some("Sender".to_string()),
                tel: None,
                mobile: Some("13800000000".to_string()),
                company: Some("Roze".to_string()),
                post_code: Some("310000".to_string()),
                country: Some("China".to_string()),
                province: Some("Zhejiang".to_string()),
                city: Some("Hangzhou".to_string()),
                area: Some("Xihu".to_string()),
                address: Some("Sender street".to_string()),
            },
            receiver: ExpressAddress {
                name: Some("Receiver".to_string()),
                tel: Some("057100000000".to_string()),
                mobile: None,
                company: None,
                post_code: Some("310000".to_string()),
                country: Some("China".to_string()),
                province: Some("Zhejiang".to_string()),
                city: Some("Hangzhou".to_string()),
                area: Some("Xihu".to_string()),
                address: Some("Receiver street".to_string()),
            },
            shop: ExpressShop {
                wxa_path: Some("pages/orders/detail".to_string()),
                img_url: Some("https://example.com/goods.png".to_string()),
                goods_name: Some("Coffee".to_string()),
                goods_count: Some(2),
                detail_list: Vec::new(),
            },
            cargo: ExpressCargo {
                count: 2,
                weight: 1.5,
                space_x: 20.0,
                space_y: 15.0,
                space_z: 10.0,
                detail_list: vec![ExpressCargoDetail {
                    name: "Coffee".to_string(),
                    count: 2,
                }],
            },
            insured: ExpressInsured {
                use_insured: 1,
                insured_value: Some(10_000),
            },
            service: ExpressService {
                service_type: 0,
                service_name: "standard".to_string(),
            },
            expect_time: Some(1_800_000_000),
            take_mode: Some(0),
        };
        assert!(request.validate().is_ok());
        let preview_custom = request.clone();
        let value = serde_json::to_value(request).unwrap();
        assert_eq!(value["sender"]["mobile"], "13800000000");
        assert_eq!(value["cargo"]["detail_list"][0]["count"], 2);
        assert_eq!(value["insured"]["insured_value"], 10_000);

        let preview = ExpressPreviewTemplateRequest {
            waybill_id: "waybill-id".to_string(),
            waybill_template: "<xml>template</xml>".to_string(),
            waybill_data: r#"{"tracking_no":"waybill-id"}"#.to_string(),
            custom: preview_custom,
        };
        assert!(preview.validate().is_ok());
        assert_eq!(
            serde_json::to_value(preview).unwrap()["custom"]["order_id"],
            "order-id"
        );

        let business = ExpressUpdateBusinessRequest {
            shop_app_id: "wx-shop".to_string(),
            biz_id: "biz-id".to_string(),
            result_code: 1,
            result_msg: Some("credentials rejected".to_string()),
        };
        assert!(business.validate().is_ok());
        assert_eq!(
            serde_json::to_value(business).unwrap()["shop_app_id"],
            "wx-shop"
        );

        let path = ExpressUpdatePathRequest {
            token: "callback-token".to_string(),
            waybill_id: "waybill-id".to_string(),
            action_time: 1_800_000_000,
            action_type: 100001,
            action_msg: "picked up".to_string(),
        };
        assert!(path.validate().is_ok());
        assert_eq!(serde_json::to_value(path).unwrap()["action_type"], 100001);
    }

    #[test]
    fn deserializes_express_responses() {
        let add: ExpressAddOrderResponse = serde_json::from_value(json!({
            "errcode": 0,
            "delivery_resultcode": 0,
            "delivery_resultmsg": "ok",
            "order_id": "order-id",
            "waybill_id": "waybill-id",
            "request_id": "add-order",
            "waybill_data": [{
                "key": "tracking_no",
                "value": "waybill-id",
                "data_extra": "retained"
            }]
        }))
        .unwrap();
        assert_eq!(add.delivery_resultcode, Some(0));
        assert_eq!(add.order_id.as_deref(), Some("order-id"));
        assert_eq!(add.waybill_data[0].key.as_deref(), Some("tracking_no"));
        assert_eq!(add.waybill_data[0].value.as_deref(), Some("waybill-id"));
        assert_eq!(add.ensure_created().unwrap(), ("order-id", "waybill-id"));
        assert_eq!(add.extra["request_id"], "add-order");
        assert_eq!(add.waybill_data[0].extra["data_extra"], "retained");

        let batch: ExpressBatchOrderListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "request_id": "batch",
            "order_list": [{
                "order_id": "order-id",
                "waybill_id": "waybill-id",
                "delivery_id": "delivery-id",
                "summary_extra": "retained"
            }]
        }))
        .unwrap();
        assert_eq!(batch.order_list[0].order_id.as_deref(), Some("order-id"));
        assert_eq!(
            batch.order_list[0].waybill_id.as_deref(),
            Some("waybill-id")
        );
        assert_eq!(batch.extra["request_id"], "batch");
        assert_eq!(batch.order_list[0].extra["summary_extra"], "retained");
        assert!(batch.validate().is_ok());
        assert!(batch.find_order("order-id").is_some());

        let cancel: ExpressCancelOrderResponse = serde_json::from_value(json!({
            "errcode": 0,
            "delivery_resultcode": 0,
            "delivery_resultmsg": "",
            "request_id": "cancel"
        }))
        .unwrap();
        assert_eq!(cancel.delivery_resultcode, Some(0));
        assert!(cancel.ensure_cancelled().is_ok());
        assert_eq!(cancel.extra["request_id"], "cancel");

        let accounts: ExpressAccountListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "count": 1,
            "request_id": "accounts",
            "list": [{
                "biz_id": "biz-id",
                "delivery_id": "delivery-id",
                "delivery_name": "Fast Delivery",
                "account_name": "Account",
                "account_extra": "retained"
            }]
        }))
        .unwrap();
        assert_eq!(accounts.count, Some(1));
        assert_eq!(accounts.list[0].biz_id.as_deref(), Some("biz-id"));
        assert_eq!(accounts.list[0].delivery_id.as_deref(), Some("delivery-id"));
        assert_eq!(accounts.extra["request_id"], "accounts");
        assert_eq!(accounts.list[0].extra["account_extra"], "retained");
        assert!(accounts.validate().is_ok());
        assert!(accounts.find("biz-id", "delivery-id").is_some());

        let deliveries: ExpressDeliveryListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "count": 1,
            "request_id": "deliveries",
            "data": [{
                "delivery_id": "delivery-id",
                "delivery_name": "Fast Delivery",
                "can_use_cash": 1,
                "can_get_quota": 1,
                "delivery_extra": "retained"
            }]
        }))
        .unwrap();
        assert_eq!(
            deliveries.data[0].delivery_name.as_deref(),
            Some("Fast Delivery")
        );
        assert_eq!(deliveries.extra["request_id"], "deliveries");
        assert_eq!(deliveries.data[0].extra["delivery_extra"], "retained");
        assert!(deliveries.validate().is_ok());
        assert_eq!(deliveries.supporting_quota().len(), 1);

        let order: ExpressGetOrderResponse = serde_json::from_value(json!({
            "errcode": 0,
            "print_html": "<html></html>",
            "request_id": "get-order",
            "waybill_data": [{
                "key": "tracking_no",
                "value": "waybill-id",
                "data_extra": "retained"
            }],
            "delivery_id": "delivery-id",
            "waybill_id": "waybill-id",
            "order_id": "order-id",
            "order_status": 1
        }))
        .unwrap();
        assert_eq!(order.order_status, Some(1));
        assert_eq!(order.print_html.as_deref(), Some("<html></html>"));
        assert_eq!(order.waybill_data[0].key.as_deref(), Some("tracking_no"));
        assert_eq!(order.extra["request_id"], "get-order");
        assert_eq!(order.waybill_data[0].extra["data_extra"], "retained");
        assert!(order.validate().is_ok());
        assert!(order.matches("order-id", "waybill-id"));

        let path: ExpressGetPathResponse = serde_json::from_value(json!({
            "errcode": 0,
            "openid": "openid",
            "delivery_id": "delivery-id",
            "waybill_id": "waybill-id",
            "path_item_num": 1,
            "request_id": "path",
            "path_item_list": [{
                "action_time": 1_800_000_000,
                "action_type": 100001,
                "action_msg": "picked up",
                "operator": "courier"
            }]
        }))
        .unwrap();
        assert_eq!(path.path_item_num, Some(1));
        assert_eq!(path.path_item_list[0].action_type, Some(100001));
        assert_eq!(
            path.path_item_list[0].action_msg.as_deref(),
            Some("picked up")
        );
        assert_eq!(path.extra["request_id"], "path");
        assert_eq!(path.path_item_list[0].extra["operator"], "courier");
        assert!(path.validate().is_ok());
        assert_eq!(path.latest().unwrap().action_type, Some(100001));

        let printer: ExpressGetPrinterResponse = serde_json::from_value(json!({
            "errcode": 0,
            "count": "1",
            "request_id": "printer",
            "openid": ["openid"],
            "tagid_list": ["tag-a"]
        }))
        .unwrap();
        assert_eq!(printer.count, Some(1));
        assert_eq!(printer.openid[0], "openid");
        assert!(printer.validate().is_ok());
        assert_eq!(printer.extra["request_id"], "printer");

        let quota: ExpressGetQuotaResponse = serde_json::from_value(json!({
            "errcode": 0,
            "quota_num": "100",
            "request_id": "quota"
        }))
        .unwrap();
        assert_eq!(quota.quota_num, Some(100));
        assert_eq!(quota.available_quota().unwrap(), 100);
        assert!(quota.has_quota().unwrap());
        assert_eq!(quota.extra["request_id"], "quota");

        let contact: ExpressGetContactResponse = serde_json::from_value(json!({
            "errcode": 0,
            "waybill_id": "waybill-id",
            "request_id": "contact",
            "sender": [{
                "name": "sender",
                "mobile": "13800000000",
                "contact_extra": "sender"
            }],
            "receiver": [{
                "name": "receiver",
                "mobile": "13900000000",
                "address": "street",
                "contact_extra": "receiver"
            }]
        }))
        .unwrap();
        assert_eq!(contact.waybill_id.as_deref(), Some("waybill-id"));
        assert_eq!(contact.sender[0].mobile.as_deref(), Some("13800000000"));
        assert_eq!(contact.receiver[0].name.as_deref(), Some("receiver"));
        assert!(contact.validate().is_ok());
        assert_eq!(contact.extra["request_id"], "contact");
        assert_eq!(contact.sender[0].extra["contact_extra"], "sender");
        assert_eq!(contact.receiver[0].extra["contact_extra"], "receiver");

        let preview: ExpressPreviewTemplateResponse = serde_json::from_value(json!({
            "errcode": 0,
            "waybill_id": "waybill-id",
            "rendered_waybill_template": "template",
            "request_id": "preview"
        }))
        .unwrap();
        assert_eq!(preview.waybill_id.as_deref(), Some("waybill-id"));
        assert_eq!(
            preview.rendered_waybill_template.as_deref(),
            Some("template")
        );
        assert!(preview.validate().is_ok());
        assert_eq!(preview.extra["request_id"], "preview");
    }

    #[test]
    fn rejects_inconsistent_express_contracts() {
        assert!(ExpressRequest::new(Value::Null).validate().is_err());
        assert!(ExpressBindAccountRequest {
            action_type: "invalid".to_string(),
            biz_id: "biz".to_string(),
            delivery_id: "delivery".to_string(),
            password: String::new(),
        }
        .validate()
        .is_err());
        assert!(ExpressGetOrderRequest {
            order_id: "order".to_string(),
            openid: "openid".to_string(),
            delivery_id: "delivery".to_string(),
            waybill_id: "waybill".to_string(),
            print_type: 2,
        }
        .validate()
        .is_err());
        assert!(validate_express_batch_orders(&[
            ExpressBatchGetOrderItem {
                order_id: "order".to_string(),
                openid: "openid".to_string(),
                delivery_id: "delivery".to_string(),
                waybill_id: "waybill-1".to_string(),
            },
            ExpressBatchGetOrderItem {
                order_id: "order".to_string(),
                openid: "openid".to_string(),
                delivery_id: "delivery".to_string(),
                waybill_id: "waybill-2".to_string(),
            },
        ])
        .is_err());

        let carrier_error: ExpressAddOrderResponse = serde_json::from_value(json!({
            "delivery_resultcode": 10002,
            "delivery_resultmsg": "invalid account password"
        }))
        .unwrap();
        assert!(matches!(
            carrier_error.ensure_created(),
            Err(WechatError::Api { code: 10002, .. })
        ));

        let duplicate_batch: ExpressBatchOrderListResponse = serde_json::from_value(json!({
            "order_list": [
                {
                    "order_id": "order",
                    "waybill_id": "waybill-1",
                    "delivery_id": "delivery"
                },
                {
                    "order_id": "order",
                    "waybill_id": "waybill-2",
                    "delivery_id": "delivery"
                }
            ]
        }))
        .unwrap();
        assert!(duplicate_batch.validate().is_err());

        let unordered_path: ExpressGetPathResponse = serde_json::from_value(json!({
            "openid": "openid",
            "delivery_id": "delivery",
            "waybill_id": "waybill",
            "path_item_num": 2,
            "path_item_list": [
                {"action_time": 20, "action_type": 100002, "action_msg": "transit"},
                {"action_time": 10, "action_type": 100001, "action_msg": "picked up"}
            ]
        }))
        .unwrap();
        assert!(unordered_path.validate().is_err());

        let negative_quota: ExpressGetQuotaResponse =
            serde_json::from_value(json!({ "quota_num": "-1" })).unwrap();
        assert!(negative_quota.available_quota().is_err());

        let mismatched_printers: ExpressGetPrinterResponse =
            serde_json::from_value(json!({ "count": "2", "openid": ["openid"] })).unwrap();
        assert!(mismatched_printers.validate().is_err());
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
                "merchant_id": "mch",
                "sub_merchant_id": "sub-mch",
                "merchant_trade_no": "trade-no",
                "openid": "openid",
                "description": "goods",
                "order_state": 2,
                "order_state_desc": "paid",
                "pay_time": 1_800_000_000,
                "amount": {
                    "total": 100,
                    "payer_total": 100,
                    "currency": "CNY",
                    "payer_currency": "CNY",
                    "amount_extra": "retained"
                },
                "shipping": {
                    "logistics_type": 1,
                    "delivery_mode": 1,
                    "is_all_delivered": true,
                    "upload_time": "2026-07-16T10:00:00+08:00",
                    "shipping_extra": "retained",
                    "shipping_list": [{
                        "tracking_no": "tracking",
                        "express_company": "SF",
                        "item_desc": "goods",
                        "contact": {
                            "consignor_contact": "13800000000",
                            "receiver_contact": "13900000000"
                        },
                        "item_extra": "retained"
                    }]
                },
                "order_extra": "retained"
            },
            "request_id": "order"
        }))
        .unwrap();
        assert_eq!(order.errcode, Some(0));
        assert_eq!(order.extra["request_id"], "order");
        let order_detail = order.order.expect("order");
        assert_eq!(order_detail.transaction_id.as_deref(), Some("tx"));
        assert_eq!(order_detail.merchant_trade_no.as_deref(), Some("trade-no"));
        assert_eq!(
            order_detail.order_state_kind(),
            Some(WxaSecOrderStateKind::Shipped)
        );
        assert_eq!(order_detail.extra["order_extra"], "retained");
        let amount = order_detail.amount.expect("amount");
        assert_eq!(amount.total, Some(100));
        assert_eq!(amount.extra["amount_extra"], "retained");
        let shipping = order_detail.shipping.expect("shipping");
        assert_eq!(shipping.extra["shipping_extra"], "retained");
        assert_eq!(
            shipping.shipping_list[0].tracking_no.as_deref(),
            Some("tracking")
        );
        assert_eq!(shipping.shipping_list[0].extra["item_extra"], "retained");

        let list: WxaSecOrderListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "last_index": "cursor",
            "has_more": true,
            "request_id": "list",
            "order_list": [{
                "transaction_id": "tx",
                "order_state": 2,
                "amount": { "total": 100, "amount_extra": "retained" },
                "order_extra": "retained"
            }]
        }))
        .unwrap();
        assert_eq!(list.last_index.as_deref(), Some("cursor"));
        assert_eq!(list.has_more, Some(true));
        assert_eq!(list.extra["request_id"], "list");
        assert_eq!(list.order_list[0].transaction_id.as_deref(), Some("tx"));
        assert_eq!(list.order_list[0].order_state, Some(2));
        assert_eq!(
            list.order_list[0].order_state_kind(),
            Some(WxaSecOrderStateKind::Shipped)
        );
        assert_eq!(list.order_list[0].extra["order_extra"], "retained");
        assert_eq!(
            list.order_list[0].amount.as_ref().expect("amount").total,
            Some(100)
        );
        assert_eq!(
            list.order_list[0].amount.as_ref().expect("amount").extra["amount_extra"],
            "retained"
        );
        assert_eq!(
            WxaSecOrderStateKind::from(99),
            WxaSecOrderStateKind::Other(99)
        );

        let managed: WxaSecTradeManagedResponse = serde_json::from_value(json!({
            "errcode": 0,
            "is_trade_managed": true,
            "request_id": "managed"
        }))
        .unwrap();
        assert_eq!(managed.is_trade_managed, Some(true));
        assert_eq!(managed.extra["request_id"], "managed");

        let confirmation: WxaSecTradeManagementConfirmationResponse =
            serde_json::from_value(json!({
                "errcode": 0,
                "completed": true,
                "request_id": "confirmation"
            }))
            .unwrap();
        assert_eq!(confirmation.completed, Some(true));
        assert_eq!(confirmation.extra["request_id"], "confirmation");
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
            sub_anchor_wechat: Some("co-host-wechat".to_string()),
            creator_wechat: Some("creator-wechat".to_string()),
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
            is_feeds_public: Some(1),
        })
        .unwrap();

        assert_eq!(value["name"], "launch");
        assert_eq!(value["coverImg"], "media-cover");
        assert_eq!(value["startTime"], 1_800_000_000);
        assert_eq!(value["endTime"], 1_800_003_600);
        assert_eq!(value["anchorName"], "host");
        assert_eq!(value["anchorWechat"], "host-wechat");
        assert_eq!(value["subAnchorWechat"], "co-host-wechat");
        assert_eq!(value["createrWechat"], "creator-wechat");
        assert_eq!(value["shareImg"], "media-share");
        assert_eq!(value["type"], 1);
        assert_eq!(value["screenType"], 0);
        assert_eq!(value["closeLike"], 0);
        assert_eq!(value["closeGoods"], 0);
        assert_eq!(value["closeComment"], 0);
        assert_eq!(value["closeReplay"], 1);
        assert_eq!(value["isFeedsPublic"], 1);
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

        let create: MiniProgramCreateLiveRoomResponse = serde_json::from_value(json!({
            "errcode": 0,
            "roomId": 1000,
            "request_id": "create-live"
        }))
        .unwrap();
        assert_eq!(create.room_id, Some(1000));
        assert_eq!(create.require_room_id().unwrap(), 1000);
        assert_eq!(create.extra["request_id"], "create-live");

        let info: MiniProgramLiveInfoResponse = serde_json::from_value(json!({
            "errcode": 0,
            "total": 1,
            "request_id": "live-info",
            "room_info": [{
                "name": "launch",
                "roomid": 1000,
                "coverImg": "cover-url",
                "shareImg": "share-url",
                "liveStatus": 101,
                "startTime": 1_800_000_000,
                "endTime": 1_800_003_600,
                "anchorName": "host",
                "anchorWechat": "host-wechat",
                "goods": [{
                    "goodsId": 200,
                    "coverImg": "goods-cover",
                    "url": "pages/goods",
                    "name": "item",
                    "price": 100,
                    "price2": 200,
                    "priceType": 1,
                    "goods_extra": "retained"
                }],
                "room_extra": "retained"
            }]
        }))
        .unwrap();
        assert_eq!(info.total, Some(1));
        assert_eq!(info.extra["request_id"], "live-info");
        assert_eq!(info.room_info[0].roomid, Some(1000));
        assert_eq!(info.room_info[0].cover_img.as_deref(), Some("cover-url"));
        assert_eq!(
            info.room_info[0].live_status_kind(),
            Some(MiniProgramLiveStatusKind::Living)
        );
        assert_eq!(info.room_info[0].extra["room_extra"], "retained");
        assert_eq!(info.room_info[0].goods[0].goods_id, Some(200));
        assert_eq!(info.room_info[0].goods[0].price_type, Some(1));
        assert_eq!(
            info.room_info[0].goods[0].price_type_kind(),
            Some(MiniProgramLiveGoodsPriceType::Fixed)
        );
        assert_eq!(info.room_info[0].goods[0].extra["goods_extra"], "retained");
        assert!(info.validate().is_ok());
        assert!(info.find_room(1000).is_some());
        assert_eq!(info.next_start(0).unwrap(), None);

        let replay: MiniProgramLiveReplayResponse = serde_json::from_value(json!({
            "errcode": 0,
            "total": 1,
            "request_id": "replay",
            "live_replay": [{
                "create_time": "2026-07-16 10:00:00",
                "expire_time": "2026-08-16 10:00:00",
                "media_url": "https://example.com/replay.mp4",
                "duration": 3600
            }]
        }))
        .unwrap();
        assert_eq!(
            replay.live_replay[0].media_url.as_deref(),
            Some("https://example.com/replay.mp4")
        );
        assert_eq!(replay.extra["request_id"], "replay");
        assert_eq!(replay.live_replay[0].extra["duration"], 3600);
        assert!(replay.validate().is_ok());
        assert_eq!(replay.next_start(0).unwrap(), None);

        let goods: MiniProgramLiveGoodsWarehouseResponse = serde_json::from_value(json!({
            "request_id": "warehouse",
            "goods": [{
                "goodsId": 100,
                "name": "item",
                "coverImgUrl": "cover",
                "price": 100,
                "price2": 200,
                "priceType": 1,
                "auditStatus": 2,
                "audit_reason": "ok"
            }]
        }))
        .unwrap();
        assert_eq!(goods.goods[0].goods_id, Some(100));
        assert_eq!(goods.goods[0].cover_img_url.as_deref(), Some("cover"));
        assert_eq!(goods.goods[0].audit_status, Some(2));
        assert_eq!(
            goods.goods[0].price_type_kind(),
            Some(MiniProgramLiveGoodsPriceType::Fixed)
        );
        assert_eq!(
            goods.goods[0].audit_status_kind(),
            Some(MiniProgramLiveGoodsAuditStatusKind::Approved)
        );
        assert_eq!(goods.extra["request_id"], "warehouse");
        assert_eq!(goods.goods[0].extra["audit_reason"], "ok");
        assert!(goods.validate().is_ok());
        assert!(goods.find(100).unwrap().is_approved());
        assert_eq!(goods.approved().len(), 1);
        assert_eq!(
            MiniProgramLiveStatusKind::from(199),
            MiniProgramLiveStatusKind::Other(199)
        );
        assert_eq!(
            MiniProgramLiveGoodsPriceType::from(9),
            MiniProgramLiveGoodsPriceType::Other(9)
        );
        assert_eq!(
            MiniProgramLiveGoodsAuditStatusKind::from(9),
            MiniProgramLiveGoodsAuditStatusKind::Other(9)
        );
        assert!(MiniProgramLiveStatusKind::Ended.is_terminal());
        assert!(MiniProgramLiveStatusKind::Abnormal.needs_attention());

        let followers: MiniProgramLiveFollowersResponse = serde_json::from_value(json!({
            "followers": [{
                "openid": "openid",
                "nickname": "viewer",
                "headimg": "avatar",
                "subscribeTime": 1_800_000_000,
                "source": "share"
            }],
            "page_break": "10",
            "request_id": "followers"
        }))
        .unwrap();
        assert_eq!(followers.followers[0].openid.as_deref(), Some("openid"));
        assert_eq!(followers.followers[0].subscribe_time, Some(1_800_000_000));
        assert_eq!(followers.followers[0].extra["source"], "share");
        assert_eq!(followers.page_break.unwrap(), 10);
        assert!(followers.validate().is_ok());
        assert_eq!(followers.next_page_break().unwrap(), Some(10));
        assert_eq!(followers.extra["request_id"], "followers");
    }

    #[test]
    fn serializes_live_broadcast_lifecycle_requests() {
        let edit_room = MiniProgramLiveRoomEditRequest {
            id: 100,
            name: "Launch".to_string(),
            cover_img: "cover-media".to_string(),
            start_time: 1_800_000_000,
            end_time: 1_800_003_600,
            anchor_name: "Host".to_string(),
            anchor_wechat: "host-wechat".to_string(),
            share_img: "share-media".to_string(),
            close_like: 0,
            close_goods: 0,
            close_comment: 0,
            is_feeds_public: 1,
            close_replay: 0,
            close_share: 0,
            close_kf: 0,
            feeds_img: "feeds-media".to_string(),
        };
        assert!(edit_room.validate().is_ok());
        let value = serde_json::to_value(edit_room).unwrap();
        assert_eq!(value["id"], 100);
        assert_eq!(value["isFeedsPublic"], 1);
        assert!(value.get("type").is_none());
        assert!(value.get("screenType").is_none());

        let add_goods = MiniProgramLiveRoomAddGoodsRequest {
            room_id: 100,
            ids: vec![10, 20],
        };
        assert!(add_goods.validate().is_ok());
        let value = serde_json::to_value(add_goods).unwrap();
        assert_eq!(value["roomId"], 100);
        assert_eq!(value["ids"], json!([10, 20]));

        let assistants = MiniProgramLiveAssistantAddRequest {
            room_id: 100,
            users: vec![MiniProgramLiveAssistantUser {
                username: "assistant-wechat".to_string(),
                nickname: "Assistant".to_string(),
            }],
        };
        assert!(assistants.validate().is_ok());
        let value = serde_json::to_value(assistants).unwrap();
        assert_eq!(value["roomId"], 100);
        assert_eq!(value["users"][0]["username"], "assistant-wechat");

        let sub_anchor = MiniProgramLiveSubAnchorRequest {
            room_id: 100,
            username: "co-host".to_string(),
        };
        assert!(sub_anchor.validate().is_ok());
        assert_eq!(
            serde_json::to_value(sub_anchor).unwrap()["roomId"],
            json!(100)
        );

        let create_goods = MiniProgramLiveGoodsMutationRequest {
            goods_info: MiniProgramLiveGoodsMutation {
                goods_id: None,
                cover_img_url: Some("media-id".to_string()),
                name: Some("Coffee".to_string()),
                price_type: Some(1),
                price: Some(10.0),
                price2: Some(0.0),
                url: Some("pages/goods/coffee".to_string()),
                third_party_appid: None,
            },
        };
        assert!(create_goods.validate_create().is_ok());
        let value = serde_json::to_value(create_goods).unwrap();
        assert_eq!(value["goodsInfo"]["coverImgUrl"], "media-id");
        assert_eq!(value["goodsInfo"]["priceType"], 1);

        let update_goods = MiniProgramLiveGoodsMutationRequest {
            goods_info: MiniProgramLiveGoodsMutation {
                goods_id: Some(10),
                cover_img_url: None,
                name: Some("Coffee beans".to_string()),
                price_type: None,
                price: None,
                price2: None,
                url: None,
                third_party_appid: None,
            },
        };
        assert!(update_goods.validate_update().is_ok());
        let value = serde_json::to_value(update_goods).unwrap();
        assert_eq!(value["goodsInfo"]["goodsId"], 10);
        assert!(value["goodsInfo"].get("price").is_none());

        let goods_list = MiniProgramLiveGoodsListRequest {
            offset: 0,
            count: 20,
            status: Some(2),
        };
        assert!(goods_list.validate().is_ok());
        assert_eq!(
            goods_list.query(),
            vec![
                ("offset".to_string(), "0".to_string()),
                ("count".to_string(), "20".to_string()),
                ("status".to_string(), "2".to_string())
            ]
        );

        let roles = MiniProgramLiveRoleListRequest {
            role: 2,
            offset: 0,
            limit: 20,
            keyword: Some("host".to_string()),
        };
        assert!(roles.validate().is_ok());
        let value = serde_json::to_value(roles).unwrap();
        assert_eq!(value["role"], 2);
        assert_eq!(value["keyword"], "host");
    }

    #[test]
    fn validates_live_broadcast_lifecycle_responses() {
        let push: MiniProgramLivePushUrlResponse =
            serde_json::from_value(json!({ "pushAddr": "rtmp://push.example.com/live" })).unwrap();
        assert_eq!(
            push.require_push_address().unwrap(),
            "rtmp://push.example.com/live"
        );

        let shared: MiniProgramLiveSharedCodeResponse = serde_json::from_value(json!({
            "cdnUrl": "https://example.com/code.png",
            "pagePath": "plugin-private://wx2b03c6e691cd7370/pages/live-player-plugin",
            "posterUrl": "https://example.com/poster.png"
        }))
        .unwrap();
        assert!(shared.validate().is_ok());

        let assistants: MiniProgramLiveAssistantListResponse = serde_json::from_value(json!({
            "count": 1,
            "maxCount": 10,
            "list": [{
                "timestamp": 1_800_000_000,
                "openid": "assistant-openid",
                "nickname": "Assistant"
            }]
        }))
        .unwrap();
        assert!(assistants.validate().is_ok());

        let sub_anchor: MiniProgramLiveSubAnchorResponse =
            serde_json::from_value(json!({ "username": "co-host" })).unwrap();
        assert_eq!(sub_anchor.require_username().unwrap(), "co-host");

        let video: MiniProgramLiveGoodsVideoResponse = serde_json::from_value(json!({
            "url": "https://example.com/goods.mp4"
        }))
        .unwrap();
        assert_eq!(
            video.require_url().unwrap(),
            "https://example.com/goods.mp4"
        );

        let added: MiniProgramLiveGoodsMutationResponse =
            serde_json::from_value(json!({ "goodsId": "10", "auditId": 20 })).unwrap();
        assert_eq!(added.require_ids().unwrap(), (10, 20));

        let audited: MiniProgramLiveGoodsAuditResponse =
            serde_json::from_value(json!({ "auditId": 21 })).unwrap();
        assert_eq!(audited.require_audit_id().unwrap(), 21);

        let goods: MiniProgramLiveGoodsListResponse = serde_json::from_value(json!({
            "total": 2,
            "goods": [{
                "goodsId": 10,
                "name": "Coffee",
                "priceType": 1,
                "price": 10,
                "auditStatus": 2
            }]
        }))
        .unwrap();
        assert!(goods.validate().is_ok());
        assert_eq!(goods.next_offset(0).unwrap(), Some(1));

        let roles: MiniProgramLiveRoleListResponse = serde_json::from_value(json!({
            "total": 1,
            "list": [{ "username": "host", "role": 2 }]
        }))
        .unwrap();
        assert!(roles.validate().is_ok());
    }

    #[test]
    fn rejects_inconsistent_live_broadcast_contracts() {
        assert!(LiveInfoRequest {
            start: -1,
            limit: 20
        }
        .validate()
        .is_err());
        assert!(MiniProgramLiveRoomAddGoodsRequest {
            room_id: 1,
            ids: vec![10, 10],
        }
        .validate()
        .is_err());
        assert!(MiniProgramLiveAssistantAddRequest {
            room_id: 1,
            users: Vec::new(),
        }
        .validate()
        .is_err());
        assert!(MiniProgramLiveGoodsMutationRequest {
            goods_info: MiniProgramLiveGoodsMutation {
                goods_id: None,
                cover_img_url: None,
                name: None,
                price_type: Some(9),
                price: Some(f64::NAN),
                price2: None,
                url: None,
                third_party_appid: None,
            },
        }
        .validate_create()
        .is_err());

        let duplicate_rooms: MiniProgramLiveInfoResponse = serde_json::from_value(json!({
            "total": 2,
            "room_info": [{ "roomid": 1 }, { "roomid": 1 }]
        }))
        .unwrap();
        assert!(duplicate_rooms.validate().is_err());

        let duplicate_followers: MiniProgramLiveFollowersResponse = serde_json::from_value(json!({
            "followers": [{ "openid": "viewer" }, { "openid": "viewer" }]
        }))
        .unwrap();
        assert!(duplicate_followers.validate().is_err());

        let stalled_goods: MiniProgramLiveGoodsListResponse =
            serde_json::from_value(json!({ "total": 2, "goods": [] })).unwrap();
        assert!(stalled_goods.next_offset(0).is_err());

        let api_error: MiniProgramLiveGoodsAuditResponse =
            serde_json::from_value(json!({ "errcode": 1, "errmsg": "failed" })).unwrap();
        assert!(matches!(
            api_error.require_audit_id(),
            Err(WechatError::Api { .. })
        ));
    }
}
