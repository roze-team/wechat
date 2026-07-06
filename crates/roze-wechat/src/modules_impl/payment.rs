use serde::{Deserialize, Serialize};
use serde_json::{to_value, Value};

use crate::{
    config::Platform,
    crypto,
    error::Result,
    modules::{DomainModule, PlatformClient},
    Client,
};

#[derive(Debug, Clone)]
pub struct Payment {
    inner: PlatformClient,
}

impl Payment {
    pub fn new(client: Client, platform: Platform) -> Self {
        Self {
            inner: PlatformClient::new(client, platform),
        }
    }

    pub fn bill(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "payment.bill")
    }

    pub fn fund_app(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "payment.fund_app")
    }

    pub async fn create_fund_app_transfer_bill(
        &self,
        credentials: &PaymentCredentials,
        request: FundAppTransferBillRequest,
    ) -> Result<FundAppTransferBillResponse> {
        self.post_v3(
            credentials,
            "/v3/fund-app/mch-transfer/transfer-bills",
            to_value(request)?,
        )
        .await
    }

    pub async fn query_fund_app_transfer_bill_by_out_no(
        &self,
        credentials: &PaymentCredentials,
        out_bill_no: impl AsRef<str>,
    ) -> Result<FundAppTransferBillDetailResponse> {
        let path = format!(
            "/v3/fund-app/mch-transfer/transfer-bills/out-bill-no/{}",
            out_bill_no.as_ref()
        );
        self.get_v3(credentials, &path, Vec::new()).await
    }

    pub async fn query_fund_app_transfer_bill_by_transfer_no(
        &self,
        credentials: &PaymentCredentials,
        transfer_bill_no: impl AsRef<str>,
    ) -> Result<FundAppTransferBillDetailResponse> {
        let path = format!(
            "/v3/fund-app/mch-transfer/transfer-bills/transfer-bill-no/{}",
            transfer_bill_no.as_ref()
        );
        self.get_v3(credentials, &path, Vec::new()).await
    }

    pub async fn cancel_fund_app_transfer_bill(
        &self,
        credentials: &PaymentCredentials,
        out_bill_no: impl AsRef<str>,
    ) -> Result<FundAppTransferBillCancelResponse> {
        let path = format!(
            "/v3/fund-app/mch-transfer/transfer-bills/out-bill-no/{}/cancel",
            out_bill_no.as_ref()
        );
        self.post_v3(credentials, &path, serde_json::json!({}))
            .await
    }

    pub async fn apply_fund_app_elec_sign_by_out_no(
        &self,
        credentials: &PaymentCredentials,
        out_bill_no: impl Into<String>,
    ) -> Result<FundAppElecSignApplyResponse> {
        self.post_v3(
            credentials,
            "/v3/fund-app/mch-transfer/elecsign/out-bill-no",
            serde_json::json!({ "out_bill_no": out_bill_no.into() }),
        )
        .await
    }

    pub async fn query_fund_app_elec_sign_by_out_no(
        &self,
        credentials: &PaymentCredentials,
        out_bill_no: impl AsRef<str>,
    ) -> Result<FundAppElecSignResponse> {
        let path = format!(
            "/v3/fund-app/mch-transfer/elecsign/out-bill-no/{}",
            out_bill_no.as_ref()
        );
        self.get_v3(credentials, &path, Vec::new()).await
    }

    pub fn jssdk(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "payment.jssdk")
    }

    pub fn base(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "payment.base")
    }

    pub fn build_jsapi_pay_params(
        credentials: &PaymentCredentials,
        app_id: impl Into<String>,
        prepay_id: impl Into<String>,
    ) -> Result<JsapiPayParams> {
        let app_id = app_id.into();
        let time_stamp = chrono::Utc::now().timestamp().to_string();
        let nonce_str = crypto::nonce_string(32);
        let package = format!("prepay_id={}", prepay_id.into());
        let message = format!("{app_id}\n{time_stamp}\n{nonce_str}\n{package}\n");
        let pay_sign =
            crypto::rsa_sha256_sign_base64(&credentials.private_key_pem, message.as_bytes())?;

        Ok(JsapiPayParams {
            app_id,
            time_stamp,
            nonce_str,
            package,
            sign_type: "RSA".to_string(),
            pay_sign,
        })
    }

    pub fn build_app_pay_params(
        credentials: &PaymentCredentials,
        app_id: impl Into<String>,
        prepay_id: impl Into<String>,
    ) -> Result<AppPayParams> {
        let app_id = app_id.into();
        let partner_id = credentials.mch_id.clone();
        let prepay_id = prepay_id.into();
        let package = "Sign=WXPay".to_string();
        let nonce_str = crypto::nonce_string(32);
        let timestamp = chrono::Utc::now().timestamp().to_string();
        let message = format!("{app_id}\n{timestamp}\n{nonce_str}\n{prepay_id}\n");
        let sign =
            crypto::rsa_sha256_sign_base64(&credentials.private_key_pem, message.as_bytes())?;

        Ok(AppPayParams {
            app_id,
            partner_id,
            prepay_id,
            package,
            nonce_str,
            timestamp,
            sign,
        })
    }

    pub fn notify(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "payment.notify")
    }

    pub fn order(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "payment.order")
    }

    pub async fn jsapi_transaction(
        &self,
        credentials: &PaymentCredentials,
        request: JsapiPrepayRequest,
    ) -> Result<PrepayResponse> {
        self.post_v3(
            credentials,
            "/v3/pay/transactions/jsapi",
            to_value(request)?,
        )
        .await
    }

    pub async fn app_transaction(
        &self,
        credentials: &PaymentCredentials,
        request: AppPrepayRequest,
    ) -> Result<PrepayResponse> {
        self.post_v3(credentials, "/v3/pay/transactions/app", to_value(request)?)
            .await
    }

    pub async fn h5_transaction(
        &self,
        credentials: &PaymentCredentials,
        request: H5PrepayRequest,
    ) -> Result<H5PrepayResponse> {
        self.post_v3(credentials, "/v3/pay/transactions/h5", to_value(request)?)
            .await
    }

    pub async fn native_transaction(
        &self,
        credentials: &PaymentCredentials,
        request: NativePrepayRequest,
    ) -> Result<NativePrepayResponse> {
        self.post_v3(
            credentials,
            "/v3/pay/transactions/native",
            to_value(request)?,
        )
        .await
    }

    pub async fn query_by_transaction_id(
        &self,
        credentials: &PaymentCredentials,
        transaction_id: impl AsRef<str>,
    ) -> Result<PaymentOrderResponse> {
        let path = format!("/v3/pay/transactions/id/{}", transaction_id.as_ref());
        self.get_v3(
            credentials,
            &path,
            vec![("mchid".to_string(), credentials.mch_id.clone())],
        )
        .await
    }

    pub async fn query_by_out_trade_no(
        &self,
        credentials: &PaymentCredentials,
        out_trade_no: impl AsRef<str>,
    ) -> Result<PaymentOrderResponse> {
        let path = format!(
            "/v3/pay/transactions/out-trade-no/{}",
            out_trade_no.as_ref()
        );
        self.get_v3(
            credentials,
            &path,
            vec![("mchid".to_string(), credentials.mch_id.clone())],
        )
        .await
    }

    pub async fn close_order(
        &self,
        credentials: &PaymentCredentials,
        out_trade_no: impl AsRef<str>,
    ) -> Result<PaymentStatusResponse> {
        let path = format!(
            "/v3/pay/transactions/out-trade-no/{}/close",
            out_trade_no.as_ref()
        );
        self.post_v3(
            credentials,
            &path,
            serde_json::json!({ "mchid": credentials.mch_id }),
        )
        .await
    }

    pub async fn reverse_order(
        &self,
        credentials: &PaymentCredentials,
        request: ReverseOrderRequest,
    ) -> Result<PaymentStatusResponse> {
        let path = format!(
            "/v3/pay/transactions/out-trade-no/{}/close",
            request.out_trade_no
        );
        let body = request.into_body(&credentials.mch_id);
        self.post_v3(credentials, &path, body).await
    }

    pub fn partner(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "payment.partner")
    }

    pub async fn partner_jsapi_transaction(
        &self,
        credentials: &PaymentCredentials,
        request: PartnerJsapiPrepayRequest,
    ) -> Result<PrepayResponse> {
        self.post_v3(
            credentials,
            "/v3/pay/partner/transactions/jsapi",
            to_value(request)?,
        )
        .await
    }

    pub async fn partner_app_transaction(
        &self,
        credentials: &PaymentCredentials,
        request: PartnerAppPrepayRequest,
    ) -> Result<PrepayResponse> {
        self.post_v3(
            credentials,
            "/v3/pay/partner/transactions/app",
            to_value(request)?,
        )
        .await
    }

    pub async fn partner_h5_transaction(
        &self,
        credentials: &PaymentCredentials,
        request: PartnerH5PrepayRequest,
    ) -> Result<H5PrepayResponse> {
        self.post_v3(
            credentials,
            "/v3/pay/partner/transactions/h5",
            to_value(request)?,
        )
        .await
    }

    pub async fn partner_native_transaction(
        &self,
        credentials: &PaymentCredentials,
        request: PartnerNativePrepayRequest,
    ) -> Result<NativePrepayResponse> {
        self.post_v3(
            credentials,
            "/v3/pay/partner/transactions/native",
            to_value(request)?,
        )
        .await
    }

    pub async fn partner_query_by_out_trade_no(
        &self,
        credentials: &PaymentCredentials,
        request: PartnerOrderQuery,
    ) -> Result<PaymentOrderResponse> {
        let path = format!(
            "/v3/pay/partner/transactions/out-trade-no/{}",
            request.out_trade_no
        );
        self.get_v3(credentials, &path, request.into_query()).await
    }

    pub async fn partner_close_order(
        &self,
        credentials: &PaymentCredentials,
        request: PartnerCloseOrderRequest,
    ) -> Result<PaymentStatusResponse> {
        let path = format!(
            "/v3/pay/partner/transactions/out-trade-no/{}/close",
            request.out_trade_no
        );
        self.post_v3(
            credentials,
            &path,
            serde_json::json!({
                "sp_mchid": request.sp_mchid,
                "sub_mchid": request.sub_mchid,
            }),
        )
        .await
    }

    pub async fn reverse_partner_order(
        &self,
        credentials: &PaymentCredentials,
        request: PartnerCloseOrderRequest,
    ) -> Result<PaymentStatusResponse> {
        self.partner_close_order(credentials, request).await
    }

    pub fn profit_sharing(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "payment.profit_sharing")
    }

    pub async fn add_profit_sharing_receiver(
        &self,
        credentials: &PaymentCredentials,
        request: ProfitSharingReceiverRequest,
    ) -> Result<ProfitSharingResponse> {
        self.post_v3(
            credentials,
            "/v3/profitsharing/receivers/add",
            to_value(request)?,
        )
        .await
    }

    pub async fn delete_profit_sharing_receiver(
        &self,
        credentials: &PaymentCredentials,
        request: ProfitSharingReceiverRequest,
    ) -> Result<ProfitSharingResponse> {
        self.post_v3(
            credentials,
            "/v3/profitsharing/receivers/delete",
            to_value(request)?,
        )
        .await
    }

    pub async fn create_profit_sharing_order(
        &self,
        credentials: &PaymentCredentials,
        request: ProfitSharingOrderRequest,
    ) -> Result<ProfitSharingResponse> {
        self.post_v3(credentials, "/v3/profitsharing/orders", to_value(request)?)
            .await
    }

    pub async fn query_profit_sharing_order(
        &self,
        credentials: &PaymentCredentials,
        transaction_id: impl Into<String>,
        out_order_no: impl Into<String>,
    ) -> Result<ProfitSharingResponse> {
        self.get_v3(
            credentials,
            "/v3/profitsharing/orders",
            vec![
                ("transaction_id".to_string(), transaction_id.into()),
                ("out_order_no".to_string(), out_order_no.into()),
            ],
        )
        .await
    }

    pub fn refund(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "payment.refund")
    }

    pub async fn create_refund(
        &self,
        credentials: &PaymentCredentials,
        request: RefundRequest,
    ) -> Result<RefundResponse> {
        self.post_v3(
            credentials,
            "/v3/refund/domestic/refunds",
            to_value(request)?,
        )
        .await
    }

    pub async fn query_refund(
        &self,
        credentials: &PaymentCredentials,
        out_refund_no: impl AsRef<str>,
    ) -> Result<RefundResponse> {
        let path = format!("/v3/refund/domestic/refunds/{}", out_refund_no.as_ref());
        self.get_v3(credentials, &path, Vec::new()).await
    }

    pub fn reverse(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "payment.reverse")
    }

    pub fn transfer(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "payment.transfer")
    }

    pub fn merchant_service(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "payment.merchant_service")
    }

    pub async fn query_complaints(
        &self,
        credentials: &PaymentCredentials,
        request: ComplaintListRequest,
    ) -> Result<ComplaintListResponse> {
        self.get_v3(
            credentials,
            "/v3/merchant-service/complaints-v2",
            request.into_query(),
        )
        .await
    }

    pub async fn query_complaint_detail(
        &self,
        credentials: &PaymentCredentials,
        complaint_id: impl AsRef<str>,
    ) -> Result<ComplaintDetailResponse> {
        let path = format!(
            "/v3/merchant-service/complaints-v2/{}",
            complaint_id.as_ref()
        );
        self.get_v3(credentials, &path, Vec::new()).await
    }

    pub fn pay_score(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "payment.pay_score")
    }

    pub async fn create_pay_score_service_order(
        &self,
        credentials: &PaymentCredentials,
        request: PayScoreServiceOrderRequest,
    ) -> Result<PayScoreServiceOrderResponse> {
        self.post_v3(credentials, "/v3/payscore/serviceorder", to_value(request)?)
            .await
    }

    pub async fn query_pay_score_service_order(
        &self,
        credentials: &PaymentCredentials,
        request: PayScoreServiceOrderQuery,
    ) -> Result<PayScoreServiceOrderResponse> {
        self.get_v3(
            credentials,
            "/v3/payscore/serviceorder",
            request.into_query(),
        )
        .await
    }

    pub fn security(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "payment.security")
    }

    pub async fn get_certificates(
        &self,
        credentials: &PaymentCredentials,
    ) -> Result<CertificateListResponse> {
        self.get_v3(credentials, "/v3/certificates", Vec::new())
            .await
    }

    pub fn tax(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "payment.tax")
    }

    pub async fn apply_tax_card_template(
        &self,
        credentials: &PaymentCredentials,
        request: TaxCardTemplateRequest,
    ) -> Result<TaxCardTemplateResponse> {
        self.post_v3(
            credentials,
            "/v3/new-tax-control-fapiao/card-template",
            to_value(request)?,
        )
        .await
    }

    pub async fn create_transfer_batch(
        &self,
        credentials: &PaymentCredentials,
        request: TransferBatchRequest,
    ) -> Result<TransferBatchResponse> {
        self.post_v3(credentials, "/v3/transfer/batches", to_value(request)?)
            .await
    }

    pub async fn query_transfer_batch_by_out_no(
        &self,
        credentials: &PaymentCredentials,
        request: TransferBatchQuery,
    ) -> Result<TransferBatchResponse> {
        let path = format!("/v3/transfer/batches/out-batch-no/{}", request.out_batch_no);
        self.get_v3(credentials, &path, request.into_query()).await
    }

    pub async fn query_transfer_detail_by_out_no(
        &self,
        credentials: &PaymentCredentials,
        out_batch_no: impl AsRef<str>,
        out_detail_no: impl AsRef<str>,
    ) -> Result<TransferDetailResponse> {
        let path = format!(
            "/v3/transfer/batches/out-batch-no/{}/details/out-detail-no/{}",
            out_batch_no.as_ref(),
            out_detail_no.as_ref()
        );
        self.get_v3(credentials, &path, Vec::new()).await
    }

    pub async fn trade_bill(
        &self,
        credentials: &PaymentCredentials,
        request: BillRequest,
    ) -> Result<BillResponse> {
        self.get_v3(credentials, "/v3/bill/tradebill", request.into_query())
            .await
    }

    pub async fn fund_flow_bill(
        &self,
        credentials: &PaymentCredentials,
        request: BillRequest,
    ) -> Result<BillResponse> {
        self.get_v3(credentials, "/v3/bill/fundflowbill", request.into_query())
            .await
    }

    async fn post_v3<R>(
        &self,
        credentials: &PaymentCredentials,
        path: &str,
        body: Value,
    ) -> Result<R>
    where
        R: serde::de::DeserializeOwned,
    {
        let body_text = body.to_string();
        let headers = vec![(
            "authorization".to_string(),
            credentials.authorization("POST", path, &body_text)?,
        )];
        self.inner.post_json(path, None, body, headers).await
    }

    async fn get_v3<R>(
        &self,
        credentials: &PaymentCredentials,
        path: &str,
        query: Vec<(String, String)>,
    ) -> Result<R>
    where
        R: serde::de::DeserializeOwned,
    {
        let path_query = if query.is_empty() {
            path.to_string()
        } else {
            let query_text = query
                .iter()
                .map(|(key, value)| format!("{key}={value}"))
                .collect::<Vec<_>>()
                .join("&");
            format!("{path}?{query_text}")
        };
        let headers = vec![(
            "authorization".to_string(),
            credentials.authorization("GET", &path_query, "")?,
        )];
        self.inner.get_with_headers(path, query, headers).await
    }
}

