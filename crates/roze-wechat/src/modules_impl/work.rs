use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    config::Platform,
    crypto,
    error::Result,
    modules::{DomainModule, PlatformClient},
    Client,
};

#[derive(Debug, Clone)]
pub struct Work {
    inner: PlatformClient,
}

impl Work {
    pub fn new(client: Client, platform: Platform) -> Self {
        Self {
            inner: PlatformClient::new(client, platform),
        }
    }

    pub fn agent(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "work.agent")
    }

    pub fn base(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "work.base")
    }

    pub async fn callback_ip(&self, access_token: impl Into<String>) -> Result<WorkIpListResponse> {
        self.inner
            .get("cgi-bin/getcallbackip", Some(access_token.into()))
            .await
    }

    pub async fn api_domain_ip(
        &self,
        access_token: impl Into<String>,
    ) -> Result<WorkIpListResponse> {
        self.inner
            .get("cgi-bin/get_api_domain_ip", Some(access_token.into()))
            .await
    }

    pub async fn list_agents(&self, access_token: impl Into<String>) -> Result<Value> {
        self.inner
            .get("cgi-bin/agent/list", Some(access_token.into()))
            .await
    }

    pub async fn get_agent(&self, access_token: impl Into<String>, agent_id: i64) -> Result<Value> {
        self.inner
            .get_with_query(
                "cgi-bin/agent/get",
                Some(access_token.into()),
                vec![("agentid".to_string(), agent_id.to_string())],
            )
            .await
    }

    pub async fn set_agent(
        &self,
        access_token: impl Into<String>,
        request: AgentUpdateRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post("cgi-bin/agent/set", Some(access_token.into()), request)
            .await
    }

    pub fn contact(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "work.contact")
    }

    pub async fn create_department(
        &self,
        access_token: impl Into<String>,
        request: DepartmentRequest,
    ) -> Result<DepartmentCreateResponse> {
        self.inner
            .post(
                "cgi-bin/department/create",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn update_department(
        &self,
        access_token: impl Into<String>,
        request: DepartmentRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/department/update",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn delete_department(
        &self,
        access_token: impl Into<String>,
        id: i64,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .get_with_query(
                "cgi-bin/department/delete",
                Some(access_token.into()),
                vec![("id".to_string(), id.to_string())],
            )
            .await
    }

    pub async fn list_departments(
        &self,
        access_token: impl Into<String>,
        id: Option<i64>,
    ) -> Result<Value> {
        let query = id
            .map(|id| vec![("id".to_string(), id.to_string())])
            .unwrap_or_default();
        self.inner
            .get_with_query("cgi-bin/department/list", Some(access_token.into()), query)
            .await
    }

    pub async fn get_user(
        &self,
        access_token: impl Into<String>,
        user_id: impl Into<String>,
    ) -> Result<Value> {
        self.inner
            .get_with_query(
                "cgi-bin/user/get",
                Some(access_token.into()),
                vec![("userid".to_string(), user_id.into())],
            )
            .await
    }

    pub async fn list_department_users(
        &self,
        access_token: impl Into<String>,
        department_id: i64,
        fetch_child: bool,
    ) -> Result<Value> {
        self.inner
            .get_with_query(
                "cgi-bin/user/simplelist",
                Some(access_token.into()),
                vec![
                    ("department_id".to_string(), department_id.to_string()),
                    (
                        "fetch_child".to_string(),
                        if fetch_child { "1" } else { "0" }.to_string(),
                    ),
                ],
            )
            .await
    }

    pub async fn list_detailed_department_users(
        &self,
        access_token: impl Into<String>,
        department_id: i64,
        fetch_child: bool,
    ) -> Result<Value> {
        self.inner
            .get_with_query(
                "cgi-bin/user/list",
                Some(access_token.into()),
                vec![
                    ("department_id".to_string(), department_id.to_string()),
                    (
                        "fetch_child".to_string(),
                        if fetch_child { "1" } else { "0" }.to_string(),
                    ),
                ],
            )
            .await
    }

    pub async fn user_id_to_openid(
        &self,
        access_token: impl Into<String>,
        user_id: impl Into<String>,
    ) -> Result<UserIdToOpenIdResponse> {
        self.convert_user_id_to_openid(
            access_token,
            UserIdToOpenIdRequest {
                userid: user_id.into(),
            },
        )
        .await
    }

    pub async fn convert_user_id_to_openid(
        &self,
        access_token: impl Into<String>,
        request: UserIdToOpenIdRequest,
    ) -> Result<UserIdToOpenIdResponse> {
        self.inner
            .post(
                "cgi-bin/user/convert_to_openid",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn openid_to_user_id(
        &self,
        access_token: impl Into<String>,
        openid: impl Into<String>,
    ) -> Result<OpenIdToUserIdResponse> {
        self.convert_openid_to_user_id(
            access_token,
            OpenIdToUserIdRequest {
                openid: openid.into(),
            },
        )
        .await
    }

    pub async fn convert_openid_to_user_id(
        &self,
        access_token: impl Into<String>,
        request: OpenIdToUserIdRequest,
    ) -> Result<OpenIdToUserIdResponse> {
        self.inner
            .post(
                "cgi-bin/user/convert_to_userid",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub fn corpgroup(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "work.corpgroup")
    }

    pub async fn corpgroup_app_share_info(
        &self,
        access_token: impl Into<String>,
        agent_id: i64,
    ) -> Result<WorkCorpGroupAppShareInfoResponse> {
        self.inner
            .post_json_with_access_token_query(
                "cgi-bin/corpgroup/corp/list_app_share_info",
                Some(access_token.into()),
                vec![("agentid".to_string(), agent_id.to_string())],
                json!({}),
                Vec::new(),
            )
            .await
    }

    pub async fn corpgroup_token(
        &self,
        access_token: impl Into<String>,
        corp_id: impl Into<String>,
        agent_id: impl Into<String>,
    ) -> Result<WorkCorpGroupTokenResponse> {
        self.inner
            .post(
                "cgi-bin/corpgroup/corp/gettoken",
                Some(access_token.into()),
                json!({ "corpid": corp_id.into(), "agentid": agent_id.into() }),
            )
            .await
    }

    pub async fn corpgroup_miniprogram_transfer_session(
        &self,
        access_token: impl Into<String>,
        user_id: impl Into<String>,
        session_key: impl Into<String>,
    ) -> Result<WorkCorpGroupTransferSessionResponse> {
        self.inner
            .post(
                "cgi-bin/corpgroup/miniprogram/transfer_session",
                Some(access_token.into()),
                json!({ "userid": user_id.into(), "session_key": session_key.into() }),
            )
            .await
    }

    pub fn mini_program(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "work.mini_program")
    }

    pub async fn mini_program_code_to_session(
        &self,
        access_token: impl Into<String>,
        code: impl Into<String>,
    ) -> Result<WorkMiniProgramSessionResponse> {
        self.inner
            .get_with_query(
                "cgi-bin/miniprogram/jscode2session",
                Some(access_token.into()),
                vec![("js_code".to_string(), code.into())],
            )
            .await
    }

    pub fn id_convert(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "work.id_convert")
    }

    pub async fn union_id_to_external_user_id(
        &self,
        access_token: impl Into<String>,
        request: WorkUnionIdToExternalUserIdRequest,
    ) -> Result<WorkUnionIdToExternalUserIdResponse> {
        self.inner
            .post(
                "cgi-bin/idconvert/unionid_to_external_userid",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn external_user_id_to_pending_id(
        &self,
        access_token: impl Into<String>,
        request: WorkExternalUserIdToPendingIdRequest,
    ) -> Result<WorkExternalUserIdToPendingIdResponse> {
        self.inner
            .post(
                "cgi-bin/idconvert/batch/external_userid_to_pending_id",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn batch_user_id_to_open_user_id(
        &self,
        access_token: impl Into<String>,
        user_id_list: Vec<String>,
    ) -> Result<WorkUserIdToOpenUserIdResponse> {
        self.inner
            .post(
                "cgi-bin/batch/userid_to_openuserid",
                Some(access_token.into()),
                json!({ "userid_list": user_id_list }),
            )
            .await
    }

    pub async fn open_user_id_to_user_id(
        &self,
        access_token: impl Into<String>,
        request: WorkOpenUserIdToUserIdRequest,
    ) -> Result<WorkOpenUserIdToUserIdResponse> {
        self.inner
            .post(
                "cgi-bin/batch/openuserid_to_userid",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn external_tag_id_to_open_external_tag_id(
        &self,
        access_token: impl Into<String>,
        external_tag_id_list: Vec<String>,
    ) -> Result<WorkExternalTagIdToOpenExternalTagIdResponse> {
        self.inner
            .post(
                "cgi-bin/idconvert/external_tagid",
                Some(access_token.into()),
                json!({ "external_tagid_list": external_tag_id_list }),
            )
            .await
    }

    pub async fn from_service_external_user_id(
        &self,
        access_token: impl Into<String>,
        request: WorkFromServiceExternalUserIdRequest,
    ) -> Result<WorkFromServiceExternalUserIdResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/from_service_external_userid",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub fn invoice(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "work.invoice")
    }

    pub async fn get_invoice_info(
        &self,
        access_token: impl Into<String>,
        request: WorkInvoiceCardRequest,
    ) -> Result<WorkInvoiceInfoResponse> {
        self.inner
            .post(
                "cgi-bin/card/invoice/reimburse/getinvoiceinfo",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_invoice_info_batch(
        &self,
        access_token: impl Into<String>,
        invoice_list: Vec<WorkInvoiceCardRequest>,
    ) -> Result<WorkInvoiceInfoBatchResponse> {
        self.inner
            .post(
                "cgi-bin/card/invoice/reimburse/getinvoiceinfobatch",
                Some(access_token.into()),
                json!({ "item_list": invoice_list }),
            )
            .await
    }

    pub async fn update_invoice_status(
        &self,
        access_token: impl Into<String>,
        request: WorkInvoiceStatusRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/card/invoice/reimburse/updateinvoicestatus",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn update_invoice_status_batch(
        &self,
        access_token: impl Into<String>,
        request: WorkInvoiceStatusBatchRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/card/invoice/reimburse/updatestatusbatch",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub fn external_pay(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "work.external_pay")
    }

    pub async fn add_external_pay_merchant(
        &self,
        access_token: impl Into<String>,
        mch_id: impl Into<String>,
        merchant_name: impl Into<String>,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/externalpay/addmerchant",
                Some(access_token.into()),
                json!({ "mch_id": mch_id.into(), "merchant_name": merchant_name.into() }),
            )
            .await
    }

    pub async fn get_external_pay_merchant(
        &self,
        access_token: impl Into<String>,
        mch_id: impl Into<String>,
    ) -> Result<WorkExternalPayMerchantResponse> {
        self.inner
            .post(
                "cgi-bin/externalpay/getmerchant",
                Some(access_token.into()),
                json!({ "mch_id": mch_id.into() }),
            )
            .await
    }

    pub async fn delete_external_pay_merchant(
        &self,
        access_token: impl Into<String>,
        mch_id: impl Into<String>,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/externalpay/delmerchant",
                Some(access_token.into()),
                json!({ "mch_id": mch_id.into() }),
            )
            .await
    }

    pub async fn set_external_pay_merchant_use_scope(
        &self,
        access_token: impl Into<String>,
        request: WorkExternalPaySetMerchantUseScopeRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/externalpay/set_mch_use_scope",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_external_pay_bill_list(
        &self,
        access_token: impl Into<String>,
        request: WorkExternalPayBillListRequest,
    ) -> Result<WorkExternalPayBillListResponse> {
        self.inner
            .post(
                "cgi-bin/externalpay/get_bill_list",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub fn external_contact(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "work.external_contact")
    }

    pub async fn list_external_contacts(
        &self,
        access_token: impl Into<String>,
        user_id: impl Into<String>,
    ) -> Result<Value> {
        self.inner
            .get_with_query(
                "cgi-bin/externalcontact/list",
                Some(access_token.into()),
                vec![("userid".to_string(), user_id.into())],
            )
            .await
    }

    pub async fn get_external_contact(
        &self,
        access_token: impl Into<String>,
        external_userid: impl Into<String>,
        cursor: Option<String>,
    ) -> Result<Value> {
        let mut query = vec![("external_userid".to_string(), external_userid.into())];
        if let Some(cursor) = cursor {
            query.push(("cursor".to_string(), cursor));
        }
        self.inner
            .get_with_query(
                "cgi-bin/externalcontact/get",
                Some(access_token.into()),
                query,
            )
            .await
    }

    pub async fn batch_get_external_contacts(
        &self,
        access_token: impl Into<String>,
        request: ExternalContactBatchGetRequest,
    ) -> Result<Value> {
        self.inner
            .post(
                "cgi-bin/externalcontact/batch/get_by_user",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn add_contact_way(
        &self,
        access_token: impl Into<String>,
        request: ContactWayRequest,
    ) -> Result<Value> {
        self.inner
            .post(
                "cgi-bin/externalcontact/add_contact_way",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_contact_way(
        &self,
        access_token: impl Into<String>,
        config_id: impl Into<String>,
    ) -> Result<Value> {
        self.inner
            .post(
                "cgi-bin/externalcontact/get_contact_way",
                Some(access_token.into()),
                json!({ "config_id": config_id.into() }),
            )
            .await
    }

    pub fn group_robot(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "work.group_robot")
    }

    pub fn group_robot_text(
        content: impl Into<String>,
        mentioned_list: Vec<String>,
    ) -> GroupRobotMessage {
        GroupRobotMessage {
            msgtype: "text".to_string(),
            text: Some(json!({
                "content": content.into(),
                "mentioned_list": mentioned_list,
            })),
            markdown: None,
            image: None,
            news: None,
            file: None,
            template_card: None,
        }
    }

    pub fn group_robot_markdown(content: impl Into<String>) -> GroupRobotMessage {
        GroupRobotMessage {
            msgtype: "markdown".to_string(),
            text: None,
            markdown: Some(json!({ "content": content.into() })),
            image: None,
            news: None,
            file: None,
            template_card: None,
        }
    }

    pub fn jssdk(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "work.jssdk")
    }

    pub async fn jsapi_ticket(
        &self,
        access_token: impl Into<String>,
    ) -> Result<WorkTicketResponse> {
        self.inner
            .get("cgi-bin/get_jsapi_ticket", Some(access_token.into()))
            .await
    }

    pub async fn agent_jsapi_ticket(
        &self,
        access_token: impl Into<String>,
    ) -> Result<WorkTicketResponse> {
        self.inner
            .get_with_query(
                "cgi-bin/ticket/get",
                Some(access_token.into()),
                vec![("type".to_string(), "agent_config".to_string())],
            )
            .await
    }

    pub fn build_jsapi_config(
        corp_id: impl Into<String>,
        jsapi_ticket: impl AsRef<str>,
        url: impl AsRef<str>,
        js_api_list: Vec<String>,
    ) -> WorkJsapiConfig {
        let nonce_str = crypto::nonce_string(16);
        let timestamp = chrono::Utc::now().timestamp();
        let plain = format!(
            "jsapi_ticket={}&noncestr={}&timestamp={}&url={}",
            jsapi_ticket.as_ref(),
            nonce_str,
            timestamp,
            url.as_ref()
        );
        let signature = crypto::sha1_signature(&[plain.as_str()]);

        WorkJsapiConfig {
            corp_id: corp_id.into(),
            timestamp,
            nonce_str,
            signature,
            js_api_list,
        }
    }

    pub fn media(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "work.media")
    }

    pub async fn get_media(
        &self,
        access_token: impl Into<String>,
        media_id: impl Into<String>,
    ) -> Result<bytes::Bytes> {
        self.inner
            .get_bytes(
                "cgi-bin/media/get",
                Some(access_token.into()),
                vec![("media_id".to_string(), media_id.into())],
            )
            .await
    }

    pub async fn get_jssdk_media(
        &self,
        access_token: impl Into<String>,
        media_id: impl Into<String>,
    ) -> Result<bytes::Bytes> {
        self.inner
            .get_bytes(
                "cgi-bin/media/get/jssdk",
                Some(access_token.into()),
                vec![("media_id".to_string(), media_id.into())],
            )
            .await
    }

    pub async fn upload_work_image_from_bytes(
        &self,
        access_token: impl Into<String>,
        file_name: impl Into<String>,
        data: Vec<u8>,
    ) -> Result<WorkUploadImageResponse> {
        let form = reqwest::multipart::Form::new().part(
            "media",
            reqwest::multipart::Part::bytes(data).file_name(file_name.into()),
        );
        self.inner
            .post_multipart(
                "cgi-bin/media/uploadimg",
                Some(access_token.into()),
                Vec::new(),
                form,
            )
            .await
    }

    pub async fn upload_temp_media_from_bytes(
        &self,
        access_token: impl Into<String>,
        media_type: impl Into<String>,
        file_name: impl Into<String>,
        data: Vec<u8>,
    ) -> Result<WorkUploadMediaResponse> {
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

    pub async fn upload_attachment_from_bytes(
        &self,
        access_token: impl Into<String>,
        media_type: impl Into<String>,
        attachment_type: impl Into<String>,
        file_name: impl Into<String>,
        data: Vec<u8>,
    ) -> Result<WorkUploadMediaResponse> {
        let form = reqwest::multipart::Form::new().part(
            "media",
            reqwest::multipart::Part::bytes(data).file_name(file_name.into()),
        );
        self.inner
            .post_multipart(
                "cgi-bin/media/upload_attachment",
                Some(access_token.into()),
                vec![
                    ("media_type".to_string(), media_type.into()),
                    ("attachment_type".to_string(), attachment_type.into()),
                ],
                form,
            )
            .await
    }

    pub fn message(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "work.message")
    }

    pub async fn send_message(
        &self,
        access_token: impl Into<String>,
        request: WorkMessage,
    ) -> Result<MessageSendResponse> {
        self.inner
            .post("cgi-bin/message/send", Some(access_token.into()), request)
            .await
    }

    pub async fn send_text_message(
        &self,
        access_token: impl Into<String>,
        agent_id: i64,
        to_user: impl Into<String>,
        content: impl Into<String>,
    ) -> Result<MessageSendResponse> {
        self.send_message(
            access_token,
            WorkMessage {
                touser: Some(to_user.into()),
                toparty: None,
                totag: None,
                msgtype: "text".to_string(),
                agentid: agent_id,
                text: Some(json!({ "content": content.into() })),
                markdown: None,
                textcard: None,
                safe: Some(0),
                enable_id_trans: None,
                enable_duplicate_check: None,
                duplicate_check_interval: None,
                extra: Value::Null,
            },
        )
        .await
    }

    pub async fn recall_message(
        &self,
        access_token: impl Into<String>,
        msg_id: impl Into<String>,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/message/recall",
                Some(access_token.into()),
                json!({ "msgid": msg_id.into() }),
            )
            .await
    }

    pub fn menu(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "work.menu")
    }

    pub async fn get_menu(
        &self,
        access_token: impl Into<String>,
        agent_id: i64,
    ) -> Result<WorkMenuResponse> {
        self.inner
            .get_with_query(
                "cgi-bin/menu/get",
                Some(access_token.into()),
                vec![("agentid".to_string(), agent_id.to_string())],
            )
            .await
    }

    pub async fn create_menu(
        &self,
        access_token: impl Into<String>,
        agent_id: i64,
        request: WorkMenuRequest,
    ) -> Result<WorkMenuCreateResponse> {
        self.inner
            .post_json_with_access_token_query(
                "cgi-bin/menu/create",
                Some(access_token.into()),
                vec![("agentid".to_string(), agent_id.to_string())],
                serde_json::to_value(request).expect("work menu request serializes"),
                Vec::new(),
            )
            .await
    }

    pub async fn delete_menu(
        &self,
        access_token: impl Into<String>,
        agent_id: i64,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .get_with_query(
                "cgi-bin/menu/delete",
                Some(access_token.into()),
                vec![("agentid".to_string(), agent_id.to_string())],
            )
            .await
    }

    pub fn msg_audit(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "work.msg_audit")
    }

    pub async fn msg_audit_permit_users(
        &self,
        access_token: impl Into<String>,
        kind: Option<i64>,
    ) -> Result<Value> {
        let mut query = Vec::new();
        if let Some(kind) = kind {
            query.push(("type".to_string(), kind.to_string()));
        }
        self.inner
            .get_with_query(
                "cgi-bin/msgaudit/get_permit_user_list",
                Some(access_token.into()),
                query,
            )
            .await
    }

    pub async fn msg_audit_chat_data(
        &self,
        access_token: impl Into<String>,
        request: MsgAuditChatDataRequest,
    ) -> Result<Value> {
        self.inner
            .post(
                "cgi-bin/msgaudit/get_chatdata",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn msg_audit_room(
        &self,
        access_token: impl Into<String>,
        room_id: impl Into<String>,
    ) -> Result<Value> {
        self.inner
            .post(
                "cgi-bin/msgaudit/groupchat/get",
                Some(access_token.into()),
                json!({ "roomid": room_id.into() }),
            )
            .await
    }

    pub fn oa(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "work.oa")
    }

    pub fn oa_calendar(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "work.oa.calendar")
    }

    pub fn oa_approval(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "work.oa.approval")
    }

    pub async fn create_approval_template(
        &self,
        access_token: impl Into<String>,
        request: WorkApprovalCreateTemplateRequest,
    ) -> Result<WorkApprovalCreateTemplateResponse> {
        self.inner
            .post(
                "cgi-bin/oa/approval/create_template",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn update_approval_template(
        &self,
        access_token: impl Into<String>,
        request: WorkApprovalUpdateTemplateRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/oa/approval/update_template",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn add_calendar(
        &self,
        access_token: impl Into<String>,
        request: WorkCalendarAddRequest,
    ) -> Result<WorkCalendarAddResponse> {
        self.inner
            .post(
                "cgi-bin/oa/calendar/add",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn update_calendar(
        &self,
        access_token: impl Into<String>,
        calendar: Value,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/oa/calendar/update",
                Some(access_token.into()),
                json!({ "calendar": calendar }),
            )
            .await
    }

    pub async fn get_calendar(
        &self,
        access_token: impl Into<String>,
        cal_id_list: Vec<String>,
    ) -> Result<WorkCalendarGetResponse> {
        self.inner
            .post(
                "cgi-bin/oa/calendar/get",
                Some(access_token.into()),
                json!({ "cal_id_list": cal_id_list }),
            )
            .await
    }

    pub async fn delete_calendar(
        &self,
        access_token: impl Into<String>,
        cal_id: impl Into<String>,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/oa/calendar/del",
                Some(access_token.into()),
                json!({ "cal_id": cal_id.into() }),
            )
            .await
    }

    pub fn oa_dial(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "work.oa.dial")
    }

    pub async fn get_dial_record(
        &self,
        access_token: impl Into<String>,
        request: WorkDialRecordRequest,
    ) -> Result<WorkDialRecordResponse> {
        self.inner
            .post(
                "cgi-bin/dial/get_dial_record",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub fn oa_journal(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "work.oa.journal")
    }

    pub async fn get_journal_record_list(
        &self,
        access_token: impl Into<String>,
        request: WorkJournalRecordListRequest,
    ) -> Result<WorkJournalRecordListResponse> {
        self.inner
            .post(
                "cgi-bin/oa/journal/get_record_list",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_journal_record_detail(
        &self,
        access_token: impl Into<String>,
        journal_uuid: impl Into<String>,
    ) -> Result<WorkJournalRecordDetailResponse> {
        self.inner
            .post(
                "cgi-bin/oa/journal/get_record_detail",
                Some(access_token.into()),
                json!({ "journaluuid": journal_uuid.into() }),
            )
            .await
    }

    pub async fn get_journal_stat_list(
        &self,
        access_token: impl Into<String>,
        request: WorkJournalStatListRequest,
    ) -> Result<WorkJournalStatListResponse> {
        self.inner
            .post(
                "cgi-bin/oa/journal/get_stat_list",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub fn oa_pstncc(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "work.oa.pstncc")
    }

    pub async fn pstncc_call(
        &self,
        access_token: impl Into<String>,
        callee_userid: Vec<String>,
    ) -> Result<WorkPstnccCallResponse> {
        self.inner
            .post(
                "cgi-bin/pstncc/call",
                Some(access_token.into()),
                json!({ "callee_userid": callee_userid }),
            )
            .await
    }

    pub async fn pstncc_get_states(
        &self,
        access_token: impl Into<String>,
        callee_userid: impl Into<String>,
        call_id: impl Into<String>,
    ) -> Result<WorkPstnccGetStatesResponse> {
        self.inner
            .post(
                "cgi-bin/pstncc/getstates",
                Some(access_token.into()),
                json!({ "callee_userid": callee_userid.into(), "callid": call_id.into() }),
            )
            .await
    }

    pub fn oa_schedule(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "work.oa.schedule")
    }

    pub async fn add_schedule(
        &self,
        access_token: impl Into<String>,
        request: WorkScheduleAddRequest,
    ) -> Result<WorkScheduleAddResponse> {
        self.inner
            .post(
                "cgi-bin/oa/schedule/add",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn update_schedule(
        &self,
        access_token: impl Into<String>,
        schedule: Value,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/oa/schedule/update",
                Some(access_token.into()),
                json!({ "schedule": schedule }),
            )
            .await
    }

    pub async fn get_schedule(
        &self,
        access_token: impl Into<String>,
        schedule_id_list: Vec<String>,
    ) -> Result<WorkScheduleGetResponse> {
        self.inner
            .post(
                "cgi-bin/oa/schedule/get",
                Some(access_token.into()),
                json!({ "schedule_id_list": schedule_id_list }),
            )
            .await
    }

    pub async fn delete_schedule(
        &self,
        access_token: impl Into<String>,
        schedule_id: impl Into<String>,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/oa/schedule/del",
                Some(access_token.into()),
                json!({ "schedule_id": schedule_id.into() }),
            )
            .await
    }

    pub async fn appchat_create(
        &self,
        access_token: impl Into<String>,
        request: AppChatCreateRequest,
    ) -> Result<Value> {
        self.inner
            .post("cgi-bin/appchat/create", Some(access_token.into()), request)
            .await
    }

    pub async fn appchat_update(
        &self,
        access_token: impl Into<String>,
        request: AppChatUpdateRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post("cgi-bin/appchat/update", Some(access_token.into()), request)
            .await
    }

    pub async fn appchat_get(
        &self,
        access_token: impl Into<String>,
        chat_id: impl Into<String>,
    ) -> Result<Value> {
        self.inner
            .get_with_query(
                "cgi-bin/appchat/get",
                Some(access_token.into()),
                vec![("chatid".to_string(), chat_id.into())],
            )
            .await
    }

    pub async fn appchat_send(
        &self,
        access_token: impl Into<String>,
        request: AppChatMessage,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post("cgi-bin/appchat/send", Some(access_token.into()), request)
            .await
    }

    pub fn oauth(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "work.oauth")
    }

    pub fn oauth_authorize_url(request: WorkOauthAuthorizeUrlRequest) -> String {
        let scope = request.scope.unwrap_or_else(|| "snsapi_base".to_string());
        let state = request.state.unwrap_or_else(|| "STATE".to_string());
        let mut url = url::Url::parse("https://open.weixin.qq.com/connect/oauth2/authorize")
            .expect("static work oauth url is valid");
        url.query_pairs_mut()
            .append_pair("appid", &request.corp_id)
            .append_pair("redirect_uri", &request.redirect_uri)
            .append_pair("response_type", "code")
            .append_pair("scope", &scope)
            .append_pair("state", &state);
        format!("{url}#wechat_redirect")
    }

    pub async fn oauth_user_info(
        &self,
        access_token: impl Into<String>,
        code: impl Into<String>,
    ) -> Result<Value> {
        self.inner
            .get_with_query(
                "cgi-bin/user/getuserinfo",
                Some(access_token.into()),
                vec![("code".to_string(), code.into())],
            )
            .await
    }

    pub async fn oauth_user_detail(
        &self,
        access_token: impl Into<String>,
        user_ticket: impl Into<String>,
    ) -> Result<Value> {
        self.inner
            .post(
                "cgi-bin/user/getuserdetail",
                Some(access_token.into()),
                json!({ "user_ticket": user_ticket.into() }),
            )
            .await
    }

    pub fn server(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "work.server")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentUpdateRequest {
    pub agentid: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_location_flag: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_mediaid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isreportenter: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentRequest {
    pub name: String,
    pub parentid: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_en: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentCreateResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkIpListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub ip_list: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCorpGroupAppShareInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub corp_list: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCorpGroupTokenResponse {
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
pub struct WorkCorpGroupTransferSessionResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub session_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMiniProgramSessionResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub corpid: Option<String>,
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub deviceid: Option<String>,
    #[serde(default)]
    pub session_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserIdToOpenIdRequest {
    pub userid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserIdToOpenIdResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub openid: Option<String>,
    #[serde(default)]
    pub appid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenIdToUserIdRequest {
    pub openid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenIdToUserIdResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub userid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUnionIdToExternalUserIdRequest {
    pub unionid: String,
    pub openid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_type: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUnionIdToExternalUserIdResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub external_userid: Option<String>,
    #[serde(default)]
    pub pending_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkExternalUserIdToPendingIdRequest {
    pub external_userid: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkExternalUserIdToPendingIdItem {
    #[serde(default)]
    pub external_userid: Option<String>,
    #[serde(default)]
    pub pending_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkExternalUserIdToPendingIdResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub result: Vec<WorkExternalUserIdToPendingIdItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUserIdToOpenUserIdItem {
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub open_userid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUserIdToOpenUserIdResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub open_userid_list: Vec<WorkUserIdToOpenUserIdItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkOpenUserIdToUserIdRequest {
    pub source_agentid: i64,
    pub open_userid_list: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkOpenUserIdToUserIdItem {
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub open_userid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkOpenUserIdToUserIdResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub userid_list: Vec<WorkOpenUserIdToUserIdItem>,
    #[serde(default)]
    pub invalid_userid_list: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkExternalTagIdToOpenExternalTagIdItem {
    #[serde(default)]
    pub external_tagid: Option<String>,
    #[serde(default)]
    pub open_external_tagid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkExternalTagIdToOpenExternalTagIdResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub items: Vec<WorkExternalTagIdToOpenExternalTagIdItem>,
    #[serde(default)]
    pub invalid_tagid_list: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkFromServiceExternalUserIdRequest {
    pub external_userid: String,
    pub source_agentid: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkFromServiceExternalUserIdResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub external_userid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkInvoiceCardRequest {
    pub card_id: String,
    pub encrypt_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkInvoiceStatusRequest {
    pub card_id: String,
    pub encrypt_code: String,
    pub reimburse_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkInvoiceStatusBatchRequest {
    pub openid: String,
    pub reimburse_status: String,
    pub invoice_list: Vec<WorkInvoiceCardRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkInvoiceInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub card_id: Option<String>,
    #[serde(default)]
    pub begin_time: Option<String>,
    #[serde(default)]
    pub end_time: Option<String>,
    #[serde(default)]
    pub openid: Option<String>,
    #[serde(default, rename = "type")]
    pub invoice_type: Option<String>,
    #[serde(default)]
    pub payee: Option<String>,
    #[serde(default)]
    pub detail: Option<String>,
    #[serde(default)]
    pub user_info: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkInvoiceInfoBatchResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub item_list: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkExternalPaySetMerchantUseScopeRequest {
    pub mch_id: String,
    pub allow_use_scope: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkExternalPayBillListRequest {
    pub begin_time: i64,
    pub end_time: i64,
    pub payee_userid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkExternalPayMerchantResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub bind_status: Option<String>,
    #[serde(default)]
    pub mch_id: Option<String>,
    #[serde(default)]
    pub merchant_name: Option<String>,
    #[serde(default)]
    pub allow_use_scope: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkExternalPayBillListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub next_cursor: Option<String>,
    #[serde(default)]
    pub bill_list: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkStatusResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkTicketResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub ticket: Option<String>,
    #[serde(default)]
    pub expires_in: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkJsapiConfig {
    #[serde(rename = "corpId")]
    pub corp_id: String,
    pub timestamp: i64,
    #[serde(rename = "nonceStr")]
    pub nonce_str: String,
    pub signature: String,
    #[serde(rename = "jsApiList")]
    pub js_api_list: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub touser: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toparty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totag: Option<String>,
    pub msgtype: String,
    pub agentid: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub textcard: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safe: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_id_trans: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_duplicate_check: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_check_interval: Option<i64>,
    #[serde(flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageSendResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub invaliduser: Option<String>,
    #[serde(default)]
    pub invalidparty: Option<String>,
    #[serde(default)]
    pub invalidtag: Option<String>,
    #[serde(default)]
    pub msgid: Option<String>,
    #[serde(default)]
    pub response_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMenuRequest {
    pub button: Vec<WorkMenuButton>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMenuButton {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagepath: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sub_button: Vec<WorkMenuButton>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMenuResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub button: Vec<WorkMenuButton>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMenuCreateResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub button: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactBatchGetRequest {
    pub userid_list: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactWayRequest {
    #[serde(rename = "type")]
    pub kind: i64,
    pub scene: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_verify: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_expires_in: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unionid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupRobotMessage {
    pub msgtype: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub news: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_card: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUploadImageResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUploadMediaResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub media_id: Option<String>,
    #[serde(default, rename = "type")]
    pub media_type: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MsgAuditChatDataRequest {
    pub seq: i64,
    pub limit: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passwd: Option<String>,
    #[serde(default)]
    pub timeout: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppChatCreateRequest {
    pub name: String,
    pub owner: String,
    pub userlist: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chatid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalCreateTemplateRequest {
    pub template_name: Vec<Value>,
    pub template_content: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalUpdateTemplateRequest {
    pub template_id: String,
    pub template_name: Vec<Value>,
    pub template_content: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalCreateTemplateResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub template_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCalendarAddRequest {
    pub calendar: Value,
    pub agentid: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCalendarAddResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub cal_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCalendarGetResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub calendar_list: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkDialRecordRequest {
    pub start_time: i64,
    pub end_time: i64,
    pub offset: i64,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkDialRecordResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub record: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkJournalRecordListRequest {
    pub starttime: String,
    pub endtime: String,
    pub cursor: String,
    pub limit: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filters: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkJournalRecordListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub journaluuid_list: Vec<String>,
    #[serde(default)]
    pub next_cursor: Option<i64>,
    #[serde(default)]
    pub endflag: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkJournalRecordDetailResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub info: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkJournalStatListRequest {
    pub template_id: String,
    pub starttime: String,
    pub endtime: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkJournalStatListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub stat_list: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkPstnccCallResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub states: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkPstnccGetStatesResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub istalked: Option<i64>,
    #[serde(default)]
    pub calltime: Option<i64>,
    #[serde(default)]
    pub talktime: Option<i64>,
    #[serde(default)]
    pub reason: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkScheduleAddRequest {
    pub schedule: Value,
    pub agentid: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkScheduleAddResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub schedule_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkScheduleGetResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub schedule_list: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppChatUpdateRequest {
    pub chatid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_user_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub del_user_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppChatMessage {
    pub chatid: String,
    pub msgtype: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Value>,
    #[serde(flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl AppChatMessage {
    pub fn text(chat_id: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            chatid: chat_id.into(),
            msgtype: "text".to_string(),
            text: Some(json!({ "content": content.into() })),
            extra: Value::Null,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkOauthAuthorizeUrlRequest {
    pub corp_id: String,
    pub redirect_uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{
        AgentUpdateRequest, AppChatCreateRequest, AppChatMessage, ContactWayRequest,
        DepartmentCreateResponse, DepartmentRequest, MsgAuditChatDataRequest,
        OpenIdToUserIdRequest, OpenIdToUserIdResponse, UserIdToOpenIdRequest,
        UserIdToOpenIdResponse, Work, WorkApprovalCreateTemplateRequest,
        WorkApprovalCreateTemplateResponse, WorkApprovalUpdateTemplateRequest,
        WorkCalendarAddRequest, WorkCalendarAddResponse, WorkCalendarGetResponse,
        WorkCorpGroupAppShareInfoResponse, WorkCorpGroupTokenResponse,
        WorkCorpGroupTransferSessionResponse, WorkDialRecordRequest, WorkDialRecordResponse,
        WorkExternalPayBillListRequest, WorkExternalPayBillListResponse,
        WorkExternalPayMerchantResponse, WorkExternalPaySetMerchantUseScopeRequest,
        WorkExternalTagIdToOpenExternalTagIdResponse, WorkExternalUserIdToPendingIdRequest,
        WorkExternalUserIdToPendingIdResponse, WorkFromServiceExternalUserIdRequest,
        WorkFromServiceExternalUserIdResponse, WorkInvoiceCardRequest,
        WorkInvoiceInfoBatchResponse, WorkInvoiceInfoResponse, WorkInvoiceStatusBatchRequest,
        WorkInvoiceStatusRequest, WorkIpListResponse, WorkJournalRecordDetailResponse,
        WorkJournalRecordListRequest, WorkJournalRecordListResponse, WorkJournalStatListRequest,
        WorkJournalStatListResponse, WorkMenuButton, WorkMenuRequest, WorkMenuResponse,
        WorkMessage, WorkMiniProgramSessionResponse, WorkOauthAuthorizeUrlRequest,
        WorkOpenUserIdToUserIdRequest, WorkOpenUserIdToUserIdResponse, WorkPstnccCallResponse,
        WorkPstnccGetStatesResponse, WorkScheduleAddRequest, WorkScheduleAddResponse,
        WorkScheduleGetResponse, WorkUnionIdToExternalUserIdRequest,
        WorkUnionIdToExternalUserIdResponse, WorkUploadMediaResponse,
        WorkUserIdToOpenUserIdResponse,
    };

    #[test]
    fn serializes_text_message_shape() {
        let value = serde_json::to_value(WorkMessage {
            touser: Some("user".to_string()),
            toparty: None,
            totag: None,
            msgtype: "text".to_string(),
            agentid: 100001,
            text: Some(json!({ "content": "hello" })),
            markdown: None,
            textcard: None,
            safe: Some(0),
            enable_id_trans: None,
            enable_duplicate_check: None,
            duplicate_check_interval: None,
            extra: serde_json::Value::Null,
        })
        .unwrap();

        assert_eq!(value["touser"], "user");
        assert_eq!(value["msgtype"], "text");
        assert_eq!(value["text"]["content"], "hello");
    }

    #[test]
    fn serializes_group_robot_text_message() {
        let value = serde_json::to_value(Work::group_robot_text("hello", vec!["@all".to_string()]))
            .unwrap();

        assert_eq!(value["msgtype"], "text");
        assert_eq!(value["text"]["mentioned_list"][0], "@all");
    }

    #[test]
    fn serializes_work_menu_request() {
        let value = serde_json::to_value(WorkMenuRequest {
            button: vec![
                WorkMenuButton {
                    kind: Some("click".to_string()),
                    name: "Today".to_string(),
                    key: Some("today".to_string()),
                    pagepath: None,
                    appid: None,
                    url: None,
                    sub_button: Vec::new(),
                },
                WorkMenuButton {
                    kind: None,
                    name: "More".to_string(),
                    key: None,
                    pagepath: None,
                    appid: None,
                    url: None,
                    sub_button: vec![WorkMenuButton {
                        kind: Some("view".to_string()),
                        name: "Docs".to_string(),
                        key: None,
                        pagepath: None,
                        appid: None,
                        url: Some("https://example.com".to_string()),
                        sub_button: Vec::new(),
                    }],
                },
            ],
        })
        .unwrap();

        assert_eq!(value["button"][0]["type"], "click");
        assert_eq!(value["button"][0]["key"], "today");
        assert_eq!(
            value["button"][1]["sub_button"][0]["url"],
            "https://example.com"
        );
        assert!(value["button"][1].get("type").is_none());
    }

    #[test]
    fn deserializes_work_menu_response() {
        let menu: WorkMenuResponse = serde_json::from_value(json!({
            "errcode": 0,
            "button": [{
                "type": "click",
                "name": "Today",
                "key": "today",
                "sub_button": []
            }]
        }))
        .unwrap();

        assert_eq!(menu.button[0].kind.as_deref(), Some("click"));
        assert_eq!(menu.button[0].key.as_deref(), Some("today"));
    }

    #[test]
    fn serializes_contact_way_type_field() {
        let value = serde_json::to_value(ContactWayRequest {
            kind: 1,
            scene: 2,
            style: None,
            remark: Some("remark".to_string()),
            skip_verify: Some(true),
            state: None,
            user: Some(vec!["user".to_string()]),
            party: None,
            expires_in: None,
            chat_expires_in: None,
            unionid: None,
        })
        .unwrap();

        assert_eq!(value["type"], 1);
        assert_eq!(value["user"][0], "user");
    }

    #[test]
    fn serializes_department_request() {
        let value = serde_json::to_value(DepartmentRequest {
            name: "Engineering".to_string(),
            parentid: 1,
            id: Some(2),
            name_en: Some("Engineering".to_string()),
            order: None,
        })
        .unwrap();

        assert_eq!(value["name"], "Engineering");
        assert_eq!(value["parentid"], 1);
        assert_eq!(value["id"], 2);
        assert_eq!(value["name_en"], "Engineering");
        assert!(value.get("order").is_none());
    }

    #[test]
    fn deserializes_department_create_response() {
        let response: DepartmentCreateResponse =
            serde_json::from_value(json!({ "errcode": 0, "id": 42 })).unwrap();

        assert_eq!(response.errcode, Some(0));
        assert_eq!(response.id, Some(42));
    }

    #[test]
    fn deserializes_work_base_responses() {
        let callback: WorkIpListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "ip_list": ["1.1.1.1", "2.2.2.2"]
        }))
        .unwrap();

        assert_eq!(callback.ip_list[0], "1.1.1.1");
        assert_eq!(callback.ip_list.len(), 2);
    }

    #[test]
    fn deserializes_work_corpgroup_responses() {
        let share: WorkCorpGroupAppShareInfoResponse = serde_json::from_value(json!({
            "corp_list": [{ "corpid": "corp", "agentid": 100001 }]
        }))
        .unwrap();
        assert_eq!(share.corp_list[0]["corpid"], "corp");

        let token: WorkCorpGroupTokenResponse = serde_json::from_value(json!({
            "access_token": "token",
            "expires_in": 7200
        }))
        .unwrap();
        assert_eq!(token.access_token.as_deref(), Some("token"));
        assert_eq!(token.expires_in, Some(7200));

        let session: WorkCorpGroupTransferSessionResponse = serde_json::from_value(json!({
            "userid": "user",
            "session_key": "session"
        }))
        .unwrap();
        assert_eq!(session.userid.as_deref(), Some("user"));
        assert_eq!(session.session_key.as_deref(), Some("session"));
    }

    #[test]
    fn deserializes_work_mini_program_session_response() {
        let session: WorkMiniProgramSessionResponse = serde_json::from_value(json!({
            "corpid": "corp",
            "userid": "user",
            "deviceid": "device",
            "session_key": "session"
        }))
        .unwrap();

        assert_eq!(session.corpid.as_deref(), Some("corp"));
        assert_eq!(session.userid.as_deref(), Some("user"));
        assert_eq!(session.deviceid.as_deref(), Some("device"));
        assert_eq!(session.session_key.as_deref(), Some("session"));
    }

    #[test]
    fn serializes_user_id_openid_conversion_requests() {
        let to_openid = serde_json::to_value(UserIdToOpenIdRequest {
            userid: "user".to_string(),
        })
        .unwrap();
        let to_userid = serde_json::to_value(OpenIdToUserIdRequest {
            openid: "openid".to_string(),
        })
        .unwrap();

        assert_eq!(to_openid, json!({ "userid": "user" }));
        assert_eq!(to_userid, json!({ "openid": "openid" }));
    }

    #[test]
    fn serializes_id_convert_requests() {
        let union = serde_json::to_value(WorkUnionIdToExternalUserIdRequest {
            unionid: "union".to_string(),
            openid: "openid".to_string(),
            subject_type: Some(1),
        })
        .unwrap();
        assert_eq!(union["unionid"], "union");
        assert_eq!(union["subject_type"], 1);

        let pending = serde_json::to_value(WorkExternalUserIdToPendingIdRequest {
            external_userid: vec!["external".to_string()],
            chat_id: None,
        })
        .unwrap();
        assert_eq!(pending, json!({ "external_userid": ["external"] }));

        let open_to_user = serde_json::to_value(WorkOpenUserIdToUserIdRequest {
            source_agentid: 100001,
            open_userid_list: vec!["open-user".to_string()],
        })
        .unwrap();
        assert_eq!(open_to_user["source_agentid"], 100001);
        assert_eq!(open_to_user["open_userid_list"][0], "open-user");

        let from_service = serde_json::to_value(WorkFromServiceExternalUserIdRequest {
            external_userid: "service-external".to_string(),
            source_agentid: 100001,
        })
        .unwrap();
        assert_eq!(from_service["external_userid"], "service-external");
    }

    #[test]
    fn deserializes_user_id_openid_conversion_responses() {
        let to_openid: UserIdToOpenIdResponse =
            serde_json::from_value(json!({ "openid": "openid", "appid": "wxappid" })).unwrap();
        let to_userid: OpenIdToUserIdResponse =
            serde_json::from_value(json!({ "userid": "user" })).unwrap();

        assert_eq!(to_openid.openid.as_deref(), Some("openid"));
        assert_eq!(to_openid.appid.as_deref(), Some("wxappid"));
        assert_eq!(to_userid.userid.as_deref(), Some("user"));
    }

    #[test]
    fn deserializes_id_convert_responses() {
        let union: WorkUnionIdToExternalUserIdResponse = serde_json::from_value(json!({
            "errcode": 0,
            "external_userid": "external",
            "pending_id": "pending"
        }))
        .unwrap();
        assert_eq!(union.external_userid.as_deref(), Some("external"));

        let pending: WorkExternalUserIdToPendingIdResponse = serde_json::from_value(json!({
            "result": [{ "external_userid": "external", "pending_id": "pending" }]
        }))
        .unwrap();
        assert_eq!(pending.result[0].pending_id.as_deref(), Some("pending"));

        let user_to_open: WorkUserIdToOpenUserIdResponse = serde_json::from_value(json!({
            "open_userid_list": [{ "userid": "user", "open_userid": "open-user" }]
        }))
        .unwrap();
        assert_eq!(
            user_to_open.open_userid_list[0].open_userid.as_deref(),
            Some("open-user")
        );

        let open_to_user: WorkOpenUserIdToUserIdResponse = serde_json::from_value(json!({
            "userid_list": [{ "userid": "user", "open_userid": "open-user" }],
            "invalid_userid_list": ["bad-open-user"]
        }))
        .unwrap();
        assert_eq!(open_to_user.userid_list[0].userid.as_deref(), Some("user"));
        assert_eq!(open_to_user.invalid_userid_list[0], "bad-open-user");

        let tag: WorkExternalTagIdToOpenExternalTagIdResponse = serde_json::from_value(json!({
            "items": [{ "external_tagid": "tag", "open_external_tagid": "open-tag" }],
            "invalid_tagid_list": []
        }))
        .unwrap();
        assert_eq!(
            tag.items[0].open_external_tagid.as_deref(),
            Some("open-tag")
        );

        let from_service: WorkFromServiceExternalUserIdResponse =
            serde_json::from_value(json!({ "external_userid": "external" })).unwrap();
        assert_eq!(from_service.external_userid.as_deref(), Some("external"));
    }

    #[test]
    fn serializes_invoice_requests() {
        let card = WorkInvoiceCardRequest {
            card_id: "card".to_string(),
            encrypt_code: "encrypted".to_string(),
        };
        let value = serde_json::to_value(&card).unwrap();
        assert_eq!(
            value,
            json!({ "card_id": "card", "encrypt_code": "encrypted" })
        );

        let status = serde_json::to_value(WorkInvoiceStatusRequest {
            card_id: "card".to_string(),
            encrypt_code: "encrypted".to_string(),
            reimburse_status: "INVOICE_REIMBURSE_INIT".to_string(),
        })
        .unwrap();
        assert_eq!(status["reimburse_status"], "INVOICE_REIMBURSE_INIT");

        let batch = serde_json::to_value(WorkInvoiceStatusBatchRequest {
            openid: "openid".to_string(),
            reimburse_status: "INVOICE_REIMBURSE_CLOSURE".to_string(),
            invoice_list: vec![card],
        })
        .unwrap();
        assert_eq!(batch["openid"], "openid");
        assert_eq!(batch["invoice_list"][0]["card_id"], "card");
    }

    #[test]
    fn deserializes_invoice_responses() {
        let info: WorkInvoiceInfoResponse = serde_json::from_value(json!({
            "errcode": 0,
            "card_id": "card",
            "begin_time": "2026-01-01",
            "end_time": "2026-01-31",
            "openid": "openid",
            "type": "vat",
            "payee": "Roze",
            "detail": "Cloud service",
            "user_info": { "fee": 100 }
        }))
        .unwrap();
        assert_eq!(info.card_id.as_deref(), Some("card"));
        assert_eq!(info.invoice_type.as_deref(), Some("vat"));
        assert_eq!(info.user_info.unwrap()["fee"], 100);

        let batch: WorkInvoiceInfoBatchResponse = serde_json::from_value(json!({
            "item_list": [{ "card_id": "card", "encrypt_code": "encrypted" }]
        }))
        .unwrap();
        assert_eq!(batch.item_list[0]["card_id"], "card");
    }

    #[test]
    fn serializes_external_pay_requests() {
        let scope = serde_json::to_value(WorkExternalPaySetMerchantUseScopeRequest {
            mch_id: "1900000109".to_string(),
            allow_use_scope: "all".to_string(),
        })
        .unwrap();
        assert_eq!(scope["mch_id"], "1900000109");
        assert_eq!(scope["allow_use_scope"], "all");

        let bill = serde_json::to_value(WorkExternalPayBillListRequest {
            begin_time: 1_800_000_000,
            end_time: 1_800_086_400,
            payee_userid: "user".to_string(),
            cursor: None,
            limit: 100,
        })
        .unwrap();
        assert_eq!(bill["payee_userid"], "user");
        assert_eq!(bill["limit"], 100);
        assert!(bill.get("cursor").is_none());
    }

    #[test]
    fn deserializes_external_pay_responses() {
        let merchant: WorkExternalPayMerchantResponse = serde_json::from_value(json!({
            "bind_status": "bind",
            "mch_id": "1900000109",
            "merchant_name": "Roze Shop",
            "allow_use_scope": [{ "type": "all" }]
        }))
        .unwrap();
        assert_eq!(merchant.mch_id.as_deref(), Some("1900000109"));
        assert_eq!(merchant.allow_use_scope[0]["type"], "all");

        let bills: WorkExternalPayBillListResponse = serde_json::from_value(json!({
            "next_cursor": "cursor",
            "bill_list": [{ "out_trade_no": "trade-no", "amount": 100 }]
        }))
        .unwrap();
        assert_eq!(bills.next_cursor.as_deref(), Some("cursor"));
        assert_eq!(bills.bill_list[0]["out_trade_no"], "trade-no");
    }

    #[test]
    fn deserializes_upload_media_response_type_field() {
        let response: WorkUploadMediaResponse =
            serde_json::from_value(json!({ "media_id": "mid", "type": "image" })).unwrap();

        assert_eq!(response.media_id.as_deref(), Some("mid"));
        assert_eq!(response.media_type.as_deref(), Some("image"));
    }

    #[test]
    fn serializes_work_oa_calendar_and_dial_requests() {
        let calendar = serde_json::to_value(WorkCalendarAddRequest {
            calendar: json!({
                "organizer": "user",
                "readonly": 0,
                "summary": "Team"
            }),
            agentid: 100001,
        })
        .unwrap();
        assert_eq!(calendar["agentid"], 100001);
        assert_eq!(calendar["calendar"]["summary"], "Team");

        let dial = serde_json::to_value(WorkDialRecordRequest {
            start_time: 1_800_000_000,
            end_time: 1_800_086_400,
            offset: 0,
            limit: 100,
        })
        .unwrap();
        assert_eq!(dial["start_time"], 1_800_000_000);
        assert_eq!(dial["limit"], 100);
    }

    #[test]
    fn deserializes_work_oa_calendar_dial_and_pstncc_responses() {
        let calendar_add: WorkCalendarAddResponse =
            serde_json::from_value(json!({ "errcode": 0, "cal_id": "wc100" })).unwrap();
        assert_eq!(calendar_add.cal_id.as_deref(), Some("wc100"));

        let calendar_get: WorkCalendarGetResponse = serde_json::from_value(json!({
            "calendar_list": [{ "cal_id": "wc100", "summary": "Team" }]
        }))
        .unwrap();
        assert_eq!(calendar_get.calendar_list[0]["summary"], "Team");

        let dial: WorkDialRecordResponse = serde_json::from_value(json!({
            "record": [{ "callee": "user", "duration": 60 }]
        }))
        .unwrap();
        assert_eq!(dial.record[0]["callee"], "user");

        let call: WorkPstnccCallResponse = serde_json::from_value(json!({
            "states": [{ "callee_userid": "user", "callid": "call-1" }]
        }))
        .unwrap();
        assert_eq!(call.states[0]["callid"], "call-1");

        let states: WorkPstnccGetStatesResponse = serde_json::from_value(json!({
            "istalked": 1,
            "calltime": 1_800_000_000,
            "talktime": 30,
            "reason": 0
        }))
        .unwrap();
        assert_eq!(states.istalked, Some(1));
        assert_eq!(states.reason, Some(0));
    }

    #[test]
    fn serializes_work_oa_approval_journal_and_schedule_requests() {
        let approval_create = serde_json::to_value(WorkApprovalCreateTemplateRequest {
            template_name: vec![json!({ "text": "Leave", "lang": "zh_CN" })],
            template_content: json!({
                "controls": [{
                    "property": {
                        "control": "Text",
                        "id": "Text-1",
                        "title": [{ "text": "Reason", "lang": "zh_CN" }],
                        "placeholder": [{ "text": "Input", "lang": "zh_CN" }],
                        "require": 1,
                        "un_print": 0
                    },
                    "config": {}
                }]
            }),
        })
        .unwrap();
        assert_eq!(approval_create["template_name"][0]["text"], "Leave");
        assert_eq!(
            approval_create["template_content"]["controls"][0]["property"]["id"],
            "Text-1"
        );

        let approval_update = serde_json::to_value(WorkApprovalUpdateTemplateRequest {
            template_id: "template-1".to_string(),
            template_name: vec![json!({ "text": "Leave", "lang": "zh_CN" })],
            template_content: json!({ "controls": [] }),
        })
        .unwrap();
        assert_eq!(approval_update["template_id"], "template-1");

        let journal = serde_json::to_value(WorkJournalRecordListRequest {
            starttime: "1800000000".to_string(),
            endtime: "1800086400".to_string(),
            cursor: "0".to_string(),
            limit: 100,
            filters: vec![json!({ "key": "template_id", "value": "template-1" })],
        })
        .unwrap();
        assert_eq!(journal["starttime"], "1800000000");
        assert_eq!(journal["filters"][0]["key"], "template_id");

        let stat = serde_json::to_value(WorkJournalStatListRequest {
            template_id: "template-1".to_string(),
            starttime: "1800000000".to_string(),
            endtime: "1800086400".to_string(),
        })
        .unwrap();
        assert_eq!(stat["template_id"], "template-1");

        let schedule = serde_json::to_value(WorkScheduleAddRequest {
            schedule: json!({
                "organizer": "user",
                "start_time": 1_800_000_000,
                "end_time": 1_800_003_600
            }),
            agentid: 100001,
        })
        .unwrap();
        assert_eq!(schedule["agentid"], 100001);
        assert_eq!(schedule["schedule"]["organizer"], "user");
    }

    #[test]
    fn deserializes_work_oa_approval_journal_and_schedule_responses() {
        let approval: WorkApprovalCreateTemplateResponse = serde_json::from_value(json!({
            "errcode": 0,
            "template_id": "template-1"
        }))
        .unwrap();
        assert_eq!(approval.template_id.as_deref(), Some("template-1"));

        let records: WorkJournalRecordListResponse = serde_json::from_value(json!({
            "journaluuid_list": ["journal-1"],
            "next_cursor": 10,
            "endflag": false
        }))
        .unwrap();
        assert_eq!(records.journaluuid_list[0], "journal-1");
        assert_eq!(records.next_cursor, Some(10));

        let detail: WorkJournalRecordDetailResponse = serde_json::from_value(json!({
            "info": { "journaluuid": "journal-1", "template_id": "template-1" }
        }))
        .unwrap();
        assert_eq!(detail.info.unwrap()["journaluuid"], "journal-1");

        let stats: WorkJournalStatListResponse = serde_json::from_value(json!({
            "stat_list": { "summary": [{ "userid": "user", "count": 3 }] }
        }))
        .unwrap();
        assert_eq!(stats.stat_list.unwrap()["summary"][0]["count"], 3);

        let schedule_add: WorkScheduleAddResponse =
            serde_json::from_value(json!({ "schedule_id": "schedule-1" })).unwrap();
        assert_eq!(schedule_add.schedule_id.as_deref(), Some("schedule-1"));

        let schedule_get: WorkScheduleGetResponse = serde_json::from_value(json!({
            "schedule_list": [{ "schedule_id": "schedule-1", "summary": "Daily" }]
        }))
        .unwrap();
        assert_eq!(schedule_get.schedule_list[0]["summary"], "Daily");
    }

    #[test]
    fn serializes_agent_update_request() {
        let value = serde_json::to_value(AgentUpdateRequest {
            agentid: 100001,
            report_location_flag: None,
            logo_mediaid: None,
            name: Some("agent".to_string()),
            description: None,
            redirect_domain: None,
            isreportenter: Some(1),
            home_url: None,
        })
        .unwrap();

        assert_eq!(value["agentid"], 100001);
        assert_eq!(value["name"], "agent");
        assert!(value.get("home_url").is_none());
    }

    #[test]
    fn serializes_msg_audit_chat_data_request() {
        let value = serde_json::to_value(MsgAuditChatDataRequest {
            seq: 1,
            limit: 100,
            proxy: None,
            passwd: None,
            timeout: 10,
        })
        .unwrap();

        assert_eq!(value["seq"], 1);
        assert_eq!(value["limit"], 100);
        assert_eq!(value["timeout"], 10);
        assert!(value.get("proxy").is_none());
    }

    #[test]
    fn serializes_appchat_requests() {
        let create = serde_json::to_value(AppChatCreateRequest {
            name: "chat".to_string(),
            owner: "owner".to_string(),
            userlist: vec!["user".to_string()],
            chatid: Some("chatid".to_string()),
        })
        .unwrap();
        let message = serde_json::to_value(AppChatMessage::text("chatid", "hello")).unwrap();

        assert_eq!(create["userlist"][0], "user");
        assert_eq!(message["chatid"], "chatid");
        assert_eq!(message["text"]["content"], "hello");
    }

    #[test]
    fn builds_work_oauth_url() {
        let url = Work::oauth_authorize_url(WorkOauthAuthorizeUrlRequest {
            corp_id: "wwid".to_string(),
            redirect_uri: "https://example.com/cb".to_string(),
            scope: None,
            state: Some("abc".to_string()),
        });

        assert!(url.contains("appid=wwid"));
        assert!(url.contains("scope=snsapi_base"));
        assert!(url.ends_with("#wechat_redirect"));
    }
}
