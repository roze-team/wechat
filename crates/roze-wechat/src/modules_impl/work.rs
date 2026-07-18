use serde::{Deserialize, Serialize};
use serde_json::{json, to_value, Map, Value};

use crate::{
    config::Platform,
    crypto,
    error::{Result, WechatError},
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

    pub async fn access_token(
        &self,
        corp_id: impl Into<String>,
        corp_secret: impl Into<String>,
    ) -> Result<WorkAccessTokenResponse> {
        self.inner
            .get_with_query(
                "cgi-bin/gettoken",
                None,
                vec![
                    ("corpid".to_string(), corp_id.into()),
                    ("corpsecret".to_string(), corp_secret.into()),
                ],
            )
            .await
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

    pub async fn list_agents(
        &self,
        access_token: impl Into<String>,
    ) -> Result<WorkAgentListResponse> {
        self.inner
            .get("cgi-bin/agent/list", Some(access_token.into()))
            .await
    }

    pub async fn get_agent(
        &self,
        access_token: impl Into<String>,
        agent_id: i64,
    ) -> Result<WorkAgentDetailResponse> {
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

    pub async fn set_agent_scope(
        &self,
        access_token: impl Into<String>,
        request: WorkAgentScopeRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/agent/set_scope",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_agent_workbench_template(
        &self,
        access_token: impl Into<String>,
        agent_id: i64,
    ) -> Result<WorkAgentWorkbenchTemplateResponse> {
        self.inner
            .get_with_query(
                "cgi-bin/agent/get_workbench_template",
                Some(access_token.into()),
                vec![("agentid".to_string(), agent_id.to_string())],
            )
            .await
    }

    pub async fn set_agent_workbench_template(
        &self,
        access_token: impl Into<String>,
        request: WorkAgentWorkbenchTemplateRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/agent/set_workbench_template",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn set_agent_workbench_data(
        &self,
        access_token: impl Into<String>,
        request: WorkAgentWorkbenchDataRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/agent/set_workbench_data",
                Some(access_token.into()),
                request,
            )
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
    ) -> Result<WorkDepartmentListResponse> {
        let query = id
            .map(|id| vec![("id".to_string(), id.to_string())])
            .unwrap_or_default();
        self.inner
            .get_with_query("cgi-bin/department/list", Some(access_token.into()), query)
            .await
    }

    pub async fn simple_list_departments(
        &self,
        access_token: impl Into<String>,
        id: Option<i64>,
    ) -> Result<WorkDepartmentSimpleListResponse> {
        let query = id
            .map(|id| vec![("id".to_string(), id.to_string())])
            .unwrap_or_default();
        self.inner
            .get_with_query(
                "cgi-bin/department/simplelist",
                Some(access_token.into()),
                query,
            )
            .await
    }

    pub async fn get_department(
        &self,
        access_token: impl Into<String>,
        id: i64,
    ) -> Result<WorkDepartmentDetailResponse> {
        self.inner
            .get_with_query(
                "cgi-bin/department/get",
                Some(access_token.into()),
                vec![("id".to_string(), id.to_string())],
            )
            .await
    }

    pub async fn get_user(
        &self,
        access_token: impl Into<String>,
        user_id: impl Into<String>,
    ) -> Result<WorkUserDetailResponse> {
        self.inner
            .get_with_query(
                "cgi-bin/user/get",
                Some(access_token.into()),
                vec![("userid".to_string(), user_id.into())],
            )
            .await
    }

    pub async fn create_user(
        &self,
        access_token: impl Into<String>,
        request: WorkUserRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post("cgi-bin/user/create", Some(access_token.into()), request)
            .await
    }

    pub async fn update_user(
        &self,
        access_token: impl Into<String>,
        request: WorkUserRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post("cgi-bin/user/update", Some(access_token.into()), request)
            .await
    }

    pub async fn delete_user(
        &self,
        access_token: impl Into<String>,
        user_id: impl Into<String>,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .get_with_query(
                "cgi-bin/user/delete",
                Some(access_token.into()),
                vec![("userid".to_string(), user_id.into())],
            )
            .await
    }

    pub async fn batch_delete_users(
        &self,
        access_token: impl Into<String>,
        user_id_list: Vec<String>,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/user/batchdelete",
                Some(access_token.into()),
                WorkUserBatchDeleteRequest {
                    useridlist: user_id_list,
                },
            )
            .await
    }

    pub async fn list_department_users(
        &self,
        access_token: impl Into<String>,
        department_id: i64,
        fetch_child: bool,
    ) -> Result<WorkDepartmentUserSimpleListResponse> {
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
    ) -> Result<WorkDepartmentUserDetailListResponse> {
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

    pub async fn list_user_ids(
        &self,
        access_token: impl Into<String>,
        cursor: impl Into<String>,
        limit: i64,
    ) -> Result<WorkUserListIdResponse> {
        self.inner
            .post(
                "cgi-bin/user/list_id",
                Some(access_token.into()),
                json!({ "cursor": cursor.into(), "limit": limit }),
            )
            .await
    }

    pub async fn sync_users_by_batch(
        &self,
        access_token: impl Into<String>,
        request: WorkUserBatchJobRequest,
    ) -> Result<WorkUserBatchJobResponse> {
        self.inner
            .post("cgi-bin/batch/syncuser", Some(access_token.into()), request)
            .await
    }

    pub async fn replace_users_by_batch(
        &self,
        access_token: impl Into<String>,
        request: WorkUserBatchJobRequest,
    ) -> Result<WorkUserBatchJobResponse> {
        self.inner
            .post(
                "cgi-bin/batch/replaceuser",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn replace_departments_by_batch(
        &self,
        access_token: impl Into<String>,
        request: WorkUserBatchJobRequest,
    ) -> Result<WorkUserBatchJobResponse> {
        self.inner
            .post(
                "cgi-bin/batch/replaceparty",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_user_batch_job_result(
        &self,
        access_token: impl Into<String>,
        job_id: impl Into<String>,
    ) -> Result<WorkUserBatchJobResultResponse> {
        self.inner
            .get_with_query(
                "cgi-bin/batch/getresult",
                Some(access_token.into()),
                vec![("jobid".to_string(), job_id.into())],
            )
            .await
    }

    pub async fn export_simple_users(
        &self,
        access_token: impl Into<String>,
        request: WorkUserExportJobRequest,
    ) -> Result<WorkUserExportJobResponse> {
        self.inner
            .post(
                "cgi-bin/export/simple_user",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn export_users(
        &self,
        access_token: impl Into<String>,
        request: WorkUserExportJobRequest,
    ) -> Result<WorkUserExportJobResponse> {
        self.inner
            .post("cgi-bin/export/user", Some(access_token.into()), request)
            .await
    }

    pub async fn export_departments(
        &self,
        access_token: impl Into<String>,
        request: WorkUserExportJobRequest,
    ) -> Result<WorkUserExportJobResponse> {
        self.inner
            .post(
                "cgi-bin/export/department",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn export_tag_users(
        &self,
        access_token: impl Into<String>,
        request: WorkUserExportTagUserJobRequest,
    ) -> Result<WorkUserExportJobResponse> {
        self.inner
            .post("cgi-bin/export/taguser", Some(access_token.into()), request)
            .await
    }

    pub async fn get_user_export_job_result(
        &self,
        access_token: impl Into<String>,
        job_id: impl Into<String>,
    ) -> Result<WorkUserExportJobResultResponse> {
        self.inner
            .get_with_query(
                "cgi-bin/export/get_result",
                Some(access_token.into()),
                vec![("jobid".to_string(), job_id.into())],
            )
            .await
    }

    pub async fn mobile_to_user_id(
        &self,
        access_token: impl Into<String>,
        mobile: impl Into<String>,
    ) -> Result<WorkUserIdLookupResponse> {
        self.inner
            .post(
                "cgi-bin/user/getuserid",
                Some(access_token.into()),
                json!({ "mobile": mobile.into() }),
            )
            .await
    }

    pub async fn email_to_user_id(
        &self,
        access_token: impl Into<String>,
        email: impl Into<String>,
        email_type: i64,
    ) -> Result<WorkUserIdLookupResponse> {
        self.inner
            .post(
                "cgi-bin/user/get_userid_by_email",
                Some(access_token.into()),
                json!({ "email": email.into(), "email_type": email_type }),
            )
            .await
    }

    pub async fn accept_user_auth_success(
        &self,
        access_token: impl Into<String>,
        user_id: impl Into<String>,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .get_with_query(
                "cgi-bin/user/authsucc",
                Some(access_token.into()),
                vec![("userid".to_string(), user_id.into())],
            )
            .await
    }

    pub async fn invite_users(
        &self,
        access_token: impl Into<String>,
        request: WorkUserInviteRequest,
    ) -> Result<WorkUserInviteResponse> {
        self.inner
            .post("cgi-bin/batch/invite", Some(access_token.into()), request)
            .await
    }

    pub async fn get_join_qrcode(
        &self,
        access_token: impl Into<String>,
        size_type: i64,
    ) -> Result<WorkJoinQrCodeResponse> {
        self.inner
            .get_with_query(
                "cgi-bin/corp/get_join_qrcode",
                Some(access_token.into()),
                vec![("size_type".to_string(), size_type.to_string())],
            )
            .await
    }

    pub async fn get_user_active_stat(
        &self,
        access_token: impl Into<String>,
        date: impl Into<String>,
    ) -> Result<WorkUserActiveStatResponse> {
        self.inner
            .post(
                "cgi-bin/user/get_active_stat",
                Some(access_token.into()),
                json!({ "date": date.into() }),
            )
            .await
    }

    pub async fn get_linked_corp_perm_list(
        &self,
        access_token: impl Into<String>,
    ) -> Result<WorkLinkedCorpPermListResponse> {
        self.inner
            .post(
                "cgi-bin/linkedcorp/agent/get_perm_list",
                Some(access_token.into()),
                Value::Null,
            )
            .await
    }

    pub async fn get_linked_corp_user(
        &self,
        access_token: impl Into<String>,
        user_id: impl Into<String>,
    ) -> Result<WorkLinkedCorpUserResponse> {
        self.inner
            .post(
                "cgi-bin/linkedcorp/user/get",
                Some(access_token.into()),
                json!({ "userid": user_id.into() }),
            )
            .await
    }

    pub async fn list_linked_corp_department_users(
        &self,
        access_token: impl Into<String>,
        department_id: impl Into<String>,
        fetch_child: bool,
    ) -> Result<WorkLinkedCorpUserListResponse> {
        self.inner
            .post(
                "cgi-bin/linkedcorp/user/simplelist",
                Some(access_token.into()),
                json!({ "department_id": department_id.into(), "fetch_child": fetch_child }),
            )
            .await
    }

    pub async fn list_linked_corp_detailed_department_users(
        &self,
        access_token: impl Into<String>,
        department_id: impl Into<String>,
        fetch_child: bool,
    ) -> Result<WorkLinkedCorpUserListResponse> {
        self.inner
            .post(
                "cgi-bin/linkedcorp/user/list",
                Some(access_token.into()),
                json!({ "department_id": department_id.into(), "fetch_child": fetch_child }),
            )
            .await
    }

    pub async fn list_linked_corp_departments(
        &self,
        access_token: impl Into<String>,
        department_id: impl Into<String>,
    ) -> Result<WorkLinkedCorpDepartmentListResponse> {
        self.inner
            .post(
                "cgi-bin/linkedcorp/department/list",
                Some(access_token.into()),
                json!({ "department_id": department_id.into() }),
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

    pub async fn to_service_external_user_id(
        &self,
        access_token: impl Into<String>,
        external_user_id: impl Into<String>,
    ) -> Result<WorkToServiceExternalUserIdResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/to_service_external_userid",
                Some(access_token.into()),
                json!({ "external_userid": external_user_id.into() }),
            )
            .await
    }

    pub async fn finish_external_user_id_migration(
        &self,
        provider_access_token: impl Into<String>,
        corp_id: impl Into<String>,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/finish_external_userid_migration",
                Some(provider_access_token.into()),
                json!({ "corpid": corp_id.into() }),
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
    ) -> Result<ExternalContactListResponse> {
        self.inner
            .get_with_query(
                "cgi-bin/externalcontact/list",
                Some(access_token.into()),
                vec![("userid".to_string(), user_id.into())],
            )
            .await
    }

    pub async fn list_served_external_contacts(
        &self,
        access_token: impl Into<String>,
        request: ExternalContactServedListRequest,
    ) -> Result<ExternalContactServedListResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/contact_list",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn external_contact_to_openid(
        &self,
        access_token: impl Into<String>,
        external_user_id: impl Into<String>,
    ) -> Result<UserIdToOpenIdResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/convert_to_openid",
                Some(access_token.into()),
                json!({ "external_userid": external_user_id.into() }),
            )
            .await
    }

    pub async fn get_school_notification_subscribe_qr_code(
        &self,
        access_token: impl Into<String>,
    ) -> Result<WorkSchoolSubscribeQrCodeResponse> {
        self.inner
            .get(
                "cgi-bin/externalcontact/get_subscribe_qr_code",
                Some(access_token.into()),
            )
            .await
    }

    pub async fn set_school_notification_subscribe_mode(
        &self,
        access_token: impl Into<String>,
        subscribe_mode: i64,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/set_subscribe_mode",
                Some(access_token.into()),
                json!({ "subscribe_mode": subscribe_mode }),
            )
            .await
    }

    pub async fn get_school_notification_subscribe_mode(
        &self,
        access_token: impl Into<String>,
    ) -> Result<WorkSchoolSubscribeModeResponse> {
        self.inner
            .get(
                "cgi-bin/externalcontact/get_subscribe_mode",
                Some(access_token.into()),
            )
            .await
    }

    pub async fn get_external_contact(
        &self,
        access_token: impl Into<String>,
        external_userid: impl Into<String>,
        cursor: Option<String>,
    ) -> Result<ExternalContactDetailResponse> {
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

    pub async fn list_external_contact_follow_users(
        &self,
        access_token: impl Into<String>,
    ) -> Result<ExternalContactFollowUserListResponse> {
        self.inner
            .get(
                "cgi-bin/externalcontact/get_follow_user_list",
                Some(access_token.into()),
            )
            .await
    }

    pub async fn get_new_external_user_id(
        &self,
        access_token: impl Into<String>,
        external_userid: impl Into<String>,
    ) -> Result<WorkExternalUserIdConvertResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/get_new_external_userid",
                Some(access_token.into()),
                json!({ "external_userid": external_userid.into() }),
            )
            .await
    }

    pub async fn external_contact_union_id_to_external_user_id(
        &self,
        access_token: impl Into<String>,
        request: WorkUnionIdToExternalUserIdRequest,
    ) -> Result<WorkExternalUserIdConvertResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/unionid_to_external_userid",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn batch_get_external_contacts(
        &self,
        access_token: impl Into<String>,
        request: ExternalContactBatchGetRequest,
    ) -> Result<ExternalContactBatchGetResponse> {
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
    ) -> Result<ContactWayAddResponse> {
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
    ) -> Result<ContactWayGetResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/get_contact_way",
                Some(access_token.into()),
                json!({ "config_id": config_id.into() }),
            )
            .await
    }

    pub async fn list_contact_way(
        &self,
        access_token: impl Into<String>,
        request: ContactWayListRequest,
    ) -> Result<ContactWayListResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/list_contact_way",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn update_contact_way(
        &self,
        access_token: impl Into<String>,
        request: ContactWayUpdateRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/update_contact_way",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn delete_contact_way(
        &self,
        access_token: impl Into<String>,
        config_id: impl Into<String>,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/del_contact_way",
                Some(access_token.into()),
                json!({ "config_id": config_id.into() }),
            )
            .await
    }

    pub async fn close_external_temp_chat(
        &self,
        access_token: impl Into<String>,
        user_id: impl Into<String>,
        external_userid: impl Into<String>,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/close_temp_chat",
                Some(access_token.into()),
                json!({ "userid": user_id.into(), "external_userid": external_userid.into() }),
            )
            .await
    }

    pub async fn remark_external_contact(
        &self,
        access_token: impl Into<String>,
        request: ExternalContactRemarkRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/remark",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_corp_tag_list(
        &self,
        access_token: impl Into<String>,
        request: CorpTagListRequest,
    ) -> Result<CorpTagListResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/get_corp_tag_list",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn add_corp_tag(
        &self,
        access_token: impl Into<String>,
        request: CorpTagAddRequest,
    ) -> Result<CorpTagAddResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/add_corp_tag",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn edit_corp_tag(
        &self,
        access_token: impl Into<String>,
        request: CorpTagEditRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/edit_corp_tag",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn delete_corp_tag(
        &self,
        access_token: impl Into<String>,
        request: CorpTagDeleteRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/del_corp_tag",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn mark_external_contact_tag(
        &self,
        access_token: impl Into<String>,
        request: ExternalContactMarkTagRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/mark_tag",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn list_external_group_chats(
        &self,
        access_token: impl Into<String>,
        request: ExternalGroupChatListRequest,
    ) -> Result<ExternalGroupChatListResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/groupchat/list",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_external_group_chat(
        &self,
        access_token: impl Into<String>,
        chat_id: impl Into<String>,
        need_name: i64,
    ) -> Result<ExternalGroupChatGetResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/groupchat/get",
                Some(access_token.into()),
                json!({ "chat_id": chat_id.into(), "need_name": need_name }),
            )
            .await
    }

    pub async fn transfer_external_group_chat(
        &self,
        access_token: impl Into<String>,
        chat_id_list: Vec<String>,
        new_owner: impl Into<String>,
    ) -> Result<ExternalGroupChatTransferResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/groupchat/transfer",
                Some(access_token.into()),
                json!({ "chat_id_list": chat_id_list, "new_owner": new_owner.into() }),
            )
            .await
    }

    pub async fn transfer_onjob_external_group_chat(
        &self,
        access_token: impl Into<String>,
        chat_id_list: Vec<String>,
        new_owner: impl Into<String>,
    ) -> Result<ExternalGroupChatTransferResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/groupchat/onjob_transfer",
                Some(access_token.into()),
                json!({ "chat_id_list": chat_id_list, "new_owner": new_owner.into() }),
            )
            .await
    }

    pub async fn external_group_chat_open_gid_to_chat_id(
        &self,
        access_token: impl Into<String>,
        open_gid: impl Into<String>,
    ) -> Result<ExternalGroupChatOpenGidToChatIdResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/opengid_to_chatid",
                Some(access_token.into()),
                json!({ "opengid": open_gid.into() }),
            )
            .await
    }

    pub async fn add_external_group_chat_join_way(
        &self,
        access_token: impl Into<String>,
        request: ExternalGroupChatJoinWayRequest,
    ) -> Result<ExternalGroupChatJoinWayAddResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/groupchat/add_join_way",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_external_group_chat_join_way(
        &self,
        access_token: impl Into<String>,
        config_id: impl Into<String>,
    ) -> Result<ExternalGroupChatJoinWayResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/groupchat/get_join_way",
                Some(access_token.into()),
                json!({ "config_id": config_id.into() }),
            )
            .await
    }

    pub async fn update_external_group_chat_join_way(
        &self,
        access_token: impl Into<String>,
        request: ExternalGroupChatJoinWayUpdateRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/groupchat/update_join_way",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn delete_external_group_chat_join_way(
        &self,
        access_token: impl Into<String>,
        config_id: impl Into<String>,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/groupchat/del_join_way",
                Some(access_token.into()),
                json!({ "config_id": config_id.into() }),
            )
            .await
    }

    pub async fn get_new_external_group_chat_user_id(
        &self,
        access_token: impl Into<String>,
        external_userid: impl Into<String>,
    ) -> Result<WorkExternalUserIdConvertResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/groupchat/get_new_external_userid",
                Some(access_token.into()),
                json!({ "external_userid": external_userid.into() }),
            )
            .await
    }

    pub async fn list_external_contact_moment_strategies(
        &self,
        access_token: impl Into<String>,
        cursor: impl Into<String>,
        limit: i64,
    ) -> Result<ExternalContactMomentStrategyListResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/moment_strategy/list",
                Some(access_token.into()),
                json!({ "cursor": cursor.into(), "limit": limit }),
            )
            .await
    }

    pub async fn get_external_contact_moment_strategy_range(
        &self,
        access_token: impl Into<String>,
        request: ExternalContactMomentStrategyRangeRequest,
    ) -> Result<ExternalContactMomentStrategyRangeResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/moment_strategy/get_range",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn create_external_contact_moment_strategy(
        &self,
        access_token: impl Into<String>,
        request: ExternalContactMomentStrategyCreateRequest,
    ) -> Result<ExternalContactMomentStrategyCreateResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/moment_strategy/create",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn edit_external_contact_moment_strategy(
        &self,
        access_token: impl Into<String>,
        request: ExternalContactMomentStrategyEditRequest,
    ) -> Result<ExternalContactMomentStrategyCreateResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/moment_strategy/edit",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn delete_external_contact_moment_strategy(
        &self,
        access_token: impl Into<String>,
        strategy_id: i64,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/moment_strategy/del",
                Some(access_token.into()),
                json!({ "strategy_id": strategy_id }),
            )
            .await
    }

    pub async fn get_external_contact_strategy_tag_list(
        &self,
        access_token: impl Into<String>,
        request: ExternalContactStrategyTagListRequest,
    ) -> Result<ExternalContactStrategyTagListResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/get_strategy_tag_list",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn add_external_contact_strategy_tag(
        &self,
        access_token: impl Into<String>,
        request: ExternalContactStrategyTagAddRequest,
    ) -> Result<ExternalContactStrategyTagAddResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/add_strategy_tag",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn edit_external_contact_strategy_tag(
        &self,
        access_token: impl Into<String>,
        request: ExternalContactStrategyTagEditRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/edit_strategy_tag",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn delete_external_contact_strategy_tag(
        &self,
        access_token: impl Into<String>,
        request: ExternalContactStrategyTagDeleteRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/del_strategy_tag",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn add_external_group_welcome_template(
        &self,
        access_token: impl Into<String>,
        request: ExternalGroupWelcomeTemplateRequest,
    ) -> Result<ExternalGroupWelcomeTemplateAddResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/group_welcome_template/add",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn edit_external_group_welcome_template(
        &self,
        access_token: impl Into<String>,
        request: ExternalGroupWelcomeTemplateUpdateRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/group_welcome_template/edit",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_external_group_welcome_template(
        &self,
        access_token: impl Into<String>,
        template_id: impl Into<String>,
    ) -> Result<ExternalGroupWelcomeTemplateResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/group_welcome_template/get",
                Some(access_token.into()),
                json!({ "template_id": template_id.into() }),
            )
            .await
    }

    pub async fn delete_external_group_welcome_template(
        &self,
        access_token: impl Into<String>,
        template_id: impl Into<String>,
        agent_id: i64,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/group_welcome_template/del",
                Some(access_token.into()),
                json!({ "template_id": template_id.into(), "agentid": agent_id }),
            )
            .await
    }

    pub async fn list_customer_acquisition_links(
        &self,
        access_token: impl Into<String>,
        request: CustomerAcquisitionLinkListRequest,
    ) -> Result<CustomerAcquisitionLinkListResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/customer_acquisition/list_link",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_customer_acquisition_link(
        &self,
        access_token: impl Into<String>,
        link_id: impl Into<String>,
    ) -> Result<CustomerAcquisitionLinkResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/customer_acquisition/get",
                Some(access_token.into()),
                json!({ "link_id": link_id.into() }),
            )
            .await
    }

    pub async fn create_customer_acquisition_link(
        &self,
        access_token: impl Into<String>,
        request: CustomerAcquisitionLinkRequest,
    ) -> Result<CustomerAcquisitionLinkCreateResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/customer_acquisition/create_link",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn update_customer_acquisition_link(
        &self,
        access_token: impl Into<String>,
        request: CustomerAcquisitionLinkUpdateRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/customer_acquisition/update_link",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn delete_customer_acquisition_link(
        &self,
        access_token: impl Into<String>,
        link_id: impl Into<String>,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/customer_acquisition/delete_link",
                Some(access_token.into()),
                json!({ "link_id": link_id.into() }),
            )
            .await
    }

    pub async fn add_external_contact_intercept_rule(
        &self,
        access_token: impl Into<String>,
        request: ExternalContactInterceptRuleAddRequest,
    ) -> Result<ExternalContactInterceptRuleAddResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/add_intercept_rule",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn update_external_contact_intercept_rule(
        &self,
        access_token: impl Into<String>,
        request: ExternalContactInterceptRuleUpdateRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/update_intercept_rule",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn delete_external_contact_intercept_rule(
        &self,
        access_token: impl Into<String>,
        rule_id: impl Into<String>,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/del_intercept_rule",
                Some(access_token.into()),
                json!({ "rule_id": rule_id.into() }),
            )
            .await
    }

    pub async fn list_external_contact_intercept_rules(
        &self,
        access_token: impl Into<String>,
    ) -> Result<ExternalContactInterceptRuleListResponse> {
        self.inner
            .get(
                "cgi-bin/externalcontact/get_intercept_rule_list",
                Some(access_token.into()),
            )
            .await
    }

    pub async fn get_external_contact_intercept_rule(
        &self,
        access_token: impl Into<String>,
        rule_id: impl Into<String>,
    ) -> Result<ExternalContactInterceptRuleResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/get_intercept_rule",
                Some(access_token.into()),
                json!({ "rule_id": rule_id.into() }),
            )
            .await
    }

    pub async fn add_external_contact_product_album(
        &self,
        access_token: impl Into<String>,
        request: ExternalContactProductAlbumAddRequest,
    ) -> Result<ExternalContactProductAlbumAddResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/add_product_album",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn update_external_contact_product_album(
        &self,
        access_token: impl Into<String>,
        request: ExternalContactProductAlbumUpdateRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/update_product_album",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn delete_external_contact_product_album(
        &self,
        access_token: impl Into<String>,
        product_id: impl Into<String>,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/delete_product_album",
                Some(access_token.into()),
                json!({ "product_id": product_id.into() }),
            )
            .await
    }

    pub async fn list_external_contact_product_albums(
        &self,
        access_token: impl Into<String>,
        request: ExternalContactProductAlbumListRequest,
    ) -> Result<ExternalContactProductAlbumListResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/get_product_album_list",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_external_contact_product_album(
        &self,
        access_token: impl Into<String>,
        product_id: impl Into<String>,
    ) -> Result<ExternalContactProductAlbumResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/get_product_album",
                Some(access_token.into()),
                json!({ "product_id": product_id.into() }),
            )
            .await
    }

    pub async fn get_customer_acquisition_quota(
        &self,
        access_token: impl Into<String>,
    ) -> Result<CustomerAcquisitionQuotaResponse> {
        self.inner
            .get(
                "cgi-bin/externalcontact/customer_acquisition_quota",
                Some(access_token.into()),
            )
            .await
    }

    pub async fn list_customer_acquisition_customers(
        &self,
        access_token: impl Into<String>,
        request: CustomerAcquisitionCustomerListRequest,
    ) -> Result<CustomerAcquisitionCustomerListResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/customer_acquisition/customer",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_customer_acquisition_statistic(
        &self,
        access_token: impl Into<String>,
        request: CustomerAcquisitionStatisticRequest,
    ) -> Result<CustomerAcquisitionStatisticResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/customer_acquisition/statistic",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_customer_acquisition_chat_info(
        &self,
        access_token: impl Into<String>,
        request: CustomerAcquisitionChatInfoRequest,
    ) -> Result<CustomerAcquisitionChatInfoResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/customer_acquisition/get_chat_info",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn add_external_contact_message_template(
        &self,
        access_token: impl Into<String>,
        request: ExternalContactMessageTemplateRequest,
    ) -> Result<ExternalContactMessageTemplateResponse> {
        request.validate()?;
        self.inner
            .post(
                "cgi-bin/externalcontact/add_msg_template",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn list_external_contact_group_messages(
        &self,
        access_token: impl Into<String>,
        request: ExternalContactGroupMessageListRequest,
    ) -> Result<ExternalContactGroupMessageListResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/get_groupmsg_list_v2",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_external_contact_group_message_tasks(
        &self,
        access_token: impl Into<String>,
        msg_id: impl Into<String>,
        limit: i64,
        cursor: impl Into<String>,
    ) -> Result<ExternalContactGroupMessageTaskResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/get_groupmsg_task",
                Some(access_token.into()),
                json!({ "msgid": msg_id.into(), "limit": limit, "msgcursorid": cursor.into() }),
            )
            .await
    }

    pub async fn get_external_contact_group_message_send_result(
        &self,
        access_token: impl Into<String>,
        msg_id: impl Into<String>,
        user_id: impl Into<String>,
        limit: i64,
        cursor: impl Into<String>,
    ) -> Result<ExternalContactGroupMessageSendResultResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/get_groupmsg_send_result",
                Some(access_token.into()),
                json!({ "msgid": msg_id.into(), "userid": user_id.into(), "limit": limit, "cursor": cursor.into() }),
            )
            .await
    }

    pub async fn get_external_contact_group_message_result(
        &self,
        access_token: impl Into<String>,
        request: ExternalContactGroupMessageResultRequest,
    ) -> Result<ExternalContactGroupMessageResultResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/get_group_msg_result",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn send_external_contact_welcome_message(
        &self,
        access_token: impl Into<String>,
        request: ExternalContactWelcomeMessageRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/send_welcome_msg",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn remind_external_contact_group_message_send(
        &self,
        access_token: impl Into<String>,
        msg_id: impl Into<String>,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/remind_groupmsg_send",
                Some(access_token.into()),
                json!({ "msgid": msg_id.into() }),
            )
            .await
    }

    pub async fn cancel_external_contact_group_message_send(
        &self,
        access_token: impl Into<String>,
        msg_id: impl Into<String>,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/cancel_groupmsg_send",
                Some(access_token.into()),
                json!({ "msgid": msg_id.into() }),
            )
            .await
    }

    pub async fn transfer_external_customer(
        &self,
        access_token: impl Into<String>,
        request: ExternalCustomerTransferRequest,
    ) -> Result<ExternalCustomerTransferResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/transfer_customer",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn transfer_unassigned_external_contact(
        &self,
        access_token: impl Into<String>,
        request: ExternalContactUnassignedTransferRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/transfer",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn query_external_customer_transfer_result(
        &self,
        access_token: impl Into<String>,
        request: ExternalCustomerTransferResultRequest,
    ) -> Result<ExternalCustomerTransferResultResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/transfer_result",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_unassigned_external_contacts(
        &self,
        access_token: impl Into<String>,
        request: ExternalContactUnassignedListRequest,
    ) -> Result<ExternalContactUnassignedListResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/get_unassigned_list",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn transfer_resigned_external_customer(
        &self,
        access_token: impl Into<String>,
        request: ResignedExternalCustomerTransferRequest,
    ) -> Result<ExternalCustomerTransferResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/resigned/transfer_customer",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn query_resigned_external_customer_transfer_result(
        &self,
        access_token: impl Into<String>,
        request: ExternalCustomerTransferResultRequest,
    ) -> Result<ExternalCustomerTransferResultResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/resigned/transfer_result",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn list_external_contact_moments(
        &self,
        access_token: impl Into<String>,
        request: ExternalContactMomentListRequest,
    ) -> Result<ExternalContactMomentListResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/get_moment_list",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_external_contact_moment_tasks(
        &self,
        access_token: impl Into<String>,
        moment_id: impl Into<String>,
        cursor: impl Into<String>,
        limit: i64,
    ) -> Result<ExternalContactMomentTaskResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/get_moment_task",
                Some(access_token.into()),
                json!({ "moment_id": moment_id.into(), "cursor": cursor.into(), "limit": limit }),
            )
            .await
    }

    pub async fn get_external_contact_moment_customer_list(
        &self,
        access_token: impl Into<String>,
        moment_id: impl Into<String>,
        user_id: impl Into<String>,
        cursor: impl Into<String>,
        limit: i64,
    ) -> Result<ExternalContactMomentCustomerListResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/get_moment_customer_list",
                Some(access_token.into()),
                json!({ "moment_id": moment_id.into(), "userid": user_id.into(), "cursor": cursor.into(), "limit": limit }),
            )
            .await
    }

    pub async fn get_external_contact_moment_send_result(
        &self,
        access_token: impl Into<String>,
        moment_id: impl Into<String>,
        user_id: impl Into<String>,
        cursor: impl Into<String>,
        limit: i64,
    ) -> Result<ExternalContactMomentCustomerListResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/get_moment_send_result",
                Some(access_token.into()),
                json!({ "moment_id": moment_id.into(), "userid": user_id.into(), "cursor": cursor.into(), "limit": limit }),
            )
            .await
    }

    pub async fn get_external_contact_moment_comments(
        &self,
        access_token: impl Into<String>,
        moment_id: impl Into<String>,
        user_id: impl Into<String>,
    ) -> Result<ExternalContactMomentCommentResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/get_moment_comments",
                Some(access_token.into()),
                json!({ "moment_id": moment_id.into(), "userid": user_id.into() }),
            )
            .await
    }

    pub async fn add_external_contact_moment_task(
        &self,
        access_token: impl Into<String>,
        request: ExternalContactMomentTaskRequest,
    ) -> Result<ExternalContactMomentTaskCreateResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/add_moment_task",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn cancel_external_contact_moment_task(
        &self,
        access_token: impl Into<String>,
        moment_id: impl Into<String>,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/cancel_moment_task",
                Some(access_token.into()),
                json!({ "moment_id": moment_id.into() }),
            )
            .await
    }

    pub async fn get_external_contact_moment_task_result(
        &self,
        access_token: impl Into<String>,
        job_id: impl Into<String>,
    ) -> Result<ExternalContactMomentTaskResultResponse> {
        self.inner
            .get_with_query(
                "cgi-bin/externalcontact/get_moment_task_result",
                Some(access_token.into()),
                vec![("jobid".to_string(), job_id.into())],
            )
            .await
    }

    pub async fn get_external_contact_user_behavior_data(
        &self,
        access_token: impl Into<String>,
        request: ExternalContactUserBehaviorDataRequest,
    ) -> Result<ExternalContactUserBehaviorDataResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/get_user_behavior_data",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_external_group_chat_statistic(
        &self,
        access_token: impl Into<String>,
        request: ExternalGroupChatStatisticRequest,
    ) -> Result<ExternalGroupChatStatisticResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/groupchat/statistic",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_external_group_chat_statistic_by_day(
        &self,
        access_token: impl Into<String>,
        request: ExternalGroupChatStatisticByDayRequest,
    ) -> Result<ExternalGroupChatStatisticResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/groupchat/statistic_group_by_day",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn list_external_contact_customer_strategies(
        &self,
        access_token: impl Into<String>,
        cursor: impl Into<String>,
        limit: i64,
    ) -> Result<ExternalContactCustomerStrategyListResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/customer_strategy/list",
                Some(access_token.into()),
                json!({ "cursor": cursor.into(), "limit": limit }),
            )
            .await
    }

    pub async fn get_external_contact_customer_strategy(
        &self,
        access_token: impl Into<String>,
        strategy_id: i64,
    ) -> Result<ExternalContactCustomerStrategyResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/customer_strategy/get",
                Some(access_token.into()),
                json!({ "strategy_id": strategy_id }),
            )
            .await
    }

    pub async fn get_external_contact_customer_strategy_range(
        &self,
        access_token: impl Into<String>,
        strategy_id: i64,
        cursor: impl Into<String>,
        limit: i64,
    ) -> Result<ExternalContactCustomerStrategyRangeResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/customer_strategy/get_range",
                Some(access_token.into()),
                json!({ "strategy_id": strategy_id, "cursor": cursor.into(), "limit": limit }),
            )
            .await
    }

    pub async fn create_external_contact_customer_strategy(
        &self,
        access_token: impl Into<String>,
        request: ExternalContactCustomerStrategyCreateRequest,
    ) -> Result<ExternalContactCustomerStrategyCreateResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/customer_strategy/create",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn edit_external_contact_customer_strategy(
        &self,
        access_token: impl Into<String>,
        request: ExternalContactCustomerStrategyEditRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/customer_strategy/edit",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn delete_external_contact_customer_strategy(
        &self,
        access_token: impl Into<String>,
        strategy_id: i64,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/customer_strategy/del",
                Some(access_token.into()),
                json!({ "strategy_id": strategy_id }),
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
            msgtype: WorkMessageTypeKind::Text.as_code().to_string(),
            text: Some(GroupRobotTextMessage {
                content: content.into(),
                mentioned_list,
                mentioned_mobile_list: Vec::new(),
            }),
            markdown: None,
            markdown_v2: None,
            image: None,
            news: None,
            file: None,
            voice: None,
            template_card: None,
        }
    }

    pub fn group_robot_markdown(content: impl Into<String>) -> GroupRobotMessage {
        GroupRobotMessage {
            msgtype: WorkMessageTypeKind::Markdown.as_code().to_string(),
            text: None,
            markdown: Some(GroupRobotMarkdownMessage {
                content: content.into(),
            }),
            markdown_v2: None,
            image: None,
            news: None,
            file: None,
            voice: None,
            template_card: None,
        }
    }

    pub fn group_robot_markdown_v2(content: impl Into<String>) -> GroupRobotMessage {
        GroupRobotMessage {
            msgtype: WorkMessageTypeKind::MarkdownV2.as_code().to_string(),
            text: None,
            markdown: None,
            markdown_v2: Some(GroupRobotMarkdownMessage {
                content: content.into(),
            }),
            image: None,
            news: None,
            file: None,
            voice: None,
            template_card: None,
        }
    }

    pub fn group_robot_voice(media_id: impl Into<String>) -> GroupRobotMessage {
        GroupRobotMessage {
            msgtype: WorkMessageTypeKind::Voice.as_code().to_string(),
            text: None,
            markdown: None,
            markdown_v2: None,
            image: None,
            news: None,
            file: None,
            voice: Some(GroupRobotVoiceMessage {
                media_id: media_id.into(),
            }),
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

    pub async fn get_media_download(
        &self,
        access_token: impl Into<String>,
        media_id: impl Into<String>,
    ) -> Result<WorkMediaDownload> {
        let response = self
            .inner
            .get_bytes_response(
                "cgi-bin/media/get",
                Some(access_token.into()),
                vec![("media_id".to_string(), media_id.into())],
                Vec::new(),
            )
            .await?;
        Ok(response.into())
    }

    pub async fn get_media_range(
        &self,
        access_token: impl Into<String>,
        media_id: impl Into<String>,
        start: u64,
        end_inclusive: Option<u64>,
    ) -> Result<WorkMediaDownload> {
        let range = work_media_range_header(start, end_inclusive)?;
        let response = self
            .inner
            .get_bytes_response(
                "cgi-bin/media/get",
                Some(access_token.into()),
                vec![("media_id".to_string(), media_id.into())],
                vec![("range".to_string(), range)],
            )
            .await?;
        Ok(response.into())
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

    pub async fn get_jssdk_media_download(
        &self,
        access_token: impl Into<String>,
        media_id: impl Into<String>,
    ) -> Result<WorkMediaDownload> {
        let response = self
            .inner
            .get_bytes_response(
                "cgi-bin/media/get/jssdk",
                Some(access_token.into()),
                vec![("media_id".to_string(), media_id.into())],
                Vec::new(),
            )
            .await?;
        Ok(response.into())
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

    pub async fn upload_temp_media_by_url(
        &self,
        access_token: impl Into<String>,
        request: WorkMediaUploadByUrlRequest,
    ) -> Result<WorkMediaUploadByUrlResponse> {
        self.inner
            .post(
                "cgi-bin/media/upload_by_url",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_temp_media_upload_by_url_result(
        &self,
        access_token: impl Into<String>,
        job_id: impl Into<String>,
    ) -> Result<WorkMediaUploadByUrlResultResponse> {
        self.inner
            .post(
                "cgi-bin/media/get_upload_by_url_result",
                Some(access_token.into()),
                WorkMediaUploadByUrlResultRequest {
                    jobid: job_id.into(),
                },
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

    pub async fn get_message_statistics(
        &self,
        access_token: impl Into<String>,
        time_type: WorkMessageStatisticsTimeKind,
    ) -> Result<WorkMessageStatisticsResponse> {
        self.inner
            .post(
                "cgi-bin/message/get_statistics",
                Some(access_token.into()),
                WorkMessageStatisticsRequest::new(time_type),
            )
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
                text: Some(WorkTextMessage {
                    content: content.into(),
                }),
                image: None,
                voice: None,
                video: None,
                file: None,
                markdown: None,
                textcard: None,
                news: None,
                mpnews: None,
                miniprogram_notice: None,
                taskcard: None,
                template_card: None,
                safe: Some(0),
                enable_id_trans: None,
                enable_duplicate_check: None,
                duplicate_check_interval: None,
                extra: Value::Null,
            },
        )
        .await
    }

    pub async fn send_markdown_message(
        &self,
        access_token: impl Into<String>,
        audience: WorkMessageAudience,
        content: impl Into<String>,
    ) -> Result<MessageSendResponse> {
        self.send_message_payload(
            access_token,
            audience,
            "markdown",
            "markdown",
            to_value(WorkMarkdownMessage {
                content: content.into(),
            })?,
        )
        .await
    }

    pub async fn send_image_message(
        &self,
        access_token: impl Into<String>,
        audience: WorkMessageAudience,
        media_id: impl Into<String>,
    ) -> Result<MessageSendResponse> {
        self.send_media_message(access_token, audience, "image", media_id)
            .await
    }

    pub async fn send_voice_message(
        &self,
        access_token: impl Into<String>,
        audience: WorkMessageAudience,
        media_id: impl Into<String>,
    ) -> Result<MessageSendResponse> {
        self.send_media_message(access_token, audience, "voice", media_id)
            .await
    }

    pub async fn send_file_message(
        &self,
        access_token: impl Into<String>,
        audience: WorkMessageAudience,
        media_id: impl Into<String>,
    ) -> Result<MessageSendResponse> {
        self.send_media_message(access_token, audience, "file", media_id)
            .await
    }

    pub async fn send_video_message(
        &self,
        access_token: impl Into<String>,
        audience: WorkMessageAudience,
        video: WorkVideoMessage,
    ) -> Result<MessageSendResponse> {
        self.send_message_payload(access_token, audience, "video", "video", to_value(video)?)
            .await
    }

    pub async fn send_text_card_message(
        &self,
        access_token: impl Into<String>,
        audience: WorkMessageAudience,
        text_card: WorkTextCardMessage,
    ) -> Result<MessageSendResponse> {
        self.send_message_payload(
            access_token,
            audience,
            "textcard",
            "textcard",
            to_value(text_card)?,
        )
        .await
    }

    pub async fn send_news_message(
        &self,
        access_token: impl Into<String>,
        audience: WorkMessageAudience,
        articles: Vec<WorkNewsArticle>,
    ) -> Result<MessageSendResponse> {
        self.send_message_payload(
            access_token,
            audience,
            "news",
            "news",
            to_value(WorkNewsMessage { articles })?,
        )
        .await
    }

    pub async fn send_mpnews_message(
        &self,
        access_token: impl Into<String>,
        audience: WorkMessageAudience,
        articles: Vec<WorkMpNewsArticle>,
    ) -> Result<MessageSendResponse> {
        self.send_message_payload(
            access_token,
            audience,
            "mpnews",
            "mpnews",
            to_value(WorkMpNewsMessage { articles })?,
        )
        .await
    }

    pub async fn send_mini_program_notice_message(
        &self,
        access_token: impl Into<String>,
        audience: WorkMessageAudience,
        notice: WorkMiniProgramNoticeMessage,
    ) -> Result<MessageSendResponse> {
        self.send_message_payload(
            access_token,
            audience,
            "miniprogram_notice",
            "miniprogram_notice",
            to_value(notice)?,
        )
        .await
    }

    pub async fn send_template_card_message(
        &self,
        access_token: impl Into<String>,
        audience: WorkMessageAudience,
        template_card: WorkTemplateCard,
    ) -> Result<MessageSendResponse> {
        self.send_message_payload(
            access_token,
            audience,
            WorkMessageTypeKind::TemplateCard.as_code(),
            "template_card",
            to_value(template_card)?,
        )
        .await
    }

    pub async fn send_task_card_message(
        &self,
        access_token: impl Into<String>,
        audience: WorkMessageAudience,
        task_card: WorkTaskCardMessage,
    ) -> Result<MessageSendResponse> {
        self.send_message_payload(
            access_token,
            audience,
            WorkMessageTypeKind::TaskCard.as_code(),
            "taskcard",
            to_value(task_card)?,
        )
        .await
    }

    async fn send_media_message(
        &self,
        access_token: impl Into<String>,
        audience: WorkMessageAudience,
        msg_type: &str,
        media_id: impl Into<String>,
    ) -> Result<MessageSendResponse> {
        self.send_message_payload(
            access_token,
            audience,
            msg_type,
            msg_type,
            to_value(WorkMediaMessage {
                media_id: media_id.into(),
            })?,
        )
        .await
    }

    async fn send_message_payload(
        &self,
        access_token: impl Into<String>,
        audience: WorkMessageAudience,
        msg_type: &str,
        payload_key: &str,
        payload: Value,
    ) -> Result<MessageSendResponse> {
        let mut body = to_value(audience)?;
        if let Some(object) = body.as_object_mut() {
            object.insert("msgtype".to_string(), Value::String(msg_type.to_string()));
            object.insert(payload_key.to_string(), payload);
        }
        self.inner
            .post("cgi-bin/message/send", Some(access_token.into()), body)
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

    pub async fn update_template_card_message(
        &self,
        access_token: impl Into<String>,
        request: WorkTemplateCardUpdateRequest,
    ) -> Result<MessageSendResponse> {
        self.inner
            .post(
                "cgi-bin/message/update_template_card",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn update_task_card_message(
        &self,
        access_token: impl Into<String>,
        request: WorkTaskCardUpdateRequest,
    ) -> Result<WorkTaskCardUpdateResponse> {
        self.inner
            .post(
                "cgi-bin/message/update_taskcard",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn send_linked_corp_message(
        &self,
        access_token: impl Into<String>,
        request: WorkLinkedCorpMessage,
    ) -> Result<WorkLinkedCorpMessageSendResponse> {
        self.inner
            .post(
                "cgi-bin/linkedcorp/message/send",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn send_external_contact_school_message(
        &self,
        access_token: impl Into<String>,
        request: WorkExternalContactSchoolMessage,
    ) -> Result<WorkExternalContactSchoolMessageSendResponse> {
        self.inner
            .post(
                "cgi-bin/externalcontact/message/send",
                Some(access_token.into()),
                request,
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
    ) -> Result<WorkMsgAuditPermitUsersResponse> {
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
    ) -> Result<WorkMsgAuditChatDataResponse> {
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
    ) -> Result<WorkMsgAuditRoomResponse> {
        self.inner
            .post(
                "cgi-bin/msgaudit/groupchat/get",
                Some(access_token.into()),
                json!({ "roomid": room_id.into() }),
            )
            .await
    }

    pub async fn msg_audit_check_single_agree(
        &self,
        access_token: impl Into<String>,
        request: WorkMsgAuditCheckSingleAgreeRequest,
    ) -> Result<WorkMsgAuditAgreeResponse> {
        request.validate()?;
        self.inner
            .post(
                "cgi-bin/msgaudit/check_single_agree",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn msg_audit_check_room_agree(
        &self,
        access_token: impl Into<String>,
        room_id: impl Into<String>,
    ) -> Result<WorkMsgAuditAgreeResponse> {
        self.inner
            .post(
                "cgi-bin/msgaudit/check_room_agree",
                Some(access_token.into()),
                json!({ "roomid": room_id.into() }),
            )
            .await
    }

    pub async fn msg_audit_robot_info(
        &self,
        access_token: impl Into<String>,
        robot_id: impl Into<String>,
    ) -> Result<WorkMsgAuditRobotInfoResponse> {
        self.inner
            .get_with_query(
                "cgi-bin/msgaudit/get_robot_info",
                Some(access_token.into()),
                vec![("robot_id".to_string(), robot_id.into())],
            )
            .await
    }

    pub fn oa(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "work.oa")
    }

    pub async fn get_corp_checkin_option(
        &self,
        access_token: impl Into<String>,
    ) -> Result<WorkCheckinCorpOptionResponse> {
        self.inner
            .post(
                "cgi-bin/checkin/getcorpcheckinoption",
                Some(access_token.into()),
                Value::Null,
            )
            .await
    }

    pub async fn get_checkin_option(
        &self,
        access_token: impl Into<String>,
        datetime: i64,
        user_id_list: Vec<String>,
    ) -> Result<WorkCheckinOptionResponse> {
        self.inner
            .post(
                "cgi-bin/checkin/getcheckinoption",
                Some(access_token.into()),
                json!({ "datetime": datetime.to_string(), "useridlist": user_id_list }),
            )
            .await
    }

    pub async fn get_checkin_data(
        &self,
        access_token: impl Into<String>,
        request: WorkCheckinDataRequest,
    ) -> Result<WorkCheckinRecordResponse> {
        self.inner
            .post(
                "cgi-bin/checkin/getcheckindata",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_checkin_day_data(
        &self,
        access_token: impl Into<String>,
        request: WorkCheckinDateRangeRequest,
    ) -> Result<WorkCheckinDayDataResponse> {
        self.inner
            .post(
                "cgi-bin/checkin/getcheckin_daydata",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_checkin_month_data(
        &self,
        access_token: impl Into<String>,
        request: WorkCheckinDateRangeRequest,
    ) -> Result<WorkCheckinMonthDataResponse> {
        self.inner
            .post(
                "cgi-bin/checkin/getcheckin_monthdata",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_checkin_schedule_list(
        &self,
        access_token: impl Into<String>,
        request: WorkCheckinDateRangeRequest,
    ) -> Result<WorkCheckinScheduleListResponse> {
        self.inner
            .post(
                "cgi-bin/checkin/getcheckinschedulist",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn set_checkin_schedule_list(
        &self,
        access_token: impl Into<String>,
        request: WorkCheckinSetScheduleListRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/checkin/setcheckinschedulist",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn add_checkin_user_face(
        &self,
        access_token: impl Into<String>,
        user_id: impl Into<String>,
        user_face: impl Into<String>,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/checkin/addcheckinuserface",
                Some(access_token.into()),
                WorkCheckinUserFaceRequest {
                    userid: user_id.into(),
                    userface: user_face.into(),
                },
            )
            .await
    }

    pub async fn add_checkin_option(
        &self,
        access_token: impl Into<String>,
        request: WorkCheckinOptionMutationRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/checkin/add_checkin_option",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn update_checkin_option(
        &self,
        access_token: impl Into<String>,
        request: WorkCheckinOptionMutationRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/checkin/update_checkin_option",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn delete_checkin_option(
        &self,
        access_token: impl Into<String>,
        group_id: i64,
        effective_now: bool,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/checkin/del_checkin_option",
                Some(access_token.into()),
                json!({ "groupid": group_id, "effective_now": effective_now }),
            )
            .await
    }

    pub fn oa_calendar(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "work.oa.calendar")
    }

    pub fn oa_approval(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "work.oa.approval")
    }

    pub async fn get_approval_template_detail(
        &self,
        access_token: impl Into<String>,
        template_id: impl Into<String>,
    ) -> Result<WorkApprovalTemplateDetailResponse> {
        self.inner
            .post(
                "cgi-bin/oa/gettemplatedetail",
                Some(access_token.into()),
                json!({ "template_id": template_id.into() }),
            )
            .await
    }

    pub async fn create_approval_apply_event(
        &self,
        access_token: impl Into<String>,
        request: WorkApprovalApplyEventRequest,
    ) -> Result<WorkApprovalApplyEventResponse> {
        self.inner
            .post("cgi-bin/oa/applyevent", Some(access_token.into()), request)
            .await
    }

    pub async fn get_approval_info(
        &self,
        access_token: impl Into<String>,
        request: WorkApprovalInfoRequest,
    ) -> Result<WorkApprovalInfoResponse> {
        self.inner
            .post(
                "cgi-bin/oa/getapprovalinfo",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_approval_detail(
        &self,
        access_token: impl Into<String>,
        sp_no: impl Into<String>,
    ) -> Result<WorkApprovalDetailResponse> {
        self.inner
            .post(
                "cgi-bin/oa/getapprovaldetail",
                Some(access_token.into()),
                json!({ "sp_no": sp_no.into() }),
            )
            .await
    }

    pub async fn get_approval_data(
        &self,
        access_token: impl Into<String>,
        request: WorkApprovalDataRequest,
    ) -> Result<WorkApprovalDataResponse> {
        self.inner
            .post(
                "cgi-bin/corp/getapprovaldata",
                Some(access_token.into()),
                request,
            )
            .await
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

    pub async fn copy_approval_template(
        &self,
        access_token: impl Into<String>,
        open_template_id: impl Into<String>,
    ) -> Result<WorkApprovalCopyTemplateResponse> {
        self.inner
            .post(
                "cgi-bin/oa/approval/copytemplate",
                Some(access_token.into()),
                json!({ "open_template_id": open_template_id.into() }),
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
        request: WorkCalendarUpdateRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/oa/calendar/update",
                Some(access_token.into()),
                request,
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

    pub async fn get_corp_vacation_config(
        &self,
        access_token: impl Into<String>,
    ) -> Result<WorkVacationConfigResponse> {
        self.inner
            .get("cgi-bin/oa/vacation/getcorpconf", Some(access_token.into()))
            .await
    }

    pub async fn get_user_vacation_quota(
        &self,
        access_token: impl Into<String>,
        user_id: impl Into<String>,
    ) -> Result<WorkVacationQuotaResponse> {
        self.inner
            .post(
                "cgi-bin/oa/vacation/getuservacationquota",
                Some(access_token.into()),
                json!({ "userid": user_id.into() }),
            )
            .await
    }

    pub async fn set_one_user_vacation_quota(
        &self,
        access_token: impl Into<String>,
        request: WorkVacationQuotaUpdateRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/oa/vacation/setoneuserquota",
                Some(access_token.into()),
                request,
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
        request: WorkScheduleUpdateRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/oa/schedule/update",
                Some(access_token.into()),
                request,
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

    pub async fn get_schedules_by_calendar(
        &self,
        access_token: impl Into<String>,
        request: WorkScheduleByCalendarRequest,
    ) -> Result<WorkScheduleGetResponse> {
        self.inner
            .post(
                "cgi-bin/oa/schedule/get_by_calendar",
                Some(access_token.into()),
                request,
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

    pub fn oa_meeting(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "work.oa.meeting")
    }

    pub async fn create_meeting(
        &self,
        access_token: impl Into<String>,
        request: WorkMeetingCreateRequest,
    ) -> Result<WorkMeetingCreateResponse> {
        self.inner
            .post("cgi-bin/meeting/create", Some(access_token.into()), request)
            .await
    }

    pub async fn update_meeting(
        &self,
        access_token: impl Into<String>,
        request: WorkMeetingUpdateRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post("cgi-bin/meeting/update", Some(access_token.into()), request)
            .await
    }

    pub async fn cancel_meeting(
        &self,
        access_token: impl Into<String>,
        meeting_id: impl Into<String>,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/meeting/cancel",
                Some(access_token.into()),
                json!({ "meetingid": meeting_id.into() }),
            )
            .await
    }

    pub async fn get_user_meeting_ids(
        &self,
        access_token: impl Into<String>,
        request: WorkMeetingGetUserMeetingIdRequest,
    ) -> Result<WorkMeetingGetUserMeetingIdResponse> {
        self.inner
            .post(
                "cgi-bin/meeting/get_user_meetingid",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_meeting_info(
        &self,
        access_token: impl Into<String>,
        meeting_id: impl Into<String>,
    ) -> Result<WorkMeetingGetInfoResponse> {
        self.inner
            .post(
                "cgi-bin/meeting/get_info",
                Some(access_token.into()),
                json!({ "meetingid": meeting_id.into() }),
            )
            .await
    }

    pub async fn get_oa_meeting_info(
        &self,
        access_token: impl Into<String>,
        meeting_id: impl Into<String>,
    ) -> Result<WorkMeetingGetInfoResponse> {
        self.inner
            .post(
                "cgi-bin/oa/meeting/get",
                Some(access_token.into()),
                json!({ "meetingid": meeting_id.into() }),
            )
            .await
    }

    pub fn oa_meetingroom(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "work.oa.meetingroom")
    }

    pub async fn add_meeting_room(
        &self,
        access_token: impl Into<String>,
        request: WorkMeetingRoomAddRequest,
    ) -> Result<WorkMeetingRoomAddResponse> {
        self.inner
            .post(
                "cgi-bin/oa/meetingroom/add",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn list_meeting_rooms(
        &self,
        access_token: impl Into<String>,
        request: WorkMeetingRoomListRequest,
    ) -> Result<WorkMeetingRoomListResponse> {
        self.inner
            .post(
                "cgi-bin/oa/meetingroom/list",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn edit_meeting_room(
        &self,
        access_token: impl Into<String>,
        request: WorkMeetingRoomEditRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/oa/meetingroom/edit",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn delete_meeting_room(
        &self,
        access_token: impl Into<String>,
        meetingroom_id: i64,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/oa/meetingroom/del",
                Some(access_token.into()),
                json!({ "meetingroom_id": meetingroom_id }),
            )
            .await
    }

    pub async fn get_meeting_room_booking_info(
        &self,
        access_token: impl Into<String>,
        request: WorkMeetingRoomGetBookingInfoRequest,
    ) -> Result<WorkMeetingRoomGetBookingInfoResponse> {
        self.inner
            .post(
                "cgi-bin/oa/meetingroom/get_booking_info",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn book_meeting_room(
        &self,
        access_token: impl Into<String>,
        request: WorkMeetingRoomBookRequest,
    ) -> Result<WorkMeetingRoomBookResponse> {
        self.inner
            .post(
                "cgi-bin/oa/meetingroom/book",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn book_meeting_room_by_schedule(
        &self,
        access_token: impl Into<String>,
        request: WorkMeetingRoomBookByScheduleRequest,
    ) -> Result<WorkMeetingRoomLinkedBookResponse> {
        self.inner
            .post(
                "cgi-bin/oa/meetingroom/book_by_schedule",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn book_meeting_room_by_meeting(
        &self,
        access_token: impl Into<String>,
        request: WorkMeetingRoomBookByMeetingRequest,
    ) -> Result<WorkMeetingRoomLinkedBookResponse> {
        self.inner
            .post(
                "cgi-bin/oa/meetingroom/book_by_meeting",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn cancel_meeting_room_book(
        &self,
        access_token: impl Into<String>,
        request: WorkMeetingRoomCancelBookRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/oa/meetingroom/cancel_book",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_meeting_room_booking_by_id(
        &self,
        access_token: impl Into<String>,
        request: WorkMeetingRoomBookingByIdRequest,
    ) -> Result<WorkMeetingRoomBookingByIdResponse> {
        self.inner
            .post(
                "cgi-bin/oa/meetingroom/bookinfo/get",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub fn oa_wedoc(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "work.oa.wedoc")
    }

    pub async fn create_wedoc_document(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocCreateDocumentRequest,
    ) -> Result<WorkWeDocCreateDocumentResponse> {
        self.inner
            .post(
                "cgi-bin/wedoc/create_doc",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn rename_wedoc_document(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocRenameDocumentRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/wedoc/rename_doc",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn delete_wedoc_document(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocDocumentTargetRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post("cgi-bin/wedoc/del_doc", Some(access_token.into()), request)
            .await
    }

    pub async fn get_wedoc_document_base_info(
        &self,
        access_token: impl Into<String>,
        docid: impl Into<String>,
    ) -> Result<WorkWeDocDocumentBaseInfoResponse> {
        self.inner
            .post(
                "cgi-bin/wedoc/get_doc_base_info",
                Some(access_token.into()),
                json!({ "docid": docid.into() }),
            )
            .await
    }

    pub async fn share_wedoc_document(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocDocumentTargetRequest,
    ) -> Result<WorkWeDocShareDocumentResponse> {
        self.inner
            .post(
                "cgi-bin/wedoc/doc_share",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_wedoc_document_auth(
        &self,
        access_token: impl Into<String>,
        docid: impl Into<String>,
    ) -> Result<WorkWeDocDocumentAuthResponse> {
        self.inner
            .post(
                "cgi-bin/wedoc/doc_get_auth",
                Some(access_token.into()),
                json!({ "docid": docid.into() }),
            )
            .await
    }

    pub async fn modify_wedoc_document_join_rule(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocModifyJoinRuleRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/wedoc/mod_doc_join_rule",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn modify_wedoc_document_members(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocModifyMembersRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/wedoc/mod_doc_member",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn modify_wedoc_document_safety_setting(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocModifySafetySettingRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/wedoc/mod_doc_safty_setting",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn batch_add_wedoc_vip_accounts(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocVipBatchRequest,
    ) -> Result<WorkWeDocVipBatchResponse> {
        self.inner
            .post(
                "cgi-bin/wedoc/vip/batch_add",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn batch_delete_wedoc_vip_accounts(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocVipBatchRequest,
    ) -> Result<WorkWeDocVipBatchResponse> {
        self.inner
            .post(
                "cgi-bin/wedoc/vip/batch_del",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn list_wedoc_vip_accounts(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocVipListRequest,
    ) -> Result<WorkWeDocVipListResponse> {
        self.inner
            .post("cgi-bin/wedoc/vip/list", Some(access_token.into()), request)
            .await
    }

    pub async fn get_wedoc_document_data(
        &self,
        access_token: impl Into<String>,
        docid: impl Into<String>,
    ) -> Result<WorkWeDocDocumentDataResponse> {
        self.inner
            .post(
                "cgi-bin/wedoc/document/get",
                Some(access_token.into()),
                json!({ "docid": docid.into() }),
            )
            .await
    }

    pub async fn get_wedoc_content_data(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocGetContentDataRequest,
    ) -> Result<WorkWeDocContentDataResponse> {
        self.inner
            .post(
                "cgi-bin/wedoc/get_doc_data",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn modify_wedoc_content(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocModifyContentRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post("cgi-bin/wedoc/mod_doc", Some(access_token.into()), request)
            .await
    }

    pub async fn upload_wedoc_image_from_bytes(
        &self,
        access_token: impl Into<String>,
        file_name: impl Into<String>,
        data: Vec<u8>,
    ) -> Result<WorkWeDocImageUploadResponse> {
        let form = reqwest::multipart::Form::new().part(
            "media",
            reqwest::multipart::Part::bytes(data).file_name(file_name.into()),
        );
        self.inner
            .post_multipart(
                "cgi-bin/wedoc/upload_doc_image",
                Some(access_token.into()),
                Vec::new(),
                form,
            )
            .await
    }

    pub async fn add_wedoc_admin(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocAdminRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/wedoc/add_admin",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn delete_wedoc_admin(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocAdminRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/wedoc/del_admin",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn list_wedoc_admins(
        &self,
        access_token: impl Into<String>,
        docid: impl Into<String>,
    ) -> Result<WorkWeDocAdminListResponse> {
        self.inner
            .post(
                "cgi-bin/wedoc/get_admin_list",
                Some(access_token.into()),
                json!({ "docid": docid.into() }),
            )
            .await
    }

    pub async fn batch_update_wedoc_document(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocDocumentBatchUpdateRequest,
    ) -> Result<WorkStatusResponse> {
        request.validate()?;
        self.inner
            .post(
                "cgi-bin/wedoc/document/batch_update",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_wedoc_spreadsheet_properties(
        &self,
        access_token: impl Into<String>,
        docid: impl Into<String>,
    ) -> Result<WorkWeDocSpreadsheetPropertiesResponse> {
        self.inner
            .post(
                "cgi-bin/wedoc/spreadsheet/get_sheet_properties",
                Some(access_token.into()),
                json!({ "docid": docid.into() }),
            )
            .await
    }

    pub async fn get_wedoc_spreadsheet_range_data(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocSpreadsheetRangeRequest,
    ) -> Result<WorkWeDocSpreadsheetRangeResponse> {
        self.inner
            .post(
                "cgi-bin/wedoc/spreadsheet/get_sheet_range_data",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn batch_update_wedoc_spreadsheet(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocSpreadsheetBatchUpdateRequest,
    ) -> Result<WorkWeDocSpreadsheetBatchUpdateResponse> {
        self.inner
            .post(
                "cgi-bin/wedoc/spreadsheet/batch_update",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn add_wedoc_smartsheet(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocSmartSheetAddRequest,
    ) -> Result<WorkWeDocSmartSheetAddResponse> {
        request.validate()?;
        self.inner
            .post(
                "cgi-bin/wedoc/smartsheet/add_sheet",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_wedoc_smartsheets(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocSmartSheetGetRequest,
    ) -> Result<WorkWeDocSmartSheetGetResponse> {
        request.validate()?;
        self.inner
            .post(
                "cgi-bin/wedoc/smartsheet/get_sheet",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn update_wedoc_smartsheet(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocSmartSheetUpdateRequest,
    ) -> Result<WorkStatusResponse> {
        request.validate()?;
        self.inner
            .post(
                "cgi-bin/wedoc/smartsheet/update_sheet",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn delete_wedoc_smartsheet(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocSmartSheetDeleteRequest,
    ) -> Result<WorkStatusResponse> {
        request.validate()?;
        self.inner
            .post(
                "cgi-bin/wedoc/smartsheet/delete_sheet",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn add_wedoc_smartsheet_view(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocSmartSheetAddViewRequest,
    ) -> Result<WorkWeDocSmartSheetViewResponse> {
        request.validate()?;
        self.inner
            .post(
                "cgi-bin/wedoc/smartsheet/add_view",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_wedoc_smartsheet_views(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocSmartSheetGetViewsRequest,
    ) -> Result<WorkWeDocSmartSheetGetViewsResponse> {
        request.validate()?;
        self.inner
            .post(
                "cgi-bin/wedoc/smartsheet/get_views",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn update_wedoc_smartsheet_view(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocSmartSheetUpdateViewRequest,
    ) -> Result<WorkWeDocSmartSheetViewResponse> {
        request.validate()?;
        self.inner
            .post(
                "cgi-bin/wedoc/smartsheet/update_view",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn delete_wedoc_smartsheet_views(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocSmartSheetDeleteViewsRequest,
    ) -> Result<WorkStatusResponse> {
        request.validate()?;
        self.inner
            .post(
                "cgi-bin/wedoc/smartsheet/delete_views",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn add_wedoc_smartsheet_fields(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocSmartSheetFieldsMutationRequest,
    ) -> Result<WorkWeDocSmartSheetFieldsResponse> {
        request.validate_for_add()?;
        self.inner
            .post(
                "cgi-bin/wedoc/smartsheet/add_fields",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_wedoc_smartsheet_fields(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocSmartSheetGetFieldsRequest,
    ) -> Result<WorkWeDocSmartSheetGetFieldsResponse> {
        self.inner
            .post(
                "cgi-bin/wedoc/smartsheet/get_fields",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn update_wedoc_smartsheet_fields(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocSmartSheetFieldsMutationRequest,
    ) -> Result<WorkWeDocSmartSheetFieldsResponse> {
        request.validate_for_update()?;
        self.inner
            .post(
                "cgi-bin/wedoc/smartsheet/update_fields",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn delete_wedoc_smartsheet_fields(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocSmartSheetDeleteFieldsRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/wedoc/smartsheet/delete_fields",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn add_wedoc_smartsheet_field_group(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocSmartSheetAddFieldGroupRequest,
    ) -> Result<WorkWeDocSmartSheetFieldGroupResponse> {
        self.inner
            .post(
                "cgi-bin/wedoc/smartsheet/add_field_group",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn update_wedoc_smartsheet_field_group(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocSmartSheetUpdateFieldGroupRequest,
    ) -> Result<WorkWeDocSmartSheetFieldGroupResponse> {
        self.inner
            .post(
                "cgi-bin/wedoc/smartsheet/update_field_group",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_wedoc_smartsheet_field_groups(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocSmartSheetGetFieldGroupsRequest,
    ) -> Result<WorkWeDocSmartSheetGetFieldGroupsResponse> {
        self.inner
            .post(
                "cgi-bin/wedoc/smartsheet/get_field_groups",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn delete_wedoc_smartsheet_field_groups(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocSmartSheetDeleteFieldGroupsRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/wedoc/smartsheet/delete_field_groups",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_wedoc_smartsheet_privileges(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocSmartSheetGetPrivilegesRequest,
    ) -> Result<WorkWeDocSmartSheetGetPrivilegesResponse> {
        self.inner
            .post(
                "cgi-bin/wedoc/smartsheet/content_priv/get_sheet_priv",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_wedoc_smartsheet_auth(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocSmartSheetAuthRequest,
    ) -> Result<WorkWeDocSmartSheetAuthResponse> {
        self.inner
            .post(
                "cgi-bin/wedoc/smartsheet/get_sheet_auth",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn modify_wedoc_smartsheet_auth(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocSmartSheetModifyAuthRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/wedoc/smartsheet/mod_sheet_auth",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn add_wedoc_smartsheet_records(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocSmartSheetRecordsMutationRequest,
    ) -> Result<WorkWeDocSmartSheetRecordsResponse> {
        request.validate_for_add()?;
        self.inner
            .post(
                "cgi-bin/wedoc/smartsheet/add_records",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_wedoc_smartsheet_records(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocSmartSheetGetRecordsRequest,
    ) -> Result<WorkWeDocSmartSheetGetRecordsResponse> {
        request.validate()?;
        self.inner
            .post(
                "cgi-bin/wedoc/smartsheet/get_records",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn update_wedoc_smartsheet_records(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocSmartSheetRecordsMutationRequest,
    ) -> Result<WorkWeDocSmartSheetRecordsResponse> {
        request.validate_for_update()?;
        self.inner
            .post(
                "cgi-bin/wedoc/smartsheet/update_records",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn delete_wedoc_smartsheet_records(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocSmartSheetDeleteRecordsRequest,
    ) -> Result<WorkStatusResponse> {
        request.validate()?;
        self.inner
            .post(
                "cgi-bin/wedoc/smartsheet/delete_records",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn create_wedoc_form(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocCreateFormRequest,
    ) -> Result<WorkWeDocCreateFormResponse> {
        self.inner
            .post(
                "cgi-bin/wedoc/create_collect",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn modify_wedoc_form(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocModifyFormRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/wedoc/modify_collect",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_wedoc_form_info(
        &self,
        access_token: impl Into<String>,
        formid: impl Into<String>,
    ) -> Result<WorkWeDocFormInfoResponse> {
        self.inner
            .post(
                "cgi-bin/wedoc/get_form_info",
                Some(access_token.into()),
                json!({ "formid": formid.into() }),
            )
            .await
    }

    pub async fn get_wedoc_form_statistics(
        &self,
        access_token: impl Into<String>,
        requests: Vec<WorkWeDocFormStatisticRequest>,
    ) -> Result<WorkWeDocFormStatisticsResponse> {
        self.inner
            .post(
                "cgi-bin/wedoc/get_form_statistic",
                Some(access_token.into()),
                requests,
            )
            .await
    }

    pub async fn get_wedoc_form_answers(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocFormAnswerRequest,
    ) -> Result<WorkWeDocFormAnswersResponse> {
        self.inner
            .post(
                "cgi-bin/wedoc/get_form_answer",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub fn oa_living(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "work.oa.living")
    }

    pub async fn create_living(
        &self,
        access_token: impl Into<String>,
        request: WorkLivingCreateRequest,
    ) -> Result<WorkLivingCreateResponse> {
        self.inner
            .post("cgi-bin/living/create", Some(access_token.into()), request)
            .await
    }

    pub async fn modify_living(
        &self,
        access_token: impl Into<String>,
        request: WorkLivingModifyRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post("cgi-bin/living/modify", Some(access_token.into()), request)
            .await
    }

    pub async fn cancel_living(
        &self,
        access_token: impl Into<String>,
        living_id: impl Into<String>,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/living/cancel",
                Some(access_token.into()),
                json!({ "livingid": living_id.into() }),
            )
            .await
    }

    pub async fn delete_living_replay_data(
        &self,
        access_token: impl Into<String>,
        living_id: impl Into<String>,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/living/delete_replay_data",
                Some(access_token.into()),
                json!({ "livingid": living_id.into() }),
            )
            .await
    }

    pub async fn get_living_code(
        &self,
        access_token: impl Into<String>,
        living_id: impl Into<String>,
        open_id: impl Into<String>,
    ) -> Result<WorkLivingCodeResponse> {
        self.inner
            .post(
                "cgi-bin/living/get_living_code",
                Some(access_token.into()),
                json!({ "livingid": living_id.into(), "openid": open_id.into() }),
            )
            .await
    }

    pub async fn get_user_all_living_ids(
        &self,
        access_token: impl Into<String>,
        request: WorkLivingGetUserAllLivingIdRequest,
    ) -> Result<WorkLivingGetUserAllLivingIdResponse> {
        self.inner
            .post(
                "cgi-bin/living/get_user_all_livingid",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn get_living_info(
        &self,
        access_token: impl Into<String>,
        living_id: impl Into<String>,
    ) -> Result<WorkLivingInfoResponse> {
        self.inner
            .get_with_query(
                "cgi-bin/living/get_living_info",
                Some(access_token.into()),
                vec![("livingid".to_string(), living_id.into())],
            )
            .await
    }

    pub async fn get_living_watch_stat(
        &self,
        access_token: impl Into<String>,
        living_id: impl Into<String>,
        next_key: impl Into<String>,
    ) -> Result<WorkLivingWatchStatResponse> {
        self.inner
            .post(
                "cgi-bin/living/get_watch_stat",
                Some(access_token.into()),
                json!({ "livingid": living_id.into(), "next_key": next_key.into() }),
            )
            .await
    }

    pub async fn get_living_share_info(
        &self,
        access_token: impl Into<String>,
        ww_share_code: impl Into<String>,
    ) -> Result<WorkLivingShareInfoResponse> {
        self.inner
            .post(
                "cgi-bin/living/get_living_share_info",
                Some(access_token.into()),
                json!({ "ww_share_code": ww_share_code.into() }),
            )
            .await
    }

    pub fn oa_wedrive(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "work.oa.wedrive")
    }

    pub async fn wedrive_space_create(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDriveSpaceCreateRequest,
    ) -> Result<WorkWeDriveSpaceCreateResponse> {
        self.inner
            .post(
                "cgi-bin/wedrive/space_create",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn wedrive_space_rename(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDriveSpaceRenameRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/wedrive/space_rename",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn wedrive_space_dismiss(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDriveSpaceIdRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/wedrive/space_dismiss",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn wedrive_space_info(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDriveSpaceIdRequest,
    ) -> Result<WorkWeDriveSpaceInfoResponse> {
        self.inner
            .post(
                "cgi-bin/wedrive/space_info",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn wedrive_new_space_info(
        &self,
        access_token: impl Into<String>,
        space_id: impl Into<String>,
    ) -> Result<WorkWeDriveSpaceInfoResponse> {
        self.inner
            .post(
                "cgi-bin/wedrive/new_space_info",
                Some(access_token.into()),
                json!({ "spaceid": space_id.into() }),
            )
            .await
    }

    pub async fn wedrive_space_acl_add(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDriveSpaceAclRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/wedrive/space_acl_add",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn wedrive_space_acl_del(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDriveSpaceAclRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/wedrive/space_acl_del",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn wedrive_space_setting(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDriveSpaceSettingRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/wedrive/space_setting",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn wedrive_space_share(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDriveSpaceIdRequest,
    ) -> Result<WorkWeDriveSpaceShareResponse> {
        self.inner
            .post(
                "cgi-bin/wedrive/space_share",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn wedrive_file_list(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDriveFileListRequest,
    ) -> Result<WorkWeDriveFileListResponse> {
        self.inner
            .post(
                "cgi-bin/wedrive/file_list",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn wedrive_file_upload(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDriveFileUploadRequest,
    ) -> Result<WorkWeDriveFileUploadResponse> {
        self.inner
            .post(
                "cgi-bin/wedrive/file_upload",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn wedrive_file_download(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDriveFileIdRequest,
    ) -> Result<WorkWeDriveFileDownloadResponse> {
        self.inner
            .post(
                "cgi-bin/wedrive/file_download",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn wedrive_file_create(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDriveFileCreateRequest,
    ) -> Result<WorkWeDriveFileCreateResponse> {
        self.inner
            .post(
                "cgi-bin/wedrive/file_create",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn wedrive_file_rename(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDriveFileRenameRequest,
    ) -> Result<WorkWeDriveFileRenameResponse> {
        self.inner
            .post(
                "cgi-bin/wedrive/file_rename",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn wedrive_file_move(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDriveFileMoveRequest,
    ) -> Result<WorkWeDriveFileMoveResponse> {
        self.inner
            .post(
                "cgi-bin/wedrive/file_move",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn wedrive_file_delete(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDriveFileIdRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/wedrive/file_delete",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn wedrive_file_info(
        &self,
        access_token: impl Into<String>,
        file_id: impl Into<String>,
    ) -> Result<WorkWeDriveFileInfoResponse> {
        self.inner
            .post(
                "cgi-bin/wedrive/file_info",
                Some(access_token.into()),
                json!({ "fileid": file_id.into() }),
            )
            .await
    }

    pub async fn wedrive_file_acl_add(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDriveFileAclRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/wedrive/file_acl_add",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn wedrive_file_acl_del(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDriveFileAclRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/wedrive/file_acl_del",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn wedrive_file_setting(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDriveFileSettingRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/wedrive/file_setting",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn wedrive_file_share(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDriveFileIdRequest,
    ) -> Result<WorkWeDriveFileShareResponse> {
        self.inner
            .post(
                "cgi-bin/wedrive/file_share",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub fn account_service(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "work.accountService")
    }

    pub async fn account_service_account_add(
        &self,
        access_token: impl Into<String>,
        name: impl Into<String>,
        media_id: impl Into<String>,
    ) -> Result<WorkAccountServiceAccountAddResponse> {
        self.inner
            .post(
                "cgi-bin/kf/account/add",
                Some(access_token.into()),
                json!({ "name": name.into(), "media_id": media_id.into() }),
            )
            .await
    }

    pub async fn account_service_account_delete(
        &self,
        access_token: impl Into<String>,
        open_kfid: impl Into<String>,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/kf/account/del",
                Some(access_token.into()),
                json!({ "open_kfid": open_kfid.into() }),
            )
            .await
    }

    pub async fn account_service_account_update(
        &self,
        access_token: impl Into<String>,
        request: WorkAccountServiceAccountUpdateRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/kf/account/update",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn account_service_account_list(
        &self,
        access_token: impl Into<String>,
    ) -> Result<WorkAccountServiceAccountListResponse> {
        self.inner
            .post(
                "cgi-bin/kf/account/list",
                Some(access_token.into()),
                json!({}),
            )
            .await
    }

    pub async fn account_service_add_contact_way(
        &self,
        access_token: impl Into<String>,
        request: WorkAccountServiceAddContactWayRequest,
    ) -> Result<WorkAccountServiceAddContactWayResponse> {
        self.inner
            .post(
                "cgi-bin/kf/add_contact_way",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub fn account_service_customer(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "work.accountService.customer")
    }

    pub async fn account_service_customer_batch_get(
        &self,
        access_token: impl Into<String>,
        request: WorkAccountServiceCustomerBatchGetRequest,
    ) -> Result<WorkAccountServiceCustomerBatchGetResponse> {
        self.inner
            .post(
                "cgi-bin/kf/customer/batchget",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn account_service_customer_get_upgrade_service_config(
        &self,
        access_token: impl Into<String>,
    ) -> Result<WorkAccountServiceCustomerUpgradeServiceConfigResponse> {
        self.inner
            .get(
                "cgi-bin/kf/customer/get_upgrade_service_config",
                Some(access_token.into()),
            )
            .await
    }

    pub async fn account_service_customer_upgrade_service(
        &self,
        access_token: impl Into<String>,
        request: WorkAccountServiceCustomerUpgradeServiceRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/kf/customer/upgrade_service",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn account_service_customer_cancel_upgrade_service(
        &self,
        access_token: impl Into<String>,
        open_kfid: impl Into<String>,
        external_userid: impl Into<String>,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/kf/customer/cancel_upgrade_service",
                Some(access_token.into()),
                json!({ "open_kfid": open_kfid.into(), "external_userid": external_userid.into() }),
            )
            .await
    }

    pub fn account_service_message(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "work.accountService.message")
    }

    pub async fn account_service_sync_msg(
        &self,
        access_token: impl Into<String>,
        request: WorkAccountServiceSyncMsgRequest,
    ) -> Result<WorkAccountServiceSyncMsgResponse> {
        self.inner
            .post("cgi-bin/kf/sync_msg", Some(access_token.into()), request)
            .await
    }

    pub async fn account_service_send_msg(
        &self,
        access_token: impl Into<String>,
        request: WorkAccountServiceSendMsgRequest,
    ) -> Result<WorkAccountServiceSendMsgResponse> {
        self.inner
            .post("cgi-bin/kf/send_msg", Some(access_token.into()), request)
            .await
    }

    pub async fn account_service_send_msg_on_event(
        &self,
        access_token: impl Into<String>,
        request: WorkAccountServiceSendMsgOnEventRequest,
    ) -> Result<WorkAccountServiceSendMsgResponse> {
        self.inner
            .post(
                "cgi-bin/kf/send_msg_on_event",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub fn account_service_servicer(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "work.accountService.servicer")
    }

    pub async fn account_service_servicer_add(
        &self,
        access_token: impl Into<String>,
        open_kfid: impl Into<String>,
        userid_list: Vec<String>,
    ) -> Result<WorkAccountServiceServicerResultResponse> {
        self.account_service_servicer_add_with_request(
            access_token,
            WorkAccountServiceServicerRequest::new(open_kfid, userid_list),
        )
        .await
    }

    pub async fn account_service_servicer_add_with_request(
        &self,
        access_token: impl Into<String>,
        request: WorkAccountServiceServicerRequest,
    ) -> Result<WorkAccountServiceServicerResultResponse> {
        self.inner
            .post(
                "cgi-bin/kf/servicer/add",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn account_service_servicer_delete(
        &self,
        access_token: impl Into<String>,
        open_kfid: impl Into<String>,
        userid_list: Vec<String>,
    ) -> Result<WorkAccountServiceServicerResultResponse> {
        self.account_service_servicer_delete_with_request(
            access_token,
            WorkAccountServiceServicerRequest::new(open_kfid, userid_list),
        )
        .await
    }

    pub async fn account_service_servicer_delete_with_request(
        &self,
        access_token: impl Into<String>,
        request: WorkAccountServiceServicerRequest,
    ) -> Result<WorkAccountServiceServicerResultResponse> {
        self.inner
            .post(
                "cgi-bin/kf/servicer/del",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn account_service_servicer_list(
        &self,
        access_token: impl Into<String>,
        open_kfid: impl Into<String>,
    ) -> Result<WorkAccountServiceServicerListResponse> {
        self.inner
            .post_json_with_access_token_query(
                "cgi-bin/kf/servicer/list",
                Some(access_token.into()),
                vec![("open_kfid".to_string(), open_kfid.into())],
                json!({}),
                Vec::new(),
            )
            .await
    }

    pub fn account_service_state(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "work.accountService.serviceState")
    }

    pub async fn account_service_state_get(
        &self,
        access_token: impl Into<String>,
        open_kfid: impl Into<String>,
        external_userid: impl Into<String>,
    ) -> Result<WorkAccountServiceStateGetResponse> {
        self.inner
            .post(
                "cgi-bin/kf/service_state/get",
                Some(access_token.into()),
                json!({ "open_kfid": open_kfid.into(), "external_userid": external_userid.into() }),
            )
            .await
    }

    pub async fn account_service_state_trans(
        &self,
        access_token: impl Into<String>,
        request: WorkAccountServiceStateTransRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/kf/service_state/trans",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn account_service_state_trans_with_response(
        &self,
        access_token: impl Into<String>,
        request: WorkAccountServiceStateTransRequest,
    ) -> Result<WorkAccountServiceStateTransResponse> {
        self.inner
            .post(
                "cgi-bin/kf/service_state/trans",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub fn account_service_tag(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "work.accountService.tag")
    }

    pub async fn account_service_tag_create(
        &self,
        access_token: impl Into<String>,
        tagname: impl Into<String>,
        tagid: i64,
    ) -> Result<WorkAccountServiceTagCreateResponse> {
        self.inner
            .post(
                "cgi-bin/tag/create",
                Some(access_token.into()),
                json!({ "tagname": tagname.into(), "tagid": tagid }),
            )
            .await
    }

    pub async fn account_service_tag_update(
        &self,
        access_token: impl Into<String>,
        tagname: impl Into<String>,
        tagid: i64,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post(
                "cgi-bin/tag/update",
                Some(access_token.into()),
                json!({ "tagname": tagname.into(), "tagid": tagid }),
            )
            .await
    }

    pub async fn account_service_tag_delete(
        &self,
        access_token: impl Into<String>,
        tagid: impl Into<String>,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .get_with_query(
                "cgi-bin/tag/delete",
                Some(access_token.into()),
                vec![("tagid".to_string(), tagid.into())],
            )
            .await
    }

    pub async fn account_service_tag_get(
        &self,
        access_token: impl Into<String>,
        tagid: impl Into<String>,
    ) -> Result<WorkAccountServiceTagDetailResponse> {
        self.inner
            .get_with_query(
                "cgi-bin/tag/get",
                Some(access_token.into()),
                vec![("tagid".to_string(), tagid.into())],
            )
            .await
    }

    pub async fn account_service_tag_users(
        &self,
        access_token: impl Into<String>,
        tagid: i64,
        userlist: Vec<String>,
    ) -> Result<WorkAccountServiceTagUserResultResponse> {
        self.account_service_tag_or_untag_users(
            access_token,
            "cgi-bin/tag/addtagusers",
            tagid,
            userlist,
            Vec::new(),
        )
        .await
    }

    pub async fn account_service_tag_departments(
        &self,
        access_token: impl Into<String>,
        tagid: i64,
        partylist: Vec<String>,
    ) -> Result<WorkAccountServiceTagUserResultResponse> {
        self.account_service_tag_or_untag_users(
            access_token,
            "cgi-bin/tag/addtagusers",
            tagid,
            Vec::new(),
            partylist,
        )
        .await
    }

    pub async fn account_service_tag_or_untag_users(
        &self,
        access_token: impl Into<String>,
        endpoint: impl Into<String>,
        tagid: i64,
        userlist: Vec<String>,
        partylist: Vec<String>,
    ) -> Result<WorkAccountServiceTagUserResultResponse> {
        self.inner
            .post(
                endpoint.into(),
                Some(access_token.into()),
                json!({ "tagid": tagid, "userlist": userlist, "partylist": partylist }),
            )
            .await
    }

    pub async fn remove_users_from_tag(
        &self,
        access_token: impl Into<String>,
        tagid: i64,
        userlist: Vec<String>,
        partylist: Vec<String>,
    ) -> Result<WorkAccountServiceTagUserResultResponse> {
        self.account_service_tag_or_untag_users(
            access_token,
            "cgi-bin/tag/deltagusers",
            tagid,
            userlist,
            partylist,
        )
        .await
    }

    pub async fn account_service_tag_list(
        &self,
        access_token: impl Into<String>,
    ) -> Result<WorkAccountServiceTagListResponse> {
        self.inner
            .get("cgi-bin/tag/list", Some(access_token.into()))
            .await
    }

    pub fn aibot(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "work.aibot")
    }

    pub fn aibot_long_connection_url(endpoint: Option<&str>) -> String {
        endpoint
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .unwrap_or("wss://openws.work.weixin.qq.com")
            .to_string()
    }

    pub fn aibot_subscribe_request(
        bot_id: impl Into<String>,
        secret: impl Into<String>,
        req_id: impl Into<String>,
    ) -> WorkAiBotLongConnectionRequest {
        WorkAiBotLongConnectionRequest {
            cmd: WORK_AIBOT_CMD_SUBSCRIBE.to_string(),
            headers: Some(WorkAiBotLongConnectionHeaders {
                req_id: Some(req_id.into()),
            }),
            body: Some(json!({ "bot_id": bot_id.into(), "secret": secret.into() })),
        }
    }

    pub fn aibot_ping_request(req_id: impl Into<String>) -> WorkAiBotLongConnectionRequest {
        WorkAiBotLongConnectionRequest {
            cmd: WORK_AIBOT_CMD_PING.to_string(),
            headers: Some(WorkAiBotLongConnectionHeaders {
                req_id: Some(req_id.into()),
            }),
            body: None,
        }
    }

    pub fn aibot_command_request(
        cmd: impl Into<String>,
        req_id: Option<String>,
        body: Option<Value>,
    ) -> WorkAiBotLongConnectionRequest {
        WorkAiBotLongConnectionRequest {
            cmd: cmd.into().trim().to_string(),
            headers: req_id.map(|req_id| WorkAiBotLongConnectionHeaders {
                req_id: Some(req_id),
            }),
            body,
        }
    }

    pub async fn appchat_create(
        &self,
        access_token: impl Into<String>,
        request: AppChatCreateRequest,
    ) -> Result<WorkAppChatCreateResponse> {
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
    ) -> Result<WorkAppChatGetResponse> {
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

    pub async fn send_group_robot_message(
        &self,
        key: impl Into<String>,
        request: GroupRobotMessage,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post_json_with_query(
                "cgi-bin/webhook/send",
                vec![("key".to_string(), key.into())],
                serde_json::to_value(request).expect("work group robot message serializes"),
                Vec::new(),
            )
            .await
    }

    pub async fn upload_group_robot_media_from_bytes(
        &self,
        key: impl Into<String>,
        file_name: impl Into<String>,
        data: Vec<u8>,
    ) -> Result<WorkUploadMediaResponse> {
        let form = reqwest::multipart::Form::new().part(
            "media",
            reqwest::multipart::Part::bytes(data).file_name(file_name.into()),
        );
        self.inner
            .post_multipart(
                "cgi-bin/webhook/upload_media",
                None,
                vec![
                    ("key".to_string(), key.into()),
                    ("type".to_string(), "file".to_string()),
                ],
                form,
            )
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
    ) -> Result<WorkOauthUserInfoResponse> {
        self.inner
            .get_with_query(
                "cgi-bin/user/getuserinfo",
                Some(access_token.into()),
                vec![("code".to_string(), code.into())],
            )
            .await
    }

    pub async fn auth_user_info(
        &self,
        access_token: impl Into<String>,
        code: impl Into<String>,
    ) -> Result<WorkOauthUserInfoResponse> {
        self.inner
            .get_with_query(
                "cgi-bin/auth/getuserinfo",
                Some(access_token.into()),
                vec![("code".to_string(), code.into())],
            )
            .await
    }

    pub async fn oauth_user_detail(
        &self,
        access_token: impl Into<String>,
        user_ticket: impl Into<String>,
    ) -> Result<WorkOauthUserDetailResponse> {
        self.inner
            .post(
                "cgi-bin/user/getuserdetail",
                Some(access_token.into()),
                json!({ "user_ticket": user_ticket.into() }),
            )
            .await
    }

    pub async fn auth_user_detail(
        &self,
        access_token: impl Into<String>,
        user_ticket: impl Into<String>,
    ) -> Result<WorkOauthUserDetailResponse> {
        self.inner
            .post(
                "cgi-bin/auth/getuserdetail",
                Some(access_token.into()),
                json!({ "user_ticket": user_ticket.into() }),
            )
            .await
    }

    pub async fn get_user_tfa_info(
        &self,
        access_token: impl Into<String>,
        request: WorkUserTfaInfoRequest,
    ) -> Result<WorkUserTfaInfoResponse> {
        self.inner
            .post(
                "cgi-bin/auth/get_tfa_info",
                Some(access_token.into()),
                request,
            )
            .await
    }

    pub async fn submit_user_tfa_success(
        &self,
        access_token: impl Into<String>,
        request: WorkUserTfaSuccessRequest,
    ) -> Result<WorkStatusResponse> {
        self.inner
            .post("cgi-bin/user/tfa_succ", Some(access_token.into()), request)
            .await
    }

    pub fn server(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "work.server")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAgentListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub agentlist: Vec<WorkAgentSummary>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAgentSummary {
    #[serde(default)]
    pub agentid: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub square_logo_url: Option<String>,
    #[serde(default)]
    pub round_logo_url: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAgentDetailResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub agentid: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub square_logo_url: Option<String>,
    #[serde(default)]
    pub round_logo_url: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub allow_userinfos: Option<WorkAgentAllowUsers>,
    #[serde(default)]
    pub allow_partys: Option<WorkAgentAllowParties>,
    #[serde(default)]
    pub allow_tags: Option<WorkAgentAllowTags>,
    #[serde(default)]
    pub close: Option<i64>,
    #[serde(default)]
    pub redirect_domain: Option<String>,
    #[serde(default)]
    pub report_location_flag: Option<i64>,
    #[serde(default)]
    pub isreportenter: Option<i64>,
    #[serde(default)]
    pub home_url: Option<String>,
    #[serde(default)]
    pub customized_publish_status: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAgentAllowUsers {
    #[serde(default)]
    pub user: Vec<WorkAgentAllowUser>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAgentAllowUser {
    #[serde(default)]
    pub userid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAgentAllowParties {
    #[serde(default)]
    pub partyid: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAgentAllowTags {
    #[serde(default)]
    pub tagid: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAgentWorkbenchTemplateResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default, rename = "type")]
    pub template_type: Option<String>,
    #[serde(default)]
    pub keydata: Option<WorkAgentWorkbenchKeyDataTemplate>,
    #[serde(default)]
    pub image: Option<WorkAgentWorkbenchImageTemplate>,
    #[serde(default)]
    pub list: Option<WorkAgentWorkbenchListTemplate>,
    #[serde(default)]
    pub webview: Option<WorkAgentWorkbenchWebviewTemplate>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAgentWorkbenchKeyDataTemplate {
    #[serde(default)]
    pub items: Vec<WorkAgentWorkbenchKeyDataItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAgentWorkbenchKeyDataItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jump_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pagepath: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAgentWorkbenchImageTemplate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jump_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pagepath: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAgentWorkbenchListTemplate {
    #[serde(default)]
    pub items: Vec<WorkAgentWorkbenchListItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAgentWorkbenchListItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jump_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pagepath: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAgentWorkbenchWebviewTemplate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkAgentScopeRequest {
    pub agentid: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_userinfos: Option<WorkAgentAllowUsers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_partys: Option<WorkAgentAllowParties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_tags: Option<WorkAgentAllowTags>,
}

impl WorkAgentScopeRequest {
    pub fn new(agentid: i64) -> Self {
        Self {
            agentid,
            allow_userinfos: None,
            allow_partys: None,
            allow_tags: None,
        }
    }

    pub fn with_users(mut self, users: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.allow_userinfos = Some(WorkAgentAllowUsers {
            user: users
                .into_iter()
                .map(|userid| WorkAgentAllowUser {
                    userid: Some(userid.into()),
                })
                .collect(),
        });
        self
    }

    pub fn with_parties(mut self, party_ids: impl IntoIterator<Item = i64>) -> Self {
        self.allow_partys = Some(WorkAgentAllowParties {
            partyid: party_ids.into_iter().collect(),
        });
        self
    }

    pub fn with_tags(mut self, tag_ids: impl IntoIterator<Item = i64>) -> Self {
        self.allow_tags = Some(WorkAgentAllowTags {
            tagid: tag_ids.into_iter().collect(),
        });
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAgentWorkbenchTemplateRequest {
    pub agentid: i64,
    #[serde(rename = "type")]
    pub template_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keydata: Option<WorkAgentWorkbenchKeyDataTemplate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<WorkAgentWorkbenchImageTemplate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<WorkAgentWorkbenchListTemplate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webview: Option<WorkAgentWorkbenchWebviewTemplate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAgentWorkbenchDataRequest {
    pub agentid: i64,
    pub userid: String,
    #[serde(rename = "type")]
    pub template_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keydata: Option<WorkAgentWorkbenchKeyDataTemplate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<WorkAgentWorkbenchImageTemplate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<WorkAgentWorkbenchListTemplate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webview: Option<WorkAgentWorkbenchWebviewTemplate>,
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
pub struct WorkDepartmentListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub department: Vec<WorkDepartment>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkDepartmentSimpleListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub department_id: Vec<WorkDepartmentSimple>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkDepartmentDetailResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(flatten)]
    pub department: WorkDepartment,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkDepartment {
    #[serde(default)]
    pub id: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub name_en: Option<String>,
    #[serde(default)]
    pub parentid: Option<i64>,
    #[serde(default)]
    pub order: Option<i64>,
    #[serde(default)]
    pub department_leader: Vec<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkDepartmentSimple {
    #[serde(default)]
    pub id: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub parentid: Option<i64>,
    #[serde(default)]
    pub order: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkIpListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub ip_list: Vec<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccessTokenResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub access_token: Option<String>,
    #[serde(default)]
    pub expires_in: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCorpGroupAppShareInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub corp_list: Vec<WorkCorpGroupAppShareCorp>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCorpGroupAppShareCorp {
    #[serde(default)]
    pub corpid: Option<String>,
    #[serde(default)]
    pub agentid: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

pub type WorkExternalUserIdConvertResponse = WorkUnionIdToExternalUserIdResponse;

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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkExternalUserIdToPendingIdResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub result: Vec<WorkExternalUserIdToPendingIdItem>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUserIdToOpenUserIdItem {
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub open_userid: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUserIdToOpenUserIdResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub open_userid_list: Vec<WorkUserIdToOpenUserIdItem>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkExternalTagIdToOpenExternalTagIdItem {
    #[serde(default)]
    pub external_tagid: Option<String>,
    #[serde(default)]
    pub open_external_tagid: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkToServiceExternalUserIdResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub external_userid: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
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

impl WorkInvoiceStatusRequest {
    pub fn new(
        card_id: impl Into<String>,
        encrypt_code: impl Into<String>,
        reimburse_status: WorkInvoiceReimburseStatusKind,
    ) -> Self {
        Self {
            card_id: card_id.into(),
            encrypt_code: encrypt_code.into(),
            reimburse_status: reimburse_status.as_code().to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkInvoiceStatusBatchRequest {
    pub openid: String,
    pub reimburse_status: String,
    pub invoice_list: Vec<WorkInvoiceCardRequest>,
}

impl WorkInvoiceStatusBatchRequest {
    pub fn new(
        openid: impl Into<String>,
        reimburse_status: WorkInvoiceReimburseStatusKind,
        invoice_list: Vec<WorkInvoiceCardRequest>,
    ) -> Self {
        Self {
            openid: openid.into(),
            reimburse_status: reimburse_status.as_code().to_string(),
            invoice_list,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkInvoiceReimburseStatusKind {
    Init,
    Locked,
    Closure,
    Other,
}

impl WorkInvoiceReimburseStatusKind {
    pub fn from_code(value: &str) -> Self {
        if value.eq_ignore_ascii_case("INVOICE_REIMBURSE_INIT") {
            Self::Init
        } else if value.eq_ignore_ascii_case("INVOICE_REIMBURSE_LOCK") {
            Self::Locked
        } else if value.eq_ignore_ascii_case("INVOICE_REIMBURSE_CLOSURE") {
            Self::Closure
        } else {
            Self::Other
        }
    }

    pub fn as_code(self) -> &'static str {
        match self {
            Self::Init => "INVOICE_REIMBURSE_INIT",
            Self::Locked => "INVOICE_REIMBURSE_LOCK",
            Self::Closure => "INVOICE_REIMBURSE_CLOSURE",
            Self::Other => "UNKNOWN",
        }
    }

    pub fn is_final(self) -> bool {
        matches!(self, Self::Closure)
    }
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
    pub user_info: Option<WorkInvoiceUserInfo>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkInvoiceUserInfo {
    #[serde(default)]
    pub fee: Option<i64>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub billing_time: Option<String>,
    #[serde(default)]
    pub billing_no: Option<String>,
    #[serde(default)]
    pub billing_code: Option<String>,
    #[serde(default)]
    pub tax_no: Option<String>,
    #[serde(default)]
    pub buyer_number: Option<String>,
    #[serde(default)]
    pub buyer_address_and_phone: Option<String>,
    #[serde(default)]
    pub buyer_bank_account: Option<String>,
    #[serde(default)]
    pub remarks: Option<String>,
    #[serde(default)]
    pub pdf_url: Option<String>,
    #[serde(default)]
    pub check_code: Option<String>,
    #[serde(default)]
    pub info: Vec<WorkInvoiceLineItem>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkInvoiceLineItem {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub num: Option<String>,
    #[serde(default)]
    pub unit: Option<String>,
    #[serde(default)]
    pub fee: Option<i64>,
    #[serde(default)]
    pub price: Option<String>,
    #[serde(default)]
    pub tax_rate: Option<String>,
    #[serde(default)]
    pub tax_amount: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkInvoiceInfoBatchResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub item_list: Vec<WorkInvoiceInfoBatchItem>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkInvoiceInfoBatchItem {
    #[serde(default)]
    pub card_id: Option<String>,
    #[serde(default)]
    pub encrypt_code: Option<String>,
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
    pub openid: Option<String>,
    #[serde(default)]
    pub reimburse_status: Option<String>,
    #[serde(default)]
    pub user_info: Option<WorkInvoiceUserInfo>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl WorkInvoiceInfoBatchItem {
    pub fn reimburse_status_kind(&self) -> Option<WorkInvoiceReimburseStatusKind> {
        self.reimburse_status
            .as_deref()
            .map(WorkInvoiceReimburseStatusKind::from_code)
    }

    pub fn is_reimbursed(&self) -> bool {
        self.reimburse_status_kind()
            .is_some_and(WorkInvoiceReimburseStatusKind::is_final)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkExternalPaySetMerchantUseScopeRequest {
    pub mch_id: String,
    pub allow_use_scope: String,
}

impl WorkExternalPaySetMerchantUseScopeRequest {
    pub fn new(mch_id: impl Into<String>, allow_use_scope: WorkExternalPayUseScopeKind) -> Self {
        Self {
            mch_id: mch_id.into(),
            allow_use_scope: allow_use_scope.as_code().to_string(),
        }
    }
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
    pub allow_use_scope: Vec<WorkExternalPayUseScope>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkExternalPayUseScope {
    #[serde(default, rename = "type")]
    pub scope_type: Option<String>,
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub partyid: Option<i64>,
    #[serde(default)]
    pub tagid: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl WorkExternalPayUseScope {
    pub fn scope_kind(&self) -> Option<WorkExternalPayUseScopeKind> {
        self.scope_type
            .as_deref()
            .map(WorkExternalPayUseScopeKind::from_code)
    }

    pub fn applies_to_all(&self) -> bool {
        self.scope_kind()
            .is_some_and(|kind| matches!(kind, WorkExternalPayUseScopeKind::All))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkExternalPayUseScopeKind {
    All,
    User,
    Party,
    Tag,
    Other,
}

impl WorkExternalPayUseScopeKind {
    pub fn from_code(value: &str) -> Self {
        if value.eq_ignore_ascii_case("all") {
            Self::All
        } else if value.eq_ignore_ascii_case("userid") || value.eq_ignore_ascii_case("user") {
            Self::User
        } else if value.eq_ignore_ascii_case("partyid")
            || value.eq_ignore_ascii_case("party")
            || value.eq_ignore_ascii_case("department")
        {
            Self::Party
        } else if value.eq_ignore_ascii_case("tagid") || value.eq_ignore_ascii_case("tag") {
            Self::Tag
        } else {
            Self::Other
        }
    }

    pub fn as_code(self) -> &'static str {
        match self {
            Self::All => "all",
            Self::User => "userid",
            Self::Party => "partyid",
            Self::Tag => "tagid",
            Self::Other => "unknown",
        }
    }
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
    pub bill_list: Vec<WorkExternalPayBill>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkExternalPayBill {
    #[serde(default)]
    pub out_trade_no: Option<String>,
    #[serde(default)]
    pub transaction_id: Option<String>,
    #[serde(default)]
    pub mch_id: Option<String>,
    #[serde(default)]
    pub merchant_name: Option<String>,
    #[serde(default)]
    pub payee_userid: Option<String>,
    #[serde(default)]
    pub payer_userid: Option<String>,
    #[serde(default)]
    pub amount: Option<i64>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub trade_state: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub pay_time: Option<i64>,
    #[serde(default)]
    pub create_time: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl WorkExternalPayBill {
    pub fn status_kind(&self) -> Option<WorkExternalPayBillStatusKind> {
        self.status
            .as_deref()
            .or(self.trade_state.as_deref())
            .map(WorkExternalPayBillStatusKind::from_code)
    }

    pub fn is_success(&self) -> bool {
        self.status_kind()
            .is_some_and(|kind| matches!(kind, WorkExternalPayBillStatusKind::Success))
    }

    pub fn is_terminal(&self) -> bool {
        self.status_kind()
            .is_some_and(WorkExternalPayBillStatusKind::is_terminal)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkExternalPayBillStatusKind {
    Success,
    Refund,
    NotPay,
    Closed,
    Revoked,
    UserPaying,
    PayError,
    Other,
}

impl WorkExternalPayBillStatusKind {
    pub fn from_code(value: &str) -> Self {
        if value.eq_ignore_ascii_case("success") {
            Self::Success
        } else if value.eq_ignore_ascii_case("refund") {
            Self::Refund
        } else if value.eq_ignore_ascii_case("notpay") || value.eq_ignore_ascii_case("not_pay") {
            Self::NotPay
        } else if value.eq_ignore_ascii_case("closed") {
            Self::Closed
        } else if value.eq_ignore_ascii_case("revoked") {
            Self::Revoked
        } else if value.eq_ignore_ascii_case("userpaying")
            || value.eq_ignore_ascii_case("user_paying")
        {
            Self::UserPaying
        } else if value.eq_ignore_ascii_case("payerror") || value.eq_ignore_ascii_case("pay_error")
        {
            Self::PayError
        } else {
            Self::Other
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Success | Self::Refund | Self::Closed | Self::Revoked | Self::PayError
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkStatusResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
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
    pub text: Option<WorkTextMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<WorkMediaMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<WorkMediaMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<WorkVideoMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<WorkMediaMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown: Option<WorkMarkdownMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub textcard: Option<WorkTextCardMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub news: Option<WorkNewsMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpnews: Option<WorkMpNewsMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miniprogram_notice: Option<WorkMiniProgramNoticeMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taskcard: Option<WorkTaskCardMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_card: Option<WorkTemplateCard>,
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

impl WorkMessage {
    pub fn msgtype_kind(&self) -> WorkMessageTypeKind {
        WorkMessageTypeKind::from_code(&self.msgtype)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkMessageTypeKind {
    Text,
    Image,
    Voice,
    Video,
    File,
    Markdown,
    MarkdownV2,
    TextCard,
    News,
    MpNews,
    MiniProgramNotice,
    TaskCard,
    TemplateCard,
    Other,
}

impl WorkMessageTypeKind {
    pub fn from_code(value: &str) -> Self {
        if value.eq_ignore_ascii_case("text") {
            Self::Text
        } else if value.eq_ignore_ascii_case("image") {
            Self::Image
        } else if value.eq_ignore_ascii_case("voice") {
            Self::Voice
        } else if value.eq_ignore_ascii_case("video") {
            Self::Video
        } else if value.eq_ignore_ascii_case("file") {
            Self::File
        } else if value.eq_ignore_ascii_case("markdown") {
            Self::Markdown
        } else if value.eq_ignore_ascii_case("markdown_v2")
            || value.eq_ignore_ascii_case("markdownv2")
        {
            Self::MarkdownV2
        } else if value.eq_ignore_ascii_case("textcard") || value.eq_ignore_ascii_case("text_card")
        {
            Self::TextCard
        } else if value.eq_ignore_ascii_case("news") {
            Self::News
        } else if value.eq_ignore_ascii_case("mpnews") || value.eq_ignore_ascii_case("mp_news") {
            Self::MpNews
        } else if value.eq_ignore_ascii_case("miniprogram_notice")
            || value.eq_ignore_ascii_case("mini_program_notice")
        {
            Self::MiniProgramNotice
        } else if value.eq_ignore_ascii_case("taskcard") || value.eq_ignore_ascii_case("task_card")
        {
            Self::TaskCard
        } else if value.eq_ignore_ascii_case("template_card")
            || value.eq_ignore_ascii_case("templatecard")
        {
            Self::TemplateCard
        } else {
            Self::Other
        }
    }

    pub fn as_code(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Image => "image",
            Self::Voice => "voice",
            Self::Video => "video",
            Self::File => "file",
            Self::Markdown => "markdown",
            Self::MarkdownV2 => "markdown_v2",
            Self::TextCard => "textcard",
            Self::News => "news",
            Self::MpNews => "mpnews",
            Self::MiniProgramNotice => "miniprogram_notice",
            Self::TaskCard => "taskcard",
            Self::TemplateCard => "template_card",
            Self::Other => "unknown",
        }
    }

    pub fn is_media(self) -> bool {
        matches!(self, Self::Image | Self::Voice | Self::Video | Self::File)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkTextMessage {
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMarkdownMessage {
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMediaMessage {
    pub media_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkTaskCardMessage {
    pub title: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    pub task_id: String,
    pub btn: Vec<WorkTaskCardButton>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkTaskCardButton {
    pub key: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_bold: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkTemplateCard {
    pub card_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<WorkTemplateCardSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_menu: Option<WorkTemplateCardActionMenu>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_title: Option<WorkTemplateCardMainTitle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_area: Option<WorkTemplateCardQuoteArea>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emphasis_content: Option<WorkTemplateCardEmphasisContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_title_text: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub horizontal_content_list: Vec<WorkTemplateCardHorizontalContent>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jump_list: Vec<WorkTemplateCardJump>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_action: Option<WorkTemplateCardAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_text_area: Option<WorkTemplateCardImageTextArea>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_image: Option<WorkTemplateCardImage>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub vertical_content_list: Vec<WorkTemplateCardVerticalContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button_selection: Option<WorkTemplateCardButtonSelection>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub button_list: Vec<WorkTemplateCardButton>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkbox: Option<WorkTemplateCardCheckbox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_button: Option<WorkTemplateCardSubmitButton>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub select_list: Vec<WorkTemplateCardSelect>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl WorkTemplateCard {
    pub fn new(card_type: WorkTemplateCardTypeKind) -> Self {
        Self {
            card_type: card_type.as_code().to_string(),
            source: None,
            action_menu: None,
            task_id: None,
            main_title: None,
            quote_area: None,
            emphasis_content: None,
            sub_title_text: None,
            horizontal_content_list: Vec::new(),
            jump_list: Vec::new(),
            card_action: None,
            image_text_area: None,
            card_image: None,
            vertical_content_list: Vec::new(),
            button_selection: None,
            button_list: Vec::new(),
            checkbox: None,
            submit_button: None,
            select_list: Vec::new(),
            extra: Value::Null,
        }
    }

    pub fn card_type_kind(&self) -> WorkTemplateCardTypeKind {
        WorkTemplateCardTypeKind::from_code(&self.card_type)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkTemplateCardSource {
    pub icon_url: String,
    pub desc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc_color: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkTemplateCardActionMenu {
    pub desc: String,
    pub action_list: Vec<WorkTemplateCardActionMenuItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkTemplateCardActionMenuItem {
    pub text: String,
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkTemplateCardMainTitle {
    pub title: String,
    pub desc: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkTemplateCardQuoteArea {
    #[serde(rename = "type")]
    pub action_type: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    pub title: String,
    pub quote_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkTemplateCardEmphasisContent {
    pub title: String,
    pub desc: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkTemplateCardHorizontalContent {
    pub keyname: String,
    pub value: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkTemplateCardJump {
    #[serde(rename = "type")]
    pub jump_type: i64,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagepath: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkTemplateCardAction {
    #[serde(rename = "type")]
    pub action_type: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagepath: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkTemplateCardImageTextArea {
    #[serde(rename = "type")]
    pub action_type: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    pub title: String,
    pub desc: String,
    pub image_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkTemplateCardImage {
    pub url: String,
    pub aspect_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkTemplateCardVerticalContent {
    pub title: String,
    pub desc: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkTemplateCardButtonSelection {
    pub question_key: String,
    pub title: String,
    pub option_list: Vec<WorkTemplateCardOption>,
    pub selected_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkTemplateCardOption {
    pub id: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkTemplateCardButton {
    pub text: String,
    pub style: i64,
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkTemplateCardCheckbox {
    pub question_key: String,
    pub option_list: Vec<WorkTemplateCardCheckboxOption>,
    pub mode: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkTemplateCardCheckboxOption {
    pub id: String,
    pub text: String,
    pub is_checked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkTemplateCardSubmitButton {
    pub text: String,
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkTemplateCardSelect {
    pub question_key: String,
    pub title: String,
    pub selected_id: String,
    pub option_list: Vec<WorkTemplateCardOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMessageAudience {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub touser: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toparty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totag: Option<String>,
    pub agentid: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safe: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_id_trans: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_duplicate_check: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_check_interval: Option<i64>,
}

impl WorkMessageAudience {
    pub fn to_user(agent_id: i64, user: impl Into<String>) -> Self {
        Self {
            touser: Some(user.into()),
            toparty: None,
            totag: None,
            agentid: agent_id,
            safe: Some(0),
            enable_id_trans: None,
            enable_duplicate_check: None,
            duplicate_check_interval: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkMessageStatisticsTimeKind {
    Today,
    Yesterday,
}

impl WorkMessageStatisticsTimeKind {
    pub const fn as_code(self) -> i64 {
        match self {
            Self::Today => 0,
            Self::Yesterday => 1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMessageStatisticsRequest {
    pub time_type: i64,
}

impl WorkMessageStatisticsRequest {
    pub fn new(time_type: WorkMessageStatisticsTimeKind) -> Self {
        Self {
            time_type: time_type.as_code(),
        }
    }

    pub fn time_kind(&self) -> Option<WorkMessageStatisticsTimeKind> {
        match self.time_type {
            0 => Some(WorkMessageStatisticsTimeKind::Today),
            1 => Some(WorkMessageStatisticsTimeKind::Yesterday),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMessageStatisticsResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub statistics: Vec<WorkMessageStatistic>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl WorkMessageStatisticsResponse {
    pub fn total_count(&self) -> i64 {
        self.statistics.iter().filter_map(|item| item.count).sum()
    }

    pub fn by_agent_id(&self, agent_id: i64) -> Option<&WorkMessageStatistic> {
        self.statistics
            .iter()
            .find(|item| item.agentid == Some(agent_id))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMessageStatistic {
    #[serde(default)]
    pub app_name: Option<String>,
    #[serde(default)]
    pub agentid: Option<i64>,
    #[serde(default)]
    pub count: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkTaskCardUpdateRequest {
    pub userids: Vec<String>,
    pub agentid: i64,
    pub task_id: String,
    pub clicked_key: String,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkTaskCardUpdateResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub invaliduser: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl WorkTaskCardUpdateResponse {
    pub fn invalid_users(&self) -> Vec<&str> {
        work_message_recipient_ids(self.invaliduser.as_deref()).collect()
    }

    pub fn has_delivery_failures(&self) -> bool {
        work_message_recipient_ids(self.invaliduser.as_deref())
            .next()
            .is_some()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkTemplateCardUpdateRequest {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub userids: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub partyids: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tagids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atall: Option<i64>,
    pub agentid: i64,
    pub response_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button: Option<WorkTemplateCardUpdateButton>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_card: Option<WorkTemplateCard>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl WorkTemplateCardUpdateRequest {
    pub fn template_card_type_kind(&self) -> Option<WorkTemplateCardTypeKind> {
        self.template_card
            .as_ref()
            .map(WorkTemplateCard::card_type_kind)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkTemplateCardUpdateButton {
    pub replace_name: String,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkTemplateCardTypeKind {
    TextNotice,
    NewsNotice,
    ButtonInteraction,
    VoteInteraction,
    MultipleInteraction,
    Other,
}

impl WorkTemplateCardTypeKind {
    pub fn from_code(value: &str) -> Self {
        if value.eq_ignore_ascii_case("text_notice") {
            Self::TextNotice
        } else if value.eq_ignore_ascii_case("news_notice") {
            Self::NewsNotice
        } else if value.eq_ignore_ascii_case("button_interaction") {
            Self::ButtonInteraction
        } else if value.eq_ignore_ascii_case("vote_interaction") {
            Self::VoteInteraction
        } else if value.eq_ignore_ascii_case("multiple_interaction") {
            Self::MultipleInteraction
        } else {
            Self::Other
        }
    }

    pub fn as_code(self) -> &'static str {
        match self {
            Self::TextNotice => "text_notice",
            Self::NewsNotice => "news_notice",
            Self::ButtonInteraction => "button_interaction",
            Self::VoteInteraction => "vote_interaction",
            Self::MultipleInteraction => "multiple_interaction",
            Self::Other => "unknown",
        }
    }

    pub fn is_interactive(self) -> bool {
        matches!(
            self,
            Self::ButtonInteraction | Self::VoteInteraction | Self::MultipleInteraction
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkLinkedCorpMessage {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub touser: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub toparty: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub totag: Vec<String>,
    pub msgtype: String,
    pub agentid: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<WorkTextMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<WorkMediaMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<WorkMediaMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<WorkVideoMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<WorkMediaMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub textcard: Option<WorkTextCardMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub news: Option<WorkNewsMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpnews: Option<WorkMpNewsMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown: Option<WorkMarkdownMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miniprogram_notice: Option<WorkMiniProgramNoticeMessage>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl WorkLinkedCorpMessage {
    pub fn msgtype_kind(&self) -> WorkMessageTypeKind {
        WorkMessageTypeKind::from_code(&self.msgtype)
    }

    pub fn text(agent_id: i64, content: impl Into<String>) -> Self {
        let mut message = Self::empty(agent_id, WorkMessageTypeKind::Text);
        message.text = Some(WorkTextMessage {
            content: content.into(),
        });
        message
    }

    pub fn image(agent_id: i64, media_id: impl Into<String>) -> Self {
        let mut message = Self::empty(agent_id, WorkMessageTypeKind::Image);
        message.image = Some(WorkMediaMessage {
            media_id: media_id.into(),
        });
        message
    }

    pub fn file(agent_id: i64, media_id: impl Into<String>) -> Self {
        let mut message = Self::empty(agent_id, WorkMessageTypeKind::File);
        message.file = Some(WorkMediaMessage {
            media_id: media_id.into(),
        });
        message
    }

    pub fn with_touser(mut self, user_id: impl Into<String>) -> Self {
        self.touser.push(user_id.into());
        self
    }

    pub fn with_toparty(mut self, party_id: impl Into<String>) -> Self {
        self.toparty.push(party_id.into());
        self
    }

    pub fn with_totag(mut self, tag_id: impl Into<String>) -> Self {
        self.totag.push(tag_id.into());
        self
    }

    fn empty(agent_id: i64, msg_type: WorkMessageTypeKind) -> Self {
        Self {
            touser: Vec::new(),
            toparty: Vec::new(),
            totag: Vec::new(),
            msgtype: msg_type.as_code().to_string(),
            agentid: agent_id,
            text: None,
            image: None,
            voice: None,
            video: None,
            file: None,
            textcard: None,
            news: None,
            mpnews: None,
            markdown: None,
            miniprogram_notice: None,
            extra: Value::Null,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkExternalContactSchoolMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_scope: Option<i64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub to_external_userid: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub to_parent_userid: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub to_student_userid: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub to_party: Vec<String>,
    pub msgtype: String,
    pub agentid: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<WorkTextMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<WorkMediaMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miniprogram_notice: Option<WorkMiniProgramNoticeMessage>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl WorkExternalContactSchoolMessage {
    pub fn msgtype_kind(&self) -> WorkMessageTypeKind {
        WorkMessageTypeKind::from_code(&self.msgtype)
    }

    pub fn text(agent_id: i64, content: impl Into<String>) -> Self {
        let mut message = Self::empty(agent_id, WorkMessageTypeKind::Text);
        message.text = Some(WorkTextMessage {
            content: content.into(),
        });
        message
    }

    pub fn image(agent_id: i64, media_id: impl Into<String>) -> Self {
        let mut message = Self::empty(agent_id, WorkMessageTypeKind::Image);
        message.image = Some(WorkMediaMessage {
            media_id: media_id.into(),
        });
        message
    }

    pub fn with_recv_scope(mut self, recv_scope: i64) -> Self {
        self.recv_scope = Some(recv_scope);
        self
    }

    pub fn with_external_user(mut self, user_id: impl Into<String>) -> Self {
        self.to_external_userid.push(user_id.into());
        self
    }

    pub fn with_parent_user(mut self, user_id: impl Into<String>) -> Self {
        self.to_parent_userid.push(user_id.into());
        self
    }

    pub fn with_student_user(mut self, user_id: impl Into<String>) -> Self {
        self.to_student_userid.push(user_id.into());
        self
    }

    pub fn with_party(mut self, party_id: impl Into<String>) -> Self {
        self.to_party.push(party_id.into());
        self
    }

    fn empty(agent_id: i64, msg_type: WorkMessageTypeKind) -> Self {
        Self {
            recv_scope: None,
            to_external_userid: Vec::new(),
            to_parent_userid: Vec::new(),
            to_student_userid: Vec::new(),
            to_party: Vec::new(),
            msgtype: msg_type.as_code().to_string(),
            agentid: agent_id,
            text: None,
            image: None,
            miniprogram_notice: None,
            extra: Value::Null,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkVideoMessage {
    pub media_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkTextCardMessage {
    pub title: String,
    pub description: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub btntxt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkNewsArticle {
    pub title: String,
    pub description: String,
    pub url: String,
    pub picurl: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkNewsMessage {
    pub articles: Vec<WorkNewsArticle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMpNewsArticle {
    pub title: String,
    pub thumb_media_id: String,
    pub author: String,
    pub content_source_url: String,
    pub content: String,
    pub digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMpNewsMessage {
    pub articles: Vec<WorkMpNewsArticle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMiniProgramNoticeMessage {
    pub appid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emphasis_first_item: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub content_item: Vec<WorkMiniProgramNoticeItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMiniProgramNoticeItem {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkDepartmentUserId {
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub department: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUserDetailResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(flatten)]
    pub user: WorkUserDetail,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkDepartmentUserSimpleListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub userlist: Vec<WorkDepartmentSimpleUser>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkDepartmentSimpleUser {
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub department: Vec<i64>,
    #[serde(default)]
    pub open_userid: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkDepartmentUserDetailListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub userlist: Vec<WorkUserDetail>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUserRequest {
    pub userid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub department: Vec<i64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub order: Vec<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub biz_mail: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub is_leader_in_dept: Vec<i64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub direct_leader: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telephone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_department: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_invite: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_mediaid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_position: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_profile: Option<WorkUserExternalProfile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extattr: Option<WorkUserExtAttr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUserBatchDeleteRequest {
    pub useridlist: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkUserDetail {
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub department: Vec<i64>,
    #[serde(default)]
    pub order: Vec<i64>,
    #[serde(default)]
    pub position: Option<String>,
    #[serde(default)]
    pub mobile: Option<String>,
    #[serde(default)]
    pub gender: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub biz_mail: Option<String>,
    #[serde(default)]
    pub is_leader_in_dept: Vec<i64>,
    #[serde(default)]
    pub direct_leader: Vec<String>,
    #[serde(default)]
    pub avatar: Option<String>,
    #[serde(default)]
    pub thumb_avatar: Option<String>,
    #[serde(default)]
    pub telephone: Option<String>,
    #[serde(default)]
    pub alias: Option<String>,
    #[serde(default)]
    pub address: Option<String>,
    #[serde(default)]
    pub open_userid: Option<String>,
    #[serde(default)]
    pub main_department: Option<i64>,
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default)]
    pub qr_code: Option<String>,
    #[serde(default)]
    pub external_position: Option<String>,
    #[serde(default)]
    pub external_profile: Option<WorkUserExternalProfile>,
    #[serde(default)]
    pub extattr: Option<WorkUserExtAttr>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkUserStatusKind {
    Active,
    Disabled,
    Inactive,
    Exited,
    Other(i64),
}

impl From<i64> for WorkUserStatusKind {
    fn from(value: i64) -> Self {
        match value {
            1 => Self::Active,
            2 => Self::Disabled,
            4 => Self::Inactive,
            5 => Self::Exited,
            other => Self::Other(other),
        }
    }
}

impl WorkUserStatusKind {
    pub fn can_login(self) -> bool {
        matches!(self, Self::Active)
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Exited)
    }
}

impl WorkUserDetail {
    pub fn status_kind(&self) -> Option<WorkUserStatusKind> {
        self.status.map(WorkUserStatusKind::from)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUserExtAttr {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attrs: Vec<WorkUserExtAttrItem>,
}

impl WorkUserExtAttr {
    pub fn text(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            attrs: vec![WorkUserExtAttrItem::text(name, value)],
        }
    }

    pub fn web(name: impl Into<String>, title: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            attrs: vec![WorkUserExtAttrItem::web(name, title, url)],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUserExtAttrItem {
    #[serde(rename = "type")]
    pub attr_type: i64,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<WorkUserExtAttrText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web: Option<WorkUserExtAttrWeb>,
}

impl WorkUserExtAttrItem {
    pub fn text(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            attr_type: 0,
            name: name.into(),
            text: Some(WorkUserExtAttrText {
                value: value.into(),
            }),
            web: None,
        }
    }

    pub fn web(name: impl Into<String>, title: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            attr_type: 1,
            name: name.into(),
            text: None,
            web: Some(WorkUserExtAttrWeb {
                title: title.into(),
                url: url.into(),
            }),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUserExtAttrText {
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUserExtAttrWeb {
    pub title: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUserExternalProfile {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_corp_name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub external_attr: Vec<WorkUserExternalAttribute>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUserExternalAttribute {
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub attr_type: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<WorkUserExternalAttributeText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web: Option<WorkUserExternalAttributeWeb>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub miniprogram: Option<WorkUserExternalAttributeMiniProgram>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUserExternalAttributeText {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUserExternalAttributeWeb {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUserExternalAttributeMiniProgram {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub appid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pagepath: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUserListIdResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub next_cursor: Option<String>,
    #[serde(default)]
    pub dept_user: Vec<WorkDepartmentUserId>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUserIdLookupResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUserInviteRequest {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub party: Vec<i64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tag: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUserInviteResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub invaliduser: Vec<String>,
    #[serde(default)]
    pub invalidparty: Vec<i64>,
    #[serde(default)]
    pub invalidtag: Vec<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkJoinQrCodeResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub join_qrcode: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUserActiveStatResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub active_cnt: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkLinkedCorpPermListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub department_ids: Vec<String>,
    #[serde(default)]
    pub userids: Vec<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkLinkedCorpUserResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub user_info: Option<WorkLinkedCorpUserInfo>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkLinkedCorpUserListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub userlist: Vec<WorkLinkedCorpUserInfo>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkLinkedCorpDepartmentListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub department_list: Vec<WorkLinkedCorpDepartment>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkLinkedCorpUserInfo {
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub mobile: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub avatar: Option<String>,
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default)]
    pub department: Vec<String>,
    #[serde(default)]
    pub position: Option<String>,
    #[serde(default)]
    pub corp_id: Option<String>,
    #[serde(default)]
    pub extattr: Option<WorkUserExtAttr>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl WorkLinkedCorpUserInfo {
    pub fn status_kind(&self) -> Option<WorkUserStatusKind> {
        self.status.map(WorkUserStatusKind::from)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkLinkedCorpDepartment {
    #[serde(default)]
    pub department_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub parentid: Option<String>,
    #[serde(default)]
    pub order: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUserBatchJobRequest {
    pub media_id: String,
    pub to_invite: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub callbacks: Option<WorkUserBatchJobCallback>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUserBatchJobCallback {
    pub url: String,
    pub token: String,
    pub encodingaeskey: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUserBatchJobResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub jobid: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUserBatchJobResultResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default, rename = "type")]
    pub job_type: Option<String>,
    #[serde(default)]
    pub total: Option<i64>,
    #[serde(default)]
    pub percentage: Option<i64>,
    #[serde(default)]
    pub result: Vec<WorkUserBatchJobResultItem>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkAsyncJobStatusKind {
    Started,
    Processing,
    Finished,
    Other(i64),
}

impl From<i64> for WorkAsyncJobStatusKind {
    fn from(value: i64) -> Self {
        match value {
            1 => Self::Started,
            2 => Self::Processing,
            3 => Self::Finished,
            other => Self::Other(other),
        }
    }
}

impl WorkAsyncJobStatusKind {
    pub fn is_finished(self) -> bool {
        matches!(self, Self::Finished)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkAsyncJobTypeKind {
    SyncUser,
    ReplaceUser,
    InviteUser,
    ReplaceParty,
    ExportUser,
    ExportSimpleUser,
    ExportDepartment,
    ExportTagUser,
    Other,
}

impl WorkAsyncJobTypeKind {
    pub fn from_code(value: &str) -> Self {
        if value.eq_ignore_ascii_case("sync_user") {
            Self::SyncUser
        } else if value.eq_ignore_ascii_case("replace_user") {
            Self::ReplaceUser
        } else if value.eq_ignore_ascii_case("invite_user") {
            Self::InviteUser
        } else if value.eq_ignore_ascii_case("replace_party") {
            Self::ReplaceParty
        } else if value.eq_ignore_ascii_case("export_user") {
            Self::ExportUser
        } else if value.eq_ignore_ascii_case("export_simple_user") {
            Self::ExportSimpleUser
        } else if value.eq_ignore_ascii_case("export_department") {
            Self::ExportDepartment
        } else if value.eq_ignore_ascii_case("export_taguser") {
            Self::ExportTagUser
        } else {
            Self::Other
        }
    }

    pub fn as_code(self) -> &'static str {
        match self {
            Self::SyncUser => "sync_user",
            Self::ReplaceUser => "replace_user",
            Self::InviteUser => "invite_user",
            Self::ReplaceParty => "replace_party",
            Self::ExportUser => "export_user",
            Self::ExportSimpleUser => "export_simple_user",
            Self::ExportDepartment => "export_department",
            Self::ExportTagUser => "export_taguser",
            Self::Other => "unknown",
        }
    }

    pub fn is_export(self) -> bool {
        matches!(
            self,
            Self::ExportUser
                | Self::ExportSimpleUser
                | Self::ExportDepartment
                | Self::ExportTagUser
        )
    }

    pub fn is_contact_import(self) -> bool {
        matches!(
            self,
            Self::SyncUser | Self::ReplaceUser | Self::InviteUser | Self::ReplaceParty
        )
    }
}

impl WorkUserBatchJobResultResponse {
    pub fn status_kind(&self) -> Option<WorkAsyncJobStatusKind> {
        self.status.map(WorkAsyncJobStatusKind::from)
    }

    pub fn job_type_kind(&self) -> Option<WorkAsyncJobTypeKind> {
        self.job_type
            .as_deref()
            .map(WorkAsyncJobTypeKind::from_code)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUserBatchJobResultItem {
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub action: Option<i64>,
    #[serde(default)]
    pub partyid: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUserExportJobRequest {
    pub encoding_aeskey: String,
    pub block_size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUserExportTagUserJobRequest {
    pub tagid: i64,
    pub encoding_aeskey: String,
    pub block_size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUserExportJobResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub jobid: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUserExportJobResultResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default)]
    pub data_list: Vec<WorkUserExportJobData>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl WorkUserExportJobResultResponse {
    pub fn status_kind(&self) -> Option<WorkAsyncJobStatusKind> {
        self.status.map(WorkAsyncJobStatusKind::from)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUserExportJobData {
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub department: Vec<i64>,
    #[serde(default)]
    pub mobile: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub avatar: Option<String>,
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default)]
    pub tagid: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl WorkUserExportJobData {
    pub fn status_kind(&self) -> Option<WorkUserStatusKind> {
        self.status.map(WorkUserStatusKind::from)
    }
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
    pub unlicenseduser: Option<String>,
    #[serde(default)]
    pub msgid: Option<String>,
    #[serde(default)]
    pub response_code: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

fn work_message_recipient_ids(value: Option<&str>) -> impl Iterator<Item = &str> {
    value
        .into_iter()
        .flat_map(|value| value.split('|'))
        .map(str::trim)
        .filter(|value| !value.is_empty())
}

impl MessageSendResponse {
    pub fn invalid_users(&self) -> Vec<&str> {
        work_message_recipient_ids(self.invaliduser.as_deref()).collect()
    }

    pub fn invalid_parties(&self) -> Vec<&str> {
        work_message_recipient_ids(self.invalidparty.as_deref()).collect()
    }

    pub fn invalid_tags(&self) -> Vec<&str> {
        work_message_recipient_ids(self.invalidtag.as_deref()).collect()
    }

    pub fn unlicensed_users(&self) -> Vec<&str> {
        work_message_recipient_ids(self.unlicenseduser.as_deref()).collect()
    }

    pub fn has_delivery_failures(&self) -> bool {
        work_message_recipient_ids(self.invaliduser.as_deref())
            .next()
            .is_some()
            || work_message_recipient_ids(self.invalidparty.as_deref())
                .next()
                .is_some()
            || work_message_recipient_ids(self.invalidtag.as_deref())
                .next()
                .is_some()
            || work_message_recipient_ids(self.unlicenseduser.as_deref())
                .next()
                .is_some()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkLinkedCorpMessageSendResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub invaliduser: Vec<String>,
    #[serde(default)]
    pub invalidparty: Vec<String>,
    #[serde(default)]
    pub invalidtag: Vec<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkExternalContactSchoolMessageSendResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub invalid_external_user: Vec<String>,
    #[serde(default)]
    pub invalid_parent_userid: Vec<String>,
    #[serde(default)]
    pub invalid_student_userid: Vec<String>,
    #[serde(default)]
    pub invalid_party: Vec<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMenuCreateResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub button: Vec<Value>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
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
pub struct ExternalContactListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub external_userid: Vec<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactServedListRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactServedListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub info_list: Vec<ExternalContactServedInfo>,
    #[serde(default)]
    pub next_cursor: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactServedInfo {
    #[serde(default)]
    pub is_customer: Option<bool>,
    #[serde(default)]
    pub tmp_openid: Option<String>,
    #[serde(default)]
    pub external_userid: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub follow_userid: Option<String>,
    #[serde(default)]
    pub chat_id: Option<String>,
    #[serde(default)]
    pub chat_name: Option<String>,
    #[serde(default)]
    pub add_time: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkSchoolSubscribeQrCodeResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub qrcode_big: Option<String>,
    #[serde(default)]
    pub qrcode_middle: Option<String>,
    #[serde(default)]
    pub qrcode_thumb: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkSchoolSubscribeModeResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub subscribe_mode: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl WorkSchoolSubscribeModeResponse {
    pub fn mode_kind(&self) -> Option<WorkSchoolSubscribeModeKind> {
        self.subscribe_mode.map(WorkSchoolSubscribeModeKind::from)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkSchoolSubscribeModeKind {
    AllowQrCodeRegistration,
    ForbidQrCodeRegistration,
    Other(i64),
}

impl From<i64> for WorkSchoolSubscribeModeKind {
    fn from(value: i64) -> Self {
        match value {
            1 => Self::AllowQrCodeRegistration,
            2 => Self::ForbidQrCodeRegistration,
            other => Self::Other(other),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactFollowUserListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub follow_user: Vec<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactDetailResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub external_contact: Option<ExternalContactInfo>,
    #[serde(default)]
    pub follow_user: Vec<ExternalContactFollowInfo>,
    #[serde(default)]
    pub next_cursor: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactBatchGetResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub external_contact_list: Vec<ExternalContactBatchItem>,
    #[serde(default)]
    pub next_cursor: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactBatchItem {
    #[serde(default)]
    pub external_contact: Option<ExternalContactInfo>,
    #[serde(default)]
    pub follow_info: Option<ExternalContactFollowInfo>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactInfo {
    #[serde(default)]
    pub external_userid: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub position: Option<String>,
    #[serde(default)]
    pub avatar: Option<String>,
    #[serde(default, rename = "type")]
    pub contact_type: Option<i64>,
    #[serde(default)]
    pub gender: Option<i64>,
    #[serde(default)]
    pub unionid: Option<String>,
    #[serde(default)]
    pub corp_name: Option<String>,
    #[serde(default)]
    pub corp_full_name: Option<String>,
    #[serde(default)]
    pub external_profile: Option<ExternalContactProfile>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExternalContactKind {
    WechatUser,
    WorkUser,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkContactGender {
    Unknown,
    Male,
    Female,
    Other,
}

impl ExternalContactInfo {
    pub fn contact_kind(&self) -> Option<ExternalContactKind> {
        self.contact_type.map(|contact_type| match contact_type {
            1 => ExternalContactKind::WechatUser,
            2 => ExternalContactKind::WorkUser,
            _ => ExternalContactKind::Other,
        })
    }

    pub fn gender_kind(&self) -> Option<WorkContactGender> {
        self.gender.map(|gender| match gender {
            0 => WorkContactGender::Unknown,
            1 => WorkContactGender::Male,
            2 => WorkContactGender::Female,
            _ => WorkContactGender::Other,
        })
    }

    pub fn is_wechat_user(&self) -> bool {
        self.contact_kind() == Some(ExternalContactKind::WechatUser)
    }

    pub fn is_work_user(&self) -> bool {
        self.contact_kind() == Some(ExternalContactKind::WorkUser)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactProfile {
    #[serde(default)]
    pub external_corp_name: Option<String>,
    #[serde(default)]
    pub external_attr: Vec<ExternalContactAttribute>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactAttribute {
    #[serde(default, rename = "type")]
    pub attr_type: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub text: Option<ExternalContactAttributeText>,
    #[serde(default)]
    pub web: Option<ExternalContactAttributeWeb>,
    #[serde(default)]
    pub miniprogram: Option<ExternalContactAttributeMiniProgram>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExternalContactAttributeKind {
    Text,
    Web,
    MiniProgram,
    Other,
}

impl ExternalContactAttribute {
    pub fn attribute_kind(&self) -> Option<ExternalContactAttributeKind> {
        self.attr_type.map(|attr_type| match attr_type {
            0 => ExternalContactAttributeKind::Text,
            1 => ExternalContactAttributeKind::Web,
            2 => ExternalContactAttributeKind::MiniProgram,
            _ => ExternalContactAttributeKind::Other,
        })
    }

    pub fn is_text(&self) -> bool {
        self.attribute_kind() == Some(ExternalContactAttributeKind::Text)
    }

    pub fn is_web(&self) -> bool {
        self.attribute_kind() == Some(ExternalContactAttributeKind::Web)
    }

    pub fn is_mini_program(&self) -> bool {
        self.attribute_kind() == Some(ExternalContactAttributeKind::MiniProgram)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactAttributeText {
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactAttributeWeb {
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactAttributeMiniProgram {
    #[serde(default)]
    pub appid: Option<String>,
    #[serde(default)]
    pub pagepath: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactFollowInfo {
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub remark: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub createtime: Option<i64>,
    #[serde(default)]
    pub tags: Vec<ExternalContactFollowTag>,
    #[serde(default)]
    pub remark_corp_name: Option<String>,
    #[serde(default)]
    pub remark_mobiles: Vec<String>,
    #[serde(default)]
    pub add_way: Option<i64>,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default)]
    pub oper_userid: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactFollowTag {
    #[serde(default)]
    pub group_name: Option<String>,
    #[serde(default)]
    pub tag_name: Option<String>,
    #[serde(default)]
    pub tag_id: Option<String>,
    #[serde(default)]
    pub r#type: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusions: Option<ContactWayConclusions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactWayAddResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub config_id: Option<String>,
    #[serde(default)]
    pub qr_code: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactWayGetResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub contact_way: Option<ContactWayDetail>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactWayId {
    #[serde(default)]
    pub config_id: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactWayDetail {
    #[serde(default)]
    pub config_id: Option<String>,
    #[serde(default, rename = "type")]
    pub kind: Option<i64>,
    #[serde(default)]
    pub scene: Option<i64>,
    #[serde(default)]
    pub style: Option<i64>,
    #[serde(default)]
    pub remark: Option<String>,
    #[serde(default)]
    pub skip_verify: Option<bool>,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default)]
    pub qr_code: Option<String>,
    #[serde(default)]
    pub user: Vec<String>,
    #[serde(default)]
    pub party: Vec<i64>,
    #[serde(default)]
    pub is_temp: Option<bool>,
    #[serde(default)]
    pub expires_in: Option<i64>,
    #[serde(default)]
    pub chat_expires_in: Option<i64>,
    #[serde(default)]
    pub unionid: Option<String>,
    #[serde(default)]
    pub conclusions: Option<ContactWayConclusions>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactWayConclusions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<ContactWayConclusionText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<ContactWayConclusionImage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<ContactWayConclusionLink>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub miniprogram: Option<ContactWayConclusionMiniProgram>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactWayConclusionText {
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactWayConclusionImage {
    pub pic_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactWayConclusionLink {
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub picurl: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactWayConclusionMiniProgram {
    pub title: String,
    pub pic_media_id: String,
    pub appid: String,
    pub page: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactWayListRequest {
    pub start_time: i64,
    pub end_time: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactWayListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub contact_way: Vec<ContactWayId>,
    #[serde(default)]
    pub next_cursor: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactWayUpdateRequest {
    pub config_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skip_verify: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_expires_in: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unionid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conclusions: Option<ContactWayConclusions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactRemarkRequest {
    pub userid: String,
    pub external_userid: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remark_company: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remark_mobiles: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remark_pic_mediaid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorpTagListRequest {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tag_id: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub group_id: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorpTag {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub order: Option<i64>,
    #[serde(default)]
    pub create_time: Option<i64>,
    #[serde(default)]
    pub deleted: Option<bool>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorpTagGroup {
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub group_name: Option<String>,
    #[serde(default)]
    pub order: Option<i64>,
    #[serde(default)]
    pub create_time: Option<i64>,
    #[serde(default)]
    pub deleted: Option<bool>,
    #[serde(default)]
    pub strategy_id: Option<i64>,
    #[serde(default)]
    pub tag: Vec<CorpTag>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorpTagListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub tag_group: Vec<CorpTagGroup>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorpTagAddRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    pub group_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
    pub tag: Vec<CorpTagAddItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agentid: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorpTagAddItem {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorpTagAddResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub tag_group: Option<CorpTagGroup>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorpTagEditRequest {
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agentid: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorpTagDeleteRequest {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tag_id: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub group_id: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agentid: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMarkTagRequest {
    pub userid: String,
    pub external_userid: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub add_tag: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub remove_tag: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalGroupChatListRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_filter: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_filter: Option<ExternalContactOwnerFilter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactOwnerFilter {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub userid_list: Vec<String>,
}

impl ExternalContactOwnerFilter {
    pub fn user(user_id: impl Into<String>) -> Self {
        Self {
            userid_list: vec![user_id.into()],
        }
    }

    pub fn users(user_ids: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            userid_list: user_ids.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalGroupChatListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub group_chat_list: Vec<ExternalGroupChatSummary>,
    #[serde(default)]
    pub next_cursor: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalGroupChatSummary {
    #[serde(default)]
    pub chat_id: Option<String>,
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExternalGroupChatStatusKind {
    Normal,
    OwnerResignedPendingTransfer,
    OwnerResignedTransferring,
    OwnerResignedTransferred,
    Other(i64),
}

impl From<i64> for ExternalGroupChatStatusKind {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::Normal,
            1 => Self::OwnerResignedPendingTransfer,
            2 => Self::OwnerResignedTransferring,
            3 => Self::OwnerResignedTransferred,
            other => Self::Other(other),
        }
    }
}

impl ExternalGroupChatSummary {
    pub fn status_kind(&self) -> Option<ExternalGroupChatStatusKind> {
        self.status.map(ExternalGroupChatStatusKind::from)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalGroupChatGetResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub group_chat: Option<ExternalGroupChatDetail>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalGroupChatDetail {
    #[serde(default)]
    pub chat_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub owner: Option<String>,
    #[serde(default)]
    pub create_time: Option<i64>,
    #[serde(default)]
    pub notice: Option<String>,
    #[serde(default)]
    pub member_list: Vec<ExternalGroupChatMember>,
    #[serde(default)]
    pub admin_list: Vec<ExternalGroupChatAdmin>,
    #[serde(default)]
    pub member_version: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalGroupChatMember {
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default, rename = "type")]
    pub member_type: Option<i64>,
    #[serde(default)]
    pub join_time: Option<i64>,
    #[serde(default)]
    pub join_scene: Option<i64>,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default)]
    pub invitor: Option<ExternalGroupChatInvitor>,
    #[serde(default)]
    pub group_nickname: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub unionid: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExternalGroupChatMemberKind {
    WorkUser,
    ExternalContact,
    Other,
}

impl ExternalGroupChatMember {
    pub fn member_kind(&self) -> Option<ExternalGroupChatMemberKind> {
        self.member_type.map(|member_type| match member_type {
            1 => ExternalGroupChatMemberKind::WorkUser,
            2 => ExternalGroupChatMemberKind::ExternalContact,
            _ => ExternalGroupChatMemberKind::Other,
        })
    }

    pub fn is_work_user(&self) -> bool {
        self.member_kind() == Some(ExternalGroupChatMemberKind::WorkUser)
    }

    pub fn is_external_contact(&self) -> bool {
        self.member_kind() == Some(ExternalGroupChatMemberKind::ExternalContact)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalGroupChatInvitor {
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalGroupChatAdmin {
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalGroupChatTransferResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub failed_chat_list: Vec<ExternalGroupChatFailedChat>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalGroupChatFailedChat {
    #[serde(default)]
    pub chat_id: Option<String>,
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalGroupChatOpenGidToChatIdResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub chat_id: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalGroupChatJoinWayRequest {
    pub scene: i64,
    pub remark: String,
    pub auto_create_room: i64,
    pub room_base_name: String,
    pub room_base_id: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub chat_id_list: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalGroupChatJoinWayUpdateRequest {
    pub config_id: String,
    pub scene: i64,
    pub remark: String,
    pub auto_create_room: i64,
    pub room_base_name: String,
    pub room_base_id: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub chat_id_list: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalGroupChatJoinWayAddResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub config_id: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalGroupChatJoinWayResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub join_way: Option<ExternalGroupChatJoinWay>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalGroupChatJoinWay {
    #[serde(default)]
    pub config_id: Option<String>,
    #[serde(default)]
    pub qr_code: Option<String>,
    #[serde(default)]
    pub scene: Option<i64>,
    #[serde(default)]
    pub remark: Option<String>,
    #[serde(default)]
    pub auto_create_room: Option<i64>,
    #[serde(default)]
    pub room_base_name: Option<String>,
    #[serde(default)]
    pub room_base_id: Option<i64>,
    #[serde(default)]
    pub chat_id_list: Vec<String>,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMomentStrategyRangeRequest {
    pub strategy_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMomentStrategyCreateRequest {
    pub parent_id: i64,
    pub strategy_name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub admin_list: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMomentStrategyEditRequest {
    pub strategy_id: i64,
    pub strategy_name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub admin_list: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMomentStrategy {
    #[serde(default)]
    pub strategy_id: Option<i64>,
    #[serde(default)]
    pub strategy_name: Option<String>,
    #[serde(default)]
    pub parent_id: Option<i64>,
    #[serde(default)]
    pub admin_list: Vec<String>,
    #[serde(default)]
    pub create_time: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMomentStrategyRange {
    #[serde(default)]
    pub user_list: Vec<String>,
    #[serde(default)]
    pub party_list: Vec<i64>,
    #[serde(default)]
    pub department_list: Vec<i64>,
    #[serde(default)]
    pub tag_list: Vec<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMomentStrategyListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub strategy: Vec<ExternalContactMomentStrategy>,
    #[serde(default)]
    pub next_cursor: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMomentStrategyRangeResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub range: Option<ExternalContactMomentStrategyRange>,
    #[serde(default)]
    pub next_cursor: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMomentStrategyCreateResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub strategy_id: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactStrategyTagListRequest {
    pub strategy_id: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tag_id: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub group_id: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactStrategyTagAddRequest {
    pub strategy_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    pub group_name: String,
    pub order: i64,
    pub tag: Vec<ExternalContactStrategyTagAddItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactStrategyTagAddItem {
    pub name: String,
    pub order: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactStrategyTagEditRequest {
    pub id: String,
    pub name: String,
    pub order: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactStrategyTagDeleteRequest {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tag_id: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub group_id: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactStrategyTagListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub tag_group: Vec<CorpTagGroup>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactStrategyTagAddResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub tag_group: Option<CorpTagGroup>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalGroupWelcomeTemplateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<ExternalContactMessageText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<ExternalContactMessageImage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<ExternalContactMessageLink>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub miniprogram: Option<ExternalContactMessageMiniProgram>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<ExternalContactMessageFile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video: Option<ExternalContactMessageVideo>,
    pub agentid: i64,
    pub notify: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalGroupWelcomeTemplateUpdateRequest {
    pub template_id: String,
    #[serde(flatten)]
    pub template: ExternalGroupWelcomeTemplateRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalGroupWelcomeTemplateAddResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub template_id: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalGroupWelcomeTemplateResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub text: Option<ExternalContactMessageText>,
    #[serde(default)]
    pub image: Option<ExternalContactMessageImage>,
    #[serde(default)]
    pub link: Option<ExternalContactMessageLink>,
    #[serde(default)]
    pub miniprogram: Option<ExternalContactMessageMiniProgram>,
    #[serde(default)]
    pub file: Option<ExternalContactMessageFile>,
    #[serde(default)]
    pub video: Option<ExternalContactMessageVideo>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAcquisitionLinkListRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAcquisitionLinkListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub link_id_list: Vec<String>,
    #[serde(default)]
    pub next_cursor: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAcquisitionRange {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub user_list: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub department_list: Vec<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAcquisitionPriorityOption {
    pub priority_type: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub priority_userid_list: Vec<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAcquisitionLinkRequest {
    pub link_name: String,
    pub range: CustomerAcquisitionRange,
    pub skip_verify: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority_option: Option<CustomerAcquisitionPriorityOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAcquisitionLinkUpdateRequest {
    pub link_id: String,
    pub link_name: String,
    pub range: CustomerAcquisitionRange,
    pub skip_verify: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority_option: Option<CustomerAcquisitionPriorityOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAcquisitionLinkCreateResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub link: Option<CustomerAcquisitionLink>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAcquisitionLinkResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub link: Option<CustomerAcquisitionLink>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAcquisitionLink {
    #[serde(default)]
    pub link_id: Option<String>,
    #[serde(default)]
    pub link_name: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub skip_verify: Option<bool>,
    #[serde(default)]
    pub range: Option<CustomerAcquisitionRange>,
    #[serde(default)]
    pub priority_option: Option<CustomerAcquisitionPriorityOption>,
    #[serde(default)]
    pub create_time: Option<i64>,
    #[serde(default)]
    pub update_time: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactInterceptRuleRange {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub user_list: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub department_list: Vec<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactInterceptRuleAddRequest {
    pub rule_name: String,
    pub word_list: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub semantics_list: Vec<i64>,
    pub intercept_type: i64,
    pub applicable_range: ExternalContactInterceptRuleRange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactInterceptRuleExtraRule {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub semantics_list: Vec<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactInterceptRuleUpdateRequest {
    pub rule_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub word_list: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra_rule: Option<ExternalContactInterceptRuleExtraRule>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intercept_type: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_applicable_range: Option<ExternalContactInterceptRuleRange>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_applicable_range: Option<ExternalContactInterceptRuleRange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactInterceptRuleAddResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub rule_id: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactInterceptRuleListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub rule_list: Vec<ExternalContactInterceptRuleSummary>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactInterceptRuleSummary {
    #[serde(default)]
    pub rule_id: Option<String>,
    #[serde(default)]
    pub rule_name: Option<String>,
    #[serde(default)]
    pub create_time: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactInterceptRuleResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub rule: Option<ExternalContactInterceptRule>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactInterceptRule {
    #[serde(default)]
    pub rule_id: Option<String>,
    #[serde(default)]
    pub rule_name: Option<String>,
    #[serde(default)]
    pub word_list: Vec<String>,
    #[serde(default)]
    pub semantics_list: Vec<i64>,
    #[serde(default)]
    pub intercept_type: Option<i64>,
    #[serde(default)]
    pub applicable_range: Option<ExternalContactInterceptRuleRange>,
    #[serde(default)]
    pub create_time: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl ExternalContactInterceptRule {
    pub fn intercept_type_kind(&self) -> Option<ExternalContactInterceptTypeKind> {
        self.intercept_type
            .map(ExternalContactInterceptTypeKind::from)
    }

    pub fn semantic_kinds(
        &self,
    ) -> impl ExactSizeIterator<Item = ExternalContactInterceptSemanticKind> + '_ {
        self.semantics_list
            .iter()
            .copied()
            .map(ExternalContactInterceptSemanticKind::from)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExternalContactInterceptTypeKind {
    WarnAndBlock,
    WarnOnly,
    Other(i64),
}

impl From<i64> for ExternalContactInterceptTypeKind {
    fn from(value: i64) -> Self {
        match value {
            1 => Self::WarnAndBlock,
            2 => Self::WarnOnly,
            other => Self::Other(other),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExternalContactInterceptSemanticKind {
    Mobile,
    Email,
    RedPacket,
    Other(i64),
}

impl From<i64> for ExternalContactInterceptSemanticKind {
    fn from(value: i64) -> Self {
        match value {
            1 => Self::Mobile,
            2 => Self::Email,
            3 => Self::RedPacket,
            other => Self::Other(other),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactProductAlbumAddRequest {
    pub description: String,
    pub price: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_sn: Option<String>,
    pub attachments: Vec<ExternalContactProductAttachment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactProductAlbumUpdateRequest {
    pub product_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_sn: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attachments: Vec<ExternalContactProductAttachment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactProductAttachment {
    #[serde(rename = "type")]
    pub attachment_type: String,
    pub image: ExternalContactProductImage,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl ExternalContactProductAttachment {
    pub fn image(media_id: impl Into<String>) -> Self {
        Self {
            attachment_type: "image".to_string(),
            image: ExternalContactProductImage {
                media_id: media_id.into(),
                extra: Value::Null,
            },
            extra: Value::Null,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactProductImage {
    pub media_id: String,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactProductAlbumAddResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub product_id: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactProductAlbumListRequest {
    pub limit: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactProductAlbumListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub product_list: Vec<ExternalContactProductAlbum>,
    #[serde(default)]
    pub next_cursor: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactProductAlbumResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub product: Option<ExternalContactProductAlbum>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactProductAlbum {
    #[serde(default)]
    pub product_id: Option<String>,
    #[serde(default)]
    pub product_sn: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub price: Option<i64>,
    #[serde(default)]
    pub create_time: Option<i64>,
    #[serde(default)]
    pub attachments: Vec<ExternalContactProductAttachment>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAcquisitionQuotaResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub total: Option<i64>,
    #[serde(default)]
    pub balance: Option<i64>,
    #[serde(default)]
    pub quota_list: Vec<CustomerAcquisitionQuota>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl CustomerAcquisitionQuotaResponse {
    pub fn is_exhausted(&self) -> bool {
        self.balance == Some(0)
    }

    pub fn next_expiring_quota(&self) -> Option<&CustomerAcquisitionQuota> {
        self.quota_list
            .iter()
            .filter(|quota| quota.expire_date.is_some())
            .min_by_key(|quota| quota.expire_date)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAcquisitionQuota {
    #[serde(default)]
    pub expire_date: Option<i64>,
    #[serde(default)]
    pub balance: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAcquisitionCustomerListRequest {
    pub link_id: String,
    pub limit: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAcquisitionCustomerListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub customer_list: Vec<CustomerAcquisitionCustomer>,
    #[serde(default)]
    pub next_cursor: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAcquisitionCustomer {
    #[serde(default)]
    pub external_userid: Option<String>,
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub chat_status: Option<i64>,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl CustomerAcquisitionCustomer {
    pub fn chat_status_kind(&self) -> Option<CustomerAcquisitionChatStatusKind> {
        self.chat_status
            .map(CustomerAcquisitionChatStatusKind::from)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CustomerAcquisitionChatStatusKind {
    NotMessaged,
    Messaged,
    Other(i64),
}

impl From<i64> for CustomerAcquisitionChatStatusKind {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::NotMessaged,
            1 => Self::Messaged,
            other => Self::Other(other),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAcquisitionStatisticRequest {
    pub link_id: String,
    pub start_time: i64,
    pub end_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAcquisitionStatisticResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub click_link_customer_cnt: Option<i64>,
    #[serde(default)]
    pub new_customer_cnt: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAcquisitionChatInfoRequest {
    pub chat_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAcquisitionChatInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub external_userid: Option<String>,
    #[serde(default)]
    pub chat_info: Option<CustomerAcquisitionChatInfo>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAcquisitionChatInfo {
    #[serde(default)]
    pub recv_msg_cnt: Option<i64>,
    #[serde(default)]
    pub link_id: Option<String>,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMessageTemplateRequest {
    pub chat_type: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub external_userid: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub chat_id_list: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_filter: Option<ExternalContactMessageTagFilter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender: Option<String>,
    pub allow_select: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<ExternalContactMessageText>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attachments: Vec<ExternalContactMessageAttachment>,
}

impl ExternalContactMessageTemplateRequest {
    pub fn chat_type_kind(&self) -> ExternalContactMessageChatTypeKind {
        ExternalContactMessageChatTypeKind::from_code(&self.chat_type)
    }

    pub fn validate(&self) -> Result<()> {
        match self.chat_type_kind() {
            ExternalContactMessageChatTypeKind::Single => {
                if !self.chat_id_list.is_empty() {
                    return Err(WechatError::Config(
                        "single external-contact message cannot contain chat_id_list".to_string(),
                    ));
                }
                if self.external_userid.is_empty()
                    && self
                        .sender
                        .as_deref()
                        .is_none_or(|sender| sender.trim().is_empty())
                {
                    return Err(WechatError::Config(
                        "single external-contact message requires external_userid or sender"
                            .to_string(),
                    ));
                }
                if self.external_userid.len() > 10_000 {
                    return Err(WechatError::Config(
                        "single external-contact message supports at most 10000 customers"
                            .to_string(),
                    ));
                }
            }
            ExternalContactMessageChatTypeKind::Group => {
                if !self.external_userid.is_empty() {
                    return Err(WechatError::Config(
                        "group external-contact message cannot contain external_userid".to_string(),
                    ));
                }
                if self
                    .sender
                    .as_deref()
                    .is_none_or(|sender| sender.trim().is_empty())
                {
                    return Err(WechatError::Config(
                        "group external-contact message requires sender".to_string(),
                    ));
                }
                if self.chat_id_list.len() > 2_000 {
                    return Err(WechatError::Config(
                        "group external-contact message supports at most 2000 chats".to_string(),
                    ));
                }
            }
            ExternalContactMessageChatTypeKind::Other => {
                return Err(WechatError::Config(format!(
                    "unsupported external-contact message chat_type: {}",
                    self.chat_type
                )));
            }
        }
        if self.attachments.len() > 9 {
            return Err(WechatError::Config(
                "external-contact message supports at most 9 attachments".to_string(),
            ));
        }
        if self.text.is_none() && self.attachments.is_empty() {
            return Err(WechatError::Config(
                "external-contact message requires text or at least one attachment".to_string(),
            ));
        }
        if let Some(tag_filter) = &self.tag_filter {
            tag_filter.validate()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExternalContactMessageChatTypeKind {
    Single,
    Group,
    Other,
}

impl ExternalContactMessageChatTypeKind {
    pub fn from_code(value: &str) -> Self {
        if value.eq_ignore_ascii_case("single") {
            Self::Single
        } else if value.eq_ignore_ascii_case("group") {
            Self::Group
        } else {
            Self::Other
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMessageTagFilter {
    pub group_list: Vec<ExternalContactMessageTagGroup>,
}

impl ExternalContactMessageTagFilter {
    pub fn from_tag_groups(
        groups: impl IntoIterator<Item = impl IntoIterator<Item = impl Into<String>>>,
    ) -> Self {
        Self {
            group_list: groups
                .into_iter()
                .map(ExternalContactMessageTagGroup::new)
                .collect(),
        }
    }

    pub fn tag_count(&self) -> usize {
        self.group_list
            .iter()
            .map(|group| group.tag_list.len())
            .sum()
    }

    pub fn validate(&self) -> Result<()> {
        if self.group_list.is_empty() {
            return Err(WechatError::Config(
                "external-contact message tag filter requires at least one group".to_string(),
            ));
        }
        if self
            .group_list
            .iter()
            .any(|group| group.tag_list.is_empty())
        {
            return Err(WechatError::Config(
                "external-contact message tag group cannot be empty".to_string(),
            ));
        }
        if self
            .group_list
            .iter()
            .any(|group| group.tag_list.iter().any(|tag_id| tag_id.trim().is_empty()))
        {
            return Err(WechatError::Config(
                "external-contact message tag id cannot be empty".to_string(),
            ));
        }
        if self
            .group_list
            .iter()
            .any(|group| group.tag_list.len() > 100)
        {
            return Err(WechatError::Config(
                "external-contact message tag group supports at most 100 tags".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMessageTagGroup {
    pub tag_list: Vec<String>,
}

impl ExternalContactMessageTagGroup {
    pub fn new(tags: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            tag_list: tags.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMessageTemplateResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub msgid: Option<String>,
    #[serde(default)]
    pub fail_list: Vec<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactGroupMessageListRequest {
    pub chat_type: String,
    pub start_time: i64,
    pub end_time: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_type: Option<i64>,
    pub limit: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactGroupMessageListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub group_msg_list: Vec<ExternalContactGroupMessage>,
    #[serde(default)]
    pub next_cursor: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactGroupMessage {
    #[serde(default)]
    pub msgid: Option<String>,
    #[serde(default)]
    pub creator: Option<String>,
    #[serde(default)]
    pub create_time: Option<i64>,
    #[serde(default)]
    pub create_type: Option<i64>,
    #[serde(default)]
    pub text: Option<ExternalContactMessageText>,
    #[serde(default)]
    pub attachments: Vec<ExternalContactMessageAttachment>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMessageText {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl ExternalContactMessageText {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: Some(content.into()),
            extra: Value::Null,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMessageAttachment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msgtype: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<ExternalContactMessageImage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<ExternalContactMessageLink>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub miniprogram: Option<ExternalContactMessageMiniProgram>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video: Option<ExternalContactMessageVideo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<ExternalContactMessageFile>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl ExternalContactMessageAttachment {
    pub fn image(media_id: impl Into<String>) -> Self {
        Self {
            msgtype: Some("image".to_string()),
            image: Some(ExternalContactMessageImage {
                media_id: Some(media_id.into()),
                pic_url: None,
                extra: Value::Null,
            }),
            link: None,
            miniprogram: None,
            video: None,
            file: None,
            extra: Value::Null,
        }
    }

    pub fn link(link: ExternalContactMessageLink) -> Self {
        Self {
            msgtype: Some("link".to_string()),
            image: None,
            link: Some(link),
            miniprogram: None,
            video: None,
            file: None,
            extra: Value::Null,
        }
    }

    pub fn miniprogram(miniprogram: ExternalContactMessageMiniProgram) -> Self {
        Self {
            msgtype: Some("miniprogram".to_string()),
            image: None,
            link: None,
            miniprogram: Some(miniprogram),
            video: None,
            file: None,
            extra: Value::Null,
        }
    }

    pub fn video(media_id: impl Into<String>) -> Self {
        Self {
            msgtype: Some("video".to_string()),
            image: None,
            link: None,
            miniprogram: None,
            video: Some(ExternalContactMessageVideo {
                media_id: Some(media_id.into()),
                extra: Value::Null,
            }),
            file: None,
            extra: Value::Null,
        }
    }

    pub fn file(media_id: impl Into<String>) -> Self {
        Self {
            msgtype: Some("file".to_string()),
            image: None,
            link: None,
            miniprogram: None,
            video: None,
            file: Some(ExternalContactMessageFile {
                media_id: Some(media_id.into()),
                extra: Value::Null,
            }),
            extra: Value::Null,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMessageImage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pic_url: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMessageLink {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub picurl: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMessageMiniProgram {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pic_media_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub appid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMessageVideo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_id: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMessageFile {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_id: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactGroupMessageTaskResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub task_list: Vec<ExternalContactGroupMessageTask>,
    #[serde(default)]
    pub next_cursor: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactGroupMessageTask {
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default)]
    pub send_time: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExternalContactGroupMessageTaskStatusKind {
    Unsent,
    Sent,
    Other,
}

impl ExternalContactGroupMessageTaskStatusKind {
    pub fn from_code(code: i64) -> Self {
        match code {
            0 => Self::Unsent,
            2 => Self::Sent,
            _ => Self::Other,
        }
    }

    pub fn is_sent(self) -> bool {
        matches!(self, Self::Sent)
    }
}

impl ExternalContactGroupMessageTask {
    pub fn status_kind(&self) -> Option<ExternalContactGroupMessageTaskStatusKind> {
        self.status
            .map(ExternalContactGroupMessageTaskStatusKind::from_code)
    }

    pub fn is_sent(&self) -> bool {
        self.status_kind()
            .is_some_and(ExternalContactGroupMessageTaskStatusKind::is_sent)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactGroupMessageSendResultResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub send_list: Vec<ExternalContactGroupMessageSendResult>,
    #[serde(default)]
    pub next_cursor: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactGroupMessageSendResult {
    #[serde(default)]
    pub external_userid: Option<String>,
    #[serde(default)]
    pub chat_id: Option<String>,
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default)]
    pub send_time: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExternalContactGroupMessageSendStatusKind {
    Unsent,
    Sent,
    CustomerNotFriend,
    DuplicateDelivery,
    ReceiveLimitExceeded,
    Other,
}

impl ExternalContactGroupMessageSendStatusKind {
    pub fn from_code(code: i64) -> Self {
        match code {
            0 => Self::Unsent,
            1 => Self::Sent,
            2 => Self::CustomerNotFriend,
            3 => Self::DuplicateDelivery,
            4 => Self::ReceiveLimitExceeded,
            _ => Self::Other,
        }
    }

    pub fn is_sent(self) -> bool {
        matches!(self, Self::Sent)
    }

    pub fn is_failure(self) -> bool {
        matches!(
            self,
            Self::CustomerNotFriend
                | Self::DuplicateDelivery
                | Self::ReceiveLimitExceeded
                | Self::Other
        )
    }
}

impl ExternalContactGroupMessageSendResult {
    pub fn status_kind(&self) -> Option<ExternalContactGroupMessageSendStatusKind> {
        self.status
            .map(ExternalContactGroupMessageSendStatusKind::from_code)
    }

    pub fn is_sent(&self) -> bool {
        self.status_kind()
            .is_some_and(ExternalContactGroupMessageSendStatusKind::is_sent)
    }

    pub fn is_failed(&self) -> bool {
        self.status_kind()
            .is_some_and(ExternalContactGroupMessageSendStatusKind::is_failure)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactGroupMessageResultRequest {
    pub msgid: String,
    pub limit: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactGroupMessageResultResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub detail_list: Vec<ExternalContactGroupMessageSendResult>,
    #[serde(default)]
    pub next_cursor: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactWelcomeMessageRequest {
    pub welcome_code: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<ExternalContactMessageText>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attachments: Vec<ExternalContactMessageAttachment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalCustomerTransferRequest {
    pub handover_userid: String,
    pub takeover_userid: String,
    pub external_userid: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transfer_success_msg: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactUnassignedTransferRequest {
    pub external_userid: String,
    pub handover_userid: String,
    pub takeover_userid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResignedExternalCustomerTransferRequest {
    pub handover_userid: String,
    pub takeover_userid: String,
    pub external_userid: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalCustomerTransferResultRequest {
    pub handover_userid: String,
    pub takeover_userid: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalCustomerTransferResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub customer: Vec<ExternalCustomerTransferRecord>,
    #[serde(default)]
    pub next_cursor: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

pub type ExternalCustomerTransferResultResponse = ExternalCustomerTransferResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalCustomerTransferRecord {
    #[serde(default)]
    pub external_userid: Option<String>,
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub takeover_time: Option<i64>,
    #[serde(default)]
    pub handover_userid: Option<String>,
    #[serde(default)]
    pub takeover_userid: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExternalCustomerTransferStatusKind {
    Completed,
    Pending,
    CustomerRefused,
    TakeoverLimitExceeded,
    NoRecord,
    Other,
}

impl ExternalCustomerTransferStatusKind {
    pub fn from_code(code: i64) -> Self {
        match code {
            1 => Self::Completed,
            2 => Self::Pending,
            3 => Self::CustomerRefused,
            4 => Self::TakeoverLimitExceeded,
            5 => Self::NoRecord,
            _ => Self::Other,
        }
    }

    pub fn is_terminal(self) -> bool {
        !matches!(self, Self::Pending)
    }

    pub fn is_failure(self) -> bool {
        matches!(
            self,
            Self::CustomerRefused | Self::TakeoverLimitExceeded | Self::NoRecord | Self::Other
        )
    }
}

impl ExternalCustomerTransferRecord {
    pub fn status_kind(&self) -> Option<ExternalCustomerTransferStatusKind> {
        self.status
            .map(ExternalCustomerTransferStatusKind::from_code)
    }

    pub fn is_completed(&self) -> bool {
        self.status_kind() == Some(ExternalCustomerTransferStatusKind::Completed)
    }

    pub fn is_pending(&self) -> bool {
        self.status_kind() == Some(ExternalCustomerTransferStatusKind::Pending)
    }

    pub fn is_failed(&self) -> bool {
        self.status_kind().is_some_and(|status| status.is_failure())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactUnassignedListRequest {
    pub page_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    pub page_size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactUnassignedListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub info: Vec<ExternalContactUnassignedInfo>,
    #[serde(default)]
    pub next_cursor: Option<String>,
    #[serde(default)]
    pub is_last: Option<bool>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactUnassignedInfo {
    #[serde(default)]
    pub handover_userid: Option<String>,
    #[serde(default)]
    pub external_userid: Option<String>,
    #[serde(default)]
    pub dimission_time: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMomentListRequest {
    pub start_time: i64,
    pub end_time: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_type: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMomentListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub moment_list: Vec<ExternalContactMomentSummary>,
    #[serde(default)]
    pub next_cursor: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMomentSummary {
    #[serde(default)]
    pub moment_id: Option<String>,
    #[serde(default)]
    pub creator: Option<String>,
    #[serde(default)]
    pub create_time: Option<i64>,
    #[serde(default)]
    pub create_type: Option<i64>,
    #[serde(default)]
    pub visible_type: Option<i64>,
    #[serde(default)]
    pub text: Option<ExternalContactMessageText>,
    #[serde(default)]
    pub attachments: Vec<ExternalContactMessageAttachment>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMomentTaskResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub task_list: Vec<ExternalContactMomentTask>,
    #[serde(default)]
    pub next_cursor: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMomentTask {
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub publish_status: Option<i64>,
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl ExternalContactMomentTask {
    pub fn publish_status_kind(&self) -> Option<ExternalContactMomentPublishStatusKind> {
        self.publish_status
            .map(ExternalContactMomentPublishStatusKind::from_code)
    }

    pub fn is_published(&self) -> bool {
        self.publish_status_kind()
            .is_some_and(ExternalContactMomentPublishStatusKind::is_published)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExternalContactMomentPublishStatusKind {
    Unpublished,
    Published,
    Other,
}

impl ExternalContactMomentPublishStatusKind {
    pub fn from_code(code: i64) -> Self {
        match code {
            0 => Self::Unpublished,
            1 => Self::Published,
            _ => Self::Other,
        }
    }

    pub fn is_published(self) -> bool {
        matches!(self, Self::Published)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMomentCustomerListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub customer_list: Vec<ExternalContactMomentCustomer>,
    #[serde(default)]
    pub next_cursor: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMomentCustomer {
    #[serde(default)]
    pub external_userid: Option<String>,
    #[serde(default)]
    pub publish_status: Option<i64>,
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl ExternalContactMomentCustomer {
    pub fn publish_status_kind(&self) -> Option<ExternalContactMomentPublishStatusKind> {
        self.publish_status
            .map(ExternalContactMomentPublishStatusKind::from_code)
    }

    pub fn is_published(&self) -> bool {
        self.publish_status_kind()
            .is_some_and(ExternalContactMomentPublishStatusKind::is_published)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMomentCommentResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub comment_list: Vec<ExternalContactMomentComment>,
    #[serde(default)]
    pub like_list: Vec<ExternalContactMomentLike>,
    #[serde(default)]
    pub next_cursor: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

pub trait ExternalContactCursorPage {
    fn next_cursor(&self) -> Option<&str>;

    fn has_more(&self) -> bool {
        self.next_cursor()
            .is_some_and(|cursor| !cursor.trim().is_empty())
    }
}

macro_rules! impl_external_contact_cursor_page {
    ($($response:ty),+ $(,)?) => {
        $(
            impl ExternalContactCursorPage for $response {
                fn next_cursor(&self) -> Option<&str> {
                    self.next_cursor.as_deref()
                }
            }
        )+
    };
}

impl_external_contact_cursor_page!(
    ExternalContactServedListResponse,
    ExternalContactDetailResponse,
    ExternalContactBatchGetResponse,
    ContactWayListResponse,
    ExternalGroupChatListResponse,
    ExternalContactMomentStrategyListResponse,
    ExternalContactMomentStrategyRangeResponse,
    CustomerAcquisitionLinkListResponse,
    ExternalContactProductAlbumListResponse,
    CustomerAcquisitionCustomerListResponse,
    ExternalContactGroupMessageListResponse,
    ExternalContactGroupMessageTaskResponse,
    ExternalContactGroupMessageSendResultResponse,
    ExternalContactGroupMessageResultResponse,
    ExternalCustomerTransferResponse,
    ExternalContactUnassignedListResponse,
    ExternalContactMomentListResponse,
    ExternalContactMomentTaskResponse,
    ExternalContactMomentCustomerListResponse,
    ExternalContactMomentCommentResponse,
);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMomentComment {
    #[serde(default)]
    pub external_userid: Option<String>,
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub comment_id: Option<String>,
    #[serde(default)]
    pub create_time: Option<i64>,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMomentLike {
    #[serde(default)]
    pub external_userid: Option<String>,
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub create_time: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMomentTaskRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<ExternalContactMessageText>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attachments: Vec<ExternalContactMessageAttachment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible_range: Option<ExternalContactMomentVisibleRange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMomentVisibleRange {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender_list: Option<ExternalContactMomentSenderList>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_contact_list: Option<ExternalContactMomentExternalContactList>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMomentSenderList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub user_list: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMomentExternalContactList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tag_list: Vec<String>,
}

impl ExternalContactMomentVisibleRange {
    pub fn sender_users(user_ids: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            sender_list: Some(ExternalContactMomentSenderList {
                user_list: user_ids.into_iter().map(Into::into).collect(),
            }),
            external_contact_list: None,
        }
    }

    pub fn with_external_contact_tags(
        mut self,
        tag_ids: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.external_contact_list = Some(ExternalContactMomentExternalContactList {
            tag_list: tag_ids.into_iter().map(Into::into).collect(),
        });
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMomentTaskCreateResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub jobid: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMomentTaskResultResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default, rename = "type")]
    pub result_type: Option<String>,
    #[serde(default)]
    pub result: Option<ExternalContactMomentTaskResult>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMomentTaskResult {
    #[serde(default)]
    pub moment_id: Option<String>,
    #[serde(default)]
    pub invalid_sender_list: Vec<String>,
    #[serde(default)]
    pub invalid_external_contact_list: Vec<String>,
    #[serde(default)]
    pub invalid_chat_list: Vec<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactUserBehaviorDataRequest {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub userid: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub partyid: Vec<i64>,
    pub start_time: i64,
    pub end_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactUserBehaviorDataResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub behavior_data: Vec<ExternalContactUserBehaviorData>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactUserBehaviorData {
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub stat_time: Option<i64>,
    #[serde(default)]
    pub chat_cnt: Option<i64>,
    #[serde(default)]
    pub message_cnt: Option<i64>,
    #[serde(default)]
    pub reply_percentage: Option<f64>,
    #[serde(default)]
    pub avg_reply_time: Option<i64>,
    #[serde(default)]
    pub negative_feedback_cnt: Option<i64>,
    #[serde(default)]
    pub new_apply_cnt: Option<i64>,
    #[serde(default)]
    pub new_contact_cnt: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalGroupChatStatisticRequest {
    pub day_begin_time: i64,
    pub day_end_time: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_filter: Option<ExternalContactOwnerFilter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_by: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_asc: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalGroupChatStatisticByDayRequest {
    pub day_begin_time: i64,
    pub day_end_time: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_filter: Option<ExternalContactOwnerFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalGroupChatStatisticResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub total: Option<i64>,
    #[serde(default)]
    pub next_offset: Option<i64>,
    #[serde(default)]
    pub items: Vec<ExternalGroupChatStatisticItem>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalGroupChatStatisticItem {
    #[serde(default)]
    pub owner: Option<String>,
    #[serde(default)]
    pub data: Option<ExternalGroupChatStatisticData>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalGroupChatStatisticData {
    #[serde(default)]
    pub new_chat_cnt: Option<i64>,
    #[serde(default)]
    pub chat_total: Option<i64>,
    #[serde(default)]
    pub chat_has_msg: Option<i64>,
    #[serde(default)]
    pub new_member_cnt: Option<i64>,
    #[serde(default)]
    pub member_total: Option<i64>,
    #[serde(default)]
    pub member_has_msg: Option<i64>,
    #[serde(default)]
    pub msg_total: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactCustomerStrategyPrivilege {
    pub view_customer_list: bool,
    pub view_customer_data: bool,
    pub view_room_list: bool,
    pub contact_me: bool,
    pub join_room: bool,
    pub share_customer: bool,
    pub oper_resign_customer: bool,
    pub send_customer_msg: bool,
    pub edit_welcome_msg: bool,
    pub view_behavior_data: bool,
    pub view_room_data: bool,
    pub send_group_msg: bool,
    pub room_deduplication: bool,
    pub rapid_reply: bool,
    pub onjob_customer_transfer: bool,
    pub edit_anti_spam_rule: bool,
    pub export_customer_list: bool,
    pub export_customer_data: bool,
    pub export_customer_group_list: bool,
    pub manage_customer_tag: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactCustomerStrategyRange {
    #[serde(rename = "type")]
    pub kind: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partyid: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub userid: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactCustomerStrategyCreateRequest {
    pub parent_id: i64,
    pub strategy_name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub admin_list: Vec<String>,
    pub privilege: ExternalContactCustomerStrategyPrivilege,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub range: Vec<ExternalContactCustomerStrategyRange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactCustomerStrategyEditRequest {
    pub strategy_id: i64,
    pub strategy_name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub admin_list: Vec<String>,
    pub privilege: ExternalContactCustomerStrategyPrivilege,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub range_add: Vec<ExternalContactCustomerStrategyRange>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub range_del: Vec<ExternalContactCustomerStrategyRange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactCustomerStrategyListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default, alias = "momentStrategy")]
    pub strategy: Vec<ExternalContactCustomerStrategy>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactCustomerStrategyResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default, alias = "momentStrategy")]
    pub strategy: Option<ExternalContactCustomerStrategy>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactCustomerStrategy {
    #[serde(default)]
    pub strategy_id: Option<i64>,
    #[serde(default)]
    pub parent_id: Option<i64>,
    #[serde(default)]
    pub strategy_name: Option<String>,
    #[serde(default)]
    pub admin_list: Vec<String>,
    #[serde(default)]
    pub privilege: Option<ExternalContactCustomerStrategyPrivilege>,
    #[serde(default)]
    pub create_time: Option<i64>,
    #[serde(default)]
    pub update_time: Option<i64>,
    #[serde(default, flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactCustomerStrategyRangeResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub range: Vec<ExternalContactCustomerStrategyRange>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactCustomerStrategyCreateResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub strategy_id: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupRobotMessage {
    pub msgtype: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<GroupRobotTextMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown: Option<GroupRobotMarkdownMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown_v2: Option<GroupRobotMarkdownMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<GroupRobotImageMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub news: Option<GroupRobotNewsMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<GroupRobotFileMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<GroupRobotVoiceMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_card: Option<GroupRobotTemplateCardMessage>,
}

impl GroupRobotMessage {
    pub fn msgtype_kind(&self) -> WorkMessageTypeKind {
        WorkMessageTypeKind::from_code(&self.msgtype)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupRobotTextMessage {
    pub content: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mentioned_list: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mentioned_mobile_list: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupRobotMarkdownMessage {
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupRobotImageMessage {
    pub base64: String,
    pub md5: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupRobotNewsMessage {
    pub articles: Vec<GroupRobotNewsArticle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupRobotNewsArticle {
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub picurl: Option<String>,
}

pub type GroupRobotTemplateCardMessage = WorkTemplateCard;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupRobotFileMessage {
    pub media_id: String,
}

pub type GroupRobotVoiceMessage = GroupRobotFileMessage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUploadImageResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone)]
pub struct WorkMediaDownload {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: bytes::Bytes,
}

fn work_media_range_header(start: u64, end_inclusive: Option<u64>) -> Result<String> {
    if end_inclusive.is_some_and(|end| end < start) {
        return Err(WechatError::Config(
            "media range end must be greater than or equal to start".to_string(),
        ));
    }
    Ok(match end_inclusive {
        Some(end) => format!("bytes={start}-{end}"),
        None => format!("bytes={start}-"),
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WorkMediaContentRange {
    pub start: u64,
    pub end_inclusive: u64,
    pub total: Option<u64>,
}

impl From<crate::client::HttpBytesResponse> for WorkMediaDownload {
    fn from(response: crate::client::HttpBytesResponse) -> Self {
        Self {
            status: response.status,
            headers: response.headers,
            body: response.body,
        }
    }
}

impl WorkMediaDownload {
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(name))
            .map(|(_, value)| value.as_str())
    }

    pub fn content_type(&self) -> Option<&str> {
        self.header("content-type")
    }

    pub fn content_disposition(&self) -> Option<&str> {
        self.header("content-disposition")
    }

    pub fn file_name(&self) -> Option<&str> {
        let name = self.content_disposition().and_then(|value| {
            value.split(';').find_map(|part| {
                let (name, value) = part.trim().split_once('=')?;
                name.eq_ignore_ascii_case("filename")
                    .then(|| value.trim().trim_matches('"'))
            })
        })?;
        let name = name.rsplit(['/', '\\']).next()?.trim();
        (!name.is_empty() && name != "." && name != "..").then_some(name)
    }

    pub fn content_length(&self) -> Option<u64> {
        self.header("content-length")?.parse().ok()
    }

    pub fn accepts_byte_ranges(&self) -> bool {
        self.header("accept-ranges")
            .is_some_and(|value| value.eq_ignore_ascii_case("bytes"))
    }

    pub fn content_range(&self) -> Option<WorkMediaContentRange> {
        let value = self.header("content-range")?.trim();
        let value = value.strip_prefix("bytes ")?;
        let (range, total) = value.split_once('/')?;
        let (start, end) = range.split_once('-')?;
        Some(WorkMediaContentRange {
            start: start.parse().ok()?,
            end_inclusive: end.parse().ok()?,
            total: if total == "*" {
                None
            } else {
                total.parse().ok()
            },
        })
    }

    pub fn is_partial(&self) -> bool {
        self.status == 206
    }

    pub fn total_length(&self) -> Option<u64> {
        if let Some(total) = self.content_range().and_then(|range| range.total) {
            Some(total)
        } else if self.is_partial() {
            None
        } else {
            self.content_length()
        }
    }

    pub fn next_range_start(&self) -> Option<u64> {
        let range = self.content_range()?;
        let next = range.end_inclusive.checked_add(1)?;
        match range.total {
            Some(total) if next >= total => None,
            _ => Some(next),
        }
    }
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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkMediaTypeKind {
    Image,
    Voice,
    Video,
    File,
    Other,
}

impl WorkMediaTypeKind {
    pub fn from_code(value: &str) -> Self {
        if value.eq_ignore_ascii_case("image") {
            Self::Image
        } else if value.eq_ignore_ascii_case("voice") {
            Self::Voice
        } else if value.eq_ignore_ascii_case("video") {
            Self::Video
        } else if value.eq_ignore_ascii_case("file") {
            Self::File
        } else {
            Self::Other
        }
    }

    pub fn is_binary_file(self) -> bool {
        matches!(self, Self::Image | Self::Voice | Self::Video | Self::File)
    }
}

impl WorkUploadMediaResponse {
    pub fn media_type_kind(&self) -> Option<WorkMediaTypeKind> {
        self.media_type.as_deref().map(WorkMediaTypeKind::from_code)
    }

    pub fn is_image(&self) -> bool {
        self.media_type_kind() == Some(WorkMediaTypeKind::Image)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkMediaUploadByUrlSceneKind {
    ExternalContactGroupWelcome,
}

impl WorkMediaUploadByUrlSceneKind {
    pub const fn as_code(self) -> i64 {
        match self {
            Self::ExternalContactGroupWelcome => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkMediaUploadByUrlTypeKind {
    Video,
    File,
}

impl WorkMediaUploadByUrlTypeKind {
    pub const fn as_code(self) -> &'static str {
        match self {
            Self::Video => "video",
            Self::File => "file",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMediaUploadByUrlRequest {
    pub scene: i64,
    #[serde(rename = "type")]
    pub media_type: String,
    pub filename: String,
    pub url: String,
    pub md5: String,
}

impl WorkMediaUploadByUrlRequest {
    pub fn new(
        scene: WorkMediaUploadByUrlSceneKind,
        media_type: WorkMediaUploadByUrlTypeKind,
        filename: impl Into<String>,
        url: impl Into<String>,
        md5: impl Into<String>,
    ) -> Self {
        Self {
            scene: scene.as_code(),
            media_type: media_type.as_code().to_string(),
            filename: filename.into(),
            url: url.into(),
            md5: md5.into(),
        }
    }

    pub fn scene_kind(&self) -> Option<WorkMediaUploadByUrlSceneKind> {
        (self.scene == WorkMediaUploadByUrlSceneKind::ExternalContactGroupWelcome.as_code())
            .then_some(WorkMediaUploadByUrlSceneKind::ExternalContactGroupWelcome)
    }

    pub fn media_type_kind(&self) -> Option<WorkMediaUploadByUrlTypeKind> {
        if self
            .media_type
            .eq_ignore_ascii_case(WorkMediaUploadByUrlTypeKind::Video.as_code())
        {
            Some(WorkMediaUploadByUrlTypeKind::Video)
        } else if self
            .media_type
            .eq_ignore_ascii_case(WorkMediaUploadByUrlTypeKind::File.as_code())
        {
            Some(WorkMediaUploadByUrlTypeKind::File)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMediaUploadByUrlResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub jobid: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMediaUploadByUrlResultRequest {
    pub jobid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMediaUploadByUrlResultResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default)]
    pub detail: Option<WorkMediaUploadByUrlResultDetail>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMediaUploadByUrlResultDetail {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub media_id: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkMediaUploadByUrlStatusKind {
    Processing,
    Completed,
    Failed,
    Other(i64),
}

impl From<i64> for WorkMediaUploadByUrlStatusKind {
    fn from(value: i64) -> Self {
        match value {
            1 => Self::Processing,
            2 => Self::Completed,
            3 => Self::Failed,
            other => Self::Other(other),
        }
    }
}

impl WorkMediaUploadByUrlStatusKind {
    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::Failed)
    }
}

impl WorkMediaUploadByUrlResultResponse {
    pub fn status_kind(&self) -> Option<WorkMediaUploadByUrlStatusKind> {
        self.status.map(WorkMediaUploadByUrlStatusKind::from)
    }

    pub fn is_completed(&self) -> bool {
        self.status_kind() == Some(WorkMediaUploadByUrlStatusKind::Completed)
    }

    pub fn is_successful(&self) -> bool {
        self.is_completed()
            && self
                .detail
                .as_ref()
                .and_then(|detail| detail.errcode)
                .is_some_and(|errcode| errcode == 0)
    }
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
pub struct WorkMsgAuditCheckSingleAgreeRequest {
    pub info: Vec<WorkMsgAuditConversationPair>,
}

impl WorkMsgAuditCheckSingleAgreeRequest {
    pub fn new(info: impl IntoIterator<Item = WorkMsgAuditConversationPair>) -> Self {
        Self {
            info: info.into_iter().collect(),
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.info.is_empty() {
            return Err(WechatError::Config(
                "message-audit agreement query requires at least one conversation pair".to_string(),
            ));
        }
        if self.info.iter().any(|item| item.userid.trim().is_empty()) {
            return Err(WechatError::Config(
                "message-audit agreement query userid cannot be empty".to_string(),
            ));
        }
        if self
            .info
            .iter()
            .any(|item| item.external_openid.trim().is_empty())
        {
            return Err(WechatError::Config(
                "message-audit agreement query external OpenID cannot be empty".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMsgAuditConversationPair {
    pub userid: String,
    #[serde(rename = "exteranalopenid")]
    pub external_openid: String,
}

impl WorkMsgAuditConversationPair {
    pub fn new(userid: impl Into<String>, external_openid: impl Into<String>) -> Self {
        Self {
            userid: userid.into(),
            external_openid: external_openid.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMsgAuditPermitUsersResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub ids: Vec<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMsgAuditChatDataResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub chatdata: Vec<WorkMsgAuditChatData>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMsgAuditChatData {
    #[serde(default)]
    pub seq: Option<i64>,
    #[serde(default)]
    pub msgid: Option<String>,
    #[serde(default)]
    pub publickey_ver: Option<i64>,
    #[serde(default)]
    pub encrypt_random_key: Option<String>,
    #[serde(default)]
    pub encrypt_chat_msg: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMsgAuditRoomResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub roomname: Option<String>,
    #[serde(default)]
    pub creator: Option<String>,
    #[serde(default)]
    pub room_create_time: Option<i64>,
    #[serde(default)]
    pub notice: Option<String>,
    #[serde(default)]
    pub members: Vec<WorkMsgAuditRoomMember>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMsgAuditRoomMember {
    #[serde(default)]
    pub memberid: Option<String>,
    #[serde(default)]
    pub jointime: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMsgAuditAgreeResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub agreeinfo: Vec<WorkMsgAuditAgreeInfo>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMsgAuditAgreeInfo {
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default, rename = "exteranalopenid")]
    pub external_openid: Option<String>,
    #[serde(default)]
    pub agree_status: Option<String>,
    #[serde(default)]
    pub status_change_time: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkMsgAuditAgreeStatusKind {
    Agree,
    Disagree,
    Other,
}

impl WorkMsgAuditAgreeStatusKind {
    pub fn from_code(value: &str) -> Self {
        if value.eq_ignore_ascii_case("Agree") {
            Self::Agree
        } else if value.eq_ignore_ascii_case("Disagree") {
            Self::Disagree
        } else {
            Self::Other
        }
    }

    pub fn is_agreed(self) -> bool {
        matches!(self, Self::Agree)
    }

    pub fn is_disagreed(self) -> bool {
        matches!(self, Self::Disagree)
    }
}

impl WorkMsgAuditAgreeInfo {
    pub fn status_kind(&self) -> Option<WorkMsgAuditAgreeStatusKind> {
        self.agree_status
            .as_deref()
            .map(WorkMsgAuditAgreeStatusKind::from_code)
    }

    pub fn is_agreed(&self) -> bool {
        self.status_kind()
            .is_some_and(WorkMsgAuditAgreeStatusKind::is_agreed)
    }

    pub fn is_disagreed(&self) -> bool {
        self.status_kind()
            .is_some_and(WorkMsgAuditAgreeStatusKind::is_disagreed)
    }
}

impl WorkMsgAuditAgreeResponse {
    pub fn all_agreed(&self) -> bool {
        !self.agreeinfo.is_empty() && self.agreeinfo.iter().all(WorkMsgAuditAgreeInfo::is_agreed)
    }

    pub fn has_disagreement(&self) -> bool {
        self.agreeinfo
            .iter()
            .any(WorkMsgAuditAgreeInfo::is_disagreed)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMsgAuditRobotInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub robot_info: Option<WorkMsgAuditRobotInfo>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMsgAuditRobotInfo {
    #[serde(default)]
    pub robot_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub creator_userid: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

pub const WORK_AIBOT_CMD_SUBSCRIBE: &str = "aibot_subscribe";
pub const WORK_AIBOT_CMD_PING: &str = "ping";
pub const WORK_AIBOT_CMD_RESPOND_WELCOME: &str = "aibot_respond_welcome_msg";
pub const WORK_AIBOT_CMD_RESPOND_MESSAGE: &str = "aibot_respond_msg";
pub const WORK_AIBOT_CMD_RESPOND_UPDATE_MESSAGE: &str = "aibot_respond_update_msg";
pub const WORK_AIBOT_CMD_SEND_MESSAGE: &str = "aibot_send_msg";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAiBotLongConnectionHeaders {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub req_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAiBotLongConnectionRequest {
    pub cmd: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<WorkAiBotLongConnectionHeaders>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAiBotLongConnectionResponse {
    #[serde(default)]
    pub cmd: Option<String>,
    #[serde(default)]
    pub headers: Option<WorkAiBotLongConnectionHeaders>,
    #[serde(default)]
    pub body: Option<Value>,
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl WorkAiBotLongConnectionResponse {
    pub fn is_error(&self) -> bool {
        self.errcode.unwrap_or_default() != 0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceAccountUpdateRequest {
    pub open_kfid: String,
    pub name: String,
    pub media_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceAddContactWayRequest {
    pub open_kfid: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scene: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceAccountAddResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub open_kfid: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceAccountListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub account_list: Vec<WorkAccountServiceAccount>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceAccount {
    #[serde(default)]
    pub open_kfid: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub avatar: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceAddContactWayResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceCustomerBatchGetRequest {
    pub external_userid_list: Vec<String>,
    pub need_enter_session_context: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceCustomerBatchGetResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub customer_list: Vec<WorkAccountServiceCustomer>,
    #[serde(default)]
    pub invalid_external_userid: Vec<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceCustomer {
    #[serde(default)]
    pub external_userid: Option<String>,
    #[serde(default)]
    pub nickname: Option<String>,
    #[serde(default)]
    pub avatar: Option<String>,
    #[serde(default)]
    pub gender: Option<i64>,
    #[serde(default)]
    pub unionid: Option<String>,
    #[serde(default)]
    pub enter_session_context: Option<WorkAccountServiceEnterSessionContext>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceEnterSessionContext {
    #[serde(default)]
    pub scene: Option<String>,
    #[serde(default)]
    pub scene_param: Option<String>,
    #[serde(default)]
    pub wechat_channels: Option<Value>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceCustomerUpgradeServiceConfigResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub member_range: Option<Value>,
    #[serde(default)]
    pub groupchat_range: Option<Value>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceCustomerUpgradeServiceRequest {
    pub open_kfid: String,
    pub external_userid: String,
    #[serde(rename = "type")]
    pub upgrade_type: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groupchat: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceSyncMsgRequest {
    pub cursor: String,
    pub token: String,
    pub limit: i64,
    pub voice_format: i64,
    pub open_kfid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceSyncMsgResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub next_cursor: Option<String>,
    #[serde(default)]
    pub has_more: Option<i64>,
    #[serde(default)]
    pub msg_list: Vec<WorkAccountServiceMessage>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceMessage {
    #[serde(default)]
    pub msgid: Option<String>,
    #[serde(default)]
    pub open_kfid: Option<String>,
    #[serde(default)]
    pub external_userid: Option<String>,
    #[serde(default)]
    pub send_time: Option<i64>,
    #[serde(default)]
    pub origin: Option<i64>,
    #[serde(default)]
    pub servicer_userid: Option<String>,
    #[serde(default)]
    pub msgtype: Option<String>,
    #[serde(default)]
    pub text: Option<WorkAccountServiceTextMessage>,
    #[serde(default)]
    pub image: Option<WorkAccountServiceMediaMessage>,
    #[serde(default)]
    pub voice: Option<WorkAccountServiceMediaMessage>,
    #[serde(default)]
    pub video: Option<WorkAccountServiceVideoMessage>,
    #[serde(default)]
    pub file: Option<WorkAccountServiceMediaMessage>,
    #[serde(default)]
    pub location: Option<WorkAccountServiceLocationMessage>,
    #[serde(default)]
    pub link: Option<WorkAccountServiceLinkMessage>,
    #[serde(default)]
    pub business_card: Option<WorkAccountServiceBusinessCardMessage>,
    #[serde(default)]
    pub miniprogram: Option<WorkAccountServiceMiniProgramMessage>,
    #[serde(default)]
    pub msgmenu: Option<WorkAccountServiceMenuMessage>,
    #[serde(default)]
    pub channels_shop_product: Option<WorkAccountServiceChannelsShopProductMessage>,
    #[serde(default)]
    pub channels_shop_order: Option<WorkAccountServiceChannelsShopOrderMessage>,
    #[serde(default)]
    pub event: Option<WorkAccountServiceEventMessage>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkAccountServiceMessageOriginKind {
    Customer,
    System,
    Servicer,
    IntelligentAssistant,
    Other(i64),
}

impl From<i64> for WorkAccountServiceMessageOriginKind {
    fn from(value: i64) -> Self {
        match value {
            3 => Self::Customer,
            4 => Self::System,
            5 => Self::Servicer,
            6 => Self::IntelligentAssistant,
            other => Self::Other(other),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkAccountServiceMessageTypeKind {
    Text,
    Image,
    Voice,
    Video,
    File,
    Location,
    Link,
    BusinessCard,
    MiniProgram,
    Menu,
    ChannelsShopProduct,
    ChannelsShopOrder,
    Event,
    Other,
}

impl WorkAccountServiceMessageTypeKind {
    pub fn from_code(code: &str) -> Self {
        match code {
            "text" => Self::Text,
            "image" => Self::Image,
            "voice" => Self::Voice,
            "video" => Self::Video,
            "file" => Self::File,
            "location" => Self::Location,
            "link" => Self::Link,
            "business_card" => Self::BusinessCard,
            "miniprogram" => Self::MiniProgram,
            "msgmenu" => Self::Menu,
            "channels_shop_product" => Self::ChannelsShopProduct,
            "channels_shop_order" => Self::ChannelsShopOrder,
            "event" => Self::Event,
            _ => Self::Other,
        }
    }

    pub fn as_code(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Image => "image",
            Self::Voice => "voice",
            Self::Video => "video",
            Self::File => "file",
            Self::Location => "location",
            Self::Link => "link",
            Self::BusinessCard => "business_card",
            Self::MiniProgram => "miniprogram",
            Self::Menu => "msgmenu",
            Self::ChannelsShopProduct => "channels_shop_product",
            Self::ChannelsShopOrder => "channels_shop_order",
            Self::Event => "event",
            Self::Other => "unknown",
        }
    }
}

impl WorkAccountServiceMessage {
    pub fn origin_kind(&self) -> Option<WorkAccountServiceMessageOriginKind> {
        self.origin.map(WorkAccountServiceMessageOriginKind::from)
    }

    pub fn msgtype_kind(&self) -> Option<WorkAccountServiceMessageTypeKind> {
        self.msgtype
            .as_deref()
            .map(WorkAccountServiceMessageTypeKind::from_code)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceTextMessage {
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub menu_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceMediaMessage {
    #[serde(default)]
    pub media_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceVideoMessage {
    #[serde(default)]
    pub media_id: Option<String>,
    #[serde(default)]
    pub thumb_media_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceLocationMessage {
    #[serde(default)]
    pub latitude: Option<f64>,
    #[serde(default)]
    pub longitude: Option<f64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceLinkMessage {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub desc: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub thumb_media_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pic_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceBusinessCardMessage {
    #[serde(default)]
    pub userid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceMiniProgramMessage {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub appid: Option<String>,
    #[serde(default)]
    pub pagepath: Option<String>,
    #[serde(default)]
    pub thumb_media_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceChannelsShopProductMessage {
    #[serde(default)]
    pub product_id: Option<String>,
    #[serde(default)]
    pub head_img: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub sales_price: Option<String>,
    #[serde(default)]
    pub shop_nickname: Option<String>,
    #[serde(default)]
    pub shop_head_img: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceChannelsShopOrderMessage {
    #[serde(default)]
    pub order_id: Option<String>,
    #[serde(default)]
    pub product_titles: Option<String>,
    #[serde(default)]
    pub price_wording: Option<String>,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default)]
    pub image_url: Option<String>,
    #[serde(default)]
    pub shop_nickname: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceMenuMessage {
    #[serde(default)]
    pub head_content: Option<String>,
    #[serde(default)]
    pub list: Vec<WorkAccountServiceMenuItem>,
    #[serde(default)]
    pub tail_content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceMenuItem {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceEventMessage {
    #[serde(default)]
    pub event_type: Option<String>,
    #[serde(default)]
    pub open_kfid: Option<String>,
    #[serde(default)]
    pub external_userid: Option<String>,
    #[serde(default)]
    pub scene: Option<String>,
    #[serde(default)]
    pub scene_param: Option<String>,
    #[serde(default)]
    pub welcome_code: Option<String>,
    #[serde(default)]
    pub menu_id: Option<String>,
    #[serde(default)]
    pub msgid: Option<String>,
    #[serde(default)]
    pub fail_msgid: Option<String>,
    #[serde(default)]
    pub fail_type: Option<i64>,
    #[serde(default)]
    pub servicer_userid: Option<String>,
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default)]
    pub old_servicer_userid: Option<String>,
    #[serde(default)]
    pub new_servicer_userid: Option<String>,
    #[serde(default)]
    pub change_type: Option<i64>,
    #[serde(default)]
    pub msg_code: Option<String>,
    #[serde(default)]
    pub recall_msgid: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkAccountServiceEventTypeKind {
    EnterSession,
    MessageSendFailed,
    ServicerStatusChanged,
    SessionStatusChanged,
    UserRecalledMessage,
    ServicerRecalledMessage,
    Other,
}

impl WorkAccountServiceEventTypeKind {
    pub fn from_code(code: &str) -> Self {
        match code {
            "enter_session" => Self::EnterSession,
            "msg_send_fail" => Self::MessageSendFailed,
            "servicer_status_change" => Self::ServicerStatusChanged,
            "session_status_change" => Self::SessionStatusChanged,
            "user_recall_msg" => Self::UserRecalledMessage,
            "servicer_recall_msg" => Self::ServicerRecalledMessage,
            _ => Self::Other,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkAccountServiceMessageFailKind {
    Unknown,
    AccountDeleted,
    ApplicationClosed,
    SessionExpired,
    SessionClosed,
    MessageLimitExceeded,
    ChannelsNotBound,
    SubjectNotVerified,
    ChannelsNotBoundAndSubjectNotVerified,
    UserRejected,
    Other(i64),
}

impl From<i64> for WorkAccountServiceMessageFailKind {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::AccountDeleted,
            2 => Self::ApplicationClosed,
            4 => Self::SessionExpired,
            5 => Self::SessionClosed,
            6 => Self::MessageLimitExceeded,
            7 => Self::ChannelsNotBound,
            8 => Self::SubjectNotVerified,
            9 => Self::ChannelsNotBoundAndSubjectNotVerified,
            10 => Self::UserRejected,
            other => Self::Other(other),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkAccountServiceServicerEventStatusKind {
    Receiving,
    Stopped,
    Other(i64),
}

impl From<i64> for WorkAccountServiceServicerEventStatusKind {
    fn from(value: i64) -> Self {
        match value {
            1 => Self::Receiving,
            2 => Self::Stopped,
            other => Self::Other(other),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkAccountServiceSessionChangeKind {
    AcceptedFromPool,
    Transferred,
    Ended,
    Reaccepted,
    Other(i64),
}

impl From<i64> for WorkAccountServiceSessionChangeKind {
    fn from(value: i64) -> Self {
        match value {
            1 => Self::AcceptedFromPool,
            2 => Self::Transferred,
            3 => Self::Ended,
            4 => Self::Reaccepted,
            other => Self::Other(other),
        }
    }
}

impl WorkAccountServiceEventMessage {
    pub fn event_type_kind(&self) -> Option<WorkAccountServiceEventTypeKind> {
        self.event_type
            .as_deref()
            .map(WorkAccountServiceEventTypeKind::from_code)
    }

    pub fn fail_type_kind(&self) -> Option<WorkAccountServiceMessageFailKind> {
        self.fail_type.map(WorkAccountServiceMessageFailKind::from)
    }

    pub fn servicer_status_kind(&self) -> Option<WorkAccountServiceServicerEventStatusKind> {
        self.status
            .map(WorkAccountServiceServicerEventStatusKind::from)
    }

    pub fn session_change_kind(&self) -> Option<WorkAccountServiceSessionChangeKind> {
        self.change_type
            .map(WorkAccountServiceSessionChangeKind::from)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceSendMsgRequest {
    pub touser: String,
    pub open_kfid: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msgid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msgtype: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<WorkAccountServiceTextMessage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<WorkAccountServiceMediaMessage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub voice: Option<WorkAccountServiceMediaMessage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video: Option<WorkAccountServiceVideoMessage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<WorkAccountServiceMediaMessage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<WorkAccountServiceLinkMessage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub miniprogram: Option<WorkAccountServiceMiniProgramMessage>,
    #[serde(default, rename = "msgmenu", skip_serializing_if = "Option::is_none")]
    pub menu: Option<WorkAccountServiceMenuMessage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<WorkAccountServiceLocationMessage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ca_link: Option<WorkAccountServiceLinkMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceSendMsgOnEventRequest {
    pub code: String,
    pub msgid: String,
    pub msgtype: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<WorkAccountServiceTextMessage>,
    #[serde(default, rename = "msgmenu", skip_serializing_if = "Option::is_none")]
    pub menu: Option<WorkAccountServiceMenuMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceSendMsgResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub msgid: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceServicerRequest {
    pub open_kfid: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub userid_list: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub department_id_list: Vec<i64>,
}

impl WorkAccountServiceServicerRequest {
    pub fn new(open_kfid: impl Into<String>, userid_list: Vec<String>) -> Self {
        Self {
            open_kfid: open_kfid.into(),
            userid_list,
            department_id_list: Vec::new(),
        }
    }

    pub fn with_departments(open_kfid: impl Into<String>, department_id_list: Vec<i64>) -> Self {
        Self {
            open_kfid: open_kfid.into(),
            userid_list: Vec::new(),
            department_id_list,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceServicerResultResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub result_list: Vec<WorkAccountServiceServicerResult>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceServicerResult {
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub department_id: Option<i64>,
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceServicerListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub servicer_list: Vec<WorkAccountServiceServicer>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceServicer {
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub department_id: Option<i64>,
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default)]
    pub stop_type: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkAccountServiceServicerStatusKind {
    Receiving,
    Stopped,
    Other(i64),
}

impl From<i64> for WorkAccountServiceServicerStatusKind {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::Receiving,
            1 => Self::Stopped,
            other => Self::Other(other),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkAccountServiceServicerStopKind {
    Stopped,
    Suspended,
    Other(i64),
}

impl From<i64> for WorkAccountServiceServicerStopKind {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::Stopped,
            1 => Self::Suspended,
            other => Self::Other(other),
        }
    }
}

impl WorkAccountServiceServicer {
    pub fn status_kind(&self) -> Option<WorkAccountServiceServicerStatusKind> {
        self.status.map(WorkAccountServiceServicerStatusKind::from)
    }

    pub fn stop_kind(&self) -> Option<WorkAccountServiceServicerStopKind> {
        self.stop_type.map(WorkAccountServiceServicerStopKind::from)
    }

    pub fn is_receiving(&self) -> bool {
        self.status_kind() == Some(WorkAccountServiceServicerStatusKind::Receiving)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceStateGetResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub service_state: Option<i64>,
    #[serde(default)]
    pub servicer_userid: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkAccountServiceStateKind {
    Unhandled,
    IntelligentAssistant,
    WaitingPool,
    HumanServicer,
    Ended,
    Other(i64),
}

impl WorkAccountServiceStateKind {
    pub fn from_code(code: i64) -> Self {
        match code {
            0 => Self::Unhandled,
            1 => Self::IntelligentAssistant,
            2 => Self::WaitingPool,
            3 => Self::HumanServicer,
            4 => Self::Ended,
            other => Self::Other(other),
        }
    }

    pub fn as_code(self) -> i64 {
        match self {
            Self::Unhandled => 0,
            Self::IntelligentAssistant => 1,
            Self::WaitingPool => 2,
            Self::HumanServicer => 3,
            Self::Ended => 4,
            Self::Other(code) => code,
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Ended)
    }
}

impl WorkAccountServiceStateGetResponse {
    pub fn service_state_kind(&self) -> Option<WorkAccountServiceStateKind> {
        self.service_state
            .map(WorkAccountServiceStateKind::from_code)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceStateTransRequest {
    pub open_kfid: String,
    pub external_userid: String,
    pub service_state: i64,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub servicer_userid: String,
}

impl WorkAccountServiceStateTransRequest {
    pub fn new(
        open_kfid: impl Into<String>,
        external_userid: impl Into<String>,
        service_state: WorkAccountServiceStateKind,
    ) -> Self {
        Self {
            open_kfid: open_kfid.into(),
            external_userid: external_userid.into(),
            service_state: service_state.as_code(),
            servicer_userid: String::new(),
        }
    }

    pub fn with_servicer(
        open_kfid: impl Into<String>,
        external_userid: impl Into<String>,
        servicer_userid: impl Into<String>,
    ) -> Self {
        Self {
            open_kfid: open_kfid.into(),
            external_userid: external_userid.into(),
            service_state: WorkAccountServiceStateKind::HumanServicer.as_code(),
            servicer_userid: servicer_userid.into(),
        }
    }

    pub fn service_state_kind(&self) -> WorkAccountServiceStateKind {
        WorkAccountServiceStateKind::from_code(self.service_state)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceStateTransResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub msg_code: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceTagCreateResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub tagid: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceTagDetailResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub tagname: Option<String>,
    #[serde(default)]
    pub userlist: Vec<WorkAccountServiceTagUser>,
    #[serde(default)]
    pub partylist: Vec<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceTagUser {
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceTagUserResultResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub invalidlist: Option<String>,
    #[serde(default)]
    pub invalidparty: Vec<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceTagListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub taglist: Vec<WorkAccountServiceTag>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceTag {
    #[serde(default)]
    pub tagid: Option<i64>,
    #[serde(default)]
    pub tagname: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
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
pub struct WorkAppChatCreateResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub chatid: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAppChatGetResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub chat_info: Option<WorkAppChatInfo>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAppChatInfo {
    #[serde(default)]
    pub chatid: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub owner: Option<String>,
    #[serde(default)]
    pub userlist: Vec<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinDataRequest {
    pub opencheckindatatype: i64,
    pub starttime: i64,
    pub endtime: i64,
    pub useridlist: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinDateRangeRequest {
    pub starttime: i64,
    pub endtime: i64,
    pub useridlist: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinSetScheduleListRequest {
    pub groupid: i64,
    pub items: Vec<WorkCheckinSetScheduleItem>,
    pub yearmonth: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinSetScheduleItem {
    pub userid: String,
    pub day: i64,
    pub schedule_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinUserFaceRequest {
    pub userid: String,
    pub userface: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinOptionMutationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_now: Option<bool>,
    pub group: WorkCheckinGroup,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinCorpOptionResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub group: Vec<WorkCheckinGroup>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkCheckinGroup {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grouptype: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groupid: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groupname: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub checkindate: Vec<WorkCheckinDateRule>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub spe_workdays: Vec<WorkCheckinSpecialWorkday>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub spe_offdays: Vec<WorkCheckinSpecialOffday>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sync_holidays: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub need_photo: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note_can_use_local_pic: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_checkin_offworkday: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_apply_offworkday: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub wifimac_infos: Vec<WorkCheckinWifi>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub loc_infos: Vec<WorkCheckinLocation>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub range: Vec<WorkCheckinRange>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub white_users: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reporterinfo: Option<WorkCheckinReporterInfo>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schedulelist: Vec<WorkCheckinRuleSchedule>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ot_info: Option<WorkCheckinOvertimeRule>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ot_info_v2: Option<WorkCheckinOvertimeRuleV2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub otapplyinfo: Option<WorkCheckinOvertimeApplyRule>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uptime: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_userid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_userid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_apply_bk_cnt: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_apply_bk_day_limit: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub option_out_range: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buka_restriction: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buka_limit_next_month: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buka_remind: Option<WorkCheckinCorrectionReminder>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offwork_interval_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span_day_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub standard_work_duration: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_sp_checkin: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_face_detect: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_face_live_detect: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkin_method_type: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sync_out_checkin: Option<bool>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinDateRule {
    #[serde(default)]
    pub workdays: Vec<i64>,
    #[serde(default)]
    pub checkintime: Vec<WorkCheckinTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flex_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub noneed_offwork: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit_aheadtime: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_flex: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flex_on_duty_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flex_off_duty_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_allow_arrive_early: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_allow_arrive_late: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub late_rule: Option<WorkCheckinLateRule>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkCheckinTime {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_sec: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub off_work_sec: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remind_work_sec: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remind_off_work_sec: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_rest: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rest_begin_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rest_end_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub earliest_work_sec: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_work_sec: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub earliest_off_work_sec: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_off_work_sec: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_need_checkon: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_need_checkoff: Option<bool>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinSpecialWorkday {
    pub timestamp: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub checkintime: Vec<WorkCheckinTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub begtime: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endtime: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinSpecialOffday {
    pub timestamp: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub checkintime: Vec<WorkCheckinTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub begtime: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endtime: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinWifi {
    pub wifiname: String,
    pub wifimac: String,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinLocation {
    pub lat: i64,
    pub lng: i64,
    pub loc_title: String,
    pub loc_detail: String,
    pub distance: i64,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinRange {
    #[serde(rename = "party_id", alias = "partyid", default)]
    pub partyid: Vec<String>,
    #[serde(default)]
    pub userid: Vec<String>,
    #[serde(default)]
    pub tagid: Vec<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinReporterInfo {
    #[serde(default)]
    pub reporters: Vec<WorkCheckinReporter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updatetime: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinReporter {
    pub userid: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tagid: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinRuleSchedule {
    pub schedule_id: i64,
    pub schedule_name: String,
    #[serde(default)]
    pub time_section: Vec<WorkCheckinTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit_aheadtime: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub noneed_offwork: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit_offtime: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flex_on_duty_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flex_off_duty_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_flex: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub late_rule: Option<WorkCheckinLateRule>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_allow_arrive_early: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_allow_arrive_late: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinLateRule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_offwork_after_time: Option<WorkCheckinSwitch>,
    #[serde(default)]
    pub timerules: Vec<WorkCheckinLateTimeRule>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WorkCheckinSwitch {
    Boolean(bool),
    Integer(i64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinLateTimeRule {
    pub offwork_after_time: i64,
    pub onwork_flex_time: i64,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinOvertimeRule {
    pub r#type: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_ot_workingday: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_ot_nonworkingday: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub otcheckinfo: Option<WorkCheckinOvertimeCheckInfo>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinOvertimeApplyRule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_ot_workingday: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_ot_nonworkingday: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uptime: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ot_workingday_restinfo: Option<WorkCheckinOvertimeRestInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ot_nonworkingday_restinfo: Option<WorkCheckinOvertimeRestInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ot_nonworkingday_spanday_time: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinOvertimeRuleV2 {
    pub workdayconf: WorkCheckinOvertimeWorkdayConfig,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinOvertimeWorkdayConfig {
    pub allow_ot: bool,
    #[serde(rename = "type")]
    pub compensation_type: i64,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinCorrectionReminder {
    pub open_remind: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buka_remind_day: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buka_remind_month: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinOvertimeCheckInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ot_workingday_time_start: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ot_workingday_time_min: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ot_workingday_time_max: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ot_nonworkingday_time_min: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ot_nonworkingday_time_max: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ot_nonworkingday_spanday_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ot_workingday_restinfo: Option<WorkCheckinOvertimeRestInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ot_nonworkingday_restinfo: Option<WorkCheckinOvertimeRestInfo>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinOvertimeRestInfo {
    pub r#type: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fix_time_rule: Option<WorkCheckinOvertimeFixedRest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cal_ottime_rule: Option<WorkCheckinOvertimeCalculatedRest>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinOvertimeFixedRest {
    pub fix_time_begin_sec: i64,
    pub fix_time_end_sec: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinOvertimeCalculatedRest {
    #[serde(default)]
    pub items: Vec<WorkCheckinOvertimeCalculatedRestItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinOvertimeCalculatedRestItem {
    pub ot_time: i64,
    pub rest_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinOptionResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub info: Vec<WorkCheckinUserOption>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinUserOption {
    pub userid: String,
    pub group: WorkCheckinGroup,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinRecordResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub checkindata: Vec<WorkCheckinRecord>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinRecord {
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub groupname: Option<String>,
    #[serde(default)]
    pub checkin_type: Option<String>,
    #[serde(default)]
    pub exception_type: Option<String>,
    #[serde(default)]
    pub checkin_time: Option<i64>,
    #[serde(default)]
    pub location_title: Option<String>,
    #[serde(default)]
    pub location_detail: Option<String>,
    #[serde(default)]
    pub wifiname: Option<String>,
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default)]
    pub wifimac: Option<String>,
    #[serde(default)]
    pub mediaids: Vec<String>,
    #[serde(default)]
    pub sch_checkin_time: Option<i64>,
    #[serde(default)]
    pub groupid: Option<i64>,
    #[serde(default)]
    pub schedule_id: Option<i64>,
    #[serde(default)]
    pub timeline_id: Option<i64>,
    #[serde(default)]
    pub lat: Option<i64>,
    #[serde(default)]
    pub lng: Option<i64>,
    #[serde(default)]
    pub deviceid: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinDayDataResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub datas: Vec<WorkCheckinDayData>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

pub type WorkCheckinDataResponse = WorkCheckinDayDataResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinDayData {
    pub base_info: WorkCheckinDayBaseInfo,
    pub summary_info: WorkCheckinDaySummary,
    #[serde(default)]
    pub exception_infos: Vec<WorkCheckinException>,
    #[serde(default)]
    pub holiday_infos: Vec<WorkCheckinHoliday>,
    #[serde(default)]
    pub sp_items: Vec<WorkCheckinSpItem>,
    #[serde(default)]
    pub ot_info: Option<WorkCheckinDayOvertime>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinDayBaseInfo {
    pub date: i64,
    pub record_type: i64,
    pub name: String,
    pub name_ex: String,
    pub departs_name: String,
    pub acctid: String,
    pub day_type: i64,
    pub rule_info: WorkCheckinDayRuleInfo,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinDayRuleInfo {
    pub groupid: i64,
    pub groupname: String,
    pub scheduleid: i64,
    pub schedulename: String,
    #[serde(default)]
    pub checkintime: Vec<WorkCheckinDayTime>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinDayTime {
    pub work_sec: i64,
    pub off_work_sec: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinDaySummary {
    pub checkin_count: i64,
    pub regular_work_sec: i64,
    pub standard_work_sec: i64,
    pub earliest_time: i64,
    pub lastest_time: i64,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinException {
    pub count: i64,
    pub duration: i64,
    pub exception: i64,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinHoliday {
    pub sp_number: String,
    pub sp_title: WorkCheckinLocalizedData,
    pub sp_description: WorkCheckinLocalizedData,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinLocalizedData {
    #[serde(default)]
    pub data: Vec<WorkApprovalLocalizedText>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinSpItem {
    pub count: i64,
    pub duration: i64,
    pub time_type: i64,
    pub r#type: i64,
    pub vacation_id: i64,
    pub name: String,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinDayOvertime {
    pub ot_status: i64,
    pub ot_duration: i64,
    #[serde(default)]
    pub exception_duration: Vec<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workday_over_as_vacation: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workday_over_as_money: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restday_over_as_vacation: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restday_over_as_money: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub holiday_over_as_vacation: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub holiday_over_as_money: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinMonthDataResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub datas: Vec<WorkCheckinMonthData>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinMonthData {
    pub base_info: WorkCheckinMonthBaseInfo,
    pub summary_info: WorkCheckinMonthSummary,
    #[serde(default)]
    pub exception_infos: Vec<WorkCheckinException>,
    #[serde(default)]
    pub sp_items: Vec<WorkCheckinSpItem>,
    #[serde(default)]
    pub overwork_info: Option<WorkCheckinMonthOvertime>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinMonthBaseInfo {
    pub record_type: i64,
    pub name: String,
    pub name_ex: String,
    pub departs_name: String,
    pub acctid: String,
    pub rule_info: WorkCheckinMonthRuleInfo,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinMonthRuleInfo {
    pub groupid: i64,
    pub groupname: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinMonthSummary {
    pub work_days: i64,
    pub except_days: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regular_days: Option<i64>,
    pub regular_work_sec: i64,
    pub standard_work_sec: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rest_days: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinMonthOvertime {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workday_over_sec: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub holidays_over_sec: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restdays_over_sec: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workdays_over_as_vacation: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workdays_over_as_money: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restdays_over_as_vacation: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restdays_over_as_money: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub holidays_over_as_vacation: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub holidays_over_as_money: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinScheduleListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub schedule_list: Vec<WorkCheckinSchedule>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinSchedule {
    pub userid: String,
    pub yearmonth: i64,
    pub groupid: i64,
    pub groupname: String,
    pub schedule: WorkCheckinUserSchedule,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinUserSchedule {
    #[serde(rename = "scheduleList", alias = "schedule_list", default)]
    pub schedule_list: Vec<WorkCheckinUserScheduleDay>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinUserScheduleDay {
    pub day: i64,
    pub schedule_info: WorkCheckinUserScheduleInfo,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinUserScheduleInfo {
    pub schedule_id: i64,
    pub schedule_name: String,
    #[serde(default)]
    pub time_section: Vec<WorkCheckinUserScheduleTime>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinUserScheduleTime {
    pub id: i64,
    pub work_sec: i64,
    pub off_work_sec: i64,
    pub remind_work_sec: i64,
    pub remind_off_work_sec: i64,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalTemplateDetailResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub template_names: Vec<WorkApprovalLocalizedText>,
    #[serde(default)]
    pub template_content: Option<WorkApprovalTemplateContent>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalLocalizedText {
    pub text: String,
    pub lang: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalTemplateContent {
    #[serde(default)]
    pub controls: Vec<WorkApprovalTemplateControl>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalTemplateControl {
    pub property: WorkApprovalTemplateProperty,
    #[serde(default)]
    pub config: WorkApprovalTemplateConfig,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalTemplateProperty {
    pub control: String,
    pub id: String,
    #[serde(default)]
    pub title: Vec<WorkApprovalLocalizedText>,
    #[serde(default)]
    pub placeholder: Vec<WorkApprovalLocalizedText>,
    pub require: i64,
    pub un_print: i64,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkApprovalTemplateConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<WorkApprovalSelector>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalSelector {
    #[serde(rename = "type")]
    pub selector_type: String,
    #[serde(default)]
    pub options: Vec<WorkApprovalSelectorOption>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalSelectorOption {
    pub key: String,
    #[serde(default)]
    pub value: Vec<WorkApprovalLocalizedText>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalApplyEventResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub sp_no: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalApplyEventRequest {
    pub creator_userid: String,
    pub template_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_template_approver: Option<i64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub approver: Vec<WorkApprovalApprover>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notifyer: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_type: Option<i64>,
    pub apply_data: WorkApprovalApplyData,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub summary_list: Vec<WorkApprovalSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalApprover {
    pub attr: i64,
    #[serde(default)]
    pub userid: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalSummary {
    #[serde(default)]
    pub summary_info: Vec<WorkApprovalLocalizedText>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalApplyData {
    #[serde(default)]
    pub contents: Vec<WorkApprovalContent>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalContent {
    pub control: String,
    pub id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub title: Vec<WorkApprovalLocalizedText>,
    pub value: WorkApprovalControlValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkApprovalControlValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_money: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<WorkApprovalDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_range: Option<WorkApprovalDateRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<WorkApprovalSelector>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub members: Vec<WorkApprovalMember>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub departments: Vec<WorkApprovalDepartment>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<WorkApprovalFile>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<WorkApprovalTableRow>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_approval: Vec<WorkApprovalRelatedApplication>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tips: Vec<WorkApprovalLocalizedText>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stat_field: Vec<WorkApprovalStatField>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sum_field: Vec<WorkApprovalStatField>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalDate {
    #[serde(rename = "type")]
    pub date_type: String,
    pub s_timestamp: String,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalDateRange {
    #[serde(rename = "type")]
    pub date_type: String,
    pub new_begin: i64,
    pub new_end: i64,
    pub new_duration: i64,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalMember {
    pub userid: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalDepartment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub openapi_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalFile {
    pub file_id: String,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalTableRow {
    #[serde(default)]
    pub list: Vec<WorkApprovalContent>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalRelatedApplication {
    pub sp_no: String,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalStatField {
    pub id: String,
    #[serde(default)]
    pub title: Vec<WorkApprovalLocalizedText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exp_type: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub control: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalInfoRequest {
    pub starttime: i64,
    pub endtime: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_cursor: Option<String>,
    pub size: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filters: Vec<WorkApprovalInfoFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalInfoFilter {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub sp_no_list: Vec<String>,
    #[serde(default)]
    pub new_next_cursor: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalDetailResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub info: Option<WorkApprovalDetail>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalDetail {
    pub sp_no: String,
    pub sp_name: String,
    pub sp_status: i64,
    pub template_id: String,
    pub apply_time: i64,
    pub applyer: WorkApprovalUser,
    #[serde(default)]
    pub sp_record: Vec<WorkApprovalRecord>,
    #[serde(default)]
    pub notifyer: Vec<WorkApprovalUser>,
    pub apply_data: WorkApprovalApplyData,
    #[serde(default)]
    pub comments: Vec<WorkApprovalComment>,
    #[serde(default)]
    pub process_list: Option<WorkApprovalProcessList>,
    #[serde(default)]
    pub batch_applyer: Vec<WorkApprovalUser>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalUser {
    pub userid: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partyid: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalRecord {
    pub sp_status: i64,
    pub approverattr: i64,
    #[serde(default)]
    pub details: Vec<WorkApprovalRecordDetail>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalRecordDetail {
    pub approver: WorkApprovalUser,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speech: Option<String>,
    pub sp_status: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sptime: Option<i64>,
    #[serde(default)]
    pub media_id: Vec<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalComment {
    pub comment_user_info: WorkApprovalUser,
    pub comment_time: i64,
    pub comment_content: String,
    pub comment_id: String,
    #[serde(default)]
    pub media_id: Vec<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalProcessList {
    #[serde(default)]
    pub node_list: Vec<WorkApprovalProcessNode>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalProcessNode {
    pub node_type: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sp_status: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apv_rel: Option<i64>,
    #[serde(default)]
    pub sub_node_list: Vec<WorkApprovalProcessSubNode>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalProcessSubNode {
    pub userid: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speech: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sp_yj: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sptime: Option<i64>,
    #[serde(default)]
    pub media_ids: Vec<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalDataRequest {
    pub starttime: i64,
    pub endtime: i64,
    pub next_spnum: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalDataResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub count: Option<i64>,
    #[serde(default)]
    pub total: Option<i64>,
    #[serde(default)]
    pub next_spnum: Option<i64>,
    #[serde(default)]
    pub data: Vec<WorkLegacyApprovalRecord>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkLegacyApprovalRecord {
    pub spname: String,
    pub apply_name: String,
    pub apply_org: String,
    #[serde(default)]
    pub approval_name: Vec<String>,
    #[serde(default)]
    pub notify_name: Vec<String>,
    pub sp_status: i64,
    pub sp_num: i64,
    #[serde(default)]
    pub mediaids: Vec<String>,
    pub apply_time: i64,
    pub apply_user_id: String,
    #[serde(default)]
    pub expense: Option<WorkLegacyApprovalExpense>,
    #[serde(default)]
    pub comm: Option<WorkLegacyApprovalCommon>,
    #[serde(default)]
    pub leave: Option<WorkLegacyApprovalLeave>,
    #[serde(default)]
    pub apply_data: Vec<WorkLegacyApprovalField>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkLegacyApprovalExpense {
    pub expense_type: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default)]
    pub item: Vec<WorkLegacyApprovalExpenseItem>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkLegacyApprovalExpenseItem {
    pub expenseitem_type: i64,
    pub time: i64,
    pub sums: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkLegacyApprovalCommon {
    pub apply_data: String,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkLegacyApprovalLeave {
    pub timeunit: i64,
    pub leave_type: i64,
    pub start_time: i64,
    pub end_time: i64,
    pub duration: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkLegacyApprovalField {
    pub id: String,
    #[serde(rename = "type")]
    pub field_type: String,
    pub value: String,
    pub title: String,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkVacationConfigResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub lists: Vec<WorkVacationConfig>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkVacationConfig {
    pub id: i64,
    pub name: String,
    pub time_attr: i64,
    pub duration_type: i64,
    #[serde(default)]
    pub quota_attr: Option<WorkVacationQuotaPolicy>,
    #[serde(default)]
    pub perday_duration: Option<i64>,
    #[serde(default)]
    pub is_newovertime: Option<i64>,
    #[serde(default)]
    pub enter_comp_time_limit: Option<i64>,
    #[serde(default)]
    pub expire_rule: Option<WorkVacationExpireRule>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkVacationQuotaPolicy {
    #[serde(rename = "type")]
    pub policy_type: i64,
    #[serde(default)]
    pub autoreset_time: Option<i64>,
    #[serde(default)]
    pub autoreset_duration: Option<i64>,
    #[serde(default)]
    pub quota_rule_type: Option<i64>,
    #[serde(default)]
    pub quota_rules: Option<WorkVacationQuotaRules>,
    #[serde(default)]
    pub at_entry_date: Option<bool>,
    #[serde(default)]
    pub auto_reset_month_day: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkVacationQuotaRules {
    #[serde(default)]
    pub list: Vec<WorkVacationQuotaRule>,
    #[serde(default)]
    pub based_on_actual_work_time: Option<bool>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkVacationQuotaRule {
    pub quota: i64,
    pub begin: i64,
    pub end: i64,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkVacationExpireRule {
    #[serde(rename = "type")]
    pub rule_type: i64,
    #[serde(default)]
    pub duration: Option<i64>,
    #[serde(default)]
    pub date: Option<WorkVacationMonthDay>,
    #[serde(default)]
    pub extern_duration_enable: Option<bool>,
    #[serde(default)]
    pub extern_duration: Option<WorkVacationMonthDay>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkVacationMonthDay {
    pub month: i64,
    pub day: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkVacationQuotaResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub lists: Vec<WorkVacationQuota>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkVacationQuota {
    pub id: i64,
    pub assignduration: i64,
    pub usedduration: i64,
    pub leftduration: i64,
    pub vacationname: String,
    #[serde(default)]
    pub real_assignduration: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkVacationQuotaUpdateRequest {
    pub userid: String,
    pub vacation_id: i64,
    pub leftduration: i64,
    pub time_attr: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalCreateTemplateRequest {
    pub template_name: Vec<WorkApprovalLocalizedText>,
    pub template_content: WorkApprovalTemplateContent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalUpdateTemplateRequest {
    pub template_id: String,
    pub template_name: Vec<WorkApprovalLocalizedText>,
    pub template_content: WorkApprovalTemplateContent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalCreateTemplateResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub template_id: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalCopyTemplateResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub template_id: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCalendarAddRequest {
    pub calendar: WorkCalendarCreate,
    pub agentid: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCalendarCreate {
    pub organizer: String,
    pub summary: String,
    pub color: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub shares: Vec<WorkCalendarShareRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCalendarUpdateRequest {
    pub calendar: WorkCalendarUpdate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCalendarUpdate {
    pub cal_id: String,
    pub summary: String,
    pub color: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub shares: Vec<WorkCalendarShareRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCalendarShareRequest {
    pub userid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCalendarAddResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub cal_id: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCalendarShare {
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub readonly: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCalendarInfo {
    #[serde(default)]
    pub cal_id: Option<String>,
    #[serde(default)]
    pub adminis: Vec<String>,
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub shares: Vec<WorkCalendarShare>,
    #[serde(default)]
    pub organizer: Option<String>,
    #[serde(default)]
    pub readonly: Option<i64>,
    #[serde(default)]
    pub set_as_default: Option<i64>,
    #[serde(default)]
    pub is_public: Option<i64>,
    #[serde(default)]
    pub public_range: Option<WorkCalendarPublicRange>,
    #[serde(default)]
    pub is_corp_calendar: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCalendarPublicRange {
    #[serde(default)]
    pub userids: Vec<String>,
    #[serde(default)]
    pub partyids: Vec<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCalendarGetResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub calendar_list: Vec<WorkCalendarInfo>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkDialRecordRequest {
    pub start_time: i64,
    pub end_time: i64,
    pub offset: i64,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkDialRecord {
    #[serde(default)]
    pub callee: Option<String>,
    #[serde(default)]
    pub caller: Option<String>,
    #[serde(default)]
    pub duration: Option<i64>,
    #[serde(default)]
    pub dial_time: Option<i64>,
    #[serde(default)]
    pub call_type: Option<i64>,
    #[serde(default)]
    pub call_result: Option<i64>,
    #[serde(default)]
    pub callid: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkDialRecordResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub record: Vec<WorkDialRecord>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkJournalRecordListRequest {
    pub starttime: String,
    pub endtime: String,
    pub cursor: String,
    pub limit: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filters: Vec<WorkJournalFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkJournalFilter {
    pub key: String,
    pub value: String,
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
    pub endflag: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkJournalRecordInfo {
    #[serde(default, alias = "journaluuid")]
    pub journal_uuid: Option<String>,
    #[serde(default)]
    pub template_name: Option<String>,
    #[serde(default)]
    pub report_time: Option<i64>,
    #[serde(default)]
    pub submitter: Option<WorkJournalUser>,
    #[serde(default)]
    pub receivers: Vec<WorkJournalUser>,
    #[serde(default)]
    pub readed_receivers: Vec<WorkJournalUser>,
    #[serde(default)]
    pub apply_data: Option<WorkApprovalApplyData>,
    #[serde(default)]
    pub comments: Vec<WorkJournalComment>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkJournalUser {
    pub userid: String,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkJournalComment {
    pub commentid: i64,
    pub tocommentid: i64,
    pub comment_userinfo: WorkJournalUser,
    pub content: String,
    pub comment_time: i64,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkJournalRecordDetailResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub info: Option<WorkJournalRecordInfo>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkJournalStatListRequest {
    pub template_id: String,
    pub starttime: String,
    pub endtime: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkJournalParty {
    pub open_partyid: String,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkJournalTag {
    pub open_tagid: String,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkJournalLeader {
    pub level: i64,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkJournalRange {
    #[serde(default)]
    pub user_list: Vec<WorkJournalUser>,
    #[serde(default)]
    pub party_list: Vec<WorkJournalParty>,
    #[serde(default)]
    pub tag_list: Vec<WorkJournalTag>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkJournalReceivers {
    #[serde(default)]
    pub user_list: Vec<WorkJournalUser>,
    #[serde(default)]
    pub tag_list: Vec<WorkJournalTag>,
    #[serde(default)]
    pub leader_list: Vec<WorkJournalLeader>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkJournalReportItem {
    pub journaluuid: String,
    pub reporttime: i64,
    pub flag: i64,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkJournalReport {
    pub user: WorkJournalUser,
    #[serde(default)]
    pub itemlist: Vec<WorkJournalReportItem>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkJournalStat {
    #[serde(default)]
    pub template_id: Option<String>,
    #[serde(default)]
    pub template_name: Option<String>,
    #[serde(default)]
    pub report_range: Option<WorkJournalRange>,
    #[serde(default)]
    pub white_range: Option<WorkJournalRange>,
    #[serde(default)]
    pub receivers: Option<WorkJournalReceivers>,
    #[serde(default)]
    pub cycle_begin_time: Option<i64>,
    #[serde(default)]
    pub cycle_end_time: Option<i64>,
    #[serde(default)]
    pub stat_begin_time: Option<i64>,
    #[serde(default)]
    pub stat_end_time: Option<i64>,
    #[serde(default)]
    pub report_list: Vec<WorkJournalReport>,
    #[serde(default)]
    pub unreport_list: Vec<WorkJournalReport>,
    #[serde(default)]
    pub report_type: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkJournalStatListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub stat_list: Vec<WorkJournalStat>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkPstnccCallState {
    #[serde(default)]
    pub callee_userid: Option<String>,
    #[serde(default)]
    pub callid: Option<String>,
    #[serde(default)]
    pub state: Option<i64>,
    #[serde(default)]
    pub reason: Option<i64>,
    #[serde(default)]
    pub callee: Option<String>,
    #[serde(default)]
    pub caller: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkPstnccCallResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub states: Vec<WorkPstnccCallState>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
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
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMeetingCreateRequest {
    pub creator_userid: String,
    pub title: String,
    pub meeting_start: i64,
    pub meeting_duration: i64,
    pub description: String,
    #[serde(rename = "type")]
    pub meeting_type: i64,
    pub remind_time: i64,
    pub agentid: i64,
    pub attendees: WorkMeetingAttendeesRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMeetingUpdateRequest {
    pub meetingid: String,
    pub title: String,
    pub meeting_start: i64,
    pub meeting_duration: i64,
    pub description: String,
    #[serde(rename = "type")]
    pub meeting_type: i64,
    pub remind_time: i64,
    pub attendees: WorkMeetingAttendeesRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMeetingAttendeesRequest {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub userids: Vec<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMeetingGetUserMeetingIdRequest {
    pub userid: String,
    pub cursor: String,
    pub begin_time: i64,
    pub end_time: i64,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMeetingCreateResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub meetingid: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMeetingGetUserMeetingIdResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub next_cursor: Option<String>,
    #[serde(default)]
    pub meetingid_list: Vec<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMeetingGetInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub creator_userid: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub reserve_meeting_start: Option<i64>,
    #[serde(default)]
    pub reserve_meeting_duration: Option<i64>,
    #[serde(default)]
    pub meeting_start: Option<i64>,
    #[serde(default)]
    pub meeting_duration: Option<i64>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub main_department: Option<i64>,
    #[serde(default, rename = "type")]
    pub meeting_type: Option<i64>,
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default)]
    pub remind_time: Option<i64>,
    #[serde(default)]
    pub attendees: Option<WorkMeetingAttendees>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMeetingAttendees {
    #[serde(default)]
    pub member: Vec<WorkMeetingMemberAttendee>,
    #[serde(default)]
    pub external_user: Vec<WorkMeetingExternalAttendee>,
    #[serde(default)]
    pub device: Vec<WorkMeetingDeviceAttendee>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMeetingMemberAttendee {
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMeetingExternalAttendee {
    #[serde(default)]
    pub external_userid: Option<String>,
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMeetingDeviceAttendee {
    #[serde(default)]
    pub device_sn: Option<String>,
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMeetingRoomAddRequest {
    pub name: String,
    pub capacity: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub building: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floor: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub equipment: Vec<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinate: Option<WorkMeetingRoomCoordinate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMeetingRoomEditRequest {
    pub meetingroom_id: i64,
    pub name: String,
    pub capacity: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub building: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floor: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub equipment: Vec<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinate: Option<WorkMeetingRoomCoordinate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMeetingRoomCoordinate {
    pub latitude: String,
    pub longitude: String,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMeetingRoomListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub building: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floor: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub equipment: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMeetingRoomGetBookingInfoRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meetingroom_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub building: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMeetingRoomBookRequest {
    pub meetingroom_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    pub start_time: i64,
    pub end_time: i64,
    pub booker: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attendees: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMeetingRoomBookByScheduleRequest {
    pub meetingroom_id: i64,
    pub schedule_id: String,
    pub booker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMeetingRoomBookByMeetingRequest {
    pub meetingroom_id: i64,
    pub meetingid: String,
    pub booker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMeetingRoomCancelBookRequest {
    pub meeting_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_schedule: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMeetingRoomBookingByIdRequest {
    pub meetingroom_id: i64,
    pub booking_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMeetingRoomAddResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub meetingroom_id: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMeetingRoomInfo {
    #[serde(default)]
    pub meetingroom_id: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub capacity: Option<i64>,
    #[serde(default)]
    pub city: Option<String>,
    #[serde(default)]
    pub building: Option<String>,
    #[serde(default)]
    pub floor: Option<String>,
    #[serde(default)]
    pub equipment: Vec<i64>,
    #[serde(default)]
    pub coordinate: Option<WorkMeetingRoomCoordinate>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMeetingRoomListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub meetingroom_list: Vec<WorkMeetingRoomInfo>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMeetingRoomBooking {
    #[serde(default)]
    pub meeting_id: Option<i64>,
    #[serde(default)]
    pub schedule_id: Option<i64>,
    #[serde(default)]
    pub meetingroom_id: Option<i64>,
    #[serde(default)]
    pub subject: Option<String>,
    #[serde(default)]
    pub start_time: Option<i64>,
    #[serde(default)]
    pub end_time: Option<i64>,
    #[serde(default)]
    pub booker: Option<String>,
    #[serde(default)]
    pub attendees: Vec<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMeetingRoomGetBookingInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub booking_list: Vec<WorkMeetingRoomBooking>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMeetingRoomBookResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub meeting_id: Option<i64>,
    #[serde(default)]
    pub schedule_id: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMeetingRoomLinkedBookResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub booking_id: Option<String>,
    #[serde(default)]
    pub schedule_id: Option<String>,
    #[serde(default)]
    pub conflict_date: Vec<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMeetingRoomBookingByIdResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub meetingroom_id: Option<i64>,
    #[serde(default)]
    pub schedule: Option<WorkMeetingRoomBookingByIdSchedule>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMeetingRoomBookingByIdSchedule {
    #[serde(default)]
    pub booking_id: Option<String>,
    #[serde(default)]
    pub schedule_id: Option<String>,
    #[serde(default)]
    pub start_time: Option<i64>,
    #[serde(default)]
    pub end_time: Option<i64>,
    #[serde(default)]
    pub booker: Option<String>,
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocCreateDocumentRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spaceid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fatherid: Option<String>,
    pub doc_type: i64,
    pub doc_name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub admin_users: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocCreateDocumentResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub docid: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocRenameDocumentRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formid: Option<String>,
    pub new_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocDocumentTargetRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocDocumentBaseInfo {
    #[serde(default)]
    pub docid: Option<String>,
    #[serde(default)]
    pub doc_name: Option<String>,
    #[serde(default)]
    pub create_time: Option<i64>,
    #[serde(default)]
    pub modify_time: Option<i64>,
    #[serde(default)]
    pub doc_type: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocDocumentBaseInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub doc_base_info: Option<WorkWeDocDocumentBaseInfo>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocShareDocumentResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub share_url: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocDocumentAuthResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub access_rule: Option<WorkWeDocAccessRule>,
    #[serde(default)]
    pub secure_setting: Option<WorkWeDocSecureSetting>,
    #[serde(default)]
    pub doc_member_list: Vec<WorkWeDocDocumentMember>,
    #[serde(default)]
    pub co_auth_list: Vec<WorkWeDocDepartmentAuth>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocAccessRule {
    #[serde(default)]
    pub enable_corp_internal: Option<bool>,
    #[serde(default)]
    pub corp_internal_auth: Option<i64>,
    #[serde(default)]
    pub enable_corp_external: Option<bool>,
    #[serde(default)]
    pub corp_external_auth: Option<i64>,
    #[serde(default)]
    pub corp_internal_approve_only_by_admin: Option<bool>,
    #[serde(default)]
    pub corp_external_approve_only_by_admin: Option<bool>,
    #[serde(default)]
    pub ban_share_external: Option<bool>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSecureSetting {
    #[serde(default)]
    pub enable_readonly_copy: Option<bool>,
    #[serde(default)]
    pub enable_readonly_comment: Option<bool>,
    #[serde(default)]
    pub watermark: Option<WorkWeDocWatermark>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocWatermark {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_type: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_visitor_name: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_text: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocDocumentMember {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub member_type: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmp_external_userid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocDepartmentAuth {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub member_type: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departmentid: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocModifyJoinRuleRequest {
    pub docid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_corp_internal: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corp_internal_auth: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_corp_external: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corp_external_auth: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corp_internal_approve_only_by_admin: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corp_external_approve_only_by_admin: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ban_share_external: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_co_auth_list: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub co_auth_list: Vec<WorkWeDocDepartmentAuth>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocModifyMembersRequest {
    pub docid: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub update_file_member_list: Vec<WorkWeDocDocumentMember>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub del_file_member_list: Vec<WorkWeDocDocumentMember>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocModifySafetySettingRequest {
    pub docid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_readonly_copy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watermark: Option<WorkWeDocWatermark>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocVipBatchRequest {
    pub userid_list: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocVipBatchResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub succ_userid_list: Vec<String>,
    #[serde(default)]
    pub fail_userid_list: Vec<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocVipListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocVipListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub has_more: Option<bool>,
    #[serde(default)]
    pub next_cursor: Option<String>,
    #[serde(default)]
    pub userid_list: Vec<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocDocumentDataResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub version: Option<i64>,
    #[serde(default)]
    pub document: Option<Value>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocGetContentDataRequest {
    pub docid: String,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocContentDataResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub docid: Option<String>,
    #[serde(default)]
    pub content: Option<Value>,
    #[serde(default)]
    pub doc_content: Option<Value>,
    #[serde(default)]
    pub has_more: Option<bool>,
    #[serde(default)]
    pub next_cursor: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl WorkWeDocContentDataResponse {
    pub fn effective_content(&self) -> Option<&Value> {
        self.content.as_ref().or(self.doc_content.as_ref())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocModifyContentRequest {
    pub docid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests: Option<Value>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocImageUploadResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub image_url: Option<String>,
    #[serde(default)]
    pub fileid: Option<String>,
    #[serde(default)]
    pub imageid: Option<String>,
    #[serde(default)]
    pub media_id: Option<String>,
    #[serde(default)]
    pub md5: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl WorkWeDocImageUploadResponse {
    pub fn effective_url(&self) -> Option<&str> {
        self.image_url.as_deref().or(self.url.as_deref())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocAdminRequest {
    pub docid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_userid: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub account_type: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocAdminListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub docid: Option<String>,
    #[serde(default)]
    pub admin_list: Vec<WorkWeDocAdmin>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocAdmin {
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub open_userid: Option<String>,
    #[serde(default, rename = "type")]
    pub account_type: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocDocumentBatchUpdateRequest {
    pub docid: String,
    pub version: i64,
    pub requests: Vec<WorkWeDocDocumentUpdateRequest>,
}

impl WorkWeDocDocumentBatchUpdateRequest {
    pub fn new(
        docid: impl Into<String>,
        version: i64,
        requests: impl IntoIterator<Item = WorkWeDocDocumentUpdateRequest>,
    ) -> Self {
        Self {
            docid: docid.into(),
            version,
            requests: requests.into_iter().collect(),
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.docid.trim().is_empty() {
            return Err(WechatError::Config(
                "WeDoc document batch update docid cannot be empty".to_string(),
            ));
        }
        if self.version < 0 {
            return Err(WechatError::Config(
                "WeDoc document batch update version cannot be negative".to_string(),
            ));
        }
        if self.requests.is_empty() {
            return Err(WechatError::Config(
                "WeDoc document batch update requires at least one operation".to_string(),
            ));
        }
        if self.requests.len() > 30 {
            return Err(WechatError::Config(
                "WeDoc document batch update supports at most 30 operations".to_string(),
            ));
        }
        for request in &self.requests {
            request.validate()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkWeDocDocumentUpdateRequest {
    #[serde(
        default,
        alias = "replace_text_request",
        skip_serializing_if = "Option::is_none"
    )]
    pub replace_text: Option<WorkWeDocDocumentReplaceText>,
    #[serde(
        default,
        alias = "insert_text_request",
        skip_serializing_if = "Option::is_none"
    )]
    pub insert_text: Option<WorkWeDocDocumentInsertText>,
    #[serde(
        default,
        alias = "delete_content_request",
        skip_serializing_if = "Option::is_none"
    )]
    pub delete_content: Option<WorkWeDocDocumentDeleteContent>,
    #[serde(
        default,
        alias = "insert_image_request",
        skip_serializing_if = "Option::is_none"
    )]
    pub insert_image: Option<WorkWeDocDocumentInsertImage>,
    #[serde(
        default,
        alias = "insert_page_break_request",
        skip_serializing_if = "Option::is_none"
    )]
    pub insert_page_break: Option<WorkWeDocDocumentInsertLocation>,
    #[serde(
        default,
        alias = "insert_table_request",
        skip_serializing_if = "Option::is_none"
    )]
    pub insert_table: Option<WorkWeDocDocumentInsertTable>,
    #[serde(
        default,
        alias = "insert_paragraph_request",
        skip_serializing_if = "Option::is_none"
    )]
    pub insert_paragraph: Option<WorkWeDocDocumentInsertLocation>,
    #[serde(
        default,
        alias = "update_text_property_request",
        skip_serializing_if = "Option::is_none"
    )]
    pub update_text_property: Option<WorkWeDocDocumentUpdateTextProperty>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl WorkWeDocDocumentUpdateRequest {
    pub fn replace_text(
        text: impl Into<String>,
        ranges: impl IntoIterator<Item = WorkWeDocDocumentRange>,
    ) -> Self {
        Self {
            replace_text: Some(WorkWeDocDocumentReplaceText {
                text: text.into(),
                ranges: ranges.into_iter().collect(),
            }),
            ..Self::default()
        }
    }

    pub fn insert_text(index: i64, text: impl Into<String>) -> Self {
        Self {
            insert_text: Some(WorkWeDocDocumentInsertText {
                text: text.into(),
                location: WorkWeDocDocumentLocation { index },
            }),
            ..Self::default()
        }
    }

    pub fn delete_content(start_index: i64, length: i64) -> Self {
        Self {
            delete_content: Some(WorkWeDocDocumentDeleteContent {
                range: WorkWeDocDocumentRange::new(start_index, length),
            }),
            ..Self::default()
        }
    }

    pub fn insert_image(index: i64, image_id: impl Into<String>) -> Self {
        Self {
            insert_image: Some(WorkWeDocDocumentInsertImage {
                image_id: image_id.into(),
                location: WorkWeDocDocumentLocation { index },
                width: None,
                height: None,
            }),
            ..Self::default()
        }
    }

    pub fn insert_sized_image(
        index: i64,
        image_id: impl Into<String>,
        width: i64,
        height: i64,
    ) -> Self {
        Self {
            insert_image: Some(WorkWeDocDocumentInsertImage {
                image_id: image_id.into(),
                location: WorkWeDocDocumentLocation { index },
                width: Some(width),
                height: Some(height),
            }),
            ..Self::default()
        }
    }

    pub fn insert_page_break(index: i64) -> Self {
        Self {
            insert_page_break: Some(WorkWeDocDocumentInsertLocation {
                location: WorkWeDocDocumentLocation { index },
            }),
            ..Self::default()
        }
    }

    pub fn insert_table(index: i64, rows: i64, cols: i64) -> Self {
        Self {
            insert_table: Some(WorkWeDocDocumentInsertTable {
                rows,
                cols,
                location: WorkWeDocDocumentLocation { index },
            }),
            ..Self::default()
        }
    }

    pub fn insert_paragraph(index: i64) -> Self {
        Self {
            insert_paragraph: Some(WorkWeDocDocumentInsertLocation {
                location: WorkWeDocDocumentLocation { index },
            }),
            ..Self::default()
        }
    }

    pub fn update_text_property(
        text_property: WorkWeDocDocumentTextProperty,
        ranges: impl IntoIterator<Item = WorkWeDocDocumentRange>,
    ) -> Self {
        Self {
            update_text_property: Some(WorkWeDocDocumentUpdateTextProperty {
                text_property,
                ranges: ranges.into_iter().collect(),
            }),
            ..Self::default()
        }
    }

    pub fn operation_kind(&self) -> Option<WorkWeDocDocumentOperationKind> {
        let mut kinds = Vec::new();
        if self.replace_text.is_some() {
            kinds.push(WorkWeDocDocumentOperationKind::ReplaceText);
        }
        if self.insert_text.is_some() {
            kinds.push(WorkWeDocDocumentOperationKind::InsertText);
        }
        if self.delete_content.is_some() {
            kinds.push(WorkWeDocDocumentOperationKind::DeleteContent);
        }
        if self.insert_image.is_some() {
            kinds.push(WorkWeDocDocumentOperationKind::InsertImage);
        }
        if self.insert_page_break.is_some() {
            kinds.push(WorkWeDocDocumentOperationKind::InsertPageBreak);
        }
        if self.insert_table.is_some() {
            kinds.push(WorkWeDocDocumentOperationKind::InsertTable);
        }
        if self.insert_paragraph.is_some() {
            kinds.push(WorkWeDocDocumentOperationKind::InsertParagraph);
        }
        if self.update_text_property.is_some() {
            kinds.push(WorkWeDocDocumentOperationKind::UpdateTextProperty);
        }
        let unknown_count = self.extra.as_object().map_or(0, serde_json::Map::len);
        match (kinds.as_slice(), unknown_count) {
            ([kind], 0) => Some(*kind),
            ([], 1) => Some(WorkWeDocDocumentOperationKind::Other),
            _ => None,
        }
    }

    pub fn validate(&self) -> Result<()> {
        let kind = self.operation_kind().ok_or_else(|| {
            WechatError::Config(
                "each WeDoc document update must contain exactly one operation".to_string(),
            )
        })?;
        match kind {
            WorkWeDocDocumentOperationKind::ReplaceText => {
                validate_wedoc_document_ranges(&self.replace_text.as_ref().unwrap().ranges)?;
            }
            WorkWeDocDocumentOperationKind::InsertText => {
                validate_wedoc_document_location(
                    self.insert_text.as_ref().unwrap().location.index,
                )?;
            }
            WorkWeDocDocumentOperationKind::DeleteContent => {
                self.delete_content.as_ref().unwrap().range.validate()?;
            }
            WorkWeDocDocumentOperationKind::InsertImage => {
                let image = self.insert_image.as_ref().unwrap();
                validate_wedoc_document_location(image.location.index)?;
                if image.image_id.trim().is_empty() {
                    return Err(WechatError::Config(
                        "WeDoc inserted image id cannot be empty".to_string(),
                    ));
                }
                if image.width.is_some_and(|width| width <= 0)
                    || image.height.is_some_and(|height| height <= 0)
                {
                    return Err(WechatError::Config(
                        "WeDoc inserted image dimensions must be positive".to_string(),
                    ));
                }
                if image.width.is_some() != image.height.is_some() {
                    return Err(WechatError::Config(
                        "WeDoc inserted image width and height must be provided together"
                            .to_string(),
                    ));
                }
            }
            WorkWeDocDocumentOperationKind::InsertPageBreak => {
                validate_wedoc_document_location(
                    self.insert_page_break.as_ref().unwrap().location.index,
                )?;
            }
            WorkWeDocDocumentOperationKind::InsertTable => {
                let table = self.insert_table.as_ref().unwrap();
                validate_wedoc_document_location(table.location.index)?;
                if !(1..=100).contains(&table.rows)
                    || !(1..=60).contains(&table.cols)
                    || table.rows * table.cols > 1_000
                {
                    return Err(WechatError::Config(
                        "WeDoc inserted table must be within 100 rows, 60 columns, and 1000 cells"
                            .to_string(),
                    ));
                }
            }
            WorkWeDocDocumentOperationKind::InsertParagraph => {
                validate_wedoc_document_location(
                    self.insert_paragraph.as_ref().unwrap().location.index,
                )?;
            }
            WorkWeDocDocumentOperationKind::UpdateTextProperty => {
                validate_wedoc_document_ranges(
                    &self.update_text_property.as_ref().unwrap().ranges,
                )?;
            }
            WorkWeDocDocumentOperationKind::Other => {}
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkWeDocDocumentOperationKind {
    ReplaceText,
    InsertText,
    DeleteContent,
    InsertImage,
    InsertPageBreak,
    InsertTable,
    InsertParagraph,
    UpdateTextProperty,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocDocumentLocation {
    pub index: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocDocumentRange {
    pub start_index: i64,
    pub length: i64,
}

impl WorkWeDocDocumentRange {
    pub fn new(start_index: i64, length: i64) -> Self {
        Self {
            start_index,
            length,
        }
    }

    pub fn end_index(&self) -> Option<i64> {
        self.start_index.checked_add(self.length)
    }

    pub fn validate(&self) -> Result<()> {
        if self.start_index < 0 || self.length <= 0 || self.end_index().is_none() {
            return Err(WechatError::Config(
                "WeDoc document range requires a non-negative start and positive valid length"
                    .to_string(),
            ));
        }
        Ok(())
    }
}

fn validate_wedoc_document_location(index: i64) -> Result<()> {
    if index < 0 {
        return Err(WechatError::Config(
            "WeDoc document location index cannot be negative".to_string(),
        ));
    }
    Ok(())
}

fn validate_wedoc_document_ranges(ranges: &[WorkWeDocDocumentRange]) -> Result<()> {
    if ranges.is_empty() {
        return Err(WechatError::Config(
            "WeDoc document text operation requires at least one range".to_string(),
        ));
    }
    if ranges.len() > 50 {
        return Err(WechatError::Config(
            "WeDoc document text operation supports at most 50 ranges".to_string(),
        ));
    }
    for range in ranges {
        range.validate()?;
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocDocumentReplaceText {
    pub text: String,
    pub ranges: Vec<WorkWeDocDocumentRange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocDocumentInsertText {
    pub text: String,
    pub location: WorkWeDocDocumentLocation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocDocumentDeleteContent {
    pub range: WorkWeDocDocumentRange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocDocumentInsertImage {
    pub image_id: String,
    pub location: WorkWeDocDocumentLocation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocDocumentInsertLocation {
    pub location: WorkWeDocDocumentLocation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocDocumentInsertTable {
    pub rows: i64,
    pub cols: i64,
    pub location: WorkWeDocDocumentLocation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocDocumentUpdateTextProperty {
    pub text_property: WorkWeDocDocumentTextProperty,
    pub ranges: Vec<WorkWeDocDocumentRange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocDocumentTextProperty {
    #[serde(alias = "blod", skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSpreadsheetPropertiesResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub properties: Vec<WorkWeDocSpreadsheetProperties>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSpreadsheetProperties {
    #[serde(default)]
    pub sheet_id: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub row_count: Option<i64>,
    #[serde(default)]
    pub column_count: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSpreadsheetRangeRequest {
    pub docid: String,
    pub sheet_id: String,
    pub range: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSpreadsheetRangeResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub data: Option<WorkWeDocSpreadsheetRangeData>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSpreadsheetRangeData {
    #[serde(default)]
    pub result: Option<WorkWeDocSpreadsheetGridData>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSpreadsheetGridData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_row: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_column: Option<i64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rows: Vec<WorkWeDocSpreadsheetRow>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSpreadsheetRow {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<WorkWeDocSpreadsheetCell>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSpreadsheetCell {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cell_value: Option<WorkWeDocSpreadsheetCellValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cell_format: Option<WorkWeDocSpreadsheetCellFormat>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSpreadsheetCellValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<WorkWeDocSpreadsheetLink>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSpreadsheetLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSpreadsheetCellFormat {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_format: Option<WorkWeDocSpreadsheetTextFormat>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSpreadsheetTextFormat {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strikethrough: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underline: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<WorkWeDocSpreadsheetColor>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSpreadsheetColor {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub red: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub green: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpha: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSpreadsheetBatchUpdateRequest {
    pub docid: String,
    pub requests: Vec<WorkWeDocSpreadsheetUpdateRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSpreadsheetUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_sheet_request: Option<WorkWeDocSpreadsheetAddSheetRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_sheet_request: Option<WorkWeDocSpreadsheetDeleteSheetRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_range_request: Option<WorkWeDocSpreadsheetUpdateRangeRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_dimension_request: Option<WorkWeDocSpreadsheetDeleteDimensionRequest>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSpreadsheetAddSheetRequest {
    pub title: String,
    pub row_count: i64,
    pub column_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSpreadsheetDeleteSheetRequest {
    pub sheet_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSpreadsheetUpdateRangeRequest {
    pub sheet_id: String,
    pub grid_data: WorkWeDocSpreadsheetGridData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSpreadsheetDeleteDimensionRequest {
    pub sheet_id: String,
    pub dimension: String,
    pub start_index: i64,
    pub end_index: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSpreadsheetBatchUpdateResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub data: Option<WorkWeDocSpreadsheetBatchUpdateData>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSpreadsheetBatchUpdateData {
    #[serde(default)]
    pub responses: Vec<WorkWeDocSpreadsheetUpdateResponse>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSpreadsheetUpdateResponse {
    #[serde(default)]
    pub add_sheet_response: Option<WorkWeDocSpreadsheetAddSheetResponse>,
    #[serde(default)]
    pub delete_sheet_response: Option<WorkWeDocSpreadsheetDeleteSheetResponse>,
    #[serde(default)]
    pub update_range_response: Option<WorkWeDocSpreadsheetUpdateRangeResponse>,
    #[serde(default)]
    pub delete_dimension_response: Option<WorkWeDocSpreadsheetDeleteDimensionResponse>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSpreadsheetAddSheetResponse {
    #[serde(default)]
    pub properties: Option<WorkWeDocSpreadsheetProperties>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSpreadsheetDeleteSheetResponse {
    #[serde(default)]
    pub sheet_id: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSpreadsheetUpdateRangeResponse {
    #[serde(default)]
    pub updated_cells: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSpreadsheetDeleteDimensionResponse {
    #[serde(default)]
    pub deleted: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetAddRequest {
    pub docid: String,
    pub properties: WorkWeDocSmartSheetProperties,
}

impl WorkWeDocSmartSheetAddRequest {
    pub fn titled(docid: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            docid: docid.into(),
            properties: WorkWeDocSmartSheetProperties::titled(title),
        }
    }

    pub fn at_index(docid: impl Into<String>, index: i64) -> Self {
        Self {
            docid: docid.into(),
            properties: WorkWeDocSmartSheetProperties::at_index(index),
        }
    }

    pub fn titled_at(docid: impl Into<String>, title: impl Into<String>, index: i64) -> Self {
        Self {
            docid: docid.into(),
            properties: WorkWeDocSmartSheetProperties::titled(title).with_index(index),
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.docid.trim().is_empty() {
            return Err(WechatError::Config(
                "WeDoc smart-sheet document id cannot be empty".to_string(),
            ));
        }
        self.properties.validate_for_add()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetAddResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub properties: Option<WorkWeDocSmartSheetProperties>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetGetRequest {
    pub docid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_all_type_sheet: Option<bool>,
}

impl WorkWeDocSmartSheetGetRequest {
    pub fn validate(&self) -> Result<()> {
        if self.docid.trim().is_empty() {
            return Err(WechatError::Config(
                "WeDoc smart-sheet query document id cannot be empty".to_string(),
            ));
        }
        if self
            .sheet_id
            .as_deref()
            .is_some_and(|sheet_id| sheet_id.trim().is_empty())
        {
            return Err(WechatError::Config(
                "WeDoc smart-sheet query sheet id cannot be empty".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetGetResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub sheet_list: Vec<WorkWeDocSmartSheetProperties>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sheet_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
    #[serde(
        default,
        rename = "type",
        alias = "sheet_type",
        skip_serializing_if = "Option::is_none"
    )]
    pub sheet_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_visible: Option<bool>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl Default for WorkWeDocSmartSheetProperties {
    fn default() -> Self {
        Self {
            sheet_id: None,
            title: None,
            index: None,
            sheet_type: None,
            is_visible: None,
            extra: Value::Null,
        }
    }
}

impl WorkWeDocSmartSheetProperties {
    pub fn titled(title: impl Into<String>) -> Self {
        Self {
            title: Some(title.into()),
            ..Self::default()
        }
    }

    pub fn at_index(index: i64) -> Self {
        Self {
            index: Some(index),
            ..Self::default()
        }
    }

    pub fn for_update(sheet_id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            sheet_id: Some(sheet_id.into()),
            title: Some(title.into()),
            ..Self::default()
        }
    }

    pub fn with_index(mut self, index: i64) -> Self {
        self.index = Some(index);
        self
    }

    pub fn sheet_type_kind(&self) -> Option<WorkWeDocSmartSheetTypeKind> {
        self.sheet_type
            .as_deref()
            .map(WorkWeDocSmartSheetTypeKind::from_code)
    }

    fn validate_for_add(&self) -> Result<()> {
        if self
            .title
            .as_deref()
            .is_some_and(|title| title.trim().is_empty())
        {
            return Err(WechatError::Config(
                "added WeDoc smart-sheet title cannot be empty".to_string(),
            ));
        }
        if self.index.is_some_and(|index| index < 0) {
            return Err(WechatError::Config(
                "added WeDoc smart-sheet index cannot be negative".to_string(),
            ));
        }
        Ok(())
    }

    fn validate_for_update(&self) -> Result<()> {
        if self
            .sheet_id
            .as_deref()
            .is_none_or(|sheet_id| sheet_id.trim().is_empty())
        {
            return Err(WechatError::Config(
                "updated WeDoc smart-sheet id cannot be empty".to_string(),
            ));
        }
        if self
            .title
            .as_deref()
            .is_none_or(|title| title.trim().is_empty())
        {
            return Err(WechatError::Config(
                "updated WeDoc smart-sheet title cannot be empty".to_string(),
            ));
        }
        if self.index.is_some()
            || self.sheet_type.is_some()
            || self.is_visible.is_some()
            || self
                .extra
                .as_object()
                .is_some_and(|extra| !extra.is_empty())
        {
            return Err(WechatError::Config(
                "WeDoc smart-sheet update only supports sheet id and title".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkWeDocSmartSheetTypeKind {
    SmartSheet,
    Dashboard,
    External,
    Other,
}

impl WorkWeDocSmartSheetTypeKind {
    pub fn from_code(value: &str) -> Self {
        if value.eq_ignore_ascii_case("smartsheet")
            || value.eq_ignore_ascii_case("smart_sheet")
            || value.eq_ignore_ascii_case("SMART_SHEET")
        {
            Self::SmartSheet
        } else if value.eq_ignore_ascii_case("dashboard") {
            Self::Dashboard
        } else if value.eq_ignore_ascii_case("external") {
            Self::External
        } else {
            Self::Other
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetUpdateRequest {
    pub docid: String,
    pub properties: WorkWeDocSmartSheetProperties,
}

impl WorkWeDocSmartSheetUpdateRequest {
    pub fn rename(
        docid: impl Into<String>,
        sheet_id: impl Into<String>,
        title: impl Into<String>,
    ) -> Self {
        Self {
            docid: docid.into(),
            properties: WorkWeDocSmartSheetProperties::for_update(sheet_id, title),
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.docid.trim().is_empty() {
            return Err(WechatError::Config(
                "updated WeDoc smart-sheet document id cannot be empty".to_string(),
            ));
        }
        self.properties.validate_for_update()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetDeleteRequest {
    pub docid: String,
    pub sheet_id: String,
}

impl WorkWeDocSmartSheetDeleteRequest {
    pub fn validate(&self) -> Result<()> {
        if self.docid.trim().is_empty() || self.sheet_id.trim().is_empty() {
            return Err(WechatError::Config(
                "deleted WeDoc smart-sheet document and sheet ids cannot be empty".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetAddViewRequest {
    pub docid: String,
    pub sheet_id: String,
    pub view_title: String,
    pub view_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_gantt: Option<WorkWeDocSmartSheetViewDateRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_calendar: Option<WorkWeDocSmartSheetViewDateRange>,
}

impl WorkWeDocSmartSheetAddViewRequest {
    pub fn grid(
        docid: impl Into<String>,
        sheet_id: impl Into<String>,
        view_title: impl Into<String>,
    ) -> Self {
        Self::new(
            docid,
            sheet_id,
            view_title,
            WorkWeDocSmartSheetViewTypeKind::Grid,
        )
    }

    pub fn kanban(
        docid: impl Into<String>,
        sheet_id: impl Into<String>,
        view_title: impl Into<String>,
    ) -> Self {
        Self::new(
            docid,
            sheet_id,
            view_title,
            WorkWeDocSmartSheetViewTypeKind::Kanban,
        )
    }

    pub fn gallery(
        docid: impl Into<String>,
        sheet_id: impl Into<String>,
        view_title: impl Into<String>,
    ) -> Self {
        Self::new(
            docid,
            sheet_id,
            view_title,
            WorkWeDocSmartSheetViewTypeKind::Gallery,
        )
    }

    pub fn gantt(
        docid: impl Into<String>,
        sheet_id: impl Into<String>,
        view_title: impl Into<String>,
        date_range: WorkWeDocSmartSheetViewDateRange,
    ) -> Self {
        Self {
            property_gantt: Some(date_range),
            ..Self::new(
                docid,
                sheet_id,
                view_title,
                WorkWeDocSmartSheetViewTypeKind::Gantt,
            )
        }
    }

    pub fn calendar(
        docid: impl Into<String>,
        sheet_id: impl Into<String>,
        view_title: impl Into<String>,
        date_range: WorkWeDocSmartSheetViewDateRange,
    ) -> Self {
        Self {
            property_calendar: Some(date_range),
            ..Self::new(
                docid,
                sheet_id,
                view_title,
                WorkWeDocSmartSheetViewTypeKind::Calendar,
            )
        }
    }

    fn new(
        docid: impl Into<String>,
        sheet_id: impl Into<String>,
        view_title: impl Into<String>,
        view_type: WorkWeDocSmartSheetViewTypeKind,
    ) -> Self {
        Self {
            docid: docid.into(),
            sheet_id: sheet_id.into(),
            view_title: view_title.into(),
            view_type: view_type.as_code().unwrap_or_default().to_string(),
            property_gantt: None,
            property_calendar: None,
        }
    }

    pub fn view_type_kind(&self) -> WorkWeDocSmartSheetViewTypeKind {
        WorkWeDocSmartSheetViewTypeKind::from_code(&self.view_type)
    }

    pub fn validate(&self) -> Result<()> {
        validate_wedoc_smartsheet_view_scope(&self.docid, &self.sheet_id)?;
        if self.view_title.trim().is_empty() {
            return Err(WechatError::Config(
                "WeDoc smart-sheet view title cannot be empty".to_string(),
            ));
        }
        match self.view_type_kind() {
            WorkWeDocSmartSheetViewTypeKind::Gantt => {
                self.property_gantt.as_ref().ok_or_else(|| {
                    WechatError::Config(
                        "WeDoc smart-sheet Gantt view requires a date range".to_string(),
                    )
                })?;
                if self.property_calendar.is_some() {
                    return Err(WechatError::Config(
                        "WeDoc smart-sheet Gantt view cannot include calendar properties"
                            .to_string(),
                    ));
                }
            }
            WorkWeDocSmartSheetViewTypeKind::Calendar => {
                self.property_calendar.as_ref().ok_or_else(|| {
                    WechatError::Config(
                        "WeDoc smart-sheet calendar view requires a date range".to_string(),
                    )
                })?;
                if self.property_gantt.is_some() {
                    return Err(WechatError::Config(
                        "WeDoc smart-sheet calendar view cannot include Gantt properties"
                            .to_string(),
                    ));
                }
            }
            WorkWeDocSmartSheetViewTypeKind::Grid
            | WorkWeDocSmartSheetViewTypeKind::Kanban
            | WorkWeDocSmartSheetViewTypeKind::Gallery => {
                if self.property_gantt.is_some() || self.property_calendar.is_some() {
                    return Err(WechatError::Config(
                        "WeDoc smart-sheet view type cannot include date-range properties"
                            .to_string(),
                    ));
                }
            }
            WorkWeDocSmartSheetViewTypeKind::Unknown | WorkWeDocSmartSheetViewTypeKind::Other => {
                return Err(WechatError::Config(
                    "WeDoc smart-sheet view type is invalid".to_string(),
                ));
            }
        }
        if let Some(range) = self
            .property_gantt
            .as_ref()
            .or(self.property_calendar.as_ref())
        {
            range.validate()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetUpdateViewRequest {
    pub docid: String,
    pub sheet_id: String,
    pub view_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<WorkWeDocSmartSheetViewProperty>,
}

impl WorkWeDocSmartSheetUpdateViewRequest {
    pub fn new(
        docid: impl Into<String>,
        sheet_id: impl Into<String>,
        view_id: impl Into<String>,
    ) -> Self {
        Self {
            docid: docid.into(),
            sheet_id: sheet_id.into(),
            view_id: view_id.into(),
            view_title: None,
            property: None,
        }
    }

    pub fn with_title(mut self, view_title: impl Into<String>) -> Self {
        self.view_title = Some(view_title.into());
        self
    }

    pub fn with_property(mut self, property: WorkWeDocSmartSheetViewProperty) -> Self {
        self.property = Some(property);
        self
    }

    pub fn validate(&self) -> Result<()> {
        validate_wedoc_smartsheet_view_scope(&self.docid, &self.sheet_id)?;
        if self.view_id.trim().is_empty() {
            return Err(WechatError::Config(
                "updated WeDoc smart-sheet view id cannot be empty".to_string(),
            ));
        }
        if self.view_title.is_none() && self.property.is_none() {
            return Err(WechatError::Config(
                "updated WeDoc smart-sheet view requires at least one change".to_string(),
            ));
        }
        if self
            .view_title
            .as_deref()
            .is_some_and(|title| title.trim().is_empty())
        {
            return Err(WechatError::Config(
                "updated WeDoc smart-sheet view title cannot be empty".to_string(),
            ));
        }
        if let Some(property) = &self.property {
            property.validate()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkWeDocSmartSheetViewTypeKind {
    Unknown,
    Grid,
    Kanban,
    Gallery,
    Gantt,
    Calendar,
    Other,
}

impl WorkWeDocSmartSheetViewTypeKind {
    pub const fn as_code(self) -> Option<&'static str> {
        Some(match self {
            Self::Unknown => "VEW_UNKNOWN",
            Self::Grid => "VIEW_TYPE_GRID",
            Self::Kanban => "VIEW_TYPE_KANBAN",
            Self::Gallery => "VIEW_TYPE_GALLERY",
            Self::Gantt => "VIEW_TYPE_GANTT",
            Self::Calendar => "VIEW_TYPE_CALENDAR",
            Self::Other => return None,
        })
    }

    pub fn from_code(value: &str) -> Self {
        match value {
            "VEW_UNKNOWN" | "VIEW_UNKNOWN" => Self::Unknown,
            "VIEW_TYPE_GRID" => Self::Grid,
            "VIEW_TYPE_KANBAN" => Self::Kanban,
            "VIEW_TYPE_GALLERY" => Self::Gallery,
            "VIEW_TYPE_GANTT" => Self::Gantt,
            "VIEW_TYPE_CALENDAR" => Self::Calendar,
            _ => Self::Other,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetViewDateRange {
    pub start_date_field_id: String,
    pub end_date_field_id: String,
}

impl WorkWeDocSmartSheetViewDateRange {
    pub fn new(
        start_date_field_id: impl Into<String>,
        end_date_field_id: impl Into<String>,
    ) -> Self {
        Self {
            start_date_field_id: start_date_field_id.into(),
            end_date_field_id: end_date_field_id.into(),
        }
    }

    fn validate(&self) -> Result<()> {
        if self.start_date_field_id.trim().is_empty() || self.end_date_field_id.trim().is_empty() {
            return Err(WechatError::Config(
                "WeDoc smart-sheet view date field ids cannot be empty".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkWeDocSmartSheetViewProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_sort: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_spec: Option<WorkWeDocSmartSheetViewSortSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_spec: Option<WorkWeDocSmartSheetViewGroupSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_spec: Option<WorkWeDocSmartSheetRecordFilter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_field_stat_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub field_visibility: std::collections::BTreeMap<String, bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frozen_field_count: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl WorkWeDocSmartSheetViewProperty {
    pub fn with_sort(
        mut self,
        sort_infos: impl IntoIterator<Item = WorkWeDocSmartSheetRecordSort>,
    ) -> Self {
        self.sort_spec = Some(WorkWeDocSmartSheetViewSortSpec {
            sort_infos: sort_infos.into_iter().collect(),
        });
        self
    }

    pub fn with_groups(
        mut self,
        groups: impl IntoIterator<Item = WorkWeDocSmartSheetRecordSort>,
    ) -> Self {
        self.group_spec = Some(WorkWeDocSmartSheetViewGroupSpec {
            sort_infos: groups.into_iter().collect(),
        });
        self
    }

    pub fn with_filter(mut self, filter: WorkWeDocSmartSheetRecordFilter) -> Self {
        self.filter_spec = Some(filter);
        self
    }

    pub fn set_field_visible(mut self, field_id: impl Into<String>, visible: bool) -> Self {
        self.field_visibility.insert(field_id.into(), visible);
        self
    }

    fn validate(&self) -> Result<()> {
        if self
            .frozen_field_count
            .is_some_and(|frozen_field_count| frozen_field_count < 0)
        {
            return Err(WechatError::Config(
                "WeDoc smart-sheet frozen field count cannot be negative".to_string(),
            ));
        }
        if self
            .field_visibility
            .iter()
            .any(|(field_id, _)| field_id.trim().is_empty())
        {
            return Err(WechatError::Config(
                "WeDoc smart-sheet field visibility requires field ids".to_string(),
            ));
        }
        if let Some(sort_spec) = &self.sort_spec {
            sort_spec.validate()?;
        }
        if let Some(group_spec) = &self.group_spec {
            group_spec.validate()?;
        }
        if let Some(filter_spec) = &self.filter_spec {
            filter_spec.validate()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetViewSortSpec {
    pub sort_infos: Vec<WorkWeDocSmartSheetRecordSort>,
}

impl WorkWeDocSmartSheetViewSortSpec {
    fn validate(&self) -> Result<()> {
        for sort in &self.sort_infos {
            sort.validate()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetViewGroupSpec {
    pub sort_infos: Vec<WorkWeDocSmartSheetRecordSort>,
}

impl WorkWeDocSmartSheetViewGroupSpec {
    fn validate(&self) -> Result<()> {
        for group in &self.sort_infos {
            group.validate()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetViewResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub view: Option<WorkWeDocSmartSheetView>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetView {
    #[serde(default)]
    pub view_id: Option<String>,
    #[serde(default)]
    pub view_title: Option<String>,
    #[serde(default)]
    pub view_type: Option<String>,
    #[serde(default)]
    pub property: Option<WorkWeDocSmartSheetViewProperty>,
    #[serde(default)]
    pub property_gantt: Option<WorkWeDocSmartSheetViewDateRange>,
    #[serde(default)]
    pub property_calendar: Option<WorkWeDocSmartSheetViewDateRange>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl WorkWeDocSmartSheetView {
    pub fn view_type_kind(&self) -> Option<WorkWeDocSmartSheetViewTypeKind> {
        self.view_type
            .as_deref()
            .map(WorkWeDocSmartSheetViewTypeKind::from_code)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetGetViewsRequest {
    pub docid: String,
    pub sheet_id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub view_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl WorkWeDocSmartSheetGetViewsRequest {
    pub fn validate(&self) -> Result<()> {
        validate_wedoc_smartsheet_view_scope(&self.docid, &self.sheet_id)?;
        if self
            .view_ids
            .iter()
            .any(|view_id| view_id.trim().is_empty())
        {
            return Err(WechatError::Config(
                "WeDoc smart-sheet view id cannot be empty".to_string(),
            ));
        }
        if self.offset.is_some_and(|offset| offset < 0) {
            return Err(WechatError::Config(
                "WeDoc smart-sheet view query offset cannot be negative".to_string(),
            ));
        }
        if self
            .limit
            .is_some_and(|limit| !(1..=1_000).contains(&limit))
        {
            return Err(WechatError::Config(
                "WeDoc smart-sheet view query limit must be between 1 and 1000".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetGetViewsResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub total: Option<i64>,
    #[serde(default)]
    pub has_more: Option<bool>,
    #[serde(default)]
    pub next: Option<i64>,
    #[serde(default)]
    pub views: Vec<WorkWeDocSmartSheetView>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetDeleteViewsRequest {
    pub docid: String,
    pub sheet_id: String,
    pub view_ids: Vec<String>,
}

impl WorkWeDocSmartSheetDeleteViewsRequest {
    pub fn validate(&self) -> Result<()> {
        validate_wedoc_smartsheet_view_scope(&self.docid, &self.sheet_id)?;
        if self.view_ids.is_empty() || self.view_ids.iter().any(|id| id.trim().is_empty()) {
            return Err(WechatError::Config(
                "WeDoc smart-sheet view deletion requires non-empty view ids".to_string(),
            ));
        }
        Ok(())
    }
}

fn validate_wedoc_smartsheet_view_scope(docid: &str, sheet_id: &str) -> Result<()> {
    if docid.trim().is_empty() || sheet_id.trim().is_empty() {
        return Err(WechatError::Config(
            "WeDoc smart-sheet view docid and sheet id cannot be empty".to_string(),
        ));
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetFieldsMutationRequest {
    pub docid: String,
    pub sheet_id: String,
    pub fields: Vec<WorkWeDocSmartSheetFieldMutation>,
}

impl WorkWeDocSmartSheetFieldsMutationRequest {
    pub fn new(
        docid: impl Into<String>,
        sheet_id: impl Into<String>,
        fields: impl IntoIterator<Item = WorkWeDocSmartSheetFieldMutation>,
    ) -> Self {
        Self {
            docid: docid.into(),
            sheet_id: sheet_id.into(),
            fields: fields.into_iter().collect(),
        }
    }

    pub fn validate_for_add(&self) -> Result<()> {
        self.validate_common()?;
        for field in &self.fields {
            field.validate_for_add()?;
        }
        Ok(())
    }

    pub fn validate_for_update(&self) -> Result<()> {
        self.validate_common()?;
        for field in &self.fields {
            field.validate_for_update()?;
        }
        Ok(())
    }

    fn validate_common(&self) -> Result<()> {
        if self.docid.trim().is_empty() {
            return Err(WechatError::Config(
                "WeDoc smart-sheet field mutation docid cannot be empty".to_string(),
            ));
        }
        if self.sheet_id.trim().is_empty() {
            return Err(WechatError::Config(
                "WeDoc smart-sheet field mutation sheet id cannot be empty".to_string(),
            ));
        }
        if self.fields.is_empty() {
            return Err(WechatError::Config(
                "WeDoc smart-sheet field mutation requires at least one field".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkWeDocSmartSheetFieldMutation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub property: Option<WorkWeDocSmartSheetFieldProperty>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl WorkWeDocSmartSheetFieldMutation {
    pub fn add(
        field_title: impl Into<String>,
        field_type: WorkWeDocSmartSheetFieldTypeKind,
    ) -> Self {
        Self {
            field_title: Some(field_title.into()),
            field_type: field_type.as_code().map(str::to_string),
            ..Self::default()
        }
    }

    pub fn update(field_id: impl Into<String>) -> Self {
        Self {
            field_id: Some(field_id.into()),
            ..Self::default()
        }
    }

    pub fn with_title(mut self, field_title: impl Into<String>) -> Self {
        self.field_title = Some(field_title.into());
        self
    }

    pub fn with_type(mut self, field_type: WorkWeDocSmartSheetFieldTypeKind) -> Self {
        self.field_type = field_type.as_code().map(str::to_string);
        self
    }

    pub fn with_property(mut self, property: WorkWeDocSmartSheetFieldProperty) -> Self {
        self.property = Some(property);
        self
    }

    pub fn field_type_kind(&self) -> Option<WorkWeDocSmartSheetFieldTypeKind> {
        self.field_type
            .as_deref()
            .map(WorkWeDocSmartSheetFieldTypeKind::from_code)
    }

    fn validate_for_add(&self) -> Result<()> {
        if self
            .field_title
            .as_deref()
            .is_none_or(|title| title.trim().is_empty())
        {
            return Err(WechatError::Config(
                "added WeDoc smart-sheet field title cannot be empty".to_string(),
            ));
        }
        if self
            .field_type
            .as_deref()
            .is_none_or(|field_type| field_type.trim().is_empty())
        {
            return Err(WechatError::Config(
                "added WeDoc smart-sheet field type cannot be empty".to_string(),
            ));
        }
        self.validate_property()
    }

    fn validate_for_update(&self) -> Result<()> {
        if self
            .field_id
            .as_deref()
            .is_none_or(|field_id| field_id.trim().is_empty())
        {
            return Err(WechatError::Config(
                "updated WeDoc smart-sheet field id cannot be empty".to_string(),
            ));
        }
        if self.field_title.is_none()
            && self.field_type.is_none()
            && self.property.is_none()
            && self.extra.as_object().is_none_or(serde_json::Map::is_empty)
        {
            return Err(WechatError::Config(
                "updated WeDoc smart-sheet field requires at least one change".to_string(),
            ));
        }
        if self
            .field_title
            .as_deref()
            .is_some_and(|title| title.trim().is_empty())
        {
            return Err(WechatError::Config(
                "updated WeDoc smart-sheet field title cannot be empty".to_string(),
            ));
        }
        if self
            .field_type
            .as_deref()
            .is_some_and(|field_type| field_type.trim().is_empty())
        {
            return Err(WechatError::Config(
                "updated WeDoc smart-sheet field type cannot be empty".to_string(),
            ));
        }
        self.validate_property()
    }

    fn validate_property(&self) -> Result<()> {
        if let Some(property) = &self.property {
            property.validate()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkWeDocSmartSheetFieldTypeKind {
    Text,
    Number,
    Checkbox,
    DateTime,
    Image,
    Attachment,
    User,
    Url,
    Select,
    CreatedUser,
    ModifiedUser,
    CreatedTime,
    ModifiedTime,
    Progress,
    PhoneNumber,
    Email,
    SingleSelect,
    Reference,
    Location,
    Currency,
    WorkGroup,
    AutoNumber,
    Percentage,
    Formula,
    Other,
}

impl WorkWeDocSmartSheetFieldTypeKind {
    pub const fn as_code(self) -> Option<&'static str> {
        Some(match self {
            Self::Text => "FIELD_TYPE_TEXT",
            Self::Number => "FIELD_TYPE_NUMBER",
            Self::Checkbox => "FIELD_TYPE_CHECKBOX",
            Self::DateTime => "FIELD_TYPE_DATE_TIME",
            Self::Image => "FIELD_TYPE_IMAGE",
            Self::Attachment => "FIELD_TYPE_ATTACHMENT",
            Self::User => "FIELD_TYPE_USER",
            Self::Url => "FIELD_TYPE_URL",
            Self::Select => "FIELD_TYPE_SELECT",
            Self::CreatedUser => "FIELD_TYPE_CREATED_USER",
            Self::ModifiedUser => "FIELD_TYPE_MODIFIED_USER",
            Self::CreatedTime => "FIELD_TYPE_CREATED_TIME",
            Self::ModifiedTime => "FIELD_TYPE_MODIFIED_TIME",
            Self::Progress => "FIELD_TYPE_PROGRESS",
            Self::PhoneNumber => "FIELD_TYPE_PHONE_NUMBER",
            Self::Email => "FIELD_TYPE_EMAIL",
            Self::SingleSelect => "FIELD_TYPE_SINGLE_SELECT",
            Self::Reference => "FIELD_TYPE_REFERENCE",
            Self::Location => "FIELD_TYPE_LOCATION",
            Self::Currency => "FIELD_TYPE_CURRENCY",
            Self::WorkGroup => "FIELD_TYPE_WWGROUP",
            Self::AutoNumber => "FIELD_TYPE_AUTONUMBER",
            Self::Percentage => "FIELD_TYPE_PERCENTAGE",
            Self::Formula => "FIELD_TYPE_FORMULA",
            Self::Other => return None,
        })
    }

    pub fn from_code(value: &str) -> Self {
        match value {
            "FIELD_TYPE_TEXT" => Self::Text,
            "FIELD_TYPE_NUMBER" => Self::Number,
            "FIELD_TYPE_CHECKBOX" => Self::Checkbox,
            "FIELD_TYPE_DATE_TIME" => Self::DateTime,
            "FIELD_TYPE_IMAGE" => Self::Image,
            "FIELD_TYPE_ATTACHMENT" => Self::Attachment,
            "FIELD_TYPE_USER" => Self::User,
            "FIELD_TYPE_URL" => Self::Url,
            "FIELD_TYPE_SELECT" => Self::Select,
            "FIELD_TYPE_CREATED_USER" => Self::CreatedUser,
            "FIELD_TYPE_MODIFIED_USER" => Self::ModifiedUser,
            "FIELD_TYPE_CREATED_TIME" => Self::CreatedTime,
            "FIELD_TYPE_MODIFIED_TIME" => Self::ModifiedTime,
            "FIELD_TYPE_PROGRESS" => Self::Progress,
            "FIELD_TYPE_PHONE_NUMBER" => Self::PhoneNumber,
            "FIELD_TYPE_EMAIL" => Self::Email,
            "FIELD_TYPE_SINGLE_SELECT" => Self::SingleSelect,
            "FIELD_TYPE_REFERENCE" => Self::Reference,
            "FIELD_TYPE_LOCATION" => Self::Location,
            "FIELD_TYPE_CURRENCY" => Self::Currency,
            "FIELD_TYPE_WWGROUP" => Self::WorkGroup,
            "FIELD_TYPE_AUTONUMBER" => Self::AutoNumber,
            "FIELD_TYPE_PERCENTAGE" => Self::Percentage,
            "FIELD_TYPE_FORMULA" => Self::Formula,
            _ => Self::Other,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkWeDocSmartSheetFieldProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decimal_places: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_separate: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_quick_add: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<WorkWeDocSmartSheetSelectOption>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_fill: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_multiple: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_notified: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checked: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_mode: Option<String>,
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub property_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_multiple: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency_type: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<WorkWeDocSmartSheetAutoNumberRule>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reformat_existing_record: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub formula_model: Vec<WorkWeDocSmartSheetFormulaModel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_id: Option<String>,
    #[serde(default, alias = "filed_id", skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub view_id: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl WorkWeDocSmartSheetFieldProperty {
    pub fn number(decimal_places: i64, use_separate: bool) -> Self {
        Self {
            decimal_places: Some(decimal_places),
            use_separate: Some(use_separate),
            ..Self::default()
        }
    }

    pub fn select(
        is_quick_add: bool,
        options: impl IntoIterator<Item = WorkWeDocSmartSheetSelectOption>,
    ) -> Self {
        Self {
            is_quick_add: Some(is_quick_add),
            options: options.into_iter().collect(),
            ..Self::default()
        }
    }

    pub fn user(is_multiple: bool, is_notified: bool) -> Self {
        Self {
            is_multiple: Some(is_multiple),
            is_notified: Some(is_notified),
            ..Self::default()
        }
    }

    pub fn date_time(format: impl Into<String>, auto_fill: bool) -> Self {
        Self {
            format: Some(format.into()),
            auto_fill: Some(auto_fill),
            ..Self::default()
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self
            .decimal_places
            .is_some_and(|decimal_places| decimal_places < 0)
        {
            return Err(WechatError::Config(
                "WeDoc smart-sheet decimal places cannot be negative".to_string(),
            ));
        }
        for option in &self.options {
            option.validate()?;
        }
        for rule in &self.rules {
            if rule.rule_type.trim().is_empty() {
                return Err(WechatError::Config(
                    "WeDoc smart-sheet auto-number rule type cannot be empty".to_string(),
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkWeDocSmartSheetSelectOption {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl WorkWeDocSmartSheetSelectOption {
    pub fn by_id(id: impl Into<String>) -> Self {
        Self {
            id: Some(id.into()),
            ..Self::default()
        }
    }

    pub fn by_text(text: impl Into<String>) -> Self {
        Self {
            text: Some(text.into()),
            ..Self::default()
        }
    }

    fn validate(&self) -> Result<()> {
        let has_id = self.id.as_deref().is_some_and(|id| !id.trim().is_empty());
        let has_text = self
            .text
            .as_deref()
            .is_some_and(|text| !text.trim().is_empty());
        if !has_id && !has_text {
            return Err(WechatError::Config(
                "WeDoc smart-sheet select option requires an id or text".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetAutoNumberRule {
    #[serde(rename = "type")]
    pub rule_type: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetFormulaModel {
    #[serde(rename = "type")]
    pub formula_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetGetFieldsRequest {
    pub docid: String,
    pub sheet_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub field_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub field_titles: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetFieldsResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub fields: Vec<WorkWeDocSmartSheetField>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetGetFieldsResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub total: Option<i64>,
    #[serde(default)]
    pub fields: Vec<WorkWeDocSmartSheetField>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetField {
    #[serde(default)]
    pub field_id: Option<String>,
    #[serde(default)]
    pub field_title: Option<String>,
    #[serde(default)]
    pub field_type: Option<String>,
    #[serde(default)]
    pub property: Option<WorkWeDocSmartSheetFieldProperty>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl WorkWeDocSmartSheetField {
    pub fn field_type_kind(&self) -> Option<WorkWeDocSmartSheetFieldTypeKind> {
        self.field_type
            .as_deref()
            .map(WorkWeDocSmartSheetFieldTypeKind::from_code)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetDeleteFieldsRequest {
    pub docid: String,
    pub sheet_id: String,
    pub field_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetFieldGroupChild {
    pub field_id: String,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetAddFieldGroupRequest {
    pub docid: String,
    pub sheet_id: String,
    pub name: String,
    pub children: Vec<WorkWeDocSmartSheetFieldGroupChild>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetUpdateFieldGroupRequest {
    pub docid: String,
    pub sheet_id: String,
    pub field_group_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<WorkWeDocSmartSheetFieldGroupChild>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetFieldGroupResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub field_group: Option<WorkWeDocSmartSheetFieldGroup>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetFieldGroup {
    #[serde(default)]
    pub field_group_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub children: Vec<WorkWeDocSmartSheetFieldGroupChild>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetGetFieldGroupsRequest {
    pub docid: String,
    pub sheet_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetGetFieldGroupsResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub total: Option<i64>,
    #[serde(default)]
    pub has_more: Option<bool>,
    #[serde(default)]
    pub next: Option<i64>,
    #[serde(default)]
    pub field_groups: Vec<WorkWeDocSmartSheetFieldGroup>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetDeleteFieldGroupsRequest {
    pub docid: String,
    pub sheet_id: String,
    pub field_group_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetGetPrivilegesRequest {
    pub docid: String,
    #[serde(rename = "type")]
    pub rule_type: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rule_id_list: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetGetPrivilegesResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub rule_list: Vec<WorkWeDocSmartSheetPrivilegeRule>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetAuthRequest {
    pub docid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_id: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetModifyAuthRequest {
    pub docid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_info: Option<Value>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetAuthResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub docid: Option<String>,
    #[serde(default)]
    pub sheet_id: Option<String>,
    #[serde(default)]
    pub auth_info: Option<Value>,
    #[serde(default)]
    pub field_auth: Option<Value>,
    #[serde(default)]
    pub record_auth: Option<Value>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl WorkWeDocSmartSheetAuthResponse {
    pub fn effective_auth_info(&self) -> Option<&Value> {
        self.auth_info
            .as_ref()
            .or(self.field_auth.as_ref())
            .or(self.record_auth.as_ref())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetPrivilegeRule {
    #[serde(default)]
    pub rule_id: Option<Value>,
    #[serde(rename = "type", default)]
    pub rule_type: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub priv_list: Vec<WorkWeDocSmartSheetPrivilege>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetPrivilege {
    #[serde(default)]
    pub sheet_id: Option<String>,
    #[serde(rename = "priv", default)]
    pub priv_level: Option<i64>,
    #[serde(default)]
    pub can_insert_record: Option<bool>,
    #[serde(default)]
    pub can_delete_record: Option<bool>,
    #[serde(default)]
    pub record_priv: Option<WorkWeDocSmartSheetRecordPrivilege>,
    #[serde(default)]
    pub field_priv: Option<WorkWeDocSmartSheetFieldPrivilege>,
    #[serde(default)]
    pub can_create_modify_delete_view: Option<bool>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetRecordPrivilege {
    #[serde(default)]
    pub record_range_type: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetFieldPrivilege {
    #[serde(default)]
    pub field_range_type: Option<i64>,
    #[serde(default)]
    pub field_rule_list: Vec<WorkWeDocSmartSheetFieldPrivilegeRule>,
    #[serde(default)]
    pub field_default_rule: Option<WorkWeDocSmartSheetFieldPrivilegeRule>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetFieldPrivilegeRule {
    #[serde(default)]
    pub field_id: Option<String>,
    #[serde(default)]
    pub field_type: Option<String>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub can_insert: Option<bool>,
    #[serde(default)]
    pub can_view: Option<bool>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetRecordsMutationRequest {
    pub docid: String,
    pub sheet_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_type: Option<String>,
    pub records: Vec<WorkWeDocSmartSheetRecordMutation>,
}

impl WorkWeDocSmartSheetRecordsMutationRequest {
    pub fn by_field_id(
        docid: impl Into<String>,
        sheet_id: impl Into<String>,
        records: impl IntoIterator<Item = WorkWeDocSmartSheetRecordMutation>,
    ) -> Self {
        Self::new(
            docid,
            sheet_id,
            WorkWeDocSmartSheetCellKeyTypeKind::FieldId,
            records,
        )
    }

    pub fn by_field_title(
        docid: impl Into<String>,
        sheet_id: impl Into<String>,
        records: impl IntoIterator<Item = WorkWeDocSmartSheetRecordMutation>,
    ) -> Self {
        Self::new(
            docid,
            sheet_id,
            WorkWeDocSmartSheetCellKeyTypeKind::FieldTitle,
            records,
        )
    }

    fn new(
        docid: impl Into<String>,
        sheet_id: impl Into<String>,
        key_type: WorkWeDocSmartSheetCellKeyTypeKind,
        records: impl IntoIterator<Item = WorkWeDocSmartSheetRecordMutation>,
    ) -> Self {
        Self {
            docid: docid.into(),
            sheet_id: sheet_id.into(),
            key_type: key_type.as_code().map(str::to_string),
            records: records.into_iter().collect(),
        }
    }

    pub fn key_type_kind(&self) -> Option<WorkWeDocSmartSheetCellKeyTypeKind> {
        self.key_type
            .as_deref()
            .map(WorkWeDocSmartSheetCellKeyTypeKind::from_code)
    }

    pub fn validate_for_add(&self) -> Result<()> {
        self.validate_common()?;
        for record in &self.records {
            record.validate_for_add()?;
        }
        Ok(())
    }

    pub fn validate_for_update(&self) -> Result<()> {
        self.validate_common()?;
        for record in &self.records {
            record.validate_for_update()?;
        }
        Ok(())
    }

    fn validate_common(&self) -> Result<()> {
        validate_wedoc_smartsheet_record_scope(&self.docid, &self.sheet_id)?;
        if self.records.is_empty() {
            return Err(WechatError::Config(
                "WeDoc smart-sheet record mutation requires at least one record".to_string(),
            ));
        }
        if self.records.len() > 500 {
            return Err(WechatError::Config(
                "WeDoc smart-sheet record mutation supports at most 500 records".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkWeDocSmartSheetCellKeyTypeKind {
    FieldTitle,
    FieldId,
    Other,
}

impl WorkWeDocSmartSheetCellKeyTypeKind {
    pub const fn as_code(self) -> Option<&'static str> {
        match self {
            Self::FieldTitle => Some("CELL_VALUE_KEY_TYPE_FIELD_TITLE"),
            Self::FieldId => Some("CELL_VALUE_KEY_TYPE_FIELD_ID"),
            Self::Other => None,
        }
    }

    pub fn from_code(value: &str) -> Self {
        match value {
            "CELL_VALUE_KEY_TYPE_FIELD_TITLE" => Self::FieldTitle,
            "CELL_VALUE_KEY_TYPE_FIELD_ID" => Self::FieldId,
            _ => Self::Other,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkWeDocSmartSheetRecordMutation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
    pub values: Map<String, Value>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl WorkWeDocSmartSheetRecordMutation {
    pub fn add() -> Self {
        Self::default()
    }

    pub fn update(record_id: impl Into<String>) -> Self {
        Self {
            record_id: Some(record_id.into()),
            ..Self::default()
        }
    }

    pub fn raw(mut self, key: impl Into<String>, value: Value) -> Self {
        self.values.insert(key.into(), value);
        self
    }

    pub fn text(
        self,
        key: impl Into<String>,
        texts: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        let values = texts
            .into_iter()
            .map(|text| json!({"type": "text", "text": text.into()}))
            .collect();
        self.raw(key, Value::Array(values))
    }

    pub fn number(self, key: impl Into<String>, number: f64) -> Self {
        self.raw(key, json!(number))
    }

    pub fn checkbox(self, key: impl Into<String>, checked: bool) -> Self {
        self.raw(key, Value::Bool(checked))
    }

    pub fn date_time_millis(self, key: impl Into<String>, timestamp_millis: i64) -> Self {
        self.raw(key, Value::String(timestamp_millis.to_string()))
    }

    pub fn users(
        self,
        key: impl Into<String>,
        userids: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        let values = userids
            .into_iter()
            .map(|userid| json!({"user_id": userid.into()}))
            .collect();
        self.raw(key, Value::Array(values))
    }

    pub fn url(
        self,
        key: impl Into<String>,
        text: impl Into<String>,
        link: impl Into<String>,
    ) -> Self {
        self.raw(
            key,
            json!([{"type": "url", "text": text.into(), "link": link.into()}]),
        )
    }

    pub fn options(
        self,
        key: impl Into<String>,
        options: impl IntoIterator<Item = WorkWeDocSmartSheetSelectOption>,
    ) -> Self {
        self.raw(
            key,
            to_value(options.into_iter().collect::<Vec<_>>()).unwrap(),
        )
    }

    pub fn images(
        self,
        key: impl Into<String>,
        images: impl IntoIterator<Item = WorkWeDocSmartSheetCellImage>,
    ) -> Self {
        self.raw(
            key,
            to_value(images.into_iter().collect::<Vec<_>>()).unwrap(),
        )
    }

    pub fn attachments(
        self,
        key: impl Into<String>,
        attachments: impl IntoIterator<Item = WorkWeDocSmartSheetCellAttachment>,
    ) -> Self {
        self.raw(
            key,
            to_value(attachments.into_iter().collect::<Vec<_>>()).unwrap(),
        )
    }

    pub fn location(
        self,
        key: impl Into<String>,
        location: WorkWeDocSmartSheetCellLocation,
    ) -> Self {
        self.raw(key, json!([location]))
    }

    fn validate_for_add(&self) -> Result<()> {
        if self.values.is_empty() {
            return Err(WechatError::Config(
                "added WeDoc smart-sheet record values cannot be empty".to_string(),
            ));
        }
        validate_wedoc_smartsheet_record_keys(&self.values)
    }

    fn validate_for_update(&self) -> Result<()> {
        if self
            .record_id
            .as_deref()
            .is_none_or(|record_id| record_id.trim().is_empty())
        {
            return Err(WechatError::Config(
                "updated WeDoc smart-sheet record id cannot be empty".to_string(),
            ));
        }
        self.validate_for_add()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkWeDocSmartSheetCellImage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkWeDocSmartSheetCellAttachment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_ext: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_type: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetCellLocation {
    pub id: String,
    pub latitude: String,
    pub longitude: String,
    pub title: String,
    #[serde(default = "default_wedoc_location_source_type")]
    pub source_type: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetGetRecordsRequest {
    pub docid: String,
    pub sheet_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub record_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_type: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub field_titles: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub field_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sort: Vec<WorkWeDocSmartSheetRecordSort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ver: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_spec: Option<WorkWeDocSmartSheetRecordFilter>,
}

impl WorkWeDocSmartSheetGetRecordsRequest {
    pub fn validate(&self) -> Result<()> {
        validate_wedoc_smartsheet_record_scope(&self.docid, &self.sheet_id)?;
        if self.offset.is_some_and(|offset| offset < 0) {
            return Err(WechatError::Config(
                "WeDoc smart-sheet record query offset cannot be negative".to_string(),
            ));
        }
        if self
            .limit
            .is_some_and(|limit| !(1..=1_000).contains(&limit))
        {
            return Err(WechatError::Config(
                "WeDoc smart-sheet record query limit must be between 1 and 1000".to_string(),
            ));
        }
        if !self.sort.is_empty() && self.filter_spec.is_some() {
            return Err(WechatError::Config(
                "WeDoc smart-sheet record query cannot combine sort and filter".to_string(),
            ));
        }
        for sort in &self.sort {
            sort.validate()?;
        }
        if let Some(filter) = &self.filter_spec {
            filter.validate()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetRecordSort {
    pub field_id: String,
    pub desc: bool,
}

impl WorkWeDocSmartSheetRecordSort {
    pub fn asc(field_id: impl Into<String>) -> Self {
        Self {
            field_id: field_id.into(),
            desc: false,
        }
    }

    pub fn desc(field_id: impl Into<String>) -> Self {
        Self {
            field_id: field_id.into(),
            desc: true,
        }
    }

    fn validate(&self) -> Result<()> {
        if self.field_id.trim().is_empty() {
            return Err(WechatError::Config(
                "WeDoc smart-sheet record sort field id cannot be empty".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetRecordFilter {
    pub conjunction: String,
    pub conditions: Vec<WorkWeDocSmartSheetRecordFilterCondition>,
}

impl WorkWeDocSmartSheetRecordFilter {
    pub fn and(
        conditions: impl IntoIterator<Item = WorkWeDocSmartSheetRecordFilterCondition>,
    ) -> Self {
        Self {
            conjunction: "CONJUNCTION_AND".to_string(),
            conditions: conditions.into_iter().collect(),
        }
    }

    pub fn or(
        conditions: impl IntoIterator<Item = WorkWeDocSmartSheetRecordFilterCondition>,
    ) -> Self {
        Self {
            conjunction: "CONJUNCTION_OR".to_string(),
            conditions: conditions.into_iter().collect(),
        }
    }

    fn validate(&self) -> Result<()> {
        if self.conditions.is_empty() {
            return Err(WechatError::Config(
                "WeDoc smart-sheet record filter requires at least one condition".to_string(),
            ));
        }
        for condition in &self.conditions {
            condition.validate()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkWeDocSmartSheetRecordFilterCondition {
    pub field_id: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub string_value: Option<WorkWeDocSmartSheetStringFilterValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_value: Option<WorkWeDocSmartSheetNumberFilterValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bool_value: Option<WorkWeDocSmartSheetBoolFilterValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_time_value: Option<WorkWeDocSmartSheetDateTimeFilterValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_value: Option<WorkWeDocSmartSheetStringFilterValue>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl WorkWeDocSmartSheetRecordFilterCondition {
    pub fn string(
        field_id: impl Into<String>,
        operator: WorkWeDocSmartSheetFilterOperatorKind,
        values: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            field_id: field_id.into(),
            operator: operator.as_code().unwrap_or_default().to_string(),
            string_value: Some(WorkWeDocSmartSheetStringFilterValue {
                value: values.into_iter().map(Into::into).collect(),
            }),
            ..Self::default()
        }
    }

    pub fn number(
        field_id: impl Into<String>,
        operator: WorkWeDocSmartSheetFilterOperatorKind,
        values: impl IntoIterator<Item = f64>,
    ) -> Self {
        Self {
            field_id: field_id.into(),
            operator: operator.as_code().unwrap_or_default().to_string(),
            number_value: Some(WorkWeDocSmartSheetNumberFilterValue {
                value: values.into_iter().collect(),
            }),
            ..Self::default()
        }
    }

    pub fn boolean(
        field_id: impl Into<String>,
        operator: WorkWeDocSmartSheetFilterOperatorKind,
        value: bool,
    ) -> Self {
        Self {
            field_id: field_id.into(),
            operator: operator.as_code().unwrap_or_default().to_string(),
            bool_value: Some(WorkWeDocSmartSheetBoolFilterValue { value }),
            ..Self::default()
        }
    }

    pub fn users(
        field_id: impl Into<String>,
        operator: WorkWeDocSmartSheetFilterOperatorKind,
        values: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            field_id: field_id.into(),
            operator: operator.as_code().unwrap_or_default().to_string(),
            user_value: Some(WorkWeDocSmartSheetStringFilterValue {
                value: values.into_iter().map(Into::into).collect(),
            }),
            ..Self::default()
        }
    }

    pub fn date_time(
        field_id: impl Into<String>,
        operator: WorkWeDocSmartSheetFilterOperatorKind,
        date_time_type: WorkWeDocSmartSheetFilterDateTimeTypeKind,
        values: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            field_id: field_id.into(),
            operator: operator.as_code().unwrap_or_default().to_string(),
            date_time_value: Some(WorkWeDocSmartSheetDateTimeFilterValue {
                date_time_type: date_time_type.as_code().unwrap_or_default().to_string(),
                value: values.into_iter().map(Into::into).collect(),
            }),
            ..Self::default()
        }
    }

    pub fn empty(
        field_id: impl Into<String>,
        operator: WorkWeDocSmartSheetFilterOperatorKind,
    ) -> Self {
        Self {
            field_id: field_id.into(),
            operator: operator.as_code().unwrap_or_default().to_string(),
            ..Self::default()
        }
    }

    pub fn operator_kind(&self) -> WorkWeDocSmartSheetFilterOperatorKind {
        WorkWeDocSmartSheetFilterOperatorKind::from_code(&self.operator)
    }

    fn validate(&self) -> Result<()> {
        if self.field_id.trim().is_empty() || self.operator.trim().is_empty() {
            return Err(WechatError::Config(
                "WeDoc smart-sheet filter field id and operator cannot be empty".to_string(),
            ));
        }
        let value_count = [
            self.string_value.is_some(),
            self.number_value.is_some(),
            self.bool_value.is_some(),
            self.date_time_value.is_some(),
            self.user_value.is_some(),
        ]
        .into_iter()
        .filter(|present| *present)
        .count();
        if value_count > 1 {
            return Err(WechatError::Config(
                "WeDoc smart-sheet filter condition supports one value kind".to_string(),
            ));
        }
        let operator = self.operator_kind();
        if value_count == 0
            && !matches!(
                operator,
                WorkWeDocSmartSheetFilterOperatorKind::IsEmpty
                    | WorkWeDocSmartSheetFilterOperatorKind::IsNotEmpty
            )
        {
            return Err(WechatError::Config(
                "WeDoc smart-sheet filter operator requires a typed value".to_string(),
            ));
        }
        if value_count != 0
            && matches!(
                operator,
                WorkWeDocSmartSheetFilterOperatorKind::IsEmpty
                    | WorkWeDocSmartSheetFilterOperatorKind::IsNotEmpty
            )
        {
            return Err(WechatError::Config(
                "WeDoc smart-sheet empty filter operator cannot include a value".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkWeDocSmartSheetFilterOperatorKind {
    Unknown,
    Is,
    IsNot,
    Contains,
    DoesNotContain,
    IsGreater,
    IsGreaterOrEqual,
    IsLess,
    IsLessOrEqual,
    IsEmpty,
    IsNotEmpty,
    Other,
}

impl WorkWeDocSmartSheetFilterOperatorKind {
    pub const fn as_code(self) -> Option<&'static str> {
        Some(match self {
            Self::Unknown => "OPERATOR_UNKNOWN",
            Self::Is => "OPERATOR_IS",
            Self::IsNot => "OPERATOR_IS_NOT",
            Self::Contains => "OPERATOR_CONTAINS",
            Self::DoesNotContain => "OPERATOR_DOES_NOT_CONTAIN",
            Self::IsGreater => "OPERATOR_IS_GREATER",
            Self::IsGreaterOrEqual => "OPERATOR_IS_GREATER_OR_EQUAL",
            Self::IsLess => "OPERATOR_IS_LESS",
            Self::IsLessOrEqual => "OPERATOR_IS_LESS_OR_EQUAL",
            Self::IsEmpty => "OPERATOR_IS_EMPTY",
            Self::IsNotEmpty => "OPERATOR_IS_NOT_EMPTY",
            Self::Other => return None,
        })
    }

    pub fn from_code(value: &str) -> Self {
        match value {
            "OPERATOR_UNKNOWN" => Self::Unknown,
            "OPERATOR_IS" => Self::Is,
            "OPERATOR_IS_NOT" => Self::IsNot,
            "OPERATOR_CONTAINS" => Self::Contains,
            "OPERATOR_DOES_NOT_CONTAIN" => Self::DoesNotContain,
            "OPERATOR_IS_GREATER" => Self::IsGreater,
            "OPERATOR_IS_GREATER_OR_EQUAL" => Self::IsGreaterOrEqual,
            "OPERATOR_IS_LESS" => Self::IsLess,
            "OPERATOR_IS_LESS_OR_EQUAL" => Self::IsLessOrEqual,
            "OPERATOR_IS_EMPTY" => Self::IsEmpty,
            "OPERATOR_IS_NOT_EMPTY" => Self::IsNotEmpty,
            _ => Self::Other,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkWeDocSmartSheetFilterDateTimeTypeKind {
    DetailDate,
    DetailDateRange,
    Today,
    Tomorrow,
    Yesterday,
    CurrentWeek,
    LastWeek,
    CurrentMonth,
    PastSevenDays,
    NextSevenDays,
    LastMonth,
    PastThirtyDays,
    NextThirtyDays,
    Other,
}

impl WorkWeDocSmartSheetFilterDateTimeTypeKind {
    pub const fn as_code(self) -> Option<&'static str> {
        Some(match self {
            Self::DetailDate => "DATE_TIME_TYPE_DETAIL_DATE",
            Self::DetailDateRange => "DATE_TIME_TYPE_DETAIL_DATE_RANGE",
            Self::Today => "DATE_TIME_TYPE_TODAY",
            Self::Tomorrow => "DATE_TIME_TYPE_TOMORROW",
            Self::Yesterday => "DATE_TIME_TYPE_YESTERDAY",
            Self::CurrentWeek => "DATE_TIME_TYPE_CURRENT_WEEK",
            Self::LastWeek => "DATE_TIME_TYPE_LAST_WEEK",
            Self::CurrentMonth => "DATE_TIME_TYPE_CURRENT_MONTH",
            Self::PastSevenDays => "DATE_TIME_TYPE_THE_PAST_7_DAYS",
            Self::NextSevenDays => "DATE_TIME_TYPE_THE_NEXT_7_DAYS",
            Self::LastMonth => "DATE_TIME_TYPE_LAST_MONTH",
            Self::PastThirtyDays => "DATE_TIME_TYPE_THE_PAST_30_DAYS",
            Self::NextThirtyDays => "DATE_TIME_TYPE_THE_NEXT_30_DAYS",
            Self::Other => return None,
        })
    }

    pub fn from_code(value: &str) -> Self {
        match value {
            "DATE_TIME_TYPE_DETAIL_DATE" => Self::DetailDate,
            "DATE_TIME_TYPE_DETAIL_DATE_RANGE" => Self::DetailDateRange,
            "DATE_TIME_TYPE_TODAY" => Self::Today,
            "DATE_TIME_TYPE_TOMORROW" => Self::Tomorrow,
            "DATE_TIME_TYPE_YESTERDAY" => Self::Yesterday,
            "DATE_TIME_TYPE_CURRENT_WEEK" => Self::CurrentWeek,
            "DATE_TIME_TYPE_LAST_WEEK" => Self::LastWeek,
            "DATE_TIME_TYPE_CURRENT_MONTH" => Self::CurrentMonth,
            "DATE_TIME_TYPE_THE_PAST_7_DAYS" => Self::PastSevenDays,
            "DATE_TIME_TYPE_THE_NEXT_7_DAYS" => Self::NextSevenDays,
            "DATE_TIME_TYPE_LAST_MONTH" => Self::LastMonth,
            "DATE_TIME_TYPE_THE_PAST_30_DAYS" => Self::PastThirtyDays,
            "DATE_TIME_TYPE_THE_NEXT_30_DAYS" => Self::NextThirtyDays,
            _ => Self::Other,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetStringFilterValue {
    pub value: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetNumberFilterValue {
    pub value: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetBoolFilterValue {
    pub value: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetDateTimeFilterValue {
    #[serde(rename = "type")]
    pub date_time_type: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<String>,
}

impl WorkWeDocSmartSheetDateTimeFilterValue {
    pub fn date_time_type_kind(&self) -> WorkWeDocSmartSheetFilterDateTimeTypeKind {
        WorkWeDocSmartSheetFilterDateTimeTypeKind::from_code(&self.date_time_type)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetRecordsResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub records: Vec<WorkWeDocSmartSheetRecord>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetGetRecordsResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub total: Option<i64>,
    #[serde(default)]
    pub has_more: Option<bool>,
    #[serde(default)]
    pub next: Option<i64>,
    #[serde(default)]
    pub records: Vec<WorkWeDocSmartSheetRecord>,
    #[serde(default)]
    pub ver: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetRecord {
    #[serde(default)]
    pub record_id: Option<String>,
    #[serde(default)]
    pub values: Option<Map<String, Value>>,
    #[serde(default)]
    pub create_time: Option<i64>,
    #[serde(default)]
    pub update_time: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocSmartSheetDeleteRecordsRequest {
    pub docid: String,
    pub sheet_id: String,
    pub record_ids: Vec<String>,
}

impl WorkWeDocSmartSheetDeleteRecordsRequest {
    pub fn validate(&self) -> Result<()> {
        validate_wedoc_smartsheet_record_scope(&self.docid, &self.sheet_id)?;
        if self.record_ids.is_empty() {
            return Err(WechatError::Config(
                "WeDoc smart-sheet record deletion requires at least one record id".to_string(),
            ));
        }
        if self.record_ids.len() > 500 {
            return Err(WechatError::Config(
                "WeDoc smart-sheet record deletion supports at most 500 record ids".to_string(),
            ));
        }
        if self.record_ids.iter().any(|id| id.trim().is_empty()) {
            return Err(WechatError::Config(
                "WeDoc smart-sheet record id cannot be empty".to_string(),
            ));
        }
        Ok(())
    }
}

fn validate_wedoc_smartsheet_record_scope(docid: &str, sheet_id: &str) -> Result<()> {
    if docid.trim().is_empty() || sheet_id.trim().is_empty() {
        return Err(WechatError::Config(
            "WeDoc smart-sheet record docid and sheet id cannot be empty".to_string(),
        ));
    }
    Ok(())
}

fn validate_wedoc_smartsheet_record_keys(values: &Map<String, Value>) -> Result<()> {
    if values.keys().any(|key| key.trim().is_empty()) {
        return Err(WechatError::Config(
            "WeDoc smart-sheet record cell key cannot be empty".to_string(),
        ));
    }
    Ok(())
}

const fn default_wedoc_location_source_type() -> i64 {
    1
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocCreateFormRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spaceid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fatherid: Option<String>,
    pub form_info: WorkWeDocFormInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocCreateFormResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub formid: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocModifyFormRequest {
    pub oper: i64,
    pub formid: String,
    pub form_info: WorkWeDocFormInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocFormInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub form_info: Option<WorkWeDocFormInfo>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocFormInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_desc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_header: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_question: Option<WorkWeDocFormQuestion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_setting: Option<WorkWeDocFormSetting>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repeated_id: Vec<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocFormQuestion {
    #[serde(default)]
    pub items: Vec<WorkWeDocFormQuestionItem>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocFormQuestionItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_type: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub must_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub option_item: Vec<WorkWeDocFormQuestionOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_extend_setting: Option<Value>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocFormQuestionOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocFormSetting {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_out_auth: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_in_range: Option<WorkWeDocFormFillInRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting_manager_range: Option<WorkWeDocFormManagerRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_repeat_info: Option<WorkWeDocFormTimedRepeatInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_multi_fill: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fill_cnt: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_finish: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_anonymous: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_notify_submit: Option<bool>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocFormFillInRange {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub userids: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub departmentids: Vec<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocFormManagerRange {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub userids: Vec<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocFormTimedRepeatInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub week_flag: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remind_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_type: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_holiday: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_month: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fork_finish_type: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_ctime: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_mtime: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocFormStatisticRequest {
    pub repeated_id: String,
    pub req_type: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocFormStatisticsResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub statistic_list: Vec<WorkWeDocFormStatistic>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocFormStatistic {
    #[serde(default)]
    pub fill_cnt: Option<i64>,
    #[serde(default)]
    pub repeated_id: Option<String>,
    #[serde(default)]
    pub repeated_name: Option<String>,
    #[serde(default)]
    pub fill_user_cnt: Option<i64>,
    #[serde(default)]
    pub unfill_user_cnt: Option<i64>,
    #[serde(default)]
    pub submit_users: Vec<WorkWeDocFormSubmitUser>,
    #[serde(default)]
    pub unfill_users: Vec<WorkWeDocFormUnfillUser>,
    #[serde(default)]
    pub has_more: Option<bool>,
    #[serde(default)]
    pub cursor: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocFormSubmitUser {
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub tmp_external_userid: Option<String>,
    #[serde(default)]
    pub submit_time: Option<i64>,
    #[serde(default)]
    pub answer_id: Option<i64>,
    #[serde(default)]
    pub user_name: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocFormUnfillUser {
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub user_name: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocFormAnswerRequest {
    pub repeated_id: String,
    pub answer_ids: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocFormAnswersResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub answer: Option<WorkWeDocFormAnswerList>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocFormAnswerList {
    #[serde(default)]
    pub answer_list: Vec<WorkWeDocFormAnswer>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocFormAnswer {
    #[serde(default)]
    pub answer_id: Option<i64>,
    #[serde(default)]
    pub user_name: Option<String>,
    #[serde(default)]
    pub ctime: Option<i64>,
    #[serde(default)]
    pub mtime: Option<i64>,
    #[serde(default)]
    pub reply: Option<WorkWeDocFormReply>,
    #[serde(default)]
    pub answer_status: Option<i64>,
    #[serde(default)]
    pub tmp_external_userid: Option<String>,
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocFormReply {
    #[serde(default)]
    pub items: Vec<WorkWeDocFormReplyItem>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocFormReplyItem {
    #[serde(default)]
    pub question_id: Option<i64>,
    #[serde(default)]
    pub text_reply: Option<String>,
    #[serde(default)]
    pub option_reply: Vec<i64>,
    #[serde(default)]
    pub option_extend_reply: Vec<WorkWeDocFormOptionExtendReply>,
    #[serde(default)]
    pub file_extend_reply: Vec<WorkWeDocFormFileReply>,
    #[serde(default)]
    pub department_reply: Option<WorkWeDocFormDepartmentReply>,
    #[serde(default)]
    pub member_reply: Option<WorkWeDocFormMemberReply>,
    #[serde(default)]
    pub duration_reply: Option<WorkWeDocFormDurationReply>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocFormOptionExtendReply {
    #[serde(default)]
    pub option_reply: Option<i64>,
    #[serde(default)]
    pub extend_text: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocFormFileReply {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub fileid: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocFormDepartmentReply {
    #[serde(default)]
    pub list: Vec<WorkWeDocFormDepartment>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocFormDepartment {
    #[serde(default)]
    pub department_id: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocFormMemberReply {
    #[serde(default)]
    pub list: Vec<WorkWeDocFormMember>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocFormMember {
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocFormDurationReply {
    #[serde(default)]
    pub begin_time: Option<i64>,
    #[serde(default)]
    pub end_time: Option<i64>,
    #[serde(default)]
    pub time_scale: Option<i64>,
    #[serde(default)]
    pub day_range: Option<i64>,
    #[serde(default)]
    pub days: Option<f64>,
    #[serde(default)]
    pub hours: Option<f64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkLivingCreateRequest {
    pub anchor_userid: String,
    pub theme: String,
    pub living_start: i64,
    pub living_duration: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub living_type: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agentid: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remind_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_cover_mediaid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_share_mediaid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_detail: Option<WorkLivingActivityDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkLivingActivityDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub image_list: Vec<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkLivingModifyRequest {
    pub livingid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub living_start: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub living_duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub living_type: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remind_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkLivingGetUserAllLivingIdRequest {
    pub userid: String,
    pub cursor: String,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkLivingCreateResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub livingid: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkLivingCodeResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub living_code: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkLivingGetUserAllLivingIdResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub next_cursor: Option<String>,
    #[serde(default)]
    pub livingid_list: Vec<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkLivingInfo {
    #[serde(default)]
    pub anchor_userid: Option<String>,
    #[serde(default)]
    pub theme: Option<String>,
    #[serde(default)]
    pub living_start: Option<i64>,
    #[serde(default)]
    pub living_duration: Option<i64>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default, rename = "type")]
    pub living_type: Option<i64>,
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default)]
    pub main_department: Option<i64>,
    #[serde(default)]
    pub viewer_num: Option<i64>,
    #[serde(default)]
    pub online_count: Option<i64>,
    #[serde(default)]
    pub open_replay: Option<i64>,
    #[serde(default)]
    pub reserve_living_duration: Option<i64>,
    #[serde(default)]
    pub reserve_start: Option<i64>,
    #[serde(default)]
    pub replay_status: Option<i64>,
    #[serde(default)]
    pub mic_num: Option<i64>,
    #[serde(default)]
    pub push_stream_url: Option<String>,
    #[serde(default)]
    pub subscribe_count: Option<i64>,
    #[serde(default)]
    pub comment_num: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkLivingInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub living_info: Option<WorkLivingInfo>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkLivingInternalViewer {
    pub userid: String,
    pub watch_time: i64,
    pub is_comment: i64,
    pub is_mic: i64,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkLivingExternalViewer {
    pub external_userid: String,
    #[serde(rename = "type")]
    pub viewer_type: i64,
    pub name: String,
    pub watch_time: i64,
    pub is_comment: i64,
    pub is_mic: i64,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkLivingWatchStatInfo {
    #[serde(default)]
    pub users: Vec<WorkLivingInternalViewer>,
    #[serde(default)]
    pub external_users: Vec<WorkLivingExternalViewer>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkLivingWatchStatResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub ending: Option<i64>,
    #[serde(default)]
    pub next_key: Option<String>,
    #[serde(default)]
    pub stat_info: Option<WorkLivingWatchStatInfo>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkLivingShareInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub livingid: Option<String>,
    #[serde(default)]
    pub viewer_userid: Option<String>,
    #[serde(default)]
    pub viewer_external_userid: Option<String>,
    #[serde(default)]
    pub invitor_userid: Option<String>,
    #[serde(default)]
    pub invitor_external_userid: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDriveSpaceCreateRequest {
    pub userid: String,
    pub space_name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub auth_info: Vec<WorkWeDriveAuthInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_sub_type: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDriveSpaceRenameRequest {
    pub userid: String,
    pub spaceid: String,
    pub space_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDriveSpaceIdRequest {
    pub userid: String,
    pub spaceid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDriveSpaceAclRequest {
    pub userid: String,
    pub spaceid: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub auth_info: Vec<WorkWeDriveAuthInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDriveSpaceSettingRequest {
    pub userid: String,
    pub spaceid: String,
    pub enable_watermark: bool,
    pub add_member_only_admin: bool,
    pub enable_share_url: bool,
    pub share_url_no_approve: bool,
    pub share_url_no_approve_default_auth: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDriveFileListRequest {
    pub userid: String,
    pub spaceid: String,
    pub fatherid: String,
    pub sort_type: i64,
    pub start: i64,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDriveFileUploadRequest {
    pub userid: String,
    pub spaceid: String,
    pub fatherid: String,
    pub file_name: String,
    pub file_base64_content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDriveFileIdRequest {
    pub userid: String,
    pub fileid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDriveFileCreateRequest {
    pub userid: String,
    pub spaceid: String,
    pub fatherid: String,
    pub file_type: String,
    pub file_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDriveFileRenameRequest {
    pub userid: String,
    pub fileid: String,
    pub new_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDriveFileMoveRequest {
    pub userid: String,
    pub fatherid: String,
    pub replace: bool,
    pub fileid: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDriveFileAclRequest {
    pub userid: String,
    pub fileid: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub auth_info: Vec<WorkWeDriveAuthInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDriveAuthInfo {
    #[serde(rename = "type")]
    pub member_type: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departmentid: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDriveFileSettingRequest {
    pub userid: String,
    pub fileid: String,
    pub auth_scope: i64,
    pub auth: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDriveSpaceCreateResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub spaceid: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDriveSpaceInfo {
    #[serde(default)]
    pub spaceid: Option<String>,
    #[serde(default)]
    pub space_name: Option<String>,
    #[serde(default)]
    pub auth_list: Option<WorkWeDriveAuthList>,
    #[serde(default)]
    pub space_sub_type: Option<i64>,
    #[serde(default)]
    pub secure_setting: Option<WorkWeDriveSpaceSecureSetting>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDriveAuthList {
    #[serde(default)]
    pub auth_info: Vec<WorkWeDriveAuthInfo>,
    #[serde(default)]
    pub quit_userid: Vec<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDriveSpaceSecureSetting {
    #[serde(default)]
    pub enable_watermark: Option<bool>,
    #[serde(default)]
    pub add_member_only_admin: Option<bool>,
    #[serde(default)]
    pub enable_share_url: Option<bool>,
    #[serde(default)]
    pub share_url_no_approve: Option<bool>,
    #[serde(default)]
    pub share_url_no_approve_default_auth: Option<i64>,
    #[serde(default)]
    pub enable_share_external: Option<bool>,
    #[serde(default)]
    pub enable_share_external_admin: Option<bool>,
    #[serde(default)]
    pub enable_space_add_external_member: Option<bool>,
    #[serde(default)]
    pub enable_space_add_external_member_admin: Option<bool>,
    #[serde(default)]
    pub enable_confidential_mode: Option<bool>,
    #[serde(default)]
    pub default_file_scope: Option<i64>,
    #[serde(default)]
    pub create_file_only_admin: Option<bool>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDriveSpaceInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub space_info: Option<WorkWeDriveSpaceInfo>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDriveSpaceShareResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub space_share_url: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDriveFileListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub has_more: Option<bool>,
    #[serde(default)]
    pub next_start: Option<i64>,
    #[serde(default)]
    pub file_list: Option<WorkWeDriveFileList>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDriveFileList {
    #[serde(default)]
    pub item: Vec<WorkWeDriveFileInfo>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDriveFileUploadResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub fileid: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDriveFileDownloadResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub download_url: Option<String>,
    #[serde(default)]
    pub cookie_name: Option<String>,
    #[serde(default)]
    pub cookie_value: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDriveFileCreateResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub fileid: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDriveFileInfo {
    #[serde(default)]
    pub fileid: Option<String>,
    #[serde(default)]
    pub file_name: Option<String>,
    #[serde(default)]
    pub file_type: Option<i64>,
    #[serde(default)]
    pub file_status: Option<i64>,
    #[serde(default)]
    pub file_size: Option<u64>,
    #[serde(default)]
    pub spaceid: Option<String>,
    #[serde(default)]
    pub fatherid: Option<String>,
    #[serde(default)]
    pub ctime: Option<u64>,
    #[serde(default)]
    pub mtime: Option<u64>,
    #[serde(default)]
    pub create_userid: Option<String>,
    #[serde(default)]
    pub update_userid: Option<String>,
    #[serde(default)]
    pub sha: Option<String>,
    #[serde(default)]
    pub md5: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDriveFileRenameResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub file: Option<WorkWeDriveFileInfo>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDriveFileInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub file_info: Option<WorkWeDriveFileInfo>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDriveFileMoveResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub file_list: Option<WorkWeDriveFileList>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDriveFileShareResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub share_url: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkScheduleAddRequest {
    pub schedule: WorkScheduleCreate,
    pub agentid: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkScheduleCreate {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub admins: Vec<String>,
    pub start_time: i64,
    pub end_time: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attendees: Vec<WorkScheduleAttendee>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminders: Option<WorkScheduleReminders>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cal_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizer: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkScheduleUpdateRequest {
    pub schedule: WorkScheduleUpdate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkScheduleUpdate {
    pub schedule_id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub admins: Vec<String>,
    pub start_time: i64,
    pub end_time: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attendees: Vec<WorkScheduleAttendee>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminders: Option<WorkScheduleReminders>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cal_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizer: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkScheduleAttendee {
    pub userid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_status: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkScheduleReminders {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_remind: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remind_before_event_secs: Option<i64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub remind_time_diffs: Vec<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_repeat: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_type: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_until: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_custom_repeat: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_interval: Option<i64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repeat_day_of_week: Vec<i64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repeat_day_of_month: Vec<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<i64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude_time_list: Vec<WorkScheduleExcludeTime>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkScheduleExcludeTime {
    pub start_time: i64,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkScheduleByCalendarRequest {
    pub cal_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkScheduleAddResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub schedule_id: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkScheduleInfo {
    #[serde(default)]
    pub schedule_id: Option<String>,
    #[serde(default)]
    pub organizer: Option<String>,
    #[serde(default)]
    pub admins: Vec<String>,
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub start_time: Option<i64>,
    #[serde(default)]
    pub end_time: Option<i64>,
    #[serde(default)]
    pub attendees: Vec<WorkScheduleAttendee>,
    #[serde(default)]
    pub reminders: Option<WorkScheduleReminders>,
    #[serde(default)]
    pub location: Option<String>,
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default)]
    pub cal_id: Option<String>,
    #[serde(default)]
    pub sequence: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkScheduleGetResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub schedule_list: Vec<WorkScheduleInfo>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
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
    pub text: Option<WorkTextMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<WorkMediaMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<WorkMediaMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<WorkVideoMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<WorkMediaMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub textcard: Option<WorkTextCardMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub news: Option<WorkNewsMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpnews: Option<WorkMpNewsMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown: Option<WorkMarkdownMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safe: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl AppChatMessage {
    pub fn msgtype_kind(&self) -> WorkMessageTypeKind {
        WorkMessageTypeKind::from_code(&self.msgtype)
    }

    pub fn text(chat_id: impl Into<String>, content: impl Into<String>) -> Self {
        let mut message = Self::empty(chat_id, WorkMessageTypeKind::Text);
        message.text = Some(WorkTextMessage {
            content: content.into(),
        });
        message
    }

    pub fn image(chat_id: impl Into<String>, media_id: impl Into<String>) -> Self {
        Self::media(chat_id, WorkMessageTypeKind::Image, media_id)
    }

    pub fn voice(chat_id: impl Into<String>, media_id: impl Into<String>) -> Self {
        Self::media(chat_id, WorkMessageTypeKind::Voice, media_id)
    }

    pub fn video(chat_id: impl Into<String>, video: WorkVideoMessage) -> Self {
        let mut message = Self::empty(chat_id, WorkMessageTypeKind::Video);
        message.video = Some(video);
        message
    }

    pub fn file(chat_id: impl Into<String>, media_id: impl Into<String>) -> Self {
        Self::media(chat_id, WorkMessageTypeKind::File, media_id)
    }

    pub fn text_card(chat_id: impl Into<String>, text_card: WorkTextCardMessage) -> Self {
        let mut message = Self::empty(chat_id, WorkMessageTypeKind::TextCard);
        message.textcard = Some(text_card);
        message
    }

    pub fn news(chat_id: impl Into<String>, articles: Vec<WorkNewsArticle>) -> Self {
        let mut message = Self::empty(chat_id, WorkMessageTypeKind::News);
        message.news = Some(WorkNewsMessage { articles });
        message
    }

    pub fn mpnews(chat_id: impl Into<String>, articles: Vec<WorkMpNewsArticle>) -> Self {
        let mut message = Self::empty(chat_id, WorkMessageTypeKind::MpNews);
        message.mpnews = Some(WorkMpNewsMessage { articles });
        message
    }

    pub fn markdown(chat_id: impl Into<String>, content: impl Into<String>) -> Self {
        let mut message = Self::empty(chat_id, WorkMessageTypeKind::Markdown);
        message.markdown = Some(WorkMarkdownMessage {
            content: content.into(),
        });
        message
    }

    pub fn with_safe(mut self, safe: bool) -> Self {
        self.safe = Some(i64::from(safe));
        self
    }

    fn media(
        chat_id: impl Into<String>,
        msg_type: WorkMessageTypeKind,
        media_id: impl Into<String>,
    ) -> Self {
        let media = WorkMediaMessage {
            media_id: media_id.into(),
        };
        let mut message = Self::empty(chat_id, msg_type);
        match msg_type {
            WorkMessageTypeKind::Image => message.image = Some(media),
            WorkMessageTypeKind::Voice => message.voice = Some(media),
            WorkMessageTypeKind::File => message.file = Some(media),
            _ => unreachable!("AppChat media helper only accepts image, voice, or file"),
        }
        message
    }

    fn empty(chat_id: impl Into<String>, msg_type: WorkMessageTypeKind) -> Self {
        Self {
            chatid: chat_id.into(),
            msgtype: msg_type.as_code().to_string(),
            text: None,
            image: None,
            voice: None,
            video: None,
            file: None,
            textcard: None,
            news: None,
            mpnews: None,
            markdown: None,
            safe: None,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkOauthUserInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default, alias = "UserId")]
    pub userid: Option<String>,
    #[serde(default)]
    pub user_ticket: Option<String>,
    #[serde(default)]
    pub expires_in: Option<i64>,
    #[serde(default, alias = "OpenId")]
    pub openid: Option<String>,
    #[serde(default)]
    pub external_userid: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkOauthUserDetailResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub gender: Option<String>,
    #[serde(default)]
    pub avatar: Option<String>,
    #[serde(default)]
    pub qr_code: Option<String>,
    #[serde(default)]
    pub mobile: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub biz_mail: Option<String>,
    #[serde(default)]
    pub address: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUserTfaInfoRequest {
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUserTfaInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub tfa_code: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUserTfaSuccessRequest {
    pub userid: String,
    pub tfa_code: String,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn serializes_text_message_shape() {
        let text_message = WorkMessage {
            touser: Some("user".to_string()),
            toparty: None,
            totag: None,
            msgtype: "text".to_string(),
            agentid: 100001,
            text: Some(WorkTextMessage {
                content: "hello".to_string(),
            }),
            image: None,
            voice: None,
            video: None,
            file: None,
            markdown: None,
            textcard: None,
            news: None,
            mpnews: None,
            miniprogram_notice: None,
            taskcard: None,
            template_card: None,
            safe: Some(0),
            enable_id_trans: None,
            enable_duplicate_check: None,
            duplicate_check_interval: None,
            extra: serde_json::Value::Null,
        };
        assert_eq!(text_message.msgtype_kind(), WorkMessageTypeKind::Text);
        let value = serde_json::to_value(text_message).unwrap();

        assert_eq!(value["touser"], "user");
        assert_eq!(value["msgtype"], "text");
        assert_eq!(value["text"]["content"], "hello");
        assert_eq!(
            WorkMessageTypeKind::from_code("mini_program_notice"),
            WorkMessageTypeKind::MiniProgramNotice
        );
        assert_eq!(
            WorkMessageTypeKind::from_code("templatecard"),
            WorkMessageTypeKind::TemplateCard
        );
        assert_eq!(
            WorkMessageTypeKind::from_code("SOMETHING_NEW"),
            WorkMessageTypeKind::Other
        );
        assert!(WorkMessageTypeKind::Image.is_media());
        assert!(!WorkMessageTypeKind::Text.is_media());

        let markdown = serde_json::to_value(WorkMessage {
            touser: Some("user".to_string()),
            toparty: None,
            totag: None,
            msgtype: "markdown".to_string(),
            agentid: 100001,
            text: None,
            image: None,
            voice: None,
            video: None,
            file: None,
            markdown: Some(WorkMarkdownMessage {
                content: "**hello**".to_string(),
            }),
            textcard: None,
            news: None,
            mpnews: None,
            miniprogram_notice: None,
            taskcard: None,
            template_card: None,
            safe: Some(0),
            enable_id_trans: None,
            enable_duplicate_check: None,
            duplicate_check_interval: None,
            extra: serde_json::Value::Null,
        })
        .unwrap();
        assert_eq!(markdown["markdown"]["content"], "**hello**");

        let textcard = serde_json::to_value(WorkMessage {
            touser: Some("user".to_string()),
            toparty: None,
            totag: None,
            msgtype: "textcard".to_string(),
            agentid: 100001,
            text: None,
            image: None,
            voice: None,
            video: None,
            file: None,
            markdown: None,
            textcard: Some(WorkTextCardMessage {
                title: "title".to_string(),
                description: "desc".to_string(),
                url: "https://example.com".to_string(),
                btntxt: Some("open".to_string()),
            }),
            news: None,
            mpnews: None,
            miniprogram_notice: None,
            taskcard: None,
            template_card: None,
            safe: Some(0),
            enable_id_trans: None,
            enable_duplicate_check: None,
            duplicate_check_interval: None,
            extra: serde_json::Value::Null,
        })
        .unwrap();
        assert_eq!(textcard["textcard"]["btntxt"], "open");

        let image = serde_json::to_value(WorkMessage {
            touser: Some("user".to_string()),
            toparty: None,
            totag: None,
            msgtype: "image".to_string(),
            agentid: 100001,
            text: None,
            image: Some(WorkMediaMessage {
                media_id: "image-media".to_string(),
            }),
            voice: None,
            video: None,
            file: None,
            markdown: None,
            textcard: None,
            news: None,
            mpnews: None,
            miniprogram_notice: None,
            taskcard: None,
            template_card: None,
            safe: Some(0),
            enable_id_trans: None,
            enable_duplicate_check: None,
            duplicate_check_interval: None,
            extra: serde_json::Value::Null,
        })
        .unwrap();
        assert_eq!(image["image"]["media_id"], "image-media");

        let news = serde_json::to_value(WorkMessage {
            touser: Some("user".to_string()),
            toparty: None,
            totag: None,
            msgtype: "news".to_string(),
            agentid: 100001,
            text: None,
            image: None,
            voice: None,
            video: None,
            file: None,
            markdown: None,
            textcard: None,
            news: Some(WorkNewsMessage {
                articles: vec![WorkNewsArticle {
                    title: "title".to_string(),
                    description: "desc".to_string(),
                    url: "https://example.com".to_string(),
                    picurl: "https://example.com/a.png".to_string(),
                }],
            }),
            mpnews: None,
            miniprogram_notice: None,
            taskcard: None,
            template_card: None,
            safe: Some(0),
            enable_id_trans: None,
            enable_duplicate_check: None,
            duplicate_check_interval: None,
            extra: serde_json::Value::Null,
        })
        .unwrap();
        assert_eq!(
            news["news"]["articles"][0]["picurl"],
            "https://example.com/a.png"
        );

        let mpnews = serde_json::to_value(WorkMessage {
            touser: Some("user".to_string()),
            toparty: None,
            totag: None,
            msgtype: "mpnews".to_string(),
            agentid: 100001,
            text: None,
            image: None,
            voice: None,
            video: None,
            file: None,
            markdown: None,
            textcard: None,
            news: None,
            mpnews: Some(WorkMpNewsMessage {
                articles: vec![WorkMpNewsArticle {
                    title: "title".to_string(),
                    thumb_media_id: "thumb".to_string(),
                    author: "author".to_string(),
                    content_source_url: "https://example.com/source".to_string(),
                    content: "content".to_string(),
                    digest: "digest".to_string(),
                }],
            }),
            miniprogram_notice: None,
            taskcard: None,
            template_card: None,
            safe: Some(0),
            enable_id_trans: None,
            enable_duplicate_check: None,
            duplicate_check_interval: None,
            extra: serde_json::Value::Null,
        })
        .unwrap();
        assert_eq!(mpnews["mpnews"]["articles"][0]["thumb_media_id"], "thumb");
    }

    #[test]
    fn serializes_work_task_card_and_message_statistics_contracts() {
        let task_card = WorkTaskCardMessage {
            title: "Approval".to_string(),
            description: "Please review".to_string(),
            url: Some("https://example.test/task".to_string()),
            task_id: "task-1".to_string(),
            btn: vec![WorkTaskCardButton {
                key: "approve".to_string(),
                name: "Approve".to_string(),
                replace_name: Some("Approved".to_string()),
                color: Some("red".to_string()),
                is_bold: Some(true),
            }],
        };
        let message = serde_json::to_value(WorkMessage {
            touser: Some("user".to_string()),
            toparty: None,
            totag: None,
            msgtype: WorkMessageTypeKind::TaskCard.as_code().to_string(),
            agentid: 100001,
            text: None,
            image: None,
            voice: None,
            video: None,
            file: None,
            markdown: None,
            textcard: None,
            news: None,
            mpnews: None,
            miniprogram_notice: None,
            taskcard: Some(task_card),
            template_card: None,
            safe: Some(0),
            enable_id_trans: None,
            enable_duplicate_check: None,
            duplicate_check_interval: None,
            extra: Value::Null,
        })
        .unwrap();
        assert_eq!(message["msgtype"], "taskcard");
        assert_eq!(message["taskcard"]["task_id"], "task-1");
        assert_eq!(message["taskcard"]["btn"][0]["replace_name"], "Approved");
        assert_eq!(message["taskcard"]["btn"][0]["is_bold"], true);
        assert_eq!(
            WorkMessageTypeKind::from_code("task_card"),
            WorkMessageTypeKind::TaskCard
        );

        let statistics =
            WorkMessageStatisticsRequest::new(WorkMessageStatisticsTimeKind::Yesterday);
        assert_eq!(
            statistics.time_kind(),
            Some(WorkMessageStatisticsTimeKind::Yesterday)
        );
        assert_eq!(
            serde_json::to_value(statistics).unwrap(),
            json!({ "time_type": 1 })
        );

        let update = serde_json::to_value(WorkTaskCardUpdateRequest {
            userids: vec!["user".to_string(), "other".to_string()],
            agentid: 100001,
            task_id: "task-1".to_string(),
            clicked_key: "approve".to_string(),
            extra: json!({ "trace_id": "task-update" }),
        })
        .unwrap();
        assert_eq!(update["userids"][1], "other");
        assert_eq!(update["clicked_key"], "approve");
        assert_eq!(update["trace_id"], "task-update");
    }

    #[test]
    fn deserializes_work_message_statistics_and_task_card_update_responses() {
        let response: WorkMessageStatisticsResponse = serde_json::from_value(json!({
            "errcode": 0,
            "statistics": [{
                "app_name": "CRM",
                "agentid": 100001,
                "count": 12,
                "failed_count": 2
            }, {
                "app_name": "OA",
                "agentid": 100002,
                "count": 8
            }],
            "statistic_date": "2026-07-17"
        }))
        .unwrap();
        assert_eq!(response.total_count(), 20);
        let crm = response.by_agent_id(100001).expect("CRM statistics");
        assert_eq!(crm.app_name.as_deref(), Some("CRM"));
        assert_eq!(crm.extra["failed_count"], 2);
        assert_eq!(response.extra["statistic_date"], "2026-07-17");

        let update: WorkTaskCardUpdateResponse = serde_json::from_value(json!({
            "errcode": 0,
            "invaliduser": "missing-user|inactive-user",
            "request_id": "task-update"
        }))
        .unwrap();
        assert_eq!(
            update.invalid_users(),
            vec!["missing-user", "inactive-user"]
        );
        assert!(update.has_delivery_failures());
        assert_eq!(update.extra["request_id"], "task-update");
    }

    #[test]
    fn serializes_typed_work_message_payloads() {
        let audience = serde_json::to_value(WorkMessageAudience::to_user(100001, "user")).unwrap();
        assert_eq!(audience["touser"], "user");
        assert_eq!(audience["agentid"], 100001);
        assert_eq!(audience["safe"], 0);

        let video = serde_json::to_value(WorkVideoMessage {
            media_id: "media".to_string(),
            title: Some("title".to_string()),
            description: Some("desc".to_string()),
        })
        .unwrap();
        assert_eq!(video["media_id"], "media");
        assert_eq!(video["description"], "desc");

        let text_card = serde_json::to_value(WorkTextCardMessage {
            title: "title".to_string(),
            description: "desc".to_string(),
            url: "https://example.com".to_string(),
            btntxt: Some("open".to_string()),
        })
        .unwrap();
        assert_eq!(text_card["btntxt"], "open");

        let news = serde_json::to_value(WorkNewsArticle {
            title: "title".to_string(),
            description: "desc".to_string(),
            url: "https://example.com".to_string(),
            picurl: "https://example.com/a.png".to_string(),
        })
        .unwrap();
        assert_eq!(news["picurl"], "https://example.com/a.png");

        let mpnews = serde_json::to_value(WorkMpNewsArticle {
            title: "title".to_string(),
            thumb_media_id: "thumb".to_string(),
            author: "author".to_string(),
            content_source_url: "https://example.com/source".to_string(),
            content: "content".to_string(),
            digest: "digest".to_string(),
        })
        .unwrap();
        assert_eq!(mpnews["thumb_media_id"], "thumb");

        let notice = serde_json::to_value(WorkMiniProgramNoticeMessage {
            appid: "wx-app".to_string(),
            page: Some("pages/index".to_string()),
            title: "notice".to_string(),
            description: Some("desc".to_string()),
            emphasis_first_item: Some(true),
            content_item: vec![WorkMiniProgramNoticeItem {
                key: "time".to_string(),
                value: "10:00".to_string(),
            }],
        })
        .unwrap();
        assert_eq!(notice["appid"], "wx-app");
        assert_eq!(notice["content_item"][0]["key"], "time");
        assert_eq!(notice["emphasis_first_item"], true);

        let mut template_card = WorkTemplateCard::new(WorkTemplateCardTypeKind::ButtonInteraction);
        template_card.source = Some(WorkTemplateCardSource {
            icon_url: "https://example.com/icon.png".to_string(),
            desc: "Roze".to_string(),
            desc_color: Some(1),
        });
        template_card.main_title = Some(WorkTemplateCardMainTitle {
            title: "Approval".to_string(),
            desc: "Choose an action".to_string(),
        });
        template_card.button_list = vec![WorkTemplateCardButton {
            text: "Approve".to_string(),
            style: 1,
            key: "approve".to_string(),
        }];
        template_card.checkbox = Some(WorkTemplateCardCheckbox {
            question_key: "terms".to_string(),
            option_list: vec![WorkTemplateCardCheckboxOption {
                id: "accept".to_string(),
                text: "Accept".to_string(),
                is_checked: true,
            }],
            mode: 0,
        });
        assert_eq!(
            template_card.card_type_kind(),
            WorkTemplateCardTypeKind::ButtonInteraction
        );

        let update_request = WorkTemplateCardUpdateRequest {
            userids: vec!["user".to_string()],
            partyids: Vec::new(),
            tagids: Vec::new(),
            atall: None,
            agentid: 100001,
            response_code: "response".to_string(),
            button: Some(WorkTemplateCardUpdateButton {
                replace_name: "done".to_string(),
                extra: Value::Null,
            }),
            template_card: Some(template_card),
            extra: serde_json::Value::Null,
        };
        assert_eq!(
            update_request.template_card_type_kind(),
            Some(WorkTemplateCardTypeKind::ButtonInteraction)
        );
        assert!(WorkTemplateCardTypeKind::ButtonInteraction.is_interactive());
        assert!(WorkTemplateCardTypeKind::VoteInteraction.is_interactive());
        assert!(WorkTemplateCardTypeKind::MultipleInteraction.is_interactive());
        assert!(!WorkTemplateCardTypeKind::TextNotice.is_interactive());
        assert_eq!(
            WorkTemplateCardTypeKind::from_code("NEWS_NOTICE"),
            WorkTemplateCardTypeKind::NewsNotice
        );
        assert_eq!(
            WorkTemplateCardTypeKind::from_code("SOMETHING_NEW"),
            WorkTemplateCardTypeKind::Other
        );
        assert_eq!(
            WorkTemplateCardTypeKind::from_code("VOTE_INTERACTION"),
            WorkTemplateCardTypeKind::VoteInteraction
        );
        assert_eq!(
            WorkTemplateCardTypeKind::MultipleInteraction.as_code(),
            "multiple_interaction"
        );
        let update = serde_json::to_value(update_request).unwrap();
        assert_eq!(update["userids"][0], "user");
        assert_eq!(update["response_code"], "response");
        assert_eq!(update["button"]["replace_name"], "done");
        assert_eq!(update["template_card"]["source"]["desc_color"], 1);
        assert_eq!(update["template_card"]["button_list"][0]["key"], "approve");
        assert_eq!(
            update["template_card"]["checkbox"]["option_list"][0]["is_checked"],
            true
        );
        assert!(update.get("partyids").is_none());

        let response: MessageSendResponse = serde_json::from_value(json!({
            "errcode": 0,
            "invaliduser": "bad-user| second-user ||",
            "invalidparty": "party-a",
            "invalidtag": "tag-a",
            "unlicenseduser": "user-c|user-d",
            "msgid": "msg",
            "response_code": "response",
            "request_id": "req-1"
        }))
        .unwrap();
        assert_eq!(
            response.invaliduser.as_deref(),
            Some("bad-user| second-user ||")
        );
        assert_eq!(response.invalid_users(), vec!["bad-user", "second-user"]);
        assert_eq!(response.invalid_parties(), vec!["party-a"]);
        assert_eq!(response.invalid_tags(), vec!["tag-a"]);
        assert_eq!(response.unlicensed_users(), vec!["user-c", "user-d"]);
        assert!(response.has_delivery_failures());
        assert_eq!(response.msgid.as_deref(), Some("msg"));
        assert_eq!(response.response_code.as_deref(), Some("response"));
        assert_eq!(response.extra["request_id"], "req-1");

        let delivered: MessageSendResponse =
            serde_json::from_value(json!({ "errcode": 0, "errmsg": "ok" })).unwrap();
        assert!(!delivered.has_delivery_failures());
    }

    #[test]
    fn serializes_linked_corp_and_school_message_responses() {
        let linked_body = serde_json::to_value(WorkLinkedCorpMessage {
            touser: vec!["Corp/user".to_string()],
            toparty: vec!["Corp/party".to_string()],
            totag: vec!["Corp/tag".to_string()],
            msgtype: "text".to_string(),
            agentid: 100001,
            text: Some(WorkTextMessage {
                content: "hello".to_string(),
            }),
            image: None,
            voice: None,
            video: None,
            file: None,
            textcard: None,
            news: None,
            mpnews: None,
            markdown: None,
            miniprogram_notice: None,
            extra: serde_json::Value::Null,
        })
        .unwrap();
        assert_eq!(linked_body["touser"][0], "Corp/user");
        assert_eq!(linked_body["text"]["content"], "hello");

        let linked_text = serde_json::to_value(
            WorkLinkedCorpMessage::text(100001, "hello")
                .with_touser("Corp/user")
                .with_toparty("Corp/party")
                .with_totag("Corp/tag"),
        )
        .unwrap();
        assert_eq!(linked_text["msgtype"], "text");
        assert_eq!(linked_text["agentid"], 100001);
        assert_eq!(linked_text["touser"][0], "Corp/user");
        assert_eq!(linked_text["text"]["content"], "hello");
        assert_eq!(
            WorkLinkedCorpMessage::text(100001, "hello").msgtype_kind(),
            WorkMessageTypeKind::Text
        );

        let linked_file =
            serde_json::to_value(WorkLinkedCorpMessage::file(100001, "file-media")).unwrap();
        assert_eq!(linked_file["msgtype"], "file");
        assert_eq!(linked_file["file"]["media_id"], "file-media");
        assert!(linked_file.get("touser").is_none());

        let mut linked_rich = WorkLinkedCorpMessage::empty(100001, WorkMessageTypeKind::Video);
        linked_rich.video = Some(WorkVideoMessage {
            media_id: "video-media".to_string(),
            title: Some("title".to_string()),
            description: Some("desc".to_string()),
        });
        linked_rich.news = Some(WorkNewsMessage {
            articles: vec![WorkNewsArticle {
                title: "news".to_string(),
                description: "desc".to_string(),
                url: "https://example.com".to_string(),
                picurl: "https://example.com/a.png".to_string(),
            }],
        });
        linked_rich.mpnews = Some(WorkMpNewsMessage {
            articles: vec![WorkMpNewsArticle {
                title: "mpnews".to_string(),
                thumb_media_id: "thumb".to_string(),
                author: "author".to_string(),
                content_source_url: "https://example.com/source".to_string(),
                content: "content".to_string(),
                digest: "digest".to_string(),
            }],
        });
        let linked_rich = serde_json::to_value(linked_rich).unwrap();
        assert_eq!(linked_rich["video"]["title"], "title");
        assert_eq!(linked_rich["news"]["articles"][0]["title"], "news");
        assert_eq!(
            linked_rich["mpnews"]["articles"][0]["thumb_media_id"],
            "thumb"
        );

        let linked_response: WorkLinkedCorpMessageSendResponse = serde_json::from_value(json!({
            "errcode": 0,
            "invaliduser": ["Corp/bad-user"],
            "invalidparty": ["Corp/bad-party"],
            "invalidtag": ["Corp/bad-tag"],
            "msgid": "linked-msg"
        }))
        .unwrap();
        assert_eq!(linked_response.invaliduser[0], "Corp/bad-user");
        assert_eq!(linked_response.invalidparty[0], "Corp/bad-party");
        assert_eq!(linked_response.invalidtag[0], "Corp/bad-tag");
        assert_eq!(linked_response.extra["msgid"], "linked-msg");

        let school_body = serde_json::to_value(WorkExternalContactSchoolMessage {
            recv_scope: Some(0),
            to_external_userid: Vec::new(),
            to_parent_userid: vec!["parent".to_string()],
            to_student_userid: vec!["student".to_string()],
            to_party: vec!["party".to_string()],
            msgtype: "text".to_string(),
            agentid: 100001,
            text: Some(WorkTextMessage {
                content: "notice".to_string(),
            }),
            image: None,
            miniprogram_notice: None,
            extra: Value::Null,
        })
        .unwrap();
        assert_eq!(school_body["to_parent_userid"][0], "parent");
        assert_eq!(school_body["to_student_userid"][0], "student");
        assert!(school_body.get("to_external_userid").is_none());

        let school_text = serde_json::to_value(
            WorkExternalContactSchoolMessage::text(100001, "notice")
                .with_recv_scope(0)
                .with_parent_user("parent")
                .with_student_user("student")
                .with_party("party"),
        )
        .unwrap();
        assert_eq!(school_text["msgtype"], "text");
        assert_eq!(school_text["recv_scope"], 0);
        assert_eq!(school_text["text"]["content"], "notice");
        assert_eq!(school_text["to_parent_userid"][0], "parent");
        assert_eq!(
            WorkExternalContactSchoolMessage::image(100001, "image").msgtype_kind(),
            WorkMessageTypeKind::Image
        );

        let school_image = serde_json::to_value(WorkExternalContactSchoolMessage::image(
            100001,
            "image-media",
        ))
        .unwrap();
        assert_eq!(school_image["msgtype"], "image");
        assert_eq!(school_image["image"]["media_id"], "image-media");
        assert!(school_image.get("to_parent_userid").is_none());

        let mut school_notice =
            WorkExternalContactSchoolMessage::empty(100001, WorkMessageTypeKind::MiniProgramNotice);
        school_notice.miniprogram_notice = Some(WorkMiniProgramNoticeMessage {
            appid: "wx-app".to_string(),
            page: Some("pages/index".to_string()),
            title: "notice".to_string(),
            description: Some("desc".to_string()),
            emphasis_first_item: Some(false),
            content_item: vec![WorkMiniProgramNoticeItem {
                key: "time".to_string(),
                value: "10:00".to_string(),
            }],
        });
        let school_notice = serde_json::to_value(school_notice).unwrap();
        assert_eq!(school_notice["miniprogram_notice"]["appid"], "wx-app");
        assert_eq!(
            school_notice["miniprogram_notice"]["content_item"][0]["value"],
            "10:00"
        );

        let school_response: WorkExternalContactSchoolMessageSendResponse =
            serde_json::from_value(json!({
                "invalid_external_user": ["external"],
                "invalid_parent_userid": ["parent"],
                "invalid_student_userid": ["student"],
                "invalid_party": ["party"],
                "send_id": "school-send"
            }))
            .unwrap();
        assert_eq!(school_response.invalid_external_user[0], "external");
        assert_eq!(school_response.invalid_parent_userid[0], "parent");
        assert_eq!(school_response.invalid_student_userid[0], "student");
        assert_eq!(school_response.invalid_party[0], "party");
        assert_eq!(school_response.extra["send_id"], "school-send");
    }

    #[test]
    fn deserializes_external_contact_base_responses() {
        let list: ExternalContactListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "external_userid": ["wm-external"],
            "next_openid": "open-cursor"
        }))
        .unwrap();
        assert_eq!(list.external_userid[0], "wm-external");
        assert_eq!(list.extra["next_openid"], "open-cursor");

        let follow_users: ExternalContactFollowUserListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "follow_user": ["user-a", "user-b"],
            "department_scope": [1, 2]
        }))
        .unwrap();
        assert_eq!(follow_users.follow_user[1], "user-b");
        assert_eq!(follow_users.extra["department_scope"][0], 1);

        let detail: ExternalContactDetailResponse = serde_json::from_value(json!({
            "errcode": 0,
            "external_contact": {
                "external_userid": "wm-external",
                "name": "Customer",
                "position": "Buyer",
                "avatar": "https://example.com/avatar.png",
                "type": 2,
                "gender": 1,
                "unionid": "unionid",
                "corp_name": "Roze",
                "corp_full_name": "Roze Inc.",
                "wechat_channels": { "nickname": "Roze Shop" },
                "external_profile": {
                    "external_corp_name": "Roze",
                    "external_extra_profile": "profile-extra",
                    "external_attr": [{
                        "type": 1,
                        "name": "Website",
                        "attr_extra": "attr-extra",
                        "web": {
                            "url": "https://example.com",
                            "title": "Home",
                            "source": "official"
                        }
                    }]
                }
            },
            "follow_user": [{
                "userid": "user-a",
                "remark": "VIP",
                "description": "important",
                "createtime": 1_800_000_000,
                "tags": [{
                    "group_name": "Level",
                    "tag_name": "Gold",
                    "tag_id": "tag-id",
                    "type": 1
                }],
                "remark_corp_name": "Roze",
                "remark_mobiles": ["13800000000"],
                "add_way": 1,
                "state": "state",
                "oper_userid": "operator",
                "tag_source": "crm"
            }],
            "next_cursor": "cursor",
            "detail_extra": true
        }))
        .unwrap();
        let contact = detail.external_contact.expect("external_contact");
        assert_eq!(contact.external_userid.as_deref(), Some("wm-external"));
        assert_eq!(contact.contact_type, Some(2));
        assert_eq!(contact.contact_kind(), Some(ExternalContactKind::WorkUser));
        assert!(contact.is_work_user());
        assert!(!contact.is_wechat_user());
        assert_eq!(contact.gender_kind(), Some(WorkContactGender::Male));
        assert_eq!(contact.extra["wechat_channels"]["nickname"], "Roze Shop");
        assert_eq!(
            contact
                .external_profile
                .as_ref()
                .expect("external_profile")
                .extra["external_extra_profile"],
            "profile-extra"
        );
        let attr = &contact
            .external_profile
            .as_ref()
            .expect("external_profile")
            .external_attr[0];
        assert_eq!(attr.extra["attr_extra"], "attr-extra");
        assert_eq!(
            attr.attribute_kind(),
            Some(ExternalContactAttributeKind::Web)
        );
        assert!(!attr.is_text());
        assert!(attr.is_web());
        assert!(!attr.is_mini_program());
        assert_eq!(
            attr.web.as_ref().expect("web").url.as_deref(),
            Some("https://example.com")
        );
        assert_eq!(attr.web.as_ref().unwrap().extra["source"], "official");
        let text_attr = ExternalContactAttribute {
            attr_type: Some(0),
            name: None,
            text: None,
            web: None,
            miniprogram: None,
            extra: Value::Null,
        };
        assert_eq!(
            text_attr.attribute_kind(),
            Some(ExternalContactAttributeKind::Text)
        );
        assert!(text_attr.is_text());
        let unknown_contact = ExternalContactInfo {
            external_userid: None,
            name: None,
            position: None,
            avatar: None,
            contact_type: Some(99),
            gender: Some(99),
            unionid: None,
            corp_name: None,
            corp_full_name: None,
            external_profile: None,
            extra: Value::Null,
        };
        assert_eq!(
            unknown_contact.contact_kind(),
            Some(ExternalContactKind::Other)
        );
        assert_eq!(
            unknown_contact.gender_kind(),
            Some(WorkContactGender::Other)
        );
        let mini_program_attr = ExternalContactAttribute {
            attr_type: Some(2),
            name: None,
            text: None,
            web: None,
            miniprogram: None,
            extra: Value::Null,
        };
        assert_eq!(
            mini_program_attr.attribute_kind(),
            Some(ExternalContactAttributeKind::MiniProgram)
        );
        assert!(mini_program_attr.is_mini_program());
        let unknown_attr = ExternalContactAttribute {
            attr_type: Some(99),
            name: None,
            text: None,
            web: None,
            miniprogram: None,
            extra: Value::Null,
        };
        assert_eq!(
            unknown_attr.attribute_kind(),
            Some(ExternalContactAttributeKind::Other)
        );
        assert_eq!(
            detail.follow_user[0].tags[0].tag_name.as_deref(),
            Some("Gold")
        );
        assert_eq!(detail.follow_user[0].extra["tag_source"], "crm");
        assert_eq!(detail.next_cursor.as_deref(), Some("cursor"));
        assert_eq!(detail.extra["detail_extra"], true);

        let batch: ExternalContactBatchGetResponse = serde_json::from_value(json!({
            "errcode": 0,
            "external_contact_list": [{
                "external_contact": {
                    "external_userid": "wm-external",
                    "name": "Customer"
                },
                "follow_info": {
                    "userid": "user-a",
                    "remark": "VIP"
                },
                "batch_item_extra": "item-extra"
            }],
            "next_cursor": "next",
            "batch_extra": "batch-extra"
        }))
        .unwrap();
        assert_eq!(
            batch.external_contact_list[0]
                .external_contact
                .as_ref()
                .expect("external_contact")
                .name
                .as_deref(),
            Some("Customer")
        );
        assert_eq!(
            batch.external_contact_list[0]
                .follow_info
                .as_ref()
                .expect("follow_info")
                .userid
                .as_deref(),
            Some("user-a")
        );
        assert_eq!(
            batch.external_contact_list[0].extra["batch_item_extra"],
            "item-extra"
        );
        assert_eq!(batch.next_cursor.as_deref(), Some("next"));
        assert_eq!(batch.extra["batch_extra"], "batch-extra");
    }

    #[test]
    fn serializes_external_contact_operational_compatibility_contracts() {
        let to_service: WorkToServiceExternalUserIdResponse = serde_json::from_value(json!({
            "external_userid": "service-external",
            "migration_version": 2
        }))
        .unwrap();
        assert_eq!(
            to_service.external_userid.as_deref(),
            Some("service-external")
        );
        assert_eq!(to_service.extra["migration_version"], 2);

        let served_request = serde_json::to_value(ExternalContactServedListRequest {
            cursor: None,
            limit: Some(1000),
        })
        .unwrap();
        assert!(served_request.get("cursor").is_none());
        assert_eq!(served_request["limit"], 1000);

        let served: ExternalContactServedListResponse = serde_json::from_value(json!({
            "info_list": [
                {
                    "is_customer": true,
                    "tmp_openid": "tmp-customer",
                    "external_userid": "external",
                    "follow_userid": "user",
                    "add_time": 1_720_000_000,
                    "source": "direct"
                },
                {
                    "is_customer": false,
                    "tmp_openid": "tmp-contact",
                    "name": "External contact",
                    "chat_id": "chat",
                    "chat_name": "Community",
                    "follow_userid": "owner",
                    "add_time": 1_720_000_001
                }
            ],
            "next_cursor": "cursor",
            "snapshot_id": "served-list"
        }))
        .unwrap();
        assert_eq!(served.info_list[0].is_customer, Some(true));
        assert_eq!(
            served.info_list[0].external_userid.as_deref(),
            Some("external")
        );
        assert_eq!(served.info_list[0].extra["source"], "direct");
        assert_eq!(served.info_list[1].is_customer, Some(false));
        assert_eq!(served.info_list[1].chat_id.as_deref(), Some("chat"));
        assert_eq!(
            served.info_list[1].name.as_deref(),
            Some("External contact")
        );
        assert_eq!(served.next_cursor.as_deref(), Some("cursor"));
        assert_eq!(served.extra["snapshot_id"], "served-list");

        let result_request = serde_json::to_value(ExternalContactGroupMessageResultRequest {
            msgid: "message".to_string(),
            limit: 500,
            cursor: None,
        })
        .unwrap();
        assert_eq!(result_request["msgid"], "message");
        assert_eq!(result_request["limit"], 500);
        assert!(result_request.get("cursor").is_none());

        let result: ExternalContactGroupMessageResultResponse = serde_json::from_value(json!({
            "detail_list": [{
                "external_userid": "external",
                "userid": "user",
                "status": 1,
                "send_time": 1_720_000_000,
                "delivery_trace": "trace"
            }, {
                "chat_id": "chat",
                "userid": "user",
                "status": 3
            }],
            "next_cursor": "next",
            "result_version": 1
        }))
        .unwrap();
        assert!(result.detail_list[0].is_sent());
        assert!(!result.detail_list[0].is_failed());
        assert_eq!(result.detail_list[0].extra["delivery_trace"], "trace");
        assert_eq!(
            result.detail_list[1].status_kind(),
            Some(ExternalContactGroupMessageSendStatusKind::DuplicateDelivery)
        );
        assert!(result.detail_list[1].is_failed());
        assert_eq!(result.next_cursor.as_deref(), Some("next"));
        assert_eq!(result.extra["result_version"], 1);

        let transfer = serde_json::to_value(ExternalContactUnassignedTransferRequest {
            external_userid: "external".to_string(),
            handover_userid: "former".to_string(),
            takeover_userid: "successor".to_string(),
        })
        .unwrap();
        assert_eq!(transfer["external_userid"], "external");
        assert_eq!(transfer["handover_userid"], "former");
        assert_eq!(transfer["takeover_userid"], "successor");

        let openid: UserIdToOpenIdResponse = serde_json::from_value(json!({
            "openid": "school-parent-openid",
            "conversion_scope": "external"
        }))
        .unwrap();
        assert_eq!(openid.openid.as_deref(), Some("school-parent-openid"));
        assert_eq!(openid.extra["conversion_scope"], "external");

        let qr_code: WorkSchoolSubscribeQrCodeResponse = serde_json::from_value(json!({
            "qrcode_big": "https://example.com/qr/big",
            "qrcode_middle": "https://example.com/qr/middle",
            "qrcode_thumb": "https://example.com/qr/thumb",
            "expires_in": 86400
        }))
        .unwrap();
        assert_eq!(
            qr_code.qrcode_big.as_deref(),
            Some("https://example.com/qr/big")
        );
        assert_eq!(
            qr_code.qrcode_middle.as_deref(),
            Some("https://example.com/qr/middle")
        );
        assert_eq!(
            qr_code.qrcode_thumb.as_deref(),
            Some("https://example.com/qr/thumb")
        );
        assert_eq!(qr_code.extra["expires_in"], 86400);

        let subscribe_mode: WorkSchoolSubscribeModeResponse = serde_json::from_value(json!({
            "subscribe_mode": 2,
            "policy_version": 3
        }))
        .unwrap();
        assert_eq!(
            subscribe_mode.mode_kind(),
            Some(WorkSchoolSubscribeModeKind::ForbidQrCodeRegistration)
        );
        assert_eq!(subscribe_mode.extra["policy_version"], 3);
        assert_eq!(
            WorkSchoolSubscribeModeKind::from(9),
            WorkSchoolSubscribeModeKind::Other(9)
        );
    }

    #[test]
    fn serializes_group_robot_text_message() {
        let value = serde_json::to_value(Work::group_robot_text("hello", vec!["@all".to_string()]))
            .unwrap();

        assert_eq!(value["msgtype"], "text");
        assert_eq!(value["text"]["mentioned_list"][0], "@all");
        assert_eq!(
            Work::group_robot_text("hello", Vec::new()).msgtype_kind(),
            WorkMessageTypeKind::Text
        );

        let markdown = serde_json::to_value(Work::group_robot_markdown("**hello**")).unwrap();
        assert_eq!(markdown["msgtype"], "markdown");
        assert_eq!(markdown["markdown"]["content"], "**hello**");

        let markdown_v2 = serde_json::to_value(Work::group_robot_markdown_v2("# hello")).unwrap();
        assert_eq!(markdown_v2["msgtype"], "markdown_v2");
        assert_eq!(markdown_v2["markdown_v2"]["content"], "# hello");
        assert_eq!(
            Work::group_robot_markdown_v2("# hello").msgtype_kind(),
            WorkMessageTypeKind::MarkdownV2
        );

        let voice = serde_json::to_value(Work::group_robot_voice("voice-media")).unwrap();
        assert_eq!(voice["msgtype"], "voice");
        assert_eq!(voice["voice"]["media_id"], "voice-media");

        let file = GroupRobotMessage {
            msgtype: "file".to_string(),
            text: None,
            markdown: None,
            markdown_v2: None,
            image: None,
            news: None,
            file: Some(GroupRobotFileMessage {
                media_id: "media".to_string(),
            }),
            voice: None,
            template_card: None,
        };
        assert_eq!(file.msgtype_kind(), WorkMessageTypeKind::File);
        let file = serde_json::to_value(file).unwrap();
        assert_eq!(file["file"]["media_id"], "media");

        let image = serde_json::to_value(GroupRobotMessage {
            msgtype: "image".to_string(),
            text: None,
            markdown: None,
            markdown_v2: None,
            image: Some(GroupRobotImageMessage {
                base64: "aW1hZ2U=".to_string(),
                md5: "md5".to_string(),
            }),
            news: None,
            file: None,
            voice: None,
            template_card: None,
        })
        .unwrap();
        assert_eq!(image["image"]["base64"], "aW1hZ2U=");

        let news = serde_json::to_value(GroupRobotMessage {
            msgtype: "news".to_string(),
            text: None,
            markdown: None,
            markdown_v2: None,
            image: None,
            news: Some(GroupRobotNewsMessage {
                articles: vec![GroupRobotNewsArticle {
                    title: "title".to_string(),
                    description: Some("desc".to_string()),
                    url: "https://example.com".to_string(),
                    picurl: Some("https://example.com/a.png".to_string()),
                }],
            }),
            file: None,
            voice: None,
            template_card: None,
        })
        .unwrap();
        assert_eq!(news["news"]["articles"][0]["title"], "title");

        let mut template_card =
            GroupRobotTemplateCardMessage::new(WorkTemplateCardTypeKind::TextNotice);
        template_card.source = Some(WorkTemplateCardSource {
            icon_url: "https://example.com/icon.png".to_string(),
            desc: "Roze".to_string(),
            desc_color: None,
        });
        template_card.main_title = Some(WorkTemplateCardMainTitle {
            title: "hello".to_string(),
            desc: "world".to_string(),
        });
        assert_eq!(
            template_card.card_type_kind(),
            WorkTemplateCardTypeKind::TextNotice
        );
        let card_message = GroupRobotMessage {
            msgtype: "template_card".to_string(),
            text: None,
            markdown: None,
            markdown_v2: None,
            image: None,
            news: None,
            file: None,
            voice: None,
            template_card: Some(template_card),
        };
        assert_eq!(
            card_message.msgtype_kind(),
            WorkMessageTypeKind::TemplateCard
        );
        let card = serde_json::to_value(card_message).unwrap();
        assert_eq!(card["template_card"]["card_type"], "text_notice");
        assert_eq!(card["template_card"]["main_title"]["title"], "hello");
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
            "menu_version": 2,
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
        assert_eq!(menu.extra["menu_version"], 2);

        let created: WorkMenuCreateResponse = serde_json::from_value(json!({
            "errcode": 0,
            "button": [{ "name": "Today", "source": "api" }],
            "request_id": "menu-create"
        }))
        .unwrap();
        assert_eq!(created.button[0]["source"], "api");
        assert_eq!(created.extra["request_id"], "menu-create");
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
            conclusions: Some(ContactWayConclusions {
                text: Some(ContactWayConclusionText {
                    content: "hello".to_string(),
                }),
                image: None,
                link: None,
                miniprogram: None,
            }),
        })
        .unwrap();

        assert_eq!(value["type"], 1);
        assert_eq!(value["user"][0], "user");
        assert_eq!(value["conclusions"]["text"]["content"], "hello");
    }

    #[test]
    fn serializes_external_contact_way_depth_requests() {
        let list = serde_json::to_value(ContactWayListRequest {
            start_time: 1_720_000_000,
            end_time: 1_720_086_400,
            cursor: None,
            limit: 100,
        })
        .unwrap();
        assert_eq!(list["start_time"], 1_720_000_000);
        assert!(list.get("cursor").is_none());

        let update = serde_json::to_value(ContactWayUpdateRequest {
            config_id: "config".to_string(),
            remark: Some("remark".to_string()),
            skip_verify: Some(true),
            style: Some(1),
            state: None,
            user: Some(vec!["user".to_string()]),
            party: None,
            expires_in: None,
            chat_expires_in: Some(3600),
            unionid: None,
            conclusions: Some(ContactWayConclusions {
                text: Some(ContactWayConclusionText {
                    content: "hello".to_string(),
                }),
                image: None,
                link: None,
                miniprogram: None,
            }),
        })
        .unwrap();
        assert_eq!(update["config_id"], "config");
        assert_eq!(update["skip_verify"], true);
        assert_eq!(update["chat_expires_in"], 3600);
        assert_eq!(update["conclusions"]["text"]["content"], "hello");
        assert!(update.get("state").is_none());

        let response: ContactWayListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "contact_way": [{ "config_id": "config", "owner_userid": "owner" }],
            "next_cursor": "cursor",
            "total": 1
        }))
        .unwrap();
        assert_eq!(response.contact_way[0].config_id.as_deref(), Some("config"));
        assert_eq!(response.contact_way[0].extra["owner_userid"], "owner");
        assert_eq!(response.next_cursor.as_deref(), Some("cursor"));
        assert_eq!(response.extra["total"], 1);

        let added: ContactWayAddResponse = serde_json::from_value(json!({
            "config_id": "config",
            "qr_code": "https://example.com/qr",
            "request_id": "contact-way-add"
        }))
        .unwrap();
        assert_eq!(added.config_id.as_deref(), Some("config"));
        assert_eq!(added.extra["request_id"], "contact-way-add");

        let detail: ContactWayGetResponse = serde_json::from_value(json!({
            "contact_way": {
                "config_id": "config",
                "type": 1,
                "scene": 2,
                "style": 1,
                "remark": "remark",
                "skip_verify": true,
                "qr_code": "https://example.com/qr",
                "user": ["user"],
                "party": [1],
                "is_temp": false,
                "channel": "qrcode",
                "conclusions": {
                    "link": {
                        "title": "title",
                        "picurl": "https://example.com/pic.png",
                        "desc": "desc",
                        "url": "https://example.com"
                    }
                }
            },
            "request_id": "contact-way-get"
        }))
        .unwrap();
        assert_eq!(detail.extra["request_id"], "contact-way-get");
        let contact_way = detail.contact_way.unwrap();
        assert_eq!(contact_way.kind, Some(1));
        assert_eq!(contact_way.user[0], "user");
        assert_eq!(contact_way.extra["channel"], "qrcode");
        assert_eq!(
            contact_way.conclusions.unwrap().link.unwrap().url,
            "https://example.com"
        );
    }

    #[test]
    fn serializes_external_contact_remark_and_tag_requests() {
        let remark = serde_json::to_value(ExternalContactRemarkRequest {
            userid: "user".to_string(),
            external_userid: "external".to_string(),
            remark: Some("name".to_string()),
            description: Some("description".to_string()),
            remark_company: None,
            remark_mobiles: Some(vec!["13800138000".to_string()]),
            remark_pic_mediaid: None,
        })
        .unwrap();
        assert_eq!(remark["userid"], "user");
        assert_eq!(remark["external_userid"], "external");
        assert_eq!(remark["remark_mobiles"][0], "13800138000");
        assert!(remark.get("remark_company").is_none());

        let add = serde_json::to_value(CorpTagAddRequest {
            group_id: None,
            group_name: "level".to_string(),
            order: Some(1),
            tag: vec![CorpTagAddItem {
                name: "vip".to_string(),
                order: None,
            }],
            agentid: Some(100001),
        })
        .unwrap();
        assert_eq!(add["group_name"], "level");
        assert_eq!(add["tag"][0]["name"], "vip");
        assert_eq!(add["agentid"], 100001);
        assert!(add.get("group_id").is_none());

        let corp_tags: CorpTagListResponse = serde_json::from_value(json!({
            "tag_group": [{
                "group_id": "level",
                "group_name": "level",
                "order": 1,
                "tag": [{ "id": "vip", "name": "vip", "order": 1, "tag_source": "crm" }],
                "group_source": "crm"
            }],
            "tag_total": 1
        }))
        .unwrap();
        assert_eq!(corp_tags.tag_group[0].group_name.as_deref(), Some("level"));
        assert_eq!(corp_tags.tag_group[0].tag[0].id.as_deref(), Some("vip"));
        assert_eq!(corp_tags.tag_group[0].tag[0].extra["tag_source"], "crm");
        assert_eq!(corp_tags.tag_group[0].extra["group_source"], "crm");
        assert_eq!(corp_tags.extra["tag_total"], 1);

        let corp_tag_created: CorpTagAddResponse = serde_json::from_value(json!({
            "tag_group": {
                "group_id": "level",
                "tag": [{ "id": "vip", "name": "vip" }],
                "group_source": "api"
            },
            "request_id": "tag-create-1"
        }))
        .unwrap();
        assert_eq!(corp_tag_created.extra["request_id"], "tag-create-1");
        assert_eq!(
            corp_tag_created.tag_group.as_ref().unwrap().tag[0]
                .name
                .as_deref(),
            Some("vip")
        );
        assert_eq!(
            corp_tag_created.tag_group.unwrap().extra["group_source"],
            "api"
        );

        let mark = serde_json::to_value(ExternalContactMarkTagRequest {
            userid: "user".to_string(),
            external_userid: "external".to_string(),
            add_tag: vec!["tag-add".to_string()],
            remove_tag: Vec::new(),
        })
        .unwrap();
        assert_eq!(mark["add_tag"][0], "tag-add");
        assert!(mark.get("remove_tag").is_none());

        let strategy_list = serde_json::to_value(ExternalContactStrategyTagListRequest {
            strategy_id: 1,
            tag_id: vec!["tag".to_string()],
            group_id: Vec::new(),
        })
        .unwrap();
        assert_eq!(strategy_list["strategy_id"], 1);
        assert_eq!(strategy_list["tag_id"][0], "tag");
        assert!(strategy_list.get("group_id").is_none());

        let strategy_add = serde_json::to_value(ExternalContactStrategyTagAddRequest {
            strategy_id: 1,
            group_id: None,
            group_name: "strategy".to_string(),
            order: 1,
            tag: vec![ExternalContactStrategyTagAddItem {
                name: "gold".to_string(),
                order: 1,
            }],
        })
        .unwrap();
        assert_eq!(strategy_add["group_name"], "strategy");
        assert_eq!(strategy_add["tag"][0]["name"], "gold");

        let strategy_edit = serde_json::to_value(ExternalContactStrategyTagEditRequest {
            id: "tag".to_string(),
            name: "platinum".to_string(),
            order: 2,
        })
        .unwrap();
        assert_eq!(strategy_edit["id"], "tag");

        let strategy_delete = serde_json::to_value(ExternalContactStrategyTagDeleteRequest {
            tag_id: Vec::new(),
            group_id: vec!["group".to_string()],
        })
        .unwrap();
        assert_eq!(strategy_delete["group_id"][0], "group");
        assert!(strategy_delete.get("tag_id").is_none());

        let strategy_tags: ExternalContactStrategyTagListResponse = serde_json::from_value(json!({
            "tag_group": [{
                "group_id": "group",
                "group_name": "strategy",
                "strategy_id": 1,
                "tag": [{ "id": "tag", "name": "gold", "order": 1, "tag_source": "strategy" }],
                "group_source": "strategy"
            }],
            "tag_total": 1
        }))
        .unwrap();
        assert_eq!(
            strategy_tags.tag_group[0].group_id.as_deref(),
            Some("group")
        );
        assert_eq!(strategy_tags.tag_group[0].strategy_id, Some(1));
        assert_eq!(
            strategy_tags.tag_group[0].tag[0].name.as_deref(),
            Some("gold")
        );
        assert_eq!(
            strategy_tags.tag_group[0].tag[0].extra["tag_source"],
            "strategy"
        );
        assert_eq!(strategy_tags.tag_group[0].extra["group_source"], "strategy");
        assert_eq!(strategy_tags.extra["tag_total"], 1);

        let strategy_created: ExternalContactStrategyTagAddResponse =
            serde_json::from_value(json!({
                "tag_group": { "group_id": "group", "tag": [{ "id": "tag" }], "group_source": "created" },
                "request_id": "strategy-tag-create"
            }))
            .unwrap();
        assert_eq!(strategy_created.extra["request_id"], "strategy-tag-create");
        let strategy_created = strategy_created.tag_group.unwrap();
        assert_eq!(strategy_created.group_id.as_deref(), Some("group"));
        assert_eq!(strategy_created.tag[0].id.as_deref(), Some("tag"));
        assert_eq!(strategy_created.extra["group_source"], "created");
    }

    #[test]
    fn serializes_external_group_chat_requests_and_responses() {
        let list = serde_json::to_value(ExternalGroupChatListRequest {
            status_filter: Some(0),
            owner_filter: Some(ExternalContactOwnerFilter::user("user")),
            cursor: None,
            limit: 50,
        })
        .unwrap();
        assert_eq!(list["status_filter"], 0);
        assert_eq!(list["owner_filter"]["userid_list"][0], "user");
        assert!(list.get("cursor").is_none());

        let chats: ExternalGroupChatListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "group_chat_list": [{ "chat_id": "chat", "status": 0, "owner": "owner" }],
            "next_cursor": "cursor",
            "group_total": 1
        }))
        .unwrap();
        assert_eq!(chats.group_chat_list[0].chat_id.as_deref(), Some("chat"));
        assert_eq!(chats.group_chat_list[0].status, Some(0));
        assert_eq!(
            chats.group_chat_list[0].status_kind(),
            Some(ExternalGroupChatStatusKind::Normal)
        );
        assert_eq!(chats.group_chat_list[0].extra["owner"], "owner");
        assert_eq!(chats.next_cursor.as_deref(), Some("cursor"));
        assert_eq!(chats.extra["group_total"], 1);
        assert_eq!(
            ExternalGroupChatStatusKind::from(1),
            ExternalGroupChatStatusKind::OwnerResignedPendingTransfer
        );
        assert_eq!(
            ExternalGroupChatStatusKind::from(99),
            ExternalGroupChatStatusKind::Other(99)
        );

        let detail: ExternalGroupChatGetResponse = serde_json::from_value(json!({
            "errcode": 0,
            "group_chat": {
                "chat_id": "chat",
                "name": "group",
                "owner": "owner",
                "create_time": 1_720_000_000,
                "notice": "notice",
                "member_list": [{
                    "userid": "member",
                    "type": 1,
                    "join_time": 1_720_000_001,
                    "join_scene": 2,
                    "state": "state",
                    "invitor": { "userid": "invitor", "invitor_role": "owner" },
                    "group_nickname": "nick",
                    "name": "name",
                    "unionid": "union",
                    "member_role": "customer"
                }],
                "admin_list": [{ "userid": "admin", "admin_type": "primary" }],
                "member_version": "v1",
                "customer_count": 10
            },
            "detail_source": "sync"
        }))
        .unwrap();
        let group_chat = detail.group_chat.unwrap();
        assert_eq!(group_chat.name.as_deref(), Some("group"));
        assert_eq!(group_chat.extra["customer_count"], 10);
        assert_eq!(group_chat.member_list[0].member_type, Some(1));
        assert_eq!(
            group_chat.member_list[0].member_kind(),
            Some(ExternalGroupChatMemberKind::WorkUser)
        );
        assert!(group_chat.member_list[0].is_work_user());
        assert!(!group_chat.member_list[0].is_external_contact());
        assert_eq!(group_chat.member_list[0].extra["member_role"], "customer");
        assert_eq!(
            group_chat.member_list[0]
                .invitor
                .as_ref()
                .and_then(|invitor| invitor.userid.as_deref()),
            Some("invitor")
        );
        assert_eq!(
            group_chat.member_list[0].invitor.as_ref().unwrap().extra["invitor_role"],
            "owner"
        );
        assert_eq!(group_chat.admin_list[0].userid.as_deref(), Some("admin"));
        assert_eq!(group_chat.admin_list[0].extra["admin_type"], "primary");
        let external_member = ExternalGroupChatMember {
            userid: None,
            member_type: Some(2),
            join_time: None,
            join_scene: None,
            state: None,
            invitor: None,
            group_nickname: None,
            name: None,
            unionid: None,
            extra: Value::Null,
        };
        assert_eq!(
            external_member.member_kind(),
            Some(ExternalGroupChatMemberKind::ExternalContact)
        );
        assert!(external_member.is_external_contact());
        let unknown_member = ExternalGroupChatMember {
            member_type: Some(99),
            ..external_member
        };
        assert_eq!(
            unknown_member.member_kind(),
            Some(ExternalGroupChatMemberKind::Other)
        );

        let transfer: ExternalGroupChatTransferResponse = serde_json::from_value(json!({
            "errcode": 0,
            "failed_chat_list": [{
                "chat_id": "bad",
                "errcode": 40003,
                "errmsg": "bad owner",
                "failed_reason": "owner left"
            }],
            "request_id": "req-1"
        }))
        .unwrap();
        assert_eq!(transfer.failed_chat_list[0].chat_id.as_deref(), Some("bad"));
        assert_eq!(transfer.failed_chat_list[0].errcode, Some(40003));
        assert_eq!(
            transfer.failed_chat_list[0].extra["failed_reason"],
            "owner left"
        );
        assert_eq!(transfer.extra["request_id"], "req-1");

        let open_gid: ExternalGroupChatOpenGidToChatIdResponse =
            serde_json::from_value(json!({ "chat_id": "chat", "source": "appshare" })).unwrap();
        assert_eq!(open_gid.chat_id.as_deref(), Some("chat"));
        assert_eq!(open_gid.extra["source"], "appshare");

        let join = serde_json::to_value(ExternalGroupChatJoinWayRequest {
            scene: 2,
            remark: "remark".to_string(),
            auto_create_room: 1,
            room_base_name: "room".to_string(),
            room_base_id: 100,
            chat_id_list: vec!["chat".to_string()],
            state: Some("state".to_string()),
        })
        .unwrap();
        assert_eq!(join["scene"], 2);
        assert_eq!(join["chat_id_list"][0], "chat");

        let join_update = serde_json::to_value(ExternalGroupChatJoinWayUpdateRequest {
            config_id: "config".to_string(),
            scene: 2,
            remark: "new".to_string(),
            auto_create_room: 0,
            room_base_name: "room".to_string(),
            room_base_id: 101,
            chat_id_list: Vec::new(),
            state: None,
        })
        .unwrap();
        assert_eq!(join_update["config_id"], "config");
        assert!(join_update.get("chat_id_list").is_none());

        let join_add: ExternalGroupChatJoinWayAddResponse = serde_json::from_value(json!({
            "config_id": "config",
            "request_id": "join-way-add"
        }))
        .unwrap();
        assert_eq!(join_add.config_id.as_deref(), Some("config"));
        assert_eq!(join_add.extra["request_id"], "join-way-add");

        let join_detail: ExternalGroupChatJoinWayResponse = serde_json::from_value(json!({
            "join_way": {
                "config_id": "config",
                "qr_code": "https://example.com/qr",
                "scene": 2,
                "remark": "remark",
                "auto_create_room": 1,
                "room_base_name": "room",
                "room_base_id": 100,
                "chat_id_list": ["chat"],
                "state": "state",
                "future_field": "kept"
            },
            "request_id": "join-way-get"
        }))
        .unwrap();
        assert_eq!(join_detail.extra["request_id"], "join-way-get");
        let join_way = join_detail.join_way.unwrap();
        assert_eq!(join_way.config_id.as_deref(), Some("config"));
        assert_eq!(join_way.qr_code.as_deref(), Some("https://example.com/qr"));
        assert_eq!(join_way.scene, Some(2));
        assert_eq!(join_way.chat_id_list[0], "chat");
        assert_eq!(join_way.state.as_deref(), Some("state"));
        assert_eq!(join_way.extra["future_field"], "kept");

        let converted: WorkExternalUserIdConvertResponse =
            serde_json::from_value(json!({ "external_userid": "new-external" })).unwrap();
        assert_eq!(converted.external_userid.as_deref(), Some("new-external"));
    }

    #[test]
    fn serializes_external_group_welcome_templates() {
        let template = ExternalGroupWelcomeTemplateRequest {
            text: Some(ExternalContactMessageText::new("welcome")),
            image: Some(ExternalContactMessageImage {
                media_id: Some("image-media".to_string()),
                pic_url: None,
                extra: Value::Null,
            }),
            link: Some(ExternalContactMessageLink {
                title: Some("docs".to_string()),
                picurl: Some("https://example.com/a.png".to_string()),
                desc: Some("desc".to_string()),
                url: Some("https://example.com".to_string()),
                extra: Value::Null,
            }),
            miniprogram: Some(ExternalContactMessageMiniProgram {
                title: Some("mini".to_string()),
                pic_media_id: Some("pic-media".to_string()),
                appid: Some("wx-app".to_string()),
                page: Some("pages/index".to_string()),
                extra: Value::Null,
            }),
            file: Some(ExternalContactMessageFile {
                media_id: Some("file-media".to_string()),
                extra: Value::Null,
            }),
            video: Some(ExternalContactMessageVideo {
                media_id: Some("video-media".to_string()),
                extra: Value::Null,
            }),
            agentid: 100001,
            notify: 1,
        };
        let value = serde_json::to_value(&template).unwrap();
        assert_eq!(value["text"]["content"], "welcome");
        assert_eq!(value["image"]["media_id"], "image-media");
        assert_eq!(value["link"]["title"], "docs");
        assert_eq!(value["miniprogram"]["appid"], "wx-app");
        assert_eq!(value["file"]["media_id"], "file-media");
        assert_eq!(value["video"]["media_id"], "video-media");

        let update = serde_json::to_value(ExternalGroupWelcomeTemplateUpdateRequest {
            template_id: "template".to_string(),
            template,
        })
        .unwrap();
        assert_eq!(update["template_id"], "template");
        assert_eq!(update["agentid"], 100001);

        let added: ExternalGroupWelcomeTemplateAddResponse = serde_json::from_value(json!({
            "template_id": "template",
            "request_id": "welcome-add"
        }))
        .unwrap();
        assert_eq!(added.template_id.as_deref(), Some("template"));
        assert_eq!(added.extra["request_id"], "welcome-add");

        let detail: ExternalGroupWelcomeTemplateResponse = serde_json::from_value(json!({
            "text": { "content": "welcome" },
            "image": { "media_id": "media" },
            "link": { "title": "docs", "url": "https://example.com" },
            "miniprogram": { "title": "mini", "appid": "wx-app", "page": "pages/index" },
            "file": { "media_id": "file-media" },
            "video": { "media_id": "video-media" },
            "template_source": "api"
        }))
        .unwrap();
        assert_eq!(detail.text.unwrap().content.as_deref(), Some("welcome"));
        assert_eq!(detail.image.unwrap().media_id.as_deref(), Some("media"));
        assert_eq!(detail.link.unwrap().title.as_deref(), Some("docs"));
        assert_eq!(detail.miniprogram.unwrap().appid.as_deref(), Some("wx-app"));
        assert_eq!(detail.file.unwrap().media_id.as_deref(), Some("file-media"));
        assert_eq!(
            detail.video.unwrap().media_id.as_deref(),
            Some("video-media")
        );
        assert_eq!(detail.extra["template_source"], "api");
    }

    #[test]
    fn serializes_customer_acquisition_link_requests_and_responses() {
        let list = serde_json::to_value(CustomerAcquisitionLinkListRequest {
            cursor: None,
            limit: 50,
        })
        .unwrap();
        assert_eq!(list["limit"], 50);
        assert!(list.get("cursor").is_none());

        let create = serde_json::to_value(CustomerAcquisitionLinkRequest {
            link_name: "summer".to_string(),
            range: CustomerAcquisitionRange {
                user_list: vec!["user".to_string()],
                department_list: Vec::new(),
                extra: Value::Null,
            },
            skip_verify: true,
            priority_option: Some(CustomerAcquisitionPriorityOption {
                priority_type: 1,
                priority_userid_list: vec!["priority".to_string()],
                extra: Value::Null,
            }),
        })
        .unwrap();
        assert_eq!(create["link_name"], "summer");
        assert_eq!(create["range"]["user_list"][0], "user");
        assert_eq!(create["skip_verify"], true);
        assert_eq!(
            create["priority_option"]["priority_userid_list"][0],
            "priority"
        );
        assert!(create["range"].get("department_list").is_none());

        let update = serde_json::to_value(CustomerAcquisitionLinkUpdateRequest {
            link_id: "link".to_string(),
            link_name: "autumn".to_string(),
            range: CustomerAcquisitionRange {
                user_list: Vec::new(),
                department_list: vec!["2".to_string()],
                extra: Value::Null,
            },
            skip_verify: false,
            priority_option: None,
        })
        .unwrap();
        assert_eq!(update["link_id"], "link");
        assert_eq!(update["range"]["department_list"][0], "2");
        assert!(update.get("priority_option").is_none());

        let links: CustomerAcquisitionLinkListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "link_id_list": ["link"],
            "next_cursor": "cursor",
            "link_total": 1
        }))
        .unwrap();
        assert_eq!(links.link_id_list[0], "link");
        assert_eq!(links.next_cursor.as_deref(), Some("cursor"));
        assert_eq!(links.extra["link_total"], 1);

        let created: CustomerAcquisitionLinkCreateResponse = serde_json::from_value(json!({
            "errcode": 0,
            "request_id": "create-link",
            "link": {
                "link_id": "link",
                "link_name": "summer",
                "url": "https://example.com",
                "skip_verify": true,
                "range": { "user_list": ["user"], "range_extra": "kept" },
                "priority_option": {
                    "priority_type": 1,
                    "priority_userid_list": ["priority"],
                    "priority_extra": "kept"
                },
                "create_time": 1_720_000_000,
                "update_time": 1_720_000_001,
                "future_field": "kept"
            }
        }))
        .unwrap();
        assert_eq!(created.extra["request_id"], "create-link");
        let created = created.link.unwrap();
        assert_eq!(created.link_id.as_deref(), Some("link"));
        assert_eq!(created.link_name.as_deref(), Some("summer"));
        assert_eq!(created.url.as_deref(), Some("https://example.com"));
        assert_eq!(created.skip_verify, Some(true));
        assert_eq!(
            created
                .range
                .as_ref()
                .and_then(|range| range.user_list.first())
                .map(String::as_str),
            Some("user")
        );
        assert_eq!(created.range.as_ref().unwrap().extra["range_extra"], "kept");
        assert_eq!(
            created
                .priority_option
                .as_ref()
                .and_then(|option| option.priority_userid_list.first())
                .map(String::as_str),
            Some("priority")
        );
        assert_eq!(
            created.priority_option.as_ref().unwrap().extra["priority_extra"],
            "kept"
        );
        assert_eq!(created.create_time, Some(1_720_000_000));
        assert_eq!(created.extra["future_field"], "kept");

        let got: CustomerAcquisitionLinkResponse = serde_json::from_value(json!({
            "errcode": 0,
            "request_id": "get-link",
            "link": { "link_id": "link", "url": "https://example.com" }
        }))
        .unwrap();
        assert_eq!(got.extra["request_id"], "get-link");
        assert_eq!(
            got.link.and_then(|link| link.url).as_deref(),
            Some("https://example.com")
        );
    }

    #[test]
    fn serializes_customer_acquisition_monitoring_requests_and_responses() {
        let customers = serde_json::to_value(CustomerAcquisitionCustomerListRequest {
            link_id: "link".to_string(),
            limit: 1000,
            cursor: None,
        })
        .unwrap();
        assert_eq!(customers["link_id"], "link");
        assert_eq!(customers["limit"], 1000);
        assert!(customers.get("cursor").is_none());

        let statistic = serde_json::to_value(CustomerAcquisitionStatisticRequest {
            link_id: "link".to_string(),
            start_time: 1_720_000_000,
            end_time: 1_720_086_400,
        })
        .unwrap();
        assert_eq!(statistic["link_id"], "link");
        assert_eq!(statistic["start_time"], 1_720_000_000_i64);
        assert_eq!(statistic["end_time"], 1_720_086_400_i64);

        let chat_request = serde_json::to_value(CustomerAcquisitionChatInfoRequest {
            chat_key: "single-use-chat-key".to_string(),
        })
        .unwrap();
        assert_eq!(chat_request, json!({ "chat_key": "single-use-chat-key" }));

        let quota: CustomerAcquisitionQuotaResponse = serde_json::from_value(json!({
            "errcode": 0,
            "total": 1000,
            "balance": 500,
            "quota_list": [
                {
                    "expire_date": 1_730_000_000,
                    "balance": 300,
                    "batch_id": "later"
                },
                {
                    "expire_date": 1_720_000_000,
                    "balance": 200,
                    "batch_id": "next"
                }
            ],
            "quota_policy": "purchased"
        }))
        .unwrap();
        assert_eq!(quota.total, Some(1000));
        assert_eq!(quota.balance, Some(500));
        assert!(!quota.is_exhausted());
        let next_quota = quota.next_expiring_quota().expect("next quota");
        assert_eq!(next_quota.expire_date, Some(1_720_000_000));
        assert_eq!(next_quota.balance, Some(200));
        assert_eq!(next_quota.extra["batch_id"], "next");
        assert_eq!(quota.extra["quota_policy"], "purchased");

        let exhausted: CustomerAcquisitionQuotaResponse =
            serde_json::from_value(json!({ "balance": 0 })).unwrap();
        assert!(exhausted.is_exhausted());
        assert!(exhausted.next_expiring_quota().is_none());

        let customer_list: CustomerAcquisitionCustomerListResponse =
            serde_json::from_value(json!({
                "errcode": 0,
                "customer_list": [{
                    "external_userid": "external",
                    "userid": "user",
                    "chat_status": 1,
                    "state": "campaign-a",
                    "acquired_at": 1_720_000_000
                }],
                "next_cursor": "cursor",
                "link_id": "link"
            }))
            .unwrap();
        let customer = &customer_list.customer_list[0];
        assert_eq!(customer.external_userid.as_deref(), Some("external"));
        assert_eq!(
            customer.chat_status_kind(),
            Some(CustomerAcquisitionChatStatusKind::Messaged)
        );
        assert_eq!(customer.extra["acquired_at"], 1_720_000_000_i64);
        assert_eq!(customer_list.next_cursor.as_deref(), Some("cursor"));
        assert_eq!(customer_list.extra["link_id"], "link");
        assert_eq!(
            CustomerAcquisitionChatStatusKind::from(9),
            CustomerAcquisitionChatStatusKind::Other(9)
        );

        let statistic: CustomerAcquisitionStatisticResponse = serde_json::from_value(json!({
            "click_link_customer_cnt": 80,
            "new_customer_cnt": 25,
            "stat_date": "2026-07-18"
        }))
        .unwrap();
        assert_eq!(statistic.click_link_customer_cnt, Some(80));
        assert_eq!(statistic.new_customer_cnt, Some(25));
        assert_eq!(statistic.extra["stat_date"], "2026-07-18");

        let chat: CustomerAcquisitionChatInfoResponse = serde_json::from_value(json!({
            "userid": "user",
            "external_userid": "external",
            "chat_info": {
                "recv_msg_cnt": 4,
                "link_id": "link",
                "state": "campaign-a",
                "latest_msg_time": 1_720_000_000
            },
            "request_id": "chat-info"
        }))
        .unwrap();
        assert_eq!(chat.userid.as_deref(), Some("user"));
        assert_eq!(chat.external_userid.as_deref(), Some("external"));
        let chat_info = chat.chat_info.expect("chat info");
        assert_eq!(chat_info.recv_msg_cnt, Some(4));
        assert_eq!(chat_info.link_id.as_deref(), Some("link"));
        assert_eq!(chat_info.state.as_deref(), Some("campaign-a"));
        assert_eq!(chat_info.extra["latest_msg_time"], 1_720_000_000_i64);
        assert_eq!(chat.extra["request_id"], "chat-info");
    }

    #[test]
    fn serializes_external_contact_intercept_rule_lifecycle() {
        let add = serde_json::to_value(ExternalContactInterceptRuleAddRequest {
            rule_name: "No private contact".to_string(),
            word_list: vec!["private-account".to_string()],
            semantics_list: vec![1, 2],
            intercept_type: 1,
            applicable_range: ExternalContactInterceptRuleRange {
                user_list: vec!["user".to_string()],
                department_list: vec![2],
                extra: Value::Null,
            },
        })
        .unwrap();
        assert_eq!(add["rule_name"], "No private contact");
        assert_eq!(add["word_list"][0], "private-account");
        assert_eq!(add["semantics_list"][1], 2);
        assert_eq!(add["applicable_range"]["user_list"][0], "user");
        assert_eq!(add["applicable_range"]["department_list"][0], 2);

        let update = serde_json::to_value(ExternalContactInterceptRuleUpdateRequest {
            rule_id: "rule".to_string(),
            rule_name: None,
            word_list: vec!["new-word".to_string()],
            extra_rule: Some(ExternalContactInterceptRuleExtraRule {
                semantics_list: vec![3],
                extra: Value::Null,
            }),
            intercept_type: Some(2),
            add_applicable_range: Some(ExternalContactInterceptRuleRange {
                user_list: vec!["new-user".to_string()],
                department_list: Vec::new(),
                extra: Value::Null,
            }),
            remove_applicable_range: Some(ExternalContactInterceptRuleRange {
                user_list: Vec::new(),
                department_list: vec![2],
                extra: Value::Null,
            }),
        })
        .unwrap();
        assert_eq!(update["rule_id"], "rule");
        assert!(update.get("rule_name").is_none());
        assert_eq!(update["extra_rule"]["semantics_list"][0], 3);
        assert_eq!(update["add_applicable_range"]["user_list"][0], "new-user");
        assert_eq!(update["remove_applicable_range"]["department_list"][0], 2);

        let added: ExternalContactInterceptRuleAddResponse = serde_json::from_value(json!({
            "rule_id": "rule",
            "request_id": "rule-add"
        }))
        .unwrap();
        assert_eq!(added.rule_id.as_deref(), Some("rule"));
        assert_eq!(added.extra["request_id"], "rule-add");

        let list: ExternalContactInterceptRuleListResponse = serde_json::from_value(json!({
            "rule_list": [{
                "rule_id": "rule",
                "rule_name": "No private contact",
                "create_time": 1_720_000_000,
                "owner": "admin"
            }],
            "rule_total": 1
        }))
        .unwrap();
        assert_eq!(list.rule_list[0].rule_id.as_deref(), Some("rule"));
        assert_eq!(
            list.rule_list[0].rule_name.as_deref(),
            Some("No private contact")
        );
        assert_eq!(list.rule_list[0].extra["owner"], "admin");
        assert_eq!(list.extra["rule_total"], 1);

        let detail: ExternalContactInterceptRuleResponse = serde_json::from_value(json!({
            "rule": {
                "rule_id": "rule",
                "rule_name": "No private contact",
                "word_list": ["private-account"],
                "semantics_list": [1, 2, 3, 9],
                "intercept_type": 1,
                "applicable_range": {
                    "user_list": ["user"],
                    "department_list": [2],
                    "range_version": 2
                },
                "create_time": 1_720_000_000,
                "updated_by": "admin"
            },
            "request_id": "rule-detail"
        }))
        .unwrap();
        assert_eq!(detail.extra["request_id"], "rule-detail");
        let rule = detail.rule.expect("rule");
        assert_eq!(
            rule.intercept_type_kind(),
            Some(ExternalContactInterceptTypeKind::WarnAndBlock)
        );
        assert_eq!(
            rule.semantic_kinds().collect::<Vec<_>>(),
            vec![
                ExternalContactInterceptSemanticKind::Mobile,
                ExternalContactInterceptSemanticKind::Email,
                ExternalContactInterceptSemanticKind::RedPacket,
                ExternalContactInterceptSemanticKind::Other(9)
            ]
        );
        assert_eq!(
            rule.applicable_range.as_ref().unwrap().extra["range_version"],
            2
        );
        assert_eq!(rule.extra["updated_by"], "admin");
        assert_eq!(
            ExternalContactInterceptTypeKind::from(9),
            ExternalContactInterceptTypeKind::Other(9)
        );
    }

    #[test]
    fn serializes_external_contact_product_album_lifecycle() {
        let add = serde_json::to_value(ExternalContactProductAlbumAddRequest {
            description: "Roze subscription".to_string(),
            price: 19900,
            product_sn: Some("ROZE-001".to_string()),
            attachments: vec![ExternalContactProductAttachment::image("media")],
        })
        .unwrap();
        assert_eq!(add["description"], "Roze subscription");
        assert_eq!(add["price"], 19900);
        assert_eq!(add["product_sn"], "ROZE-001");
        assert_eq!(add["attachments"][0]["type"], "image");
        assert_eq!(add["attachments"][0]["image"]["media_id"], "media");

        let update = serde_json::to_value(ExternalContactProductAlbumUpdateRequest {
            product_id: "product".to_string(),
            description: Some("Roze annual subscription".to_string()),
            price: Some(29900),
            product_sn: None,
            attachments: Vec::new(),
        })
        .unwrap();
        assert_eq!(update["product_id"], "product");
        assert_eq!(update["price"], 29900);
        assert!(update.get("product_sn").is_none());
        assert!(update.get("attachments").is_none());

        let list_request = serde_json::to_value(ExternalContactProductAlbumListRequest {
            limit: 50,
            cursor: None,
        })
        .unwrap();
        assert_eq!(list_request["limit"], 50);
        assert!(list_request.get("cursor").is_none());

        let added: ExternalContactProductAlbumAddResponse = serde_json::from_value(json!({
            "product_id": "product",
            "request_id": "product-add"
        }))
        .unwrap();
        assert_eq!(added.product_id.as_deref(), Some("product"));
        assert_eq!(added.extra["request_id"], "product-add");

        let list: ExternalContactProductAlbumListResponse = serde_json::from_value(json!({
            "product_list": [{
                "product_id": "product",
                "product_sn": "ROZE-001",
                "description": "Roze subscription",
                "price": 19900,
                "catalog": "software"
            }],
            "next_cursor": "cursor",
            "product_total": 1
        }))
        .unwrap();
        assert_eq!(list.product_list[0].product_id.as_deref(), Some("product"));
        assert_eq!(list.product_list[0].price, Some(19900));
        assert_eq!(list.product_list[0].extra["catalog"], "software");
        assert_eq!(list.next_cursor.as_deref(), Some("cursor"));
        assert_eq!(list.extra["product_total"], 1);

        let detail: ExternalContactProductAlbumResponse = serde_json::from_value(json!({
            "product": {
                "product_id": "product",
                "product_sn": "ROZE-001",
                "description": "Roze subscription",
                "price": 19900,
                "create_time": 1_720_000_000,
                "attachments": [{
                    "type": "image",
                    "image": {
                        "media_id": "media",
                        "image_hash": "sha256"
                    },
                    "attachment_version": 2
                }],
                "updated_at": 1_720_000_001
            },
            "request_id": "product-detail"
        }))
        .unwrap();
        assert_eq!(detail.extra["request_id"], "product-detail");
        let product = detail.product.expect("product");
        assert_eq!(product.create_time, Some(1_720_000_000));
        assert_eq!(product.attachments[0].attachment_type, "image");
        assert_eq!(product.attachments[0].image.media_id, "media");
        assert_eq!(product.attachments[0].image.extra["image_hash"], "sha256");
        assert_eq!(product.attachments[0].extra["attachment_version"], 2);
        assert_eq!(product.extra["updated_at"], 1_720_000_001_i64);
    }

    #[test]
    fn serializes_external_contact_message_template_requests() {
        let request = ExternalContactMessageTemplateRequest {
            chat_type: "single".to_string(),
            external_userid: vec!["external".to_string()],
            chat_id_list: Vec::new(),
            tag_filter: Some(ExternalContactMessageTagFilter::from_tag_groups([["tag"]])),
            sender: Some("sender".to_string()),
            allow_select: true,
            text: Some(ExternalContactMessageText::new("hello")),
            attachments: vec![ExternalContactMessageAttachment::link(
                ExternalContactMessageLink {
                    title: Some("title".to_string()),
                    picurl: None,
                    desc: None,
                    url: Some("https://example.com".to_string()),
                    extra: Value::Null,
                },
            )],
        };
        request.validate().unwrap();
        assert_eq!(
            request.chat_type_kind(),
            ExternalContactMessageChatTypeKind::Single
        );
        assert_eq!(request.tag_filter.as_ref().unwrap().tag_count(), 1);
        let template = serde_json::to_value(request).unwrap();
        assert_eq!(template["chat_type"], "single");
        assert_eq!(template["external_userid"][0], "external");
        assert_eq!(
            template["tag_filter"]["group_list"][0]["tag_list"][0],
            "tag"
        );
        assert_eq!(template["attachments"][0]["msgtype"], "link");
        assert_eq!(
            template["attachments"][0]["link"],
            json!({ "title": "title", "url": "https://example.com" })
        );
        assert!(template.get("chat_id_list").is_none());
        assert!(template["attachments"][0].get("image").is_none());

        let invalid_group = ExternalContactMessageTemplateRequest {
            chat_type: "group".to_string(),
            external_userid: vec!["external".to_string()],
            chat_id_list: Vec::new(),
            tag_filter: None,
            sender: None,
            allow_select: false,
            text: Some(ExternalContactMessageText::new("hello")),
            attachments: Vec::new(),
        };
        assert!(invalid_group.validate().is_err());

        let invalid_filter = ExternalContactMessageTagFilter::from_tag_groups([[""]]);
        assert!(invalid_filter.validate().is_err());
        let oversized_filter = ExternalContactMessageTagFilter::from_tag_groups([
            (0..101).map(|index| format!("tag-{index}"))
        ]);
        assert!(oversized_filter.validate().is_err());

        let valid_group = ExternalContactMessageTemplateRequest {
            chat_type: "group".to_string(),
            external_userid: Vec::new(),
            chat_id_list: vec!["chat".to_string()],
            tag_filter: None,
            sender: Some("sender".to_string()),
            allow_select: false,
            text: None,
            attachments: vec![ExternalContactMessageAttachment::image("media")],
        };
        valid_group.validate().unwrap();
        assert_eq!(
            valid_group.chat_type_kind(),
            ExternalContactMessageChatTypeKind::Group
        );

        let page: ExternalContactGroupMessageListResponse =
            serde_json::from_value(json!({ "next_cursor": " cursor " })).unwrap();
        assert!(page.has_more());
        assert_eq!(page.next_cursor(), Some(" cursor "));
        let last_page: ExternalContactGroupMessageListResponse =
            serde_json::from_value(json!({ "next_cursor": "" })).unwrap();
        assert!(!last_page.has_more());

        let list = serde_json::to_value(ExternalContactGroupMessageListRequest {
            chat_type: "group".to_string(),
            start_time: 1,
            end_time: 2,
            creator: None,
            filter_type: Some(1),
            limit: 10,
            cursor: None,
        })
        .unwrap();
        assert_eq!(list["filter_type"], 1);
        assert!(list.get("creator").is_none());

        let welcome = serde_json::to_value(ExternalContactWelcomeMessageRequest {
            welcome_code: "welcome".to_string(),
            text: Some(ExternalContactMessageText::new("hi")),
            attachments: vec![ExternalContactMessageAttachment::image("image-media")],
        })
        .unwrap();
        assert_eq!(welcome["welcome_code"], "welcome");
        assert_eq!(welcome["text"]["content"], "hi");
        assert_eq!(welcome["attachments"][0]["msgtype"], "image");
        assert_eq!(
            welcome["attachments"][0]["image"]["media_id"],
            "image-media"
        );

        let empty_welcome = serde_json::to_value(ExternalContactWelcomeMessageRequest {
            welcome_code: "welcome".to_string(),
            text: None,
            attachments: Vec::new(),
        })
        .unwrap();
        assert!(empty_welcome.get("text").is_none());
        assert!(empty_welcome.get("attachments").is_none());
    }

    #[test]
    fn deserializes_external_contact_message_template_responses() {
        let added: ExternalContactMessageTemplateResponse = serde_json::from_value(json!({
            "errcode": 0,
            "msgid": "msg",
            "fail_list": ["bad"],
            "request_id": "req-template"
        }))
        .unwrap();
        assert_eq!(added.msgid.as_deref(), Some("msg"));
        assert_eq!(added.fail_list[0], "bad");
        assert_eq!(added.extra["request_id"], "req-template");

        let messages: ExternalContactGroupMessageListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "group_msg_list": [{
                "msgid": "msg",
                "creator": "creator",
                "create_time": 1_720_000_000,
                "create_type": 1,
                "text": { "content": "hello", "locale": "zh_CN" },
                "attachments": [{
                    "msgtype": "link",
                    "attachment_id": "attachment-1",
                    "link": {
                        "title": "title",
                        "picurl": "https://example.com/pic.png",
                        "desc": "desc",
                        "url": "https://example.com",
                        "source": "crm"
                    }
                }],
                "visible_range": "all"
            }],
            "next_cursor": "cursor",
            "total": 1
        }))
        .unwrap();
        assert_eq!(messages.group_msg_list[0].msgid.as_deref(), Some("msg"));
        assert_eq!(
            messages.group_msg_list[0]
                .text
                .as_ref()
                .and_then(|text| text.content.as_deref()),
            Some("hello")
        );
        assert_eq!(
            messages.group_msg_list[0].text.as_ref().unwrap().extra["locale"],
            "zh_CN"
        );
        assert_eq!(messages.group_msg_list[0].extra["visible_range"], "all");
        assert_eq!(
            messages.group_msg_list[0].attachments[0].extra["attachment_id"],
            "attachment-1"
        );
        assert_eq!(
            messages.group_msg_list[0].attachments[0]
                .link
                .as_ref()
                .and_then(|link| link.url.as_deref()),
            Some("https://example.com")
        );
        assert_eq!(
            messages.group_msg_list[0].attachments[0]
                .link
                .as_ref()
                .unwrap()
                .extra["source"],
            "crm"
        );
        assert_eq!(messages.next_cursor.as_deref(), Some("cursor"));
        assert_eq!(messages.extra["total"], 1);

        let tasks: ExternalContactGroupMessageTaskResponse = serde_json::from_value(json!({
            "task_list": [{
                "userid": "user",
                "status": 2,
                "send_time": 1_720_000_001,
                "task_remark": "manual"
            }],
            "next_cursor": "task-cursor",
            "task_total": 1
        }))
        .unwrap();
        assert_eq!(tasks.task_list[0].userid.as_deref(), Some("user"));
        assert_eq!(
            tasks.task_list[0].status_kind(),
            Some(ExternalContactGroupMessageTaskStatusKind::Sent)
        );
        assert!(tasks.task_list[0].is_sent());
        assert_eq!(tasks.task_list[0].send_time, Some(1_720_000_001));
        assert_eq!(tasks.task_list[0].extra["task_remark"], "manual");
        assert_eq!(tasks.next_cursor.as_deref(), Some("task-cursor"));
        assert_eq!(tasks.extra["task_total"], 1);
        assert_eq!(
            ExternalContactGroupMessageTaskStatusKind::from_code(0),
            ExternalContactGroupMessageTaskStatusKind::Unsent
        );
        assert_eq!(
            ExternalContactGroupMessageTaskStatusKind::from_code(1),
            ExternalContactGroupMessageTaskStatusKind::Other
        );

        let send_result: ExternalContactGroupMessageSendResultResponse =
            serde_json::from_value(json!({
                "send_list": [{
                    "external_userid": "external",
                    "chat_id": "chat",
                    "userid": "user",
                    "status": 1,
                    "send_time": 1_720_000_002,
                    "fail_reason": "none"
                }],
                "next_cursor": "send-cursor",
                "send_total": 1
            }))
            .unwrap();
        assert_eq!(
            send_result.send_list[0].external_userid.as_deref(),
            Some("external")
        );
        assert_eq!(send_result.send_list[0].chat_id.as_deref(), Some("chat"));
        assert_eq!(
            send_result.send_list[0].status_kind(),
            Some(ExternalContactGroupMessageSendStatusKind::Sent)
        );
        assert!(send_result.send_list[0].is_sent());
        assert!(!send_result.send_list[0].is_failed());
        assert_eq!(send_result.send_list[0].extra["fail_reason"], "none");
        assert_eq!(send_result.extra["send_total"], 1);
        assert_eq!(
            ExternalContactGroupMessageSendStatusKind::from_code(0),
            ExternalContactGroupMessageSendStatusKind::Unsent
        );
        assert_eq!(
            ExternalContactGroupMessageSendStatusKind::from_code(2),
            ExternalContactGroupMessageSendStatusKind::CustomerNotFriend
        );
        assert_eq!(
            ExternalContactGroupMessageSendStatusKind::from_code(3),
            ExternalContactGroupMessageSendStatusKind::DuplicateDelivery
        );
        assert_eq!(
            ExternalContactGroupMessageSendStatusKind::from_code(4),
            ExternalContactGroupMessageSendStatusKind::ReceiveLimitExceeded
        );
        assert!(ExternalContactGroupMessageSendStatusKind::CustomerNotFriend.is_failure());
        assert!(ExternalContactGroupMessageSendStatusKind::DuplicateDelivery.is_failure());
        assert!(ExternalContactGroupMessageSendStatusKind::ReceiveLimitExceeded.is_failure());
        assert!(ExternalContactGroupMessageSendStatusKind::Other.is_failure());
    }

    #[test]
    fn serializes_external_customer_transfer_requests_and_responses() {
        let transfer = serde_json::to_value(ExternalCustomerTransferRequest {
            handover_userid: "old".to_string(),
            takeover_userid: "new".to_string(),
            external_userid: vec!["external".to_string()],
            transfer_success_msg: Some("done".to_string()),
        })
        .unwrap();
        assert_eq!(transfer["handover_userid"], "old");
        assert_eq!(transfer["takeover_userid"], "new");
        assert_eq!(transfer["external_userid"][0], "external");
        assert_eq!(transfer["transfer_success_msg"], "done");

        let resigned = serde_json::to_value(ResignedExternalCustomerTransferRequest {
            handover_userid: "old".to_string(),
            takeover_userid: "new".to_string(),
            external_userid: vec!["external".to_string()],
        })
        .unwrap();
        assert_eq!(resigned["external_userid"][0], "external");

        let query = serde_json::to_value(ExternalCustomerTransferResultRequest {
            handover_userid: "old".to_string(),
            takeover_userid: "new".to_string(),
            cursor: None,
        })
        .unwrap();
        assert!(query.get("cursor").is_none());

        let unassigned = serde_json::to_value(ExternalContactUnassignedListRequest {
            page_id: 1,
            cursor: Some("cursor".to_string()),
            page_size: 100,
        })
        .unwrap();
        assert_eq!(unassigned["page_id"], 1);
        assert_eq!(unassigned["cursor"], "cursor");

        let response: ExternalCustomerTransferResponse = serde_json::from_value(json!({
            "errcode": 0,
            "customer": [{
                "external_userid": "external",
                "status": 1,
                "takeover_time": 100,
                "handover_userid": "old",
                "takeover_userid": "new",
                "transfer_remark": "manual"
            }],
            "next_cursor": "cursor",
            "job_id": "transfer-job-1"
        }))
        .unwrap();
        assert_eq!(
            response.customer[0].external_userid.as_deref(),
            Some("external")
        );
        assert_eq!(response.customer[0].status, Some(1));
        assert_eq!(
            response.customer[0].status_kind(),
            Some(ExternalCustomerTransferStatusKind::Completed)
        );
        assert!(response.customer[0].is_completed());
        assert!(response.customer[0]
            .status_kind()
            .expect("status")
            .is_terminal());
        assert_eq!(response.customer[0].takeover_time, Some(100));
        assert_eq!(response.customer[0].extra["transfer_remark"], "manual");
        assert_eq!(response.next_cursor.as_deref(), Some("cursor"));
        assert_eq!(response.extra["job_id"], "transfer-job-1");
        assert_eq!(
            ExternalCustomerTransferStatusKind::from_code(2),
            ExternalCustomerTransferStatusKind::Pending
        );
        assert!(!ExternalCustomerTransferStatusKind::Pending.is_terminal());
        assert_eq!(
            ExternalCustomerTransferStatusKind::from_code(3),
            ExternalCustomerTransferStatusKind::CustomerRefused
        );
        assert_eq!(
            ExternalCustomerTransferStatusKind::from_code(4),
            ExternalCustomerTransferStatusKind::TakeoverLimitExceeded
        );
        assert_eq!(
            ExternalCustomerTransferStatusKind::from_code(5),
            ExternalCustomerTransferStatusKind::NoRecord
        );
        assert!(ExternalCustomerTransferStatusKind::CustomerRefused.is_failure());
        assert!(ExternalCustomerTransferStatusKind::TakeoverLimitExceeded.is_failure());
        assert!(ExternalCustomerTransferStatusKind::NoRecord.is_failure());
        assert!(ExternalCustomerTransferStatusKind::Other.is_failure());

        let unassigned_response: ExternalContactUnassignedListResponse =
            serde_json::from_value(json!({
                "info": [{
                    "handover_userid": "old",
                    "external_userid": "external",
                    "dimission_time": 100,
                    "handover_department": 1
                }],
                "is_last": false,
                "next_cursor": "next",
                "total": 1
            }))
            .unwrap();
        assert_eq!(
            unassigned_response.info[0].handover_userid.as_deref(),
            Some("old")
        );
        assert_eq!(
            unassigned_response.info[0].external_userid.as_deref(),
            Some("external")
        );
        assert_eq!(unassigned_response.info[0].dimission_time, Some(100));
        assert_eq!(unassigned_response.info[0].extra["handover_department"], 1);
        assert_eq!(unassigned_response.is_last, Some(false));
        assert_eq!(unassigned_response.extra["total"], 1);
    }

    #[test]
    fn serializes_external_contact_moment_requests_and_responses() {
        let list = serde_json::to_value(ExternalContactMomentListRequest {
            start_time: 1,
            end_time: 2,
            creator: Some("creator".to_string()),
            filter_type: Some(1),
            cursor: None,
            limit: 100,
        })
        .unwrap();
        assert_eq!(list["creator"], "creator");
        assert_eq!(list["filter_type"], 1);
        assert!(list.get("cursor").is_none());

        let task = serde_json::to_value(ExternalContactMomentTaskRequest {
            text: Some(ExternalContactMessageText::new("hello")),
            attachments: vec![ExternalContactMessageAttachment::image("media")],
            visible_range: Some(
                ExternalContactMomentVisibleRange::sender_users(["user"])
                    .with_external_contact_tags(["tag"]),
            ),
        })
        .unwrap();
        assert_eq!(task["text"]["content"], "hello");
        assert_eq!(task["attachments"][0]["image"]["media_id"], "media");
        assert_eq!(task["visible_range"]["sender_list"]["user_list"][0], "user");

        let moments: ExternalContactMomentListResponse = serde_json::from_value(json!({
            "moment_list": [{
                "moment_id": "moment",
                "creator": "creator",
                "create_time": 100,
                "text": { "content": "hello" },
                "attachments": [{ "msgtype": "image", "image": { "media_id": "media" } }],
                "publish_scope": "customer"
            }],
            "next_cursor": "cursor",
            "total": 1
        }))
        .unwrap();
        assert_eq!(moments.moment_list[0].moment_id.as_deref(), Some("moment"));
        assert_eq!(moments.moment_list[0].creator.as_deref(), Some("creator"));
        assert_eq!(
            moments.moment_list[0]
                .text
                .as_ref()
                .unwrap()
                .content
                .as_deref(),
            Some("hello")
        );
        assert_eq!(moments.moment_list[0].extra["publish_scope"], "customer");
        assert_eq!(moments.next_cursor.as_deref(), Some("cursor"));
        assert_eq!(moments.extra["total"], 1);

        let tasks: ExternalContactMomentTaskResponse = serde_json::from_value(json!({
            "task_list": [{ "userid": "user", "publish_status": 2, "fail_reason": "none" }],
            "next_cursor": "next",
            "task_total": 1
        }))
        .unwrap();
        assert_eq!(tasks.task_list[0].userid.as_deref(), Some("user"));
        assert_eq!(tasks.task_list[0].publish_status, Some(2));
        assert_eq!(
            tasks.task_list[0].publish_status_kind(),
            Some(ExternalContactMomentPublishStatusKind::Other)
        );
        assert!(!tasks.task_list[0].is_published());
        assert_eq!(tasks.task_list[0].extra["fail_reason"], "none");
        assert_eq!(tasks.extra["task_total"], 1);
        assert_eq!(
            ExternalContactMomentPublishStatusKind::from_code(0),
            ExternalContactMomentPublishStatusKind::Unpublished
        );
        assert_eq!(
            ExternalContactMomentPublishStatusKind::from_code(1),
            ExternalContactMomentPublishStatusKind::Published
        );
        assert!(ExternalContactMomentPublishStatusKind::Published.is_published());

        let customers: ExternalContactMomentCustomerListResponse = serde_json::from_value(json!({
            "customer_list": [{
                "external_userid": "external",
                "publish_status": 1,
                "view_time": 101
            }],
            "customer_total": 1
        }))
        .unwrap();
        assert_eq!(
            customers.customer_list[0].external_userid.as_deref(),
            Some("external")
        );
        assert_eq!(customers.customer_list[0].publish_status, Some(1));
        assert_eq!(
            customers.customer_list[0].publish_status_kind(),
            Some(ExternalContactMomentPublishStatusKind::Published)
        );
        assert!(customers.customer_list[0].is_published());
        assert_eq!(customers.customer_list[0].extra["view_time"], 101);
        assert_eq!(customers.extra["customer_total"], 1);

        let comments: ExternalContactMomentCommentResponse = serde_json::from_value(json!({
            "comment_list": [{
                "userid": "user",
                "comment_id": "comment",
                "content": "nice",
                "source": "mobile"
            }],
            "like_list": [{
                "external_userid": "external",
                "create_time": 100,
                "reaction": "like"
            }],
            "interaction_total": 2
        }))
        .unwrap();
        assert_eq!(comments.comment_list[0].userid.as_deref(), Some("user"));
        assert_eq!(
            comments.comment_list[0].comment_id.as_deref(),
            Some("comment")
        );
        assert_eq!(
            comments.like_list[0].external_userid.as_deref(),
            Some("external")
        );
        assert_eq!(comments.comment_list[0].extra["source"], "mobile");
        assert_eq!(comments.like_list[0].extra["reaction"], "like");
        assert_eq!(comments.extra["interaction_total"], 2);

        let created: ExternalContactMomentTaskCreateResponse =
            serde_json::from_value(json!({ "jobid": "job", "trace_id": "trace" })).unwrap();
        assert_eq!(created.jobid.as_deref(), Some("job"));
        assert_eq!(created.extra["trace_id"], "trace");

        let result: ExternalContactMomentTaskResultResponse = serde_json::from_value(json!({
            "status": 2,
            "type": "add_moment_task",
            "result": { "moment_id": "moment", "invalid_reason": "none" },
            "result_source": "async"
        }))
        .unwrap();
        assert_eq!(result.status, Some(2));
        assert_eq!(result.result_type.as_deref(), Some("add_moment_task"));
        let result_payload = result.result.as_ref().unwrap();
        assert_eq!(result_payload.moment_id.as_deref(), Some("moment"));
        assert_eq!(result_payload.extra["invalid_reason"], "none");
        assert_eq!(result.extra["result_source"], "async");

        let strategy_range = serde_json::to_value(ExternalContactMomentStrategyRangeRequest {
            strategy_id: 100,
            cursor: Some("cursor".to_string()),
            limit: 50,
        })
        .unwrap();
        assert_eq!(strategy_range["strategy_id"], 100);
        assert_eq!(strategy_range["cursor"], "cursor");

        let create_strategy = serde_json::to_value(ExternalContactMomentStrategyCreateRequest {
            parent_id: 0,
            strategy_name: "vip".to_string(),
            admin_list: vec!["admin".to_string()],
        })
        .unwrap();
        assert_eq!(create_strategy["parent_id"], 0);
        assert_eq!(create_strategy["strategy_name"], "vip");
        assert_eq!(create_strategy["admin_list"][0], "admin");

        let edit_strategy = serde_json::to_value(ExternalContactMomentStrategyEditRequest {
            strategy_id: 100,
            strategy_name: "vip2".to_string(),
            admin_list: Vec::new(),
        })
        .unwrap();
        assert_eq!(edit_strategy["strategy_id"], 100);
        assert_eq!(edit_strategy["strategy_name"], "vip2");
        assert!(edit_strategy.get("admin_list").is_none());

        let strategies: ExternalContactMomentStrategyListResponse = serde_json::from_value(json!({
            "strategy": [{
                "strategy_id": 100,
                "strategy_name": "vip",
                "parent_id": 0,
                "admin_list": ["admin"],
                "create_time": 1_720_000_000,
                "strategy_source": "moment"
            }],
            "next_cursor": "next",
            "strategy_total": 1
        }))
        .unwrap();
        assert_eq!(strategies.strategy[0].strategy_name.as_deref(), Some("vip"));
        assert_eq!(strategies.strategy[0].admin_list[0], "admin");
        assert_eq!(strategies.strategy[0].extra["strategy_source"], "moment");
        assert_eq!(strategies.next_cursor.as_deref(), Some("next"));
        assert_eq!(strategies.extra["strategy_total"], 1);

        let range: ExternalContactMomentStrategyRangeResponse = serde_json::from_value(json!({
            "range": { "user_list": ["user"], "party_list": [2], "tag_list": ["tag"], "range_source": "manual" },
            "next_cursor": "next",
            "range_total": 1
        }))
        .unwrap();
        assert_eq!(range.extra["range_total"], 1);
        let range_info = range.range.unwrap();
        assert_eq!(range_info.user_list[0], "user");
        assert_eq!(range_info.party_list[0], 2);
        assert_eq!(range_info.extra["range_source"], "manual");

        let created_strategy: ExternalContactMomentStrategyCreateResponse = serde_json::from_value(
            json!({ "strategy_id": 100, "request_id": "moment-strategy-create" }),
        )
        .unwrap();
        assert_eq!(created_strategy.strategy_id, Some(100));
        assert_eq!(
            created_strategy.extra["request_id"],
            "moment-strategy-create"
        );
    }

    #[test]
    fn serializes_external_contact_statistics_requests_and_responses() {
        let behavior = serde_json::to_value(ExternalContactUserBehaviorDataRequest {
            userid: vec!["user".to_string()],
            partyid: Vec::new(),
            start_time: 1,
            end_time: 2,
        })
        .unwrap();
        assert_eq!(behavior["userid"][0], "user");
        assert!(behavior.get("partyid").is_none());

        let statistic = serde_json::to_value(ExternalGroupChatStatisticRequest {
            day_begin_time: 1,
            day_end_time: 2,
            owner_filter: Some(ExternalContactOwnerFilter::user("owner")),
            order_by: Some(1),
            order_asc: Some(0),
            offset: Some(0),
            limit: Some(50),
        })
        .unwrap();
        assert_eq!(statistic["owner_filter"]["userid_list"][0], "owner");
        assert_eq!(statistic["order_asc"], 0);

        let by_day = serde_json::to_value(ExternalGroupChatStatisticByDayRequest {
            day_begin_time: 1,
            day_end_time: 2,
            owner_filter: None,
        })
        .unwrap();
        assert!(by_day.get("owner_filter").is_none());

        let behavior_response: ExternalContactUserBehaviorDataResponse =
            serde_json::from_value(json!({
                "behavior_data": [{
                    "userid": "user",
                    "stat_time": 1,
                    "new_apply_cnt": 1,
                    "reply_percentage": 99.5,
                    "avg_reply_percentage": 88.8
                }],
                "total": 1
            }))
            .unwrap();
        assert_eq!(
            behavior_response.behavior_data[0].userid.as_deref(),
            Some("user")
        );
        assert_eq!(behavior_response.behavior_data[0].new_apply_cnt, Some(1));
        assert_eq!(
            behavior_response.behavior_data[0].reply_percentage,
            Some(99.5)
        );
        assert_eq!(
            behavior_response.behavior_data[0].extra["avg_reply_percentage"],
            88.8
        );
        assert_eq!(behavior_response.extra["total"], 1);

        let statistic_response: ExternalGroupChatStatisticResponse =
            serde_json::from_value(json!({
                "total": 1,
                "next_offset": 50,
                "items": [{
                    "owner": "owner",
                    "data": {
                        "new_chat_cnt": 1,
                        "msg_total": 2,
                        "active_member_rate": 0.75
                    },
                    "owner_name": "Owner"
                }],
                "report_id": "report-1"
            }))
            .unwrap();
        assert_eq!(statistic_response.total, Some(1));
        assert_eq!(statistic_response.items[0].owner.as_deref(), Some("owner"));
        assert_eq!(statistic_response.items[0].extra["owner_name"], "Owner");
        assert_eq!(
            statistic_response.items[0].data.as_ref().unwrap().msg_total,
            Some(2)
        );
        assert_eq!(
            statistic_response.items[0].data.as_ref().unwrap().extra["active_member_rate"],
            0.75
        );
        assert_eq!(statistic_response.extra["report_id"], "report-1");
    }

    #[test]
    fn serializes_external_contact_customer_strategy_requests_and_responses() {
        let privilege = ExternalContactCustomerStrategyPrivilege {
            view_customer_list: true,
            view_customer_data: true,
            view_room_list: true,
            contact_me: true,
            join_room: true,
            share_customer: true,
            oper_resign_customer: true,
            send_customer_msg: true,
            edit_welcome_msg: true,
            view_behavior_data: true,
            view_room_data: true,
            send_group_msg: true,
            room_deduplication: true,
            rapid_reply: true,
            onjob_customer_transfer: true,
            edit_anti_spam_rule: true,
            export_customer_list: true,
            export_customer_data: true,
            export_customer_group_list: true,
            manage_customer_tag: true,
        };

        let create = serde_json::to_value(ExternalContactCustomerStrategyCreateRequest {
            parent_id: 1,
            strategy_name: "strategy".to_string(),
            admin_list: vec!["admin".to_string()],
            privilege: privilege.clone(),
            range: vec![ExternalContactCustomerStrategyRange {
                kind: 1,
                partyid: Some(2),
                userid: None,
                extra: Value::Null,
            }],
        })
        .unwrap();
        assert_eq!(create["parent_id"], 1);
        assert_eq!(create["privilege"]["view_customer_list"], true);
        assert_eq!(create["range"][0]["type"], 1);
        assert!(create["range"][0].get("userid").is_none());

        let edit = serde_json::to_value(ExternalContactCustomerStrategyEditRequest {
            strategy_id: 2,
            strategy_name: "strategy-new".to_string(),
            admin_list: Vec::new(),
            privilege,
            range_add: vec![ExternalContactCustomerStrategyRange {
                kind: 2,
                partyid: None,
                userid: Some("user".to_string()),
                extra: Value::Null,
            }],
            range_del: Vec::new(),
        })
        .unwrap();
        assert_eq!(edit["strategy_id"], 2);
        assert!(edit.get("admin_list").is_none());
        assert_eq!(edit["range_add"][0]["userid"], "user");

        let list: ExternalContactCustomerStrategyListResponse = serde_json::from_value(json!({
            "momentStrategy": [{
                "strategy_id": 1,
                "strategy_name": "strategy",
                "parent_id": 0,
                "admin_list": ["admin"],
                "create_time": 1_720_000_000,
                "strategy_source": "crm"
            }],
            "total": 1
        }))
        .unwrap();
        assert_eq!(list.strategy[0].strategy_id, Some(1));
        assert_eq!(list.strategy[0].strategy_name.as_deref(), Some("strategy"));
        assert_eq!(list.strategy[0].admin_list[0], "admin");
        assert_eq!(list.strategy[0].extra["strategy_source"], "crm");
        assert_eq!(list.extra["total"], 1);

        let detail: ExternalContactCustomerStrategyResponse = serde_json::from_value(json!({
            "momentStrategy": {
                "strategy_id": 1,
                "strategy_name": "strategy",
                "privilege": {
                    "view_customer_list": true,
                    "view_customer_data": true,
                    "view_room_list": true,
                    "contact_me": true,
                    "join_room": true,
                    "share_customer": true,
                    "oper_resign_customer": true,
                    "send_customer_msg": true,
                    "edit_welcome_msg": true,
                    "view_behavior_data": true,
                    "view_room_data": true,
                    "send_group_msg": true,
                    "room_deduplication": true,
                    "rapid_reply": true,
                    "onjob_customer_transfer": true,
                    "edit_anti_spam_rule": true,
                    "export_customer_list": true,
                    "export_customer_data": true,
                    "export_customer_group_list": true,
                    "manage_customer_tag": true
                },
                "strategy_source": "detail"
            },
            "request_id": "strategy-detail-1"
        }))
        .unwrap();
        assert_eq!(detail.extra["request_id"], "strategy-detail-1");
        let strategy = detail.strategy.unwrap();
        assert_eq!(strategy.strategy_name.as_deref(), Some("strategy"));
        assert!(strategy.privilege.unwrap().view_customer_list);
        assert_eq!(strategy.extra["strategy_source"], "detail");

        let range: ExternalContactCustomerStrategyRangeResponse = serde_json::from_value(json!({
            "range": [{ "type": 2, "userid": "user", "range_source": "manual" }],
            "range_total": 1
        }))
        .unwrap();
        assert_eq!(range.range[0].kind, 2);
        assert_eq!(range.range[0].userid.as_deref(), Some("user"));
        assert_eq!(range.range[0].extra["range_source"], "manual");
        assert_eq!(range.extra["range_total"], 1);

        let created: ExternalContactCustomerStrategyCreateResponse =
            serde_json::from_value(json!({ "strategy_id": 3, "request_id": "strategy-create-1" }))
                .unwrap();
        assert_eq!(created.strategy_id, Some(3));
        assert_eq!(created.extra["request_id"], "strategy-create-1");
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

        let departments: WorkDepartmentListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "trace_id": "department-list",
            "department": [{
                "id": 1,
                "name": "Engineering",
                "name_en": "Engineering",
                "parentid": 0,
                "order": 10,
                "department_leader": ["leader"],
                "department_type": "core"
            }]
        }))
        .unwrap();
        assert_eq!(departments.extra["trace_id"], "department-list");
        assert_eq!(departments.department[0].id, Some(1));
        assert_eq!(
            departments.department[0].name.as_deref(),
            Some("Engineering")
        );
        assert_eq!(departments.department[0].department_leader[0], "leader");
        assert_eq!(departments.department[0].extra["department_type"], "core");

        let simple: WorkDepartmentSimpleListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "trace_id": "department-simple",
            "department_id": [{
                "id": 1,
                "name": "Engineering",
                "parentid": 0,
                "order": 10,
                "member_count": 12
            }]
        }))
        .unwrap();
        assert_eq!(simple.extra["trace_id"], "department-simple");
        assert_eq!(simple.department_id[0].id, Some(1));
        assert_eq!(simple.department_id[0].parentid, Some(0));
        assert_eq!(simple.department_id[0].extra["member_count"], 12);

        let detail: WorkDepartmentDetailResponse = serde_json::from_value(json!({
            "errcode": 0,
            "id": 1,
            "name": "Engineering",
            "parentid": 0,
            "department_leader": ["leader"],
            "department_type": "core"
        }))
        .unwrap();
        assert_eq!(detail.department.id, Some(1));
        assert_eq!(detail.department.department_leader[0], "leader");
        assert_eq!(detail.department.extra["department_type"], "core");
    }

    #[test]
    fn deserializes_work_agent_responses() {
        let list: WorkAgentListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "trace_id": "agent-list",
            "agentlist": [{
                "agentid": 100001,
                "name": "App",
                "square_logo_url": "https://example.com/logo.png",
                "round_logo_url": "https://example.com/round.png",
                "visible_scope": "all"
            }]
        }))
        .unwrap();
        assert_eq!(list.extra["trace_id"], "agent-list");
        assert_eq!(list.agentlist[0].agentid, Some(100001));
        assert_eq!(list.agentlist[0].name.as_deref(), Some("App"));
        assert_eq!(list.agentlist[0].extra["visible_scope"], "all");

        let detail: WorkAgentDetailResponse = serde_json::from_value(json!({
            "errcode": 0,
            "agentid": 100001,
            "name": "App",
            "description": "Work app",
            "allow_userinfos": { "user": [{ "userid": "user" }] },
            "allow_partys": { "partyid": [1] },
            "allow_tags": { "tagid": [2] },
            "close": 0,
            "redirect_domain": "example.com",
            "report_location_flag": 1,
            "isreportenter": 0,
            "home_url": "https://example.com/home",
            "customized_publish_status": 1,
            "beta_feature_flag": true
        }))
        .unwrap();
        assert_eq!(detail.agentid, Some(100001));
        assert_eq!(
            detail.allow_userinfos.as_ref().unwrap().user[0]
                .userid
                .as_deref(),
            Some("user")
        );
        assert_eq!(detail.allow_partys.as_ref().unwrap().partyid[0], 1);
        assert_eq!(detail.allow_tags.as_ref().unwrap().tagid[0], 2);
        assert_eq!(detail.extra["beta_feature_flag"], true);

        let template: WorkAgentWorkbenchTemplateResponse = serde_json::from_value(json!({
            "errcode": 0,
            "type": "keydata",
            "keydata": {
                "items": [{
                    "key": "Pending",
                    "data": "42",
                    "jump_url": "https://example.com/tasks",
                    "pagepath": "pages/tasks/index"
                }]
            },
            "image": {
                "url": "https://example.com/banner.png",
                "jump_url": "https://example.com/banner"
            },
            "list": {
                "items": [{
                    "title": "Task",
                    "subtitle": "Due today",
                    "jump_url": "https://example.com/task"
                }]
            },
            "webview": {
                "url": "https://example.com/workbench"
            },
            "template_version": 2
        }))
        .unwrap();
        assert_eq!(template.template_type.as_deref(), Some("keydata"));
        assert_eq!(
            template.keydata.as_ref().unwrap().items[0].key.as_deref(),
            Some("Pending")
        );
        assert_eq!(
            template.image.as_ref().unwrap().url.as_deref(),
            Some("https://example.com/banner.png")
        );
        assert_eq!(
            template.list.as_ref().unwrap().items[0].title.as_deref(),
            Some("Task")
        );
        assert_eq!(
            template.webview.as_ref().unwrap().url.as_deref(),
            Some("https://example.com/workbench")
        );
        assert_eq!(template.extra["template_version"], 2);
    }

    #[test]
    fn deserializes_work_base_responses() {
        let callback: WorkIpListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "ip_list": ["1.1.1.1", "2.2.2.2"],
            "trace_id": "ip-list"
        }))
        .unwrap();

        assert_eq!(callback.ip_list[0], "1.1.1.1");
        assert_eq!(callback.ip_list.len(), 2);
        assert_eq!(callback.extra["trace_id"], "ip-list");

        let token: WorkAccessTokenResponse = serde_json::from_value(json!({
            "access_token": "token",
            "expires_in": 7200,
            "issued_at": 1_800_000_000
        }))
        .unwrap();
        assert_eq!(token.access_token.as_deref(), Some("token"));
        assert_eq!(token.expires_in, Some(7200));
        assert_eq!(token.extra["issued_at"], 1_800_000_000);

        let status: WorkStatusResponse =
            serde_json::from_value(json!({ "errcode": 0, "request_id": "status-ok" })).unwrap();
        assert_eq!(status.errcode, Some(0));
        assert_eq!(status.extra["request_id"], "status-ok");

        let ticket: WorkTicketResponse = serde_json::from_value(json!({
            "ticket": "ticket",
            "expires_in": 7200,
            "issued_at": 1_800_000_000
        }))
        .unwrap();
        assert_eq!(ticket.ticket.as_deref(), Some("ticket"));
        assert_eq!(ticket.extra["issued_at"], 1_800_000_000);
    }

    #[test]
    fn deserializes_work_corpgroup_responses() {
        let share: WorkCorpGroupAppShareInfoResponse = serde_json::from_value(json!({
            "trace_id": "corp-share",
            "corp_list": [{ "corpid": "corp", "agentid": 100001, "corp_name": "Corp" }]
        }))
        .unwrap();
        assert_eq!(share.extra["trace_id"], "corp-share");
        assert_eq!(share.corp_list[0].corpid.as_deref(), Some("corp"));
        assert_eq!(share.corp_list[0].agentid, Some(100001));
        assert_eq!(share.corp_list[0].extra["corp_name"], "Corp");

        let token: WorkCorpGroupTokenResponse = serde_json::from_value(json!({
            "access_token": "token",
            "expires_in": 7200,
            "issued_at": 1_800_000_000
        }))
        .unwrap();
        assert_eq!(token.access_token.as_deref(), Some("token"));
        assert_eq!(token.expires_in, Some(7200));
        assert_eq!(token.extra["issued_at"], 1_800_000_000);

        let session: WorkCorpGroupTransferSessionResponse = serde_json::from_value(json!({
            "userid": "user",
            "session_key": "session",
            "session_expire": 300
        }))
        .unwrap();
        assert_eq!(session.userid.as_deref(), Some("user"));
        assert_eq!(session.session_key.as_deref(), Some("session"));
        assert_eq!(session.extra["session_expire"], 300);
    }

    #[test]
    fn deserializes_work_mini_program_session_response() {
        let session: WorkMiniProgramSessionResponse = serde_json::from_value(json!({
            "corpid": "corp",
            "userid": "user",
            "deviceid": "device",
            "session_key": "session",
            "open_data_scope": "full"
        }))
        .unwrap();

        assert_eq!(session.corpid.as_deref(), Some("corp"));
        assert_eq!(session.userid.as_deref(), Some("user"));
        assert_eq!(session.deviceid.as_deref(), Some("device"));
        assert_eq!(session.session_key.as_deref(), Some("session"));
        assert_eq!(session.extra["open_data_scope"], "full");
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
        let to_openid: UserIdToOpenIdResponse = serde_json::from_value(json!({
            "openid": "openid",
            "appid": "wxappid",
            "convert_source": "userid"
        }))
        .unwrap();
        let to_userid: OpenIdToUserIdResponse =
            serde_json::from_value(json!({ "userid": "user", "convert_source": "openid" }))
                .unwrap();

        assert_eq!(to_openid.openid.as_deref(), Some("openid"));
        assert_eq!(to_openid.appid.as_deref(), Some("wxappid"));
        assert_eq!(to_openid.extra["convert_source"], "userid");
        assert_eq!(to_userid.userid.as_deref(), Some("user"));
        assert_eq!(to_userid.extra["convert_source"], "openid");
    }

    #[test]
    fn serializes_work_user_management_requests_and_responses() {
        let create = serde_json::to_value(WorkUserRequest {
            userid: "user".to_string(),
            name: Some("User".to_string()),
            department: vec![1],
            order: Vec::new(),
            position: None,
            mobile: Some("13800000000".to_string()),
            gender: None,
            email: None,
            biz_mail: None,
            is_leader_in_dept: Vec::new(),
            direct_leader: Vec::new(),
            telephone: None,
            alias: None,
            address: None,
            main_department: None,
            to_invite: Some(true),
            enable: None,
            avatar_mediaid: None,
            external_position: None,
            external_profile: Some(WorkUserExternalProfile {
                external_corp_name: Some("Roze".to_string()),
                external_attr: vec![WorkUserExternalAttribute {
                    attr_type: Some(0),
                    name: Some("site".to_string()),
                    text: Some(WorkUserExternalAttributeText {
                        value: Some("hello".to_string()),
                    }),
                    web: None,
                    miniprogram: None,
                }],
            }),
            extattr: Some(WorkUserExtAttr {
                attrs: vec![
                    WorkUserExtAttrItem::text("level", "gold"),
                    WorkUserExtAttrItem::web("site", "Portal", "https://example.com"),
                ],
            }),
        })
        .unwrap();
        assert_eq!(create["userid"], "user");
        assert_eq!(create["department"][0], 1);
        assert_eq!(create["external_profile"]["external_corp_name"], "Roze");
        assert_eq!(
            create["external_profile"]["external_attr"][0]["text"]["value"],
            "hello"
        );
        assert!(create.get("email").is_none());
        assert!(create["external_profile"]["external_attr"][0]
            .as_object()
            .unwrap()
            .get("web")
            .is_none());
        assert_eq!(create["extattr"]["attrs"][0]["text"]["value"], "gold");
        assert_eq!(
            create["extattr"]["attrs"][1]["web"]["url"],
            "https://example.com"
        );

        let update = serde_json::to_value(WorkUserRequest {
            userid: "user".to_string(),
            name: None,
            department: Vec::new(),
            order: Vec::new(),
            position: Some("Engineer".to_string()),
            mobile: None,
            gender: None,
            email: Some("user@example.com".to_string()),
            biz_mail: None,
            is_leader_in_dept: Vec::new(),
            direct_leader: Vec::new(),
            telephone: None,
            alias: Some("alias".to_string()),
            address: None,
            main_department: None,
            to_invite: None,
            enable: Some(1),
            avatar_mediaid: None,
            external_position: None,
            external_profile: None,
            extattr: None,
        })
        .unwrap();
        assert_eq!(update["position"], "Engineer");
        assert_eq!(update["email"], "user@example.com");
        assert_eq!(update["enable"], 1);
        assert!(update.get("department").is_none());

        let batch_delete = serde_json::to_value(WorkUserBatchDeleteRequest {
            useridlist: vec!["user1".to_string(), "user2".to_string()],
        })
        .unwrap();
        assert_eq!(batch_delete["useridlist"][1], "user2");

        let list_id: WorkUserListIdResponse = serde_json::from_value(json!({
            "next_cursor": "cursor",
            "dept_user": [{ "userid": "user", "department": 1, "dept_role": "owner" }],
            "next_open_cursor": "open-cursor"
        }))
        .unwrap();
        assert_eq!(list_id.next_cursor.as_deref(), Some("cursor"));
        assert_eq!(list_id.dept_user[0].userid.as_deref(), Some("user"));
        assert_eq!(list_id.dept_user[0].extra["dept_role"], "owner");
        assert_eq!(list_id.extra["next_open_cursor"], "open-cursor");

        let lookup: WorkUserIdLookupResponse =
            serde_json::from_value(json!({ "userid": "user", "source": "mobile" })).unwrap();
        assert_eq!(lookup.userid.as_deref(), Some("user"));
        assert_eq!(lookup.extra["source"], "mobile");

        let user: WorkUserDetailResponse = serde_json::from_value(json!({
            "errcode": 0,
            "userid": "user",
            "name": "User",
            "department": [1],
            "order": [10],
            "position": "Engineer",
            "mobile": "13800000000",
            "gender": "1",
            "email": "user@example.com",
            "biz_mail": "user@corp.example",
            "is_leader_in_dept": [1],
            "direct_leader": ["leader"],
            "avatar": "https://example.com/avatar.png",
            "thumb_avatar": "https://example.com/thumb.png",
            "telephone": "010",
            "alias": "alias",
            "address": "addr",
            "open_userid": "open-user",
            "main_department": 1,
            "status": 1,
            "qr_code": "https://example.com/qr",
            "external_position": "External",
            "external_profile": {
                "external_corp_name": "Roze",
                "external_attr": [{
                    "type": 0,
                    "name": "Website",
                    "web": { "url": "https://example.com", "title": "Home" }
                }]
            },
            "custom_status_text": "busy"
        }))
        .unwrap();
        assert_eq!(user.errcode, Some(0));
        assert_eq!(user.user.userid.as_deref(), Some("user"));
        assert_eq!(user.user.department[0], 1);
        assert_eq!(user.user.status_kind(), Some(WorkUserStatusKind::Active));
        assert!(user.user.status_kind().expect("status").can_login());
        assert_eq!(user.user.extra["custom_status_text"], "busy");
        assert_eq!(
            user.user.external_profile.as_ref().unwrap().external_attr[0]
                .web
                .as_ref()
                .unwrap()
                .url
                .as_deref(),
            Some("https://example.com")
        );

        let simple_list: WorkDepartmentUserSimpleListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "has_more": true,
            "userlist": [{
                "userid": "user",
                "name": "User",
                "department": [1],
                "open_userid": "open-user",
                "user_ticket": "ticket"
            }]
        }))
        .unwrap();
        assert_eq!(simple_list.userlist[0].userid.as_deref(), Some("user"));
        assert_eq!(simple_list.userlist[0].department[0], 1);
        assert_eq!(
            simple_list.userlist[0].open_userid.as_deref(),
            Some("open-user")
        );
        assert_eq!(simple_list.extra["has_more"], true);
        assert_eq!(simple_list.userlist[0].extra["user_ticket"], "ticket");

        let detail_list: WorkDepartmentUserDetailListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "cursor": "next",
            "userlist": [{
                "userid": "user",
                "name": "User",
                "mobile": "13800000000",
                "department": [1],
                "biz_ext": { "level": "gold" }
            }]
        }))
        .unwrap();
        assert_eq!(detail_list.userlist[0].userid.as_deref(), Some("user"));
        assert_eq!(
            detail_list.userlist[0].mobile.as_deref(),
            Some("13800000000")
        );
        assert_eq!(detail_list.extra["cursor"], "next");
        assert_eq!(detail_list.userlist[0].extra["biz_ext"]["level"], "gold");

        let invite = serde_json::to_value(WorkUserInviteRequest {
            user: vec!["user".to_string()],
            party: vec![1],
            tag: Vec::new(),
        })
        .unwrap();
        assert_eq!(invite["user"][0], "user");
        assert_eq!(invite["party"][0], 1);
        assert!(invite.get("tag").is_none());

        let invite_response: WorkUserInviteResponse = serde_json::from_value(json!({
            "invaliduser": ["bad-user"],
            "invalidparty": [2],
            "invalidtag": [3],
            "invalid_open_userid": ["bad-open"]
        }))
        .unwrap();
        assert_eq!(invite_response.invaliduser[0], "bad-user");
        assert_eq!(invite_response.invalidparty[0], 2);
        assert_eq!(invite_response.invalidtag[0], 3);
        assert_eq!(invite_response.extra["invalid_open_userid"][0], "bad-open");

        let qrcode: WorkJoinQrCodeResponse = serde_json::from_value(json!({
            "join_qrcode": "https://example.com/qr",
            "expires_in": 300
        }))
        .unwrap();
        assert_eq!(
            qrcode.join_qrcode.as_deref(),
            Some("https://example.com/qr")
        );
        assert_eq!(qrcode.extra["expires_in"], 300);

        let active: WorkUserActiveStatResponse =
            serde_json::from_value(json!({ "active_cnt": "42", "stat_date": "2026-07-17" }))
                .unwrap();
        assert_eq!(active.active_cnt.as_deref(), Some("42"));
        assert_eq!(active.extra["stat_date"], "2026-07-17");
        assert_eq!(WorkUserStatusKind::from(2), WorkUserStatusKind::Disabled);
        assert_eq!(WorkUserStatusKind::from(4), WorkUserStatusKind::Inactive);
        assert!(WorkUserStatusKind::Exited.is_terminal());
        assert_eq!(WorkUserStatusKind::from(99), WorkUserStatusKind::Other(99));
    }

    #[test]
    fn deserializes_work_linked_corp_user_responses() {
        let perm: WorkLinkedCorpPermListResponse = serde_json::from_value(json!({
            "department_ids": ["Corp/department"],
            "userids": ["Corp/user"],
            "trace_id": "linked-perm"
        }))
        .unwrap();
        assert_eq!(perm.department_ids[0], "Corp/department");
        assert_eq!(perm.userids[0], "Corp/user");
        assert_eq!(perm.extra["trace_id"], "linked-perm");

        let user: WorkLinkedCorpUserResponse = serde_json::from_value(json!({
            "trace_id": "linked-user",
            "user_info": {
                "userid": "Corp/user",
                "name": "User",
                "mobile": "13800000000",
                "department": ["Corp/department"],
                "status": 1,
                "member_source": "linked"
            }
        }))
        .unwrap();
        assert_eq!(user.extra["trace_id"], "linked-user");
        let user_info = user.user_info.unwrap();
        assert_eq!(user_info.userid.as_deref(), Some("Corp/user"));
        assert_eq!(user_info.name.as_deref(), Some("User"));
        assert_eq!(user_info.mobile.as_deref(), Some("13800000000"));
        assert_eq!(user_info.department[0], "Corp/department");
        assert_eq!(user_info.status_kind(), Some(WorkUserStatusKind::Active));
        assert_eq!(user_info.extra["member_source"], "linked");

        let simple: WorkLinkedCorpUserListResponse = serde_json::from_value(json!({
            "next_cursor": "linked-next",
            "userlist": [{ "userid": "Corp/user", "name": "User", "member_source": "linked" }]
        }))
        .unwrap();
        assert_eq!(simple.userlist[0].userid.as_deref(), Some("Corp/user"));
        assert_eq!(simple.userlist[0].name.as_deref(), Some("User"));
        assert_eq!(simple.extra["next_cursor"], "linked-next");
        assert_eq!(simple.userlist[0].extra["member_source"], "linked");

        let departments: WorkLinkedCorpDepartmentListResponse = serde_json::from_value(json!({
            "trace_id": "linked-department",
            "department_list": [{
                "department_id": "Corp/department",
                "name": "Dept",
                "parentid": "Corp/root",
                "order": 1,
                "department_level": 2
            }]
        }))
        .unwrap();
        assert_eq!(departments.extra["trace_id"], "linked-department");
        assert_eq!(
            departments.department_list[0].department_id.as_deref(),
            Some("Corp/department")
        );
        assert_eq!(departments.department_list[0].name.as_deref(), Some("Dept"));
        assert_eq!(departments.department_list[0].order, Some(1));
        assert_eq!(departments.department_list[0].extra["department_level"], 2);
    }

    #[test]
    fn serializes_work_user_batch_and_export_jobs() {
        let batch = serde_json::to_value(WorkUserBatchJobRequest {
            media_id: "media".to_string(),
            to_invite: true,
            callbacks: Some(WorkUserBatchJobCallback {
                url: "https://example.com/callback".to_string(),
                token: "token".to_string(),
                encodingaeskey: "key".to_string(),
            }),
        })
        .unwrap();
        assert_eq!(batch["media_id"], "media");
        assert_eq!(batch["to_invite"], true);
        assert_eq!(batch["callbacks"]["token"], "token");

        let without_callback = serde_json::to_value(WorkUserBatchJobRequest {
            media_id: "media".to_string(),
            to_invite: false,
            callbacks: None,
        })
        .unwrap();
        assert!(without_callback.get("callbacks").is_none());

        let export = serde_json::to_value(WorkUserExportJobRequest {
            encoding_aeskey: "aes-key".to_string(),
            block_size: 10_000,
        })
        .unwrap();
        assert_eq!(export["encoding_aeskey"], "aes-key");
        assert_eq!(export["block_size"], 10_000);

        let tag_export = serde_json::to_value(WorkUserExportTagUserJobRequest {
            tagid: 1,
            encoding_aeskey: "aes-key".to_string(),
            block_size: 10_000,
        })
        .unwrap();
        assert_eq!(tag_export["tagid"], 1);

        let batch_job: WorkUserBatchJobResponse =
            serde_json::from_value(json!({ "jobid": "batch-job", "job_source": "csv" })).unwrap();
        assert_eq!(batch_job.jobid.as_deref(), Some("batch-job"));
        assert_eq!(batch_job.extra["job_source"], "csv");

        let batch_result: WorkUserBatchJobResultResponse = serde_json::from_value(json!({
            "status": 2,
            "type": "sync_user",
            "total": 10,
            "percentage": 100,
            "result": [{ "userid": "user", "errcode": 0, "row_num": 3 }],
            "fail_count": 0
        }))
        .unwrap();
        assert_eq!(batch_result.job_type.as_deref(), Some("sync_user"));
        assert_eq!(
            batch_result.status_kind(),
            Some(WorkAsyncJobStatusKind::Processing)
        );
        assert_eq!(
            batch_result.job_type_kind(),
            Some(WorkAsyncJobTypeKind::SyncUser)
        );
        assert!(batch_result
            .job_type_kind()
            .expect("job type")
            .is_contact_import());
        assert_eq!(batch_result.result[0].userid.as_deref(), Some("user"));
        assert_eq!(batch_result.result[0].errcode, Some(0));
        assert_eq!(batch_result.result[0].extra["row_num"], 3);
        assert_eq!(batch_result.extra["fail_count"], 0);

        let export_job: WorkUserExportJobResponse =
            serde_json::from_value(json!({ "jobid": "export-job", "export_type": "user" }))
                .unwrap();
        assert_eq!(export_job.jobid.as_deref(), Some("export-job"));
        assert_eq!(export_job.extra["export_type"], "user");

        let export_result: WorkUserExportJobResultResponse = serde_json::from_value(json!({
            "status": 2,
            "next_cursor": "cursor",
            "data_list": [{
                "userid": "user",
                "name": "User",
                "department": [1],
                "mobile": "13800000000",
                "biz_ext": { "grade": "A" }
            }]
        }))
        .unwrap();
        assert_eq!(export_result.status, Some(2));
        assert_eq!(
            export_result.status_kind(),
            Some(WorkAsyncJobStatusKind::Processing)
        );
        assert_eq!(export_result.data_list[0].userid.as_deref(), Some("user"));
        assert_eq!(export_result.data_list[0].name.as_deref(), Some("User"));
        assert_eq!(export_result.data_list[0].department[0], 1);
        assert_eq!(export_result.extra["next_cursor"], "cursor");
        assert_eq!(export_result.data_list[0].extra["biz_ext"]["grade"], "A");
        assert_eq!(
            WorkAsyncJobStatusKind::from(3),
            WorkAsyncJobStatusKind::Finished
        );
        assert!(WorkAsyncJobStatusKind::Finished.is_finished());
        assert_eq!(
            WorkAsyncJobStatusKind::from(99),
            WorkAsyncJobStatusKind::Other(99)
        );
        assert_eq!(
            WorkAsyncJobTypeKind::from_code("REPLACE_PARTY"),
            WorkAsyncJobTypeKind::ReplaceParty
        );
        assert_eq!(
            WorkAsyncJobTypeKind::from_code("export_simple_user"),
            WorkAsyncJobTypeKind::ExportSimpleUser
        );
        assert!(WorkAsyncJobTypeKind::ExportUser.is_export());
        assert!(!WorkAsyncJobTypeKind::ReplaceUser.is_export());
        assert_eq!(
            WorkAsyncJobTypeKind::from_code("SOMETHING_NEW"),
            WorkAsyncJobTypeKind::Other
        );
    }

    #[test]
    fn deserializes_id_convert_responses() {
        let union: WorkUnionIdToExternalUserIdResponse = serde_json::from_value(json!({
            "errcode": 0,
            "external_userid": "external",
            "pending_id": "pending",
            "convert_scene": "union"
        }))
        .unwrap();
        assert_eq!(union.external_userid.as_deref(), Some("external"));
        assert_eq!(union.extra["convert_scene"], "union");

        let pending: WorkExternalUserIdToPendingIdResponse = serde_json::from_value(json!({
            "result": [{ "external_userid": "external", "pending_id": "pending", "item_source": "batch" }],
            "request_id": "pending-list"
        }))
        .unwrap();
        assert_eq!(pending.result[0].pending_id.as_deref(), Some("pending"));
        assert_eq!(pending.result[0].extra["item_source"], "batch");
        assert_eq!(pending.extra["request_id"], "pending-list");

        let user_to_open: WorkUserIdToOpenUserIdResponse = serde_json::from_value(json!({
            "open_userid_list": [{ "userid": "user", "open_userid": "open-user", "source": "legacy" }],
            "trace_id": "user-to-open"
        }))
        .unwrap();
        assert_eq!(
            user_to_open.open_userid_list[0].open_userid.as_deref(),
            Some("open-user")
        );
        assert_eq!(user_to_open.open_userid_list[0].extra["source"], "legacy");
        assert_eq!(user_to_open.extra["trace_id"], "user-to-open");

        let open_to_user: WorkOpenUserIdToUserIdResponse = serde_json::from_value(json!({
            "userid_list": [{ "userid": "user", "open_userid": "open-user", "user_source": "api" }],
            "invalid_userid_list": ["bad-open-user"],
            "trace_id": "open-to-user"
        }))
        .unwrap();
        assert_eq!(open_to_user.userid_list[0].userid.as_deref(), Some("user"));
        assert_eq!(open_to_user.invalid_userid_list[0], "bad-open-user");
        assert_eq!(open_to_user.userid_list[0].extra["user_source"], "api");
        assert_eq!(open_to_user.extra["trace_id"], "open-to-user");

        let tag: WorkExternalTagIdToOpenExternalTagIdResponse = serde_json::from_value(json!({
            "items": [{ "external_tagid": "tag", "open_external_tagid": "open-tag", "tag_source": "crm" }],
            "invalid_tagid_list": [],
            "trace_id": "tag-open"
        }))
        .unwrap();
        assert_eq!(
            tag.items[0].open_external_tagid.as_deref(),
            Some("open-tag")
        );
        assert_eq!(tag.items[0].extra["tag_source"], "crm");
        assert_eq!(tag.extra["trace_id"], "tag-open");

        let from_service: WorkFromServiceExternalUserIdResponse = serde_json::from_value(json!({
            "external_userid": "external",
            "source_agentid": 100001
        }))
        .unwrap();
        assert_eq!(from_service.external_userid.as_deref(), Some("external"));
        assert_eq!(from_service.extra["source_agentid"], 100001);
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
        let typed_status = serde_json::to_value(WorkInvoiceStatusRequest::new(
            "card",
            "encrypted",
            WorkInvoiceReimburseStatusKind::Locked,
        ))
        .unwrap();
        assert_eq!(
            typed_status["reimburse_status"],
            WorkInvoiceReimburseStatusKind::Locked.as_code()
        );

        let batch = serde_json::to_value(WorkInvoiceStatusBatchRequest {
            openid: "openid".to_string(),
            reimburse_status: "INVOICE_REIMBURSE_CLOSURE".to_string(),
            invoice_list: vec![card],
        })
        .unwrap();
        assert_eq!(batch["openid"], "openid");
        assert_eq!(batch["invoice_list"][0]["card_id"], "card");
        let typed_batch = serde_json::to_value(WorkInvoiceStatusBatchRequest::new(
            "openid",
            WorkInvoiceReimburseStatusKind::Closure,
            vec![WorkInvoiceCardRequest {
                card_id: "card".to_string(),
                encrypt_code: "encrypted".to_string(),
            }],
        ))
        .unwrap();
        assert_eq!(typed_batch["reimburse_status"], "INVOICE_REIMBURSE_CLOSURE");
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
            "user_info": {
                "fee": 100,
                "title": "Roze",
                "billing_no": "NO100",
                "tax_no": "TAX100",
                "info": [{
                    "name": "Cloud service",
                    "num": "1",
                    "unit": "month",
                    "fee": 100,
                    "price": "100.00",
                    "tax_rate": "0.06",
                    "tax_amount": 6,
                    "discount_amount": 2
                }]
            },
            "invoice_source": "wechat"
        }))
        .unwrap();
        assert_eq!(info.card_id.as_deref(), Some("card"));
        assert_eq!(info.invoice_type.as_deref(), Some("vat"));
        assert_eq!(info.extra["invoice_source"], "wechat");
        let user_info = info.user_info.unwrap();
        assert_eq!(user_info.fee, Some(100));
        assert_eq!(user_info.title.as_deref(), Some("Roze"));
        assert_eq!(user_info.billing_no.as_deref(), Some("NO100"));
        assert_eq!(user_info.info[0].name.as_deref(), Some("Cloud service"));
        assert_eq!(user_info.info[0].tax_amount, Some(6));
        assert_eq!(user_info.info[0].extra["discount_amount"], 2);

        let batch: WorkInvoiceInfoBatchResponse = serde_json::from_value(json!({
            "trace_id": "invoice-batch",
            "item_list": [{
                "card_id": "card",
                "encrypt_code": "encrypted",
                "reimburse_status": "INVOICE_REIMBURSE_INIT",
                "user_info": {
                    "fee": 100,
                    "title": "Roze",
                    "info": [{ "name": "Cloud service", "fee": 100 }]
                },
                "item_source": "batch"
            }]
        }))
        .unwrap();
        assert_eq!(batch.extra["trace_id"], "invoice-batch");
        assert_eq!(batch.item_list[0].card_id.as_deref(), Some("card"));
        assert_eq!(
            batch.item_list[0].reimburse_status.as_deref(),
            Some("INVOICE_REIMBURSE_INIT")
        );
        assert_eq!(
            batch.item_list[0].reimburse_status_kind(),
            Some(WorkInvoiceReimburseStatusKind::Init)
        );
        assert!(!batch.item_list[0].is_reimbursed());
        assert_eq!(
            WorkInvoiceReimburseStatusKind::from_code("invoice_reimburse_lock"),
            WorkInvoiceReimburseStatusKind::Locked
        );
        assert_eq!(
            WorkInvoiceReimburseStatusKind::from_code("INVOICE_REIMBURSE_CLOSURE"),
            WorkInvoiceReimburseStatusKind::Closure
        );
        assert!(WorkInvoiceReimburseStatusKind::Closure.is_final());
        assert_eq!(
            WorkInvoiceReimburseStatusKind::from_code("SOMETHING_NEW"),
            WorkInvoiceReimburseStatusKind::Other
        );
        assert_eq!(batch.item_list[0].extra["item_source"], "batch");
        assert_eq!(
            batch.item_list[0]
                .user_info
                .as_ref()
                .unwrap()
                .title
                .as_deref(),
            Some("Roze")
        );
        assert_eq!(
            batch.item_list[0].user_info.as_ref().unwrap().info[0]
                .name
                .as_deref(),
            Some("Cloud service")
        );
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
        let typed_scope = serde_json::to_value(WorkExternalPaySetMerchantUseScopeRequest::new(
            "1900000109",
            WorkExternalPayUseScopeKind::Party,
        ))
        .unwrap();
        assert_eq!(typed_scope["allow_use_scope"], "partyid");

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
            "allow_use_scope": [{ "type": "all", "scope_name": "all staff" }],
            "merchant_status": "active"
        }))
        .unwrap();
        assert_eq!(merchant.mch_id.as_deref(), Some("1900000109"));
        assert_eq!(
            merchant.allow_use_scope[0].scope_type.as_deref(),
            Some("all")
        );
        assert_eq!(
            merchant.allow_use_scope[0].scope_kind(),
            Some(WorkExternalPayUseScopeKind::All)
        );
        assert!(merchant.allow_use_scope[0].applies_to_all());
        assert_eq!(
            WorkExternalPayUseScopeKind::from_code("department"),
            WorkExternalPayUseScopeKind::Party
        );
        assert_eq!(
            WorkExternalPayUseScopeKind::from_code("unknown"),
            WorkExternalPayUseScopeKind::Other
        );
        assert_eq!(merchant.extra["merchant_status"], "active");
        assert_eq!(merchant.allow_use_scope[0].extra["scope_name"], "all staff");

        let bills: WorkExternalPayBillListResponse = serde_json::from_value(json!({
            "next_cursor": "cursor",
            "total_count": 1,
            "bill_list": [{
                "out_trade_no": "trade-no",
                "transaction_id": "transaction",
                "payee_userid": "payee",
                "payer_userid": "payer",
                "amount": 100,
                "status": "success",
                "pay_time": 1_800_000_000,
                "trade_type": "JSAPI"
            }]
        }))
        .unwrap();
        assert_eq!(bills.next_cursor.as_deref(), Some("cursor"));
        assert_eq!(bills.extra["total_count"], 1);
        assert_eq!(bills.bill_list[0].out_trade_no.as_deref(), Some("trade-no"));
        assert_eq!(bills.bill_list[0].amount, Some(100));
        assert_eq!(bills.bill_list[0].payee_userid.as_deref(), Some("payee"));
        assert_eq!(
            bills.bill_list[0].status_kind(),
            Some(WorkExternalPayBillStatusKind::Success)
        );
        assert!(bills.bill_list[0].is_success());
        assert!(bills.bill_list[0].is_terminal());
        assert_eq!(
            WorkExternalPayBillStatusKind::from_code("USER_PAYING"),
            WorkExternalPayBillStatusKind::UserPaying
        );
        assert!(!WorkExternalPayBillStatusKind::UserPaying.is_terminal());
        assert_eq!(
            WorkExternalPayBillStatusKind::from_code("SOMETHING_NEW"),
            WorkExternalPayBillStatusKind::Other
        );
        assert_eq!(bills.bill_list[0].extra["trade_type"], "JSAPI");
    }

    #[test]
    fn deserializes_upload_media_response_type_field() {
        let image: WorkUploadImageResponse = serde_json::from_value(json!({
            "url": "https://example.com/image.png",
            "cdn_file_id": "cdn-image"
        }))
        .unwrap();
        assert_eq!(image.url.as_deref(), Some("https://example.com/image.png"));
        assert_eq!(image.extra["cdn_file_id"], "cdn-image");

        let response: WorkUploadMediaResponse = serde_json::from_value(json!({
            "media_id": "mid",
            "type": "image",
            "created_at": "1800000000",
            "file_size": 1024
        }))
        .unwrap();

        assert_eq!(response.media_id.as_deref(), Some("mid"));
        assert_eq!(response.media_type.as_deref(), Some("image"));
        assert_eq!(response.media_type_kind(), Some(WorkMediaTypeKind::Image));
        assert!(response.is_image());
        assert!(response
            .media_type_kind()
            .expect("media type")
            .is_binary_file());
        assert_eq!(
            WorkMediaTypeKind::from_code("VOICE"),
            WorkMediaTypeKind::Voice
        );
        assert_eq!(
            WorkMediaTypeKind::from_code("video"),
            WorkMediaTypeKind::Video
        );
        assert_eq!(
            WorkMediaTypeKind::from_code("file"),
            WorkMediaTypeKind::File
        );
        assert_eq!(
            WorkMediaTypeKind::from_code("link"),
            WorkMediaTypeKind::Other
        );
        assert!(!WorkMediaTypeKind::Other.is_binary_file());
        assert_eq!(response.created_at.as_deref(), Some("1800000000"));
        assert_eq!(response.extra["file_size"], 1024);

        let full = WorkMediaDownload {
            status: 200,
            headers: vec![
                ("Content-Type".to_string(), "image/jpeg".to_string()),
                (
                    "Content-Disposition".to_string(),
                    "attachment; filename=\"../media.jpg\"".to_string(),
                ),
                ("Content-Length".to_string(), "10".to_string()),
                ("Accept-Ranges".to_string(), "bytes".to_string()),
            ],
            body: bytes::Bytes::from_static(b"0123456789"),
        };
        assert_eq!(full.content_type(), Some("image/jpeg"));
        assert_eq!(full.file_name(), Some("media.jpg"));
        assert_eq!(full.content_length(), Some(10));
        assert_eq!(full.total_length(), Some(10));
        assert!(full.accepts_byte_ranges());
        assert!(!full.is_partial());
        assert_eq!(full.body.len(), 10);

        let partial = WorkMediaDownload {
            status: 206,
            headers: vec![
                ("Content-Range".to_string(), "bytes 10-19/25".to_string()),
                ("Content-Length".to_string(), "10".to_string()),
            ],
            body: bytes::Bytes::from_static(b"0123456789"),
        };
        assert_eq!(
            partial.content_range(),
            Some(WorkMediaContentRange {
                start: 10,
                end_inclusive: 19,
                total: Some(25),
            })
        );
        assert!(partial.is_partial());
        assert_eq!(partial.total_length(), Some(25));
        assert_eq!(partial.next_range_start(), Some(20));

        let final_chunk = WorkMediaDownload {
            status: 206,
            headers: vec![("content-range".to_string(), "bytes 20-24/25".to_string())],
            body: bytes::Bytes::from_static(b"01234"),
        };
        assert_eq!(final_chunk.next_range_start(), None);
        assert_eq!(
            work_media_range_header(0, Some(1023)).unwrap(),
            "bytes=0-1023"
        );
        assert_eq!(work_media_range_header(1024, None).unwrap(), "bytes=1024-");
        assert!(work_media_range_header(10, Some(9)).is_err());
    }

    #[test]
    fn serializes_work_media_upload_by_url_requests() {
        let request = WorkMediaUploadByUrlRequest::new(
            WorkMediaUploadByUrlSceneKind::ExternalContactGroupWelcome,
            WorkMediaUploadByUrlTypeKind::Video,
            "video.mp4",
            "https://cdn.example.com/video.mp4",
            "198918f40ecc7cab0fc4231adaf67c96",
        );
        assert_eq!(
            request.scene_kind(),
            Some(WorkMediaUploadByUrlSceneKind::ExternalContactGroupWelcome)
        );
        assert_eq!(
            request.media_type_kind(),
            Some(WorkMediaUploadByUrlTypeKind::Video)
        );

        let value = serde_json::to_value(request).unwrap();
        assert_eq!(value["scene"], 1);
        assert_eq!(value["type"], "video");
        assert_eq!(value["filename"], "video.mp4");
        assert_eq!(value["url"], "https://cdn.example.com/video.mp4");
        assert_eq!(value["md5"], "198918f40ecc7cab0fc4231adaf67c96");
        assert!(value.get("media_type").is_none());

        let result_request = serde_json::to_value(WorkMediaUploadByUrlResultRequest {
            jobid: "job-id".to_string(),
        })
        .unwrap();
        assert_eq!(result_request, json!({ "jobid": "job-id" }));
    }

    #[test]
    fn deserializes_work_media_upload_by_url_responses() {
        let created: WorkMediaUploadByUrlResponse = serde_json::from_value(json!({
            "errcode": 0,
            "errmsg": "ok",
            "jobid": "job-id",
            "expires_in": 3600
        }))
        .unwrap();
        assert_eq!(created.jobid.as_deref(), Some("job-id"));
        assert_eq!(created.extra["expires_in"], 3600);

        let completed: WorkMediaUploadByUrlResultResponse = serde_json::from_value(json!({
            "errcode": 0,
            "errmsg": "ok",
            "status": 2,
            "detail": {
                "errcode": 0,
                "errmsg": "ok",
                "media_id": "media-id",
                "created_at": "1380000000",
                "file_size": 2048
            },
            "trace_id": "trace-id"
        }))
        .unwrap();
        assert_eq!(
            completed.status_kind(),
            Some(WorkMediaUploadByUrlStatusKind::Completed)
        );
        assert!(completed.is_completed());
        assert!(completed.is_successful());
        assert!(completed.status_kind().expect("status").is_terminal());
        assert_eq!(
            completed
                .detail
                .as_ref()
                .and_then(|detail| detail.media_id.as_deref()),
            Some("media-id")
        );
        assert_eq!(
            completed
                .detail
                .as_ref()
                .map(|detail| &detail.extra["file_size"]),
            Some(&json!(2048))
        );
        assert_eq!(completed.extra["trace_id"], "trace-id");

        let failed: WorkMediaUploadByUrlResultResponse = serde_json::from_value(json!({
            "status": 3,
            "detail": {
                "errcode": 301019,
                "errmsg": "md5 mismatch"
            }
        }))
        .unwrap();
        assert_eq!(
            failed.status_kind(),
            Some(WorkMediaUploadByUrlStatusKind::Failed)
        );
        assert!(!failed.is_successful());
        assert!(failed.status_kind().expect("status").is_terminal());

        assert_eq!(
            WorkMediaUploadByUrlStatusKind::from(1),
            WorkMediaUploadByUrlStatusKind::Processing
        );
        assert_eq!(
            WorkMediaUploadByUrlStatusKind::from(99),
            WorkMediaUploadByUrlStatusKind::Other(99)
        );
        assert!(!WorkMediaUploadByUrlStatusKind::Processing.is_terminal());
    }

    #[test]
    fn serializes_work_oa_calendar_and_dial_requests() {
        let calendar = serde_json::to_value(WorkCalendarAddRequest {
            calendar: WorkCalendarCreate {
                organizer: "user".to_string(),
                summary: "Team".to_string(),
                color: "#FF3030".to_string(),
                description: Some("Team calendar".to_string()),
                shares: vec![WorkCalendarShareRequest {
                    userid: "member".to_string(),
                }],
                readonly: Some(0),
                extra: Value::Null,
            },
            agentid: 100001,
        })
        .unwrap();
        assert_eq!(calendar["agentid"], 100001);
        assert_eq!(calendar["calendar"]["summary"], "Team");
        assert_eq!(calendar["calendar"]["shares"][0]["userid"], "member");

        let calendar_update = serde_json::to_value(WorkCalendarUpdateRequest {
            calendar: WorkCalendarUpdate {
                cal_id: "wc100".to_string(),
                summary: "Team updated".to_string(),
                color: "#00FF00".to_string(),
                description: None,
                shares: Vec::new(),
                readonly: None,
                extra: Value::Null,
            },
        })
        .unwrap();
        assert_eq!(calendar_update["calendar"]["cal_id"], "wc100");
        assert!(calendar_update["calendar"].get("description").is_none());

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
        let calendar_add: WorkCalendarAddResponse = serde_json::from_value(json!({
            "errcode": 0,
            "cal_id": "wc100",
            "request_id": "calendar-add-1"
        }))
        .unwrap();
        assert_eq!(calendar_add.cal_id.as_deref(), Some("wc100"));
        assert_eq!(calendar_add.extra["request_id"], "calendar-add-1");

        let calendar_get: WorkCalendarGetResponse = serde_json::from_value(json!({
            "next_cursor": "calendar-cursor",
            "calendar_list": [{
                "cal_id": "wc100",
                "adminis": ["admin"],
                "summary": "Team",
                "shares": [{ "userid": "user", "readonly": 1, "share_type": "member" }],
                "is_public": 1,
                "public_range": {
                    "userids": ["user"],
                    "partyids": [2],
                    "range_version": 1
                },
                "is_corp_calendar": 1,
                "timezone": "Asia/Shanghai"
            }]
        }))
        .unwrap();
        assert_eq!(
            calendar_get.calendar_list[0].summary.as_deref(),
            Some("Team")
        );
        assert_eq!(
            calendar_get.calendar_list[0].shares[0].userid.as_deref(),
            Some("user")
        );
        assert_eq!(calendar_get.calendar_list[0].adminis[0], "admin");
        assert_eq!(calendar_get.calendar_list[0].is_public, Some(1));
        let public_range = calendar_get.calendar_list[0]
            .public_range
            .as_ref()
            .expect("calendar public range");
        assert_eq!(public_range.userids[0], "user");
        assert_eq!(public_range.partyids[0], 2);
        assert_eq!(public_range.extra["range_version"], 1);
        assert_eq!(calendar_get.calendar_list[0].is_corp_calendar, Some(1));
        assert_eq!(calendar_get.extra["next_cursor"], "calendar-cursor");
        assert_eq!(
            calendar_get.calendar_list[0].extra["timezone"],
            "Asia/Shanghai"
        );
        assert_eq!(
            calendar_get.calendar_list[0].shares[0].extra["share_type"],
            "member"
        );

        let dial: WorkDialRecordResponse = serde_json::from_value(json!({
            "record": [{ "callee": "user", "caller": "agent", "duration": 60, "call_scene": "pstn" }],
            "has_more": false
        }))
        .unwrap();
        assert_eq!(dial.record[0].callee.as_deref(), Some("user"));
        assert_eq!(dial.record[0].duration, Some(60));
        assert_eq!(dial.record[0].extra["call_scene"], "pstn");
        assert_eq!(dial.extra["has_more"], false);

        let call: WorkPstnccCallResponse = serde_json::from_value(json!({
            "states": [{ "callee_userid": "user", "callid": "call-1", "state": 1, "state_text": "ringing" }],
            "session_id": "pstn-session"
        }))
        .unwrap();
        assert_eq!(call.states[0].callid.as_deref(), Some("call-1"));
        assert_eq!(call.states[0].state, Some(1));
        assert_eq!(call.states[0].extra["state_text"], "ringing");
        assert_eq!(call.extra["session_id"], "pstn-session");

        let states: WorkPstnccGetStatesResponse = serde_json::from_value(json!({
            "istalked": 1,
            "calltime": 1_800_000_000,
            "talktime": 30,
            "reason": 0,
            "state_detail": "completed"
        }))
        .unwrap();
        assert_eq!(states.istalked, Some(1));
        assert_eq!(states.reason, Some(0));
        assert_eq!(states.extra["state_detail"], "completed");
    }

    #[test]
    fn serializes_work_oa_approval_journal_and_schedule_requests() {
        let approval_create = serde_json::to_value(WorkApprovalCreateTemplateRequest {
            template_name: vec![WorkApprovalLocalizedText {
                text: "Leave".to_string(),
                lang: "zh_CN".to_string(),
            }],
            template_content: WorkApprovalTemplateContent {
                controls: vec![WorkApprovalTemplateControl {
                    property: WorkApprovalTemplateProperty {
                        control: "Text".to_string(),
                        id: "Text-1".to_string(),
                        title: vec![WorkApprovalLocalizedText {
                            text: "Reason".to_string(),
                            lang: "zh_CN".to_string(),
                        }],
                        placeholder: vec![WorkApprovalLocalizedText {
                            text: "Input".to_string(),
                            lang: "zh_CN".to_string(),
                        }],
                        require: 1,
                        un_print: 0,
                        extra: Value::Null,
                    },
                    config: WorkApprovalTemplateConfig::default(),
                    extra: Value::Null,
                }],
                extra: Value::Null,
            },
        })
        .unwrap();
        assert_eq!(approval_create["template_name"][0]["text"], "Leave");
        assert_eq!(
            approval_create["template_content"]["controls"][0]["property"]["id"],
            "Text-1"
        );

        let approval_update = serde_json::to_value(WorkApprovalUpdateTemplateRequest {
            template_id: "template-1".to_string(),
            template_name: vec![WorkApprovalLocalizedText {
                text: "Leave".to_string(),
                lang: "zh_CN".to_string(),
            }],
            template_content: WorkApprovalTemplateContent {
                controls: Vec::new(),
                extra: Value::Null,
            },
        })
        .unwrap();
        assert_eq!(approval_update["template_id"], "template-1");

        let journal = serde_json::to_value(WorkJournalRecordListRequest {
            starttime: "1800000000".to_string(),
            endtime: "1800086400".to_string(),
            cursor: "0".to_string(),
            limit: 100,
            filters: vec![WorkJournalFilter {
                key: "template_id".to_string(),
                value: "template-1".to_string(),
            }],
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
            schedule: WorkScheduleCreate {
                admins: vec!["admin".to_string()],
                start_time: 1_800_000_000,
                end_time: 1_800_003_600,
                attendees: vec![WorkScheduleAttendee {
                    userid: "attendee".to_string(),
                    response_status: None,
                    extra: Value::Null,
                }],
                summary: Some("Daily".to_string()),
                description: Some("Daily sync".to_string()),
                reminders: Some(WorkScheduleReminders {
                    is_remind: Some(1),
                    remind_before_event_secs: Some(3600),
                    remind_time_diffs: vec![-3600, -900],
                    is_repeat: Some(1),
                    repeat_type: Some(7),
                    repeat_until: Some(1_900_000_000),
                    is_custom_repeat: Some(1),
                    repeat_interval: Some(1),
                    repeat_day_of_week: vec![3, 7],
                    repeat_day_of_month: vec![10, 21],
                    timezone: Some(8),
                    exclude_time_list: vec![WorkScheduleExcludeTime {
                        start_time: 1_800_086_400,
                        extra: Value::Null,
                    }],
                    extra: Value::Null,
                }),
                location: Some("Room A".to_string()),
                cal_id: Some("wc100".to_string()),
                organizer: Some("user".to_string()),
                extra: Value::Null,
            },
            agentid: 100001,
        })
        .unwrap();
        assert_eq!(schedule["agentid"], 100001);
        assert_eq!(schedule["schedule"]["organizer"], "user");
        assert_eq!(
            schedule["schedule"]["reminders"]["repeat_day_of_week"][1],
            7
        );
        assert_eq!(
            schedule["schedule"]["reminders"]["exclude_time_list"][0]["start_time"],
            1_800_086_400
        );

        let schedule_update = serde_json::to_value(WorkScheduleUpdateRequest {
            schedule: WorkScheduleUpdate {
                schedule_id: "schedule-1".to_string(),
                admins: Vec::new(),
                start_time: 1_800_000_100,
                end_time: 1_800_003_700,
                attendees: Vec::new(),
                summary: Some("Daily updated".to_string()),
                description: None,
                reminders: None,
                location: None,
                cal_id: Some("wc100".to_string()),
                organizer: None,
                extra: Value::Null,
            },
        })
        .unwrap();
        assert_eq!(schedule_update["schedule"]["schedule_id"], "schedule-1");
        assert!(schedule_update["schedule"].get("attendees").is_none());

        let by_calendar = serde_json::to_value(WorkScheduleByCalendarRequest {
            cal_id: "wc100".to_string(),
            offset: Some(100),
            limit: Some(1000),
        })
        .unwrap();
        assert_eq!(by_calendar["cal_id"], "wc100");
        assert_eq!(by_calendar["limit"], 1000);
    }

    #[test]
    fn serializes_work_oa_checkin_approval_and_vacation_requests() {
        let checkin = serde_json::to_value(WorkCheckinDataRequest {
            opencheckindatatype: 3,
            starttime: 1_800_000_000,
            endtime: 1_800_086_400,
            useridlist: vec!["user".to_string()],
        })
        .unwrap();
        assert_eq!(checkin["opencheckindatatype"], 3);
        assert_eq!(checkin["useridlist"][0], "user");

        let range = serde_json::to_value(WorkCheckinDateRangeRequest {
            starttime: 1_800_000_000,
            endtime: 1_800_086_400,
            useridlist: vec!["user".to_string()],
        })
        .unwrap();
        assert_eq!(range["starttime"], 1_800_000_000);

        let schedule = serde_json::to_value(WorkCheckinSetScheduleListRequest {
            groupid: 1,
            items: vec![WorkCheckinSetScheduleItem {
                userid: "user".to_string(),
                day: 16,
                schedule_id: 2,
            }],
            yearmonth: 202607,
        })
        .unwrap();
        assert_eq!(schedule["groupid"], 1);
        assert_eq!(schedule["items"][0]["userid"], "user");
        assert_eq!(schedule["items"][0]["schedule_id"], 2);

        let face = serde_json::to_value(WorkCheckinUserFaceRequest {
            userid: "user".to_string(),
            userface: "base64-image".to_string(),
        })
        .unwrap();
        assert_eq!(face["userid"], "user");
        assert!(face.get("userID").is_none());

        let option = serde_json::to_value(WorkCheckinOptionMutationRequest {
            effective_now: Some(true),
            group: WorkCheckinGroup {
                grouptype: Some(1),
                groupid: Some(1),
                groupname: Some("default".to_string()),
                checkindate: vec![WorkCheckinDateRule {
                    workdays: vec![1, 2, 3, 4, 5],
                    checkintime: vec![WorkCheckinTime {
                        work_sec: Some(32_400),
                        off_work_sec: Some(61_200),
                        ..WorkCheckinTime::default()
                    }],
                    flex_time: Some(300),
                    noneed_offwork: Some(false),
                    limit_aheadtime: None,
                    allow_flex: None,
                    flex_on_duty_time: None,
                    flex_off_duty_time: None,
                    max_allow_arrive_early: None,
                    max_allow_arrive_late: None,
                    late_rule: None,
                    extra: Value::Null,
                }],
                range: vec![WorkCheckinRange {
                    partyid: vec!["2".to_string()],
                    userid: vec!["user".to_string()],
                    tagid: Vec::new(),
                    extra: Value::Null,
                }],
                ot_info_v2: Some(WorkCheckinOvertimeRuleV2 {
                    workdayconf: WorkCheckinOvertimeWorkdayConfig {
                        allow_ot: true,
                        compensation_type: 1,
                        extra: Value::Null,
                    },
                    extra: Value::Null,
                }),
                buka_remind: Some(WorkCheckinCorrectionReminder {
                    open_remind: true,
                    buka_remind_day: Some(1),
                    buka_remind_month: None,
                    extra: Value::Null,
                }),
                ..WorkCheckinGroup::default()
            },
        })
        .unwrap();
        assert_eq!(option["effective_now"], true);
        assert_eq!(option["group"]["groupname"], "default");
        assert_eq!(option["group"]["range"][0]["party_id"][0], "2");
        assert_eq!(option["group"]["range"][0]["userid"][0], "user");
        assert_eq!(
            option["group"]["checkindate"][0]["checkintime"][0]["work_sec"],
            32_400
        );
        assert_eq!(option["group"]["ot_info_v2"]["workdayconf"]["type"], 1);
        assert_eq!(option["group"]["buka_remind"]["buka_remind_day"], 1);

        let apply = serde_json::to_value(WorkApprovalApplyEventRequest {
            creator_userid: "user".to_string(),
            template_id: "template".to_string(),
            use_template_approver: Some(1),
            approver: vec![WorkApprovalApprover {
                attr: 1,
                userid: vec!["manager".to_string()],
            }],
            notifyer: vec!["notify".to_string()],
            notify_type: Some(1),
            apply_data: WorkApprovalApplyData {
                contents: vec![WorkApprovalContent {
                    control: "Text".to_string(),
                    id: "Text-1".to_string(),
                    title: Vec::new(),
                    value: WorkApprovalControlValue {
                        text: Some("hi".to_string()),
                        ..WorkApprovalControlValue::default()
                    },
                    display: None,
                    require: None,
                    extra: Value::Null,
                }],
                extra: Value::Null,
            },
            summary_list: vec![WorkApprovalSummary {
                summary_info: vec![WorkApprovalLocalizedText {
                    text: "hi".to_string(),
                    lang: "zh_CN".to_string(),
                }],
            }],
        })
        .unwrap();
        assert_eq!(apply["creator_userid"], "user");
        assert_eq!(apply["approver"][0]["userid"][0], "manager");
        assert_eq!(apply["summary_list"][0]["summary_info"][0]["text"], "hi");

        let info = serde_json::to_value(WorkApprovalInfoRequest {
            starttime: 1_800_000_000,
            endtime: 1_800_086_400,
            new_cursor: None,
            size: 100,
            filters: vec![WorkApprovalInfoFilter {
                key: "template_id".to_string(),
                value: "template".to_string(),
            }],
        })
        .unwrap();
        assert!(info.get("new_cursor").is_none());
        assert_eq!(info["filters"][0]["key"], "template_id");

        let data = serde_json::to_value(WorkApprovalDataRequest {
            starttime: 1_800_000_000,
            endtime: 1_800_086_400,
            next_spnum: 10,
        })
        .unwrap();
        assert_eq!(data["next_spnum"], 10);

        let quota = serde_json::to_value(WorkVacationQuotaUpdateRequest {
            userid: "user".to_string(),
            vacation_id: 1,
            leftduration: 3600,
            time_attr: 0,
            remarks: Some("annual adjustment".to_string()),
        })
        .unwrap();
        assert_eq!(quota["userid"], "user");
        assert_eq!(quota["vacation_id"], 1);
        assert_eq!(quota["leftduration"], 3600);
        assert_eq!(quota["remarks"], "annual adjustment");
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
            "endflag": 0,
            "trace_id": "journal-list"
        }))
        .unwrap();
        assert_eq!(records.journaluuid_list[0], "journal-1");
        assert_eq!(records.next_cursor, Some(10));
        assert_eq!(records.endflag, Some(0));
        assert_eq!(records.extra["trace_id"], "journal-list");

        let detail: WorkJournalRecordDetailResponse = serde_json::from_value(json!({
            "trace_id": "journal-detail",
            "info": {
                "journal_uuid": "journal-1",
                "template_name": "Daily",
                "report_time": 1_800_000_000,
                "submitter": {
                    "userid": "user",
                    "display_name": "Reporter"
                },
                "receivers": [{ "userid": "manager" }],
                "readed_receivers": [{ "userid": "manager" }],
                "apply_data": {
                    "contents": [{
                        "control": "Text",
                        "id": "Text-1",
                        "title": [{ "text": "Work", "lang": "en" }],
                        "value": {
                            "text": "Completed",
                            "control_version": 2
                        }
                    }],
                    "form_revision": 3
                },
                "comments": [{
                    "commentid": 100,
                    "tocommentid": 0,
                    "comment_userinfo": {
                        "userid": "manager",
                        "department_name": "Management"
                    },
                    "content": "Good",
                    "comment_time": 1_800_000_100,
                    "comment_source": "mobile"
                }],
                "form_version": 2
            }
        }))
        .unwrap();
        assert_eq!(detail.extra["trace_id"], "journal-detail");
        let detail = detail.info.unwrap();
        assert_eq!(detail.journal_uuid.as_deref(), Some("journal-1"));
        assert_eq!(
            detail.submitter.as_ref().map(|user| user.userid.as_str()),
            Some("user")
        );
        assert_eq!(
            detail
                .apply_data
                .as_ref()
                .and_then(|data| data.contents[0].value.text.as_deref()),
            Some("Completed")
        );
        assert_eq!(
            detail.apply_data.as_ref().unwrap().contents[0].value.extra["control_version"],
            2
        );
        assert_eq!(detail.comments[0].comment_userinfo.userid, "manager");
        assert_eq!(detail.comments[0].content, "Good");
        assert_eq!(detail.comments[0].extra["comment_source"], "mobile");
        assert_eq!(detail.extra["form_version"], 2);

        let stats: WorkJournalStatListResponse = serde_json::from_value(json!({
            "trace_id": "journal-stat",
            "stat_list": [{
                "template_id": "template-1",
                "template_name": "Daily",
                "report_range": {
                    "user_list": [{ "userid": "user-1", "name": "Reporter" }],
                    "party_list": [{ "open_partyid": "party-1" }],
                    "tag_list": [{ "open_tagid": "tag-1" }],
                    "range_version": 2
                },
                "white_range": {
                    "user_list": [],
                    "party_list": [],
                    "tag_list": []
                },
                "receivers": {
                    "user_list": [{ "userid": "manager" }],
                    "tag_list": [],
                    "leader_list": [{ "level": 1, "scope": "direct" }]
                },
                "cycle_begin_time": 1_800_000_000,
                "cycle_end_time": 1_800_086_400,
                "stat_begin_time": 1_800_000_000,
                "stat_end_time": 1_800_086_000,
                "report_list": [{
                    "user": { "userid": "user-1" },
                    "itemlist": [{
                        "journaluuid": "journal-1",
                        "reporttime": 1_800_010_000,
                        "flag": 0,
                        "source": "app"
                    }]
                }],
                "unreport_list": [{
                    "user": { "userid": "user-2" },
                    "itemlist": [{
                        "journaluuid": "",
                        "reporttime": 1_800_000_000,
                        "flag": 0
                    }]
                }],
                "report_type": 2,
                "stat_version": 3
            }]
        }))
        .unwrap();
        assert_eq!(stats.extra["trace_id"], "journal-stat");
        let stats = &stats.stat_list[0];
        assert_eq!(stats.template_name.as_deref(), Some("Daily"));
        assert_eq!(
            stats.report_range.as_ref().unwrap().user_list[0].userid,
            "user-1"
        );
        assert_eq!(stats.receivers.as_ref().unwrap().leader_list[0].level, 1);
        assert_eq!(stats.report_list[0].itemlist[0].journaluuid, "journal-1");
        assert_eq!(stats.report_list[0].itemlist[0].extra["source"], "app");
        assert_eq!(stats.unreport_list[0].user.userid, "user-2");
        assert_eq!(stats.report_type, Some(2));
        assert_eq!(stats.extra["stat_version"], 3);

        let schedule_add: WorkScheduleAddResponse = serde_json::from_value(json!({
            "schedule_id": "schedule-1",
            "request_id": "schedule-add"
        }))
        .unwrap();
        assert_eq!(schedule_add.schedule_id.as_deref(), Some("schedule-1"));
        assert_eq!(schedule_add.extra["request_id"], "schedule-add");

        let schedule_get: WorkScheduleGetResponse = serde_json::from_value(json!({
            "next_cursor": "schedule-cursor",
            "schedule_list": [{
                "schedule_id": "schedule-1",
                "sequence": 100,
                "admins": ["admin"],
                "summary": "Daily",
                "organizer": "user",
                "start_time": 1_800_000_000,
                "end_time": 1_800_003_600,
                "attendees": [{
                    "userid": "user",
                    "response_status": 1,
                    "attendee_role": "required"
                }],
                "reminders": {
                    "is_remind": 1,
                    "remind_before_event_secs": 3600,
                    "remind_time_diffs": [-3600],
                    "is_repeat": 1,
                    "repeat_type": 7,
                    "repeat_until": 1_900_000_000,
                    "is_custom_repeat": 1,
                    "repeat_interval": 1,
                    "repeat_day_of_week": [3, 7],
                    "repeat_day_of_month": [10, 21],
                    "timezone": 8,
                    "exclude_time_list": [{
                        "start_time": 1_800_086_400,
                        "exclude_reason": "holiday"
                    }],
                    "reminder_version": 2
                },
                "cal_id": "wc100",
                "status": 1,
                "timezone": "Asia/Shanghai"
            }]
        }))
        .unwrap();
        assert_eq!(
            schedule_get.schedule_list[0].schedule_id.as_deref(),
            Some("schedule-1")
        );
        assert_eq!(
            schedule_get.schedule_list[0].summary.as_deref(),
            Some("Daily")
        );
        assert_eq!(schedule_get.schedule_list[0].admins[0], "admin");
        assert_eq!(schedule_get.schedule_list[0].attendees[0].userid, "user");
        assert_eq!(
            schedule_get.schedule_list[0].attendees[0].response_status,
            Some(1)
        );
        assert_eq!(
            schedule_get.schedule_list[0].attendees[0].extra["attendee_role"],
            "required"
        );
        let reminders = schedule_get.schedule_list[0]
            .reminders
            .as_ref()
            .expect("schedule reminders");
        assert_eq!(reminders.repeat_day_of_week, vec![3, 7]);
        assert_eq!(
            reminders.exclude_time_list[0].extra["exclude_reason"],
            "holiday"
        );
        assert_eq!(reminders.extra["reminder_version"], 2);
        assert_eq!(
            schedule_get.schedule_list[0].cal_id.as_deref(),
            Some("wc100")
        );
        assert_eq!(schedule_get.schedule_list[0].sequence, Some(100));
        assert_eq!(schedule_get.schedule_list[0].status, Some(1));
        assert_eq!(schedule_get.extra["next_cursor"], "schedule-cursor");
        assert_eq!(
            schedule_get.schedule_list[0].extra["timezone"],
            "Asia/Shanghai"
        );
    }

    #[test]
    fn deserializes_work_oa_checkin_approval_and_vacation_responses() {
        let corp_option: WorkCheckinCorpOptionResponse = serde_json::from_value(json!({
            "trace_id": "checkin-corp",
            "group": [{
                "grouptype": 1,
                "groupid": 1,
                "groupname": "Default",
                "checkindate": [{
                    "workdays": [1, 2, 3],
                    "checkintime": [{
                        "time_id": 1,
                        "work_sec": 32400,
                        "off_work_sec": 61200,
                        "allow_rest": true,
                        "rest_begin_time": 43200,
                        "rest_end_time": 46800
                    }],
                    "flex_time": 30,
                    "rule_version": 2
                }],
                "spe_workdays": [{
                    "timestamp": 1_800_000_000,
                    "notes": "release",
                    "checkintime": [{ "work_sec": 36000, "off_work_sec": 64800 }]
                }],
                "spe_offdays": [{
                    "timestamp": 1_800_086_400,
                    "notes": "holiday",
                    "checkintime": []
                }],
                "sync_holidays": true,
                "need_photo": true,
                "wifimac_infos": [{
                    "wifiname": "Office",
                    "wifimac": "00:11:22:33:44:55",
                    "wifi_version": 2
                }],
                "loc_infos": [{
                    "lat": 30547030,
                    "lng": 104062890,
                    "loc_title": "Office",
                    "loc_detail": "Building A",
                    "distance": 300
                }],
                "range": [{
                    "partyid": ["2"],
                    "userid": ["user"],
                    "tagid": [3]
                }],
                "reporterinfo": {
                    "reporters": [{ "userid": "manager", "tagid": 4 }],
                    "updatetime": 1_800_000_000
                },
                "schedulelist": [{
                    "schedule_id": 2,
                    "schedule_name": "Day",
                    "time_section": [{
                        "work_sec": 32400,
                        "off_work_sec": 61200
                    }],
                    "late_rule": {
                        "allow_offwork_after_time": true,
                        "timerules": [{
                            "offwork_after_time": 3600,
                            "onwork_flex_time": 1800
                        }]
                    }
                }],
                "ot_info": {
                    "type": 1,
                    "allow_ot_workingday": true,
                    "otcheckinfo": {
                        "ot_workingday_time_start": 64800,
                        "ot_workingday_restinfo": {
                            "type": 2,
                            "fix_time_rule": {
                                "fix_time_begin_sec": 43200,
                                "fix_time_end_sec": 46800
                            },
                            "cal_ottime_rule": {
                                "items": [{ "ot_time": 18000, "rest_time": 3600 }]
                            }
                        }
                    }
                },
                "group_owner": "admin"
            }]
        }))
        .unwrap();
        assert_eq!(corp_option.extra["trace_id"], "checkin-corp");
        assert_eq!(corp_option.group[0].groupid, Some(1));
        assert_eq!(corp_option.group[0].groupname.as_deref(), Some("Default"));
        assert_eq!(corp_option.group[0].checkindate[0].flex_time, Some(30));
        assert_eq!(
            corp_option.group[0].checkindate[0].checkintime[0].work_sec,
            Some(32_400)
        );
        assert_eq!(
            corp_option.group[0].spe_workdays[0].notes.as_deref(),
            Some("release")
        );
        assert_eq!(
            corp_option.group[0].wifimac_infos[0].extra["wifi_version"],
            2
        );
        assert_eq!(corp_option.group[0].loc_infos[0].distance, 300);
        assert_eq!(corp_option.group[0].range[0].userid[0], "user");
        assert_eq!(
            corp_option.group[0]
                .reporterinfo
                .as_ref()
                .expect("reporter info")
                .reporters[0]
                .userid,
            "manager"
        );
        assert_eq!(corp_option.group[0].schedulelist[0].schedule_id, 2);
        assert_eq!(
            corp_option.group[0]
                .ot_info
                .as_ref()
                .expect("overtime rule")
                .otcheckinfo
                .as_ref()
                .expect("overtime check rule")
                .ot_workingday_time_start,
            Some(64_800)
        );
        assert_eq!(corp_option.group[0].extra["group_owner"], "admin");
        assert_eq!(corp_option.group[0].checkindate[0].extra["rule_version"], 2);

        let option: WorkCheckinOptionResponse = serde_json::from_value(json!({
            "trace_id": "checkin-option",
            "info": [{
                "userid": "user",
                "group": {
                    "grouptype": 1,
                    "groupid": 1,
                    "groupname": "Default",
                    "checkindate": [],
                    "option_version": 2
                },
                "option_source": "rule"
            }]
        }))
        .unwrap();
        assert_eq!(option.extra["trace_id"], "checkin-option");
        assert_eq!(option.info[0].userid, "user");
        assert_eq!(option.info[0].group.groupid, Some(1));
        assert_eq!(option.info[0].group.extra["option_version"], 2);
        assert_eq!(option.info[0].extra["option_source"], "rule");

        let record: WorkCheckinRecordResponse = serde_json::from_value(json!({
            "has_more": false,
            "checkindata": [{
                "userid": "user",
                "checkin_type": "上班打卡",
                "checkin_time": 1_800_000_000,
                "wifimac": "00:11:22:33:44:55",
                "mediaids": ["media-1"],
                "sch_checkin_time": 1_799_999_900,
                "groupid": 1,
                "schedule_id": 2,
                "timeline_id": 3,
                "lat": 30547030,
                "lng": 104062890,
                "deviceid": "device",
                "device_id": "device"
            }]
        }))
        .unwrap();
        assert_eq!(record.extra["has_more"], false);
        assert_eq!(record.checkindata[0].userid.as_deref(), Some("user"));
        assert_eq!(
            record.checkindata[0].checkin_type.as_deref(),
            Some("上班打卡")
        );
        assert_eq!(record.checkindata[0].checkin_time, Some(1_800_000_000));
        assert_eq!(record.checkindata[0].mediaids[0], "media-1");
        assert_eq!(record.checkindata[0].deviceid.as_deref(), Some("device"));
        assert_eq!(record.checkindata[0].schedule_id, Some(2));
        assert_eq!(record.checkindata[0].extra["device_id"], "device");

        let day: WorkCheckinDataResponse = serde_json::from_value(json!({
            "trace_id": "checkin-data",
            "datas": [{
                "base_info": {
                    "date": 20260717,
                    "record_type": 1,
                    "name": "Alice",
                    "name_ex": "Alice",
                    "departs_name": "Engineering",
                    "acctid": "user",
                    "day_type": 1,
                    "rule_info": {
                        "groupid": 1,
                        "groupname": "Default",
                        "scheduleid": 2,
                        "schedulename": "Day",
                        "checkintime": [{ "work_sec": 32400, "off_work_sec": 61200 }]
                    }
                },
                "summary_info": {
                    "checkin_count": 2,
                    "regular_work_sec": 28800,
                    "standard_work_sec": 28800,
                    "earliest_time": 1_800_000_000,
                    "lastest_time": 1_800_028_800,
                    "summary_version": 2
                },
                "exception_infos": [{ "count": 1, "duration": 300, "exception": 1 }],
                "holiday_infos": [{
                    "sp_number": "202607170001",
                    "sp_title": {
                        "data": [{ "text": "Annual leave", "lang": "zh_CN" }]
                    },
                    "sp_description": {
                        "data": [{ "text": "Half day", "lang": "zh_CN" }]
                    }
                }],
                "sp_items": [{
                    "count": 1,
                    "duration": 14400,
                    "time_type": 0,
                    "type": 1,
                    "vacation_id": 1,
                    "name": "Annual leave"
                }],
                "ot_info": {
                    "ot_status": 1,
                    "ot_duration": 3600,
                    "exception_duration": [300]
                },
                "daily_version": 2
            }]
        }))
        .unwrap();
        assert_eq!(day.extra["trace_id"], "checkin-data");
        assert_eq!(day.datas[0].base_info.acctid, "user");
        assert_eq!(day.datas[0].base_info.rule_info.scheduleid, 2);
        assert_eq!(day.datas[0].summary_info.checkin_count, 2);
        assert_eq!(day.datas[0].summary_info.extra["summary_version"], 2);
        assert_eq!(
            day.datas[0].holiday_infos[0].sp_title.data[0].text,
            "Annual leave"
        );
        assert_eq!(day.datas[0].sp_items[0].vacation_id, 1);
        assert_eq!(
            day.datas[0]
                .ot_info
                .as_ref()
                .expect("day overtime")
                .ot_duration,
            3600
        );
        assert_eq!(day.datas[0].extra["daily_version"], 2);

        let month: WorkCheckinMonthDataResponse = serde_json::from_value(json!({
            "datas": [{
                "base_info": {
                    "record_type": 2,
                    "name": "Alice",
                    "name_ex": "Alice",
                    "departs_name": "Engineering",
                    "acctid": "user",
                    "rule_info": { "groupid": 1, "groupname": "Default" }
                },
                "summary_info": {
                    "work_days": 22,
                    "except_days": 1,
                    "regular_days": 21,
                    "regular_work_sec": 604800,
                    "standard_work_sec": 633600,
                    "rest_days": 8,
                    "month_version": 2
                },
                "exception_infos": [{ "count": 1, "duration": 300, "exception": 1 }],
                "sp_items": [],
                "overwork_info": {
                    "workday_over_sec": 3600,
                    "holidays_over_sec": 0,
                    "restdays_over_sec": 7200,
                    "workdays_over_as_vacation": 0,
                    "workdays_over_as_money": 3600,
                    "restdays_over_as_vacation": 7200,
                    "restdays_over_as_money": 0,
                    "holidays_over_as_vacation": 0,
                    "holidays_over_as_money": 0
                }
            }]
        }))
        .unwrap();
        assert_eq!(month.datas[0].base_info.rule_info.groupid, 1);
        assert_eq!(month.datas[0].summary_info.work_days, 22);
        assert_eq!(month.datas[0].summary_info.extra["month_version"], 2);
        assert_eq!(
            month.datas[0]
                .overwork_info
                .as_ref()
                .expect("month overtime")
                .restdays_over_sec,
            Some(7200)
        );

        let sparse_month: WorkCheckinMonthDataResponse = serde_json::from_value(json!({
            "datas": [{
                "base_info": {
                    "record_type": 1,
                    "name": "Alice",
                    "name_ex": "Alice",
                    "departs_name": "Engineering",
                    "acctid": "user",
                    "rule_info": { "groupid": 1, "groupname": "Default" }
                },
                "summary_info": {
                    "work_days": 3,
                    "except_days": 1,
                    "regular_work_sec": 31,
                    "standard_work_sec": 29040
                },
                "exception_infos": [],
                "sp_items": [],
                "overwork_info": { "workday_over_sec": 10800 }
            }]
        }))
        .unwrap();
        assert_eq!(sparse_month.datas[0].summary_info.regular_days, None);
        assert_eq!(
            sparse_month.datas[0]
                .overwork_info
                .as_ref()
                .expect("sparse month overtime")
                .workday_over_sec,
            Some(10800)
        );

        let schedule: WorkCheckinScheduleListResponse = serde_json::from_value(json!({
            "trace_id": "checkin-schedule",
            "schedule_list": [{
                "userid": "user",
                "yearmonth": 202607,
                "groupid": 2,
                "groupname": "Shift",
                "schedule": {
                    "scheduleList": [{
                        "day": 17,
                        "schedule_info": {
                            "schedule_id": 1,
                            "schedule_name": "Morning",
                            "time_section": [{
                                "id": 1,
                                "work_sec": 32400,
                                "off_work_sec": 61200,
                                "remind_work_sec": 31800,
                                "remind_off_work_sec": 61200,
                                "section_version": 2
                            }]
                        }
                    }],
                    "schedule_version": 2
                },
                "shift_name": "morning"
            }]
        }))
        .unwrap();
        assert_eq!(schedule.extra["trace_id"], "checkin-schedule");
        assert_eq!(schedule.schedule_list[0].userid, "user");
        assert_eq!(schedule.schedule_list[0].groupid, 2);
        assert_eq!(
            schedule.schedule_list[0].schedule.schedule_list[0]
                .schedule_info
                .schedule_id,
            1
        );
        assert_eq!(
            schedule.schedule_list[0].schedule.schedule_list[0]
                .schedule_info
                .time_section[0]
                .extra["section_version"],
            2
        );
        assert_eq!(
            schedule.schedule_list[0].schedule.extra["schedule_version"],
            2
        );
        assert_eq!(schedule.schedule_list[0].extra["shift_name"], "morning");

        let template: WorkApprovalTemplateDetailResponse = serde_json::from_value(json!({
            "template_names": [{ "text": "Leave", "lang": "zh_CN" }],
            "template_content": {
                "controls": [{
                    "property": {
                        "control": "Selector",
                        "id": "Selector-1",
                        "title": [{ "text": "Leave type", "lang": "zh_CN" }],
                        "placeholder": [],
                        "require": 1,
                        "un_print": 0
                    },
                    "config": {
                        "selector": {
                            "type": "single",
                            "options": [{
                                "key": "annual",
                                "value": [{ "text": "Annual", "lang": "zh_CN" }]
                            }],
                            "selector_version": 2
                        }
                    },
                    "control_version": 3
                }],
                "content_version": 4
            },
            "template_version": 2
        }))
        .unwrap();
        assert_eq!(template.template_names[0].text, "Leave");
        let template_content = template
            .template_content
            .as_ref()
            .expect("approval template content");
        assert_eq!(template_content.controls[0].property.id, "Selector-1");
        let selector = template_content.controls[0]
            .config
            .selector
            .as_ref()
            .expect("approval selector");
        assert_eq!(selector.selector_type, "single");
        assert_eq!(selector.options[0].value[0].text, "Annual");
        assert_eq!(selector.extra["selector_version"], 2);
        assert_eq!(template_content.controls[0].extra["control_version"], 3);
        assert_eq!(template_content.extra["content_version"], 4);
        assert_eq!(template.extra["template_version"], 2);

        let apply: WorkApprovalApplyEventResponse = serde_json::from_value(json!({
            "sp_no": "202607160001",
            "request_id": "approval-apply"
        }))
        .unwrap();
        assert_eq!(apply.sp_no.as_deref(), Some("202607160001"));
        assert_eq!(apply.extra["request_id"], "approval-apply");

        let info: WorkApprovalInfoResponse = serde_json::from_value(json!({
            "sp_no_list": ["202607160001"],
            "new_next_cursor": "cursor",
            "total": 1
        }))
        .unwrap();
        assert_eq!(info.sp_no_list[0], "202607160001");
        assert_eq!(info.extra["total"], 1);

        let detail: WorkApprovalDetailResponse = serde_json::from_value(json!({
            "info": {
                "sp_no": "202607160001",
                "sp_name": "Leave",
                "sp_status": 2,
                "template_id": "template",
                "apply_time": 1_800_000_000,
                "applyer": { "userid": "user", "partyid": "2" },
                "sp_record": [{
                    "sp_status": 2,
                    "approverattr": 1,
                    "details": [{
                        "approver": { "userid": "manager" },
                        "speech": "approved",
                        "sp_status": 2,
                        "sptime": 1_800_000_100,
                        "media_id": ["media-1"]
                    }]
                }],
                "notifyer": [{ "userid": "notify" }],
                "apply_data": {
                    "contents": [{
                        "control": "Table",
                        "id": "Table-1",
                        "title": [{ "text": "Items", "lang": "zh_CN" }],
                        "value": {
                            "children": [{
                                "list": [{
                                    "control": "Money",
                                    "id": "Money-1",
                                    "value": { "new_money": "12.50" },
                                    "display": 1,
                                    "require": 1
                                }]
                            }],
                            "stat_field": [{
                                "id": "Money-1",
                                "title": [{ "text": "Total", "lang": "zh_CN" }],
                                "value": "12.50",
                                "exp_type": 1,
                                "control": "Money"
                            }]
                        },
                        "display": 1,
                        "require": 1
                    }]
                },
                "comments": [{
                    "comment_user_info": { "userid": "manager" },
                    "comment_time": 1_800_000_200,
                    "comment_content": "done",
                    "comment_id": "comment-1",
                    "media_id": []
                }],
                "process_list": {
                    "node_list": [{
                        "node_type": 1,
                        "sp_status": 2,
                        "apv_rel": 2,
                        "sub_node_list": [{
                            "userid": "manager",
                            "sp_yj": 2,
                            "sptime": 1_800_000_100,
                            "media_ids": []
                        }]
                    }]
                },
                "batch_applyer": [],
                "detail_version": 2
            },
            "trace_id": "approval-detail"
        }))
        .unwrap();
        assert_eq!(detail.extra["trace_id"], "approval-detail");
        let detail = detail.info.expect("approval detail");
        assert_eq!(detail.sp_no, "202607160001");
        assert_eq!(detail.applyer.partyid.as_deref(), Some("2"));
        assert_eq!(detail.sp_record[0].details[0].media_id[0], "media-1");
        assert_eq!(
            detail.apply_data.contents[0].value.children[0].list[0]
                .value
                .new_money
                .as_deref(),
            Some("12.50")
        );
        assert_eq!(detail.comments[0].comment_id, "comment-1");
        assert_eq!(
            detail
                .process_list
                .as_ref()
                .expect("approval process")
                .node_list[0]
                .sub_node_list[0]
                .userid,
            "manager"
        );
        assert_eq!(detail.extra["detail_version"], 2);

        let data: WorkApprovalDataResponse = serde_json::from_value(json!({
            "count": 1,
            "total": 1,
            "next_spnum": 2,
            "data": [{
                "spname": "Expense",
                "apply_name": "Alice",
                "apply_org": "Finance",
                "approval_name": ["Manager"],
                "notify_name": ["Auditor"],
                "sp_status": 2,
                "sp_num": 1,
                "mediaids": ["media-1"],
                "apply_time": 1_800_000_000,
                "apply_user_id": "alice",
                "expense": {
                    "expense_type": 4,
                    "reason": "travel",
                    "item": [{
                        "expenseitem_type": 2,
                        "time": 1_800_000_000,
                        "sums": 12.5,
                        "reason": "taxi"
                    }]
                },
                "comm": { "apply_data": "{}" },
                "apply_data": [{
                    "id": "Text-1",
                    "type": "text",
                    "value": "travel",
                    "title": "Reason"
                }]
            }],
            "has_more": true
        }))
        .unwrap();
        assert_eq!(data.next_spnum, Some(2));
        assert_eq!(data.data[0].spname, "Expense");
        assert_eq!(
            data.data[0].expense.as_ref().expect("legacy expense").item[0].sums,
            12.5
        );
        assert_eq!(data.data[0].apply_data[0].field_type, "text");
        assert_eq!(data.extra["has_more"], true);

        let vacation: WorkVacationConfigResponse = serde_json::from_value(json!({
            "lists": [{
                "id": 1,
                "name": "Annual Leave",
                "time_attr": 0,
                "duration_type": 0,
                "quota_attr": {
                    "type": 1,
                    "autoreset_time": 1_900_000_000,
                    "autoreset_duration": 432000,
                    "quota_rule_type": 1,
                    "quota_rules": {
                        "list": [{
                            "quota": 432000,
                            "begin": 0,
                            "end": 1,
                            "rule_version": 2
                        }],
                        "based_on_actual_work_time": true
                    },
                    "at_entry_date": true,
                    "auto_reset_month_day": 101
                },
                "perday_duration": 86400,
                "is_newovertime": 0,
                "enter_comp_time_limit": 0,
                "expire_rule": {
                    "type": 2,
                    "duration": 2,
                    "date": { "month": 12, "day": 31 },
                    "extern_duration_enable": false,
                    "extern_duration": { "month": 1, "day": 31 }
                },
                "vacation_version": 2
            }],
            "config_version": 2
        }))
        .unwrap();
        assert_eq!(vacation.lists[0].name, "Annual Leave");
        let policy = vacation.lists[0]
            .quota_attr
            .as_ref()
            .expect("vacation quota policy");
        assert_eq!(policy.policy_type, 1);
        assert_eq!(
            policy
                .quota_rules
                .as_ref()
                .expect("vacation quota rules")
                .list[0]
                .quota,
            432000
        );
        assert_eq!(
            policy
                .quota_rules
                .as_ref()
                .expect("vacation quota rules")
                .list[0]
                .extra["rule_version"],
            2
        );
        assert_eq!(
            vacation.lists[0]
                .expire_rule
                .as_ref()
                .expect("vacation expiration")
                .date
                .as_ref()
                .expect("vacation expiration date")
                .month,
            12
        );
        assert_eq!(vacation.lists[0].extra["vacation_version"], 2);
        assert_eq!(vacation.extra["config_version"], 2);

        let quota: WorkVacationQuotaResponse = serde_json::from_value(json!({
            "lists": [{
                "id": 1,
                "assignduration": 604800,
                "usedduration": 86400,
                "leftduration": 518400,
                "vacationname": "Annual Leave",
                "real_assignduration": 604800,
                "quota_version": 2
            }],
            "quota_trace": "vacation-quota"
        }))
        .unwrap();
        assert_eq!(quota.lists[0].leftduration, 518400);
        assert_eq!(quota.lists[0].real_assignduration, Some(604800));
        assert_eq!(quota.lists[0].extra["quota_version"], 2);
        assert_eq!(quota.extra["quota_trace"], "vacation-quota");
    }

    #[test]
    fn serializes_work_oa_linked_meetingroom_and_wedoc_content_requests() {
        let schedule = serde_json::to_value(WorkMeetingRoomBookByScheduleRequest {
            meetingroom_id: 7,
            schedule_id: "schedule-1".to_string(),
            booker: "user".to_string(),
        })
        .unwrap();
        assert_eq!(schedule["meetingroom_id"], 7);
        assert_eq!(schedule["schedule_id"], "schedule-1");

        let meeting = serde_json::to_value(WorkMeetingRoomBookByMeetingRequest {
            meetingroom_id: 7,
            meetingid: "meeting-1".to_string(),
            booker: "user".to_string(),
        })
        .unwrap();
        assert_eq!(meeting["meetingid"], "meeting-1");

        let booking = serde_json::to_value(WorkMeetingRoomBookingByIdRequest {
            meetingroom_id: 7,
            booking_id: "booking-1".to_string(),
        })
        .unwrap();
        assert_eq!(booking["booking_id"], "booking-1");

        let content = serde_json::to_value(WorkWeDocGetContentDataRequest {
            docid: "doc-1".to_string(),
            extra: json!({ "cursor": "next" }),
        })
        .unwrap();
        assert_eq!(content["docid"], "doc-1");
        assert_eq!(content["cursor"], "next");

        let modify = serde_json::to_value(WorkWeDocModifyContentRequest {
            docid: "doc-1".to_string(),
            requests: Some(json!([{ "insert_text": { "text": "hello" } }])),
            extra: Value::Null,
        })
        .unwrap();
        assert_eq!(modify["requests"][0]["insert_text"]["text"], "hello");

        let admin = serde_json::to_value(WorkWeDocAdminRequest {
            docid: "doc-1".to_string(),
            userid: Some("user".to_string()),
            open_userid: None,
            account_type: Some(1),
            extra: json!({ "source": "sdk" }),
        })
        .unwrap();
        assert_eq!(admin["userid"], "user");
        assert_eq!(admin["type"], 1);
        assert_eq!(admin["source"], "sdk");

        let auth = serde_json::to_value(WorkWeDocSmartSheetModifyAuthRequest {
            docid: "doc-1".to_string(),
            sheet_id: Some("sheet-1".to_string()),
            auth_info: Some(json!({ "field_auth": [] })),
            extra: Value::Null,
        })
        .unwrap();
        assert_eq!(auth["sheet_id"], "sheet-1");
        assert!(auth["auth_info"]["field_auth"].is_array());
    }

    #[test]
    fn deserializes_work_oa_linked_meetingroom_and_wedoc_content_responses() {
        let linked: WorkMeetingRoomLinkedBookResponse = serde_json::from_value(json!({
            "booking_id": "booking-1",
            "schedule_id": "schedule-1",
            "conflict_date": [1_800_000_000],
            "approval_status": "pending"
        }))
        .unwrap();
        assert_eq!(linked.booking_id.as_deref(), Some("booking-1"));
        assert_eq!(linked.conflict_date, vec![1_800_000_000]);
        assert_eq!(linked.extra["approval_status"], "pending");

        let booking: WorkMeetingRoomBookingByIdResponse = serde_json::from_value(json!({
            "meetingroom_id": 7,
            "schedule": {
                "booking_id": "booking-1",
                "schedule_id": "schedule-1",
                "start_time": 1_800_000_000,
                "end_time": 1_800_003_600,
                "booker": "user",
                "status": 0,
                "subject": "Weekly"
            },
            "request_id": "booking-query"
        }))
        .unwrap();
        let schedule = booking.schedule.expect("booking schedule");
        assert_eq!(schedule.booker.as_deref(), Some("user"));
        assert_eq!(schedule.extra["subject"], "Weekly");
        assert_eq!(booking.extra["request_id"], "booking-query");

        let content: WorkWeDocContentDataResponse = serde_json::from_value(json!({
            "docid": "doc-1",
            "doc_content": { "blocks": [{ "text": "hello" }] },
            "has_more": true,
            "next_cursor": "next",
            "version": 2
        }))
        .unwrap();
        assert_eq!(
            content.effective_content().expect("document content")["blocks"][0]["text"],
            "hello"
        );
        assert_eq!(content.extra["version"], 2);

        let image: WorkWeDocImageUploadResponse = serde_json::from_value(json!({
            "url": "https://example.test/image.png",
            "media_id": "media-1",
            "size": 1024
        }))
        .unwrap();
        assert_eq!(
            image.effective_url(),
            Some("https://example.test/image.png")
        );
        assert_eq!(image.extra["size"], 1024);

        let admins: WorkWeDocAdminListResponse = serde_json::from_value(json!({
            "docid": "doc-1",
            "admin_list": [{
                "userid": "user",
                "type": 1,
                "expires_at": 1_900_000_000_i64
            }],
            "total": 1
        }))
        .unwrap();
        assert_eq!(admins.admin_list[0].account_type, Some(1));
        assert_eq!(admins.admin_list[0].extra["expires_at"], 1_900_000_000_i64);
        assert_eq!(admins.extra["total"], 1);

        let auth: WorkWeDocSmartSheetAuthResponse = serde_json::from_value(json!({
            "docid": "doc-1",
            "sheet_id": "sheet-1",
            "field_auth": [{ "field_id": "field-1" }],
            "policy_version": 3
        }))
        .unwrap();
        assert_eq!(
            auth.effective_auth_info().expect("sheet auth")[0]["field_id"],
            "field-1"
        );
        assert_eq!(auth.extra["policy_version"], 3);
    }

    #[test]
    fn serializes_work_oa_meeting_meetingroom_and_wedoc_requests() {
        let meeting = serde_json::to_value(WorkMeetingCreateRequest {
            creator_userid: "creator".to_string(),
            title: "Weekly".to_string(),
            meeting_start: 1_800_000_000,
            meeting_duration: 60,
            description: "sync".to_string(),
            meeting_type: 1,
            remind_time: 15,
            agentid: 100001,
            attendees: WorkMeetingAttendeesRequest {
                userids: vec!["user".to_string()],
                extra: Value::Null,
            },
        })
        .unwrap();
        assert_eq!(meeting["creator_userid"], "creator");
        assert_eq!(meeting["type"], 1);
        assert_eq!(meeting["attendees"]["userids"][0], "user");

        let update = serde_json::to_value(WorkMeetingUpdateRequest {
            meetingid: "123".to_string(),
            title: "Weekly updated".to_string(),
            meeting_start: 1_800_000_300,
            meeting_duration: 30,
            description: "sync".to_string(),
            meeting_type: 1,
            remind_time: 10,
            attendees: WorkMeetingAttendeesRequest {
                userids: vec!["user".to_string()],
                extra: Value::Null,
            },
        })
        .unwrap();
        assert_eq!(update["meetingid"], "123");
        assert_eq!(update["meeting_duration"], 30);

        let query = serde_json::to_value(WorkMeetingGetUserMeetingIdRequest {
            userid: "user".to_string(),
            cursor: "cursor".to_string(),
            begin_time: 1_800_000_000,
            end_time: 1_800_086_400,
            limit: 100,
        })
        .unwrap();
        assert_eq!(query["userid"], "user");
        assert_eq!(query["begin_time"], 1_800_000_000);

        let room = serde_json::to_value(WorkMeetingRoomAddRequest {
            name: "Room A".to_string(),
            capacity: 8,
            city: Some("Shanghai".to_string()),
            building: Some("HQ".to_string()),
            floor: Some("3".to_string()),
            equipment: vec![1, 2],
            coordinate: Some(WorkMeetingRoomCoordinate {
                longitude: "121.5".to_string(),
                latitude: "31.2".to_string(),
                extra: Value::Null,
            }),
        })
        .unwrap();
        assert_eq!(room["name"], "Room A");
        assert_eq!(room["coordinate"]["longitude"], "121.5");

        let room_edit = serde_json::to_value(WorkMeetingRoomEditRequest {
            meetingroom_id: 7,
            name: "Room B".to_string(),
            capacity: 10,
            city: Some("Shanghai".to_string()),
            building: Some("HQ".to_string()),
            floor: Some("4".to_string()),
            equipment: vec![1],
            coordinate: Some(WorkMeetingRoomCoordinate {
                longitude: "121.5".to_string(),
                latitude: "31.2".to_string(),
                extra: Value::Null,
            }),
        })
        .unwrap();
        assert_eq!(room_edit["meetingroom_id"], 7);
        assert_eq!(room_edit["capacity"], 10);

        let room_list = serde_json::to_value(WorkMeetingRoomListRequest {
            city: Some("Shanghai".to_string()),
            building: Some("HQ".to_string()),
            floor: Some("3".to_string()),
            equipment: vec![1],
        })
        .unwrap();
        assert_eq!(room_list["equipment"][0], 1);

        let booking_info = serde_json::to_value(WorkMeetingRoomGetBookingInfoRequest {
            meetingroom_id: Some(7),
            start_time: Some(1_800_000_000),
            end_time: Some(1_800_003_600),
            city: None,
            building: None,
            floor: None,
        })
        .unwrap();
        assert_eq!(booking_info["meetingroom_id"], 7);
        assert!(booking_info.get("city").is_none());

        let book = serde_json::to_value(WorkMeetingRoomBookRequest {
            meetingroom_id: 7,
            subject: Some("Weekly".to_string()),
            start_time: 1_800_000_000,
            end_time: 1_800_003_600,
            booker: "user".to_string(),
            attendees: vec!["user".to_string()],
        })
        .unwrap();
        assert_eq!(book["subject"], "Weekly");
        assert_eq!(book["attendees"][0], "user");

        let cancel = serde_json::to_value(WorkMeetingRoomCancelBookRequest {
            meeting_id: "meeting-1".to_string(),
            keep_schedule: Some(1),
        })
        .unwrap();
        assert_eq!(cancel["meeting_id"], "meeting-1");

        let form = serde_json::to_value(WorkWeDocCreateFormRequest {
            spaceid: Some("space".to_string()),
            fatherid: Some("father".to_string()),
            form_info: WorkWeDocFormInfo {
                formid: None,
                form_title: Some("Survey".to_string()),
                form_desc: None,
                form_header: None,
                form_question: Some(WorkWeDocFormQuestion {
                    items: Vec::new(),
                    extra: Value::Null,
                }),
                form_setting: None,
                repeated_id: Vec::new(),
                extra: Value::Null,
            },
        })
        .unwrap();
        assert_eq!(form["spaceid"], "space");
        assert_eq!(form["form_info"]["form_title"], "Survey");
    }

    #[test]
    fn deserializes_work_oa_meeting_meetingroom_and_wedoc_responses() {
        let meeting_create: WorkMeetingCreateResponse = serde_json::from_value(json!({
            "meetingid": 123,
            "request_id": "meeting-create"
        }))
        .unwrap();
        assert_eq!(meeting_create.meetingid, Some(123));
        assert_eq!(meeting_create.extra["request_id"], "meeting-create");

        let meeting_ids: WorkMeetingGetUserMeetingIdResponse = serde_json::from_value(json!({
            "next_cursor": "next",
            "meetingid_list": ["123"],
            "total": 1
        }))
        .unwrap();
        assert_eq!(meeting_ids.next_cursor.as_deref(), Some("next"));
        assert_eq!(meeting_ids.meetingid_list[0], "123");
        assert_eq!(meeting_ids.extra["total"], 1);

        let meeting_info: WorkMeetingGetInfoResponse = serde_json::from_value(json!({
            "creator_userid": "creator",
            "title": "Weekly",
            "reserve_meeting_start": 1_800_000_000,
            "reserve_meeting_duration": 60,
            "meeting_start": 1_800_000_000,
            "meeting_duration": 60,
            "description": "sync",
            "main_department": 1,
            "type": 1,
            "status": 2,
            "remind_time": 15,
            "attendees": {
                "member": [{
                    "userid": "user",
                    "status": 1,
                    "join_count": 2
                }],
                "external_user": [{
                    "external_userid": "external-user",
                    "status": 2
                }],
                "device": [{
                    "device_sn": "device-1",
                    "status": 1
                }],
                "attendee_count": 3
            },
            "meeting_code": "8888"
        }))
        .unwrap();
        assert_eq!(meeting_info.creator_userid.as_deref(), Some("creator"));
        assert_eq!(meeting_info.meeting_type, Some(1));
        let attendees = meeting_info.attendees.expect("meeting attendees");
        assert_eq!(attendees.member[0].userid.as_deref(), Some("user"));
        assert_eq!(attendees.member[0].status, Some(1));
        assert_eq!(attendees.member[0].extra["join_count"], 2);
        assert_eq!(
            attendees.external_user[0].external_userid.as_deref(),
            Some("external-user")
        );
        assert_eq!(attendees.device[0].device_sn.as_deref(), Some("device-1"));
        assert_eq!(attendees.extra["attendee_count"], 3);
        assert_eq!(meeting_info.extra["meeting_code"], "8888");

        let room_add: WorkMeetingRoomAddResponse = serde_json::from_value(json!({
            "meetingroom_id": 7,
            "request_id": "room-add"
        }))
        .unwrap();
        assert_eq!(room_add.meetingroom_id, Some(7));
        assert_eq!(room_add.extra["request_id"], "room-add");

        let room_list: WorkMeetingRoomListResponse = serde_json::from_value(json!({
            "total": 1,
            "meetingroom_list": [{
                "meetingroom_id": 7,
                "name": "Room A",
                "capacity": 12,
                "equipment": [1, 2],
                "coordinate": {
                    "latitude": "31.2",
                    "longitude": "121.5",
                    "coordinate_system": "gcj02"
                },
                "room_status": "available"
            }]
        }))
        .unwrap();
        assert_eq!(
            room_list.meetingroom_list[0].name.as_deref(),
            Some("Room A")
        );
        assert_eq!(room_list.meetingroom_list[0].capacity, Some(12));
        let coordinate = room_list.meetingroom_list[0]
            .coordinate
            .as_ref()
            .expect("meeting room coordinate");
        assert_eq!(coordinate.latitude, "31.2");
        assert_eq!(coordinate.longitude, "121.5");
        assert_eq!(coordinate.extra["coordinate_system"], "gcj02");
        assert_eq!(room_list.extra["total"], 1);
        assert_eq!(
            room_list.meetingroom_list[0].extra["room_status"],
            "available"
        );

        let room_booking: WorkMeetingRoomGetBookingInfoResponse = serde_json::from_value(json!({
            "next_cursor": "booking-cursor",
            "booking_list": [{
                "meeting_id": 123,
                "schedule_id": 456,
                "subject": "Weekly",
                "booker": "user",
                "attendees": ["user", "other"],
                "booking_status": "confirmed"
            }]
        }))
        .unwrap();
        assert_eq!(
            room_booking.booking_list[0].subject.as_deref(),
            Some("Weekly")
        );
        assert_eq!(room_booking.booking_list[0].schedule_id, Some(456));
        assert_eq!(room_booking.extra["next_cursor"], "booking-cursor");
        assert_eq!(
            room_booking.booking_list[0].extra["booking_status"],
            "confirmed"
        );

        let room_book: WorkMeetingRoomBookResponse = serde_json::from_value(json!({
            "meeting_id": 123,
            "schedule_id": 456,
            "approval_status": "none"
        }))
        .unwrap();
        assert_eq!(room_book.meeting_id, Some(123));
        assert_eq!(room_book.schedule_id, Some(456));
        assert_eq!(room_book.extra["approval_status"], "none");

        let form: WorkWeDocCreateFormResponse = serde_json::from_value(json!({
            "formid": "form-1",
            "form_url": "https://example.com/form"
        }))
        .unwrap();
        assert_eq!(form.formid.as_deref(), Some("form-1"));
        assert_eq!(form.extra["form_url"], "https://example.com/form");
    }

    #[test]
    fn serializes_work_wedoc_document_and_form_lifecycle_requests() {
        let create = serde_json::to_value(WorkWeDocCreateDocumentRequest {
            spaceid: None,
            fatherid: None,
            doc_type: 10,
            doc_name: "Operations".to_string(),
            admin_users: vec!["manager".to_string()],
        })
        .unwrap();
        assert_eq!(create["doc_type"], 10);
        assert_eq!(create["admin_users"][0], "manager");
        assert!(create.get("spaceid").is_none());

        let rename = serde_json::to_value(WorkWeDocRenameDocumentRequest {
            docid: Some("doc-1".to_string()),
            formid: None,
            new_name: "Operations 2026".to_string(),
        })
        .unwrap();
        assert_eq!(rename["docid"], "doc-1");
        assert!(rename.get("formid").is_none());

        let target = serde_json::to_value(WorkWeDocDocumentTargetRequest {
            docid: None,
            formid: Some("form-1".to_string()),
        })
        .unwrap();
        assert_eq!(target["formid"], "form-1");
        assert!(target.get("docid").is_none());

        let modify: WorkWeDocModifyFormRequest = serde_json::from_value(json!({
            "oper": 1,
            "formid": "form-1",
            "form_info": {
                "form_title": "Survey",
                "form_question": {
                    "items": [{
                        "question_id": 1,
                        "title": "Environment",
                        "pos": 1,
                        "status": 1,
                        "reply_type": 2,
                        "must_reply": true,
                        "option_item": [{
                            "key": 1,
                            "value": "Production",
                            "status": 1
                        }],
                        "question_extend_setting": {
                            "allow_other": true
                        }
                    }]
                }
            }
        }))
        .unwrap();
        let modify = serde_json::to_value(modify).unwrap();
        assert_eq!(modify["oper"], 1);
        assert_eq!(
            modify["form_info"]["form_question"]["items"][0]["option_item"][0]["value"],
            "Production"
        );
        assert_eq!(
            modify["form_info"]["form_question"]["items"][0]["question_extend_setting"]
                ["allow_other"],
            true
        );

        let statistics = serde_json::to_value(vec![WorkWeDocFormStatisticRequest {
            repeated_id: "cycle-1".to_string(),
            req_type: 2,
            start_time: Some(1_800_000_000),
            end_time: Some(1_800_086_400),
            limit: Some(100),
            cursor: None,
        }])
        .unwrap();
        assert!(statistics.is_array());
        assert_eq!(statistics[0]["repeated_id"], "cycle-1");
        assert!(statistics[0].get("cursor").is_none());

        let answers = serde_json::to_value(WorkWeDocFormAnswerRequest {
            repeated_id: "cycle-1".to_string(),
            answer_ids: vec![1, 2],
        })
        .unwrap();
        assert_eq!(answers["answer_ids"][1], 2);
    }

    #[test]
    fn serializes_work_wedoc_permission_and_vip_requests() {
        let join_rule = serde_json::to_value(WorkWeDocModifyJoinRuleRequest {
            docid: "doc-1".to_string(),
            enable_corp_internal: Some(true),
            corp_internal_auth: Some(1),
            enable_corp_external: Some(false),
            corp_external_auth: None,
            corp_internal_approve_only_by_admin: Some(true),
            corp_external_approve_only_by_admin: None,
            ban_share_external: Some(true),
            update_co_auth_list: Some(true),
            co_auth_list: vec![WorkWeDocDepartmentAuth {
                member_type: Some(2),
                departmentid: Some(10),
                auth: Some(1),
                extra: json!({ "scope_source": "directory" }),
            }],
        })
        .unwrap();
        assert_eq!(join_rule["docid"], "doc-1");
        assert_eq!(join_rule["co_auth_list"][0]["type"], 2);
        assert_eq!(join_rule["co_auth_list"][0]["departmentid"], 10);
        assert_eq!(join_rule["co_auth_list"][0]["scope_source"], "directory");
        assert!(join_rule.get("corp_external_auth").is_none());

        let members = serde_json::to_value(WorkWeDocModifyMembersRequest {
            docid: "doc-1".to_string(),
            update_file_member_list: vec![WorkWeDocDocumentMember {
                member_type: Some(1),
                userid: Some("user-1".to_string()),
                tmp_external_userid: None,
                auth: Some(7),
                extra: Value::Null,
            }],
            del_file_member_list: vec![WorkWeDocDocumentMember {
                member_type: Some(2),
                userid: None,
                tmp_external_userid: Some("external-1".to_string()),
                auth: None,
                extra: Value::Null,
            }],
        })
        .unwrap();
        assert_eq!(members["update_file_member_list"][0]["userid"], "user-1");
        assert_eq!(members["update_file_member_list"][0]["auth"], 7);
        assert_eq!(
            members["del_file_member_list"][0]["tmp_external_userid"],
            "external-1"
        );
        assert!(members["del_file_member_list"][0].get("auth").is_none());

        let safety = serde_json::to_value(WorkWeDocModifySafetySettingRequest {
            docid: "doc-1".to_string(),
            enable_readonly_copy: Some(false),
            watermark: Some(WorkWeDocWatermark {
                margin_type: Some(2),
                show_visitor_name: Some(true),
                show_text: Some(true),
                text: Some("Confidential".to_string()),
                extra: json!({ "color": "gray" }),
            }),
        })
        .unwrap();
        assert_eq!(safety["enable_readonly_copy"], false);
        assert_eq!(safety["watermark"]["margin_type"], 2);
        assert_eq!(safety["watermark"]["color"], "gray");

        let batch = serde_json::to_value(WorkWeDocVipBatchRequest {
            userid_list: vec!["user-1".to_string(), "user-2".to_string()],
        })
        .unwrap();
        assert_eq!(batch["userid_list"][1], "user-2");

        let list = serde_json::to_value(WorkWeDocVipListRequest {
            cursor: None,
            limit: Some(100),
        })
        .unwrap();
        assert_eq!(list["limit"], 100);
        assert!(list.get("cursor").is_none());
    }

    #[test]
    fn deserializes_work_wedoc_permission_and_vip_responses() {
        let auth: WorkWeDocDocumentAuthResponse = serde_json::from_value(json!({
            "access_rule": {
                "enable_corp_internal": true,
                "corp_internal_auth": 1,
                "enable_corp_external": false,
                "corp_internal_approve_only_by_admin": true,
                "corp_external_approve_only_by_admin": false,
                "ban_share_external": true,
                "rule_version": 2
            },
            "secure_setting": {
                "enable_readonly_copy": false,
                "enable_readonly_comment": true,
                "watermark": {
                    "margin_type": 2,
                    "show_visitor_name": true,
                    "show_text": true,
                    "text": "Confidential",
                    "watermark_version": 3
                },
                "security_level": "strict"
            },
            "doc_member_list": [{
                "type": 1,
                "userid": "user-1",
                "auth": 7,
                "member_source": "admin"
            }, {
                "type": 2,
                "tmp_external_userid": "external-1",
                "auth": 1
            }],
            "co_auth_list": [{
                "type": 2,
                "departmentid": 10,
                "auth": 1,
                "department_name": "Engineering"
            }],
            "trace_id": "auth"
        }))
        .unwrap();
        let access_rule = auth.access_rule.expect("access rule");
        assert_eq!(access_rule.corp_internal_auth, Some(1));
        assert_eq!(access_rule.extra["rule_version"], 2);
        let secure_setting = auth.secure_setting.expect("secure setting");
        assert_eq!(secure_setting.enable_readonly_comment, Some(true));
        assert_eq!(secure_setting.extra["security_level"], "strict");
        let watermark = secure_setting.watermark.expect("watermark");
        assert_eq!(watermark.text.as_deref(), Some("Confidential"));
        assert_eq!(watermark.extra["watermark_version"], 3);
        assert_eq!(auth.doc_member_list[0].member_type, Some(1));
        assert_eq!(auth.doc_member_list[0].extra["member_source"], "admin");
        assert_eq!(
            auth.doc_member_list[1].tmp_external_userid.as_deref(),
            Some("external-1")
        );
        assert_eq!(auth.co_auth_list[0].departmentid, Some(10));
        assert_eq!(auth.co_auth_list[0].extra["department_name"], "Engineering");
        assert_eq!(auth.extra["trace_id"], "auth");

        let batch: WorkWeDocVipBatchResponse = serde_json::from_value(json!({
            "succ_userid_list": ["user-1"],
            "fail_userid_list": ["user-2"],
            "request_id": "batch"
        }))
        .unwrap();
        assert_eq!(batch.succ_userid_list[0], "user-1");
        assert_eq!(batch.fail_userid_list[0], "user-2");
        assert_eq!(batch.extra["request_id"], "batch");

        let list: WorkWeDocVipListResponse = serde_json::from_value(json!({
            "has_more": true,
            "next_cursor": "next",
            "userid_list": ["user-1", "user-2"],
            "total": 2
        }))
        .unwrap();
        assert_eq!(list.has_more, Some(true));
        assert_eq!(list.next_cursor.as_deref(), Some("next"));
        assert_eq!(list.userid_list[1], "user-2");
        assert_eq!(list.extra["total"], 2);
    }

    #[test]
    fn serializes_work_wedoc_document_and_spreadsheet_content_requests() {
        let document_request = WorkWeDocDocumentBatchUpdateRequest::new(
            "doc-1",
            8,
            [
                WorkWeDocDocumentUpdateRequest::insert_text(1, "Weekly report"),
                WorkWeDocDocumentUpdateRequest::delete_content(10, 10),
                WorkWeDocDocumentUpdateRequest::replace_text(
                    "Revenue",
                    [WorkWeDocDocumentRange::new(20, 7)],
                ),
                WorkWeDocDocumentUpdateRequest::insert_image(30, "image-1"),
                WorkWeDocDocumentUpdateRequest::insert_page_break(31),
                WorkWeDocDocumentUpdateRequest::insert_table(32, 10, 6),
                WorkWeDocDocumentUpdateRequest::insert_paragraph(33),
                WorkWeDocDocumentUpdateRequest::update_text_property(
                    WorkWeDocDocumentTextProperty {
                        bold: Some(true),
                        color: Some("FF0000".to_string()),
                        background_color: Some("FFFFFF".to_string()),
                    },
                    [WorkWeDocDocumentRange::new(40, 5)],
                ),
            ],
        );
        document_request.validate().unwrap();
        assert_eq!(
            document_request.requests[0].operation_kind(),
            Some(WorkWeDocDocumentOperationKind::InsertText)
        );
        assert_eq!(WorkWeDocDocumentRange::new(10, 10).end_index(), Some(20));
        let document = serde_json::to_value(document_request).unwrap();
        assert_eq!(document["version"], 8);
        assert_eq!(
            document["requests"][0]["insert_text"]["text"],
            "Weekly report"
        );
        assert!(document["requests"][0].get("insert_text_request").is_none());
        assert_eq!(
            document["requests"][1]["delete_content"]["range"]["length"],
            10
        );
        assert_eq!(
            document["requests"][2]["replace_text"]["ranges"][0]["start_index"],
            20
        );
        assert_eq!(
            document["requests"][3]["insert_image"]["image_id"],
            "image-1"
        );
        assert_eq!(
            document["requests"][4]["insert_page_break"]["location"]["index"],
            31
        );
        assert_eq!(document["requests"][5]["insert_table"]["cols"], 6);
        assert_eq!(
            document["requests"][6]["insert_paragraph"]["location"]["index"],
            33
        );
        assert_eq!(
            document["requests"][7]["update_text_property"]["text_property"]["background_color"],
            "FFFFFF"
        );

        let unknown: WorkWeDocDocumentUpdateRequest =
            serde_json::from_value(json!({ "future_operation": { "enabled": true } })).unwrap();
        assert_eq!(
            unknown.operation_kind(),
            Some(WorkWeDocDocumentOperationKind::Other)
        );
        unknown.validate().unwrap();
        assert_eq!(
            serde_json::to_value(unknown).unwrap()["future_operation"]["enabled"],
            true
        );
        let legacy: WorkWeDocDocumentUpdateRequest = serde_json::from_value(json!({
            "insert_text_request": {
                "text": "legacy",
                "location": { "index": 2 }
            }
        }))
        .unwrap();
        assert_eq!(
            legacy.operation_kind(),
            Some(WorkWeDocDocumentOperationKind::InsertText)
        );
        assert!(serde_json::to_value(legacy)
            .unwrap()
            .get("insert_text")
            .is_some());

        assert!(WorkWeDocDocumentBatchUpdateRequest::new("doc", 1, [])
            .validate()
            .is_err());
        assert!(WorkWeDocDocumentBatchUpdateRequest::new(
            "doc",
            1,
            (0..31).map(WorkWeDocDocumentUpdateRequest::insert_paragraph)
        )
        .validate()
        .is_err());
        assert!(WorkWeDocDocumentBatchUpdateRequest::new(
            "doc",
            1,
            [WorkWeDocDocumentUpdateRequest::insert_table(0, 20, 60)]
        )
        .validate()
        .is_err());
        let mut incomplete_image = WorkWeDocDocumentUpdateRequest::insert_image(0, "image-1");
        incomplete_image.insert_image.as_mut().unwrap().width = Some(100);
        assert!(incomplete_image.validate().is_err());
        let conflicting = WorkWeDocDocumentUpdateRequest {
            insert_text: Some(WorkWeDocDocumentInsertText {
                text: "text".to_string(),
                location: WorkWeDocDocumentLocation { index: 0 },
            }),
            insert_paragraph: Some(WorkWeDocDocumentInsertLocation {
                location: WorkWeDocDocumentLocation { index: 1 },
            }),
            ..WorkWeDocDocumentUpdateRequest::default()
        };
        assert!(conflicting.validate().is_err());
        let known_and_unknown: WorkWeDocDocumentUpdateRequest = serde_json::from_value(json!({
            "insert_paragraph": { "location": { "index": 1 } },
            "future_operation": { "enabled": true }
        }))
        .unwrap();
        assert!(known_and_unknown.validate().is_err());

        let range = serde_json::to_value(WorkWeDocSpreadsheetRangeRequest {
            docid: "sheet-doc".to_string(),
            sheet_id: "sheet-1".to_string(),
            range: "A1:B2".to_string(),
        })
        .unwrap();
        assert_eq!(range["sheet_id"], "sheet-1");
        assert_eq!(range["range"], "A1:B2");

        let batch: WorkWeDocSpreadsheetBatchUpdateRequest = serde_json::from_value(json!({
            "docid": "sheet-doc",
            "requests": [{
                "add_sheet_request": {
                    "title": "Summary",
                    "row_count": 100,
                    "column_count": 20
                }
            }, {
                "update_range_request": {
                    "sheet_id": "sheet-1",
                    "grid_data": {
                        "start_row": 0,
                        "start_column": 0,
                        "rows": [{
                            "values": [{
                                "cell_value": {
                                    "text": "Revenue"
                                },
                                "cell_format": {
                                    "text_format": {
                                        "font": "Arial",
                                        "font_size": 12,
                                        "bold": true,
                                        "italic": false,
                                        "strikethrough": false,
                                        "underline": true,
                                        "color": {
                                            "red": 10,
                                            "green": 20,
                                            "blue": 30,
                                            "alpha": 255
                                        }
                                    }
                                }
                            }, {
                                "cell_value": {
                                    "link": {
                                        "url": "https://example.com",
                                        "text": "Details"
                                    }
                                }
                            }]
                        }]
                    }
                }
            }, {
                "delete_dimension_request": {
                    "sheet_id": "sheet-1",
                    "dimension": "ROW",
                    "start_index": 10,
                    "end_index": 20
                }
            }, {
                "delete_sheet_request": {
                    "sheet_id": "sheet-old"
                }
            }]
        }))
        .unwrap();
        let batch = serde_json::to_value(batch).unwrap();
        assert_eq!(
            batch["requests"][0]["add_sheet_request"]["title"],
            "Summary"
        );
        assert_eq!(
            batch["requests"][1]["update_range_request"]["grid_data"]["rows"][0]["values"][0]
                ["cell_format"]["text_format"]["color"]["alpha"],
            255
        );
        assert_eq!(
            batch["requests"][1]["update_range_request"]["grid_data"]["rows"][0]["values"][1]
                ["cell_value"]["link"]["url"],
            "https://example.com"
        );
        assert_eq!(
            batch["requests"][2]["delete_dimension_request"]["dimension"],
            "ROW"
        );
        assert_eq!(
            batch["requests"][3]["delete_sheet_request"]["sheet_id"],
            "sheet-old"
        );
    }

    #[test]
    fn deserializes_work_wedoc_document_and_spreadsheet_content_responses() {
        let document: WorkWeDocDocumentDataResponse = serde_json::from_value(json!({
            "version": 8,
            "document": {
                "document_id": "doc-1",
                "body": {
                    "blocks": [{
                        "type": "paragraph",
                        "text": "Weekly report"
                    }]
                }
            },
            "request_id": "document"
        }))
        .unwrap();
        assert_eq!(document.version, Some(8));
        assert_eq!(
            document.document.as_ref().expect("document")["document_id"],
            "doc-1"
        );
        assert_eq!(document.extra["request_id"], "document");

        let properties: WorkWeDocSpreadsheetPropertiesResponse = serde_json::from_value(json!({
            "properties": [{
                "sheet_id": "sheet-1",
                "title": "Summary",
                "row_count": 100,
                "column_count": 20,
                "frozen_row_count": 1
            }],
            "trace_id": "properties"
        }))
        .unwrap();
        assert_eq!(
            properties.properties[0].sheet_id.as_deref(),
            Some("sheet-1")
        );
        assert_eq!(properties.properties[0].row_count, Some(100));
        assert_eq!(properties.properties[0].extra["frozen_row_count"], 1);
        assert_eq!(properties.extra["trace_id"], "properties");

        let range: WorkWeDocSpreadsheetRangeResponse = serde_json::from_value(json!({
            "data": {
                "result": {
                    "start_row": 0,
                    "start_column": 0,
                    "rows": [{
                        "values": [{
                            "cell_value": {
                                "text": "Revenue",
                                "value_type": "text"
                            },
                            "cell_format": {
                                "text_format": {
                                    "font": "Arial",
                                    "font_size": 12,
                                    "bold": true,
                                    "italic": false,
                                    "strikethrough": false,
                                    "underline": true,
                                    "color": {
                                        "red": 10,
                                        "green": 20,
                                        "blue": 30,
                                        "alpha": 255,
                                        "theme": "custom"
                                    },
                                    "format_version": 2
                                },
                                "horizontal_alignment": "CENTER"
                            },
                            "cell_note": "header"
                        }]
                    }],
                    "range_version": 3
                },
                "data_source": "live"
            },
            "trace_id": "range"
        }))
        .unwrap();
        let data = range.data.as_ref().expect("range data");
        let grid = data.result.as_ref().expect("grid data");
        let cell = &grid.rows[0].values[0];
        assert_eq!(
            cell.cell_value
                .as_ref()
                .expect("cell value")
                .text
                .as_deref(),
            Some("Revenue")
        );
        assert_eq!(
            cell.cell_value.as_ref().expect("cell value").extra["value_type"],
            "text"
        );
        let format = cell.cell_format.as_ref().expect("cell format");
        let text_format = format.text_format.as_ref().expect("text format");
        assert_eq!(text_format.bold, Some(true));
        assert_eq!(
            text_format.color.as_ref().expect("color").extra["theme"],
            "custom"
        );
        assert_eq!(text_format.extra["format_version"], 2);
        assert_eq!(format.extra["horizontal_alignment"], "CENTER");
        assert_eq!(cell.extra["cell_note"], "header");
        assert_eq!(grid.extra["range_version"], 3);
        assert_eq!(data.extra["data_source"], "live");
        assert_eq!(range.extra["trace_id"], "range");

        let update: WorkWeDocSpreadsheetBatchUpdateResponse = serde_json::from_value(json!({
            "data": {
                "responses": [{
                    "add_sheet_response": {
                        "properties": {
                            "sheet_id": "sheet-new",
                            "title": "Summary",
                            "row_count": 100,
                            "column_count": 20,
                            "tab_color": "blue"
                        },
                        "operation_id": "add"
                    }
                }, {
                    "update_range_response": {
                        "updated_cells": 4,
                        "updated_rows": 2
                    }
                }, {
                    "delete_dimension_response": {
                        "deleted": 10,
                        "dimension": "ROW"
                    }
                }, {
                    "delete_sheet_response": {
                        "sheet_id": "sheet-old",
                        "recycle_bin": true
                    }
                }],
                "batch_version": 9
            },
            "request_id": "batch-update"
        }))
        .unwrap();
        let data = update.data.as_ref().expect("batch data");
        let add = data.responses[0]
            .add_sheet_response
            .as_ref()
            .expect("add sheet response");
        assert_eq!(
            add.properties
                .as_ref()
                .expect("created sheet properties")
                .sheet_id
                .as_deref(),
            Some("sheet-new")
        );
        assert_eq!(
            add.properties
                .as_ref()
                .expect("created sheet properties")
                .extra["tab_color"],
            "blue"
        );
        assert_eq!(add.extra["operation_id"], "add");
        assert_eq!(
            data.responses[1]
                .update_range_response
                .as_ref()
                .expect("update range response")
                .extra["updated_rows"],
            2
        );
        assert_eq!(
            data.responses[2]
                .delete_dimension_response
                .as_ref()
                .expect("delete dimension response")
                .deleted,
            Some(10)
        );
        assert_eq!(
            data.responses[3]
                .delete_sheet_response
                .as_ref()
                .expect("delete sheet response")
                .extra["recycle_bin"],
            true
        );
        assert_eq!(data.extra["batch_version"], 9);
        assert_eq!(update.extra["request_id"], "batch-update");
    }

    #[test]
    fn deserializes_work_wedoc_document_and_form_lifecycle_responses() {
        let create: WorkWeDocCreateDocumentResponse = serde_json::from_value(json!({
            "url": "https://example.com/doc",
            "docid": "doc-1",
            "request_id": "create-doc"
        }))
        .unwrap();
        assert_eq!(create.docid.as_deref(), Some("doc-1"));
        assert_eq!(create.extra["request_id"], "create-doc");

        let info: WorkWeDocDocumentBaseInfoResponse = serde_json::from_value(json!({
            "doc_base_info": {
                "docid": "doc-1",
                "doc_name": "Operations",
                "create_time": 1_800_000_000,
                "modify_time": 1_800_000_100,
                "doc_type": 10,
                "owner_userid": "manager"
            },
            "trace_id": "doc-info"
        }))
        .unwrap();
        let base = info.doc_base_info.expect("document base info");
        assert_eq!(base.doc_type, Some(10));
        assert_eq!(base.extra["owner_userid"], "manager");
        assert_eq!(info.extra["trace_id"], "doc-info");

        let share: WorkWeDocShareDocumentResponse = serde_json::from_value(json!({
            "share_url": "https://example.com/share",
            "expires_in": 3600
        }))
        .unwrap();
        assert_eq!(
            share.share_url.as_deref(),
            Some("https://example.com/share")
        );
        assert_eq!(share.extra["expires_in"], 3600);

        let form: WorkWeDocFormInfoResponse = serde_json::from_value(json!({
            "form_info": {
                "formid": "form-1",
                "form_title": "Survey",
                "form_question": {
                    "items": [{
                        "question_id": 1,
                        "title": "Environment",
                        "reply_type": 2,
                        "question_version": 3
                    }],
                    "question_trace": "typed"
                },
                "form_setting": {
                    "fill_out_auth": 1,
                    "max_fill_cnt": 2,
                    "timed_repeat_info": {
                        "enable": true,
                        "repeat_type": 2,
                        "rule_ctime": 1_800_000_000,
                        "timezone": 8
                    },
                    "setting_version": 4
                },
                "repeated_id": ["cycle-1"],
                "form_version": 5
            },
            "trace_id": "form-info"
        }))
        .unwrap();
        let form_info = form.form_info.expect("form info");
        assert_eq!(form_info.repeated_id[0], "cycle-1");
        let questions = form_info.form_question.expect("form questions");
        assert_eq!(questions.items[0].reply_type, Some(2));
        assert_eq!(questions.items[0].extra["question_version"], 3);
        assert_eq!(questions.extra["question_trace"], "typed");
        let setting = form_info.form_setting.expect("form setting");
        assert_eq!(setting.max_fill_cnt, Some(2));
        assert_eq!(
            setting.timed_repeat_info.expect("timed repeat").extra["timezone"],
            8
        );
        assert_eq!(setting.extra["setting_version"], 4);
        assert_eq!(form_info.extra["form_version"], 5);
        assert_eq!(form.extra["trace_id"], "form-info");

        let statistics: WorkWeDocFormStatisticsResponse = serde_json::from_value(json!({
            "statistic_list": [{
                "fill_cnt": 2,
                "repeated_id": "cycle-1",
                "fill_user_cnt": 2,
                "unfill_user_cnt": 1,
                "submit_users": [{
                    "userid": "user-1",
                    "submit_time": 1_800_000_000,
                    "answer_id": 1,
                    "user_name": "Alice",
                    "source": "internal"
                }],
                "unfill_users": [{
                    "userid": "user-2",
                    "user_name": "Bob",
                    "department": 2
                }],
                "has_more": false,
                "cursor": 2,
                "statistic_version": 3
            }],
            "trace_id": "statistics"
        }))
        .unwrap();
        assert_eq!(statistics.statistic_list[0].fill_cnt, Some(2));
        assert_eq!(
            statistics.statistic_list[0].submit_users[0].extra["source"],
            "internal"
        );
        assert_eq!(
            statistics.statistic_list[0].unfill_users[0].extra["department"],
            2
        );
        assert_eq!(statistics.statistic_list[0].extra["statistic_version"], 3);
        assert_eq!(statistics.extra["trace_id"], "statistics");

        let answers: WorkWeDocFormAnswersResponse = serde_json::from_value(json!({
            "answer": {
                "answer_list": [{
                    "answer_id": 1,
                    "user_name": "Alice",
                    "ctime": 1_800_000_000,
                    "mtime": 1_800_000_100,
                    "reply": {
                        "items": [{
                            "question_id": 1,
                            "text_reply": "Production",
                            "option_reply": [1],
                            "option_extend_reply": [{
                                "option_reply": 1,
                                "extend_text": "Primary",
                                "locale": "zh-CN"
                            }],
                            "file_extend_reply": [{
                                "name": "evidence.txt",
                                "fileid": "file-1",
                                "size": 10
                            }],
                            "department_reply": {
                                "list": [{
                                    "department_id": 2,
                                    "name": "Engineering"
                                }]
                            },
                            "member_reply": {
                                "list": [{
                                    "userid": "user-2",
                                    "display_name": "Bob"
                                }]
                            },
                            "duration_reply": {
                                "begin_time": 1_800_000_000,
                                "end_time": 1_800_003_600,
                                "time_scale": 1,
                                "day_range": 0,
                                "days": 0.5,
                                "hours": 1.0,
                                "timezone": 8
                            },
                            "reply_version": 4
                        }],
                        "reply_trace": "typed"
                    },
                    "answer_status": 1,
                    "userid": "user-1",
                    "answer_version": 5
                }],
                "answer_trace": "list"
            },
            "trace_id": "answers"
        }))
        .unwrap();
        let answer = &answers.answer.as_ref().expect("answer list").answer_list[0];
        let reply = answer.reply.as_ref().expect("answer reply");
        assert_eq!(reply.items[0].text_reply.as_deref(), Some("Production"));
        assert_eq!(
            reply.items[0].option_extend_reply[0].extra["locale"],
            "zh-CN"
        );
        assert_eq!(reply.items[0].file_extend_reply[0].extra["size"], 10);
        assert_eq!(
            reply.items[0]
                .department_reply
                .as_ref()
                .expect("department reply")
                .list[0]
                .extra["name"],
            "Engineering"
        );
        assert_eq!(
            reply.items[0]
                .member_reply
                .as_ref()
                .expect("member reply")
                .list[0]
                .extra["display_name"],
            "Bob"
        );
        assert_eq!(
            reply.items[0]
                .duration_reply
                .as_ref()
                .expect("duration reply")
                .extra["timezone"],
            8
        );
        assert_eq!(reply.items[0].extra["reply_version"], 4);
        assert_eq!(reply.extra["reply_trace"], "typed");
        assert_eq!(answer.extra["answer_version"], 5);
        assert_eq!(
            answers.answer.as_ref().expect("answer list").extra["answer_trace"],
            "list"
        );
        assert_eq!(answers.extra["trace_id"], "answers");
    }

    #[test]
    fn serializes_work_oa_living_and_wedrive_requests() {
        let living = serde_json::to_value(WorkLivingCreateRequest {
            anchor_userid: "anchor".to_string(),
            theme: "Launch".to_string(),
            living_start: 1_800_000_000,
            living_duration: 3600,
            description: Some("product update".to_string()),
            living_type: Some(4),
            agentid: Some(100001),
            remind_time: Some(15),
            activity_cover_mediaid: Some("cover".to_string()),
            activity_share_mediaid: Some("share".to_string()),
            activity_detail: Some(WorkLivingActivityDetail {
                description: Some("detail".to_string()),
                image_list: vec!["detail-image".to_string()],
                extra: json!({ "layout": "gallery" }),
            }),
        })
        .unwrap();
        assert_eq!(living["anchor_userid"], "anchor");
        assert_eq!(living["type"], 4);
        assert_eq!(living["activity_detail"]["description"], "detail");
        assert_eq!(living["activity_detail"]["image_list"][0], "detail-image");
        assert_eq!(living["activity_detail"]["layout"], "gallery");

        let minimal_living = serde_json::to_value(WorkLivingCreateRequest {
            anchor_userid: "anchor".to_string(),
            theme: "Minimal".to_string(),
            living_start: 1_800_000_000,
            living_duration: 1800,
            description: None,
            living_type: None,
            agentid: None,
            remind_time: None,
            activity_cover_mediaid: None,
            activity_share_mediaid: None,
            activity_detail: None,
        })
        .unwrap();
        assert!(minimal_living.get("description").is_none());
        assert!(minimal_living.get("type").is_none());
        assert!(minimal_living.get("activity_detail").is_none());

        let modify = serde_json::to_value(WorkLivingModifyRequest {
            livingid: "living-1".to_string(),
            theme: Some("Launch updated".to_string()),
            living_start: None,
            living_duration: Some(1800),
            description: None,
            living_type: None,
            remind_time: Some(10),
        })
        .unwrap();
        assert_eq!(modify["livingid"], "living-1");
        assert_eq!(modify["living_duration"], 1800);
        assert!(modify.get("description").is_none());
        assert!(modify.get("type").is_none());

        let living_ids = serde_json::to_value(WorkLivingGetUserAllLivingIdRequest {
            userid: "user".to_string(),
            cursor: "cursor".to_string(),
            limit: 100,
        })
        .unwrap();
        assert_eq!(living_ids["userid"], "user");
        assert_eq!(living_ids["limit"], 100);

        let space = serde_json::to_value(WorkWeDriveSpaceCreateRequest {
            userid: "user".to_string(),
            space_name: "Team Space".to_string(),
            auth_info: vec![
                WorkWeDriveAuthInfo {
                    member_type: 1,
                    userid: Some("member".to_string()),
                    departmentid: None,
                    auth: Some(7),
                    create_time: None,
                    extra: Value::Null,
                },
                WorkWeDriveAuthInfo {
                    member_type: 2,
                    userid: None,
                    departmentid: Some(10),
                    auth: Some(1),
                    create_time: None,
                    extra: json!({ "source": "directory" }),
                },
            ],
            space_sub_type: Some(0),
        })
        .unwrap();
        assert_eq!(space["space_name"], "Team Space");
        assert_eq!(space["auth_info"][0]["auth"], 7);
        assert_eq!(space["auth_info"][0]["userid"], "member");
        assert_eq!(space["auth_info"][1]["departmentid"], 10);
        assert_eq!(space["auth_info"][1]["source"], "directory");
        assert_eq!(space["space_sub_type"], 0);

        let rename = serde_json::to_value(WorkWeDriveSpaceRenameRequest {
            userid: "user".to_string(),
            spaceid: "space".to_string(),
            space_name: "New Space".to_string(),
        })
        .unwrap();
        assert_eq!(rename["spaceid"], "space");

        let space_id = serde_json::to_value(WorkWeDriveSpaceIdRequest {
            userid: "user".to_string(),
            spaceid: "space".to_string(),
        })
        .unwrap();
        assert_eq!(space_id["userid"], "user");

        let space_acl = serde_json::to_value(WorkWeDriveSpaceAclRequest {
            userid: "user".to_string(),
            spaceid: "space".to_string(),
            auth_info: vec![WorkWeDriveAuthInfo {
                member_type: 1,
                userid: Some("member".to_string()),
                departmentid: None,
                auth: None,
                create_time: None,
                extra: Value::Null,
            }],
        })
        .unwrap();
        assert_eq!(space_acl["auth_info"][0]["userid"], "member");
        assert!(space_acl["auth_info"][0].get("auth").is_none());

        let space_setting = serde_json::to_value(WorkWeDriveSpaceSettingRequest {
            userid: "user".to_string(),
            spaceid: "space".to_string(),
            enable_watermark: true,
            add_member_only_admin: false,
            enable_share_url: true,
            share_url_no_approve: false,
            share_url_no_approve_default_auth: 1,
        })
        .unwrap();
        assert_eq!(space_setting["enable_watermark"], true);

        let file_list = serde_json::to_value(WorkWeDriveFileListRequest {
            userid: "user".to_string(),
            spaceid: "space".to_string(),
            fatherid: "root".to_string(),
            sort_type: 1,
            start: 0,
            limit: 100,
        })
        .unwrap();
        assert_eq!(file_list["fatherid"], "root");

        let upload = serde_json::to_value(WorkWeDriveFileUploadRequest {
            userid: "user".to_string(),
            spaceid: "space".to_string(),
            fatherid: "root".to_string(),
            file_name: "a.txt".to_string(),
            file_base64_content: "YQ==".to_string(),
        })
        .unwrap();
        assert_eq!(upload["file_base64_content"], "YQ==");

        let file_id = serde_json::to_value(WorkWeDriveFileIdRequest {
            userid: "user".to_string(),
            fileid: "file".to_string(),
        })
        .unwrap();
        assert_eq!(file_id["fileid"], "file");

        let create = serde_json::to_value(WorkWeDriveFileCreateRequest {
            userid: "user".to_string(),
            spaceid: "space".to_string(),
            fatherid: "root".to_string(),
            file_type: "doc".to_string(),
            file_name: "doc".to_string(),
        })
        .unwrap();
        assert_eq!(create["file_type"], "doc");

        let file_rename = serde_json::to_value(WorkWeDriveFileRenameRequest {
            userid: "user".to_string(),
            fileid: "file".to_string(),
            new_name: "new.txt".to_string(),
        })
        .unwrap();
        assert_eq!(file_rename["new_name"], "new.txt");

        let file_move = serde_json::to_value(WorkWeDriveFileMoveRequest {
            userid: "user".to_string(),
            fatherid: "new-parent".to_string(),
            replace: true,
            fileid: vec!["file".to_string()],
        })
        .unwrap();
        assert_eq!(file_move["replace"], true);
        assert_eq!(file_move["fileid"][0], "file");

        let file_acl = serde_json::to_value(WorkWeDriveFileAclRequest {
            userid: "user".to_string(),
            fileid: "file".to_string(),
            auth_info: vec![WorkWeDriveAuthInfo {
                member_type: 1,
                userid: Some("member".to_string()),
                departmentid: None,
                auth: Some(1),
                create_time: None,
                extra: Value::Null,
            }],
        })
        .unwrap();
        assert_eq!(file_acl["auth_info"][0]["auth"], 1);

        let file_setting = serde_json::to_value(WorkWeDriveFileSettingRequest {
            userid: "user".to_string(),
            fileid: "file".to_string(),
            auth_scope: 1,
            auth: 7,
        })
        .unwrap();
        assert_eq!(file_setting["auth_scope"], 1);
    }

    #[test]
    fn deserializes_work_oa_living_and_wedrive_responses() {
        let living: WorkLivingCreateResponse = serde_json::from_value(json!({
            "livingid": "living-100",
            "request_id": "living-create"
        }))
        .unwrap();
        assert_eq!(living.livingid.as_deref(), Some("living-100"));
        assert_eq!(living.extra["request_id"], "living-create");

        let code: WorkLivingCodeResponse = serde_json::from_value(json!({
            "living_code": "living-code-200",
            "expire_time": 1_800_003_600
        }))
        .unwrap();
        assert_eq!(code.living_code.as_deref(), Some("living-code-200"));
        assert_eq!(code.extra["expire_time"], 1_800_003_600);

        let ids: WorkLivingGetUserAllLivingIdResponse = serde_json::from_value(json!({
            "next_cursor": "next",
            "livingid_list": ["living-1"],
            "total": 1
        }))
        .unwrap();
        assert_eq!(ids.next_cursor.as_deref(), Some("next"));
        assert_eq!(ids.livingid_list[0], "living-1");
        assert_eq!(ids.extra["total"], 1);

        let info: WorkLivingInfoResponse = serde_json::from_value(json!({
            "trace_id": "living-info",
            "living_info": {
                "anchor_userid": "anchor",
                "theme": "Launch",
                "living_start": 1_800_000_000,
                "living_duration": 3600,
                "description": "Product update",
                "type": 1,
                "status": 2,
                "main_department": 10,
                "viewer_num": 3,
                "online_count": 1,
                "open_replay": 1,
                "reserve_living_duration": 7200,
                "reserve_start": 1_799_999_000,
                "replay_status": 1,
                "mic_num": 2,
                "push_stream_url": "https://example.com/push",
                "subscribe_count": 8,
                "comment_num": 4,
                "info_version": 2
            }
        }))
        .unwrap();
        assert_eq!(info.extra["trace_id"], "living-info");
        let info = info.living_info.unwrap();
        assert_eq!(info.theme.as_deref(), Some("Launch"));
        assert_eq!(info.living_type, Some(1));
        assert_eq!(info.viewer_num, Some(3));
        assert_eq!(info.online_count, Some(1));
        assert_eq!(info.replay_status, Some(1));
        assert_eq!(
            info.push_stream_url.as_deref(),
            Some("https://example.com/push")
        );
        assert_eq!(info.extra["info_version"], 2);

        let stat: WorkLivingWatchStatResponse = serde_json::from_value(json!({
            "ending": 0,
            "next_key": "next",
            "request_id": "watch-stat",
            "stat_info": {
                "users": [{
                    "userid": "viewer",
                    "watch_time": 120,
                    "is_comment": 1,
                    "is_mic": 0,
                    "terminal": "desktop"
                }],
                "external_users": [{
                    "external_userid": "external-viewer",
                    "type": 2,
                    "name": "External Viewer",
                    "watch_time": 60,
                    "is_comment": 0,
                    "is_mic": 1,
                    "unionid": "union-1"
                }],
                "stat_version": 2
            }
        }))
        .unwrap();
        assert_eq!(stat.ending, Some(0));
        assert_eq!(stat.extra["request_id"], "watch-stat");
        let stat_info = stat.stat_info.unwrap();
        assert_eq!(stat_info.users[0].userid, "viewer");
        assert_eq!(stat_info.users[0].extra["terminal"], "desktop");
        assert_eq!(
            stat_info.external_users[0].external_userid,
            "external-viewer"
        );
        assert_eq!(stat_info.external_users[0].viewer_type, 2);
        assert_eq!(stat_info.external_users[0].extra["unionid"], "union-1");
        assert_eq!(stat_info.extra["stat_version"], 2);

        let share_info: WorkLivingShareInfoResponse = serde_json::from_value(json!({
            "livingid": "living-1",
            "viewer_userid": "viewer",
            "viewer_external_userid": "external-viewer",
            "invitor_userid": "invitor",
            "invitor_external_userid": "external-invitor",
            "share_channel": "timeline"
        }))
        .unwrap();
        assert_eq!(share_info.viewer_userid.as_deref(), Some("viewer"));
        assert_eq!(share_info.extra["share_channel"], "timeline");

        let space_create: WorkWeDriveSpaceCreateResponse =
            serde_json::from_value(json!({ "spaceid": "space", "request_id": "space-create" }))
                .unwrap();
        assert_eq!(space_create.spaceid.as_deref(), Some("space"));
        assert_eq!(space_create.extra["request_id"], "space-create");

        let space_info: WorkWeDriveSpaceInfoResponse = serde_json::from_value(json!({
            "trace_id": "space-info",
            "space_info": {
                "spaceid": "space",
                "space_name": "Team Space",
                "auth_list": {
                    "auth_info": [{
                        "type": 1,
                        "userid": "member",
                        "auth": 7,
                        "create_time": 1_800_000_000,
                        "display_name": "Member"
                    }, {
                        "type": 2,
                        "departmentid": 10,
                        "auth": 1
                    }],
                    "quit_userid": ["former-member"],
                    "auth_version": 2
                },
                "space_sub_type": 0,
                "secure_setting": {
                    "enable_watermark": true,
                    "add_member_only_admin": true,
                    "enable_share_url": false,
                    "share_url_no_approve": false,
                    "share_url_no_approve_default_auth": 2,
                    "enable_share_external": false,
                    "enable_share_external_admin": true,
                    "enable_space_add_external_member": false,
                    "enable_space_add_external_member_admin": true,
                    "enable_confidential_mode": true,
                    "default_file_scope": 2,
                    "create_file_only_admin": false,
                    "setting_version": 3
                },
                "owner_department": 1
            }
        }))
        .unwrap();
        assert_eq!(space_info.extra["trace_id"], "space-info");
        let space_info = space_info.space_info.unwrap();
        assert_eq!(space_info.space_name.as_deref(), Some("Team Space"));
        let auth_list = space_info.auth_list.as_ref().unwrap();
        assert_eq!(auth_list.auth_info[0].userid.as_deref(), Some("member"));
        assert_eq!(auth_list.auth_info[0].auth, Some(7));
        assert_eq!(auth_list.auth_info[0].extra["display_name"], "Member");
        assert_eq!(auth_list.quit_userid[0], "former-member");
        assert_eq!(auth_list.extra["auth_version"], 2);
        let secure = space_info.secure_setting.as_ref().unwrap();
        assert_eq!(secure.enable_watermark, Some(true));
        assert_eq!(secure.enable_confidential_mode, Some(true));
        assert_eq!(secure.extra["setting_version"], 3);
        assert_eq!(space_info.extra["owner_department"], 1);

        let space_share: WorkWeDriveSpaceShareResponse = serde_json::from_value(json!({
            "space_share_url": "https://example.com/space",
            "expire_time": 1_800_003_600
        }))
        .unwrap();
        assert_eq!(
            space_share.space_share_url.as_deref(),
            Some("https://example.com/space")
        );
        assert_eq!(space_share.extra["expire_time"], 1_800_003_600);

        let file_list: WorkWeDriveFileListResponse = serde_json::from_value(json!({
            "has_more": true,
            "next_start": 100,
            "scan_id": "file-list",
            "file_list": {
                "item": [{
                    "fileid": "file",
                    "file_name": "doc.txt",
                    "spaceid": "space",
                    "fatherid": "root",
                    "file_size": 10,
                    "ctime": 1_800_000_000,
                    "mtime": 1_800_000_100,
                    "file_type": 2,
                    "file_status": 1,
                    "create_userid": "creator",
                    "update_userid": "editor",
                    "sha": "sha-hash",
                    "md5": "md5-hash",
                    "url": "https://example.com/file",
                    "virus_scan_status": 0
                }],
                "list_version": 2
            }
        }))
        .unwrap();
        assert_eq!(file_list.has_more, Some(true));
        assert_eq!(file_list.extra["scan_id"], "file-list");
        let files = file_list.file_list.as_ref().unwrap();
        assert_eq!(files.item[0].fileid.as_deref(), Some("file"));
        assert_eq!(files.item[0].file_name.as_deref(), Some("doc.txt"));
        assert_eq!(files.item[0].file_type, Some(2));
        assert_eq!(files.item[0].sha.as_deref(), Some("sha-hash"));
        assert_eq!(files.item[0].extra["virus_scan_status"], 0);
        assert_eq!(files.extra["list_version"], 2);

        let upload: WorkWeDriveFileUploadResponse =
            serde_json::from_value(json!({ "fileid": "file", "upload_token": "token" })).unwrap();
        assert_eq!(upload.fileid.as_deref(), Some("file"));
        assert_eq!(upload.extra["upload_token"], "token");

        let download: WorkWeDriveFileDownloadResponse = serde_json::from_value(json!({
            "download_url": "https://example.com/file",
            "cookie_name": "SESSION",
            "cookie_value": "value",
            "expire_time": 1_800_003_600
        }))
        .unwrap();
        assert_eq!(download.cookie_name.as_deref(), Some("SESSION"));
        assert_eq!(download.extra["expire_time"], 1_800_003_600);

        let create: WorkWeDriveFileCreateResponse = serde_json::from_value(json!({
            "fileid": "file",
            "url": "https://example.com/doc",
            "template_id": "template"
        }))
        .unwrap();
        assert_eq!(create.url.as_deref(), Some("https://example.com/doc"));
        assert_eq!(create.extra["template_id"], "template");

        let rename: WorkWeDriveFileRenameResponse = serde_json::from_value(json!({
            "request_id": "rename",
            "file": { "fileid": "file", "file_name": "new.txt", "version": 2 }
        }))
        .unwrap();
        assert_eq!(rename.extra["request_id"], "rename");
        let rename_file = rename.file.unwrap();
        assert_eq!(rename_file.file_name.as_deref(), Some("new.txt"));
        assert_eq!(rename_file.extra["version"], 2);

        let file_info: WorkWeDriveFileInfoResponse = serde_json::from_value(json!({
            "request_id": "file-info",
            "file_info": {
                "fileid": "file",
                "file_name": "doc.txt",
                "spaceid": "space",
                "fatherid": "root",
                "file_size": 10,
                "ctime": 1_800_000_000,
                "mtime": 1_800_000_100,
                "file_type": 2,
                "file_status": 1,
                "sha": "sha-hash",
                "md5": "md5-hash",
                "classification": "internal"
            }
        }))
        .unwrap();
        assert_eq!(file_info.extra["request_id"], "file-info");
        let file_info = file_info.file_info.unwrap();
        assert_eq!(file_info.file_size, Some(10));
        assert_eq!(file_info.file_status, Some(1));
        assert_eq!(file_info.extra["classification"], "internal");

        let moved: WorkWeDriveFileMoveResponse = serde_json::from_value(json!({
            "request_id": "move",
            "file_list": {
                "item": [{
                    "fileid": "file",
                    "file_name": "moved.txt",
                    "spaceid": "space",
                    "fatherid": "archive",
                    "file_size": 10,
                    "ctime": 1_800_000_000,
                    "mtime": 1_800_000_200,
                    "file_type": 2,
                    "file_status": 1,
                    "create_userid": "creator",
                    "update_userid": "editor",
                    "sha": "sha-hash",
                    "md5": "md5-hash",
                    "move_revision": 2
                }],
                "replace_count": 1
            }
        }))
        .unwrap();
        assert_eq!(moved.extra["request_id"], "move");
        let moved = moved.file_list.unwrap();
        assert_eq!(moved.item[0].fileid.as_deref(), Some("file"));
        assert_eq!(moved.item[0].fatherid.as_deref(), Some("archive"));
        assert_eq!(moved.item[0].extra["move_revision"], 2);
        assert_eq!(moved.extra["replace_count"], 1);

        let share: WorkWeDriveFileShareResponse = serde_json::from_value(json!({
            "share_url": "https://example.com/share",
            "expire_time": 1_800_003_600
        }))
        .unwrap();
        assert_eq!(
            share.share_url.as_deref(),
            Some("https://example.com/share")
        );
        assert_eq!(share.extra["expire_time"], 1_800_003_600);
    }

    #[test]
    fn serializes_work_account_service_and_aibot_requests() {
        let account = serde_json::to_value(WorkAccountServiceAccountUpdateRequest {
            open_kfid: "kf".to_string(),
            name: "Support".to_string(),
            media_id: "media".to_string(),
        })
        .unwrap();
        assert_eq!(account["open_kfid"], "kf");
        assert_eq!(account["media_id"], "media");

        let contact_way = serde_json::to_value(WorkAccountServiceAddContactWayRequest {
            open_kfid: "kf".to_string(),
            scene: Some("scene".to_string()),
        })
        .unwrap();
        assert_eq!(contact_way["scene"], "scene");

        let batch = serde_json::to_value(WorkAccountServiceCustomerBatchGetRequest {
            external_userid_list: vec!["external".to_string()],
            need_enter_session_context: 1,
        })
        .unwrap();
        assert_eq!(batch["external_userid_list"][0], "external");

        let upgrade = serde_json::to_value(WorkAccountServiceCustomerUpgradeServiceRequest {
            open_kfid: "kf".to_string(),
            external_userid: "external".to_string(),
            upgrade_type: 1,
            member: Some(json!({ "userid": "servicer", "wording": "hello" })),
            groupchat: None,
        })
        .unwrap();
        assert_eq!(upgrade["type"], 1);
        assert_eq!(upgrade["member"]["userid"], "servicer");
        assert!(upgrade.get("groupchat").is_none());

        let sync = serde_json::to_value(WorkAccountServiceSyncMsgRequest {
            cursor: "cursor".to_string(),
            token: "token".to_string(),
            limit: 100,
            voice_format: 0,
            open_kfid: "kf".to_string(),
        })
        .unwrap();
        assert_eq!(sync["open_kfid"], "kf");

        let send = serde_json::to_value(WorkAccountServiceSendMsgRequest {
            touser: "external".to_string(),
            open_kfid: "kf".to_string(),
            msgid: Some("msg".to_string()),
            msgtype: Some("text".to_string()),
            text: Some(WorkAccountServiceTextMessage {
                content: Some("hello".to_string()),
                menu_id: None,
            }),
            image: Some(WorkAccountServiceMediaMessage {
                media_id: Some("image-media".to_string()),
            }),
            voice: None,
            video: Some(WorkAccountServiceVideoMessage {
                media_id: Some("video-media".to_string()),
                thumb_media_id: Some("thumb-media".to_string()),
            }),
            file: None,
            link: Some(WorkAccountServiceLinkMessage {
                title: Some("docs".to_string()),
                desc: Some("desc".to_string()),
                url: Some("https://example.com".to_string()),
                thumb_media_id: Some("thumb".to_string()),
                pic_url: None,
            }),
            miniprogram: Some(WorkAccountServiceMiniProgramMessage {
                title: Some("mini".to_string()),
                appid: Some("wx-app".to_string()),
                pagepath: Some("pages/index".to_string()),
                thumb_media_id: Some("thumb".to_string()),
            }),
            menu: Some(WorkAccountServiceMenuMessage {
                head_content: Some("choose".to_string()),
                list: vec![WorkAccountServiceMenuItem {
                    id: Some("id-1".to_string()),
                    content: Some("open".to_string()),
                }],
                tail_content: Some("tail".to_string()),
            }),
            location: Some(WorkAccountServiceLocationMessage {
                latitude: Some(31.2),
                longitude: Some(121.5),
                name: Some("office".to_string()),
                address: Some("Shanghai".to_string()),
            }),
            ca_link: Some(WorkAccountServiceLinkMessage {
                title: Some("customer".to_string()),
                desc: None,
                url: Some("https://example.com/customer".to_string()),
                thumb_media_id: None,
                pic_url: None,
            }),
        })
        .unwrap();
        assert_eq!(send["text"]["content"], "hello");
        assert_eq!(send["image"]["media_id"], "image-media");
        assert_eq!(send["video"]["thumb_media_id"], "thumb-media");
        assert_eq!(send["link"]["title"], "docs");
        assert_eq!(send["miniprogram"]["appid"], "wx-app");
        assert_eq!(send["msgmenu"]["head_content"], "choose");
        assert_eq!(send["msgmenu"]["list"][0]["id"], "id-1");
        assert_eq!(send["location"]["name"], "office");
        assert_eq!(send["ca_link"]["title"], "customer");

        let on_event = serde_json::to_value(WorkAccountServiceSendMsgOnEventRequest {
            code: "code".to_string(),
            msgid: "msg".to_string(),
            msgtype: "text".to_string(),
            text: Some(WorkAccountServiceTextMessage {
                content: Some("hello".to_string()),
                menu_id: None,
            }),
            menu: Some(WorkAccountServiceMenuMessage {
                head_content: Some("choose".to_string()),
                list: Vec::new(),
                tail_content: None,
            }),
        })
        .unwrap();
        assert_eq!(on_event["code"], "code");
        assert_eq!(on_event["text"]["content"], "hello");
        assert_eq!(on_event["msgmenu"]["head_content"], "choose");

        let servicers = serde_json::to_value(WorkAccountServiceServicerRequest::new(
            "kf",
            vec!["servicer".to_string()],
        ))
        .unwrap();
        assert_eq!(servicers["open_kfid"], "kf");
        assert_eq!(servicers["userid_list"][0], "servicer");
        assert!(servicers.get("department_id_list").is_none());

        let departments = serde_json::to_value(
            WorkAccountServiceServicerRequest::with_departments("kf", vec![2, 4]),
        )
        .unwrap();
        assert_eq!(departments["department_id_list"], json!([2, 4]));
        assert!(departments.get("userid_list").is_none());

        let state = serde_json::to_value(WorkAccountServiceStateTransRequest {
            open_kfid: "kf".to_string(),
            external_userid: "external".to_string(),
            service_state: 2,
            servicer_userid: "servicer".to_string(),
        })
        .unwrap();
        assert_eq!(state["service_state"], 2);

        let waiting = WorkAccountServiceStateTransRequest::new(
            "kf",
            "external",
            WorkAccountServiceStateKind::WaitingPool,
        );
        assert_eq!(
            waiting.service_state_kind(),
            WorkAccountServiceStateKind::WaitingPool
        );
        let waiting = serde_json::to_value(waiting).unwrap();
        assert_eq!(waiting["service_state"], 2);
        assert!(waiting.get("servicer_userid").is_none());

        let assigned =
            WorkAccountServiceStateTransRequest::with_servicer("kf", "external", "servicer");
        assert_eq!(
            assigned.service_state_kind(),
            WorkAccountServiceStateKind::HumanServicer
        );
        let assigned = serde_json::to_value(assigned).unwrap();
        assert_eq!(assigned["service_state"], 3);
        assert_eq!(assigned["servicer_userid"], "servicer");

        let subscribe =
            serde_json::to_value(Work::aibot_subscribe_request("bot", "secret", "req-1")).unwrap();
        assert_eq!(subscribe["cmd"], WORK_AIBOT_CMD_SUBSCRIBE);
        assert_eq!(subscribe["headers"]["req_id"], "req-1");
        assert_eq!(subscribe["body"]["bot_id"], "bot");

        let ping = serde_json::to_value(Work::aibot_ping_request("req-2")).unwrap();
        assert_eq!(ping["cmd"], WORK_AIBOT_CMD_PING);
        assert!(ping.get("body").is_none());

        let command = serde_json::to_value(Work::aibot_command_request(
            WORK_AIBOT_CMD_SEND_MESSAGE,
            Some("req-3".to_string()),
            Some(json!({ "content": "hello" })),
        ))
        .unwrap();
        assert_eq!(command["cmd"], WORK_AIBOT_CMD_SEND_MESSAGE);
        assert_eq!(command["body"]["content"], "hello");
        assert_eq!(
            Work::aibot_long_connection_url(None),
            "wss://openws.work.weixin.qq.com"
        );
    }

    #[test]
    fn deserializes_work_account_service_and_aibot_responses() {
        let account_add: WorkAccountServiceAccountAddResponse = serde_json::from_value(json!({
            "open_kfid": "kf",
            "request_id": "account-add"
        }))
        .unwrap();
        assert_eq!(account_add.open_kfid.as_deref(), Some("kf"));
        assert_eq!(account_add.extra["request_id"], "account-add");

        let accounts: WorkAccountServiceAccountListResponse = serde_json::from_value(json!({
            "account_list": [{
                "open_kfid": "kf",
                "name": "Support",
                "avatar": "https://example.com/a.png",
                "account_extra": "retained"
            }],
            "request_id": "account-list"
        }))
        .unwrap();
        assert_eq!(accounts.extra["request_id"], "account-list");
        assert_eq!(accounts.account_list[0].open_kfid.as_deref(), Some("kf"));
        assert_eq!(accounts.account_list[0].name.as_deref(), Some("Support"));
        assert_eq!(
            accounts.account_list[0].avatar.as_deref(),
            Some("https://example.com/a.png")
        );
        assert_eq!(accounts.account_list[0].extra["account_extra"], "retained");

        let contact_way: WorkAccountServiceAddContactWayResponse = serde_json::from_value(json!({
            "url": "https://example.com/kf",
            "request_id": "contact-way"
        }))
        .unwrap();
        assert_eq!(contact_way.url.as_deref(), Some("https://example.com/kf"));
        assert_eq!(contact_way.extra["request_id"], "contact-way");

        let customers: WorkAccountServiceCustomerBatchGetResponse = serde_json::from_value(json!({
            "customer_list": [{
                "external_userid": "external",
                "nickname": "Customer",
                "gender": 1,
                "enter_session_context": {
                    "scene": "scene",
                    "scene_param": "param",
                    "context_extra": "retained"
                },
                "customer_extra": "retained"
            }],
            "invalid_external_userid": ["bad"],
            "request_id": "customer-batch"
        }))
        .unwrap();
        assert_eq!(customers.extra["request_id"], "customer-batch");
        assert_eq!(
            customers.customer_list[0].external_userid.as_deref(),
            Some("external")
        );
        assert_eq!(
            customers.customer_list[0].nickname.as_deref(),
            Some("Customer")
        );
        assert_eq!(
            customers.customer_list[0]
                .enter_session_context
                .as_ref()
                .unwrap()
                .scene
                .as_deref(),
            Some("scene")
        );
        assert_eq!(
            customers.customer_list[0].extra["customer_extra"],
            "retained"
        );
        assert_eq!(
            customers.customer_list[0]
                .enter_session_context
                .as_ref()
                .unwrap()
                .extra["context_extra"],
            "retained"
        );

        let config: WorkAccountServiceCustomerUpgradeServiceConfigResponse =
            serde_json::from_value(json!({
                "member_range": { "userid": ["servicer"] },
                "groupchat_range": { "chat_id": ["chat"] },
                "request_id": "upgrade-config"
            }))
            .unwrap();
        assert_eq!(
            config.member_range.as_ref().unwrap()["userid"][0],
            "servicer"
        );
        assert_eq!(config.extra["request_id"], "upgrade-config");

        let sync: WorkAccountServiceSyncMsgResponse = serde_json::from_value(json!({
            "next_cursor": "next",
            "has_more": 1,
            "sync_id": "sync-msg",
            "msg_list": [
                {
                    "msgid": "msg",
                    "open_kfid": "kf",
                    "external_userid": "external",
                    "send_time": 100,
                    "origin": 3,
                    "msgtype": "text",
                    "text": { "content": "hello", "menu_id": "clicked-menu" },
                    "msg_source": "customer"
                },
                {
                    "msgid": "image-msg",
                    "origin": 5,
                    "servicer_userid": "servicer",
                    "msgtype": "image",
                    "image": { "media_id": "image-media" }
                },
                {
                    "msgid": "link-msg",
                    "msgtype": "link",
                    "link": {
                        "title": "Docs",
                        "desc": "Read",
                        "url": "https://example.com",
                        "thumb_media_id": "thumb",
                        "pic_url": "https://example.com/thumb.png"
                    }
                },
                {
                    "msgid": "menu-msg",
                    "msgtype": "msgmenu",
                    "msgmenu": {
                        "head_content": "choose",
                        "list": [{ "id": "m1", "content": "Option" }],
                        "tail_content": "tail"
                    }
                },
                {
                    "msgid": "event-msg",
                    "msgtype": "event",
                    "event": {
                        "event_type": "enter_session",
                        "open_kfid": "kf",
                        "external_userid": "external",
                        "scene": "scene",
                        "scene_param": "param",
                        "welcome_code": "welcome",
                        "event_source": "qr"
                    },
                    "event_seq": 5
                },
                {
                    "msgid": "failed-msg",
                    "msgtype": "event",
                    "event": {
                        "event_type": "msg_send_fail",
                        "fail_msgid": "outbound-msg",
                        "fail_type": 10
                    }
                },
                {
                    "msgid": "servicer-event",
                    "msgtype": "event",
                    "event": {
                        "event_type": "servicer_status_change",
                        "servicer_userid": "servicer",
                        "status": 1
                    }
                },
                {
                    "msgid": "session-event",
                    "msgtype": "event",
                    "event": {
                        "event_type": "session_status_change",
                        "old_servicer_userid": "old",
                        "new_servicer_userid": "new",
                        "change_type": 2,
                        "msg_code": "event-code"
                    }
                },
                {
                    "msgid": "recall-event",
                    "msgtype": "event",
                    "event": {
                        "event_type": "servicer_recall_msg",
                        "recall_msgid": "recalled",
                        "servicer_userid": "servicer"
                    }
                },
                {
                    "msgid": "product-msg",
                    "origin": 6,
                    "msgtype": "channels_shop_product",
                    "channels_shop_product": {
                        "product_id": "product",
                        "head_img": "https://example.com/product.png",
                        "title": "Product",
                        "sales_price": "100",
                        "shop_nickname": "Shop",
                        "shop_head_img": "https://example.com/shop.png",
                        "currency": "CNY"
                    }
                },
                {
                    "msgid": "order-msg",
                    "origin": 4,
                    "msgtype": "channels_shop_order",
                    "channels_shop_order": {
                        "order_id": "order",
                        "product_titles": "Product",
                        "price_wording": "¥1.00",
                        "state": "paid",
                        "image_url": "https://example.com/order.png",
                        "shop_nickname": "Shop",
                        "order_version": 2
                    }
                }
            ]
        }))
        .unwrap();
        assert_eq!(sync.next_cursor.as_deref(), Some("next"));
        assert_eq!(sync.extra["sync_id"], "sync-msg");
        assert_eq!(sync.msg_list[0].msgid.as_deref(), Some("msg"));
        assert_eq!(sync.msg_list[0].open_kfid.as_deref(), Some("kf"));
        assert_eq!(sync.msg_list[0].msgtype.as_deref(), Some("text"));
        assert_eq!(
            sync.msg_list[0].msgtype_kind(),
            Some(WorkAccountServiceMessageTypeKind::Text)
        );
        assert_eq!(
            sync.msg_list[0].origin_kind(),
            Some(WorkAccountServiceMessageOriginKind::Customer)
        );
        assert_eq!(sync.msg_list[0].extra["msg_source"], "customer");
        assert_eq!(
            sync.msg_list[0]
                .text
                .as_ref()
                .expect("text")
                .content
                .as_deref(),
            Some("hello")
        );
        assert_eq!(
            sync.msg_list[0]
                .text
                .as_ref()
                .expect("text")
                .menu_id
                .as_deref(),
            Some("clicked-menu")
        );
        assert_eq!(
            sync.msg_list[1]
                .image
                .as_ref()
                .expect("image")
                .media_id
                .as_deref(),
            Some("image-media")
        );
        assert_eq!(
            sync.msg_list[1].origin_kind(),
            Some(WorkAccountServiceMessageOriginKind::Servicer)
        );
        assert_eq!(
            sync.msg_list[1].servicer_userid.as_deref(),
            Some("servicer")
        );
        assert_eq!(
            sync.msg_list[2].link.as_ref().expect("link").url.as_deref(),
            Some("https://example.com")
        );
        assert_eq!(
            sync.msg_list[2]
                .link
                .as_ref()
                .expect("link")
                .pic_url
                .as_deref(),
            Some("https://example.com/thumb.png")
        );
        assert_eq!(
            sync.msg_list[3].msgmenu.as_ref().expect("msgmenu").list[0]
                .content
                .as_deref(),
            Some("Option")
        );
        assert_eq!(
            sync.msg_list[4]
                .event
                .as_ref()
                .expect("event")
                .welcome_code
                .as_deref(),
            Some("welcome")
        );
        assert_eq!(sync.msg_list[4].extra["event_seq"], 5);
        assert_eq!(
            sync.msg_list[4].event.as_ref().expect("event").extra["event_source"],
            "qr"
        );
        let failed = sync.msg_list[5].event.as_ref().expect("failed event");
        assert_eq!(
            failed.event_type_kind(),
            Some(WorkAccountServiceEventTypeKind::MessageSendFailed)
        );
        assert_eq!(failed.fail_msgid.as_deref(), Some("outbound-msg"));
        assert_eq!(
            failed.fail_type_kind(),
            Some(WorkAccountServiceMessageFailKind::UserRejected)
        );
        let servicer = sync.msg_list[6].event.as_ref().expect("servicer event");
        assert_eq!(
            servicer.event_type_kind(),
            Some(WorkAccountServiceEventTypeKind::ServicerStatusChanged)
        );
        assert_eq!(servicer.servicer_userid.as_deref(), Some("servicer"));
        assert_eq!(
            servicer.servicer_status_kind(),
            Some(WorkAccountServiceServicerEventStatusKind::Receiving)
        );
        let session = sync.msg_list[7].event.as_ref().expect("session event");
        assert_eq!(
            session.event_type_kind(),
            Some(WorkAccountServiceEventTypeKind::SessionStatusChanged)
        );
        assert_eq!(
            session.session_change_kind(),
            Some(WorkAccountServiceSessionChangeKind::Transferred)
        );
        assert_eq!(session.old_servicer_userid.as_deref(), Some("old"));
        assert_eq!(session.new_servicer_userid.as_deref(), Some("new"));
        assert_eq!(session.msg_code.as_deref(), Some("event-code"));
        let recall = sync.msg_list[8].event.as_ref().expect("recall event");
        assert_eq!(
            recall.event_type_kind(),
            Some(WorkAccountServiceEventTypeKind::ServicerRecalledMessage)
        );
        assert_eq!(recall.recall_msgid.as_deref(), Some("recalled"));
        assert_eq!(
            WorkAccountServiceMessageFailKind::from(99),
            WorkAccountServiceMessageFailKind::Other(99)
        );
        assert_eq!(
            WorkAccountServiceSessionChangeKind::from(99),
            WorkAccountServiceSessionChangeKind::Other(99)
        );
        let product = sync.msg_list[9]
            .channels_shop_product
            .as_ref()
            .expect("channels product");
        assert_eq!(
            sync.msg_list[9].msgtype_kind(),
            Some(WorkAccountServiceMessageTypeKind::ChannelsShopProduct)
        );
        assert_eq!(
            sync.msg_list[9].origin_kind(),
            Some(WorkAccountServiceMessageOriginKind::IntelligentAssistant)
        );
        assert_eq!(product.product_id.as_deref(), Some("product"));
        assert_eq!(product.extra["currency"], "CNY");
        let order = sync.msg_list[10]
            .channels_shop_order
            .as_ref()
            .expect("channels order");
        assert_eq!(
            sync.msg_list[10].msgtype_kind(),
            Some(WorkAccountServiceMessageTypeKind::ChannelsShopOrder)
        );
        assert_eq!(
            sync.msg_list[10].origin_kind(),
            Some(WorkAccountServiceMessageOriginKind::System)
        );
        assert_eq!(order.order_id.as_deref(), Some("order"));
        assert_eq!(order.extra["order_version"], 2);
        assert_eq!(
            WorkAccountServiceMessageOriginKind::from(99),
            WorkAccountServiceMessageOriginKind::Other(99)
        );
        assert_eq!(
            WorkAccountServiceMessageTypeKind::from_code("future"),
            WorkAccountServiceMessageTypeKind::Other
        );
        assert_eq!(
            WorkAccountServiceMessageTypeKind::Other.as_code(),
            "unknown"
        );

        let send: WorkAccountServiceSendMsgResponse =
            serde_json::from_value(json!({ "msgid": "msg", "request_id": "send-msg" })).unwrap();
        assert_eq!(send.msgid.as_deref(), Some("msg"));
        assert_eq!(send.extra["request_id"], "send-msg");

        let servicer_result: WorkAccountServiceServicerResultResponse =
            serde_json::from_value(json!({
                "result_list": [
                    { "userid": "servicer", "errcode": 0, "result_source": "bind" },
                    { "department_id": 2, "errcode": 0 }
                ],
                "request_id": "servicer-result"
            }))
            .unwrap();
        assert_eq!(servicer_result.extra["request_id"], "servicer-result");
        assert_eq!(
            servicer_result.result_list[0].userid.as_deref(),
            Some("servicer")
        );
        assert_eq!(servicer_result.result_list[0].errcode, Some(0));
        assert_eq!(
            servicer_result.result_list[0].extra["result_source"],
            "bind"
        );
        assert_eq!(servicer_result.result_list[1].department_id, Some(2));

        let servicers: WorkAccountServiceServicerListResponse = serde_json::from_value(json!({
            "servicer_list": [
                {
                    "userid": "servicer",
                    "status": 1,
                    "stop_type": 1,
                    "online_status": "ready"
                },
                { "userid": "receiving", "status": 0 },
                { "department_id": 2 }
            ],
            "request_id": "servicer-list"
        }))
        .unwrap();
        assert_eq!(servicers.extra["request_id"], "servicer-list");
        assert_eq!(
            servicers.servicer_list[0].userid.as_deref(),
            Some("servicer")
        );
        assert_eq!(servicers.servicer_list[0].status, Some(1));
        assert_eq!(
            servicers.servicer_list[0].status_kind(),
            Some(WorkAccountServiceServicerStatusKind::Stopped)
        );
        assert_eq!(
            servicers.servicer_list[0].stop_kind(),
            Some(WorkAccountServiceServicerStopKind::Suspended)
        );
        assert!(!servicers.servicer_list[0].is_receiving());
        assert_eq!(servicers.servicer_list[0].extra["online_status"], "ready");
        assert!(servicers.servicer_list[1].is_receiving());
        assert_eq!(servicers.servicer_list[2].department_id, Some(2));
        assert_eq!(
            WorkAccountServiceServicerStatusKind::from(99),
            WorkAccountServiceServicerStatusKind::Other(99)
        );

        let state: WorkAccountServiceStateGetResponse = serde_json::from_value(json!({
            "service_state": 2,
            "servicer_userid": "servicer",
            "state_source": "api"
        }))
        .unwrap();
        assert_eq!(state.service_state, Some(2));
        assert_eq!(
            state.service_state_kind(),
            Some(WorkAccountServiceStateKind::WaitingPool)
        );
        assert_eq!(state.extra["state_source"], "api");
        assert!(WorkAccountServiceStateKind::Ended.is_terminal());
        assert!(!WorkAccountServiceStateKind::HumanServicer.is_terminal());
        assert_eq!(
            WorkAccountServiceStateKind::from_code(99),
            WorkAccountServiceStateKind::Other(99)
        );

        let state_trans: WorkAccountServiceStateTransResponse = serde_json::from_value(json!({
            "errcode": 0,
            "errmsg": "ok",
            "msg_code": "state-code",
            "request_id": "state-trans"
        }))
        .unwrap();
        assert_eq!(state_trans.msg_code.as_deref(), Some("state-code"));
        assert_eq!(state_trans.extra["request_id"], "state-trans");

        let tag_create: WorkAccountServiceTagCreateResponse =
            serde_json::from_value(json!({ "tagid": 1, "request_id": "tag-create" })).unwrap();
        assert_eq!(tag_create.tagid, Some(1));
        assert_eq!(tag_create.extra["request_id"], "tag-create");

        let tag_detail: WorkAccountServiceTagDetailResponse = serde_json::from_value(json!({
            "tagname": "tag",
            "userlist": [{ "userid": "user", "name": "User", "member_role": "owner" }],
            "partylist": [1],
            "tag_source": "kf"
        }))
        .unwrap();
        assert_eq!(tag_detail.tagname.as_deref(), Some("tag"));
        assert_eq!(tag_detail.extra["tag_source"], "kf");
        assert_eq!(tag_detail.userlist[0].userid.as_deref(), Some("user"));
        assert_eq!(tag_detail.userlist[0].name.as_deref(), Some("User"));
        assert_eq!(tag_detail.userlist[0].extra["member_role"], "owner");
        assert_eq!(tag_detail.partylist[0], 1);

        let tag_user: WorkAccountServiceTagUserResultResponse = serde_json::from_value(json!({
            "invalidlist": "bad",
            "invalidparty": [2],
            "request_id": "tag-user"
        }))
        .unwrap();
        assert_eq!(tag_user.invalidparty[0], 2);
        assert_eq!(tag_user.extra["request_id"], "tag-user");

        let tags: WorkAccountServiceTagListResponse = serde_json::from_value(json!({
            "taglist": [{ "tagid": 1, "tagname": "tag", "member_count": 2 }],
            "request_id": "tag-list"
        }))
        .unwrap();
        assert_eq!(tags.extra["request_id"], "tag-list");
        assert_eq!(tags.taglist[0].tagid, Some(1));
        assert_eq!(tags.taglist[0].tagname.as_deref(), Some("tag"));
        assert_eq!(tags.taglist[0].extra["member_count"], 2);

        let ok: WorkAiBotLongConnectionResponse = serde_json::from_value(json!({
            "cmd": "pong",
            "headers": { "req_id": "req-1" },
            "body": { "ok": true },
            "trace_id": "aibot-ok"
        }))
        .unwrap();
        assert!(!ok.is_error());
        assert_eq!(ok.headers.unwrap().req_id.as_deref(), Some("req-1"));
        assert_eq!(ok.extra["trace_id"], "aibot-ok");

        let err: WorkAiBotLongConnectionResponse =
            serde_json::from_value(json!({ "errcode": 40001, "errmsg": "invalid" })).unwrap();
        assert!(err.is_error());
        assert_eq!(err.errmsg.as_deref(), Some("invalid"));
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

        let scope = serde_json::to_value(
            WorkAgentScopeRequest::new(100001)
                .with_users(["user"])
                .with_parties([2, 3])
                .with_tags([4]),
        )
        .unwrap();
        assert_eq!(scope["agentid"], 100001);
        assert_eq!(scope["allow_userinfos"]["user"][0]["userid"], "user");
        assert_eq!(scope["allow_partys"]["partyid"][1], 3);
        assert_eq!(scope["allow_tags"]["tagid"][0], 4);

        let template = serde_json::to_value(WorkAgentWorkbenchTemplateRequest {
            agentid: 100001,
            template_type: "keydata".to_string(),
            keydata: Some(WorkAgentWorkbenchKeyDataTemplate {
                items: vec![WorkAgentWorkbenchKeyDataItem {
                    key: Some("today".to_string()),
                    data: Some("10".to_string()),
                    jump_url: None,
                    pagepath: None,
                }],
            }),
            image: None,
            list: None,
            webview: None,
        })
        .unwrap();
        assert_eq!(template["type"], "keydata");
        assert_eq!(template["keydata"]["items"][0]["key"], "today");
        assert!(template.get("image").is_none());
        assert!(template["keydata"]["items"][0]
            .as_object()
            .unwrap()
            .get("jump_url")
            .is_none());

        let data = serde_json::to_value(WorkAgentWorkbenchDataRequest {
            agentid: 100001,
            userid: "user".to_string(),
            template_type: "webview".to_string(),
            keydata: None,
            image: None,
            list: None,
            webview: Some(WorkAgentWorkbenchWebviewTemplate {
                url: Some("https://example.com/workbench".to_string()),
            }),
        })
        .unwrap();
        assert_eq!(data["userid"], "user");
        assert_eq!(data["type"], "webview");
        assert_eq!(data["webview"]["url"], "https://example.com/workbench");
    }

    #[test]
    fn serializes_msg_audit_requests_and_responses() {
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

        let request =
            WorkMsgAuditCheckSingleAgreeRequest::new([WorkMsgAuditConversationPair::new(
                "user", "openid",
            )]);
        request.validate().unwrap();
        let single_agree = serde_json::to_value(request).unwrap();
        assert_eq!(single_agree["info"][0]["userid"], "user");
        assert_eq!(single_agree["info"][0]["exteranalopenid"], "openid");
        assert!(single_agree["info"][0].get("external_openid").is_none());

        assert!(WorkMsgAuditCheckSingleAgreeRequest::new([])
            .validate()
            .is_err());
        assert!(
            WorkMsgAuditCheckSingleAgreeRequest::new([WorkMsgAuditConversationPair::new(
                " ", "openid"
            )])
            .validate()
            .is_err()
        );
        assert!(
            WorkMsgAuditCheckSingleAgreeRequest::new([WorkMsgAuditConversationPair::new(
                "user", ""
            )])
            .validate()
            .is_err()
        );

        let room_agree = json!({ "roomid": "room" });
        assert_eq!(room_agree["roomid"], "room");

        let permit: WorkMsgAuditPermitUsersResponse = serde_json::from_value(json!({
            "errcode": 0,
            "ids": ["user", "external-openid"],
            "request_id": "permit-users"
        }))
        .unwrap();
        assert_eq!(permit.ids[0], "user");
        assert_eq!(permit.ids[1], "external-openid");
        assert_eq!(permit.extra["request_id"], "permit-users");

        let chat_data: WorkMsgAuditChatDataResponse = serde_json::from_value(json!({
            "errcode": 0,
            "chatdata": [{
                "seq": 1,
                "msgid": "msg",
                "publickey_ver": 2,
                "encrypt_random_key": "random",
                "encrypt_chat_msg": "cipher",
                "chat_type": "single"
            }],
            "next_seq": 2
        }))
        .unwrap();
        assert_eq!(chat_data.chatdata[0].seq, Some(1));
        assert_eq!(chat_data.chatdata[0].msgid.as_deref(), Some("msg"));
        assert_eq!(chat_data.chatdata[0].publickey_ver, Some(2));
        assert_eq!(chat_data.chatdata[0].extra["chat_type"], "single");
        assert_eq!(chat_data.extra["next_seq"], 2);

        let room: WorkMsgAuditRoomResponse = serde_json::from_value(json!({
            "errcode": 0,
            "roomname": "room",
            "creator": "creator",
            "room_create_time": 1_720_000_000,
            "notice": "notice",
            "members": [{ "memberid": "user", "jointime": 1_720_000_001, "member_role": "owner" }],
            "room_status": "active"
        }))
        .unwrap();
        assert_eq!(room.roomname.as_deref(), Some("room"));
        assert_eq!(room.members[0].memberid.as_deref(), Some("user"));
        assert_eq!(room.members[0].extra["member_role"], "owner");
        assert_eq!(room.extra["room_status"], "active");

        let agree: WorkMsgAuditAgreeResponse = serde_json::from_value(json!({
            "errcode": 0,
            "agreeinfo": [{
                "userid": "user",
                "exteranalopenid": "openid",
                "agree_status": "Agree",
                "status_change_time": 1_720_000_003,
                "source": "single"
            }],
            "agree_total": 1
        }))
        .unwrap();
        assert!(agree.all_agreed());
        assert!(!agree.has_disagreement());
        assert_eq!(agree.agreeinfo[0].userid.as_deref(), Some("user"));
        assert_eq!(
            agree.agreeinfo[0].external_openid.as_deref(),
            Some("openid")
        );
        assert_eq!(
            agree.agreeinfo[0].status_kind(),
            Some(WorkMsgAuditAgreeStatusKind::Agree)
        );
        assert_eq!(agree.agreeinfo[0].status_change_time, Some(1_720_000_003));
        assert_eq!(agree.agreeinfo[0].extra["source"], "single");
        assert_eq!(agree.extra["agree_total"], 1);

        let room_agree: WorkMsgAuditAgreeResponse = serde_json::from_value(json!({
            "agreeinfo": [{
                "exteranalopenid": "external",
                "agree_status": "Disagree",
                "status_change_time": 1_720_000_004
            }, {
                "exteranalopenid": "future",
                "agree_status": "Pending"
            }]
        }))
        .unwrap();
        assert!(room_agree.agreeinfo[0].userid.is_none());
        assert!(room_agree.has_disagreement());
        assert!(!room_agree.all_agreed());
        assert_eq!(
            room_agree.agreeinfo[1].status_kind(),
            Some(WorkMsgAuditAgreeStatusKind::Other)
        );

        let empty_agree: WorkMsgAuditAgreeResponse =
            serde_json::from_value(json!({ "agreeinfo": [] })).unwrap();
        assert!(!empty_agree.all_agreed());
        assert!(!empty_agree.has_disagreement());

        let robot: WorkMsgAuditRobotInfoResponse = serde_json::from_value(json!({
            "errcode": 0,
            "robot_info": {
                "robot_id": "robot",
                "name": "Robot",
                "creator_userid": "creator",
                "robot_status": "enabled"
            },
            "request_id": "robot-info"
        }))
        .unwrap();
        assert_eq!(robot.extra["request_id"], "robot-info");
        let robot_info = robot.robot_info.unwrap();
        assert_eq!(robot_info.robot_id.as_deref(), Some("robot"));
        assert_eq!(robot_info.creator_userid.as_deref(), Some("creator"));
        assert_eq!(robot_info.extra["robot_status"], "enabled");
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
        let appchat_message = AppChatMessage::text("chatid", "hello");
        assert_eq!(appchat_message.msgtype_kind(), WorkMessageTypeKind::Text);
        let message = serde_json::to_value(appchat_message).unwrap();

        assert_eq!(create["userlist"][0], "user");
        assert_eq!(message["chatid"], "chatid");
        assert_eq!(message["text"]["content"], "hello");
        assert!(message.get("image").is_none());

        let image =
            serde_json::to_value(AppChatMessage::image("chatid", "image-media").with_safe(true))
                .unwrap();
        assert_eq!(image["msgtype"], "image");
        assert_eq!(image["image"]["media_id"], "image-media");
        assert_eq!(image["safe"], 1);

        let voice = serde_json::to_value(AppChatMessage::voice("chatid", "voice-media")).unwrap();
        assert_eq!(voice["msgtype"], "voice");
        assert_eq!(voice["voice"]["media_id"], "voice-media");

        let video = serde_json::to_value(AppChatMessage::video(
            "chatid",
            WorkVideoMessage {
                media_id: "video-media".to_string(),
                title: Some("title".to_string()),
                description: Some("description".to_string()),
            },
        ))
        .unwrap();
        assert_eq!(video["msgtype"], "video");
        assert_eq!(video["video"]["title"], "title");

        let file = serde_json::to_value(AppChatMessage::file("chatid", "file-media")).unwrap();
        assert_eq!(file["msgtype"], "file");
        assert_eq!(file["file"]["media_id"], "file-media");

        let text_card = serde_json::to_value(AppChatMessage::text_card(
            "chatid",
            WorkTextCardMessage {
                title: "title".to_string(),
                description: "description".to_string(),
                url: "https://example.com/card".to_string(),
                btntxt: Some("details".to_string()),
            },
        ))
        .unwrap();
        assert_eq!(text_card["msgtype"], "textcard");
        assert_eq!(text_card["textcard"]["btntxt"], "details");

        let news = serde_json::to_value(AppChatMessage::news(
            "chatid",
            vec![WorkNewsArticle {
                title: "news".to_string(),
                description: "description".to_string(),
                url: "https://example.com/news".to_string(),
                picurl: "https://example.com/news.png".to_string(),
            }],
        ))
        .unwrap();
        assert_eq!(news["msgtype"], "news");
        assert_eq!(news["news"]["articles"][0]["title"], "news");

        let mpnews = serde_json::to_value(AppChatMessage::mpnews(
            "chatid",
            vec![WorkMpNewsArticle {
                title: "mpnews".to_string(),
                thumb_media_id: "thumb-media".to_string(),
                author: "author".to_string(),
                content_source_url: "https://example.com/source".to_string(),
                content: "content".to_string(),
                digest: "digest".to_string(),
            }],
        ))
        .unwrap();
        assert_eq!(mpnews["msgtype"], "mpnews");
        assert_eq!(
            mpnews["mpnews"]["articles"][0]["thumb_media_id"],
            "thumb-media"
        );

        let markdown =
            serde_json::to_value(AppChatMessage::markdown("chatid", "**hello**")).unwrap();
        assert_eq!(markdown["msgtype"], "markdown");
        assert_eq!(markdown["markdown"]["content"], "**hello**");
        assert_eq!(
            AppChatMessage::markdown("chatid", "hello").msgtype_kind(),
            WorkMessageTypeKind::Markdown
        );

        let created: WorkAppChatCreateResponse = serde_json::from_value(
            json!({ "errcode": 0, "chatid": "chatid", "request_id": "appchat-create" }),
        )
        .unwrap();
        assert_eq!(created.chatid.as_deref(), Some("chatid"));
        assert_eq!(created.extra["request_id"], "appchat-create");

        let got: WorkAppChatGetResponse = serde_json::from_value(json!({
            "errcode": 0,
            "trace_id": "appchat-get",
            "chat_info": {
                "chatid": "chatid",
                "name": "chat",
                "owner": "owner",
                "userlist": ["user"],
                "member_count": 1
            }
        }))
        .unwrap();
        assert_eq!(got.extra["trace_id"], "appchat-get");
        let chat = got.chat_info.unwrap();
        assert_eq!(chat.chatid.as_deref(), Some("chatid"));
        assert_eq!(chat.owner.as_deref(), Some("owner"));
        assert_eq!(chat.userlist[0], "user");
        assert_eq!(chat.extra["member_count"], 1);
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

        let info: WorkOauthUserInfoResponse = serde_json::from_value(json!({
            "errcode": 0,
            "UserId": "legacy-user",
            "user_ticket": "ticket",
            "expires_in": 7200,
            "OpenId": "legacy-openid",
            "external_userid": "external",
            "identity_source": "oauth"
        }))
        .unwrap();
        assert_eq!(info.userid.as_deref(), Some("legacy-user"));
        assert_eq!(info.user_ticket.as_deref(), Some("ticket"));
        assert_eq!(info.openid.as_deref(), Some("legacy-openid"));
        assert_eq!(info.external_userid.as_deref(), Some("external"));
        assert_eq!(info.extra["identity_source"], "oauth");

        let detail: WorkOauthUserDetailResponse = serde_json::from_value(json!({
            "errcode": 0,
            "userid": "user",
            "name": "User",
            "gender": "1",
            "avatar": "https://example.com/avatar.png",
            "qr_code": "https://example.com/qr",
            "mobile": "13800000000",
            "email": "user@example.com",
            "biz_mail": "user@corp.example",
            "address": "addr",
            "department": [1, 2]
        }))
        .unwrap();
        assert_eq!(detail.userid.as_deref(), Some("user"));
        assert_eq!(detail.name.as_deref(), Some("User"));
        assert_eq!(detail.mobile.as_deref(), Some("13800000000"));
        assert_eq!(detail.extra["department"][0], 1);
    }

    #[test]
    fn serializes_work_user_tfa_requests_and_response() {
        let info_request = serde_json::to_value(WorkUserTfaInfoRequest {
            code: "single-use-code".to_string(),
        })
        .unwrap();
        assert_eq!(info_request, json!({ "code": "single-use-code" }));

        let info: WorkUserTfaInfoResponse = serde_json::from_value(json!({
            "errcode": 0,
            "errmsg": "ok",
            "userid": "zhangsan",
            "tfa_code": "single-use-tfa-code",
            "expires_in": 300
        }))
        .unwrap();
        assert_eq!(info.errcode, Some(0));
        assert_eq!(info.userid.as_deref(), Some("zhangsan"));
        assert_eq!(info.tfa_code.as_deref(), Some("single-use-tfa-code"));
        assert_eq!(info.extra["expires_in"], 300);

        let success_request = serde_json::to_value(WorkUserTfaSuccessRequest {
            userid: "zhangsan".to_string(),
            tfa_code: "single-use-tfa-code".to_string(),
        })
        .unwrap();
        assert_eq!(
            success_request,
            json!({
                "userid": "zhangsan",
                "tfa_code": "single-use-tfa-code"
            })
        );
    }

    #[test]
    fn serializes_work_wedoc_smartsheet_requests() {
        let add_request = WorkWeDocSmartSheetAddRequest::titled_at("doc", "Tasks", 2);
        add_request.validate().unwrap();
        let add = serde_json::to_value(add_request).unwrap();
        assert_eq!(add["properties"]["title"], "Tasks");
        assert_eq!(add["properties"]["index"], 2);
        assert!(add["properties"].get("sheet_id").is_none());

        let get = serde_json::to_value(WorkWeDocSmartSheetGetRequest {
            docid: "doc".to_string(),
            sheet_id: Some("sheet".to_string()),
            need_all_type_sheet: Some(true),
        })
        .unwrap();
        assert_eq!(get["sheet_id"], "sheet");
        assert_eq!(get["need_all_type_sheet"], true);

        let view_request = WorkWeDocSmartSheetAddViewRequest::calendar(
            "doc",
            "sheet",
            "Calendar",
            WorkWeDocSmartSheetViewDateRange::new("field-start", "field-end"),
        );
        view_request.validate().unwrap();
        let view = serde_json::to_value(view_request).unwrap();
        assert!(view.get("property_gantt").is_none());
        assert_eq!(
            view["property_calendar"]["start_date_field_id"],
            "field-start"
        );

        let fields_request = WorkWeDocSmartSheetFieldsMutationRequest::new(
            "doc",
            "sheet",
            [WorkWeDocSmartSheetFieldMutation::add(
                "Owner",
                WorkWeDocSmartSheetFieldTypeKind::User,
            )
            .with_property(WorkWeDocSmartSheetFieldProperty::user(true, false))],
        );
        fields_request.validate_for_add().unwrap();
        let fields = serde_json::to_value(fields_request).unwrap();
        assert_eq!(fields["fields"][0]["field_title"], "Owner");
        assert_eq!(fields["fields"][0]["field_type"], "FIELD_TYPE_USER");
        assert_eq!(fields["fields"][0]["property"]["is_multiple"], true);

        let records = serde_json::to_value(WorkWeDocSmartSheetGetRecordsRequest {
            docid: "doc".to_string(),
            sheet_id: "sheet".to_string(),
            view_id: Some("view".to_string()),
            record_ids: vec!["record".to_string()],
            key_type: Some("CELL_VALUE_KEY_TYPE_FIELD_ID".to_string()),
            field_titles: Vec::new(),
            field_ids: vec!["field-owner".to_string()],
            sort: vec![WorkWeDocSmartSheetRecordSort::desc("field-owner")],
            offset: Some(10),
            limit: Some(100),
            ver: Some(7),
            filter_spec: None,
        })
        .unwrap();
        assert!(records.get("field_titles").is_none());
        assert_eq!(records["record_ids"][0], "record");
        assert_eq!(records["sort"][0]["desc"], true);

        let deletes = serde_json::to_value(WorkWeDocSmartSheetDeleteRecordsRequest {
            docid: "doc".to_string(),
            sheet_id: "sheet".to_string(),
            record_ids: vec!["record-1".to_string(), "record-2".to_string()],
        })
        .unwrap();
        assert_eq!(deletes["record_ids"][1], "record-2");
    }

    #[test]
    fn validates_work_wedoc_smartsheet_lifecycle_requests() {
        let rename =
            WorkWeDocSmartSheetUpdateRequest::rename("doc", "sheet-tasks", "Production tasks");
        rename.validate().unwrap();
        let value = serde_json::to_value(rename).unwrap();
        assert_eq!(value["properties"]["sheet_id"], "sheet-tasks");
        assert_eq!(value["properties"]["title"], "Production tasks");
        assert!(value["properties"].get("index").is_none());

        let invalid_add = WorkWeDocSmartSheetAddRequest::at_index("doc", -1);
        assert!(invalid_add.validate().is_err());

        let invalid_update = WorkWeDocSmartSheetUpdateRequest {
            docid: "doc".to_string(),
            properties: WorkWeDocSmartSheetProperties::for_update("sheet", "Tasks").with_index(2),
        };
        assert!(invalid_update.validate().is_err());

        let invalid_get = WorkWeDocSmartSheetGetRequest {
            docid: "doc".to_string(),
            sheet_id: Some(" ".to_string()),
            need_all_type_sheet: None,
        };
        assert!(invalid_get.validate().is_err());

        let invalid_delete = WorkWeDocSmartSheetDeleteRequest {
            docid: "doc".to_string(),
            sheet_id: String::new(),
        };
        assert!(invalid_delete.validate().is_err());
    }

    #[test]
    fn serializes_and_validates_work_wedoc_smartsheet_views() {
        let gantt = WorkWeDocSmartSheetAddViewRequest::gantt(
            "doc",
            "sheet",
            "Schedule",
            WorkWeDocSmartSheetViewDateRange::new("field-start", "field-end"),
        );
        gantt.validate().unwrap();
        assert_eq!(
            gantt.view_type_kind(),
            WorkWeDocSmartSheetViewTypeKind::Gantt
        );
        let value = serde_json::to_value(gantt).unwrap();
        assert_eq!(value["view_type"], "VIEW_TYPE_GANTT");
        assert_eq!(value["property_gantt"]["end_date_field_id"], "field-end");

        let property = WorkWeDocSmartSheetViewProperty::default()
            .with_sort([WorkWeDocSmartSheetRecordSort::desc("field-score")])
            .with_groups([WorkWeDocSmartSheetRecordSort::asc("field-status")])
            .with_filter(WorkWeDocSmartSheetRecordFilter::and([
                WorkWeDocSmartSheetRecordFilterCondition::string(
                    "field-status",
                    WorkWeDocSmartSheetFilterOperatorKind::Is,
                    ["Ready"],
                ),
            ]))
            .set_field_visible("field-private", false);
        let update = WorkWeDocSmartSheetUpdateViewRequest::new("doc", "sheet", "view-1")
            .with_title("Ready orders")
            .with_property(property);
        update.validate().unwrap();
        let value = serde_json::to_value(update).unwrap();
        assert_eq!(
            value["property"]["sort_spec"]["sort_infos"][0]["desc"],
            true
        );
        assert_eq!(
            value["property"]["group_spec"]["sort_infos"][0]["field_id"],
            "field-status"
        );
        assert_eq!(
            value["property"]["field_visibility"]["field-private"],
            false
        );

        let empty_update = WorkWeDocSmartSheetUpdateViewRequest::new("doc", "sheet", "view-1");
        assert!(empty_update.validate().is_err());

        let invalid_calendar = WorkWeDocSmartSheetAddViewRequest {
            docid: "doc".to_string(),
            sheet_id: "sheet".to_string(),
            view_title: "Calendar".to_string(),
            view_type: "VIEW_TYPE_CALENDAR".to_string(),
            property_gantt: None,
            property_calendar: None,
        };
        assert!(invalid_calendar.validate().is_err());

        let invalid_query = WorkWeDocSmartSheetGetViewsRequest {
            docid: "doc".to_string(),
            sheet_id: "sheet".to_string(),
            view_ids: Vec::new(),
            offset: Some(-1),
            limit: Some(1_001),
        };
        assert!(invalid_query.validate().is_err());
    }

    #[test]
    fn serializes_and_validates_work_wedoc_smartsheet_record_mutations() {
        let add = WorkWeDocSmartSheetRecordsMutationRequest::by_field_id(
            "doc",
            "sheet",
            [WorkWeDocSmartSheetRecordMutation::add()
                .text("field-title", ["Ship order"])
                .number("field-score", 98.5)
                .checkbox("field-done", true)
                .users("field-owner", ["zhangsan"])
                .url("field-link", "Order", "https://example.com/orders/1")
                .options(
                    "field-status",
                    [WorkWeDocSmartSheetSelectOption::by_text("Ready")],
                )],
        );
        add.validate_for_add().unwrap();
        assert_eq!(
            add.key_type_kind(),
            Some(WorkWeDocSmartSheetCellKeyTypeKind::FieldId)
        );
        let value = serde_json::to_value(add).unwrap();
        assert_eq!(
            value["records"][0]["values"]["field-title"][0]["type"],
            "text"
        );
        assert_eq!(
            value["records"][0]["values"]["field-owner"][0]["user_id"],
            "zhangsan"
        );

        let update = WorkWeDocSmartSheetRecordsMutationRequest::by_field_title(
            "doc",
            "sheet",
            [WorkWeDocSmartSheetRecordMutation::update("record-1")
                .date_time_millis("Due", 1_720_000_000_000)],
        );
        update.validate_for_update().unwrap();
        let value = serde_json::to_value(update).unwrap();
        assert_eq!(value["records"][0]["record_id"], "record-1");
        assert_eq!(value["records"][0]["values"]["Due"], "1720000000000");

        let missing_record_id = WorkWeDocSmartSheetRecordsMutationRequest::by_field_id(
            "doc",
            "sheet",
            [WorkWeDocSmartSheetRecordMutation::add().number("field-score", 1.0)],
        );
        assert!(missing_record_id.validate_for_update().is_err());

        let empty_values = WorkWeDocSmartSheetRecordsMutationRequest::by_field_id(
            "doc",
            "sheet",
            [WorkWeDocSmartSheetRecordMutation::add()],
        );
        assert!(empty_values.validate_for_add().is_err());
    }

    #[test]
    fn validates_work_wedoc_smartsheet_record_queries() {
        let filter = WorkWeDocSmartSheetRecordFilter::and([
            WorkWeDocSmartSheetRecordFilterCondition::string(
                "field-status",
                WorkWeDocSmartSheetFilterOperatorKind::Is,
                ["Ready"],
            ),
            WorkWeDocSmartSheetRecordFilterCondition::boolean(
                "field-done",
                WorkWeDocSmartSheetFilterOperatorKind::Is,
                true,
            ),
        ]);
        let query = WorkWeDocSmartSheetGetRecordsRequest {
            docid: "doc".to_string(),
            sheet_id: "sheet".to_string(),
            view_id: None,
            record_ids: Vec::new(),
            key_type: Some("CELL_VALUE_KEY_TYPE_FIELD_ID".to_string()),
            field_titles: Vec::new(),
            field_ids: vec!["field-status".to_string()],
            sort: Vec::new(),
            offset: Some(0),
            limit: Some(1_000),
            ver: None,
            filter_spec: Some(filter),
        };
        query.validate().unwrap();
        let value = serde_json::to_value(query).unwrap();
        assert_eq!(value["filter_spec"]["conjunction"], "CONJUNCTION_AND");
        assert_eq!(
            value["filter_spec"]["conditions"][0]["string_value"]["value"][0],
            "Ready"
        );

        let invalid_query = WorkWeDocSmartSheetGetRecordsRequest {
            docid: "doc".to_string(),
            sheet_id: "sheet".to_string(),
            view_id: None,
            record_ids: Vec::new(),
            key_type: None,
            field_titles: Vec::new(),
            field_ids: Vec::new(),
            sort: vec![WorkWeDocSmartSheetRecordSort::asc("field-score")],
            offset: None,
            limit: Some(1_001),
            ver: None,
            filter_spec: Some(WorkWeDocSmartSheetRecordFilter::or([
                WorkWeDocSmartSheetRecordFilterCondition::number(
                    "field-score",
                    WorkWeDocSmartSheetFilterOperatorKind::IsGreater,
                    [60.0],
                ),
            ])),
        };
        assert!(invalid_query.validate().is_err());
    }

    #[test]
    fn validates_work_wedoc_smartsheet_field_mutations() {
        let update = WorkWeDocSmartSheetFieldsMutationRequest::new(
            "doc",
            "sheet",
            [WorkWeDocSmartSheetFieldMutation::update("field-status")
                .with_title("Delivery status")
                .with_type(WorkWeDocSmartSheetFieldTypeKind::SingleSelect)
                .with_property(WorkWeDocSmartSheetFieldProperty::select(
                    false,
                    [
                        WorkWeDocSmartSheetSelectOption::by_id("option-existing"),
                        WorkWeDocSmartSheetSelectOption::by_text("Delivered"),
                    ],
                ))],
        );
        update.validate_for_update().unwrap();
        let value = serde_json::to_value(update).unwrap();
        assert_eq!(value["fields"][0]["field_id"], "field-status");
        assert_eq!(
            value["fields"][0]["property"]["options"][1]["text"],
            "Delivered"
        );

        let missing_id = WorkWeDocSmartSheetFieldsMutationRequest::new(
            "doc",
            "sheet",
            [WorkWeDocSmartSheetFieldMutation::default().with_title("Status")],
        );
        assert!(missing_id.validate_for_update().is_err());

        let empty_add = WorkWeDocSmartSheetFieldsMutationRequest::new("doc", "sheet", Vec::new());
        assert!(empty_add.validate_for_add().is_err());

        let invalid_option = WorkWeDocSmartSheetFieldsMutationRequest::new(
            "doc",
            "sheet",
            [WorkWeDocSmartSheetFieldMutation::add(
                "Status",
                WorkWeDocSmartSheetFieldTypeKind::Select,
            )
            .with_property(WorkWeDocSmartSheetFieldProperty::select(
                false,
                [WorkWeDocSmartSheetSelectOption::default()],
            ))],
        );
        assert!(invalid_option.validate_for_add().is_err());

        let field: WorkWeDocSmartSheetField = serde_json::from_value(json!({
            "field_id": "field-future",
            "field_title": "Future",
            "field_type": "FIELD_TYPE_FUTURE",
            "property": {
                "future_setting": "preserved"
            }
        }))
        .unwrap();
        assert_eq!(
            field.field_type_kind(),
            Some(WorkWeDocSmartSheetFieldTypeKind::Other)
        );
        assert_eq!(field.property.unwrap().extra["future_setting"], "preserved");
    }

    #[test]
    fn deserializes_work_wedoc_smartsheet_responses() {
        let sheets: WorkWeDocSmartSheetGetResponse = serde_json::from_value(json!({
            "errcode": 0,
            "sheet_list": [{
                "sheet_id": "sheet",
                "title": "Tasks",
                "index": 2,
                "type": "smartsheet",
                "is_visible": true,
                "frozen_row_count": 1
            }],
            "request_id": "sheet-request"
        }))
        .unwrap();
        assert_eq!(sheets.sheet_list[0].sheet_id.as_deref(), Some("sheet"));
        assert_eq!(
            sheets.sheet_list[0].sheet_type_kind(),
            Some(WorkWeDocSmartSheetTypeKind::SmartSheet)
        );
        assert_eq!(sheets.sheet_list[0].is_visible, Some(true));
        assert_eq!(sheets.sheet_list[0].extra["frozen_row_count"], 1);
        assert_eq!(sheets.extra["request_id"], "sheet-request");

        let views: WorkWeDocSmartSheetGetViewsResponse = serde_json::from_value(json!({
            "errcode": 0,
            "total": 1,
            "has_more": false,
            "next": 1,
            "views": [{
                "view_id": "view",
                "view_title": "Calendar",
                "view_type": "VIEW_TYPE_CALENDAR",
                "property": {"groups": []},
                "display_density": "compact"
            }]
        }))
        .unwrap();
        assert_eq!(views.total, Some(1));
        assert_eq!(views.views[0].view_id.as_deref(), Some("view"));
        assert_eq!(
            views.views[0].view_type_kind(),
            Some(WorkWeDocSmartSheetViewTypeKind::Calendar)
        );
        assert_eq!(
            views.views[0].property.as_ref().unwrap().extra["groups"],
            json!([])
        );
        assert_eq!(views.views[0].extra["display_density"], "compact");

        let fields: WorkWeDocSmartSheetGetFieldsResponse = serde_json::from_value(json!({
            "errcode": 0,
            "total": 1,
            "fields": [{
                "field_id": "field-owner",
                "field_title": "Owner",
                "field_type": "FIELD_TYPE_USER",
                "property": {"is_multiple": true},
                "is_primary": false
            }]
        }))
        .unwrap();
        assert_eq!(fields.fields[0].field_title.as_deref(), Some("Owner"));
        assert_eq!(
            fields.fields[0].property.as_ref().unwrap().is_multiple,
            Some(true)
        );
        assert_eq!(fields.fields[0].extra["is_primary"], false);

        let records: WorkWeDocSmartSheetGetRecordsResponse = serde_json::from_value(json!({
            "errcode": 0,
            "total": 1,
            "has_more": true,
            "next": 100,
            "ver": 8,
            "records": [{
                "record_id": "record",
                "values": {
                    "field-owner": [{"userid": "user"}],
                    "field-score": 98.5
                },
                "create_time": 1710000000,
                "update_time": 1710000100,
                "creator_userid": "creator"
            }],
            "trace_id": "record-request"
        }))
        .unwrap();
        assert_eq!(records.next, Some(100));
        assert_eq!(records.ver, Some(8));
        assert_eq!(
            records.records[0].values.as_ref().unwrap()["field-score"],
            98.5
        );
        assert_eq!(records.records[0].extra["creator_userid"], "creator");
        assert_eq!(records.extra["trace_id"], "record-request");
    }

    #[test]
    fn serializes_work_wedoc_smartsheet_field_group_and_privilege_requests() {
        let add = serde_json::to_value(WorkWeDocSmartSheetAddFieldGroupRequest {
            docid: "doc".to_string(),
            sheet_id: "sheet".to_string(),
            name: "Ownership".to_string(),
            children: vec![WorkWeDocSmartSheetFieldGroupChild {
                field_id: "field-owner".to_string(),
                extra: Value::Null,
            }],
        })
        .unwrap();
        assert_eq!(add["children"][0]["field_id"], "field-owner");

        let update = serde_json::to_value(WorkWeDocSmartSheetUpdateFieldGroupRequest {
            docid: "doc".to_string(),
            sheet_id: "sheet".to_string(),
            field_group_id: "group".to_string(),
            name: None,
            children: vec![WorkWeDocSmartSheetFieldGroupChild {
                field_id: "field-status".to_string(),
                extra: Value::Null,
            }],
        })
        .unwrap();
        assert!(update.get("name").is_none());
        assert_eq!(update["field_group_id"], "group");

        let list = serde_json::to_value(WorkWeDocSmartSheetGetFieldGroupsRequest {
            docid: "doc".to_string(),
            sheet_id: "sheet".to_string(),
            offset: Some(20),
            limit: Some(10),
        })
        .unwrap();
        assert_eq!(list["offset"], 20);
        assert_eq!(list["limit"], 10);

        let privileges = serde_json::to_value(WorkWeDocSmartSheetGetPrivilegesRequest {
            docid: "doc".to_string(),
            rule_type: 2,
            rule_id_list: vec!["rule-1".to_string()],
        })
        .unwrap();
        assert_eq!(privileges["type"], 2);
        assert!(privileges.get("rule_type").is_none());
        assert_eq!(privileges["rule_id_list"][0], "rule-1");
    }

    #[test]
    fn deserializes_work_wedoc_smartsheet_field_groups_and_privileges() {
        let groups: WorkWeDocSmartSheetGetFieldGroupsResponse = serde_json::from_value(json!({
            "errcode": 0,
            "total": 1,
            "has_more": false,
            "next": 0,
            "field_groups": [{
                "field_group_id": "group",
                "name": "Ownership",
                "children": [{
                    "field_id": "field-owner",
                    "position": 1
                }],
                "collapsed": false
            }],
            "request_id": "field-groups"
        }))
        .unwrap();
        assert_eq!(groups.total, Some(1));
        assert_eq!(
            groups.field_groups[0].field_group_id.as_deref(),
            Some("group")
        );
        assert_eq!(groups.field_groups[0].children[0].extra["position"], 1);
        assert_eq!(groups.field_groups[0].extra["collapsed"], false);
        assert_eq!(groups.extra["request_id"], "field-groups");

        let privileges: WorkWeDocSmartSheetGetPrivilegesResponse = serde_json::from_value(json!({
            "errcode": 0,
            "rule_list": [{
                "rule_id": 1,
                "type": 1,
                "name": "Default",
                "priv_list": [{
                    "sheet_id": "sheet",
                    "priv": 2,
                    "can_insert_record": true,
                    "can_delete_record": false,
                    "record_priv": {
                        "record_range_type": 1,
                        "owner_field_id": "field-owner"
                    },
                    "field_priv": {
                        "field_range_type": 2,
                        "field_rule_list": [{
                            "field_id": "field-owner",
                            "field_type": "FIELD_TYPE_USER",
                            "can_edit": false,
                            "can_insert": true,
                            "can_view": true,
                            "mask_mode": "none"
                        }],
                        "field_default_rule": {
                            "can_edit": false,
                            "can_insert": false,
                            "can_view": true
                        }
                    },
                    "can_create_modify_delete_view": true,
                    "audit_required": true
                }]
            }],
            "trace_id": "privileges"
        }))
        .unwrap();
        let rule = &privileges.rule_list[0];
        assert_eq!(rule.rule_id.as_ref().unwrap(), &json!(1));
        assert_eq!(rule.rule_type, Some(1));
        let sheet = &rule.priv_list[0];
        assert_eq!(sheet.priv_level, Some(2));
        assert_eq!(
            sheet.record_priv.as_ref().unwrap().extra["owner_field_id"],
            "field-owner"
        );
        let field_priv = sheet.field_priv.as_ref().unwrap();
        assert_eq!(field_priv.field_range_type, Some(2));
        assert_eq!(
            field_priv.field_rule_list[0].field_type.as_deref(),
            Some("FIELD_TYPE_USER")
        );
        assert_eq!(field_priv.field_rule_list[0].extra["mask_mode"], "none");
        assert_eq!(sheet.extra["audit_required"], true);
        assert_eq!(privileges.extra["trace_id"], "privileges");
    }
}
