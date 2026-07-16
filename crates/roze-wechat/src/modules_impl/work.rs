use serde::{Deserialize, Serialize};
use serde_json::{json, to_value, Value};

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

    pub async fn add_external_contact_message_template(
        &self,
        access_token: impl Into<String>,
        request: ExternalContactMessageTemplateRequest,
    ) -> Result<ExternalContactMessageTemplateResponse> {
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
            json!({ "content": content.into() }),
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
            json!({ "articles": articles }),
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
            json!({ "articles": articles }),
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
            json!({ "media_id": media_id.into() }),
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
        info: Vec<Value>,
    ) -> Result<WorkMsgAuditAgreeResponse> {
        self.inner
            .post(
                "cgi-bin/msgaudit/check_single_agree",
                Some(access_token.into()),
                json!({ "info": info }),
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
    ) -> Result<WorkCheckinDataResponse> {
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
    ) -> Result<WorkCheckinDataResponse> {
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
                json!({ "userID": user_id.into(), "userface": user_face.into() }),
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

    pub fn oa_wedoc(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "work.oa.wedoc")
    }

    pub async fn create_wedoc_form(
        &self,
        access_token: impl Into<String>,
        request: WorkWeDocCreateFormRequest,
    ) -> Result<WorkWeDocCreateFormResponse> {
        self.inner
            .post(
                "cgi-bin/wedoc/create_form",
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
            .post(
                "cgi-bin/living/get_living_info",
                Some(access_token.into()),
                json!({ "livingid": living_id.into() }),
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
        self.inner
            .post(
                "cgi-bin/kf/servicer/add",
                Some(access_token.into()),
                json!({ "open_kfid": open_kfid.into(), "userid_list": userid_list }),
            )
            .await
    }

    pub async fn account_service_servicer_delete(
        &self,
        access_token: impl Into<String>,
        open_kfid: impl Into<String>,
        userid_list: Vec<String>,
    ) -> Result<WorkAccountServiceServicerResultResponse> {
        self.inner
            .post(
                "cgi-bin/kf/servicer/del",
                Some(access_token.into()),
                json!({ "open_kfid": open_kfid.into(), "userid_list": userid_list }),
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkDepartmentSimpleListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub department_id: Vec<WorkDepartmentSimple>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkDepartmentDetailResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(flatten)]
    pub department: WorkDepartment,
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
pub struct WorkAccessTokenResponse {
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
pub struct WorkCorpGroupAppShareInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub corp_list: Vec<WorkCorpGroupAppShareCorp>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCorpGroupAppShareCorp {
    #[serde(default)]
    pub corpid: Option<String>,
    #[serde(default)]
    pub agentid: Option<i64>,
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
    pub user_info: Option<WorkInvoiceUserInfo>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkInvoiceInfoBatchResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub item_list: Vec<WorkInvoiceInfoBatchItem>,
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
    pub allow_use_scope: Vec<WorkExternalPayUseScope>,
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
    pub button: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_card: Option<Value>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
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
    pub text: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub textcard: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub news: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpnews: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miniprogram_notice: Option<Value>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl WorkLinkedCorpMessage {
    pub fn text(agent_id: i64, content: impl Into<String>) -> Self {
        let mut message = Self::empty(agent_id, "text");
        message.text = Some(json!({ "content": content.into() }));
        message
    }

    pub fn image(agent_id: i64, media_id: impl Into<String>) -> Self {
        let mut message = Self::empty(agent_id, "image");
        message.image = Some(json!({ "media_id": media_id.into() }));
        message
    }

    pub fn file(agent_id: i64, media_id: impl Into<String>) -> Self {
        let mut message = Self::empty(agent_id, "file");
        message.file = Some(json!({ "media_id": media_id.into() }));
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

    fn empty(agent_id: i64, msg_type: impl Into<String>) -> Self {
        Self {
            touser: Vec::new(),
            toparty: Vec::new(),
            totag: Vec::new(),
            msgtype: msg_type.into(),
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
    pub text: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miniprogram_notice: Option<Value>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

impl WorkExternalContactSchoolMessage {
    pub fn text(agent_id: i64, content: impl Into<String>) -> Self {
        let mut message = Self::empty(agent_id, "text");
        message.text = Some(json!({ "content": content.into() }));
        message
    }

    pub fn image(agent_id: i64, media_id: impl Into<String>) -> Self {
        let mut message = Self::empty(agent_id, "image");
        message.image = Some(json!({ "media_id": media_id.into() }));
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

    fn empty(agent_id: i64, msg_type: impl Into<String>) -> Self {
        Self {
            recv_scope: None,
            to_external_userid: Vec::new(),
            to_parent_userid: Vec::new(),
            to_student_userid: Vec::new(),
            to_party: Vec::new(),
            msgtype: msg_type.into(),
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
pub struct WorkMpNewsArticle {
    pub title: String,
    pub thumb_media_id: String,
    pub author: String,
    pub content_source_url: String,
    pub content: String,
    pub digest: String,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUserDetailResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(flatten)]
    pub user: WorkUserDetail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkDepartmentUserSimpleListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub userlist: Vec<WorkDepartmentSimpleUser>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkDepartmentUserDetailListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub userlist: Vec<WorkUserDetail>,
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
    pub extattr: Option<Value>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUserIdLookupResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub userid: Option<String>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkJoinQrCodeResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub join_qrcode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUserActiveStatResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub active_cnt: Option<String>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkLinkedCorpUserResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub user_info: Option<WorkLinkedCorpUserInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkLinkedCorpUserListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub userlist: Vec<WorkLinkedCorpUserInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkLinkedCorpDepartmentListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub department_list: Vec<WorkLinkedCorpDepartment>,
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
    pub extattr: Option<Value>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUserBatchJobRequest {
    pub media_id: String,
    pub to_invite: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub callbacks: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUserBatchJobResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub jobid: Option<String>,
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
pub struct ExternalContactListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub external_userid: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactFollowUserListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub follow_user: Vec<String>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactBatchItem {
    #[serde(default)]
    pub external_contact: Option<ExternalContactInfo>,
    #[serde(default)]
    pub follow_info: Option<ExternalContactFollowInfo>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactProfile {
    #[serde(default)]
    pub external_corp_name: Option<String>,
    #[serde(default)]
    pub external_attr: Vec<ExternalContactAttribute>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactAttributeText {
    #[serde(default)]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactAttributeWeb {
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactAttributeMiniProgram {
    #[serde(default)]
    pub appid: Option<String>,
    #[serde(default)]
    pub pagepath: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactWayGetResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub contact_way: Option<ContactWayDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactWayId {
    #[serde(default)]
    pub config_id: Option<String>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorpTagListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub tag_group: Vec<CorpTagGroup>,
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
    pub owner_filter: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    pub limit: i64,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalGroupChatSummary {
    #[serde(default)]
    pub chat_id: Option<String>,
    #[serde(default)]
    pub status: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalGroupChatGetResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub group_chat: Option<ExternalGroupChatDetail>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalGroupChatInvitor {
    #[serde(default)]
    pub userid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalGroupChatAdmin {
    #[serde(default)]
    pub userid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalGroupChatTransferResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub failed_chat_list: Vec<ExternalGroupChatFailedChat>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalGroupChatFailedChat {
    #[serde(default)]
    pub chat_id: Option<String>,
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalGroupChatOpenGidToChatIdResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub chat_id: Option<String>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalGroupChatJoinWayResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub join_way: Option<Value>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMomentStrategyCreateResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub strategy_id: Option<i64>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactStrategyTagAddResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub tag_group: Option<CorpTagGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalGroupWelcomeTemplateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub miniprogram: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video: Option<Value>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalGroupWelcomeTemplateResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub text: Option<Value>,
    #[serde(default)]
    pub image: Option<Value>,
    #[serde(default)]
    pub link: Option<Value>,
    #[serde(default)]
    pub miniprogram: Option<Value>,
    #[serde(default)]
    pub file: Option<Value>,
    #[serde(default)]
    pub video: Option<Value>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAcquisitionRange {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub user_list: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub department_list: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAcquisitionPriorityOption {
    pub priority_type: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub priority_userid_list: Vec<String>,
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
    pub link: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAcquisitionLinkResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub link: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMessageTemplateRequest {
    pub chat_type: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub external_userid: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub chat_id_list: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_filter: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender: Option<String>,
    pub allow_select: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<ExternalContactMessageText>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attachments: Vec<ExternalContactMessageAttachment>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMessageText {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

impl ExternalContactMessageText {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: Some(content.into()),
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
}

impl ExternalContactMessageAttachment {
    pub fn image(media_id: impl Into<String>) -> Self {
        Self {
            msgtype: Some("image".to_string()),
            image: Some(ExternalContactMessageImage {
                media_id: Some(media_id.into()),
                pic_url: None,
            }),
            link: None,
            miniprogram: None,
            video: None,
            file: None,
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
            }),
            file: None,
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
            }),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMessageImage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pic_url: Option<String>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMessageVideo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMessageFile {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_id: Option<String>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactGroupMessageTask {
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default)]
    pub send_time: Option<i64>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactUnassignedInfo {
    #[serde(default)]
    pub handover_userid: Option<String>,
    #[serde(default)]
    pub external_userid: Option<String>,
    #[serde(default)]
    pub dimission_time: Option<i64>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMomentTask {
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub publish_status: Option<i64>,
    #[serde(default)]
    pub status: Option<i64>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMomentCustomer {
    #[serde(default)]
    pub external_userid: Option<String>,
    #[serde(default)]
    pub publish_status: Option<i64>,
    #[serde(default)]
    pub status: Option<i64>,
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
}

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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMomentLike {
    #[serde(default)]
    pub external_userid: Option<String>,
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub create_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMomentTaskRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<ExternalContactMessageText>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attachments: Vec<ExternalContactMessageAttachment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible_range: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactMomentTaskCreateResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub jobid: Option<String>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalGroupChatStatisticRequest {
    pub day_begin_time: i64,
    pub day_end_time: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_filter: Option<Value>,
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
    pub owner_filter: Option<Value>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalGroupChatStatisticItem {
    #[serde(default)]
    pub owner: Option<String>,
    #[serde(default)]
    pub data: Option<ExternalGroupChatStatisticData>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactCustomerStrategyResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default, alias = "momentStrategy")]
    pub strategy: Option<ExternalContactCustomerStrategy>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContactCustomerStrategyCreateResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub strategy_id: Option<i64>,
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
pub struct WorkMsgAuditPermitUsersResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMsgAuditChatDataResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub chatdata: Vec<WorkMsgAuditChatData>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMsgAuditRoomMember {
    #[serde(default)]
    pub memberid: Option<String>,
    #[serde(default)]
    pub jointime: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMsgAuditAgreeResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub agreeinfo: Vec<WorkMsgAuditAgreeInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMsgAuditAgreeInfo {
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub exteranalopenid: Option<String>,
    #[serde(default)]
    pub agree_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMsgAuditRobotInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub robot_info: Option<WorkMsgAuditRobotInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMsgAuditRobotInfo {
    #[serde(default)]
    pub robot_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub creator_userid: Option<String>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceAccountListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub account_list: Vec<WorkAccountServiceAccount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceAccount {
    #[serde(default)]
    pub open_kfid: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub avatar: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceAddContactWayResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceEnterSessionContext {
    #[serde(default)]
    pub scene: Option<String>,
    #[serde(default)]
    pub scene_param: Option<String>,
    #[serde(default)]
    pub wechat_channels: Option<Value>,
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
    pub event: Option<WorkAccountServiceEventMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceTextMessage {
    #[serde(default)]
    pub content: Option<String>,
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
    pub text: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub voice: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub miniprogram: Option<Value>,
    #[serde(default, rename = "msgmenu", skip_serializing_if = "Option::is_none")]
    pub menu: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ca_link: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceSendMsgOnEventRequest {
    pub code: String,
    pub msgid: String,
    pub msgtype: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<Value>,
    #[serde(default, rename = "msgmenu", skip_serializing_if = "Option::is_none")]
    pub menu: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceSendMsgResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub msgid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceServicerResultResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub result_list: Vec<WorkAccountServiceServicerResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceServicerResult {
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceServicerListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub servicer_list: Vec<WorkAccountServiceServicer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceServicer {
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub status: Option<i64>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceStateTransRequest {
    pub open_kfid: String,
    pub external_userid: String,
    pub service_state: i64,
    pub servicer_userid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceTagCreateResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub tagid: Option<i64>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceTagUser {
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceTagListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub taglist: Vec<WorkAccountServiceTag>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAccountServiceTag {
    #[serde(default)]
    pub tagid: Option<i64>,
    #[serde(default)]
    pub tagname: Option<String>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAppChatGetResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub chat_info: Option<WorkAppChatInfo>,
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
    pub items: Vec<Value>,
    pub yearmonth: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinOptionMutationRequest {
    pub group: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinCorpOptionResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub group: Vec<WorkCheckinGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinGroup {
    #[serde(default)]
    pub groupid: Option<i64>,
    #[serde(default)]
    pub groupname: Option<String>,
    #[serde(default)]
    pub checkindate: Vec<WorkCheckinDateRule>,
    #[serde(default)]
    pub spe_workdays: Vec<Value>,
    #[serde(default)]
    pub spe_offdays: Vec<Value>,
    #[serde(default)]
    pub wifimac_infos: Vec<Value>,
    #[serde(default)]
    pub loc_infos: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinDateRule {
    #[serde(default)]
    pub workdays: Vec<i64>,
    #[serde(default)]
    pub checkintime: Vec<Value>,
    #[serde(default)]
    pub flex_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinOptionResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub info: Vec<WorkCheckinUserOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinUserOption {
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub groupid: Option<i64>,
    #[serde(default)]
    pub groupname: Option<String>,
    #[serde(default)]
    pub checkin_date: Option<i64>,
    #[serde(default)]
    pub day_type: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinRecordResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub checkindata: Vec<WorkCheckinRecord>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinDataResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub datas: Vec<WorkCheckinDataItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinDataItem {
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub groupid: Option<i64>,
    #[serde(default)]
    pub date: Option<i64>,
    #[serde(default)]
    pub base_info: Option<Value>,
    #[serde(default)]
    pub summary_info: Option<Value>,
    #[serde(default)]
    pub exception_infos: Vec<Value>,
    #[serde(default)]
    pub holiday_infos: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinScheduleListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub schedule_list: Vec<WorkCheckinSchedule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCheckinSchedule {
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub schedule_id: Option<i64>,
    #[serde(default)]
    pub groupid: Option<i64>,
    #[serde(default)]
    pub day: Option<i64>,
    #[serde(default)]
    pub schedule: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalTemplateDetailResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub template_names: Vec<Value>,
    #[serde(default)]
    pub template_content: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalApplyEventResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub sp_no: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalApplyEventRequest {
    pub creator_userid: String,
    pub template_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_template_approver: Option<i64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub approver: Vec<Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notifyer: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_type: Option<i64>,
    pub apply_data: Value,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub summary_list: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalInfoRequest {
    pub starttime: i64,
    pub endtime: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_cursor: Option<String>,
    pub size: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filters: Vec<Value>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkApprovalDetailResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub info: Option<Value>,
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
    pub data: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkVacationConfigResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub lists: Vec<Value>,
}

pub type WorkVacationQuotaResponse = WorkVacationConfigResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkVacationQuotaUpdateRequest {
    pub userid: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lists: Vec<WorkVacationQuotaUpdateItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkVacationQuotaUpdateItem {
    pub vacation_id: i64,
    pub leftduration: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_attr: Option<i64>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
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
pub struct WorkCalendarShare {
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub readonly: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCalendarInfo {
    #[serde(default)]
    pub cal_id: Option<String>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCalendarGetResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub calendar_list: Vec<WorkCalendarInfo>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkDialRecordResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub record: Vec<WorkDialRecord>,
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
pub struct WorkJournalRecordInfo {
    #[serde(default)]
    pub journaluuid: Option<String>,
    #[serde(default)]
    pub template_id: Option<String>,
    #[serde(default)]
    pub creator: Option<String>,
    #[serde(default)]
    pub apply_time: Option<i64>,
    #[serde(default)]
    pub state: Option<i64>,
    #[serde(default)]
    pub apply_data: Option<Value>,
    #[serde(default)]
    pub comments: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkJournalRecordDetailResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub info: Option<WorkJournalRecordInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkJournalStatListRequest {
    pub template_id: String,
    pub starttime: String,
    pub endtime: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkJournalStatSummary {
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkJournalStatList {
    #[serde(default)]
    pub summary: Vec<WorkJournalStatSummary>,
    #[serde(default)]
    pub total: Option<i64>,
    #[serde(default)]
    pub details: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkJournalStatListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub stat_list: Option<WorkJournalStatList>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkPstnccCallResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub states: Vec<WorkPstnccCallState>,
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
    pub attendees: Value,
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
    pub attendees: Value,
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
    pub attendees: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMeetingRoomAddRequest {
    pub name: String,
    pub capacity: i64,
    pub city: String,
    pub building: String,
    pub floor: String,
    pub equipment: Vec<i64>,
    pub coordinate: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMeetingRoomEditRequest {
    pub meetingroom_id: i64,
    pub name: String,
    pub capacity: i64,
    pub city: String,
    pub building: String,
    pub floor: String,
    pub equipment: Vec<i64>,
    pub coordinate: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMeetingRoomListRequest {
    pub city: String,
    pub building: String,
    pub floor: String,
    pub equipment: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMeetingRoomGetBookingInfoRequest {
    pub meetingroom_id: i64,
    pub start_time: i64,
    pub end_time: i64,
    pub city: String,
    pub building: String,
    pub floor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMeetingRoomBookRequest {
    pub meetingroom_id: i64,
    pub subject: String,
    pub start_time: i64,
    pub end_time: i64,
    pub booker: String,
    pub attendees: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMeetingRoomCancelBookRequest {
    pub meeting_id: String,
    pub keep_schedule: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMeetingRoomAddResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub meetingroom_id: Option<i64>,
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
    pub coordinate: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMeetingRoomListResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub meetingroom_list: Vec<WorkMeetingRoomInfo>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMeetingRoomGetBookingInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub booking_list: Vec<WorkMeetingRoomBooking>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocCreateFormRequest {
    pub spaceid: String,
    pub fatherid: String,
    pub form_info: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDocCreateFormResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub formid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkLivingCreateRequest {
    pub anchor_userid: String,
    pub theme: String,
    pub living_start: i64,
    pub living_duration: i64,
    pub description: String,
    #[serde(rename = "type")]
    pub living_type: i64,
    pub agentid: i64,
    pub remind_time: i64,
    pub activity_cover_mediaid: String,
    pub activity_share_mediaid: String,
    pub activity_detail: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkLivingModifyRequest {
    pub livingid: String,
    pub theme: String,
    pub living_start: i64,
    pub living_duration: i64,
    pub description: String,
    #[serde(rename = "type")]
    pub living_type: i64,
    pub remind_time: i64,
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
    pub livingid: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkLivingCodeResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub living_code: Option<i64>,
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
    pub viewer_count: Option<i64>,
    #[serde(default)]
    pub comment_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkLivingInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub living_info: Option<WorkLivingInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkLivingWatchStat {
    #[serde(default)]
    pub viewer_userid: Option<String>,
    #[serde(default)]
    pub viewer_external_userid: Option<String>,
    #[serde(default)]
    pub watch_time: Option<i64>,
    #[serde(default)]
    pub is_comment: Option<i64>,
    #[serde(default)]
    pub is_mic: Option<i64>,
    #[serde(default)]
    pub invite_userid: Option<String>,
    #[serde(default)]
    pub invite_external_userid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkLivingWatchStatInfo {
    #[serde(default)]
    pub viewer_count: Option<i64>,
    #[serde(default)]
    pub comment_count: Option<i64>,
    #[serde(default)]
    pub mic_count: Option<i64>,
    #[serde(default)]
    pub watch_stat: Vec<WorkLivingWatchStat>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkLivingWatchStatResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub next_key: Option<String>,
    #[serde(default)]
    pub stat_info: Option<WorkLivingWatchStatInfo>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDriveSpaceCreateRequest {
    pub userid: String,
    pub space_name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub auth_info: Vec<Value>,
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
    pub auth_info: Vec<Value>,
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
    pub auth_info: Vec<Value>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDriveSpaceInfo {
    #[serde(default)]
    pub spaceid: Option<String>,
    #[serde(default)]
    pub space_name: Option<String>,
    #[serde(default)]
    pub userid: Option<String>,
    #[serde(default)]
    pub quota: Option<i64>,
    #[serde(default)]
    pub used_size: Option<i64>,
    #[serde(default)]
    pub auth_info: Vec<Value>,
    #[serde(default)]
    pub add_member_only_admin: Option<bool>,
    #[serde(default)]
    pub enable_watermark: Option<bool>,
    #[serde(default)]
    pub enable_share_url: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDriveSpaceInfoResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub space_info: Option<WorkWeDriveSpaceInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDriveSpaceShareResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub space_share_url: Option<String>,
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
    pub file_list: Vec<WorkWeDriveFileInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDriveFileUploadResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub fileid: Option<String>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDriveFileInfo {
    #[serde(default)]
    pub fileid: Option<String>,
    #[serde(default)]
    pub file_name: Option<String>,
    #[serde(default)]
    pub file_type: Option<String>,
    #[serde(default)]
    pub file_size: Option<i64>,
    #[serde(default)]
    pub spaceid: Option<String>,
    #[serde(default)]
    pub fatherid: Option<String>,
    #[serde(default)]
    pub creator: Option<String>,
    #[serde(default)]
    pub create_time: Option<i64>,
    #[serde(default)]
    pub update_time: Option<i64>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub auth_info: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDriveFileRenameResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub file: Option<WorkWeDriveFileInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDriveFileMoveResult {
    #[serde(default)]
    pub success: Vec<String>,
    #[serde(default)]
    pub failed: Vec<WorkWeDriveFileMoveFailure>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDriveFileMoveFailure {
    #[serde(default)]
    pub fileid: Option<String>,
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDriveFileMoveResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub file_list: Option<WorkWeDriveFileMoveResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkWeDriveFileShareResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub share_url: Option<String>,
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
pub struct WorkScheduleInfo {
    #[serde(default)]
    pub schedule_id: Option<String>,
    #[serde(default)]
    pub organizer: Option<String>,
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub start_time: Option<i64>,
    #[serde(default)]
    pub end_time: Option<i64>,
    #[serde(default)]
    pub attendees: Option<Value>,
    #[serde(default)]
    pub location: Option<String>,
    #[serde(default)]
    pub status: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkScheduleGetResponse {
    #[serde(default)]
    pub errcode: Option<i64>,
    #[serde(default)]
    pub errmsg: Option<String>,
    #[serde(default)]
    pub schedule_list: Vec<WorkScheduleInfo>,
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
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

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

        let update = serde_json::to_value(WorkTemplateCardUpdateRequest {
            userids: vec!["user".to_string()],
            partyids: Vec::new(),
            tagids: Vec::new(),
            atall: None,
            agentid: 100001,
            response_code: "response".to_string(),
            button: Some(json!({ "replace_name": "done" })),
            template_card: Some(json!({ "card_type": "button_interaction" })),
            extra: serde_json::Value::Null,
        })
        .unwrap();
        assert_eq!(update["userids"][0], "user");
        assert_eq!(update["response_code"], "response");
        assert_eq!(update["button"]["replace_name"], "done");
        assert!(update.get("partyids").is_none());
    }

    #[test]
    fn serializes_linked_corp_and_school_message_responses() {
        let linked_body = serde_json::to_value(WorkLinkedCorpMessage {
            touser: vec!["Corp/user".to_string()],
            toparty: vec!["Corp/party".to_string()],
            totag: vec!["Corp/tag".to_string()],
            msgtype: "text".to_string(),
            agentid: 100001,
            text: Some(json!({ "content": "hello" })),
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

        let linked_file =
            serde_json::to_value(WorkLinkedCorpMessage::file(100001, "file-media")).unwrap();
        assert_eq!(linked_file["msgtype"], "file");
        assert_eq!(linked_file["file"]["media_id"], "file-media");
        assert!(linked_file.get("touser").is_none());

        let linked_response: WorkLinkedCorpMessageSendResponse = serde_json::from_value(json!({
            "errcode": 0,
            "invaliduser": ["Corp/bad-user"],
            "invalidparty": ["Corp/bad-party"],
            "invalidtag": ["Corp/bad-tag"]
        }))
        .unwrap();
        assert_eq!(linked_response.invaliduser[0], "Corp/bad-user");
        assert_eq!(linked_response.invalidparty[0], "Corp/bad-party");
        assert_eq!(linked_response.invalidtag[0], "Corp/bad-tag");

        let school_body = serde_json::to_value(WorkExternalContactSchoolMessage {
            recv_scope: Some(0),
            to_external_userid: Vec::new(),
            to_parent_userid: vec!["parent".to_string()],
            to_student_userid: vec!["student".to_string()],
            to_party: vec!["party".to_string()],
            msgtype: "text".to_string(),
            agentid: 100001,
            text: Some(json!({ "content": "notice" })),
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

        let school_image = serde_json::to_value(WorkExternalContactSchoolMessage::image(
            100001,
            "image-media",
        ))
        .unwrap();
        assert_eq!(school_image["msgtype"], "image");
        assert_eq!(school_image["image"]["media_id"], "image-media");
        assert!(school_image.get("to_parent_userid").is_none());

        let school_response: WorkExternalContactSchoolMessageSendResponse =
            serde_json::from_value(json!({
                "invalid_external_user": ["external"],
                "invalid_parent_userid": ["parent"],
                "invalid_student_userid": ["student"],
                "invalid_party": ["party"]
            }))
            .unwrap();
        assert_eq!(school_response.invalid_external_user[0], "external");
        assert_eq!(school_response.invalid_parent_userid[0], "parent");
        assert_eq!(school_response.invalid_student_userid[0], "student");
        assert_eq!(school_response.invalid_party[0], "party");
    }

    #[test]
    fn deserializes_external_contact_base_responses() {
        let list: ExternalContactListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "external_userid": ["wm-external"]
        }))
        .unwrap();
        assert_eq!(list.external_userid[0], "wm-external");

        let follow_users: ExternalContactFollowUserListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "follow_user": ["user-a", "user-b"]
        }))
        .unwrap();
        assert_eq!(follow_users.follow_user[1], "user-b");

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
                "external_profile": {
                    "external_corp_name": "Roze",
                    "external_attr": [{
                        "type": 0,
                        "name": "Website",
                        "web": {
                            "url": "https://example.com",
                            "title": "Home"
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
                "oper_userid": "operator"
            }],
            "next_cursor": "cursor"
        }))
        .unwrap();
        let contact = detail.external_contact.expect("external_contact");
        assert_eq!(contact.external_userid.as_deref(), Some("wm-external"));
        assert_eq!(contact.contact_type, Some(2));
        assert_eq!(
            contact
                .external_profile
                .expect("external_profile")
                .external_attr[0]
                .web
                .as_ref()
                .expect("web")
                .url
                .as_deref(),
            Some("https://example.com")
        );
        assert_eq!(
            detail.follow_user[0].tags[0].tag_name.as_deref(),
            Some("Gold")
        );
        assert_eq!(detail.next_cursor.as_deref(), Some("cursor"));

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
                }
            }],
            "next_cursor": "next"
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
        assert_eq!(batch.next_cursor.as_deref(), Some("next"));
    }

    #[test]
    fn serializes_group_robot_text_message() {
        let value = serde_json::to_value(Work::group_robot_text("hello", vec!["@all".to_string()]))
            .unwrap();

        assert_eq!(value["msgtype"], "text");
        assert_eq!(value["text"]["mentioned_list"][0], "@all");

        let markdown = serde_json::to_value(Work::group_robot_markdown("**hello**")).unwrap();
        assert_eq!(markdown["msgtype"], "markdown");
        assert_eq!(markdown["markdown"]["content"], "**hello**");

        let file = serde_json::to_value(GroupRobotMessage {
            msgtype: "file".to_string(),
            text: None,
            markdown: None,
            image: None,
            news: None,
            file: Some(json!({ "media_id": "media" })),
            template_card: None,
        })
        .unwrap();
        assert_eq!(file["file"]["media_id"], "media");
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
            "contact_way": [{ "config_id": "config" }],
            "next_cursor": "cursor"
        }))
        .unwrap();
        assert_eq!(response.contact_way[0].config_id.as_deref(), Some("config"));
        assert_eq!(response.next_cursor.as_deref(), Some("cursor"));

        let added: ContactWayAddResponse = serde_json::from_value(json!({
            "config_id": "config",
            "qr_code": "https://example.com/qr"
        }))
        .unwrap();
        assert_eq!(added.config_id.as_deref(), Some("config"));

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
                "conclusions": {
                    "link": {
                        "title": "title",
                        "picurl": "https://example.com/pic.png",
                        "desc": "desc",
                        "url": "https://example.com"
                    }
                }
            }
        }))
        .unwrap();
        let contact_way = detail.contact_way.unwrap();
        assert_eq!(contact_way.kind, Some(1));
        assert_eq!(contact_way.user[0], "user");
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
                "tag": [{ "id": "vip", "name": "vip", "order": 1 }]
            }]
        }))
        .unwrap();
        assert_eq!(corp_tags.tag_group[0].group_name.as_deref(), Some("level"));
        assert_eq!(corp_tags.tag_group[0].tag[0].id.as_deref(), Some("vip"));

        let corp_tag_created: CorpTagAddResponse = serde_json::from_value(json!({
            "tag_group": {
                "group_id": "level",
                "tag": [{ "id": "vip", "name": "vip" }]
            }
        }))
        .unwrap();
        assert_eq!(
            corp_tag_created.tag_group.unwrap().tag[0].name.as_deref(),
            Some("vip")
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
                "tag": [{ "id": "tag", "name": "gold", "order": 1 }]
            }]
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

        let strategy_created: ExternalContactStrategyTagAddResponse =
            serde_json::from_value(json!({
                "tag_group": { "group_id": "group", "tag": [{ "id": "tag" }] }
            }))
            .unwrap();
        let strategy_created = strategy_created.tag_group.unwrap();
        assert_eq!(strategy_created.group_id.as_deref(), Some("group"));
        assert_eq!(strategy_created.tag[0].id.as_deref(), Some("tag"));
    }

    #[test]
    fn serializes_external_group_chat_requests_and_responses() {
        let list = serde_json::to_value(ExternalGroupChatListRequest {
            status_filter: Some(0),
            owner_filter: Some(json!({ "userid_list": ["user"] })),
            cursor: None,
            limit: 50,
        })
        .unwrap();
        assert_eq!(list["status_filter"], 0);
        assert_eq!(list["owner_filter"]["userid_list"][0], "user");
        assert!(list.get("cursor").is_none());

        let chats: ExternalGroupChatListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "group_chat_list": [{ "chat_id": "chat", "status": 0 }],
            "next_cursor": "cursor"
        }))
        .unwrap();
        assert_eq!(chats.group_chat_list[0].chat_id.as_deref(), Some("chat"));
        assert_eq!(chats.group_chat_list[0].status, Some(0));
        assert_eq!(chats.next_cursor.as_deref(), Some("cursor"));

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
                    "invitor": { "userid": "invitor" },
                    "group_nickname": "nick",
                    "name": "name",
                    "unionid": "union"
                }],
                "admin_list": [{ "userid": "admin" }],
                "member_version": "v1"
            }
        }))
        .unwrap();
        let group_chat = detail.group_chat.unwrap();
        assert_eq!(group_chat.name.as_deref(), Some("group"));
        assert_eq!(group_chat.member_list[0].member_type, Some(1));
        assert_eq!(
            group_chat.member_list[0]
                .invitor
                .as_ref()
                .and_then(|invitor| invitor.userid.as_deref()),
            Some("invitor")
        );
        assert_eq!(group_chat.admin_list[0].userid.as_deref(), Some("admin"));

        let transfer: ExternalGroupChatTransferResponse = serde_json::from_value(json!({
            "errcode": 0,
            "failed_chat_list": [{ "chat_id": "bad", "errcode": 40003, "errmsg": "bad owner" }]
        }))
        .unwrap();
        assert_eq!(transfer.failed_chat_list[0].chat_id.as_deref(), Some("bad"));
        assert_eq!(transfer.failed_chat_list[0].errcode, Some(40003));

        let open_gid: ExternalGroupChatOpenGidToChatIdResponse =
            serde_json::from_value(json!({ "chat_id": "chat" })).unwrap();
        assert_eq!(open_gid.chat_id.as_deref(), Some("chat"));

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

        let join_add: ExternalGroupChatJoinWayAddResponse =
            serde_json::from_value(json!({ "config_id": "config" })).unwrap();
        assert_eq!(join_add.config_id.as_deref(), Some("config"));

        let join_detail: ExternalGroupChatJoinWayResponse = serde_json::from_value(json!({
            "join_way": { "config_id": "config", "qr_code": "https://example.com/qr" }
        }))
        .unwrap();
        assert_eq!(join_detail.join_way.unwrap()["config_id"], "config");

        let converted: WorkExternalUserIdConvertResponse =
            serde_json::from_value(json!({ "external_userid": "new-external" })).unwrap();
        assert_eq!(converted.external_userid.as_deref(), Some("new-external"));
    }

    #[test]
    fn serializes_external_group_welcome_templates() {
        let template = ExternalGroupWelcomeTemplateRequest {
            text: Some(json!({ "content": "welcome" })),
            image: None,
            link: Some(json!({ "title": "docs", "url": "https://example.com" })),
            miniprogram: None,
            file: None,
            video: None,
            agentid: 100001,
            notify: 1,
        };
        let value = serde_json::to_value(&template).unwrap();
        assert_eq!(value["text"]["content"], "welcome");
        assert_eq!(value["link"]["title"], "docs");
        assert!(value.get("image").is_none());

        let update = serde_json::to_value(ExternalGroupWelcomeTemplateUpdateRequest {
            template_id: "template".to_string(),
            template,
        })
        .unwrap();
        assert_eq!(update["template_id"], "template");
        assert_eq!(update["agentid"], 100001);

        let added: ExternalGroupWelcomeTemplateAddResponse =
            serde_json::from_value(json!({ "template_id": "template" })).unwrap();
        assert_eq!(added.template_id.as_deref(), Some("template"));

        let detail: ExternalGroupWelcomeTemplateResponse = serde_json::from_value(json!({
            "text": { "content": "welcome" },
            "image": { "media_id": "media" }
        }))
        .unwrap();
        assert_eq!(detail.text.unwrap()["content"], "welcome");
        assert_eq!(detail.image.unwrap()["media_id"], "media");
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
            },
            skip_verify: true,
            priority_option: Some(CustomerAcquisitionPriorityOption {
                priority_type: 1,
                priority_userid_list: vec!["priority".to_string()],
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
            "next_cursor": "cursor"
        }))
        .unwrap();
        assert_eq!(links.link_id_list[0], "link");
        assert_eq!(links.next_cursor.as_deref(), Some("cursor"));

        let created: CustomerAcquisitionLinkCreateResponse = serde_json::from_value(json!({
            "errcode": 0,
            "link": { "link_id": "link", "url": "https://example.com" }
        }))
        .unwrap();
        assert_eq!(created.link.unwrap()["link_id"], "link");
    }

    #[test]
    fn serializes_external_contact_message_template_requests() {
        let template = serde_json::to_value(ExternalContactMessageTemplateRequest {
            chat_type: "single".to_string(),
            external_userid: vec!["external".to_string()],
            chat_id_list: Vec::new(),
            tag_filter: Some(json!({ "group_list": [{ "tag_list": ["tag"] }] })),
            sender: Some("sender".to_string()),
            allow_select: true,
            text: Some(ExternalContactMessageText::new("hello")),
            attachments: vec![ExternalContactMessageAttachment::link(
                ExternalContactMessageLink {
                    title: Some("title".to_string()),
                    picurl: None,
                    desc: None,
                    url: Some("https://example.com".to_string()),
                },
            )],
        })
        .unwrap();
        assert_eq!(template["chat_type"], "single");
        assert_eq!(template["external_userid"][0], "external");
        assert_eq!(template["attachments"][0]["msgtype"], "link");
        assert_eq!(
            template["attachments"][0]["link"],
            json!({ "title": "title", "url": "https://example.com" })
        );
        assert!(template.get("chat_id_list").is_none());
        assert!(template["attachments"][0].get("image").is_none());

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
            "fail_list": ["bad"]
        }))
        .unwrap();
        assert_eq!(added.msgid.as_deref(), Some("msg"));
        assert_eq!(added.fail_list[0], "bad");

        let messages: ExternalContactGroupMessageListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "group_msg_list": [{
                "msgid": "msg",
                "creator": "creator",
                "create_time": 1_720_000_000,
                "create_type": 1,
                "text": { "content": "hello" },
                "attachments": [{
                    "msgtype": "link",
                    "link": {
                        "title": "title",
                        "picurl": "https://example.com/pic.png",
                        "desc": "desc",
                        "url": "https://example.com"
                    }
                }]
            }],
            "next_cursor": "cursor"
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
            messages.group_msg_list[0].attachments[0]
                .link
                .as_ref()
                .and_then(|link| link.url.as_deref()),
            Some("https://example.com")
        );
        assert_eq!(messages.next_cursor.as_deref(), Some("cursor"));

        let tasks: ExternalContactGroupMessageTaskResponse = serde_json::from_value(json!({
            "task_list": [{ "userid": "user", "status": 1, "send_time": 1_720_000_001 }],
            "next_cursor": "task-cursor"
        }))
        .unwrap();
        assert_eq!(tasks.task_list[0].userid.as_deref(), Some("user"));
        assert_eq!(tasks.task_list[0].send_time, Some(1_720_000_001));
        assert_eq!(tasks.next_cursor.as_deref(), Some("task-cursor"));

        let send_result: ExternalContactGroupMessageSendResultResponse =
            serde_json::from_value(json!({
                "send_list": [{
                    "external_userid": "external",
                    "chat_id": "chat",
                    "userid": "user",
                    "status": 1,
                    "send_time": 1_720_000_002
                }],
                "next_cursor": "send-cursor"
            }))
            .unwrap();
        assert_eq!(
            send_result.send_list[0].external_userid.as_deref(),
            Some("external")
        );
        assert_eq!(send_result.send_list[0].chat_id.as_deref(), Some("chat"));
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
                "takeover_userid": "new"
            }],
            "next_cursor": "cursor"
        }))
        .unwrap();
        assert_eq!(
            response.customer[0].external_userid.as_deref(),
            Some("external")
        );
        assert_eq!(response.customer[0].status, Some(1));
        assert_eq!(response.customer[0].takeover_time, Some(100));
        assert_eq!(response.next_cursor.as_deref(), Some("cursor"));

        let unassigned_response: ExternalContactUnassignedListResponse =
            serde_json::from_value(json!({
                "info": [{
                    "handover_userid": "old",
                    "external_userid": "external",
                    "dimission_time": 100
                }],
                "is_last": false,
                "next_cursor": "next"
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
        assert_eq!(unassigned_response.is_last, Some(false));
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
            visible_range: Some(json!({
                "sender_list": { "user_list": ["user"] },
                "external_contact_list": { "tag_list": ["tag"] }
            })),
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
                "attachments": [{ "msgtype": "image", "image": { "media_id": "media" } }]
            }],
            "next_cursor": "cursor"
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
        assert_eq!(moments.next_cursor.as_deref(), Some("cursor"));

        let tasks: ExternalContactMomentTaskResponse = serde_json::from_value(json!({
            "task_list": [{ "userid": "user", "publish_status": 2 }],
            "next_cursor": "next"
        }))
        .unwrap();
        assert_eq!(tasks.task_list[0].userid.as_deref(), Some("user"));
        assert_eq!(tasks.task_list[0].publish_status, Some(2));

        let customers: ExternalContactMomentCustomerListResponse = serde_json::from_value(json!({
            "customer_list": [{ "external_userid": "external", "publish_status": 1 }]
        }))
        .unwrap();
        assert_eq!(
            customers.customer_list[0].external_userid.as_deref(),
            Some("external")
        );
        assert_eq!(customers.customer_list[0].publish_status, Some(1));

        let comments: ExternalContactMomentCommentResponse = serde_json::from_value(json!({
            "comment_list": [{ "userid": "user", "comment_id": "comment", "content": "nice" }],
            "like_list": [{ "external_userid": "external", "create_time": 100 }]
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

        let created: ExternalContactMomentTaskCreateResponse =
            serde_json::from_value(json!({ "jobid": "job" })).unwrap();
        assert_eq!(created.jobid.as_deref(), Some("job"));

        let result: ExternalContactMomentTaskResultResponse = serde_json::from_value(json!({
            "status": 2,
            "type": "add_moment_task",
            "result": { "moment_id": "moment" }
        }))
        .unwrap();
        assert_eq!(result.status, Some(2));
        assert_eq!(result.result_type.as_deref(), Some("add_moment_task"));
        assert_eq!(result.result.unwrap().moment_id.as_deref(), Some("moment"));

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
                "create_time": 1_720_000_000
            }],
            "next_cursor": "next"
        }))
        .unwrap();
        assert_eq!(strategies.strategy[0].strategy_name.as_deref(), Some("vip"));
        assert_eq!(strategies.strategy[0].admin_list[0], "admin");
        assert_eq!(strategies.next_cursor.as_deref(), Some("next"));

        let range: ExternalContactMomentStrategyRangeResponse = serde_json::from_value(json!({
            "range": { "user_list": ["user"], "party_list": [2], "tag_list": ["tag"] },
            "next_cursor": "next"
        }))
        .unwrap();
        let range_info = range.range.unwrap();
        assert_eq!(range_info.user_list[0], "user");
        assert_eq!(range_info.party_list[0], 2);

        let created_strategy: ExternalContactMomentStrategyCreateResponse =
            serde_json::from_value(json!({ "strategy_id": 100 })).unwrap();
        assert_eq!(created_strategy.strategy_id, Some(100));
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
            owner_filter: Some(json!({ "userid_list": ["owner"] })),
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
                    "reply_percentage": 99.5
                }]
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

        let statistic_response: ExternalGroupChatStatisticResponse =
            serde_json::from_value(json!({
                "total": 1,
                "next_offset": 50,
                "items": [{
                    "owner": "owner",
                    "data": { "new_chat_cnt": 1, "msg_total": 2 }
                }]
            }))
            .unwrap();
        assert_eq!(statistic_response.total, Some(1));
        assert_eq!(statistic_response.items[0].owner.as_deref(), Some("owner"));
        assert_eq!(
            statistic_response.items[0].data.as_ref().unwrap().msg_total,
            Some(2)
        );
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
                "create_time": 1_720_000_000
            }]
        }))
        .unwrap();
        assert_eq!(list.strategy[0].strategy_id, Some(1));
        assert_eq!(list.strategy[0].strategy_name.as_deref(), Some("strategy"));
        assert_eq!(list.strategy[0].admin_list[0], "admin");

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
                }
            }
        }))
        .unwrap();
        let strategy = detail.strategy.unwrap();
        assert_eq!(strategy.strategy_name.as_deref(), Some("strategy"));
        assert!(strategy.privilege.unwrap().view_customer_list);

        let range: ExternalContactCustomerStrategyRangeResponse = serde_json::from_value(json!({
            "range": [{ "type": 2, "userid": "user" }]
        }))
        .unwrap();
        assert_eq!(range.range[0].kind, 2);
        assert_eq!(range.range[0].userid.as_deref(), Some("user"));

        let created: ExternalContactCustomerStrategyCreateResponse =
            serde_json::from_value(json!({ "strategy_id": 3 })).unwrap();
        assert_eq!(created.strategy_id, Some(3));
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
            "department": [{
                "id": 1,
                "name": "Engineering",
                "name_en": "Engineering",
                "parentid": 0,
                "order": 10,
                "department_leader": ["leader"]
            }]
        }))
        .unwrap();
        assert_eq!(departments.department[0].id, Some(1));
        assert_eq!(
            departments.department[0].name.as_deref(),
            Some("Engineering")
        );
        assert_eq!(departments.department[0].department_leader[0], "leader");

        let simple: WorkDepartmentSimpleListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "department_id": [{
                "id": 1,
                "name": "Engineering",
                "parentid": 0,
                "order": 10
            }]
        }))
        .unwrap();
        assert_eq!(simple.department_id[0].id, Some(1));
        assert_eq!(simple.department_id[0].parentid, Some(0));

        let detail: WorkDepartmentDetailResponse = serde_json::from_value(json!({
            "errcode": 0,
            "id": 1,
            "name": "Engineering",
            "parentid": 0,
            "department_leader": ["leader"]
        }))
        .unwrap();
        assert_eq!(detail.department.id, Some(1));
        assert_eq!(detail.department.department_leader[0], "leader");
    }

    #[test]
    fn deserializes_work_agent_responses() {
        let list: WorkAgentListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "agentlist": [{
                "agentid": 100001,
                "name": "App",
                "square_logo_url": "https://example.com/logo.png",
                "round_logo_url": "https://example.com/round.png"
            }]
        }))
        .unwrap();
        assert_eq!(list.agentlist[0].agentid, Some(100001));
        assert_eq!(list.agentlist[0].name.as_deref(), Some("App"));

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
            "customized_publish_status": 1
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
            }
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

        let token: WorkAccessTokenResponse = serde_json::from_value(json!({
            "access_token": "token",
            "expires_in": 7200
        }))
        .unwrap();
        assert_eq!(token.access_token.as_deref(), Some("token"));
        assert_eq!(token.expires_in, Some(7200));
    }

    #[test]
    fn deserializes_work_corpgroup_responses() {
        let share: WorkCorpGroupAppShareInfoResponse = serde_json::from_value(json!({
            "corp_list": [{ "corpid": "corp", "agentid": 100001 }]
        }))
        .unwrap();
        assert_eq!(share.corp_list[0].corpid.as_deref(), Some("corp"));
        assert_eq!(share.corp_list[0].agentid, Some(100001));

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
            extattr: None,
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
            "dept_user": [{ "userid": "user", "department": 1 }]
        }))
        .unwrap();
        assert_eq!(list_id.next_cursor.as_deref(), Some("cursor"));
        assert_eq!(list_id.dept_user[0].userid.as_deref(), Some("user"));

        let lookup: WorkUserIdLookupResponse =
            serde_json::from_value(json!({ "userid": "user" })).unwrap();
        assert_eq!(lookup.userid.as_deref(), Some("user"));

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
            }
        }))
        .unwrap();
        assert_eq!(user.errcode, Some(0));
        assert_eq!(user.user.userid.as_deref(), Some("user"));
        assert_eq!(user.user.department[0], 1);
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
            "userlist": [{
                "userid": "user",
                "name": "User",
                "department": [1],
                "open_userid": "open-user"
            }]
        }))
        .unwrap();
        assert_eq!(simple_list.userlist[0].userid.as_deref(), Some("user"));
        assert_eq!(simple_list.userlist[0].department[0], 1);
        assert_eq!(
            simple_list.userlist[0].open_userid.as_deref(),
            Some("open-user")
        );

        let detail_list: WorkDepartmentUserDetailListResponse = serde_json::from_value(json!({
            "errcode": 0,
            "userlist": [{
                "userid": "user",
                "name": "User",
                "mobile": "13800000000",
                "department": [1]
            }]
        }))
        .unwrap();
        assert_eq!(detail_list.userlist[0].userid.as_deref(), Some("user"));
        assert_eq!(
            detail_list.userlist[0].mobile.as_deref(),
            Some("13800000000")
        );

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
            "invalidtag": [3]
        }))
        .unwrap();
        assert_eq!(invite_response.invaliduser[0], "bad-user");
        assert_eq!(invite_response.invalidparty[0], 2);
        assert_eq!(invite_response.invalidtag[0], 3);

        let qrcode: WorkJoinQrCodeResponse =
            serde_json::from_value(json!({ "join_qrcode": "https://example.com/qr" })).unwrap();
        assert_eq!(
            qrcode.join_qrcode.as_deref(),
            Some("https://example.com/qr")
        );

        let active: WorkUserActiveStatResponse =
            serde_json::from_value(json!({ "active_cnt": "42" })).unwrap();
        assert_eq!(active.active_cnt.as_deref(), Some("42"));
    }

    #[test]
    fn deserializes_work_linked_corp_user_responses() {
        let perm: WorkLinkedCorpPermListResponse = serde_json::from_value(json!({
            "department_ids": ["Corp/department"],
            "userids": ["Corp/user"]
        }))
        .unwrap();
        assert_eq!(perm.department_ids[0], "Corp/department");
        assert_eq!(perm.userids[0], "Corp/user");

        let user: WorkLinkedCorpUserResponse = serde_json::from_value(json!({
            "user_info": {
                "userid": "Corp/user",
                "name": "User",
                "mobile": "13800000000",
                "department": ["Corp/department"]
            }
        }))
        .unwrap();
        let user_info = user.user_info.unwrap();
        assert_eq!(user_info.userid.as_deref(), Some("Corp/user"));
        assert_eq!(user_info.name.as_deref(), Some("User"));
        assert_eq!(user_info.mobile.as_deref(), Some("13800000000"));
        assert_eq!(user_info.department[0], "Corp/department");

        let simple: WorkLinkedCorpUserListResponse = serde_json::from_value(json!({
            "userlist": [{ "userid": "Corp/user", "name": "User" }]
        }))
        .unwrap();
        assert_eq!(simple.userlist[0].userid.as_deref(), Some("Corp/user"));
        assert_eq!(simple.userlist[0].name.as_deref(), Some("User"));

        let departments: WorkLinkedCorpDepartmentListResponse = serde_json::from_value(json!({
            "department_list": [{
                "department_id": "Corp/department",
                "name": "Dept",
                "parentid": "Corp/root",
                "order": 1
            }]
        }))
        .unwrap();
        assert_eq!(
            departments.department_list[0].department_id.as_deref(),
            Some("Corp/department")
        );
        assert_eq!(departments.department_list[0].name.as_deref(), Some("Dept"));
        assert_eq!(departments.department_list[0].order, Some(1));
    }

    #[test]
    fn serializes_work_user_batch_and_export_jobs() {
        let batch = serde_json::to_value(WorkUserBatchJobRequest {
            media_id: "media".to_string(),
            to_invite: true,
            callbacks: Some(json!({
                "url": "https://example.com/callback",
                "token": "token",
                "encodingaeskey": "key"
            })),
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
            serde_json::from_value(json!({ "jobid": "batch-job" })).unwrap();
        assert_eq!(batch_job.jobid.as_deref(), Some("batch-job"));

        let batch_result: WorkUserBatchJobResultResponse = serde_json::from_value(json!({
            "status": 2,
            "type": "sync_user",
            "total": 10,
            "percentage": 100,
            "result": [{ "userid": "user", "errcode": 0 }]
        }))
        .unwrap();
        assert_eq!(batch_result.job_type.as_deref(), Some("sync_user"));
        assert_eq!(batch_result.result[0].userid.as_deref(), Some("user"));
        assert_eq!(batch_result.result[0].errcode, Some(0));

        let export_job: WorkUserExportJobResponse =
            serde_json::from_value(json!({ "jobid": "export-job" })).unwrap();
        assert_eq!(export_job.jobid.as_deref(), Some("export-job"));

        let export_result: WorkUserExportJobResultResponse = serde_json::from_value(json!({
            "status": 2,
            "data_list": [{
                "userid": "user",
                "name": "User",
                "department": [1],
                "mobile": "13800000000"
            }]
        }))
        .unwrap();
        assert_eq!(export_result.status, Some(2));
        assert_eq!(export_result.data_list[0].userid.as_deref(), Some("user"));
        assert_eq!(export_result.data_list[0].name.as_deref(), Some("User"));
        assert_eq!(export_result.data_list[0].department[0], 1);
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
                    "tax_amount": 6
                }]
            }
        }))
        .unwrap();
        assert_eq!(info.card_id.as_deref(), Some("card"));
        assert_eq!(info.invoice_type.as_deref(), Some("vat"));
        let user_info = info.user_info.unwrap();
        assert_eq!(user_info.fee, Some(100));
        assert_eq!(user_info.title.as_deref(), Some("Roze"));
        assert_eq!(user_info.billing_no.as_deref(), Some("NO100"));
        assert_eq!(user_info.info[0].name.as_deref(), Some("Cloud service"));
        assert_eq!(user_info.info[0].tax_amount, Some(6));

        let batch: WorkInvoiceInfoBatchResponse = serde_json::from_value(json!({
            "item_list": [{
                "card_id": "card",
                "encrypt_code": "encrypted",
                "reimburse_status": "INVOICE_REIMBURSE_INIT",
                "user_info": {
                    "fee": 100,
                    "title": "Roze",
                    "info": [{ "name": "Cloud service", "fee": 100 }]
                }
            }]
        }))
        .unwrap();
        assert_eq!(batch.item_list[0].card_id.as_deref(), Some("card"));
        assert_eq!(
            batch.item_list[0].reimburse_status.as_deref(),
            Some("INVOICE_REIMBURSE_INIT")
        );
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
        assert_eq!(
            merchant.allow_use_scope[0].scope_type.as_deref(),
            Some("all")
        );

        let bills: WorkExternalPayBillListResponse = serde_json::from_value(json!({
            "next_cursor": "cursor",
            "bill_list": [{
                "out_trade_no": "trade-no",
                "transaction_id": "transaction",
                "payee_userid": "payee",
                "payer_userid": "payer",
                "amount": 100,
                "status": "success",
                "pay_time": 1_800_000_000
            }]
        }))
        .unwrap();
        assert_eq!(bills.next_cursor.as_deref(), Some("cursor"));
        assert_eq!(bills.bill_list[0].out_trade_no.as_deref(), Some("trade-no"));
        assert_eq!(bills.bill_list[0].amount, Some(100));
        assert_eq!(bills.bill_list[0].payee_userid.as_deref(), Some("payee"));
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
            "calendar_list": [{
                "cal_id": "wc100",
                "summary": "Team",
                "shares": [{ "userid": "user", "readonly": 1 }]
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

        let dial: WorkDialRecordResponse = serde_json::from_value(json!({
            "record": [{ "callee": "user", "caller": "agent", "duration": 60 }]
        }))
        .unwrap();
        assert_eq!(dial.record[0].callee.as_deref(), Some("user"));
        assert_eq!(dial.record[0].duration, Some(60));

        let call: WorkPstnccCallResponse = serde_json::from_value(json!({
            "states": [{ "callee_userid": "user", "callid": "call-1", "state": 1 }]
        }))
        .unwrap();
        assert_eq!(call.states[0].callid.as_deref(), Some("call-1"));
        assert_eq!(call.states[0].state, Some(1));

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
            items: vec![json!({ "userid": "user", "day": 20260716 })],
            yearmonth: 202607,
        })
        .unwrap();
        assert_eq!(schedule["groupid"], 1);
        assert_eq!(schedule["items"][0]["userid"], "user");

        let option = serde_json::to_value(WorkCheckinOptionMutationRequest {
            group: json!({
                "groupid": 1,
                "groupname": "default",
                "range": { "userid": ["user"] }
            }),
        })
        .unwrap();
        assert_eq!(option["group"]["groupname"], "default");
        assert_eq!(option["group"]["range"]["userid"][0], "user");

        let apply = serde_json::to_value(WorkApprovalApplyEventRequest {
            creator_userid: "user".to_string(),
            template_id: "template".to_string(),
            use_template_approver: Some(1),
            approver: vec![json!({ "attr": 1, "userid": ["manager"] })],
            notifyer: vec!["notify".to_string()],
            notify_type: Some(1),
            apply_data: json!({ "contents": [{ "control": "Text", "value": { "text": "hi" } }] }),
            summary_list: vec![json!({ "summary_info": [{ "text": "hi", "lang": "zh_CN" }] })],
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
            filters: vec![json!({ "key": "template_id", "value": "template" })],
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
            lists: vec![WorkVacationQuotaUpdateItem {
                vacation_id: 1,
                leftduration: 3600,
                time_attr: Some(0),
                extra: serde_json::Value::Null,
            }],
        })
        .unwrap();
        assert_eq!(quota["userid"], "user");
        assert_eq!(quota["lists"][0]["vacation_id"], 1);
        assert_eq!(quota["lists"][0]["leftduration"], 3600);
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
            "info": {
                "journaluuid": "journal-1",
                "template_id": "template-1",
                "creator": "user",
                "apply_time": 1_800_000_000,
                "state": 1,
                "apply_data": { "contents": [] },
                "comments": [{ "comment_userid": "manager" }]
            }
        }))
        .unwrap();
        let detail = detail.info.unwrap();
        assert_eq!(detail.journaluuid.as_deref(), Some("journal-1"));
        assert_eq!(detail.creator.as_deref(), Some("user"));
        assert_eq!(detail.comments[0]["comment_userid"], "manager");

        let stats: WorkJournalStatListResponse = serde_json::from_value(json!({
            "stat_list": {
                "summary": [{ "userid": "user", "count": 3 }],
                "total": 3,
                "details": [{ "journaluuid": "journal-1" }]
            }
        }))
        .unwrap();
        let stats = stats.stat_list.unwrap();
        assert_eq!(stats.summary[0].userid.as_deref(), Some("user"));
        assert_eq!(stats.summary[0].count, Some(3));
        assert_eq!(stats.total, Some(3));

        let schedule_add: WorkScheduleAddResponse =
            serde_json::from_value(json!({ "schedule_id": "schedule-1" })).unwrap();
        assert_eq!(schedule_add.schedule_id.as_deref(), Some("schedule-1"));

        let schedule_get: WorkScheduleGetResponse = serde_json::from_value(json!({
            "schedule_list": [{
                "schedule_id": "schedule-1",
                "summary": "Daily",
                "organizer": "user",
                "start_time": 1_800_000_000,
                "end_time": 1_800_003_600,
                "attendees": [{ "userid": "user" }],
                "status": 1
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
        assert_eq!(schedule_get.schedule_list[0].status, Some(1));
    }

    #[test]
    fn deserializes_work_oa_checkin_approval_and_vacation_responses() {
        let corp_option: WorkCheckinCorpOptionResponse = serde_json::from_value(json!({
            "group": [{
                "groupid": 1,
                "groupname": "Default",
                "checkindate": [{ "workdays": [1, 2, 3], "flex_time": 30 }]
            }]
        }))
        .unwrap();
        assert_eq!(corp_option.group[0].groupid, Some(1));
        assert_eq!(corp_option.group[0].groupname.as_deref(), Some("Default"));
        assert_eq!(corp_option.group[0].checkindate[0].flex_time, Some(30));

        let option: WorkCheckinOptionResponse = serde_json::from_value(json!({
            "info": [{ "userid": "user", "groupid": 1, "groupname": "Default" }]
        }))
        .unwrap();
        assert_eq!(option.info[0].userid.as_deref(), Some("user"));
        assert_eq!(option.info[0].groupid, Some(1));

        let record: WorkCheckinRecordResponse = serde_json::from_value(json!({
            "checkindata": [{
                "userid": "user",
                "checkin_type": "上班打卡",
                "checkin_time": 1_800_000_000
            }]
        }))
        .unwrap();
        assert_eq!(record.checkindata[0].userid.as_deref(), Some("user"));
        assert_eq!(
            record.checkindata[0].checkin_type.as_deref(),
            Some("上班打卡")
        );
        assert_eq!(record.checkindata[0].checkin_time, Some(1_800_000_000));

        let day: WorkCheckinDataResponse = serde_json::from_value(json!({
            "datas": [{ "userid": "user", "groupid": 1, "base_info": {} }]
        }))
        .unwrap();
        assert_eq!(day.datas[0].userid.as_deref(), Some("user"));
        assert_eq!(day.datas[0].groupid, Some(1));
        assert!(day.datas[0].base_info.is_some());

        let schedule: WorkCheckinScheduleListResponse = serde_json::from_value(json!({
            "schedule_list": [{ "userid": "user", "schedule_id": 1, "groupid": 2 }]
        }))
        .unwrap();
        assert_eq!(schedule.schedule_list[0].userid.as_deref(), Some("user"));
        assert_eq!(schedule.schedule_list[0].schedule_id, Some(1));
        assert_eq!(schedule.schedule_list[0].groupid, Some(2));

        let template: WorkApprovalTemplateDetailResponse = serde_json::from_value(json!({
            "template_names": [{ "text": "Leave", "lang": "zh_CN" }],
            "template_content": { "controls": [] }
        }))
        .unwrap();
        assert_eq!(template.template_names[0]["text"], "Leave");

        let apply: WorkApprovalApplyEventResponse =
            serde_json::from_value(json!({ "sp_no": "202607160001" })).unwrap();
        assert_eq!(apply.sp_no.as_deref(), Some("202607160001"));

        let info: WorkApprovalInfoResponse = serde_json::from_value(json!({
            "sp_no_list": ["202607160001"],
            "new_next_cursor": "cursor"
        }))
        .unwrap();
        assert_eq!(info.sp_no_list[0], "202607160001");

        let detail: WorkApprovalDetailResponse = serde_json::from_value(json!({
            "info": { "sp_no": "202607160001" }
        }))
        .unwrap();
        assert_eq!(detail.info.unwrap()["sp_no"], "202607160001");

        let data: WorkApprovalDataResponse = serde_json::from_value(json!({
            "count": 1,
            "total": 1,
            "next_spnum": 2,
            "data": { "sp_no": "1" }
        }))
        .unwrap();
        assert_eq!(data.next_spnum, Some(2));

        let vacation: WorkVacationConfigResponse = serde_json::from_value(json!({
            "lists": [{ "id": 1, "name": "Annual Leave" }]
        }))
        .unwrap();
        assert_eq!(vacation.lists[0]["name"], "Annual Leave");
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
            attendees: json!({ "userids": ["user"] }),
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
            attendees: json!({ "userids": ["user"] }),
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
            city: "Shanghai".to_string(),
            building: "HQ".to_string(),
            floor: "3".to_string(),
            equipment: vec![1, 2],
            coordinate: json!({ "longitude": "121.5", "latitude": "31.2" }),
        })
        .unwrap();
        assert_eq!(room["name"], "Room A");
        assert_eq!(room["coordinate"]["longitude"], "121.5");

        let room_edit = serde_json::to_value(WorkMeetingRoomEditRequest {
            meetingroom_id: 7,
            name: "Room B".to_string(),
            capacity: 10,
            city: "Shanghai".to_string(),
            building: "HQ".to_string(),
            floor: "4".to_string(),
            equipment: vec![1],
            coordinate: json!({ "longitude": "121.5" }),
        })
        .unwrap();
        assert_eq!(room_edit["meetingroom_id"], 7);
        assert_eq!(room_edit["capacity"], 10);

        let room_list = serde_json::to_value(WorkMeetingRoomListRequest {
            city: "Shanghai".to_string(),
            building: "HQ".to_string(),
            floor: "3".to_string(),
            equipment: vec![1],
        })
        .unwrap();
        assert_eq!(room_list["equipment"][0], 1);

        let booking_info = serde_json::to_value(WorkMeetingRoomGetBookingInfoRequest {
            meetingroom_id: 7,
            start_time: 1_800_000_000,
            end_time: 1_800_003_600,
            city: "Shanghai".to_string(),
            building: "HQ".to_string(),
            floor: "3".to_string(),
        })
        .unwrap();
        assert_eq!(booking_info["meetingroom_id"], 7);

        let book = serde_json::to_value(WorkMeetingRoomBookRequest {
            meetingroom_id: 7,
            subject: "Weekly".to_string(),
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
            keep_schedule: 1,
        })
        .unwrap();
        assert_eq!(cancel["meeting_id"], "meeting-1");

        let form = serde_json::to_value(WorkWeDocCreateFormRequest {
            spaceid: "space".to_string(),
            fatherid: "father".to_string(),
            form_info: json!({
                "form_title": "Survey",
                "form_question": { "items": [] }
            }),
        })
        .unwrap();
        assert_eq!(form["spaceid"], "space");
        assert_eq!(form["form_info"]["form_title"], "Survey");
    }

    #[test]
    fn deserializes_work_oa_meeting_meetingroom_and_wedoc_responses() {
        let meeting_create: WorkMeetingCreateResponse =
            serde_json::from_value(json!({ "meetingid": 123 })).unwrap();
        assert_eq!(meeting_create.meetingid, Some(123));

        let meeting_ids: WorkMeetingGetUserMeetingIdResponse = serde_json::from_value(json!({
            "next_cursor": "next",
            "meetingid_list": ["123"]
        }))
        .unwrap();
        assert_eq!(meeting_ids.next_cursor.as_deref(), Some("next"));
        assert_eq!(meeting_ids.meetingid_list[0], "123");

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
            "attendees": { "userids": ["user"] }
        }))
        .unwrap();
        assert_eq!(meeting_info.creator_userid.as_deref(), Some("creator"));
        assert_eq!(meeting_info.meeting_type, Some(1));
        assert_eq!(meeting_info.attendees.unwrap()["userids"][0], "user");

        let room_add: WorkMeetingRoomAddResponse =
            serde_json::from_value(json!({ "meetingroom_id": 7 })).unwrap();
        assert_eq!(room_add.meetingroom_id, Some(7));

        let room_list: WorkMeetingRoomListResponse = serde_json::from_value(json!({
            "meetingroom_list": [{
                "meetingroom_id": 7,
                "name": "Room A",
                "capacity": 12,
                "equipment": [1, 2]
            }]
        }))
        .unwrap();
        assert_eq!(
            room_list.meetingroom_list[0].name.as_deref(),
            Some("Room A")
        );
        assert_eq!(room_list.meetingroom_list[0].capacity, Some(12));

        let room_booking: WorkMeetingRoomGetBookingInfoResponse = serde_json::from_value(json!({
            "booking_list": [{
                "meeting_id": 123,
                "schedule_id": 456,
                "subject": "Weekly",
                "booker": "user",
                "attendees": ["user", "other"]
            }]
        }))
        .unwrap();
        assert_eq!(
            room_booking.booking_list[0].subject.as_deref(),
            Some("Weekly")
        );
        assert_eq!(room_booking.booking_list[0].schedule_id, Some(456));

        let room_book: WorkMeetingRoomBookResponse =
            serde_json::from_value(json!({ "meeting_id": 123, "schedule_id": 456 })).unwrap();
        assert_eq!(room_book.meeting_id, Some(123));
        assert_eq!(room_book.schedule_id, Some(456));

        let form: WorkWeDocCreateFormResponse =
            serde_json::from_value(json!({ "formid": "form-1" })).unwrap();
        assert_eq!(form.formid.as_deref(), Some("form-1"));
    }

    #[test]
    fn serializes_work_oa_living_and_wedrive_requests() {
        let living = serde_json::to_value(WorkLivingCreateRequest {
            anchor_userid: "anchor".to_string(),
            theme: "Launch".to_string(),
            living_start: 1_800_000_000,
            living_duration: 3600,
            description: "product update".to_string(),
            living_type: 1,
            agentid: 100001,
            remind_time: 15,
            activity_cover_mediaid: "cover".to_string(),
            activity_share_mediaid: "share".to_string(),
            activity_detail: json!({ "description": "detail" }),
        })
        .unwrap();
        assert_eq!(living["anchor_userid"], "anchor");
        assert_eq!(living["type"], 1);
        assert_eq!(living["activity_detail"]["description"], "detail");

        let modify = serde_json::to_value(WorkLivingModifyRequest {
            livingid: "living-1".to_string(),
            theme: "Launch updated".to_string(),
            living_start: 1_800_000_300,
            living_duration: 1800,
            description: "update".to_string(),
            living_type: 1,
            remind_time: 10,
        })
        .unwrap();
        assert_eq!(modify["livingid"], "living-1");
        assert_eq!(modify["living_duration"], 1800);

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
            auth_info: vec![json!({ "type": 1, "auth": 7 })],
        })
        .unwrap();
        assert_eq!(space["space_name"], "Team Space");
        assert_eq!(space["auth_info"][0]["auth"], 7);

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
            auth_info: vec![json!({ "userid": "member", "auth": 1 })],
        })
        .unwrap();
        assert_eq!(space_acl["auth_info"][0]["userid"], "member");

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
            auth_info: vec![json!({ "userid": "member", "auth": 1 })],
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
        let living: WorkLivingCreateResponse =
            serde_json::from_value(json!({ "livingid": 100 })).unwrap();
        assert_eq!(living.livingid, Some(100));

        let code: WorkLivingCodeResponse =
            serde_json::from_value(json!({ "living_code": 200 })).unwrap();
        assert_eq!(code.living_code, Some(200));

        let ids: WorkLivingGetUserAllLivingIdResponse = serde_json::from_value(json!({
            "next_cursor": "next",
            "livingid_list": ["living-1"]
        }))
        .unwrap();
        assert_eq!(ids.next_cursor.as_deref(), Some("next"));
        assert_eq!(ids.livingid_list[0], "living-1");

        let info: WorkLivingInfoResponse = serde_json::from_value(json!({
            "living_info": {
                "anchor_userid": "anchor",
                "theme": "Launch",
                "living_start": 1_800_000_000,
                "living_duration": 3600,
                "type": 1,
                "status": 2,
                "viewer_count": 3
            }
        }))
        .unwrap();
        let info = info.living_info.unwrap();
        assert_eq!(info.theme.as_deref(), Some("Launch"));
        assert_eq!(info.living_type, Some(1));
        assert_eq!(info.viewer_count, Some(3));

        let stat: WorkLivingWatchStatResponse = serde_json::from_value(json!({
            "next_key": "next",
            "stat_info": {
                "viewer_count": 3,
                "comment_count": 1,
                "watch_stat": [{
                    "viewer_userid": "viewer",
                    "watch_time": 120,
                    "is_comment": 1
                }]
            }
        }))
        .unwrap();
        let stat_info = stat.stat_info.unwrap();
        assert_eq!(stat_info.viewer_count, Some(3));
        assert_eq!(
            stat_info.watch_stat[0].viewer_userid.as_deref(),
            Some("viewer")
        );

        let share_info: WorkLivingShareInfoResponse = serde_json::from_value(json!({
            "livingid": "living-1",
            "viewer_userid": "viewer",
            "viewer_external_userid": "external-viewer",
            "invitor_userid": "invitor",
            "invitor_external_userid": "external-invitor"
        }))
        .unwrap();
        assert_eq!(share_info.viewer_userid.as_deref(), Some("viewer"));

        let space_create: WorkWeDriveSpaceCreateResponse =
            serde_json::from_value(json!({ "spaceid": "space" })).unwrap();
        assert_eq!(space_create.spaceid.as_deref(), Some("space"));

        let space_info: WorkWeDriveSpaceInfoResponse = serde_json::from_value(json!({
            "space_info": {
                "spaceid": "space",
                "space_name": "Team Space",
                "userid": "user",
                "quota": 1024,
                "auth_info": [{ "type": 1, "auth": 7 }]
            }
        }))
        .unwrap();
        let space_info = space_info.space_info.unwrap();
        assert_eq!(space_info.space_name.as_deref(), Some("Team Space"));
        assert_eq!(space_info.auth_info[0]["auth"], 7);

        let space_share: WorkWeDriveSpaceShareResponse = serde_json::from_value(json!({
            "space_share_url": "https://example.com/space"
        }))
        .unwrap();
        assert_eq!(
            space_share.space_share_url.as_deref(),
            Some("https://example.com/space")
        );

        let file_list: WorkWeDriveFileListResponse = serde_json::from_value(json!({
            "has_more": true,
            "next_start": 100,
            "file_list": [{ "fileid": "file", "file_name": "doc.txt", "file_size": 10 }]
        }))
        .unwrap();
        assert_eq!(file_list.has_more, Some(true));
        assert_eq!(file_list.file_list[0].fileid.as_deref(), Some("file"));
        assert_eq!(file_list.file_list[0].file_name.as_deref(), Some("doc.txt"));

        let upload: WorkWeDriveFileUploadResponse =
            serde_json::from_value(json!({ "fileid": "file" })).unwrap();
        assert_eq!(upload.fileid.as_deref(), Some("file"));

        let download: WorkWeDriveFileDownloadResponse = serde_json::from_value(json!({
            "download_url": "https://example.com/file",
            "cookie_name": "SESSION",
            "cookie_value": "value"
        }))
        .unwrap();
        assert_eq!(download.cookie_name.as_deref(), Some("SESSION"));

        let create: WorkWeDriveFileCreateResponse = serde_json::from_value(json!({
            "fileid": "file",
            "url": "https://example.com/doc"
        }))
        .unwrap();
        assert_eq!(create.url.as_deref(), Some("https://example.com/doc"));

        let rename: WorkWeDriveFileRenameResponse = serde_json::from_value(json!({
            "file": { "fileid": "file", "file_name": "new.txt" }
        }))
        .unwrap();
        assert_eq!(rename.file.unwrap().file_name.as_deref(), Some("new.txt"));

        let moved: WorkWeDriveFileMoveResponse = serde_json::from_value(json!({
            "file_list": {
                "success": ["file"],
                "failed": [{ "fileid": "bad", "errcode": 40001, "errmsg": "invalid" }]
            }
        }))
        .unwrap();
        let moved = moved.file_list.unwrap();
        assert_eq!(moved.success[0], "file");
        assert_eq!(moved.failed[0].fileid.as_deref(), Some("bad"));

        let share: WorkWeDriveFileShareResponse = serde_json::from_value(json!({
            "share_url": "https://example.com/share"
        }))
        .unwrap();
        assert_eq!(
            share.share_url.as_deref(),
            Some("https://example.com/share")
        );
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
            text: Some(json!({ "content": "hello" })),
            image: None,
            voice: None,
            video: None,
            file: None,
            link: None,
            miniprogram: None,
            menu: Some(json!({ "head_content": "choose", "list": [] })),
            location: None,
            ca_link: None,
        })
        .unwrap();
        assert_eq!(send["msgmenu"]["head_content"], "choose");
        assert!(send.get("image").is_none());

        let on_event = serde_json::to_value(WorkAccountServiceSendMsgOnEventRequest {
            code: "code".to_string(),
            msgid: "msg".to_string(),
            msgtype: "text".to_string(),
            text: Some(json!({ "content": "hello" })),
            menu: None,
        })
        .unwrap();
        assert_eq!(on_event["code"], "code");

        let state = serde_json::to_value(WorkAccountServiceStateTransRequest {
            open_kfid: "kf".to_string(),
            external_userid: "external".to_string(),
            service_state: 2,
            servicer_userid: "servicer".to_string(),
        })
        .unwrap();
        assert_eq!(state["service_state"], 2);

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
        let account_add: WorkAccountServiceAccountAddResponse =
            serde_json::from_value(json!({ "open_kfid": "kf" })).unwrap();
        assert_eq!(account_add.open_kfid.as_deref(), Some("kf"));

        let accounts: WorkAccountServiceAccountListResponse = serde_json::from_value(json!({
            "account_list": [{ "open_kfid": "kf", "name": "Support", "avatar": "https://example.com/a.png" }]
        }))
        .unwrap();
        assert_eq!(accounts.account_list[0].open_kfid.as_deref(), Some("kf"));
        assert_eq!(accounts.account_list[0].name.as_deref(), Some("Support"));
        assert_eq!(
            accounts.account_list[0].avatar.as_deref(),
            Some("https://example.com/a.png")
        );

        let contact_way: WorkAccountServiceAddContactWayResponse =
            serde_json::from_value(json!({ "url": "https://example.com/kf" })).unwrap();
        assert_eq!(contact_way.url.as_deref(), Some("https://example.com/kf"));

        let customers: WorkAccountServiceCustomerBatchGetResponse = serde_json::from_value(json!({
            "customer_list": [{
                "external_userid": "external",
                "nickname": "Customer",
                "gender": 1,
                "enter_session_context": { "scene": "scene", "scene_param": "param" }
            }],
            "invalid_external_userid": ["bad"]
        }))
        .unwrap();
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

        let config: WorkAccountServiceCustomerUpgradeServiceConfigResponse =
            serde_json::from_value(json!({
                "member_range": { "userid": ["servicer"] },
                "groupchat_range": { "chat_id": ["chat"] }
            }))
            .unwrap();
        assert_eq!(config.member_range.unwrap()["userid"][0], "servicer");

        let sync: WorkAccountServiceSyncMsgResponse = serde_json::from_value(json!({
            "next_cursor": "next",
            "has_more": 1,
            "msg_list": [
                {
                    "msgid": "msg",
                    "open_kfid": "kf",
                    "external_userid": "external",
                    "send_time": 100,
                    "origin": 3,
                    "msgtype": "text",
                    "text": { "content": "hello" }
                },
                {
                    "msgid": "image-msg",
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
                        "thumb_media_id": "thumb"
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
                        "welcome_code": "welcome"
                    }
                }
            ]
        }))
        .unwrap();
        assert_eq!(sync.next_cursor.as_deref(), Some("next"));
        assert_eq!(sync.msg_list[0].msgid.as_deref(), Some("msg"));
        assert_eq!(sync.msg_list[0].open_kfid.as_deref(), Some("kf"));
        assert_eq!(sync.msg_list[0].msgtype.as_deref(), Some("text"));
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
            sync.msg_list[1]
                .image
                .as_ref()
                .expect("image")
                .media_id
                .as_deref(),
            Some("image-media")
        );
        assert_eq!(
            sync.msg_list[2].link.as_ref().expect("link").url.as_deref(),
            Some("https://example.com")
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

        let send: WorkAccountServiceSendMsgResponse =
            serde_json::from_value(json!({ "msgid": "msg" })).unwrap();
        assert_eq!(send.msgid.as_deref(), Some("msg"));

        let servicer_result: WorkAccountServiceServicerResultResponse =
            serde_json::from_value(json!({
                "result_list": [{ "userid": "servicer", "errcode": 0 }]
            }))
            .unwrap();
        assert_eq!(
            servicer_result.result_list[0].userid.as_deref(),
            Some("servicer")
        );
        assert_eq!(servicer_result.result_list[0].errcode, Some(0));

        let servicers: WorkAccountServiceServicerListResponse = serde_json::from_value(json!({
            "servicer_list": [{ "userid": "servicer", "status": 1 }]
        }))
        .unwrap();
        assert_eq!(
            servicers.servicer_list[0].userid.as_deref(),
            Some("servicer")
        );
        assert_eq!(servicers.servicer_list[0].status, Some(1));

        let state: WorkAccountServiceStateGetResponse = serde_json::from_value(json!({
            "service_state": 2,
            "servicer_userid": "servicer"
        }))
        .unwrap();
        assert_eq!(state.service_state, Some(2));

        let tag_create: WorkAccountServiceTagCreateResponse =
            serde_json::from_value(json!({ "tagid": 1 })).unwrap();
        assert_eq!(tag_create.tagid, Some(1));

        let tag_detail: WorkAccountServiceTagDetailResponse = serde_json::from_value(json!({
            "tagname": "tag",
            "userlist": [{ "userid": "user", "name": "User" }],
            "partylist": [1]
        }))
        .unwrap();
        assert_eq!(tag_detail.tagname.as_deref(), Some("tag"));
        assert_eq!(tag_detail.userlist[0].userid.as_deref(), Some("user"));
        assert_eq!(tag_detail.userlist[0].name.as_deref(), Some("User"));
        assert_eq!(tag_detail.partylist[0], 1);

        let tag_user: WorkAccountServiceTagUserResultResponse = serde_json::from_value(json!({
            "invalidlist": "bad",
            "invalidparty": [2]
        }))
        .unwrap();
        assert_eq!(tag_user.invalidparty[0], 2);

        let tags: WorkAccountServiceTagListResponse = serde_json::from_value(json!({
            "taglist": [{ "tagid": 1, "tagname": "tag" }]
        }))
        .unwrap();
        assert_eq!(tags.taglist[0].tagid, Some(1));
        assert_eq!(tags.taglist[0].tagname.as_deref(), Some("tag"));

        let ok: WorkAiBotLongConnectionResponse = serde_json::from_value(json!({
            "cmd": "pong",
            "headers": { "req_id": "req-1" },
            "body": { "ok": true }
        }))
        .unwrap();
        assert!(!ok.is_error());
        assert_eq!(ok.headers.unwrap().req_id.as_deref(), Some("req-1"));

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

        let single_agree = json!({
            "info": [{ "userid": "user", "exteranalopenid": "openid" }]
        });
        assert_eq!(single_agree["info"][0]["userid"], "user");

        let room_agree = json!({ "roomid": "room" });
        assert_eq!(room_agree["roomid"], "room");

        let permit: WorkMsgAuditPermitUsersResponse = serde_json::from_value(json!({
            "errcode": 0,
            "ids": ["user", "external-openid"]
        }))
        .unwrap();
        assert_eq!(permit.ids[0], "user");
        assert_eq!(permit.ids[1], "external-openid");

        let chat_data: WorkMsgAuditChatDataResponse = serde_json::from_value(json!({
            "errcode": 0,
            "chatdata": [{
                "seq": 1,
                "msgid": "msg",
                "publickey_ver": 2,
                "encrypt_random_key": "random",
                "encrypt_chat_msg": "cipher"
            }]
        }))
        .unwrap();
        assert_eq!(chat_data.chatdata[0].seq, Some(1));
        assert_eq!(chat_data.chatdata[0].msgid.as_deref(), Some("msg"));
        assert_eq!(chat_data.chatdata[0].publickey_ver, Some(2));

        let room: WorkMsgAuditRoomResponse = serde_json::from_value(json!({
            "errcode": 0,
            "roomname": "room",
            "creator": "creator",
            "room_create_time": 1_720_000_000,
            "notice": "notice",
            "members": [{ "memberid": "user", "jointime": 1_720_000_001 }]
        }))
        .unwrap();
        assert_eq!(room.roomname.as_deref(), Some("room"));
        assert_eq!(room.members[0].memberid.as_deref(), Some("user"));

        let agree: WorkMsgAuditAgreeResponse = serde_json::from_value(json!({
            "errcode": 0,
            "agreeinfo": [{
                "userid": "user",
                "exteranalopenid": "openid",
                "agree_status": "Agree"
            }]
        }))
        .unwrap();
        assert_eq!(agree.agreeinfo[0].userid.as_deref(), Some("user"));
        assert_eq!(
            agree.agreeinfo[0].exteranalopenid.as_deref(),
            Some("openid")
        );
        assert_eq!(agree.agreeinfo[0].agree_status.as_deref(), Some("Agree"));

        let robot: WorkMsgAuditRobotInfoResponse = serde_json::from_value(json!({
            "errcode": 0,
            "robot_info": {
                "robot_id": "robot",
                "name": "Robot",
                "creator_userid": "creator"
            }
        }))
        .unwrap();
        let robot_info = robot.robot_info.unwrap();
        assert_eq!(robot_info.robot_id.as_deref(), Some("robot"));
        assert_eq!(robot_info.creator_userid.as_deref(), Some("creator"));
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

        let created: WorkAppChatCreateResponse =
            serde_json::from_value(json!({ "errcode": 0, "chatid": "chatid" })).unwrap();
        assert_eq!(created.chatid.as_deref(), Some("chatid"));

        let got: WorkAppChatGetResponse = serde_json::from_value(json!({
            "errcode": 0,
            "chat_info": {
                "chatid": "chatid",
                "name": "chat",
                "owner": "owner",
                "userlist": ["user"]
            }
        }))
        .unwrap();
        let chat = got.chat_info.unwrap();
        assert_eq!(chat.chatid.as_deref(), Some("chatid"));
        assert_eq!(chat.owner.as_deref(), Some("owner"));
        assert_eq!(chat.userlist[0], "user");
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
            "external_userid": "external"
        }))
        .unwrap();
        assert_eq!(info.userid.as_deref(), Some("legacy-user"));
        assert_eq!(info.user_ticket.as_deref(), Some("ticket"));
        assert_eq!(info.openid.as_deref(), Some("legacy-openid"));
        assert_eq!(info.external_userid.as_deref(), Some("external"));

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
            "address": "addr"
        }))
        .unwrap();
        assert_eq!(detail.userid.as_deref(), Some("user"));
        assert_eq!(detail.name.as_deref(), Some("User"));
        assert_eq!(detail.mobile.as_deref(), Some("13800000000"));
    }
}