#[derive(Debug, Clone)]
pub struct PaymentCredentials {
    pub mch_id: String,
    pub serial_no: String,
    pub private_key_pem: String,
}

impl PaymentCredentials {
    pub fn authorization(&self, method: &str, path_query: &str, body: &str) -> Result<String> {
        let timestamp = chrono::Utc::now().timestamp();
        let nonce = crypto::nonce_string(32);
        let message = crypto::payment_v3_message(method, path_query, timestamp, &nonce, body);
        let signature = crypto::rsa_sha256_sign_base64(&self.private_key_pem, message.as_bytes())?;
        Ok(format!(
            "WECHATPAY2-SHA256-RSA2048 mchid=\"{}\",nonce_str=\"{}\",signature=\"{}\",timestamp=\"{}\",serial_no=\"{}\"",
            self.mch_id, nonce, signature, timestamp, self.serial_no
        ))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsapiPrepayRequest {
    pub appid: String,
    pub mchid: String,
    pub description: String,
    pub out_trade_no: String,
    pub notify_url: String,
    pub amount: Amount,
    pub payer: Payer,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_expire: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_info: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppPrepayRequest {
    pub appid: String,
    pub mchid: String,
    pub description: String,
    pub out_trade_no: String,
    pub notify_url: String,
    pub amount: Amount,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_expire: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_info: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct H5PrepayRequest {
    pub appid: String,
    pub mchid: String,
    pub description: String,
    pub out_trade_no: String,
    pub notify_url: String,
    pub amount: Amount,
    pub scene_info: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_expire: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativePrepayRequest {
    pub appid: String,
    pub mchid: String,
    pub description: String,
    pub out_trade_no: String,
    pub notify_url: String,
    pub amount: Amount,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_expire: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_info: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerJsapiPrepayRequest {
    pub sp_appid: String,
    pub sp_mchid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_appid: Option<String>,
    pub sub_mchid: String,
    pub description: String,
    pub out_trade_no: String,
    pub notify_url: String,
    pub amount: Amount,
    pub payer: PartnerPayer,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_expire: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_info: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_info: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerAppPrepayRequest {
    pub sp_appid: String,
    pub sp_mchid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_appid: Option<String>,
    pub sub_mchid: String,
    pub description: String,
    pub out_trade_no: String,
    pub notify_url: String,
    pub amount: Amount,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_expire: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_info: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_info: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerH5PrepayRequest {
    pub sp_appid: String,
    pub sp_mchid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_appid: Option<String>,
    pub sub_mchid: String,
    pub description: String,
    pub out_trade_no: String,
    pub notify_url: String,
    pub amount: Amount,
    pub scene_info: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_expire: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_info: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerNativePrepayRequest {
    pub sp_appid: String,
    pub sp_mchid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_appid: Option<String>,
    pub sub_mchid: String,
    pub description: String,
    pub out_trade_no: String,
    pub notify_url: String,
    pub amount: Amount,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_expire: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_info: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_info: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerPayer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sp_openid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_openid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerOrderQuery {
    pub out_trade_no: String,
    pub sp_mchid: String,
    pub sub_mchid: String,
}

impl PartnerOrderQuery {
    fn into_query(self) -> Vec<(String, String)> {
        vec![
            ("sp_mchid".to_string(), self.sp_mchid),
            ("sub_mchid".to_string(), self.sub_mchid),
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerCloseOrderRequest {
    pub out_trade_no: String,
    pub sp_mchid: String,
    pub sub_mchid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReverseOrderRequest {
    pub out_trade_no: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mchid: Option<String>,
}

impl ReverseOrderRequest {
    fn into_body(self, default_mch_id: &str) -> Value {
        serde_json::json!({
            "mchid": self.mchid.unwrap_or_else(|| default_mch_id.to_string()),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Amount {
    pub total: i64,
    #[serde(default = "default_cny")]
    pub currency: String,
}

fn default_cny() -> String {
    "CNY".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payer {
    pub openid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrepayResponse {
    pub prepay_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsapiPayParams {
    #[serde(rename = "appId")]
    pub app_id: String,
    #[serde(rename = "timeStamp")]
    pub time_stamp: String,
    #[serde(rename = "nonceStr")]
    pub nonce_str: String,
    pub package: String,
    #[serde(rename = "signType")]
    pub sign_type: String,
    #[serde(rename = "paySign")]
    pub pay_sign: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppPayParams {
    pub app_id: String,
    pub partner_id: String,
    pub prepay_id: String,
    pub package: String,
    pub nonce_str: String,
    pub timestamp: String,
    pub sign: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct H5PrepayResponse {
    pub h5_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativePrepayResponse {
    pub code_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentOrderResponse {
    #[serde(flatten)]
    pub value: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentStatusResponse {
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_trade_no: Option<String>,
    pub out_refund_no: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_url: Option<String>,
    pub amount: RefundAmount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundAmount {
    pub refund: i64,
    pub total: i64,
    #[serde(default = "default_cny")]
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundResponse {
    #[serde(flatten)]
    pub value: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfitSharingReceiverRequest {
    pub appid: String,
    #[serde(rename = "type")]
    pub receiver_type: String,
    pub account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_relation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfitSharingOrderRequest {
    pub appid: String,
    pub transaction_id: String,
    pub out_order_no: String,
    pub receivers: Vec<ProfitSharingReceiver>,
    #[serde(default)]
    pub unfreeze_unsplit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfitSharingReceiver {
    #[serde(rename = "type")]
    pub receiver_type: String,
    pub account: String,
    pub amount: i64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfitSharingResponse {
    #[serde(flatten)]
    pub value: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferBatchRequest {
    pub appid: String,
    pub out_batch_no: String,
    pub batch_name: String,
    pub batch_remark: String,
    pub total_amount: i64,
    pub total_num: i64,
    pub transfer_detail_list: Vec<TransferDetailInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_scene_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferDetailInput {
    pub out_detail_no: String,
    pub transfer_amount: i64,
    pub transfer_remark: String,
    pub openid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferBatchQuery {
    pub out_batch_no: String,
    #[serde(default)]
    pub need_query_detail: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_status: Option<String>,
}

impl TransferBatchQuery {
    fn into_query(self) -> Vec<(String, String)> {
        let mut query = vec![(
            "need_query_detail".to_string(),
            self.need_query_detail.to_string(),
        )];
        if let Some(offset) = self.offset {
            query.push(("offset".to_string(), offset.to_string()));
        }
        if let Some(limit) = self.limit {
            query.push(("limit".to_string(), limit.to_string()));
        }
        if let Some(detail_status) = self.detail_status {
            query.push(("detail_status".to_string(), detail_status));
        }
        query
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferBatchResponse {
    #[serde(flatten)]
    pub value: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferDetailResponse {
    #[serde(flatten)]
    pub value: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillRequest {
    pub bill_date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tar_type: Option<String>,
}

impl BillRequest {
    fn into_query(self) -> Vec<(String, String)> {
        let mut query = vec![("bill_date".to_string(), self.bill_date)];
        if let Some(bill_type) = self.bill_type {
            query.push(("bill_type".to_string(), bill_type));
        }
        if let Some(tar_type) = self.tar_type {
            query.push(("tar_type".to_string(), tar_type));
        }
        query
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillResponse {
    pub download_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundAppTransferBillRequest {
    pub appid: String,
    pub out_bill_no: String,
    pub transfer_scene_id: String,
    pub openid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    pub transfer_amount: i64,
    pub transfer_remark: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_recv_perception: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_scene_report_infos: Option<Vec<TransferSceneReportInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferSceneReportInfo {
    pub info_type: String,
    pub info_content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundAppTransferBillResponse {
    pub out_bill_no: String,
    pub transfer_bill_no: String,
    pub create_time: String,
    pub state: String,
    #[serde(default)]
    pub fail_reason: Option<String>,
    #[serde(default)]
    pub package_info: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundAppTransferBillDetailResponse {
    #[serde(flatten)]
    pub value: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundAppTransferBillCancelResponse {
    pub out_bill_no: String,
    pub transfer_bill_no: String,
    pub state: String,
    pub update_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundAppElecSignApplyResponse {
    pub state: String,
    pub create_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundAppElecSignResponse {
    pub state: String,
    pub create_time: String,
    pub update_time: String,
    pub hash_type: String,
    pub hash_value: String,
    pub download_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxCardTemplateRequest {
    pub card_appid: String,
    pub card_template_information: TaxCardTemplateInformation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxCardTemplateInformation {
    pub payee_name: String,
    pub logo_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_cell: Option<TaxCustomCell>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxCustomCell {
    pub words: String,
    pub description: String,
    pub jump_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miniprogram_user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miniprogram_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxCardTemplateResponse {
    pub card_appid: String,
    pub card_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplaintListRequest {
    pub begin_date: String,
    pub end_date: String,
    pub limit: i64,
    pub offset: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complainted_mchid: Option<String>,
}

impl ComplaintListRequest {
    fn into_query(self) -> Vec<(String, String)> {
        let mut query = vec![
            ("begin_date".to_string(), self.begin_date),
            ("end_date".to_string(), self.end_date),
            ("limit".to_string(), self.limit.to_string()),
            ("offset".to_string(), self.offset.to_string()),
        ];
        if let Some(complainted_mchid) = self.complainted_mchid {
            query.push(("complainted_mchid".to_string(), complainted_mchid));
        }
        query
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplaintListResponse {
    #[serde(flatten)]
    pub value: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplaintDetailResponse {
    #[serde(flatten)]
    pub value: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayScoreServiceOrderRequest {
    pub appid: String,
    pub service_id: String,
    pub out_order_no: String,
    pub service_introduction: String,
    pub time_range: PayScoreTimeRange,
    pub risk_fund: PayScoreRiskFund,
    pub notify_url: String,
    pub openid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_user_confirm: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_payments: Option<Vec<PayScorePostPayment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_discounts: Option<Vec<PayScorePostDiscount>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayScoreTimeRange {
    pub start_time: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time_remark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time_remark: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayScoreRiskFund {
    pub name: String,
    pub amount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayScorePostPayment {
    pub name: String,
    pub amount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayScorePostDiscount {
    pub name: String,
    pub amount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayScoreServiceOrderQuery {
    pub out_order_no: String,
    pub appid: String,
    pub service_id: String,
}

impl PayScoreServiceOrderQuery {
    fn into_query(self) -> Vec<(String, String)> {
        vec![
            ("out_order_no".to_string(), self.out_order_no),
            ("appid".to_string(), self.appid),
            ("service_id".to_string(), self.service_id),
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayScoreServiceOrderResponse {
    #[serde(flatten)]
    pub value: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateListResponse {
    pub data: Vec<WechatPayCertificate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatPayCertificate {
    pub serial_no: String,
    pub effective_time: String,
    pub expire_time: String,
    pub encrypt_certificate: WechatPayEncryptedCertificate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatPayEncryptedCertificate {
    pub algorithm: String,
    pub nonce: String,
    pub associated_data: String,
    pub ciphertext: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentNotification {
    pub id: String,
    pub create_time: String,
    pub event_type: String,
    pub resource_type: String,
    pub resource: PaymentResource,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentResource {
    pub algorithm: String,
    pub ciphertext: String,
    pub nonce: String,
    #[serde(default)]
    pub associated_data: String,
}

impl PaymentNotification {
    pub fn decrypt_resource<T>(&self, api_v3_key: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let plaintext = crypto::payment_v3_decrypt(
            api_v3_key,
            &self.resource.nonce,
            &self.resource.associated_data,
            &self.resource.ciphertext,
        )?;
        Ok(serde_json::from_slice(&plaintext)?)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::crypto;

    use super::{
        Amount, AppPayParams, BillRequest, CertificateListResponse, ComplaintListRequest,
        FundAppElecSignResponse, FundAppTransferBillRequest, FundAppTransferBillResponse,
        JsapiPayParams, NativePrepayRequest, PartnerCloseOrderRequest, PartnerH5PrepayRequest,
        PartnerJsapiPrepayRequest, PartnerOrderQuery, PartnerPayer, PayScoreRiskFund,
        PayScoreServiceOrderQuery, PayScoreServiceOrderRequest, PayScoreTimeRange,
        PaymentNotification, PaymentResource, ProfitSharingOrderRequest, ProfitSharingReceiver,
        ProfitSharingReceiverRequest, RefundAmount, RefundRequest, ReverseOrderRequest,
        TaxCardTemplateInformation, TaxCardTemplateRequest, TaxCustomCell, TransferBatchQuery,
        TransferBatchRequest, TransferDetailInput, TransferSceneReportInfo,
    };

    #[test]
    fn decrypts_payment_notification_resource() {
        let key = "0123456789abcdef0123456789abcdef";
        let nonce = "nonce-123456";
        let aad = "transaction";
        let ciphertext =
            crypto::payment_v3_encrypt_for_test(key, nonce, aad, br#"{"trade_state":"SUCCESS"}"#)
                .unwrap();
        let notification = PaymentNotification {
            id: "id".to_string(),
            create_time: "2026-07-04T00:00:00+08:00".to_string(),
            event_type: "TRANSACTION.SUCCESS".to_string(),
            resource_type: "encrypt-resource".to_string(),
            resource: PaymentResource {
                algorithm: "AEAD_AES_256_GCM".to_string(),
                ciphertext,
                nonce: nonce.to_string(),
                associated_data: aad.to_string(),
            },
            summary: "success".to_string(),
        };

        let value: serde_json::Value = notification.decrypt_resource(key).unwrap();
        assert_eq!(value, json!({ "trade_state": "SUCCESS" }));
    }

    #[test]
    fn serializes_native_transaction_request() {
        let value = serde_json::to_value(NativePrepayRequest {
            appid: "appid".to_string(),
            mchid: "mchid".to_string(),
            description: "desc".to_string(),
            out_trade_no: "out".to_string(),
            notify_url: "https://example.com/notify".to_string(),
            amount: Amount {
                total: 100,
                currency: "CNY".to_string(),
            },
            attach: None,
            time_expire: None,
            goods_tag: None,
            detail: None,
            scene_info: None,
        })
        .unwrap();

        assert_eq!(value["amount"]["total"], 100);
        assert_eq!(value["out_trade_no"], "out");
    }

    #[test]
    fn serializes_jsapi_pay_params_wire_names() {
        let value = serde_json::to_value(JsapiPayParams {
            app_id: "appid".to_string(),
            time_stamp: "1".to_string(),
            nonce_str: "nonce".to_string(),
            package: "prepay_id=prepay".to_string(),
            sign_type: "RSA".to_string(),
            pay_sign: "sig".to_string(),
        })
        .unwrap();

        assert_eq!(value["appId"], "appid");
        assert_eq!(value["timeStamp"], "1");
        assert_eq!(value["nonceStr"], "nonce");
        assert_eq!(value["signType"], "RSA");
        assert_eq!(value["paySign"], "sig");
    }

    #[test]
    fn serializes_app_pay_params() {
        let value = serde_json::to_value(AppPayParams {
            app_id: "appid".to_string(),
            partner_id: "mchid".to_string(),
            prepay_id: "prepay".to_string(),
            package: "Sign=WXPay".to_string(),
            nonce_str: "nonce".to_string(),
            timestamp: "1".to_string(),
            sign: "sig".to_string(),
        })
        .unwrap();

        assert_eq!(value["partner_id"], "mchid");
        assert_eq!(value["package"], "Sign=WXPay");
    }

    #[test]
    fn serializes_partner_jsapi_transaction_request() {
        let value = serde_json::to_value(PartnerJsapiPrepayRequest {
            sp_appid: "sp_appid".to_string(),
            sp_mchid: "sp_mchid".to_string(),
            sub_appid: Some("sub_appid".to_string()),
            sub_mchid: "sub_mchid".to_string(),
            description: "desc".to_string(),
            out_trade_no: "out".to_string(),
            notify_url: "https://example.com/notify".to_string(),
            amount: Amount {
                total: 100,
                currency: "CNY".to_string(),
            },
            payer: PartnerPayer {
                sp_openid: None,
                sub_openid: Some("sub_openid".to_string()),
            },
            attach: None,
            time_expire: None,
            goods_tag: None,
            detail: None,
            scene_info: None,
            settle_info: Some(json!({ "profit_sharing": true })),
        })
        .unwrap();

        assert_eq!(value["sp_appid"], "sp_appid");
        assert_eq!(value["sub_mchid"], "sub_mchid");
        assert_eq!(value["payer"]["sub_openid"], "sub_openid");
        assert_eq!(value["settle_info"]["profit_sharing"], true);
        assert!(value.get("attach").is_none());
    }

    #[test]
    fn serializes_partner_h5_transaction_request() {
        let value = serde_json::to_value(PartnerH5PrepayRequest {
            sp_appid: "sp_appid".to_string(),
            sp_mchid: "sp_mchid".to_string(),
            sub_appid: None,
            sub_mchid: "sub_mchid".to_string(),
            description: "desc".to_string(),
            out_trade_no: "out".to_string(),
            notify_url: "https://example.com/notify".to_string(),
            amount: Amount {
                total: 100,
                currency: "CNY".to_string(),
            },
            scene_info: json!({ "payer_client_ip": "127.0.0.1" }),
            attach: None,
            time_expire: None,
            goods_tag: None,
            detail: None,
            settle_info: None,
        })
        .unwrap();

        assert_eq!(value["sp_mchid"], "sp_mchid");
        assert_eq!(value["scene_info"]["payer_client_ip"], "127.0.0.1");
        assert!(value.get("sub_appid").is_none());
    }

    #[test]
    fn builds_partner_order_query() {
        let query = PartnerOrderQuery {
            out_trade_no: "out".to_string(),
            sp_mchid: "sp_mchid".to_string(),
            sub_mchid: "sub_mchid".to_string(),
        }
        .into_query();

        assert_eq!(
            query,
            vec![
                ("sp_mchid".to_string(), "sp_mchid".to_string()),
                ("sub_mchid".to_string(), "sub_mchid".to_string())
            ]
        );
    }

    #[test]
    fn serializes_partner_close_order_request() {
        let value = serde_json::to_value(PartnerCloseOrderRequest {
            out_trade_no: "out".to_string(),
            sp_mchid: "sp_mchid".to_string(),
            sub_mchid: "sub_mchid".to_string(),
        })
        .unwrap();

        assert_eq!(value["out_trade_no"], "out");
        assert_eq!(value["sp_mchid"], "sp_mchid");
        assert_eq!(value["sub_mchid"], "sub_mchid");
    }

    #[test]
    fn serializes_reverse_order_request() {
        let value = serde_json::to_value(ReverseOrderRequest {
            out_trade_no: "out".to_string(),
            mchid: Some("mchid".to_string()),
        })
        .unwrap();

        assert_eq!(value["out_trade_no"], "out");
        assert_eq!(value["mchid"], "mchid");
    }

    #[test]
    fn builds_reverse_order_body_with_default_mchid() {
        let body = ReverseOrderRequest {
            out_trade_no: "out".to_string(),
            mchid: None,
        }
        .into_body("default_mchid");

        assert_eq!(body, json!({ "mchid": "default_mchid" }));
    }

    #[test]
    fn builds_bill_query() {
        let query = BillRequest {
            bill_date: "2026-07-04".to_string(),
            bill_type: Some("ALL".to_string()),
            tar_type: None,
        }
        .into_query();

        assert_eq!(
            query,
            vec![
                ("bill_date".to_string(), "2026-07-04".to_string()),
                ("bill_type".to_string(), "ALL".to_string())
            ]
        );
    }

    #[test]
    fn serializes_refund_request() {
        let value = serde_json::to_value(RefundRequest {
            transaction_id: None,
            out_trade_no: Some("order-1".to_string()),
            out_refund_no: "refund-1".to_string(),
            reason: Some("user requested".to_string()),
            notify_url: Some("https://example.com/pay/refund".to_string()),
            amount: RefundAmount {
                refund: 50,
                total: 100,
                currency: "CNY".to_string(),
            },
        })
        .unwrap();

        assert!(value.get("transaction_id").is_none());
        assert_eq!(value["out_trade_no"], "order-1");
        assert_eq!(value["out_refund_no"], "refund-1");
        assert_eq!(value["amount"]["refund"], 50);
        assert_eq!(value["amount"]["total"], 100);
        assert_eq!(value["amount"]["currency"], "CNY");
    }

    #[test]
    fn serializes_profit_sharing_receiver_request() {
        let value = serde_json::to_value(ProfitSharingReceiverRequest {
            appid: "wxappid".to_string(),
            receiver_type: "PERSONAL_OPENID".to_string(),
            account: "openid".to_string(),
            name: None,
            relation_type: Some("PARTNER".to_string()),
            custom_relation: None,
        })
        .unwrap();

        assert_eq!(value["appid"], "wxappid");
        assert_eq!(value["type"], "PERSONAL_OPENID");
        assert_eq!(value["account"], "openid");
        assert_eq!(value["relation_type"], "PARTNER");
        assert!(value.get("name").is_none());
    }

    #[test]
    fn serializes_profit_sharing_order_request() {
        let value = serde_json::to_value(ProfitSharingOrderRequest {
            appid: "wxappid".to_string(),
            transaction_id: "4200000000".to_string(),
            out_order_no: "share-1".to_string(),
            receivers: vec![ProfitSharingReceiver {
                receiver_type: "PERSONAL_OPENID".to_string(),
                account: "openid".to_string(),
                amount: 30,
                description: "commission".to_string(),
            }],
            unfreeze_unsplit: true,
        })
        .unwrap();

        assert_eq!(value["transaction_id"], "4200000000");
        assert_eq!(value["out_order_no"], "share-1");
        assert_eq!(value["receivers"][0]["type"], "PERSONAL_OPENID");
        assert_eq!(value["receivers"][0]["amount"], 30);
        assert_eq!(value["unfreeze_unsplit"], true);
    }

    #[test]
    fn serializes_transfer_batch_request() {
        let value = serde_json::to_value(TransferBatchRequest {
            appid: "wxappid".to_string(),
            out_batch_no: "batch-1".to_string(),
            batch_name: "payroll".to_string(),
            batch_remark: "July".to_string(),
            total_amount: 100,
            total_num: 1,
            transfer_detail_list: vec![TransferDetailInput {
                out_detail_no: "detail-1".to_string(),
                transfer_amount: 100,
                transfer_remark: "bonus".to_string(),
                openid: "openid".to_string(),
                user_name: None,
            }],
            transfer_scene_id: Some("1000".to_string()),
            notify_url: None,
        })
        .unwrap();

        assert_eq!(value["out_batch_no"], "batch-1");
        assert_eq!(value["transfer_detail_list"][0]["openid"], "openid");
        assert_eq!(value["transfer_scene_id"], "1000");
        assert!(value.get("notify_url").is_none());
    }

    #[test]
    fn builds_transfer_batch_query() {
        let query = TransferBatchQuery {
            out_batch_no: "batch-1".to_string(),
            need_query_detail: true,
            offset: Some(0),
            limit: Some(20),
            detail_status: Some("SUCCESS".to_string()),
        }
        .into_query();

        assert_eq!(
            query,
            vec![
                ("need_query_detail".to_string(), "true".to_string()),
                ("offset".to_string(), "0".to_string()),
                ("limit".to_string(), "20".to_string()),
                ("detail_status".to_string(), "SUCCESS".to_string())
            ]
        );
    }

    #[test]
    fn builds_complaint_list_query() {
        let query = ComplaintListRequest {
            begin_date: "2026-07-01".to_string(),
            end_date: "2026-07-06".to_string(),
            limit: 20,
            offset: 0,
            complainted_mchid: Some("mchid".to_string()),
        }
        .into_query();

        assert_eq!(
            query,
            vec![
                ("begin_date".to_string(), "2026-07-01".to_string()),
                ("end_date".to_string(), "2026-07-06".to_string()),
                ("limit".to_string(), "20".to_string()),
                ("offset".to_string(), "0".to_string()),
                ("complainted_mchid".to_string(), "mchid".to_string())
            ]
        );
    }

    #[test]
    fn serializes_pay_score_service_order_request() {
        let value = serde_json::to_value(PayScoreServiceOrderRequest {
            appid: "wxappid".to_string(),
            service_id: "service-id".to_string(),
            out_order_no: "score-order-1".to_string(),
            service_introduction: "rental".to_string(),
            time_range: PayScoreTimeRange {
                start_time: "2026-07-06T00:00:00+08:00".to_string(),
                end_time: None,
                start_time_remark: Some("start".to_string()),
                end_time_remark: None,
            },
            risk_fund: PayScoreRiskFund {
                name: "DEPOSIT".to_string(),
                amount: 100,
                description: None,
            },
            notify_url: "https://example.com/pay-score".to_string(),
            openid: "openid".to_string(),
            need_user_confirm: Some(true),
            post_payments: None,
            post_discounts: None,
            location: None,
            attach: None,
        })
        .unwrap();

        assert_eq!(value["appid"], "wxappid");
        assert_eq!(value["service_id"], "service-id");
        assert_eq!(value["out_order_no"], "score-order-1");
        assert_eq!(value["time_range"]["start_time_remark"], "start");
        assert_eq!(value["risk_fund"]["amount"], 100);
        assert_eq!(value["need_user_confirm"], true);
        assert!(value.get("attach").is_none());
    }

    #[test]
    fn builds_pay_score_service_order_query() {
        let query = PayScoreServiceOrderQuery {
            out_order_no: "score-order-1".to_string(),
            appid: "wxappid".to_string(),
            service_id: "service-id".to_string(),
        }
        .into_query();

        assert_eq!(
            query,
            vec![
                ("out_order_no".to_string(), "score-order-1".to_string()),
                ("appid".to_string(), "wxappid".to_string()),
                ("service_id".to_string(), "service-id".to_string())
            ]
        );
    }

    #[test]
    fn deserializes_certificate_list_response() {
        let response: CertificateListResponse = serde_json::from_value(json!({
            "data": [{
                "serial_no": "serial",
                "effective_time": "2026-07-06T00:00:00+08:00",
                "expire_time": "2027-07-06T00:00:00+08:00",
                "encrypt_certificate": {
                    "algorithm": "AEAD_AES_256_GCM",
                    "nonce": "nonce",
                    "associated_data": "certificate",
                    "ciphertext": "ciphertext"
                }
            }]
        }))
        .unwrap();

        assert_eq!(response.data[0].serial_no, "serial");
        assert_eq!(
            response.data[0].encrypt_certificate.algorithm,
            "AEAD_AES_256_GCM"
        );
    }

    #[test]
    fn serializes_fund_app_transfer_bill_request() {
        let value = serde_json::to_value(FundAppTransferBillRequest {
            appid: "wxappid".to_string(),
            out_bill_no: "bill-1".to_string(),
            transfer_scene_id: "1000".to_string(),
            openid: "openid".to_string(),
            user_name: None,
            transfer_amount: 100,
            transfer_remark: "bonus".to_string(),
            notify_url: Some("https://example.com/fund-app".to_string()),
            user_recv_perception: Some("cash reward".to_string()),
            transfer_scene_report_infos: Some(vec![TransferSceneReportInfo {
                info_type: "activity_name".to_string(),
                info_content: "July campaign".to_string(),
            }]),
        })
        .unwrap();

        assert_eq!(value["appid"], "wxappid");
        assert_eq!(value["out_bill_no"], "bill-1");
        assert_eq!(value["transfer_amount"], 100);
        assert_eq!(value["notify_url"], "https://example.com/fund-app");
        assert_eq!(
            value["transfer_scene_report_infos"][0]["info_content"],
            "July campaign"
        );
        assert!(value.get("user_name").is_none());
    }

    #[test]
    fn deserializes_fund_app_transfer_bill_response() {
        let response: FundAppTransferBillResponse = serde_json::from_value(json!({
            "out_bill_no": "bill-1",
            "transfer_bill_no": "transfer-1",
            "create_time": "2026-07-06T00:00:00+08:00",
            "state": "ACCEPTED",
            "package_info": "package"
        }))
        .unwrap();

        assert_eq!(response.out_bill_no, "bill-1");
        assert_eq!(response.transfer_bill_no, "transfer-1");
        assert_eq!(response.state, "ACCEPTED");
        assert_eq!(response.package_info.as_deref(), Some("package"));
    }

    #[test]
    fn deserializes_fund_app_elec_sign_response() {
        let response: FundAppElecSignResponse = serde_json::from_value(json!({
            "state": "FINISHED",
            "create_time": "2026-07-06T00:00:00+08:00",
            "update_time": "2026-07-06T00:01:00+08:00",
            "hash_type": "SHA256",
            "hash_value": "hash",
            "download_url": "https://example.com/sign.pdf"
        }))
        .unwrap();

        assert_eq!(response.state, "FINISHED");
        assert_eq!(response.hash_type, "SHA256");
        assert_eq!(response.download_url, "https://example.com/sign.pdf");
    }

    #[test]
    fn serializes_tax_card_template_request() {
        let value = serde_json::to_value(TaxCardTemplateRequest {
            card_appid: "wxappid".to_string(),
            card_template_information: TaxCardTemplateInformation {
                payee_name: "merchant".to_string(),
                logo_url: "https://example.com/logo.png".to_string(),
                custom_cell: Some(TaxCustomCell {
                    words: "invoice".to_string(),
                    description: "view invoice".to_string(),
                    jump_url: "https://example.com/invoice".to_string(),
                    miniprogram_user_name: None,
                    miniprogram_path: Some("pages/invoice".to_string()),
                }),
            },
        })
        .unwrap();

        assert_eq!(value["card_appid"], "wxappid");
        assert_eq!(value["card_template_information"]["payee_name"], "merchant");
        assert_eq!(
            value["card_template_information"]["custom_cell"]["miniprogram_path"],
            "pages/invoice"
        );
        assert!(value["card_template_information"]["custom_cell"]
            .get("miniprogram_user_name")
            .is_none());
    }
}
