use bytes::Bytes;
use serde::{Deserialize, Serialize};
use serde_json::{to_value, Value};
use sha1::Digest as _;
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};
use tokio::io::AsyncWriteExt;

use crate::{
    config::Platform,
    crypto,
    error::{Result, WechatError},
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

    pub async fn apply_fund_app_elec_sign_by_transfer_bill_no(
        &self,
        credentials: &PaymentCredentials,
        transfer_bill_no: impl Into<String>,
    ) -> Result<FundAppElecSignApplyResponse> {
        self.post_v3(
            credentials,
            "/v3/fund-app/mch-transfer/elecsign/transfer-bill-no",
            serde_json::json!({ "transfer_bill_no": transfer_bill_no.into() }),
        )
        .await
    }

    pub async fn query_fund_app_elec_sign_by_transfer_bill_no(
        &self,
        credentials: &PaymentCredentials,
        transfer_bill_no: impl AsRef<str>,
    ) -> Result<FundAppElecSignResponse> {
        let path = format!(
            "/v3/fund-app/mch-transfer/elecsign/transfer-bill-no/{}",
            transfer_bill_no.as_ref()
        );
        self.get_v3(credentials, &path, Vec::new()).await
    }

    pub fn jssdk(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "payment.jssdk")
    }

    pub fn base(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "payment.base")
    }

    pub async fn micropay(
        &self,
        credentials: &PaymentCredentials,
        request: MicropayRequest,
    ) -> Result<PaymentOrderResponse> {
        self.post_v3(credentials, "/v3/pay/micropay", to_value(request)?)
            .await
    }

    pub fn apply4_sub(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "payment.apply4_sub")
    }

    pub async fn create_applyment4_sub(
        &self,
        credentials: &PaymentCredentials,
        request: Applyment4SubRequest,
    ) -> Result<Applyment4SubResponse> {
        self.post_v3(
            credentials,
            "/v3/applyment4sub/applyment/",
            to_value(request)?,
        )
        .await
    }

    pub async fn query_applyment4_sub_by_business_code(
        &self,
        credentials: &PaymentCredentials,
        business_code: impl AsRef<str>,
    ) -> Result<Applyment4SubQueryResponse> {
        let path = format!(
            "/v3/applyment4sub/applyment/business_code/{}",
            business_code.as_ref()
        );
        self.get_v3(credentials, &path, Vec::new()).await
    }

    pub async fn query_applyment4_sub_by_applyment_id(
        &self,
        credentials: &PaymentCredentials,
        applyment_id: impl AsRef<str>,
    ) -> Result<Applyment4SubQueryResponse> {
        let path = format!(
            "/v3/applyment4sub/applyment/applyment_id/{}",
            applyment_id.as_ref()
        );
        self.get_v3(credentials, &path, Vec::new()).await
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

    pub async fn combine_app_transaction(
        &self,
        credentials: &PaymentCredentials,
        request: CombineAppPrepayRequest,
    ) -> Result<PrepayResponse> {
        self.post_v3(
            credentials,
            "/v3/combine-transactions/app",
            to_value(request)?,
        )
        .await
    }

    pub async fn codepay_transaction(
        &self,
        credentials: &PaymentCredentials,
        request: CodepayRequest,
    ) -> Result<PaymentOrderResponse> {
        self.post_v3(
            credentials,
            "/v3/pay/transactions/codepay",
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

    pub async fn partner_query_by_transaction_id(
        &self,
        credentials: &PaymentCredentials,
        request: PartnerTransactionQuery,
    ) -> Result<PaymentOrderResponse> {
        let path = format!("/v3/pay/partner/transactions/id/{}", request.transaction_id);
        self.get_v3(credentials, &path, request.into_query()).await
    }

    pub async fn partner_query_refund_by_out_refund_no(
        &self,
        credentials: &PaymentCredentials,
        request: PartnerRefundQuery,
    ) -> Result<PartnerRefundDetailResponse> {
        let path = format!("/v3/refund/domestic/refunds/{}", request.out_refund_no);
        self.get_v3(credentials, &path, request.into_query()).await
    }

    pub async fn partner_combine_app_transaction(
        &self,
        credentials: &PaymentCredentials,
        request: CombineAppPrepayRequest,
    ) -> Result<PrepayResponse> {
        self.combine_app_transaction(credentials, request).await
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

    pub fn promotion(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "payment.promotion")
    }

    pub async fn create_coupon_stock(
        &self,
        credentials: &PaymentCredentials,
        request: CouponStockCreateRequest,
    ) -> Result<CouponStockResponse> {
        self.post_v3(
            credentials,
            "/v3/marketing/favor/stocks",
            to_value(request)?,
        )
        .await
    }

    pub async fn start_coupon_stock(
        &self,
        credentials: &PaymentCredentials,
        stock_id: impl AsRef<str>,
        request: CouponStockOperationRequest,
    ) -> Result<CouponStockOperationResponse> {
        let path = format!("/v3/marketing/favor/stocks/{}/start", stock_id.as_ref());
        self.post_v3(credentials, &path, to_value(request)?).await
    }

    pub async fn pause_coupon_stock(
        &self,
        credentials: &PaymentCredentials,
        stock_id: impl AsRef<str>,
        request: CouponStockOperationRequest,
    ) -> Result<CouponStockOperationResponse> {
        let path = format!("/v3/marketing/favor/stocks/{}/pause", stock_id.as_ref());
        self.post_v3(credentials, &path, to_value(request)?).await
    }

    pub async fn restart_coupon_stock(
        &self,
        credentials: &PaymentCredentials,
        stock_id: impl AsRef<str>,
        request: CouponStockOperationRequest,
    ) -> Result<CouponStockOperationResponse> {
        let path = format!("/v3/marketing/favor/stocks/{}/restart", stock_id.as_ref());
        self.post_v3(credentials, &path, to_value(request)?).await
    }

    pub async fn query_coupon_stock(
        &self,
        credentials: &PaymentCredentials,
        stock_id: impl AsRef<str>,
        stock_creator_mchid: Option<String>,
    ) -> Result<CouponStockResponse> {
        let path = format!("/v3/marketing/favor/stocks/{}", stock_id.as_ref());
        let mut query = Vec::new();
        push_optional_query(&mut query, "stock_creator_mchid", stock_creator_mchid);
        self.get_v3(credentials, &path, query).await
    }

    pub async fn list_coupon_stocks(
        &self,
        credentials: &PaymentCredentials,
        request: CouponStockListRequest,
    ) -> Result<CouponStockListResponse> {
        self.get_v3(
            credentials,
            "/v3/marketing/favor/stocks",
            request.into_query(),
        )
        .await
    }

    pub async fn send_coupon(
        &self,
        credentials: &PaymentCredentials,
        openid: impl AsRef<str>,
        request: SendCouponRequest,
    ) -> Result<SendCouponResponse> {
        let path = format!("/v3/marketing/favor/users/{}/coupons", openid.as_ref());
        self.post_v3(credentials, &path, to_value(request)?).await
    }

    pub async fn list_user_coupons(
        &self,
        credentials: &PaymentCredentials,
        openid: impl AsRef<str>,
        request: UserCouponListRequest,
    ) -> Result<UserCouponListResponse> {
        let path = format!("/v3/marketing/favor/users/{}/coupons", openid.as_ref());
        self.get_v3(credentials, &path, request.into_query()).await
    }

    pub async fn query_user_coupon(
        &self,
        credentials: &PaymentCredentials,
        openid: impl AsRef<str>,
        coupon_id: impl AsRef<str>,
        appid: Option<String>,
    ) -> Result<UserCouponResponse> {
        let path = format!(
            "/v3/marketing/favor/users/{}/coupons/{}",
            openid.as_ref(),
            coupon_id.as_ref()
        );
        let mut query = Vec::new();
        push_optional_query(&mut query, "appid", appid);
        self.get_v3(credentials, &path, query).await
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

    pub async fn create_profit_sharing_return_order(
        &self,
        credentials: &PaymentCredentials,
        request: ProfitSharingReturnOrderRequest,
    ) -> Result<ProfitSharingResponse> {
        self.post_v3(
            credentials,
            "/v3/profitsharing/return-orders",
            to_value(request)?,
        )
        .await
    }

    pub async fn query_profit_sharing_return_order(
        &self,
        credentials: &PaymentCredentials,
        out_return_no: impl AsRef<str>,
        query: ProfitSharingReturnOrderQuery,
    ) -> Result<ProfitSharingResponse> {
        let path = format!("/v3/profitsharing/return-orders/{}", out_return_no.as_ref());
        self.get_v3(credentials, &path, query.into_query()).await
    }

    pub async fn unfreeze_profit_sharing_order(
        &self,
        credentials: &PaymentCredentials,
        request: ProfitSharingUnfreezeRequest,
    ) -> Result<ProfitSharingResponse> {
        self.post_v3(
            credentials,
            "/v3/profitsharing/orders/unfreeze",
            to_value(request)?,
        )
        .await
    }

    pub async fn query_profit_sharing_transaction_amounts(
        &self,
        credentials: &PaymentCredentials,
        transaction_id: impl AsRef<str>,
    ) -> Result<ProfitSharingResponse> {
        let path = format!(
            "/v3/profitsharing/transactions/{}/amounts",
            transaction_id.as_ref()
        );
        self.get_v3(credentials, &path, Vec::new()).await
    }

    pub async fn profit_sharing_bill(
        &self,
        credentials: &PaymentCredentials,
        request: ProfitSharingBillRequest,
    ) -> Result<BillResponse> {
        request.validate()?;
        let response: BillResponse = self
            .get_v3(credentials, "/v3/profitsharing/bills", request.into_query())
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn download_profit_sharing_bill_bytes(
        &self,
        credentials: &PaymentCredentials,
        request: ProfitSharingBillRequest,
    ) -> Result<Bytes> {
        let bill = self.profit_sharing_bill(credentials, request).await?;
        self.download_bill_bytes(credentials, bill.into()).await
    }

    pub async fn download_profit_sharing_bill(
        &self,
        credentials: &PaymentCredentials,
        request: ProfitSharingBillRequest,
    ) -> Result<PaymentDownloadedBill> {
        let bill = self.profit_sharing_bill(credentials, request).await?;
        self.download_bill(credentials, bill.into()).await
    }

    pub async fn download_profit_sharing_bill_to_file(
        &self,
        credentials: &PaymentCredentials,
        request: ProfitSharingBillRequest,
        destination: impl AsRef<Path>,
    ) -> Result<PaymentDownloadedBillFile> {
        let bill = self.profit_sharing_bill(credentials, request).await?;
        self.download_bill_to_file(credentials, bill.into(), destination)
            .await
    }

    pub async fn legacy_profit_sharing_return(
        &self,
        credentials: &PaymentCredentials,
        api_key: impl AsRef<str>,
        request: LegacyProfitSharingReturnRequest,
    ) -> Result<LegacyProfitSharingReturnResponse> {
        self.post_legacy_xml(
            credentials,
            api_key.as_ref(),
            "/secapi/pay/profitsharingreturn",
            request.into_params(),
        )
        .await
    }

    pub fn refund(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "payment.refund")
    }

    pub fn redpack(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "payment.redpack")
    }

    pub async fn send_redpack(
        &self,
        credentials: &PaymentCredentials,
        api_key: impl AsRef<str>,
        request: SendRedpackRequest,
    ) -> Result<RedpackResponse> {
        self.post_legacy_xml(
            credentials,
            api_key.as_ref(),
            "/mmpaymkttransfers/sendredpack",
            request.into_params(),
        )
        .await
    }

    pub async fn send_group_redpack(
        &self,
        credentials: &PaymentCredentials,
        api_key: impl AsRef<str>,
        request: SendGroupRedpackRequest,
    ) -> Result<RedpackResponse> {
        self.post_legacy_xml(
            credentials,
            api_key.as_ref(),
            "/mmpaymkttransfers/sendgroupredpack",
            request.into_params(),
        )
        .await
    }

    pub async fn query_redpack(
        &self,
        credentials: &PaymentCredentials,
        api_key: impl AsRef<str>,
        request: QueryRedpackRequest,
    ) -> Result<RedpackInfoResponse> {
        self.post_legacy_xml(
            credentials,
            api_key.as_ref(),
            "/mmpaymkttransfers/gethbinfo",
            request.into_params(),
        )
        .await
    }

    pub async fn send_work_redpack(
        &self,
        credentials: &PaymentCredentials,
        api_key: impl AsRef<str>,
        request: WorkRedpackRequest,
    ) -> Result<RedpackResponse> {
        self.post_legacy_xml(
            credentials,
            api_key.as_ref(),
            "/mmpaymkttransfers/sendworkwxredpack",
            request.into_params(),
        )
        .await
    }

    pub async fn query_work_redpack(
        &self,
        credentials: &PaymentCredentials,
        api_key: impl AsRef<str>,
        request: QueryWorkRedpackRequest,
    ) -> Result<RedpackInfoResponse> {
        self.post_legacy_xml(
            credentials,
            api_key.as_ref(),
            "/mmpaymkttransfers/queryworkwxredpack",
            request.into_params(),
        )
        .await
    }

    pub async fn send_mini_program_redpack(
        &self,
        credentials: &PaymentCredentials,
        api_key: impl AsRef<str>,
        request: MiniProgramRedpackRequest,
    ) -> Result<RedpackResponse> {
        self.post_legacy_xml(
            credentials,
            api_key.as_ref(),
            "/mmpaymkttransfers/sendminiprogramhb",
            request.into_params(),
        )
        .await
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

    pub async fn create_refund_detail(
        &self,
        credentials: &PaymentCredentials,
        request: RefundRequest,
    ) -> Result<RefundDetailResponse> {
        self.post_v3(
            credentials,
            "/v3/refund/domestic/refunds",
            to_value(request)?,
        )
        .await
    }

    pub async fn query_refund_detail(
        &self,
        credentials: &PaymentCredentials,
        out_refund_no: impl AsRef<str>,
    ) -> Result<RefundDetailResponse> {
        let path = format!("/v3/refund/domestic/refunds/{}", out_refund_no.as_ref());
        self.get_v3(credentials, &path, Vec::new()).await
    }

    pub fn reverse(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "payment.reverse")
    }

    pub fn transfer(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "payment.transfer")
    }

    pub async fn query_balance_transfer_order(
        &self,
        credentials: &PaymentCredentials,
        api_key: impl AsRef<str>,
        appid: impl Into<String>,
        partner_trade_no: impl Into<String>,
    ) -> Result<LegacyTransferInfoResponse> {
        self.post_legacy_xml(
            credentials,
            api_key.as_ref(),
            "/mmpaymkttransfers/gettransferinfo",
            vec![
                ("appid".to_string(), appid.into()),
                ("partner_trade_no".to_string(), partner_trade_no.into()),
            ],
        )
        .await
    }

    pub async fn transfer_to_balance(
        &self,
        credentials: &PaymentCredentials,
        api_key: impl AsRef<str>,
        request: TransferToBalanceRequest,
    ) -> Result<TransferToBalanceResponse> {
        self.post_legacy_xml_raw(
            api_key.as_ref(),
            "/mmpaymkttransfers/promotion/transfers",
            request.into_params(credentials),
        )
        .await
    }

    pub async fn query_bank_card_transfer_order(
        &self,
        credentials: &PaymentCredentials,
        api_key: impl AsRef<str>,
        partner_trade_no: impl Into<String>,
    ) -> Result<LegacyTransferInfoResponse> {
        self.post_legacy_xml(
            credentials,
            api_key.as_ref(),
            "/mmpaymkttransfers/query_bank",
            vec![("partner_trade_no".to_string(), partner_trade_no.into())],
        )
        .await
    }

    pub fn merchant_service(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "payment.merchant_service")
    }

    pub async fn merchant_fund_balance(
        &self,
        credentials: &PaymentCredentials,
        account_type: impl AsRef<str>,
    ) -> Result<MerchantFundBalanceResponse> {
        let path = format!("/v3/merchant/fund/balance/{}", account_type.as_ref());
        self.get_v3(credentials, &path, Vec::new()).await
    }

    pub fn merchant(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "payment.merchant")
    }

    pub async fn upload_merchant_media(
        &self,
        credentials: &PaymentCredentials,
        request: MerchantMediaUploadRequest,
    ) -> Result<MerchantMediaUploadResponse> {
        let path = "/v3/merchant/media/upload";
        let (content_type, body) = build_merchant_media_upload_body(&request);
        let headers = vec![(
            "authorization".to_string(),
            credentials.authorization_bytes("POST", path, &body)?,
        )];
        self.inner
            .post_raw_json(path, Vec::new(), content_type, body, headers)
            .await
    }

    pub async fn upload_merchant_media_from_bytes(
        &self,
        credentials: &PaymentCredentials,
        file_name: impl Into<String>,
        data: impl Into<Vec<u8>>,
    ) -> Result<MerchantMediaUploadResponse> {
        let data = data.into();
        self.upload_merchant_media(
            credentials,
            MerchantMediaUploadRequest {
                file_name: file_name.into(),
                sha256: crypto::sha256_hex(&data),
                data,
            },
        )
        .await
    }

    pub async fn query_complaints(
        &self,
        credentials: &PaymentCredentials,
        request: ComplaintListRequest,
    ) -> Result<ComplaintListResponse> {
        request.validate()?;
        let response: ComplaintListResponse = self
            .get_v3(
                credentials,
                "/v3/merchant-service/complaints-v2",
                request.into_query(),
            )
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn query_complaint_detail(
        &self,
        credentials: &PaymentCredentials,
        complaint_id: impl AsRef<str>,
    ) -> Result<ComplaintDetailResponse> {
        let complaint_id = payment_path_identifier(complaint_id.as_ref(), "complaint id", 64)?;
        let path = format!(
            "/v3/merchant-service/complaints-v2/{}",
            encode_payment_path_segment(complaint_id)
        );
        let response: ComplaintDetailResponse = self.get_v3(credentials, &path, Vec::new()).await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn query_complaint_negotiation_history(
        &self,
        credentials: &PaymentCredentials,
        complaint_id: impl AsRef<str>,
        request: ComplaintNegotiationHistoryRequest,
    ) -> Result<ComplaintNegotiationHistoryResponse> {
        let complaint_id = payment_path_identifier(complaint_id.as_ref(), "complaint id", 64)?;
        request.validate()?;
        let path = format!(
            "/v3/merchant-service/complaints-v2/{}/negotiation-historys",
            encode_payment_path_segment(complaint_id)
        );
        let response: ComplaintNegotiationHistoryResponse = self
            .get_v3(credentials, &path, request.into_query())
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn create_complaint_notification(
        &self,
        credentials: &PaymentCredentials,
        request: ComplaintNotificationRequest,
    ) -> Result<ComplaintNotificationResponse> {
        request.validate()?;
        let response: ComplaintNotificationResponse = self
            .post_v3(
                credentials,
                "/v3/merchant-service/complaint-notifications",
                to_value(request)?,
            )
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn query_complaint_notification(
        &self,
        credentials: &PaymentCredentials,
    ) -> Result<ComplaintNotificationResponse> {
        let response: ComplaintNotificationResponse = self
            .get_v3(
                credentials,
                "/v3/merchant-service/complaint-notifications",
                Vec::new(),
            )
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn update_complaint_notification(
        &self,
        credentials: &PaymentCredentials,
        request: ComplaintNotificationRequest,
    ) -> Result<ComplaintNotificationResponse> {
        request.validate()?;
        let response: ComplaintNotificationResponse = self
            .put_v3(
                credentials,
                "/v3/merchant-service/complaint-notifications",
                to_value(request)?,
            )
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn delete_complaint_notification(
        &self,
        credentials: &PaymentCredentials,
    ) -> Result<()> {
        self.delete_v3_empty(credentials, "/v3/merchant-service/complaint-notifications")
            .await
    }

    pub async fn reply_to_complaint_user(
        &self,
        credentials: &PaymentCredentials,
        complaint_id: impl AsRef<str>,
        request: ComplaintReplyRequest,
    ) -> Result<()> {
        let complaint_id = payment_path_identifier(complaint_id.as_ref(), "complaint id", 64)?;
        request.validate()?;
        let path = format!(
            "/v3/merchant-service/complaints-v2/{}/response",
            encode_payment_path_segment(complaint_id)
        );
        self.post_v3_empty(credentials, &path, to_value(request)?)
            .await
    }

    pub async fn complete_complaint(
        &self,
        credentials: &PaymentCredentials,
        complaint_id: impl AsRef<str>,
        request: ComplaintCompleteRequest,
    ) -> Result<()> {
        let complaint_id = payment_path_identifier(complaint_id.as_ref(), "complaint id", 64)?;
        request.validate()?;
        let path = format!(
            "/v3/merchant-service/complaints-v2/{}/complete",
            encode_payment_path_segment(complaint_id)
        );
        self.post_v3_empty(credentials, &path, to_value(request)?)
            .await
    }

    pub async fn update_complaint_refund_progress(
        &self,
        credentials: &PaymentCredentials,
        complaint_id: impl AsRef<str>,
        request: ComplaintRefundProgressRequest,
    ) -> Result<()> {
        let complaint_id = payment_path_identifier(complaint_id.as_ref(), "complaint id", 64)?;
        request.validate()?;
        let path = format!(
            "/v3/merchant-service/complaints-v2/{}/update-refund-progress",
            encode_payment_path_segment(complaint_id)
        );
        self.post_v3_empty(credentials, &path, to_value(request)?)
            .await
    }

    pub async fn upload_complaint_image(
        &self,
        credentials: &PaymentCredentials,
        request: MerchantMediaUploadRequest,
    ) -> Result<MerchantMediaUploadResponse> {
        let path = "/v3/merchant-service/images/upload";
        let (content_type, body) = build_merchant_media_upload_body(&request);
        let headers = vec![(
            "authorization".to_string(),
            credentials.authorization_bytes("POST", path, &body)?,
        )];
        self.inner
            .post_raw_json(path, Vec::new(), content_type, body, headers)
            .await
    }

    pub async fn upload_complaint_image_from_bytes(
        &self,
        credentials: &PaymentCredentials,
        file_name: impl Into<String>,
        data: impl Into<Vec<u8>>,
    ) -> Result<MerchantMediaUploadResponse> {
        let data = data.into();
        self.upload_complaint_image(
            credentials,
            MerchantMediaUploadRequest {
                file_name: file_name.into(),
                sha256: crypto::sha256_hex(&data),
                data,
            },
        )
        .await
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

    pub fn sandbox(&self) -> DomainModule {
        DomainModule::new(self.inner.clone(), "payment.sandbox")
    }

    pub async fn get_sandbox_sign_key(
        &self,
        credentials: &PaymentCredentials,
        api_key: impl AsRef<str>,
    ) -> Result<SandboxSignKeyResponse> {
        let body =
            build_sandbox_sign_key_xml(credentials, api_key.as_ref(), &crypto::nonce_string(32));
        self.inner
            .post_xml("/sandboxnew/pay/getsignkey", body)
            .await
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

    pub async fn create_transfer_bill_receipt(
        &self,
        credentials: &PaymentCredentials,
        out_batch_no: impl Into<String>,
    ) -> Result<TransferBillReceiptResponse> {
        self.post_v3(
            credentials,
            "/v3/transfer/bill-receipt",
            serde_json::json!({ "out_batch_no": out_batch_no.into() }),
        )
        .await
    }

    pub async fn query_transfer_bill_receipt(
        &self,
        credentials: &PaymentCredentials,
        out_batch_no: impl AsRef<str>,
    ) -> Result<TransferBillReceiptResponse> {
        let path = format!("/v3/transfer/bill-receipt/{}", out_batch_no.as_ref());
        self.post_v3(credentials, &path, serde_json::json!({}))
            .await
    }

    pub async fn create_transfer_detail_receipt(
        &self,
        credentials: &PaymentCredentials,
        request: TransferDetailReceiptRequest,
    ) -> Result<TransferDetailReceiptResponse> {
        self.post_v3(
            credentials,
            "/v3/transfer-detail/electronic-receipts",
            to_value(request)?,
        )
        .await
    }

    pub async fn query_transfer_detail_receipt(
        &self,
        credentials: &PaymentCredentials,
        request: TransferDetailReceiptQuery,
    ) -> Result<TransferDetailReceiptResponse> {
        self.post_v3_with_query(
            credentials,
            "/v3/transfer-detail/electronic-receipts",
            request.into_query(),
            serde_json::json!({}),
        )
        .await
    }

    pub async fn trade_bill(
        &self,
        credentials: &PaymentCredentials,
        request: BillRequest,
    ) -> Result<BillResponse> {
        request.validate_trade()?;
        let response: BillResponse = self
            .get_v3(credentials, "/v3/bill/tradebill", request.into_query())
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn fund_flow_bill(
        &self,
        credentials: &PaymentCredentials,
        request: BillRequest,
    ) -> Result<BillResponse> {
        request.validate_fund_flow()?;
        let response: BillResponse = self
            .get_v3(
                credentials,
                "/v3/bill/fundflowbill",
                request.into_fund_flow_query(),
            )
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn fund_flow_bill_with_request(
        &self,
        credentials: &PaymentCredentials,
        request: FundFlowBillRequest,
    ) -> Result<BillResponse> {
        request.validate()?;
        let response: BillResponse = self
            .get_v3(credentials, "/v3/bill/fundflowbill", request.into_query())
            .await?;
        response.validate()?;
        Ok(response)
    }

    pub async fn download_bill_bytes(
        &self,
        credentials: &PaymentCredentials,
        request: PaymentBillDownloadRequest,
    ) -> Result<Bytes> {
        request.validate(false)?;
        let (path, query) = split_payment_download_url(&request.download_url)?;
        let path_query = path_with_query(&path, &query);
        let headers = vec![(
            "authorization".to_string(),
            credentials.authorization("GET", &path_query, "")?,
        )];
        let bytes = self
            .inner
            .get_bytes_with_headers(path, query, headers)
            .await?;
        verify_payment_download_hash(
            &bytes,
            request.hash_type.as_deref(),
            request.hash_value.as_deref(),
        )?;
        Ok(bytes)
    }

    pub async fn download_bill(
        &self,
        credentials: &PaymentCredentials,
        request: PaymentBillDownloadRequest,
    ) -> Result<PaymentDownloadedBill> {
        let hash_type = request.hash_type.clone();
        let hash_value = request.hash_value.clone();
        let bytes = self.download_bill_bytes(credentials, request).await?;
        PaymentDownloadedBill::from_verified_bytes(bytes, hash_type, hash_value)
    }

    pub async fn download_bill_to_file(
        &self,
        credentials: &PaymentCredentials,
        request: PaymentBillDownloadRequest,
        destination: impl AsRef<Path>,
    ) -> Result<PaymentDownloadedBillFile> {
        self.download_bill_to_file_inner(credentials, request, destination.as_ref(), None)
            .await
    }

    pub async fn download_bill_to_file_with_max_bytes(
        &self,
        credentials: &PaymentCredentials,
        request: PaymentBillDownloadRequest,
        destination: impl AsRef<Path>,
        max_bytes: u64,
    ) -> Result<PaymentDownloadedBillFile> {
        if max_bytes == 0 {
            return Err(WechatError::Config(
                "payment bill download maximum bytes must be positive".to_string(),
            ));
        }
        self.download_bill_to_file_inner(
            credentials,
            request,
            destination.as_ref(),
            Some(max_bytes),
        )
        .await
    }

    async fn download_bill_to_file_inner(
        &self,
        credentials: &PaymentCredentials,
        request: PaymentBillDownloadRequest,
        destination: &Path,
        max_bytes: Option<u64>,
    ) -> Result<PaymentDownloadedBillFile> {
        request.validate(true)?;
        let expected_hash = request
            .hash_value
            .as_deref()
            .filter(|value| !value.is_empty())
            .ok_or_else(|| {
                WechatError::Config(
                    "payment bill file download requires a non-empty expected hash".to_string(),
                )
            })?;
        let mut hasher = PaymentDownloadHasher::new(request.hash_type.as_deref())?;
        let destination = destination.to_path_buf();
        let parent = destination.parent().unwrap_or_else(|| Path::new("."));
        let file_name = destination
            .file_name()
            .and_then(|value| value.to_str())
            .ok_or_else(|| {
                WechatError::Config("payment bill destination must name a file".to_string())
            })?;
        if tokio::fs::try_exists(&destination).await? {
            return Err(WechatError::Config(format!(
                "payment bill destination already exists: {}",
                destination.display()
            )));
        }

        let temporary = parent.join(format!(".{file_name}.{}.part", uuid::Uuid::now_v7()));
        let mut cleanup = TemporaryFileGuard::new(temporary.clone());
        let mut file = tokio::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&temporary)
            .await?;

        let (path, query) = split_payment_download_url(&request.download_url)?;
        let path_query = path_with_query(&path, &query);
        let headers = vec![(
            "authorization".to_string(),
            credentials.authorization("GET", &path_query, "")?,
        )];
        let mut response = self.inner.get_stream_response(path, query, headers).await?;
        let mut bytes_written = 0_u64;
        while let Some(chunk) = response.next_chunk().await? {
            bytes_written = checked_payment_download_size(bytes_written, chunk.len(), max_bytes)?;
            file.write_all(&chunk).await?;
            hasher.update(&chunk);
        }
        if bytes_written == 0 {
            return Err(WechatError::Config(
                "payment bill download is empty".to_string(),
            ));
        }
        file.flush().await?;
        file.sync_all().await?;
        drop(file);

        let hash_type = hasher.hash_type().to_string();
        let actual_hash = hasher.finalize();
        if !actual_hash.eq_ignore_ascii_case(expected_hash) {
            return Err(WechatError::Crypto(
                "payment download hash mismatch".to_string(),
            ));
        }

        tokio::fs::hard_link(&temporary, &destination)
            .await
            .map_err(|err| {
                WechatError::Config(format!(
                    "cannot commit payment bill to {}: {err}",
                    destination.display()
                ))
            })?;
        if tokio::fs::remove_file(&temporary).await.is_ok() {
            cleanup.disarm();
        }

        Ok(PaymentDownloadedBillFile {
            path: destination,
            bytes_written,
            hash_type,
            hash_value: actual_hash,
        })
    }

    pub async fn download_trade_bill_bytes(
        &self,
        credentials: &PaymentCredentials,
        request: BillRequest,
    ) -> Result<Bytes> {
        let bill: BillResponse = self.trade_bill(credentials, request).await?;
        self.download_bill_bytes(
            credentials,
            PaymentBillDownloadRequest {
                download_url: bill.download_url,
                hash_type: bill.hash_type,
                hash_value: bill.hash_value,
            },
        )
        .await
    }

    pub async fn download_trade_bill(
        &self,
        credentials: &PaymentCredentials,
        request: BillRequest,
    ) -> Result<PaymentDownloadedBill> {
        let bill: BillResponse = self.trade_bill(credentials, request).await?;
        self.download_bill(
            credentials,
            PaymentBillDownloadRequest {
                download_url: bill.download_url,
                hash_type: bill.hash_type,
                hash_value: bill.hash_value,
            },
        )
        .await
    }

    pub async fn download_trade_bill_to_file(
        &self,
        credentials: &PaymentCredentials,
        request: BillRequest,
        destination: impl AsRef<Path>,
    ) -> Result<PaymentDownloadedBillFile> {
        let bill: BillResponse = self.trade_bill(credentials, request).await?;
        self.download_bill_to_file(
            credentials,
            PaymentBillDownloadRequest {
                download_url: bill.download_url,
                hash_type: bill.hash_type,
                hash_value: bill.hash_value,
            },
            destination,
        )
        .await
    }

    pub async fn download_fund_flow_bill_bytes(
        &self,
        credentials: &PaymentCredentials,
        request: BillRequest,
    ) -> Result<Bytes> {
        let bill: BillResponse = self.fund_flow_bill(credentials, request).await?;
        self.download_bill_bytes(
            credentials,
            PaymentBillDownloadRequest {
                download_url: bill.download_url,
                hash_type: bill.hash_type,
                hash_value: bill.hash_value,
            },
        )
        .await
    }

    pub async fn download_fund_flow_bill(
        &self,
        credentials: &PaymentCredentials,
        request: BillRequest,
    ) -> Result<PaymentDownloadedBill> {
        let bill: BillResponse = self.fund_flow_bill(credentials, request).await?;
        self.download_bill(
            credentials,
            PaymentBillDownloadRequest {
                download_url: bill.download_url,
                hash_type: bill.hash_type,
                hash_value: bill.hash_value,
            },
        )
        .await
    }

    pub async fn download_fund_flow_bill_to_file(
        &self,
        credentials: &PaymentCredentials,
        request: BillRequest,
        destination: impl AsRef<Path>,
    ) -> Result<PaymentDownloadedBillFile> {
        let bill: BillResponse = self.fund_flow_bill(credentials, request).await?;
        self.download_bill_to_file(
            credentials,
            PaymentBillDownloadRequest {
                download_url: bill.download_url,
                hash_type: bill.hash_type,
                hash_value: bill.hash_value,
            },
            destination,
        )
        .await
    }

    pub async fn download_transfer_bill_receipt_to_file(
        &self,
        credentials: &PaymentCredentials,
        out_batch_no: impl AsRef<str>,
        destination: impl AsRef<Path>,
    ) -> Result<PaymentDownloadedBillFile> {
        let receipt = self
            .query_transfer_bill_receipt(credentials, out_batch_no)
            .await?;
        self.download_bill_to_file(credentials, receipt.try_into()?, destination)
            .await
    }

    pub async fn download_transfer_detail_receipt_to_file(
        &self,
        credentials: &PaymentCredentials,
        request: TransferDetailReceiptQuery,
        destination: impl AsRef<Path>,
    ) -> Result<PaymentDownloadedBillFile> {
        let receipt = self
            .query_transfer_detail_receipt(credentials, request)
            .await?;
        self.download_bill_to_file(credentials, receipt.try_into()?, destination)
            .await
    }

    pub async fn download_fund_app_elec_sign_by_out_no_to_file(
        &self,
        credentials: &PaymentCredentials,
        out_bill_no: impl AsRef<str>,
        destination: impl AsRef<Path>,
    ) -> Result<PaymentDownloadedBillFile> {
        let receipt = self
            .query_fund_app_elec_sign_by_out_no(credentials, out_bill_no)
            .await?;
        self.download_bill_to_file(credentials, receipt.into(), destination)
            .await
    }

    pub async fn download_fund_app_elec_sign_by_transfer_bill_no_to_file(
        &self,
        credentials: &PaymentCredentials,
        transfer_bill_no: impl AsRef<str>,
        destination: impl AsRef<Path>,
    ) -> Result<PaymentDownloadedBillFile> {
        let receipt = self
            .query_fund_app_elec_sign_by_transfer_bill_no(credentials, transfer_bill_no)
            .await?;
        self.download_bill_to_file(credentials, receipt.into(), destination)
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

    async fn post_v3_empty(
        &self,
        credentials: &PaymentCredentials,
        path: &str,
        body: Value,
    ) -> Result<()> {
        let body_text = body.to_string();
        let headers = vec![(
            "authorization".to_string(),
            credentials.authorization("POST", path, &body_text)?,
        )];
        self.inner.post_json_empty(path, body, headers).await
    }

    async fn post_v3_with_query<R>(
        &self,
        credentials: &PaymentCredentials,
        path: &str,
        query: Vec<(String, String)>,
        body: Value,
    ) -> Result<R>
    where
        R: serde::de::DeserializeOwned,
    {
        let body_text = body.to_string();
        let path_query = path_with_query(path, &query);
        let headers = vec![(
            "authorization".to_string(),
            credentials.authorization("POST", &path_query, &body_text)?,
        )];
        self.inner
            .post_json_with_query(path, query, body, headers)
            .await
    }

    async fn put_v3<R>(
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
            credentials.authorization("PUT", path, &body_text)?,
        )];
        self.inner.put_json(path, body, headers).await
    }

    async fn delete_v3_empty(&self, credentials: &PaymentCredentials, path: &str) -> Result<()> {
        let headers = vec![(
            "authorization".to_string(),
            credentials.authorization("DELETE", path, "")?,
        )];
        self.inner.delete_empty(path, headers).await
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
        let path_query = path_with_query(path, &query);
        let headers = vec![(
            "authorization".to_string(),
            credentials.authorization("GET", &path_query, "")?,
        )];
        self.inner.get_with_headers(path, query, headers).await
    }

    async fn post_legacy_xml<R>(
        &self,
        credentials: &PaymentCredentials,
        api_key: &str,
        path: &str,
        mut params: Vec<(String, String)>,
    ) -> Result<R>
    where
        R: serde::de::DeserializeOwned,
    {
        params.push(("mch_id".to_string(), credentials.mch_id.clone()));
        params.push(("nonce_str".to_string(), crypto::nonce_string(32)));
        let sign = crypto::payment_legacy_sign(&params, api_key);
        params.push(("sign".to_string(), sign));
        self.inner
            .post_xml(path, crypto::payment_legacy_xml(&params))
            .await
    }

    async fn post_legacy_xml_raw<R>(
        &self,
        api_key: &str,
        path: &str,
        mut params: Vec<(String, String)>,
    ) -> Result<R>
    where
        R: serde::de::DeserializeOwned,
    {
        params.push(("nonce_str".to_string(), crypto::nonce_string(32)));
        let sign = crypto::payment_legacy_sign(&params, api_key);
        params.push(("sign".to_string(), sign));
        self.inner
            .post_xml(path, crypto::payment_legacy_xml(&params))
            .await
    }
}

fn path_with_query(path: &str, query: &[(String, String)]) -> String {
    if query.is_empty() {
        path.to_string()
    } else {
        let query_text = query
            .iter()
            .map(|(key, value)| format!("{key}={value}"))
            .collect::<Vec<_>>()
            .join("&");
        format!("{path}?{query_text}")
    }
}

fn split_payment_download_url(download_url: &str) -> Result<(String, Vec<(String, String)>)> {
    if let Ok(url) = reqwest::Url::parse(download_url) {
        let query = parse_raw_query(url.query().unwrap_or(""));
        return Ok((url.path().to_string(), query));
    }

    let (path, query_text) = download_url.split_once('?').unwrap_or((download_url, ""));
    if path.is_empty() {
        return Err(WechatError::Config(
            "payment download url path is empty".to_string(),
        ));
    }
    let query = parse_raw_query(query_text);
    Ok((path.to_string(), query))
}

fn parse_raw_query(query_text: &str) -> Vec<(String, String)> {
    query_text
        .split('&')
        .filter(|item| !item.is_empty())
        .map(|item| {
            let (key, value) = item.split_once('=').unwrap_or((item, ""));
            (key.to_string(), value.to_string())
        })
        .collect()
}

fn validate_payment_bill_date(value: &str) -> Result<()> {
    let bytes = value.as_bytes();
    if bytes.len() != 10
        || bytes[4] != b'-'
        || bytes[7] != b'-'
        || !bytes
            .iter()
            .enumerate()
            .all(|(index, byte)| matches!(index, 4 | 7) || byte.is_ascii_digit())
    {
        return Err(WechatError::Config(
            "payment bill date must use a valid YYYY-MM-DD calendar date".to_string(),
        ));
    }
    chrono::NaiveDate::parse_from_str(value, "%Y-%m-%d").map_err(|_| {
        WechatError::Config(
            "payment bill date must use a valid YYYY-MM-DD calendar date".to_string(),
        )
    })?;
    Ok(())
}

fn validate_payment_trade_bill_type(value: Option<&str>) -> Result<()> {
    if value.is_none_or(|value| matches!(value, "ALL" | "SUCCESS" | "REFUND")) {
        return Ok(());
    }
    Err(WechatError::Config(
        "payment trade bill type must be ALL, SUCCESS, or REFUND".to_string(),
    ))
}

fn validate_payment_fund_flow_account_type(value: Option<&str>) -> Result<()> {
    if value.is_none_or(|value| matches!(value, "BASIC" | "OPERATION" | "FEES")) {
        return Ok(());
    }
    Err(WechatError::Config(
        "payment fund-flow account type must be BASIC, OPERATION, or FEES".to_string(),
    ))
}

fn validate_payment_bill_archive_type(value: Option<&str>) -> Result<()> {
    if value.is_none_or(|value| value == "GZIP") {
        return Ok(());
    }
    Err(WechatError::Config(
        "payment bill archive type must be GZIP when provided".to_string(),
    ))
}

fn validate_payment_download_hash_metadata(
    hash_type: Option<&str>,
    hash_value: Option<&str>,
    required: bool,
) -> Result<()> {
    match (hash_type, hash_value) {
        (None, None) if !required => return Ok(()),
        (None, None) => {
            return Err(WechatError::Config(
                "payment bill response requires hash_type and hash_value".to_string(),
            ));
        }
        (Some(_), None) | (None, Some(_)) => {
            return Err(WechatError::Config(
                "payment bill hash_type and hash_value must be provided together".to_string(),
            ));
        }
        (Some(hash_type), Some(hash_value)) => {
            let expected_len = match hash_type {
                "SHA1" | "SHA-1" => 40,
                "SHA256" | "SHA-256" => 64,
                other => {
                    return Err(WechatError::Config(format!(
                        "unsupported payment download hash type: {other}"
                    )));
                }
            };
            if hash_value.len() != expected_len
                || !hash_value.bytes().all(|byte| byte.is_ascii_hexdigit())
            {
                return Err(WechatError::Config(format!(
                    "payment bill {hash_type} hash must contain {expected_len} hexadecimal characters"
                )));
            }
        }
    }
    Ok(())
}

fn verify_payment_download_hash(
    bytes: &[u8],
    hash_type: Option<&str>,
    hash_value: Option<&str>,
) -> Result<()> {
    let Some(hash_value) = hash_value.filter(|value| !value.is_empty()) else {
        return Ok(());
    };
    let actual = match hash_type.unwrap_or("SHA1").to_ascii_uppercase().as_str() {
        "SHA1" | "SHA-1" => {
            let mut hasher = sha1::Sha1::new();
            hasher.update(bytes);
            hex::encode(hasher.finalize())
        }
        "SHA256" | "SHA-256" => crypto::sha256_hex(bytes),
        other => {
            return Err(WechatError::Crypto(format!(
                "unsupported payment download hash type: {other}"
            )));
        }
    };

    if !actual.eq_ignore_ascii_case(hash_value) {
        return Err(WechatError::Crypto(
            "payment download hash mismatch".to_string(),
        ));
    }
    Ok(())
}

fn checked_payment_download_size(
    current: u64,
    chunk_len: usize,
    max_bytes: Option<u64>,
) -> Result<u64> {
    let chunk_len = u64::try_from(chunk_len)
        .map_err(|_| WechatError::Config("payment bill is too large".to_string()))?;
    let next = current
        .checked_add(chunk_len)
        .ok_or_else(|| WechatError::Config("payment bill is too large".to_string()))?;
    if let Some(maximum) = max_bytes {
        if next > maximum {
            return Err(WechatError::Config(format!(
                "payment bill exceeds the configured {maximum} byte limit"
            )));
        }
    }
    Ok(next)
}

enum PaymentDownloadHasher {
    Sha1(sha1::Sha1),
    Sha256(sha2::Sha256),
}

impl PaymentDownloadHasher {
    fn new(hash_type: Option<&str>) -> Result<Self> {
        match hash_type.unwrap_or("SHA1").to_ascii_uppercase().as_str() {
            "SHA1" | "SHA-1" => Ok(Self::Sha1(sha1::Sha1::new())),
            "SHA256" | "SHA-256" => Ok(Self::Sha256(sha2::Sha256::new())),
            other => Err(WechatError::Crypto(format!(
                "unsupported payment download hash type: {other}"
            ))),
        }
    }

    fn hash_type(&self) -> &'static str {
        match self {
            Self::Sha1(_) => "SHA1",
            Self::Sha256(_) => "SHA256",
        }
    }

    fn update(&mut self, bytes: &[u8]) {
        match self {
            Self::Sha1(hasher) => hasher.update(bytes),
            Self::Sha256(hasher) => hasher.update(bytes),
        }
    }

    fn finalize(self) -> String {
        match self {
            Self::Sha1(hasher) => hex::encode(hasher.finalize()),
            Self::Sha256(hasher) => hex::encode(hasher.finalize()),
        }
    }
}

struct TemporaryFileGuard {
    path: PathBuf,
    armed: bool,
}

impl TemporaryFileGuard {
    fn new(path: PathBuf) -> Self {
        Self { path, armed: true }
    }

    fn disarm(&mut self) {
        self.armed = false;
    }
}

impl Drop for TemporaryFileGuard {
    fn drop(&mut self) {
        if self.armed {
            let _ = std::fs::remove_file(&self.path);
        }
    }
}

fn build_sandbox_sign_key_xml(
    credentials: &PaymentCredentials,
    api_key: &str,
    nonce_str: &str,
) -> String {
    let mut params = vec![
        ("mch_id".to_string(), credentials.mch_id.clone()),
        ("nonce_str".to_string(), nonce_str.to_string()),
    ];
    let sign = crypto::payment_legacy_sign(&params, api_key);
    params.push(("sign".to_string(), sign));
    crypto::payment_legacy_xml(&params)
}

const MERCHANT_MEDIA_UPLOAD_BOUNDARY: &str = "----roze-wechat-pay-v3-media-upload";

fn build_merchant_media_upload_body(request: &MerchantMediaUploadRequest) -> (String, Vec<u8>) {
    let meta = serde_json::json!({
        "filename": request.file_name,
        "sha256": request.sha256,
    })
    .to_string();
    let file_name = multipart_quoted(&request.file_name);
    let mut body = Vec::new();
    body.extend_from_slice(format!("--{MERCHANT_MEDIA_UPLOAD_BOUNDARY}\r\n").as_bytes());
    body.extend_from_slice(
        b"Content-Disposition: form-data; name=\"meta\"\r\nContent-Type: application/json\r\n\r\n",
    );
    body.extend_from_slice(meta.as_bytes());
    body.extend_from_slice(b"\r\n");
    body.extend_from_slice(format!("--{MERCHANT_MEDIA_UPLOAD_BOUNDARY}\r\n").as_bytes());
    body.extend_from_slice(
        format!(
            "Content-Disposition: form-data; name=\"file\"; filename=\"{file_name}\"\r\nContent-Type: application/octet-stream\r\n\r\n"
        )
        .as_bytes(),
    );
    body.extend_from_slice(&request.data);
    body.extend_from_slice(b"\r\n");
    body.extend_from_slice(format!("--{MERCHANT_MEDIA_UPLOAD_BOUNDARY}--\r\n").as_bytes());

    (
        format!("multipart/form-data; boundary={MERCHANT_MEDIA_UPLOAD_BOUNDARY}"),
        body,
    )
}

fn multipart_quoted(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

#[derive(Debug, Clone)]
pub struct PaymentCredentials {
    pub mch_id: String,
    pub serial_no: String,
    pub private_key_pem: String,
}

impl PaymentCredentials {
    pub fn authorization(&self, method: &str, path_query: &str, body: &str) -> Result<String> {
        self.authorization_bytes(method, path_query, body.as_bytes())
    }

    pub fn authorization_bytes(
        &self,
        method: &str,
        path_query: &str,
        body: &[u8],
    ) -> Result<String> {
        let timestamp = chrono::Utc::now().timestamp();
        let nonce = crypto::nonce_string(32);
        let mut message = Vec::new();
        message.extend_from_slice(
            format!("{method}\n{path_query}\n{timestamp}\n{nonce}\n").as_bytes(),
        );
        message.extend_from_slice(body);
        message.extend_from_slice(b"\n");
        let signature = crypto::rsa_sha256_sign_base64(&self.private_key_pem, &message)?;
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
pub struct CombineAppPrepayRequest {
    pub combine_appid: String,
    pub combine_mchid: String,
    pub combine_out_trade_no: String,
    pub notify_url: String,
    pub sub_orders: Vec<CombineSubOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_info: Option<CombineSceneInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub combine_payer_info: Option<CombinePayerInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_expire: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombineSceneInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    pub payer_client_ip: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombineSubOrder {
    pub mchid: String,
    pub out_trade_no: String,
    pub description: String,
    pub amount: CombineAmount,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_info: Option<CombineSettleInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombineAmount {
    pub total_amount: i64,
    #[serde(default = "default_cny")]
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombineSettleInfo {
    pub profit_sharing: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subsidy_amount: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombinePayerInfo {
    pub openid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodepayRequest {
    pub appid: String,
    pub mchid: String,
    pub description: String,
    pub out_trade_no: String,
    pub attach: String,
    pub payer: CodepayPayer,
    pub amount: CodepayAmount,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_fapiao: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_info: Option<CodepaySceneInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<CodepayDetail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_info: Option<CodepaySettleInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodepayPayer {
    pub auth_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodepayAmount {
    pub total: i64,
    #[serde(default = "default_cny")]
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodepaySceneInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_info: Option<CodepayStoreInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_ip: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodepayStoreInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodepayDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_price: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_detail: Option<Vec<CodepayGoodsDetail>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodepayGoodsDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_goods_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wxpay_goods_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_name: Option<String>,
    pub quantity: i64,
    pub unit_price: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodepaySettleInfo {
    pub profit_sharing: bool,
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
pub struct PartnerTransactionQuery {
    pub transaction_id: String,
    pub sp_mchid: String,
    pub sub_mchid: String,
}

impl PartnerTransactionQuery {
    fn into_query(self) -> Vec<(String, String)> {
        vec![
            ("sp_mchid".to_string(), self.sp_mchid),
            ("sub_mchid".to_string(), self.sub_mchid),
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerRefundQuery {
    pub out_refund_no: String,
    pub sub_mchid: String,
}

impl PartnerRefundQuery {
    fn into_query(self) -> Vec<(String, String)> {
        vec![("sub_mchid".to_string(), self.sub_mchid)]
    }
}

pub type PartnerRefundDetailResponse = RefundDetailResponse;

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

fn default_one_i64() -> i64 {
    1
}

fn push_optional_param(params: &mut Vec<(String, String)>, key: &str, value: Option<String>) {
    if let Some(value) = value {
        params.push((key.to_string(), value));
    }
}

fn push_optional_query(query: &mut Vec<(String, String)>, key: &str, value: Option<String>) {
    if let Some(value) = value {
        query.push((key.to_string(), value));
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payer {
    pub openid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrepayResponse {
    pub prepay_id: String,
    #[serde(default, flatten)]
    pub extra: Value,
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
    #[serde(default, flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativePrepayResponse {
    pub code_url: String,
    #[serde(default, flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentOrderResponse {
    #[serde(default)]
    pub appid: Option<String>,
    #[serde(default)]
    pub mchid: Option<String>,
    #[serde(default)]
    pub sp_appid: Option<String>,
    #[serde(default)]
    pub sp_mchid: Option<String>,
    #[serde(default)]
    pub sub_appid: Option<String>,
    #[serde(default)]
    pub sub_mchid: Option<String>,
    #[serde(default)]
    pub out_trade_no: Option<String>,
    #[serde(default)]
    pub transaction_id: Option<String>,
    #[serde(default)]
    pub trade_type: Option<String>,
    #[serde(default)]
    pub trade_state: Option<String>,
    #[serde(default)]
    pub trade_state_desc: Option<String>,
    #[serde(default)]
    pub bank_type: Option<String>,
    #[serde(default)]
    pub attach: Option<String>,
    #[serde(default)]
    pub success_time: Option<String>,
    #[serde(default)]
    pub amount: Option<PaymentTransactionAmount>,
    #[serde(default)]
    pub payer: Option<PaymentTransactionPayer>,
    #[serde(default)]
    pub scene_info: Option<PaymentTransactionSceneInfo>,
    #[serde(default)]
    pub promotion_detail: Vec<PaymentPromotionDetail>,
    #[serde(default, flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaymentTradeStateKind {
    Success,
    Refund,
    NotPay,
    Closed,
    Revoked,
    UserPaying,
    PayError,
    Accept,
    Other,
}

impl PaymentTradeStateKind {
    pub fn from_code(value: &str) -> Self {
        if value.eq_ignore_ascii_case("SUCCESS") {
            Self::Success
        } else if value.eq_ignore_ascii_case("REFUND") {
            Self::Refund
        } else if value.eq_ignore_ascii_case("NOTPAY") {
            Self::NotPay
        } else if value.eq_ignore_ascii_case("CLOSED") {
            Self::Closed
        } else if value.eq_ignore_ascii_case("REVOKED") {
            Self::Revoked
        } else if value.eq_ignore_ascii_case("USERPAYING") {
            Self::UserPaying
        } else if value.eq_ignore_ascii_case("PAYERROR") {
            Self::PayError
        } else if value.eq_ignore_ascii_case("ACCEPT") {
            Self::Accept
        } else {
            Self::Other
        }
    }

    pub fn is_success(self) -> bool {
        matches!(self, Self::Success)
    }

    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Success | Self::Refund | Self::Closed | Self::Revoked | Self::PayError
        )
    }

    pub fn needs_polling(self) -> bool {
        matches!(self, Self::NotPay | Self::UserPaying | Self::Accept)
    }
}

impl PaymentOrderResponse {
    pub fn trade_state_kind(&self) -> Option<PaymentTradeStateKind> {
        self.trade_state
            .as_deref()
            .map(PaymentTradeStateKind::from_code)
    }

    pub fn is_paid(&self) -> bool {
        self.trade_state_kind()
            .is_some_and(PaymentTradeStateKind::is_success)
    }

    pub fn effective_appid(&self) -> Option<&str> {
        self.sub_appid
            .as_deref()
            .or(self.appid.as_deref())
            .or(self.sp_appid.as_deref())
    }

    pub fn effective_mchid(&self) -> Option<&str> {
        self.sub_mchid
            .as_deref()
            .or(self.mchid.as_deref())
            .or(self.sp_mchid.as_deref())
    }

    pub fn promotion_total(&self) -> Result<i64> {
        payment_promotion_total(&self.promotion_detail)
    }

    pub fn assert_amount_reconciles(&self) -> Result<()> {
        let amount = self
            .amount
            .as_ref()
            .ok_or_else(|| WechatError::Config("payment order amount is missing".to_string()))?;
        amount.assert_reconciles(&self.promotion_detail)
    }

    pub fn verify_paid_order(
        &self,
        expected_mchid: &str,
        expected_out_trade_no: &str,
        expected_total: i64,
        expected_currency: &str,
    ) -> Result<()> {
        verify_paid_transaction(
            self.trade_state.as_deref(),
            self.effective_mchid(),
            self.out_trade_no.as_deref(),
            self.transaction_id.as_deref(),
            self.amount.as_ref(),
            &self.promotion_detail,
            expected_mchid,
            expected_out_trade_no,
            expected_total,
            expected_currency,
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentStatusResponse {
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicropayRequest {
    pub appid: String,
    pub mchid: String,
    pub description: String,
    pub out_trade_no: String,
    pub auth_code: String,
    pub amount: Amount,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_info: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Applyment4SubRequest {
    pub business_code: String,
    pub contact_info: Value,
    pub subject_info: Value,
    pub business_info: Value,
    pub settlement_info: Value,
    pub bank_account_info: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addition_info: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Applyment4SubResponse {
    #[serde(default)]
    pub applyment_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Applyment4SubQueryResponse {
    #[serde(flatten)]
    pub value: Value,
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
pub struct RefundDetailResponse {
    pub refund_id: String,
    pub out_refund_no: String,
    #[serde(default)]
    pub transaction_id: Option<String>,
    #[serde(default)]
    pub out_trade_no: Option<String>,
    pub channel: String,
    pub user_received_account: String,
    #[serde(default)]
    pub success_time: Option<String>,
    pub create_time: String,
    pub status: String,
    #[serde(default)]
    pub funds_account: Option<String>,
    pub amount: RefundDetailAmount,
    #[serde(default)]
    pub promotion_detail: Vec<RefundPromotionDetail>,
    #[serde(default, flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaymentRefundStatusKind {
    Success,
    Closed,
    Processing,
    Abnormal,
    Other,
}

impl PaymentRefundStatusKind {
    pub fn from_code(value: &str) -> Self {
        if value.eq_ignore_ascii_case("SUCCESS") {
            Self::Success
        } else if value.eq_ignore_ascii_case("CLOSED") {
            Self::Closed
        } else if value.eq_ignore_ascii_case("PROCESSING") {
            Self::Processing
        } else if value.eq_ignore_ascii_case("ABNORMAL") {
            Self::Abnormal
        } else {
            Self::Other
        }
    }

    pub fn is_success(self) -> bool {
        matches!(self, Self::Success)
    }

    pub fn is_terminal(self) -> bool {
        !matches!(self, Self::Processing)
    }

    pub fn needs_attention(self) -> bool {
        matches!(self, Self::Closed | Self::Abnormal | Self::Other)
    }
}

impl RefundDetailResponse {
    pub fn status_kind(&self) -> PaymentRefundStatusKind {
        PaymentRefundStatusKind::from_code(&self.status)
    }

    pub fn is_success(&self) -> bool {
        self.status_kind().is_success()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundDetailAmount {
    pub refund: i64,
    pub total: i64,
    #[serde(default = "default_cny")]
    pub currency: String,
    #[serde(default)]
    pub from: Vec<RefundAmountFrom>,
    #[serde(default)]
    pub payer_total: Option<i64>,
    #[serde(default)]
    pub payer_refund: Option<i64>,
    #[serde(default)]
    pub settlement_refund: Option<i64>,
    #[serde(default)]
    pub settlement_total: Option<i64>,
    #[serde(default)]
    pub discount_refund: Option<i64>,
    #[serde(default, flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundAmountFrom {
    pub account: String,
    pub amount: i64,
    #[serde(default, flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundPromotionDetail {
    pub promotion_id: String,
    pub scope: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub amount: i64,
    pub refund_amount: i64,
    #[serde(default)]
    pub goods_detail: Vec<RefundGoodsDetail>,
    #[serde(default, flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundGoodsDetail {
    pub merchant_goods_id: String,
    #[serde(default)]
    pub wechatpay_goods_id: Option<String>,
    #[serde(default)]
    pub goods_name: Option<String>,
    pub unit_price: i64,
    pub refund_amount: i64,
    pub refund_quantity: i64,
    #[serde(default, flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendRedpackRequest {
    pub mch_billno: String,
    pub wxappid: String,
    pub send_name: String,
    pub re_openid: String,
    pub total_amount: i64,
    #[serde(default = "default_one_i64")]
    pub total_num: i64,
    pub wishing: String,
    pub client_ip: String,
    pub act_name: String,
    pub remark: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_info: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consume_mch_id: Option<String>,
}

impl SendRedpackRequest {
    fn into_params(self) -> Vec<(String, String)> {
        let mut params = vec![
            ("mch_billno".to_string(), self.mch_billno),
            ("wxappid".to_string(), self.wxappid),
            ("send_name".to_string(), self.send_name),
            ("re_openid".to_string(), self.re_openid),
            ("total_amount".to_string(), self.total_amount.to_string()),
            ("total_num".to_string(), self.total_num.to_string()),
            ("wishing".to_string(), self.wishing),
            ("client_ip".to_string(), self.client_ip),
            ("act_name".to_string(), self.act_name),
            ("remark".to_string(), self.remark),
        ];
        push_optional_param(&mut params, "scene_id", self.scene_id);
        push_optional_param(&mut params, "risk_info", self.risk_info);
        push_optional_param(&mut params, "consume_mch_id", self.consume_mch_id);
        params
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendGroupRedpackRequest {
    pub mch_billno: String,
    pub wxappid: String,
    pub send_name: String,
    pub re_openid: String,
    pub total_amount: i64,
    pub total_num: i64,
    pub amt_type: String,
    pub wishing: String,
    pub act_name: String,
    pub remark: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_info: Option<String>,
}

impl SendGroupRedpackRequest {
    fn into_params(self) -> Vec<(String, String)> {
        let mut params = vec![
            ("mch_billno".to_string(), self.mch_billno),
            ("wxappid".to_string(), self.wxappid),
            ("send_name".to_string(), self.send_name),
            ("re_openid".to_string(), self.re_openid),
            ("total_amount".to_string(), self.total_amount.to_string()),
            ("total_num".to_string(), self.total_num.to_string()),
            ("amt_type".to_string(), self.amt_type),
            ("wishing".to_string(), self.wishing),
            ("act_name".to_string(), self.act_name),
            ("remark".to_string(), self.remark),
        ];
        push_optional_param(&mut params, "scene_id", self.scene_id);
        push_optional_param(&mut params, "risk_info", self.risk_info);
        params
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRedpackRequest {
    pub mch_billno: String,
    pub appid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_type: Option<String>,
}

impl QueryRedpackRequest {
    fn into_params(self) -> Vec<(String, String)> {
        vec![
            ("mch_billno".to_string(), self.mch_billno),
            ("appid".to_string(), self.appid),
            (
                "bill_type".to_string(),
                self.bill_type.unwrap_or_else(|| "MCHT".to_string()),
            ),
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkRedpackRequest {
    pub mch_billno: String,
    pub wxappid: String,
    pub sender_name: String,
    pub sender_header_media_id: String,
    pub re_openid: String,
    pub total_amount: i64,
    pub wishing: String,
    pub act_name: String,
    pub remark: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workwx_sign: Option<String>,
}

impl WorkRedpackRequest {
    fn into_params(self) -> Vec<(String, String)> {
        let mut params = vec![
            ("mch_billno".to_string(), self.mch_billno),
            ("wxappid".to_string(), self.wxappid),
            ("sender_name".to_string(), self.sender_name),
            (
                "sender_header_media_id".to_string(),
                self.sender_header_media_id,
            ),
            ("re_openid".to_string(), self.re_openid),
            ("total_amount".to_string(), self.total_amount.to_string()),
            ("wishing".to_string(), self.wishing),
            ("act_name".to_string(), self.act_name),
            ("remark".to_string(), self.remark),
        ];
        push_optional_param(&mut params, "scene_id", self.scene_id);
        push_optional_param(&mut params, "workwx_sign", self.workwx_sign);
        params
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryWorkRedpackRequest {
    pub mch_billno: String,
    pub appid: String,
}

impl QueryWorkRedpackRequest {
    fn into_params(self) -> Vec<(String, String)> {
        vec![
            ("mch_billno".to_string(), self.mch_billno),
            ("appid".to_string(), self.appid),
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniProgramRedpackRequest {
    pub mch_billno: String,
    pub wxappid: String,
    pub send_name: String,
    pub re_openid: String,
    pub total_amount: i64,
    pub total_num: i64,
    pub wishing: String,
    pub act_name: String,
    pub remark: String,
    pub notify_way: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_id: Option<String>,
}

impl MiniProgramRedpackRequest {
    fn into_params(self) -> Vec<(String, String)> {
        let mut params = vec![
            ("mch_billno".to_string(), self.mch_billno),
            ("wxappid".to_string(), self.wxappid),
            ("send_name".to_string(), self.send_name),
            ("re_openid".to_string(), self.re_openid),
            ("total_amount".to_string(), self.total_amount.to_string()),
            ("total_num".to_string(), self.total_num.to_string()),
            ("wishing".to_string(), self.wishing),
            ("act_name".to_string(), self.act_name),
            ("remark".to_string(), self.remark),
            ("notify_way".to_string(), self.notify_way),
        ];
        push_optional_param(&mut params, "scene_id", self.scene_id);
        params
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedpackResponse {
    #[serde(rename = "return_code")]
    pub return_code: String,
    #[serde(default, rename = "return_msg")]
    pub return_msg: Option<String>,
    #[serde(default, rename = "result_code")]
    pub result_code: Option<String>,
    #[serde(default, rename = "err_code")]
    pub err_code: Option<String>,
    #[serde(default, rename = "err_code_des")]
    pub err_code_des: Option<String>,
    #[serde(default, rename = "mch_billno")]
    pub mch_billno: Option<String>,
    #[serde(default, rename = "mch_id")]
    pub mch_id: Option<String>,
    #[serde(default)]
    pub wxappid: Option<String>,
    #[serde(default, rename = "re_openid")]
    pub re_openid: Option<String>,
    #[serde(default, rename = "total_amount")]
    pub total_amount: Option<i64>,
    #[serde(default, rename = "send_listid")]
    pub send_list_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedpackInfoResponse {
    #[serde(rename = "return_code")]
    pub return_code: String,
    #[serde(default, rename = "return_msg")]
    pub return_msg: Option<String>,
    #[serde(default, rename = "result_code")]
    pub result_code: Option<String>,
    #[serde(default, rename = "err_code")]
    pub err_code: Option<String>,
    #[serde(default, rename = "err_code_des")]
    pub err_code_des: Option<String>,
    #[serde(default, rename = "mch_billno")]
    pub mch_billno: Option<String>,
    #[serde(default, rename = "mch_id")]
    pub mch_id: Option<String>,
    #[serde(default, rename = "detail_id")]
    pub detail_id: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default, rename = "send_type")]
    pub send_type: Option<String>,
    #[serde(default, rename = "hb_type")]
    pub hb_type: Option<String>,
    #[serde(default, rename = "total_num")]
    pub total_num: Option<i64>,
    #[serde(default, rename = "total_amount")]
    pub total_amount: Option<i64>,
    #[serde(default, rename = "send_time")]
    pub send_time: Option<String>,
    #[serde(default, rename = "refund_time")]
    pub refund_time: Option<String>,
    #[serde(default, rename = "refund_amount")]
    pub refund_amount: Option<i64>,
    #[serde(default)]
    pub wishing: Option<String>,
    #[serde(default)]
    pub remark: Option<String>,
    #[serde(default, rename = "act_name")]
    pub act_name: Option<String>,
    #[serde(default)]
    pub hblist: Option<RedpackReceiverList>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedpackReceiverList {
    #[serde(default, rename = "hbinfo")]
    pub receivers: Vec<RedpackReceiver>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedpackReceiver {
    pub openid: String,
    pub amount: i64,
    #[serde(rename = "rcv_time")]
    pub receive_time: String,
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
pub struct ProfitSharingReturnOrderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_mchid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_order_no: Option<String>,
    pub out_return_no: String,
    pub return_mchid: String,
    pub amount: i64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfitSharingReturnOrderQuery {
    pub out_order_no: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_mchid: Option<String>,
}

impl ProfitSharingReturnOrderQuery {
    fn into_query(self) -> Vec<(String, String)> {
        let mut query = vec![("out_order_no".to_string(), self.out_order_no)];
        push_optional_query(&mut query, "sub_mchid", self.sub_mchid);
        query
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfitSharingUnfreezeRequest {
    pub transaction_id: String,
    pub out_order_no: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_mchid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfitSharingBillRequest {
    pub bill_date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tar_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_mchid: Option<String>,
}

impl ProfitSharingBillRequest {
    pub fn validate(&self) -> Result<()> {
        validate_payment_bill_date(&self.bill_date)?;
        validate_payment_bill_archive_type(self.tar_type.as_deref())?;
        if let Some(sub_mchid) = self.sub_mchid.as_deref() {
            validate_payment_identifier(sub_mchid, "profit-sharing bill sub merchant id", 32)?;
        }
        Ok(())
    }

    fn into_query(self) -> Vec<(String, String)> {
        let mut query = vec![("bill_date".to_string(), self.bill_date)];
        push_optional_query(&mut query, "tar_type", self.tar_type);
        push_optional_query(&mut query, "sub_mchid", self.sub_mchid);
        query
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyProfitSharingReturnRequest {
    pub appid: String,
    pub out_order_no: String,
    pub out_return_no: String,
    pub return_account_type: String,
    pub return_account: String,
    pub return_amount: String,
    pub description: String,
}

impl LegacyProfitSharingReturnRequest {
    fn into_params(self) -> Vec<(String, String)> {
        vec![
            ("appid".to_string(), self.appid),
            ("out_order_no".to_string(), self.out_order_no),
            ("out_return_no".to_string(), self.out_return_no),
            ("return_account_type".to_string(), self.return_account_type),
            ("return_account".to_string(), self.return_account),
            ("return_amount".to_string(), self.return_amount),
            ("description".to_string(), self.description),
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyProfitSharingReturnResponse {
    #[serde(rename = "return_code")]
    pub return_code: String,
    #[serde(default, rename = "return_msg")]
    pub return_msg: Option<String>,
    #[serde(default, rename = "mch_id")]
    pub mch_id: Option<String>,
    #[serde(default)]
    pub appid: Option<String>,
    #[serde(default, rename = "order_id")]
    pub order_id: Option<String>,
    #[serde(default, rename = "out_order_no")]
    pub out_order_no: Option<String>,
    #[serde(default, rename = "out_return_no")]
    pub out_return_no: Option<String>,
    #[serde(default, rename = "return_no")]
    pub return_no: Option<String>,
    #[serde(default)]
    pub result: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouponStockCreateRequest {
    #[serde(flatten)]
    pub value: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouponStockOperationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stock_creator_mchid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouponStockListRequest {
    pub stock_creator_mchid: String,
    pub offset: i64,
    pub limit: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stock_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_start_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_end_time: Option<String>,
}

impl CouponStockListRequest {
    fn into_query(self) -> Vec<(String, String)> {
        let mut query = vec![
            ("stock_creator_mchid".to_string(), self.stock_creator_mchid),
            ("offset".to_string(), self.offset.to_string()),
            ("limit".to_string(), self.limit.to_string()),
        ];
        push_optional_query(&mut query, "status", self.status);
        push_optional_query(&mut query, "stock_id", self.stock_id);
        push_optional_query(&mut query, "create_start_time", self.create_start_time);
        push_optional_query(&mut query, "create_end_time", self.create_end_time);
        query
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendCouponRequest {
    pub appid: String,
    pub stock_id: String,
    pub out_request_no: String,
    pub stock_creator_mchid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_value: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_minimum: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCouponListRequest {
    pub appid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stock_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_mchid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_mchid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl UserCouponListRequest {
    fn into_query(self) -> Vec<(String, String)> {
        let mut query = vec![("appid".to_string(), self.appid)];
        push_optional_query(&mut query, "stock_id", self.stock_id);
        push_optional_query(&mut query, "coupon_state", self.coupon_state);
        push_optional_query(&mut query, "creator_mchid", self.creator_mchid);
        push_optional_query(&mut query, "sender_mchid", self.sender_mchid);
        if let Some(offset) = self.offset {
            query.push(("offset".to_string(), offset.to_string()));
        }
        if let Some(limit) = self.limit {
            query.push(("limit".to_string(), limit.to_string()));
        }
        query
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouponStockResponse {
    #[serde(flatten)]
    pub value: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouponStockOperationResponse {
    #[serde(flatten)]
    pub value: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouponStockListResponse {
    #[serde(default)]
    pub total_count: Option<i64>,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub offset: Option<i64>,
    #[serde(default)]
    pub data: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendCouponResponse {
    pub coupon_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCouponListResponse {
    #[serde(default)]
    pub total_count: Option<i64>,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub offset: Option<i64>,
    #[serde(default)]
    pub data: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCouponResponse {
    #[serde(flatten)]
    pub value: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferToBalanceRequest {
    pub mch_appid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_info: Option<String>,
    pub partner_trade_no: String,
    pub openid: String,
    pub check_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub re_user_name: Option<String>,
    pub amount: i64,
    pub desc: String,
    pub spbill_create_ip: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finder_template_id: Option<String>,
}

impl TransferToBalanceRequest {
    fn into_params(self, credentials: &PaymentCredentials) -> Vec<(String, String)> {
        let mut params = vec![
            ("mch_appid".to_string(), self.mch_appid),
            ("mchid".to_string(), credentials.mch_id.clone()),
            ("partner_trade_no".to_string(), self.partner_trade_no),
            ("openid".to_string(), self.openid),
            ("check_name".to_string(), self.check_name),
            ("amount".to_string(), self.amount.to_string()),
            ("desc".to_string(), self.desc),
            ("spbill_create_ip".to_string(), self.spbill_create_ip),
        ];
        push_optional_param(&mut params, "device_info", self.device_info);
        push_optional_param(&mut params, "re_user_name", self.re_user_name);
        push_optional_param(&mut params, "scene", self.scene);
        if let Some(brand_id) = self.brand_id {
            params.push(("brand_id".to_string(), brand_id.to_string()));
        }
        push_optional_param(&mut params, "finder_template_id", self.finder_template_id);
        params
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyTransferInfoResponse {
    #[serde(rename = "return_code")]
    pub return_code: String,
    #[serde(default, rename = "return_msg")]
    pub return_msg: Option<String>,
    #[serde(default, rename = "result_code")]
    pub result_code: Option<String>,
    #[serde(default, rename = "err_code")]
    pub err_code: Option<String>,
    #[serde(default, rename = "err_code_des")]
    pub err_code_des: Option<String>,
    #[serde(default, rename = "mch_id")]
    pub mch_id: Option<String>,
    #[serde(default)]
    pub appid: Option<String>,
    #[serde(default, rename = "detail_id")]
    pub detail_id: Option<String>,
    #[serde(default, rename = "partner_trade_no")]
    pub partner_trade_no: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default, rename = "payment_amount")]
    pub payment_amount: Option<String>,
    #[serde(default)]
    pub openid: Option<String>,
    #[serde(default, rename = "transfer_time")]
    pub transfer_time: Option<String>,
    #[serde(default, rename = "transfer_name")]
    pub transfer_name: Option<String>,
    #[serde(default)]
    pub desc: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferToBalanceResponse {
    #[serde(rename = "return_code")]
    pub return_code: String,
    #[serde(default, rename = "return_msg")]
    pub return_msg: Option<String>,
    #[serde(default, rename = "result_code")]
    pub result_code: Option<String>,
    #[serde(default, rename = "err_code")]
    pub err_code: Option<String>,
    #[serde(default, rename = "err_code_des")]
    pub err_code_des: Option<String>,
    #[serde(default, rename = "mch_appid")]
    pub mch_appid: Option<String>,
    #[serde(default, rename = "mchid")]
    pub mchid: Option<String>,
    #[serde(default, rename = "partner_trade_no")]
    pub partner_trade_no: Option<String>,
    #[serde(default, rename = "payment_no")]
    pub payment_no: Option<String>,
    #[serde(default, rename = "payment_time")]
    pub payment_time: Option<String>,
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
pub struct TransferBillReceiptResponse {
    pub out_batch_no: String,
    #[serde(default)]
    pub signature_no: Option<String>,
    pub signature_status: String,
    #[serde(default)]
    pub hash_type: Option<String>,
    #[serde(default)]
    pub hash_value: Option<String>,
    #[serde(default)]
    pub download_url: Option<String>,
    pub create_time: String,
    pub update_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferDetailReceiptRequest {
    pub accept_type: String,
    pub out_batch_no: String,
    pub out_detail_no: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferDetailReceiptQuery {
    pub accept_type: String,
    pub out_batch_no: String,
    pub out_detail_no: String,
}

impl TransferDetailReceiptQuery {
    fn into_query(self) -> Vec<(String, String)> {
        vec![
            ("accept_type".to_string(), self.accept_type),
            ("out_batch_no".to_string(), self.out_batch_no),
            ("out_detail_no".to_string(), self.out_detail_no),
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferDetailReceiptResponse {
    pub accept_type: String,
    pub out_batch_no: String,
    pub out_detail_no: String,
    #[serde(default)]
    pub signature_no: Option<String>,
    pub signature_status: String,
    #[serde(default)]
    pub hash_type: Option<String>,
    #[serde(default)]
    pub hash_value: Option<String>,
    #[serde(default)]
    pub download_url: Option<String>,
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
    pub fn validate_trade(&self) -> Result<()> {
        validate_payment_bill_date(&self.bill_date)?;
        validate_payment_trade_bill_type(self.bill_type.as_deref())?;
        validate_payment_bill_archive_type(self.tar_type.as_deref())
    }

    pub fn validate_fund_flow(&self) -> Result<()> {
        validate_payment_bill_date(&self.bill_date)?;
        validate_payment_fund_flow_account_type(self.bill_type.as_deref())?;
        validate_payment_bill_archive_type(self.tar_type.as_deref())
    }

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

    fn into_fund_flow_query(self) -> Vec<(String, String)> {
        let mut query = vec![("bill_date".to_string(), self.bill_date)];
        if let Some(account_type) = self.bill_type {
            query.push(("account_type".to_string(), account_type));
        }
        if let Some(tar_type) = self.tar_type {
            query.push(("tar_type".to_string(), tar_type));
        }
        query
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundFlowBillRequest {
    pub bill_date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tar_type: Option<String>,
}

impl FundFlowBillRequest {
    pub fn validate(&self) -> Result<()> {
        validate_payment_bill_date(&self.bill_date)?;
        validate_payment_fund_flow_account_type(self.account_type.as_deref())?;
        validate_payment_bill_archive_type(self.tar_type.as_deref())
    }

    fn into_query(self) -> Vec<(String, String)> {
        let mut query = vec![("bill_date".to_string(), self.bill_date)];
        if let Some(account_type) = self.account_type {
            query.push(("account_type".to_string(), account_type));
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
    #[serde(default)]
    pub hash_type: Option<String>,
    #[serde(default)]
    pub hash_value: Option<String>,
}

impl BillResponse {
    pub fn validate(&self) -> Result<()> {
        validate_https_url(&self.download_url, "payment bill download URL", 4096, true)?;
        validate_payment_download_hash_metadata(
            self.hash_type.as_deref(),
            self.hash_value.as_deref(),
            true,
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentBillDownloadRequest {
    pub download_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hash_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hash_value: Option<String>,
}

impl PaymentBillDownloadRequest {
    pub fn validate(&self, require_hash: bool) -> Result<()> {
        validate_https_url(&self.download_url, "payment bill download URL", 4096, true)?;
        validate_payment_download_hash_metadata(
            self.hash_type.as_deref(),
            self.hash_value.as_deref(),
            require_hash,
        )
    }
}

impl From<BillResponse> for PaymentBillDownloadRequest {
    fn from(value: BillResponse) -> Self {
        Self {
            download_url: value.download_url,
            hash_type: value.hash_type,
            hash_value: value.hash_value,
        }
    }
}

impl TryFrom<TransferBillReceiptResponse> for PaymentBillDownloadRequest {
    type Error = WechatError;

    fn try_from(value: TransferBillReceiptResponse) -> Result<Self> {
        payment_receipt_download_request(
            "transfer bill receipt",
            &value.signature_status,
            value.download_url,
            value.hash_type,
            value.hash_value,
        )
    }
}

impl TryFrom<TransferDetailReceiptResponse> for PaymentBillDownloadRequest {
    type Error = WechatError;

    fn try_from(value: TransferDetailReceiptResponse) -> Result<Self> {
        payment_receipt_download_request(
            "transfer detail receipt",
            &value.signature_status,
            value.download_url,
            value.hash_type,
            value.hash_value,
        )
    }
}

impl From<FundAppElecSignResponse> for PaymentBillDownloadRequest {
    fn from(value: FundAppElecSignResponse) -> Self {
        Self {
            download_url: value.download_url,
            hash_type: Some(value.hash_type),
            hash_value: Some(value.hash_value),
        }
    }
}

fn payment_receipt_download_request(
    kind: &str,
    status: &str,
    download_url: Option<String>,
    hash_type: Option<String>,
    hash_value: Option<String>,
) -> Result<PaymentBillDownloadRequest> {
    let download_url = download_url
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            WechatError::Config(format!(
                "payment {kind} is not ready for download (status: {status})"
            ))
        })?;
    Ok(PaymentBillDownloadRequest {
        download_url,
        hash_type,
        hash_value,
    })
}

#[derive(Debug, Clone)]
pub struct PaymentDownloadedBill {
    pub bytes: Bytes,
    pub text: String,
    pub hash_type: Option<String>,
    pub hash_value: Option<String>,
    pub line_count: usize,
    pub header: Option<String>,
    pub summary: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaymentDownloadedBillFile {
    pub path: PathBuf,
    pub bytes_written: u64,
    pub hash_type: String,
    pub hash_value: String,
}

impl PaymentDownloadedBillFile {
    pub async fn read_verified(&self, max_bytes: u64) -> Result<PaymentDownloadedBill> {
        if max_bytes == 0 {
            return Err(WechatError::Config(
                "payment bill read maximum bytes must be positive".to_string(),
            ));
        }
        let metadata = tokio::fs::metadata(&self.path).await?;
        if metadata.len() != self.bytes_written {
            return Err(WechatError::Config(format!(
                "payment bill file length changed: expected {}, got {}",
                self.bytes_written,
                metadata.len()
            )));
        }
        if metadata.len() > max_bytes {
            return Err(WechatError::Config(format!(
                "payment bill file exceeds the configured {max_bytes} byte read limit"
            )));
        }

        let bytes = tokio::fs::read(&self.path).await?;
        let actual_len = u64::try_from(bytes.len())
            .map_err(|_| WechatError::Config("payment bill file is too large".to_string()))?;
        if actual_len != self.bytes_written {
            return Err(WechatError::Config(format!(
                "payment bill file length changed while reading: expected {}, got {actual_len}",
                self.bytes_written
            )));
        }
        if actual_len > max_bytes {
            return Err(WechatError::Config(format!(
                "payment bill file exceeds the configured {max_bytes} byte read limit"
            )));
        }
        verify_payment_download_hash(&bytes, Some(&self.hash_type), Some(&self.hash_value))?;
        PaymentDownloadedBill::from_verified_bytes(
            Bytes::from(bytes),
            Some(self.hash_type.clone()),
            Some(self.hash_value.clone()),
        )
    }

    pub async fn statement(&self, max_bytes: u64) -> Result<PaymentBillStatement> {
        self.read_verified(max_bytes).await?.statement()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaymentBillRecord {
    pub raw: String,
    pub fields: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaymentBillStatement {
    pub headers: Vec<String>,
    pub records: Vec<PaymentBillRecord>,
    pub summary: PaymentBillSummary,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaymentBillSummary {
    pub raw: String,
    pub values: Vec<String>,
}

impl PaymentBillRecord {
    pub fn get(&self, name: &str) -> Option<&str> {
        self.fields
            .iter()
            .find_map(|(key, value)| (key == name).then_some(value.as_str()))
    }

    pub fn require(&self, name: &str) -> Result<&str> {
        self.get(name).ok_or_else(|| {
            WechatError::Config(format!("payment bill required field {name} is missing"))
        })
    }

    pub fn get_i64(&self, name: &str) -> Result<Option<i64>> {
        self.get(name)
            .map(|value| {
                value.parse::<i64>().map_err(|err| {
                    WechatError::Config(format!(
                        "payment bill field {name} is not a valid i64: {err}"
                    ))
                })
            })
            .transpose()
    }

    pub fn require_i64(&self, name: &str) -> Result<i64> {
        self.require(name)?.parse::<i64>().map_err(|err| {
            WechatError::Config(format!(
                "payment bill required field {name} is not a valid i64: {err}"
            ))
        })
    }
}

impl PaymentBillSummary {
    pub fn get(&self, index: usize) -> Option<&str> {
        self.values.get(index).map(String::as_str)
    }

    pub fn require(&self, index: usize) -> Result<&str> {
        self.get(index).ok_or_else(|| {
            WechatError::Config(format!("payment bill summary field {index} is missing"))
        })
    }

    pub fn get_i64(&self, index: usize) -> Result<Option<i64>> {
        self.get(index)
            .map(|value| {
                value.parse::<i64>().map_err(|err| {
                    WechatError::Config(format!(
                        "payment bill summary field {index} is not a valid i64: {err}"
                    ))
                })
            })
            .transpose()
    }

    pub fn require_i64(&self, index: usize) -> Result<i64> {
        self.require(index)?.parse::<i64>().map_err(|err| {
            WechatError::Config(format!(
                "payment bill required summary field {index} is not a valid i64: {err}"
            ))
        })
    }
}

impl PaymentBillStatement {
    pub fn column_index(&self, name: &str) -> Option<usize> {
        self.headers.iter().position(|header| header == name)
    }

    pub fn require_columns(&self, names: &[&str]) -> Result<()> {
        let missing = names
            .iter()
            .copied()
            .filter(|name| self.column_index(name).is_none())
            .collect::<Vec<_>>();
        if missing.is_empty() {
            return Ok(());
        }

        Err(WechatError::Config(format!(
            "payment bill required columns are missing: {}",
            missing.join(", ")
        )))
    }

    pub fn sum_i64(&self, name: &str) -> Result<i64> {
        self.records.iter().try_fold(0_i64, |sum, record| {
            let amount = record.get_i64(name)?.unwrap_or_default();
            checked_payment_bill_add(sum, amount, name)
        })
    }

    pub fn non_empty_count(&self, name: &str) -> usize {
        self.records
            .iter()
            .filter(|record| record.get(name).is_some_and(|value| !value.is_empty()))
            .count()
    }

    pub fn records_matching(&self, name: &str, value: &str) -> Result<Vec<&PaymentBillRecord>> {
        self.require_columns(&[name])?;
        Ok(self
            .records
            .iter()
            .filter(|record| record.get(name) == Some(value))
            .collect())
    }

    pub fn records_matching_all(
        &self,
        filters: &[(&str, &str)],
    ) -> Result<Vec<&PaymentBillRecord>> {
        let columns = filters.iter().map(|(name, _)| *name).collect::<Vec<_>>();
        self.require_columns(&columns)?;
        Ok(self
            .records
            .iter()
            .filter(|record| {
                filters
                    .iter()
                    .all(|(name, value)| record.get(name) == Some(*value))
            })
            .collect())
    }

    pub fn record_by_unique_column(
        &self,
        name: &str,
        value: &str,
    ) -> Result<Option<&PaymentBillRecord>> {
        self.require_columns(&[name])?;
        let mut matched = None;
        for record in &self.records {
            if record.get(name) == Some(value) {
                if matched.is_some() {
                    return Err(WechatError::Config(format!(
                        "payment bill unique column {name} contains duplicate value {value}"
                    )));
                }
                matched = Some(record);
            }
        }
        Ok(matched)
    }

    pub fn sum_i64_where(
        &self,
        filter_name: &str,
        filter_value: &str,
        amount_name: &str,
    ) -> Result<i64> {
        self.require_columns(&[filter_name, amount_name])?;
        self.records.iter().try_fold(0_i64, |sum, record| {
            if record.get(filter_name) == Some(filter_value) {
                checked_payment_bill_add(sum, record.require_i64(amount_name)?, amount_name)
            } else {
                Ok(sum)
            }
        })
    }

    pub fn sum_i64_where_all(&self, filters: &[(&str, &str)], amount_name: &str) -> Result<i64> {
        let mut columns = filters.iter().map(|(name, _)| *name).collect::<Vec<_>>();
        columns.push(amount_name);
        self.require_columns(&columns)?;
        self.records.iter().try_fold(0_i64, |sum, record| {
            if filters
                .iter()
                .all(|(name, value)| record.get(name) == Some(*value))
            {
                checked_payment_bill_add(sum, record.require_i64(amount_name)?, amount_name)
            } else {
                Ok(sum)
            }
        })
    }

    pub fn non_empty_count_where(
        &self,
        filter_name: &str,
        filter_value: &str,
        count_name: &str,
    ) -> Result<usize> {
        self.require_columns(&[filter_name, count_name])?;
        Ok(self
            .records
            .iter()
            .filter(|record| {
                record.get(filter_name) == Some(filter_value)
                    && record
                        .get(count_name)
                        .is_some_and(|value| !value.is_empty())
            })
            .count())
    }

    pub fn group_count(&self, group_name: &str) -> Result<BTreeMap<String, usize>> {
        self.require_columns(&[group_name])?;
        let mut counts = BTreeMap::new();
        for record in &self.records {
            let group = record.require(group_name)?.to_string();
            *counts.entry(group).or_insert(0) += 1;
        }
        Ok(counts)
    }

    pub fn group_count_where(
        &self,
        filter_name: &str,
        filter_value: &str,
        group_name: &str,
    ) -> Result<BTreeMap<String, usize>> {
        self.group_count_where_all(&[(filter_name, filter_value)], group_name)
    }

    pub fn group_count_where_all(
        &self,
        filters: &[(&str, &str)],
        group_name: &str,
    ) -> Result<BTreeMap<String, usize>> {
        let mut columns = filters.iter().map(|(name, _)| *name).collect::<Vec<_>>();
        columns.push(group_name);
        self.require_columns(&columns)?;
        let mut counts = BTreeMap::new();
        for record in &self.records {
            if filters
                .iter()
                .all(|(name, value)| record.get(name) == Some(*value))
            {
                let group = record.require(group_name)?.to_string();
                *counts.entry(group).or_insert(0) += 1;
            }
        }
        Ok(counts)
    }

    pub fn group_non_empty_count(
        &self,
        group_name: &str,
        count_name: &str,
    ) -> Result<BTreeMap<String, usize>> {
        self.require_columns(&[group_name, count_name])?;
        let mut counts = BTreeMap::new();
        for record in &self.records {
            let group = record.require(group_name)?.to_string();
            let entry = counts.entry(group).or_insert(0);
            if record
                .get(count_name)
                .is_some_and(|value| !value.is_empty())
            {
                *entry += 1;
            }
        }
        Ok(counts)
    }

    pub fn index_by_unique_column(
        &self,
        name: &str,
    ) -> Result<BTreeMap<String, &PaymentBillRecord>> {
        self.require_columns(&[name])?;
        let mut records = BTreeMap::new();
        for record in &self.records {
            let key = record.require(name)?;
            if key.is_empty() {
                return Err(WechatError::Config(format!(
                    "payment bill unique column {name} contains an empty value"
                )));
            }
            if records.insert(key.to_string(), record).is_some() {
                return Err(WechatError::Config(format!(
                    "payment bill unique column {name} contains duplicate value {key}"
                )));
            }
        }
        Ok(records)
    }

    pub fn group_sum_i64(
        &self,
        group_name: &str,
        amount_name: &str,
    ) -> Result<BTreeMap<String, i64>> {
        self.require_columns(&[group_name, amount_name])?;
        let mut sums = BTreeMap::new();
        for record in &self.records {
            let group = record.require(group_name)?.to_string();
            let amount = record.require_i64(amount_name)?;
            let entry = sums.entry(group).or_insert(0);
            *entry = checked_payment_bill_add(*entry, amount, amount_name)?;
        }
        Ok(sums)
    }

    pub fn group_sum_i64_where(
        &self,
        filter_name: &str,
        filter_value: &str,
        group_name: &str,
        amount_name: &str,
    ) -> Result<BTreeMap<String, i64>> {
        self.group_sum_i64_where_all(&[(filter_name, filter_value)], group_name, amount_name)
    }

    pub fn group_sum_i64_where_all(
        &self,
        filters: &[(&str, &str)],
        group_name: &str,
        amount_name: &str,
    ) -> Result<BTreeMap<String, i64>> {
        let mut columns = filters.iter().map(|(name, _)| *name).collect::<Vec<_>>();
        columns.push(group_name);
        columns.push(amount_name);
        self.require_columns(&columns)?;
        let mut sums = BTreeMap::new();
        for record in &self.records {
            if filters
                .iter()
                .all(|(name, value)| record.get(name) == Some(*value))
            {
                let group = record.require(group_name)?.to_string();
                let amount = record.require_i64(amount_name)?;
                let entry = sums.entry(group).or_insert(0);
                *entry = checked_payment_bill_add(*entry, amount, amount_name)?;
            }
        }
        Ok(sums)
    }

    pub fn assert_sum_matches_summary(&self, name: &str, summary_index: usize) -> Result<i64> {
        self.require_columns(&[name])?;
        let sum = self.sum_i64(name)?;
        let expected = self.summary.require_i64(summary_index)?;
        if sum != expected {
            return Err(WechatError::Config(format!(
                "payment bill column {name} sum {sum} does not match summary field {summary_index} value {expected}"
            )));
        }

        Ok(sum)
    }

    pub fn assert_non_empty_count_matches_summary(
        &self,
        name: &str,
        summary_index: usize,
    ) -> Result<usize> {
        self.require_columns(&[name])?;
        let expected = self.summary.require_i64(summary_index)?;
        let actual = self.non_empty_count(name);
        let actual_i64 = checked_payment_bill_count(actual)?;
        if actual_i64 != expected {
            return Err(WechatError::Config(format!(
                "payment bill column {name} non-empty count {actual} does not match summary field {summary_index} value {expected}"
            )));
        }

        Ok(actual)
    }

    pub fn assert_record_count_matches_summary(&self, summary_index: usize) -> Result<usize> {
        let expected = self.summary.require_i64(summary_index)?;
        let actual = self.records.len();
        let actual_i64 = checked_payment_bill_count(actual)?;
        if actual_i64 != expected {
            return Err(WechatError::Config(format!(
                "payment bill record count {actual} does not match summary field {summary_index} value {expected}"
            )));
        }

        Ok(actual)
    }
}

impl PaymentDownloadedBill {
    pub fn from_verified_bytes(
        bytes: Bytes,
        hash_type: Option<String>,
        hash_value: Option<String>,
    ) -> Result<Self> {
        let text = String::from_utf8(bytes.to_vec()).map_err(|err| {
            WechatError::Config(format!("payment bill download is not valid UTF-8: {err}"))
        })?;
        let mut non_empty_lines = text.lines().filter(|line| !line.trim().is_empty());
        let header = non_empty_lines.next().map(ToString::to_string);
        let summary = non_empty_lines.next_back().map(ToString::to_string);
        let line_count = text.lines().filter(|line| !line.trim().is_empty()).count();

        Ok(Self {
            bytes,
            text,
            hash_type,
            hash_value,
            line_count,
            header,
            summary,
        })
    }

    pub fn rows(&self) -> impl Iterator<Item = &str> {
        self.text.lines().filter(|line| !line.trim().is_empty())
    }

    pub fn data_rows(&self) -> impl Iterator<Item = &str> {
        self.rows()
            .enumerate()
            .filter_map(|(index, line)| (index > 0 && index + 1 < self.line_count).then_some(line))
    }

    pub fn data_records(&self) -> Result<Vec<PaymentBillRecord>> {
        let headers = self.parse_headers()?;
        self.data_records_with_headers(&headers)
    }

    pub fn statement(&self) -> Result<PaymentBillStatement> {
        let headers = self.parse_headers()?;
        let records = self.data_records_with_headers(&headers)?;
        let summary_raw = self
            .summary
            .as_deref()
            .ok_or_else(|| WechatError::Config("payment bill summary is missing".to_string()))?;
        let summary = PaymentBillSummary {
            raw: summary_raw.to_string(),
            values: parse_payment_bill_csv_line(summary_raw)?
                .into_iter()
                .map(clean_payment_bill_cell)
                .collect(),
        };

        Ok(PaymentBillStatement {
            headers,
            records,
            summary,
        })
    }

    fn parse_headers(&self) -> Result<Vec<String>> {
        let header = self
            .header
            .as_deref()
            .ok_or_else(|| WechatError::Config("payment bill header is missing".to_string()))?;
        let headers = parse_payment_bill_csv_line(header)?
            .into_iter()
            .map(clean_payment_bill_cell)
            .collect::<Vec<_>>();
        Ok(headers)
    }

    fn data_records_with_headers(&self, headers: &[String]) -> Result<Vec<PaymentBillRecord>> {
        self.data_rows()
            .map(|row| {
                let values = parse_payment_bill_csv_line(row)?
                    .into_iter()
                    .map(clean_payment_bill_cell)
                    .collect::<Vec<_>>();
                if values.len() != headers.len() {
                    return Err(WechatError::Config(format!(
                        "payment bill row field count mismatch: expected {}, got {}",
                        headers.len(),
                        values.len()
                    )));
                }
                Ok(PaymentBillRecord {
                    raw: row.to_string(),
                    fields: headers.iter().cloned().zip(values).collect(),
                })
            })
            .collect()
    }
}

fn checked_payment_bill_add(total: i64, amount: i64, name: &str) -> Result<i64> {
    total.checked_add(amount).ok_or_else(|| {
        WechatError::Config(format!(
            "payment bill column {name} sum exceeds the supported i64 range"
        ))
    })
}

fn checked_payment_bill_count(count: usize) -> Result<i64> {
    i64::try_from(count).map_err(|_| {
        WechatError::Config("payment bill record count exceeds the supported i64 range".to_string())
    })
}

fn parse_payment_bill_csv_line(line: &str) -> Result<Vec<String>> {
    let mut cells = Vec::new();
    let mut current = String::new();
    let mut chars = line.chars().peekable();
    let mut in_quotes = false;

    while let Some(ch) = chars.next() {
        match ch {
            '"' if in_quotes && chars.peek() == Some(&'"') => {
                current.push('"');
                chars.next();
            }
            '"' => in_quotes = !in_quotes,
            ',' if !in_quotes => {
                cells.push(current);
                current = String::new();
            }
            _ => current.push(ch),
        }
    }

    if in_quotes {
        return Err(WechatError::Config(
            "payment bill CSV row has an unterminated quoted field".to_string(),
        ));
    }

    cells.push(current);
    Ok(cells)
}

fn clean_payment_bill_cell(cell: String) -> String {
    cell.trim_start_matches('\u{feff}')
        .trim_start_matches('`')
        .to_string()
}

fn deserialize_complaint_media_list<'de, D>(
    deserializer: D,
) -> std::result::Result<Vec<ComplaintMedia>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    if value.is_null() {
        return Ok(Vec::new());
    }
    if value.is_array() {
        return serde_json::from_value(value).map_err(serde::de::Error::custom);
    }
    serde_json::from_value(value)
        .map(|media| vec![media])
        .map_err(serde::de::Error::custom)
}

fn deserialize_complaint_media_urls<'de, D>(
    deserializer: D,
) -> std::result::Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    if value.is_null() {
        return Ok(Vec::new());
    }
    if value.is_array() {
        return serde_json::from_value(value).map_err(serde::de::Error::custom);
    }
    serde_json::from_value(value)
        .map(|url| vec![url])
        .map_err(serde::de::Error::custom)
}

fn deserialize_string_list<'de, D>(deserializer: D) -> std::result::Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    if value.is_null() {
        return Ok(Vec::new());
    }
    if value.is_array() {
        return serde_json::from_value(value).map_err(serde::de::Error::custom);
    }
    serde_json::from_value(value)
        .map(|item| vec![item])
        .map_err(serde::de::Error::custom)
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
pub struct MerchantFundBalanceResponse {
    pub available_amount: i64,
    pub pending_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxSignKeyResponse {
    #[serde(rename = "return_code")]
    pub return_code: String,
    #[serde(default, rename = "return_msg")]
    pub return_msg: Option<String>,
    #[serde(default, rename = "mch_id")]
    pub mch_id: Option<String>,
    #[serde(default, rename = "sandbox_signkey")]
    pub sandbox_sign_key: Option<String>,
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

#[derive(Debug, Clone)]
pub struct MerchantMediaUploadRequest {
    pub file_name: String,
    pub sha256: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerchantMediaUploadResponse {
    pub media_id: String,
    #[serde(default, flatten, skip_serializing_if = "Value::is_null")]
    pub extra: Value,
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
    pub fn first_page(
        begin_date: impl Into<String>,
        end_date: impl Into<String>,
        limit: i64,
    ) -> Self {
        Self {
            begin_date: begin_date.into(),
            end_date: end_date.into(),
            limit,
            offset: 0,
            complainted_mchid: None,
        }
    }

    pub fn next_page(
        begin_date: impl Into<String>,
        end_date: impl Into<String>,
        limit: i64,
        offset: i64,
    ) -> Self {
        Self {
            begin_date: begin_date.into(),
            end_date: end_date.into(),
            limit,
            offset,
            complainted_mchid: None,
        }
    }

    pub fn for_complainted_merchant(mut self, merchant_id: impl Into<String>) -> Self {
        self.complainted_mchid = Some(merchant_id.into());
        self
    }

    pub fn validate(&self) -> Result<()> {
        let begin_date =
            chrono::NaiveDate::parse_from_str(&self.begin_date, "%Y-%m-%d").map_err(|_| {
                WechatError::Config(
                    "complaint begin date must use the YYYY-MM-DD format".to_string(),
                )
            })?;
        let end_date =
            chrono::NaiveDate::parse_from_str(&self.end_date, "%Y-%m-%d").map_err(|_| {
                WechatError::Config("complaint end date must use the YYYY-MM-DD format".to_string())
            })?;
        let span = end_date.signed_duration_since(begin_date).num_days();
        if !(0..=30).contains(&span) {
            return Err(WechatError::Config(
                "complaint query dates must be ordered and span at most 30 days".to_string(),
            ));
        }
        if !(1..=50).contains(&self.limit) {
            return Err(WechatError::Config(
                "complaint query limit must be between 1 and 50".to_string(),
            ));
        }
        if self.offset < 0 {
            return Err(WechatError::Config(
                "complaint query offset cannot be negative".to_string(),
            ));
        }
        if let Some(merchant_id) = self.complainted_mchid.as_deref() {
            validate_payment_identifier(merchant_id, "complainted merchant id", 64)?;
        }
        Ok(())
    }

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
    #[serde(default)]
    pub data: Vec<ComplaintDetailResponse>,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub offset: Option<i64>,
    #[serde(default)]
    pub total_count: Option<i64>,
    #[serde(default, flatten)]
    pub extra: Value,
}

impl ComplaintListResponse {
    pub fn validate(&self) -> Result<()> {
        validate_complaint_page(
            self.limit,
            self.offset,
            self.total_count,
            self.data.len(),
            50,
            "complaint list",
        )?;
        let mut complaint_ids = std::collections::HashSet::with_capacity(self.data.len());
        for complaint in &self.data {
            complaint.validate()?;
            let complaint_id = complaint.complaint_id.as_deref().unwrap_or_default();
            if !complaint_ids.insert(complaint_id) {
                return Err(WechatError::Config(
                    "complaint list cannot contain duplicate complaint ids".to_string(),
                ));
            }
        }
        Ok(())
    }

    pub fn has_more(&self) -> Option<bool> {
        payment_page_has_more(self.offset, self.total_count, self.data.len())
    }

    pub fn next_offset(&self) -> Option<i64> {
        payment_page_next_offset(self.offset, self.total_count, self.data.len())
    }

    pub fn identified_count(&self) -> usize {
        self.data
            .iter()
            .filter(|complaint| complaint.has_identity())
            .count()
    }

    pub fn pending_response_count(&self) -> usize {
        self.data
            .iter()
            .filter(|complaint| complaint.needs_merchant_response())
            .count()
    }

    pub fn priority_attention_count(&self) -> usize {
        self.data
            .iter()
            .filter(|complaint| complaint.needs_priority_attention())
            .count()
    }

    pub fn find_complaint(&self, complaint_id: &str) -> Option<&ComplaintDetailResponse> {
        self.data.iter().find(|complaint| {
            complaint
                .complaint_id
                .as_deref()
                .is_some_and(|value| value == complaint_id)
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplaintDetailResponse {
    #[serde(default)]
    pub complaint_id: Option<String>,
    #[serde(default)]
    pub complaint_time: Option<String>,
    #[serde(default)]
    pub complaint_detail: Option<String>,
    #[serde(default)]
    pub complaint_state: Option<String>,
    #[serde(default)]
    pub payer_phone: Option<String>,
    #[serde(default)]
    pub payer_openid: Option<String>,
    #[serde(default)]
    pub complaint_order_info: Vec<ComplaintOrderInfo>,
    #[serde(default)]
    pub complaint_full_refunded: Option<bool>,
    #[serde(default)]
    pub incoming_user_response: Option<bool>,
    #[serde(default)]
    pub user_complaint_times: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_complaint_media_list")]
    pub complaint_media_list: Vec<ComplaintMedia>,
    #[serde(default)]
    pub problem_description: Option<String>,
    #[serde(default)]
    pub problem_type: Option<String>,
    #[serde(default)]
    pub apply_refund_amount: Option<i64>,
    #[serde(default)]
    pub user_tag_list: Vec<String>,
    #[serde(default)]
    pub service_order_info: Vec<ComplaintServiceOrderInfo>,
    #[serde(default)]
    pub additional_info: Option<ComplaintAdditionalInfo>,
    #[serde(default)]
    pub in_platform_service: Option<bool>,
    #[serde(default)]
    pub need_immediate_service: Option<bool>,
    #[serde(default)]
    pub is_agent_mode: Option<bool>,
    #[serde(default, flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplaintStateKind {
    Pending,
    Processing,
    Processed,
    Other,
}

impl ComplaintStateKind {
    pub fn from_code(value: &str) -> Self {
        if value.eq_ignore_ascii_case("PENDING") {
            Self::Pending
        } else if value.eq_ignore_ascii_case("PROCESSING") {
            Self::Processing
        } else if value.eq_ignore_ascii_case("PROCESSED") || value.eq_ignore_ascii_case("COMPLETED")
        {
            Self::Processed
        } else {
            Self::Other
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Processed)
    }
}

impl ComplaintDetailResponse {
    pub fn validate(&self) -> Result<()> {
        let complaint_id = self
            .complaint_id
            .as_deref()
            .ok_or_else(|| WechatError::Config("complaint response id is required".to_string()))?;
        validate_payment_identifier(complaint_id, "complaint response id", 64)?;
        let complaint_time = self.complaint_time.as_deref().ok_or_else(|| {
            WechatError::Config("complaint response time is required".to_string())
        })?;
        validate_payment_rfc3339(complaint_time, "complaint response time")?;
        let complaint_state = self.complaint_state.as_deref().ok_or_else(|| {
            WechatError::Config("complaint response state is required".to_string())
        })?;
        validate_payment_identifier(complaint_state, "complaint response state", 64)?;
        if let Some(complaint_detail) = self.complaint_detail.as_deref() {
            validate_payment_text(complaint_detail, "complaint response detail", 2000)?;
        }
        if let Some(payer_openid) = self.payer_openid.as_deref() {
            validate_payment_identifier(payer_openid, "complaint payer openid", 128)?;
        }
        if let Some(problem_description) = self.problem_description.as_deref() {
            validate_payment_text(problem_description, "complaint problem description", 2000)?;
        }
        if let Some(problem_type) = self.problem_type.as_deref() {
            validate_payment_identifier(problem_type, "complaint problem type", 64)?;
        }

        if self.user_complaint_times.is_some_and(|count| count < 0) {
            return Err(WechatError::Config(
                "complaint response user complaint count cannot be negative".to_string(),
            ));
        }
        if self.apply_refund_amount.is_some_and(|amount| amount < 0) {
            return Err(WechatError::Config(
                "complaint response refund amount cannot be negative".to_string(),
            ));
        }

        let mut order_ids =
            std::collections::HashSet::with_capacity(self.complaint_order_info.len());
        for order in &self.complaint_order_info {
            order.validate()?;
            let identity = (
                order.transaction_id.as_deref().unwrap_or_default(),
                order.out_trade_no.as_deref().unwrap_or_default(),
            );
            if !order_ids.insert(identity) {
                return Err(WechatError::Config(
                    "complaint response cannot contain duplicate order identities".to_string(),
                ));
            }
        }

        for media in &self.complaint_media_list {
            media.validate()?;
        }
        validate_unique_payment_strings(&self.user_tag_list, "complaint response user tag", 100)?;

        let mut service_order_ids =
            std::collections::HashSet::with_capacity(self.service_order_info.len());
        for order in &self.service_order_info {
            order.validate()?;
            let identity = (
                order.order_id.as_deref().unwrap_or_default(),
                order.out_order_no.as_deref().unwrap_or_default(),
            );
            if !service_order_ids.insert(identity) {
                return Err(WechatError::Config(
                    "complaint response cannot contain duplicate service-order identities"
                        .to_string(),
                ));
            }
        }
        if let Some(additional_info) = self.additional_info.as_ref() {
            additional_info.validate()?;
        }
        Ok(())
    }

    pub fn has_identity(&self) -> bool {
        self.complaint_id
            .as_deref()
            .is_some_and(|complaint_id| !complaint_id.trim().is_empty())
    }

    pub fn complaint_state_kind(&self) -> Option<ComplaintStateKind> {
        self.complaint_state
            .as_deref()
            .map(ComplaintStateKind::from_code)
    }

    pub fn problem_type_kind(&self) -> Option<ComplaintProblemType> {
        self.problem_type
            .as_deref()
            .map(ComplaintProblemType::from_code)
    }

    pub fn user_tag_kinds(&self) -> impl Iterator<Item = ComplaintUserTag> + '_ {
        self.user_tag_list
            .iter()
            .map(|value| ComplaintUserTag::from_code(value))
    }

    pub fn decrypt_payer_phone(&self, merchant_private_key_pem: &str) -> Result<Option<String>> {
        self.payer_phone
            .as_deref()
            .map(|ciphertext| {
                crypto::rsa_oaep_sha1_decrypt_base64(merchant_private_key_pem, ciphertext)
            })
            .transpose()
    }

    pub fn is_refund_request(&self) -> bool {
        self.problem_type_kind() == Some(ComplaintProblemType::Refund)
    }

    pub fn is_fully_refunded(&self) -> bool {
        self.complaint_full_refunded == Some(true)
    }

    pub fn has_pending_user_response(&self) -> bool {
        self.incoming_user_response == Some(true)
    }

    pub fn identified_order_count(&self) -> usize {
        self.complaint_order_info
            .iter()
            .filter(|order| order.has_identity())
            .count()
    }

    pub fn known_order_amount(&self) -> Result<i64> {
        self.complaint_order_info
            .iter()
            .filter_map(|order| order.amount)
            .try_fold(0_i64, |total, amount| {
                total.checked_add(amount).ok_or_else(|| {
                    WechatError::Config("complaint order amount total overflowed".to_string())
                })
            })
    }

    pub fn needs_merchant_response(&self) -> bool {
        self.has_pending_user_response()
            && !self
                .complaint_state_kind()
                .is_some_and(ComplaintStateKind::is_terminal)
    }

    pub fn needs_priority_attention(&self) -> bool {
        !self
            .complaint_state_kind()
            .is_some_and(ComplaintStateKind::is_terminal)
            && (self.need_immediate_service == Some(true)
                || self.is_refund_request()
                || self
                    .user_tag_kinds()
                    .any(|kind| kind == ComplaintUserTag::HighRisk))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplaintProblemType {
    Refund,
    ServiceNotWork,
    Others,
    Other,
}

impl ComplaintProblemType {
    pub fn from_code(value: &str) -> Self {
        match value.to_ascii_uppercase().as_str() {
            "REFUND" => Self::Refund,
            "SERVICE_NOT_WORK" => Self::ServiceNotWork,
            "OTHERS" => Self::Others,
            _ => Self::Other,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplaintUserTag {
    Trusted,
    HighRisk,
    Other,
}

impl ComplaintUserTag {
    pub fn from_code(value: &str) -> Self {
        if value.eq_ignore_ascii_case("TRUSTED") {
            Self::Trusted
        } else if value.eq_ignore_ascii_case("HIGH_RISK") {
            Self::HighRisk
        } else {
            Self::Other
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplaintOrderInfo {
    #[serde(default)]
    pub transaction_id: Option<String>,
    #[serde(default)]
    pub out_trade_no: Option<String>,
    #[serde(default)]
    pub amount: Option<i64>,
    #[serde(default, flatten)]
    pub extra: Value,
}

impl ComplaintOrderInfo {
    pub fn validate(&self) -> Result<()> {
        if !self.has_identity() {
            return Err(WechatError::Config(
                "complaint order must include transaction_id or out_trade_no".to_string(),
            ));
        }
        if let Some(transaction_id) = self.transaction_id.as_deref() {
            validate_payment_identifier(transaction_id, "complaint transaction id", 64)?;
        }
        if let Some(out_trade_no) = self.out_trade_no.as_deref() {
            validate_payment_identifier(out_trade_no, "complaint out trade number", 64)?;
        }
        if self.amount.is_some_and(|amount| amount < 0) {
            return Err(WechatError::Config(
                "complaint order amount cannot be negative".to_string(),
            ));
        }
        Ok(())
    }

    pub fn has_identity(&self) -> bool {
        self.transaction_id
            .as_deref()
            .is_some_and(|value| !value.trim().is_empty())
            || self
                .out_trade_no
                .as_deref()
                .is_some_and(|value| !value.trim().is_empty())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplaintMedia {
    #[serde(default)]
    pub media_id: Option<String>,
    #[serde(default)]
    pub media_type: Option<String>,
    #[serde(default, deserialize_with = "deserialize_complaint_media_urls")]
    pub media_url: Vec<String>,
    #[serde(default)]
    pub thumbnail_url: Option<String>,
    #[serde(default, flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplaintMediaType {
    UserComplaintImage,
    OperationImage,
    Image,
    Video,
    Other,
}

impl ComplaintMedia {
    pub fn validate(&self) -> Result<()> {
        if self.media_id.is_none() && self.media_url.is_empty() {
            return Err(WechatError::Config(
                "complaint media must include a media id or URL".to_string(),
            ));
        }
        if let Some(media_id) = self.media_id.as_deref() {
            validate_payment_identifier(media_id, "complaint media id", 512)?;
        }
        validate_unique_payment_urls(&self.media_url, "complaint media URL", 20)?;
        if let Some(thumbnail_url) = self.thumbnail_url.as_deref() {
            validate_https_url(thumbnail_url, "complaint media thumbnail URL", 2048, true)?;
        }
        Ok(())
    }

    pub fn media_kind(&self) -> Option<ComplaintMediaType> {
        self.media_type.as_deref().map(|media_type| {
            if media_type.eq_ignore_ascii_case("USER_COMPLAINT_IMAGE") {
                ComplaintMediaType::UserComplaintImage
            } else if media_type.eq_ignore_ascii_case("OPERATION_IMAGE") {
                ComplaintMediaType::OperationImage
            } else if media_type.eq_ignore_ascii_case("IMAGE") {
                ComplaintMediaType::Image
            } else if media_type.eq_ignore_ascii_case("VIDEO") {
                ComplaintMediaType::Video
            } else {
                ComplaintMediaType::Other
            }
        })
    }

    pub fn is_image(&self) -> bool {
        matches!(
            self.media_kind(),
            Some(
                ComplaintMediaType::UserComplaintImage
                    | ComplaintMediaType::OperationImage
                    | ComplaintMediaType::Image
            )
        )
    }

    pub fn is_video(&self) -> bool {
        self.media_kind() == Some(ComplaintMediaType::Video)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplaintServiceOrderInfo {
    #[serde(default)]
    pub order_id: Option<String>,
    #[serde(default)]
    pub out_order_no: Option<String>,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default, flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplaintServiceOrderStateKind {
    Doing,
    Revoked,
    WaitPay,
    Done,
    Other,
}

impl ComplaintServiceOrderStateKind {
    pub fn from_code(value: &str) -> Self {
        if value.eq_ignore_ascii_case("DOING") {
            Self::Doing
        } else if value.eq_ignore_ascii_case("REVOKED") {
            Self::Revoked
        } else if value.eq_ignore_ascii_case("WAITPAY") {
            Self::WaitPay
        } else if value.eq_ignore_ascii_case("DONE") {
            Self::Done
        } else {
            Self::Other
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Revoked | Self::Done)
    }
}

impl ComplaintServiceOrderInfo {
    pub fn validate(&self) -> Result<()> {
        if self.order_id.is_none() && self.out_order_no.is_none() {
            return Err(WechatError::Config(
                "complaint service order must include order_id or out_order_no".to_string(),
            ));
        }
        if let Some(order_id) = self.order_id.as_deref() {
            validate_payment_identifier(order_id, "complaint service order id", 64)?;
        }
        if let Some(out_order_no) = self.out_order_no.as_deref() {
            validate_payment_identifier(out_order_no, "complaint service out order number", 64)?;
        }
        if let Some(state) = self.state.as_deref() {
            validate_payment_identifier(state, "complaint service order state", 64)?;
        }
        Ok(())
    }

    pub fn state_kind(&self) -> Option<ComplaintServiceOrderStateKind> {
        self.state
            .as_deref()
            .map(ComplaintServiceOrderStateKind::from_code)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplaintAdditionalInfo {
    #[serde(default, rename = "type")]
    pub info_type: Option<String>,
    #[serde(default)]
    pub share_power_info: Option<ComplaintSharePowerInfo>,
    #[serde(default, flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplaintAdditionalInfoType {
    SharePower,
    Other,
}

impl ComplaintAdditionalInfoType {
    pub fn from_code(value: &str) -> Self {
        if value.eq_ignore_ascii_case("SHARE_POWER_TYPE")
            || value.eq_ignore_ascii_case("SHARE_POWER_BANK")
        {
            Self::SharePower
        } else {
            Self::Other
        }
    }
}

impl ComplaintAdditionalInfo {
    pub fn validate(&self) -> Result<()> {
        if let Some(info_type) = self.info_type.as_deref() {
            validate_payment_identifier(info_type, "complaint additional-info type", 64)?;
        }
        if self.info_type_kind() == Some(ComplaintAdditionalInfoType::SharePower)
            && self.share_power_info.is_none()
        {
            return Err(WechatError::Config(
                "complaint share-power additional info requires share_power_info".to_string(),
            ));
        }
        if let Some(share_power_info) = self.share_power_info.as_ref() {
            share_power_info.validate()?;
        }
        Ok(())
    }

    pub fn info_type_kind(&self) -> Option<ComplaintAdditionalInfoType> {
        self.info_type
            .as_deref()
            .map(ComplaintAdditionalInfoType::from_code)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplaintSharePowerInfo {
    #[serde(default)]
    pub return_time: Option<String>,
    #[serde(default)]
    pub return_address_info: Option<ComplaintReturnAddressInfo>,
    #[serde(default)]
    pub is_returned_to_same_machine: Option<bool>,
    #[serde(default, flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplaintReturnAddressInfo {
    #[serde(default)]
    pub return_address: Option<String>,
    #[serde(default)]
    pub longitude: Option<String>,
    #[serde(default)]
    pub latitude: Option<String>,
    #[serde(default, flatten)]
    pub extra: Value,
}

impl ComplaintSharePowerInfo {
    pub fn validate(&self) -> Result<()> {
        let return_time = self.return_time.as_deref().ok_or_else(|| {
            WechatError::Config("complaint share-power return time is required".to_string())
        })?;
        validate_payment_rfc3339(return_time, "complaint share-power return time")?;
        let address = self.return_address_info.as_ref().ok_or_else(|| {
            WechatError::Config("complaint share-power return address is required".to_string())
        })?;
        address.validate()
    }
}

impl ComplaintReturnAddressInfo {
    pub fn validate(&self) -> Result<()> {
        let address = self.return_address.as_deref().ok_or_else(|| {
            WechatError::Config("complaint return address text is required".to_string())
        })?;
        validate_payment_text(address, "complaint return address", 512)?;
        let longitude = parse_complaint_coordinate(
            self.longitude.as_deref(),
            "complaint return longitude",
            -180.0,
            180.0,
        )?;
        let latitude = parse_complaint_coordinate(
            self.latitude.as_deref(),
            "complaint return latitude",
            -90.0,
            90.0,
        )?;
        if !longitude.is_finite() || !latitude.is_finite() {
            return Err(WechatError::Config(
                "complaint return coordinates must be finite".to_string(),
            ));
        }
        Ok(())
    }
}

fn parse_complaint_coordinate(
    value: Option<&str>,
    field: &str,
    minimum: f64,
    maximum: f64,
) -> Result<f64> {
    let value = value.ok_or_else(|| WechatError::Config(format!("{field} is required")))?;
    let parsed = value
        .parse::<f64>()
        .map_err(|_| WechatError::Config(format!("{field} must be numeric")))?;
    if !parsed.is_finite() || !(minimum..=maximum).contains(&parsed) {
        return Err(WechatError::Config(format!(
            "{field} must be between {minimum} and {maximum}"
        )));
    }
    Ok(parsed)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplaintNegotiationHistoryRequest {
    pub limit: i64,
    pub offset: i64,
}

impl ComplaintNegotiationHistoryRequest {
    pub const fn first_page(limit: i64) -> Self {
        Self { limit, offset: 0 }
    }

    pub const fn next_page(limit: i64, offset: i64) -> Self {
        Self { limit, offset }
    }

    pub fn validate(&self) -> Result<()> {
        if !(1..=300).contains(&self.limit) {
            return Err(WechatError::Config(
                "complaint negotiation-history limit must be between 1 and 300".to_string(),
            ));
        }
        if self.offset < 0 {
            return Err(WechatError::Config(
                "complaint negotiation-history offset cannot be negative".to_string(),
            ));
        }
        Ok(())
    }

    fn into_query(self) -> Vec<(String, String)> {
        vec![
            ("limit".to_string(), self.limit.to_string()),
            ("offset".to_string(), self.offset.to_string()),
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplaintNegotiationHistoryResponse {
    #[serde(default)]
    pub data: Vec<ComplaintNegotiationHistoryRecord>,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub offset: Option<i64>,
    #[serde(default)]
    pub total_count: Option<i64>,
    #[serde(default, flatten)]
    pub extra: Value,
}

impl ComplaintNegotiationHistoryResponse {
    pub fn validate(&self) -> Result<()> {
        validate_complaint_page(
            self.limit,
            self.offset,
            self.total_count,
            self.data.len(),
            300,
            "complaint negotiation history",
        )?;
        let mut log_ids = std::collections::HashSet::with_capacity(self.data.len());
        for record in &self.data {
            record.validate()?;
            let log_id = record.log_id.as_deref().unwrap_or_default();
            if !log_ids.insert(log_id) {
                return Err(WechatError::Config(
                    "complaint negotiation history cannot contain duplicate log ids".to_string(),
                ));
            }
        }
        Ok(())
    }

    pub fn has_more(&self) -> Option<bool> {
        payment_page_has_more(self.offset, self.total_count, self.data.len())
    }

    pub fn next_offset(&self) -> Option<i64> {
        payment_page_next_offset(self.offset, self.total_count, self.data.len())
    }

    pub fn identified_count(&self) -> usize {
        self.data
            .iter()
            .filter(|record| record.has_identity())
            .count()
    }

    pub fn refund_event_count(&self) -> usize {
        self.data
            .iter()
            .filter(|record| {
                record
                    .operate_type_kind()
                    .is_some_and(ComplaintNegotiationOperateType::is_refund_event)
            })
            .count()
    }

    pub fn system_event_count(&self) -> usize {
        self.data
            .iter()
            .filter(|record| {
                record
                    .operate_type_kind()
                    .is_some_and(ComplaintNegotiationOperateType::is_system_event)
            })
            .count()
    }

    pub fn find_log(&self, log_id: &str) -> Option<&ComplaintNegotiationHistoryRecord> {
        self.data.iter().find(|record| {
            record
                .log_id
                .as_deref()
                .is_some_and(|value| value == log_id)
        })
    }
}

fn payment_page_has_more(
    offset: Option<i64>,
    total_count: Option<i64>,
    returned: usize,
) -> Option<bool> {
    let returned = i64::try_from(returned).ok()?;
    Some(offset?.checked_add(returned)? < total_count?)
}

fn payment_page_next_offset(
    offset: Option<i64>,
    total_count: Option<i64>,
    returned: usize,
) -> Option<i64> {
    if returned == 0 {
        return None;
    }
    let returned = i64::try_from(returned).ok()?;
    let next = offset?.checked_add(returned)?;
    (next < total_count?).then_some(next)
}

fn validate_complaint_page(
    limit: Option<i64>,
    offset: Option<i64>,
    total_count: Option<i64>,
    returned: usize,
    max_limit: i64,
    field: &str,
) -> Result<()> {
    let limit = limit.ok_or_else(|| WechatError::Config(format!("{field} limit is required")))?;
    if !(1..=max_limit).contains(&limit) {
        return Err(WechatError::Config(format!(
            "{field} limit must be between 1 and {max_limit}"
        )));
    }
    let offset =
        offset.ok_or_else(|| WechatError::Config(format!("{field} offset is required")))?;
    let total_count = total_count
        .ok_or_else(|| WechatError::Config(format!("{field} total count is required")))?;
    if offset < 0 || total_count < 0 {
        return Err(WechatError::Config(format!(
            "{field} offset and total count cannot be negative"
        )));
    }
    let returned = i64::try_from(returned)
        .map_err(|_| WechatError::Config(format!("{field} returned item count overflowed")))?;
    if returned > limit {
        return Err(WechatError::Config(format!(
            "{field} returned more items than its page limit"
        )));
    }
    let end = offset
        .checked_add(returned)
        .ok_or_else(|| WechatError::Config(format!("{field} page boundary overflowed")))?;
    if offset > total_count || end > total_count {
        return Err(WechatError::Config(format!(
            "{field} pagination is inconsistent with total count"
        )));
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplaintNegotiationHistoryRecord {
    #[serde(default)]
    pub log_id: Option<String>,
    #[serde(default)]
    pub operator: Option<String>,
    #[serde(default)]
    pub operate_time: Option<String>,
    #[serde(default)]
    pub operate_type: Option<String>,
    #[serde(default)]
    pub operate_details: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_list")]
    pub image_list: Vec<String>,
    #[serde(default, deserialize_with = "deserialize_complaint_media_list")]
    pub complaint_media_list: Vec<ComplaintMedia>,
    #[serde(default)]
    pub user_appy_platform_service_reason: Option<String>,
    #[serde(default)]
    pub user_appy_platform_service_reason_description: Option<String>,
    #[serde(default)]
    pub normal_message: Option<ComplaintNormalMessage>,
    #[serde(default)]
    pub click_message: Option<ComplaintClickMessage>,
    #[serde(default, flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplaintNegotiationOperateType {
    UserCreateComplaint,
    UserContinueComplaint,
    UserResponse,
    PlatformResponse,
    MerchantResponse,
    MerchantConfirmComplete,
    UserApplyPlatformService,
    UserCancelPlatformService,
    PlatformServiceFinished,
    MerchantApproveRefund,
    MerchantRejectRefund,
    UserRevokeComplaint,
    UserConfirmComplaint,
    PlatformHelpApplication,
    UserApplyPlatformHelp,
    UserSubmitSatisfaction,
    ServiceOrderCancel,
    ServiceOrderComplete,
    ComplaintFullRefunded,
    ComplaintPartialRefunded,
    ComplaintRefundReceived,
    ComplaintEntrustedRefund,
    UserClickResponse,
    SystemMessage,
    Other,
}

impl ComplaintNegotiationOperateType {
    pub fn from_code(value: &str) -> Self {
        match value.to_ascii_uppercase().as_str() {
            "USER_CREATE_COMPLAINT" => Self::UserCreateComplaint,
            "USER_CONTINUE_COMPLAINT" => Self::UserContinueComplaint,
            "USER_RESPONSE" => Self::UserResponse,
            "PLATFORM_RESPONSE" => Self::PlatformResponse,
            "MERCHANT_RESPONSE" => Self::MerchantResponse,
            "MERCHANT_CONFIRM_COMPLETE" => Self::MerchantConfirmComplete,
            "USER_APPLY_PLATFORM_SERVICE" => Self::UserApplyPlatformService,
            "USER_CANCEL_PLATFORM_SERVICE" => Self::UserCancelPlatformService,
            "PLATFORM_SERVICE_FINISHED" => Self::PlatformServiceFinished,
            "MERCHANT_APPROVE_REFUND" => Self::MerchantApproveRefund,
            "MERCHANT_REFUSE_RERUND" | "MERCHANT_REJECT_REFUND" => Self::MerchantRejectRefund,
            "USER_REVOKE_COMPLAINT" => Self::UserRevokeComplaint,
            "USER_COMFIRM_COMPLAINT" | "USER_CONFIRM_COMPLAINT" => Self::UserConfirmComplaint,
            "PLATFORM_HELP_APPLICATION" => Self::PlatformHelpApplication,
            "USER_APPLY_PLATFORM_HELP" => Self::UserApplyPlatformHelp,
            "USER_SUBMIT_SATISFACTION" => Self::UserSubmitSatisfaction,
            "SERVICE_ORDER_CANCEL" => Self::ServiceOrderCancel,
            "SERVICE_ORDER_COMPLETE" => Self::ServiceOrderComplete,
            "COMPLAINT_FULL_REFUNDED_SYSTEM_MESSAGE" => Self::ComplaintFullRefunded,
            "COMPLAINT_PARTIAL_REFUNDED_SYSTEM_MESSAGE" => Self::ComplaintPartialRefunded,
            "COMPLAINT_REFUND_RECEIVED_SYSTEM_MESSAGE" => Self::ComplaintRefundReceived,
            "COMPLAINT_ENTRUSTED_REFUND_SYSTEM_MESSAGE" => Self::ComplaintEntrustedRefund,
            "USER_CLICK_RESPONSE" => Self::UserClickResponse,
            value if value.ends_with("_SYSTEM_MESSAGE") => Self::SystemMessage,
            _ => Self::Other,
        }
    }

    pub fn is_refund_event(self) -> bool {
        matches!(
            self,
            Self::MerchantApproveRefund
                | Self::MerchantRejectRefund
                | Self::ComplaintFullRefunded
                | Self::ComplaintPartialRefunded
                | Self::ComplaintRefundReceived
                | Self::ComplaintEntrustedRefund
        )
    }

    pub fn is_system_event(self) -> bool {
        matches!(
            self,
            Self::ComplaintFullRefunded
                | Self::ComplaintPartialRefunded
                | Self::ComplaintRefundReceived
                | Self::ComplaintEntrustedRefund
                | Self::SystemMessage
        )
    }
}

impl ComplaintNegotiationHistoryRecord {
    pub fn validate(&self) -> Result<()> {
        let log_id = self.log_id.as_deref().ok_or_else(|| {
            WechatError::Config("complaint negotiation log id is required".to_string())
        })?;
        validate_payment_identifier(log_id, "complaint negotiation log id", 64)?;
        let operator = self.operator.as_deref().ok_or_else(|| {
            WechatError::Config("complaint negotiation operator is required".to_string())
        })?;
        validate_payment_identifier(operator, "complaint negotiation operator", 64)?;
        let operate_time = self.operate_time.as_deref().ok_or_else(|| {
            WechatError::Config("complaint negotiation operation time is required".to_string())
        })?;
        validate_payment_rfc3339(operate_time, "complaint negotiation operation time")?;
        let operate_type = self.operate_type.as_deref().ok_or_else(|| {
            WechatError::Config("complaint negotiation operation type is required".to_string())
        })?;
        validate_payment_identifier(operate_type, "complaint negotiation operation type", 64)?;
        validate_unique_payment_urls(&self.image_list, "complaint negotiation image URL", 20)?;
        for media in &self.complaint_media_list {
            media.validate()?;
        }
        if let Some(normal_message) = self.normal_message.as_ref() {
            normal_message.validate()?;
        }
        if let Some(click_message) = self.click_message.as_ref() {
            click_message.validate()?;
        }
        if self.operate_type_kind() == Some(ComplaintNegotiationOperateType::UserClickResponse)
            && self.click_message.is_none()
        {
            return Err(WechatError::Config(
                "complaint user-click operation requires click_message".to_string(),
            ));
        }
        Ok(())
    }

    pub fn has_identity(&self) -> bool {
        self.log_id
            .as_deref()
            .is_some_and(|log_id| !log_id.trim().is_empty())
    }

    pub fn operate_type_kind(&self) -> Option<ComplaintNegotiationOperateType> {
        self.operate_type
            .as_deref()
            .map(ComplaintNegotiationOperateType::from_code)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplaintNormalMessage {
    #[serde(default)]
    pub blocks: Vec<ComplaintMessageBlock>,
    #[serde(default)]
    pub sender_identity: Option<String>,
    #[serde(default)]
    pub custom_data: Option<String>,
    #[serde(default, flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplaintMessageSenderIdentity {
    Unknown,
    Manual,
    Machine,
    Other,
}

impl ComplaintMessageSenderIdentity {
    pub fn from_code(value: &str) -> Self {
        match value.to_ascii_uppercase().as_str() {
            "UNKNOWN" => Self::Unknown,
            "MANUAL" => Self::Manual,
            "MACHINE" => Self::Machine,
            _ => Self::Other,
        }
    }
}

impl ComplaintNormalMessage {
    pub fn validate(&self) -> Result<()> {
        if self.blocks.is_empty() || self.blocks.len() > 100 {
            return Err(WechatError::Config(
                "complaint normal message must contain 1 to 100 blocks".to_string(),
            ));
        }
        if let Some(sender_identity) = self.sender_identity.as_deref() {
            validate_payment_identifier(sender_identity, "complaint message sender identity", 64)?;
        }
        if let Some(custom_data) = self.custom_data.as_deref() {
            validate_payment_text(custom_data, "complaint message custom data", 2048)?;
        }
        for block in &self.blocks {
            block.validate()?;
        }
        Ok(())
    }

    pub fn sender_identity_kind(&self) -> Option<ComplaintMessageSenderIdentity> {
        self.sender_identity
            .as_deref()
            .map(ComplaintMessageSenderIdentity::from_code)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplaintClickMessage {
    #[serde(default)]
    pub message_content: Option<String>,
    #[serde(default)]
    pub action_id: Option<String>,
    #[serde(default)]
    pub clicked_log_id: Option<String>,
    #[serde(default, flatten)]
    pub extra: Value,
}

impl ComplaintClickMessage {
    pub fn validate(&self) -> Result<()> {
        if let Some(message_content) = self.message_content.as_deref() {
            validate_payment_text(message_content, "complaint click message content", 2000)?;
        }
        let action_id = self.action_id.as_deref().ok_or_else(|| {
            WechatError::Config("complaint click message action id is required".to_string())
        })?;
        validate_payment_identifier(action_id, "complaint click message action id", 128)?;
        let clicked_log_id = self.clicked_log_id.as_deref().ok_or_else(|| {
            WechatError::Config("complaint click message log id is required".to_string())
        })?;
        validate_payment_identifier(clicked_log_id, "complaint click message log id", 64)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplaintMessageBlock {
    #[serde(default, rename = "type")]
    pub block_type: Option<String>,
    #[serde(default)]
    pub text: Option<ComplaintMessageText>,
    #[serde(default)]
    pub image: Option<ComplaintMessageImage>,
    #[serde(default)]
    pub link: Option<ComplaintMessageLink>,
    #[serde(default)]
    pub faq_list: Option<ComplaintMessageFaqList>,
    #[serde(default)]
    pub button: Option<ComplaintMessageButton>,
    #[serde(default)]
    pub button_group: Option<ComplaintMessageButtonGroup>,
    #[serde(default, flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplaintMessageBlockType {
    Text,
    Image,
    Link,
    FaqList,
    Button,
    ButtonGroup,
    Other,
}

impl ComplaintMessageBlockType {
    pub fn from_code(value: &str) -> Self {
        match value.to_ascii_uppercase().as_str() {
            "TEXT" => Self::Text,
            "IMAGE" => Self::Image,
            "LINK" => Self::Link,
            "FAQ_LIST" => Self::FaqList,
            "BUTTON" => Self::Button,
            "BUTTON_GROUP" => Self::ButtonGroup,
            _ => Self::Other,
        }
    }
}

impl ComplaintMessageBlock {
    pub fn validate(&self) -> Result<()> {
        let block_type = self.block_type.as_deref().ok_or_else(|| {
            WechatError::Config("complaint message block type is required".to_string())
        })?;
        validate_payment_identifier(block_type, "complaint message block type", 64)?;
        match self.block_type_kind() {
            Some(ComplaintMessageBlockType::Text) => self
                .text
                .as_ref()
                .ok_or_else(|| {
                    WechatError::Config("complaint text block requires a text payload".to_string())
                })?
                .validate(),
            Some(ComplaintMessageBlockType::Image) => self
                .image
                .as_ref()
                .ok_or_else(|| {
                    WechatError::Config(
                        "complaint image block requires an image payload".to_string(),
                    )
                })?
                .validate(),
            Some(ComplaintMessageBlockType::Link) => self
                .link
                .as_ref()
                .ok_or_else(|| {
                    WechatError::Config("complaint link block requires a link payload".to_string())
                })?
                .validate(),
            Some(ComplaintMessageBlockType::FaqList) => self
                .faq_list
                .as_ref()
                .ok_or_else(|| {
                    WechatError::Config(
                        "complaint FAQ block requires an faq_list payload".to_string(),
                    )
                })?
                .validate(),
            Some(ComplaintMessageBlockType::Button) => self
                .button
                .as_ref()
                .ok_or_else(|| {
                    WechatError::Config(
                        "complaint button block requires a button payload".to_string(),
                    )
                })?
                .validate(),
            Some(ComplaintMessageBlockType::ButtonGroup) => self
                .button_group
                .as_ref()
                .ok_or_else(|| {
                    WechatError::Config(
                        "complaint button-group block requires a button_group payload".to_string(),
                    )
                })?
                .validate(),
            Some(ComplaintMessageBlockType::Other) | None => Ok(()),
        }
    }

    pub fn block_type_kind(&self) -> Option<ComplaintMessageBlockType> {
        self.block_type
            .as_deref()
            .map(ComplaintMessageBlockType::from_code)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplaintMessageText {
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub is_bold: Option<bool>,
    #[serde(default, flatten)]
    pub extra: Value,
}

impl ComplaintMessageText {
    pub fn validate(&self) -> Result<()> {
        let text = self
            .text
            .as_deref()
            .ok_or_else(|| WechatError::Config("complaint message text is required".to_string()))?;
        validate_payment_text(text, "complaint message text", 2000)?;
        if let Some(color) = self.color.as_deref() {
            validate_payment_identifier(color, "complaint message text color", 32)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplaintMessageImage {
    #[serde(default)]
    pub media_id: Option<String>,
    #[serde(default)]
    pub image_style_type: Option<String>,
    #[serde(default, flatten)]
    pub extra: Value,
}

impl ComplaintMessageImage {
    pub fn validate(&self) -> Result<()> {
        let media_id = self.media_id.as_deref().ok_or_else(|| {
            WechatError::Config("complaint message image media id is required".to_string())
        })?;
        validate_payment_identifier(media_id, "complaint message image media id", 512)?;
        if let Some(style) = self.image_style_type.as_deref() {
            validate_payment_identifier(style, "complaint message image style", 64)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplaintMessageLink {
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub action: Option<ComplaintMessageAction>,
    #[serde(default)]
    pub invalid_info: Option<ComplaintMessageInvalidInfo>,
    #[serde(default, flatten)]
    pub extra: Value,
}

impl ComplaintMessageLink {
    pub fn validate(&self) -> Result<()> {
        let text = self.text.as_deref().ok_or_else(|| {
            WechatError::Config("complaint message link text is required".to_string())
        })?;
        validate_payment_text(text, "complaint message link text", 512)?;
        self.action
            .as_ref()
            .ok_or_else(|| {
                WechatError::Config("complaint message link action is required".to_string())
            })?
            .validate()?;
        if let Some(invalid_info) = self.invalid_info.as_ref() {
            invalid_info.validate()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplaintMessageFaqList {
    #[serde(default)]
    pub faqs: Vec<ComplaintMessageFaq>,
    #[serde(default, flatten)]
    pub extra: Value,
}

impl ComplaintMessageFaqList {
    pub fn validate(&self) -> Result<()> {
        if self.faqs.is_empty() || self.faqs.len() > 100 {
            return Err(WechatError::Config(
                "complaint FAQ list must contain 1 to 100 items".to_string(),
            ));
        }
        let mut ids = std::collections::HashSet::with_capacity(self.faqs.len());
        for faq in &self.faqs {
            faq.validate()?;
            let faq_id = faq.faq_id.as_deref().unwrap_or_default();
            if !ids.insert(faq_id) {
                return Err(WechatError::Config(
                    "complaint FAQ list cannot contain duplicate faq ids".to_string(),
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplaintMessageFaq {
    #[serde(default)]
    pub faq_id: Option<String>,
    #[serde(default)]
    pub faq_title: Option<String>,
    #[serde(default)]
    pub action: Option<ComplaintMessageAction>,
    #[serde(default, flatten)]
    pub extra: Value,
}

impl ComplaintMessageFaq {
    pub fn validate(&self) -> Result<()> {
        let faq_id = self
            .faq_id
            .as_deref()
            .ok_or_else(|| WechatError::Config("complaint FAQ id is required".to_string()))?;
        validate_payment_identifier(faq_id, "complaint FAQ id", 128)?;
        let title = self
            .faq_title
            .as_deref()
            .ok_or_else(|| WechatError::Config("complaint FAQ title is required".to_string()))?;
        validate_payment_text(title, "complaint FAQ title", 512)?;
        self.action
            .as_ref()
            .ok_or_else(|| WechatError::Config("complaint FAQ action is required".to_string()))?
            .validate()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplaintMessageButton {
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub action: Option<ComplaintMessageAction>,
    #[serde(default)]
    pub invalid_info: Option<ComplaintMessageInvalidInfo>,
    #[serde(default, flatten)]
    pub extra: Value,
}

impl ComplaintMessageButton {
    pub fn validate(&self) -> Result<()> {
        let text = self.text.as_deref().ok_or_else(|| {
            WechatError::Config("complaint message button text is required".to_string())
        })?;
        validate_payment_text(text, "complaint message button text", 512)?;
        self.action
            .as_ref()
            .ok_or_else(|| {
                WechatError::Config("complaint message button action is required".to_string())
            })?
            .validate()?;
        if let Some(invalid_info) = self.invalid_info.as_ref() {
            invalid_info.validate()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplaintMessageButtonGroup {
    #[serde(default)]
    pub buttons: Vec<ComplaintMessageButton>,
    #[serde(default)]
    pub button_layout: Option<String>,
    #[serde(default)]
    pub invalid_info: Option<ComplaintMessageInvalidInfo>,
    #[serde(default, flatten)]
    pub extra: Value,
}

impl ComplaintMessageButtonGroup {
    pub fn validate(&self) -> Result<()> {
        if self.buttons.is_empty() || self.buttons.len() > 20 {
            return Err(WechatError::Config(
                "complaint button group must contain 1 to 20 buttons".to_string(),
            ));
        }
        let mut action_ids = std::collections::HashSet::with_capacity(self.buttons.len());
        for button in &self.buttons {
            button.validate()?;
            if let Some(action_id) = button
                .action
                .as_ref()
                .and_then(|action| action.action_id.as_deref())
            {
                if !action_ids.insert(action_id) {
                    return Err(WechatError::Config(
                        "complaint button group cannot contain duplicate action ids".to_string(),
                    ));
                }
            }
        }
        if let Some(layout) = self.button_layout.as_deref() {
            validate_payment_identifier(layout, "complaint button-group layout", 64)?;
        }
        if let Some(invalid_info) = self.invalid_info.as_ref() {
            invalid_info.validate()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplaintMessageAction {
    #[serde(default)]
    pub action_type: Option<String>,
    #[serde(default)]
    pub jump_url: Option<String>,
    #[serde(default)]
    pub mini_program_jump_info: Option<ComplaintMiniProgramJumpInfo>,
    #[serde(default)]
    pub message_info: Option<ComplaintMessageInfo>,
    #[serde(default)]
    pub action_id: Option<String>,
    #[serde(default, flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplaintMessageActionType {
    SendMessage,
    JumpUrl,
    JumpMiniProgram,
    Other,
}

impl ComplaintMessageActionType {
    pub fn from_code(value: &str) -> Self {
        match value.to_ascii_uppercase().as_str() {
            "ACTION_TYPE_SEND_MESSAGE" => Self::SendMessage,
            "ACTION_TYPE_JUMP_URL" => Self::JumpUrl,
            "ACTION_TYPE_JUMP_MINI_PROGRAM" => Self::JumpMiniProgram,
            _ => Self::Other,
        }
    }
}

impl ComplaintMessageAction {
    pub fn validate(&self) -> Result<()> {
        let action_type = self.action_type.as_deref().ok_or_else(|| {
            WechatError::Config("complaint message action type is required".to_string())
        })?;
        validate_payment_identifier(action_type, "complaint message action type", 64)?;
        if let Some(action_id) = self.action_id.as_deref() {
            validate_payment_identifier(action_id, "complaint message action id", 128)?;
        }
        match self.action_type_kind() {
            Some(ComplaintMessageActionType::SendMessage) => self
                .message_info
                .as_ref()
                .ok_or_else(|| {
                    WechatError::Config(
                        "complaint send-message action requires message_info".to_string(),
                    )
                })?
                .validate(),
            Some(ComplaintMessageActionType::JumpUrl) => {
                let jump_url = self.jump_url.as_deref().ok_or_else(|| {
                    WechatError::Config("complaint jump-URL action requires jump_url".to_string())
                })?;
                validate_https_url(jump_url, "complaint message jump URL", 2048, true)
            }
            Some(ComplaintMessageActionType::JumpMiniProgram) => self
                .mini_program_jump_info
                .as_ref()
                .ok_or_else(|| {
                    WechatError::Config(
                        "complaint mini-program action requires mini_program_jump_info".to_string(),
                    )
                })?
                .validate_response(),
            Some(ComplaintMessageActionType::Other) | None => Ok(()),
        }
    }

    pub fn action_type_kind(&self) -> Option<ComplaintMessageActionType> {
        self.action_type
            .as_deref()
            .map(ComplaintMessageActionType::from_code)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplaintMessageInvalidInfo {
    #[serde(default)]
    pub expired_time: Option<String>,
    #[serde(default)]
    pub multi_clickable: Option<bool>,
    #[serde(default, flatten)]
    pub extra: Value,
}

impl ComplaintMessageInvalidInfo {
    pub fn validate(&self) -> Result<()> {
        if let Some(expired_time) = self.expired_time.as_deref() {
            validate_payment_rfc3339(expired_time, "complaint message expiration time")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplaintMiniProgramJumpInfo {
    pub appid: String,
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, flatten)]
    pub extra: Value,
}

impl ComplaintMiniProgramJumpInfo {
    pub fn validate(&self) -> Result<()> {
        validate_payment_text(&self.appid, "complaint mini-program appid", 128)?;
        validate_payment_text(&self.path, "complaint mini-program path", 1024)?;
        let text = self.text.as_deref().ok_or_else(|| {
            WechatError::Config("complaint mini-program jump text is required".to_string())
        })?;
        validate_payment_text(text, "complaint mini-program jump text", 128)
    }

    fn validate_response(&self) -> Result<()> {
        validate_payment_identifier(&self.appid, "complaint mini-program appid", 128)?;
        validate_payment_text(&self.path, "complaint mini-program path", 1024)?;
        if let Some(text) = self.text.as_deref() {
            validate_payment_text(text, "complaint mini-program jump text", 128)?;
        }
        Ok(())
    }
}

fn payment_path_identifier<'a>(value: &'a str, field: &str, max_chars: usize) -> Result<&'a str> {
    validate_payment_identifier(value, field, max_chars)?;
    Ok(value)
}

fn encode_payment_path_segment(value: &str) -> String {
    url::form_urlencoded::byte_serialize(value.as_bytes()).collect()
}

fn validate_payment_text(value: &str, field: &str, max_chars: usize) -> Result<()> {
    let count = value.chars().count();
    if value.trim().is_empty() || count > max_chars {
        return Err(WechatError::Config(format!(
            "{field} must contain between 1 and {max_chars} characters"
        )));
    }
    if value.chars().any(char::is_control) {
        return Err(WechatError::Config(format!(
            "{field} cannot contain control characters"
        )));
    }
    Ok(())
}

fn validate_payment_identifier(value: &str, field: &str, max_chars: usize) -> Result<()> {
    validate_payment_text(value, field, max_chars)?;
    if value.trim() != value || value.chars().any(char::is_whitespace) {
        return Err(WechatError::Config(format!(
            "{field} cannot contain whitespace"
        )));
    }
    Ok(())
}

fn validate_unique_payment_strings(values: &[String], field: &str, max_items: usize) -> Result<()> {
    if values.len() > max_items {
        return Err(WechatError::Config(format!(
            "{field} list cannot contain more than {max_items} items"
        )));
    }
    let mut seen = std::collections::HashSet::with_capacity(values.len());
    for value in values {
        validate_payment_identifier(value, field, 512)?;
        if !seen.insert(value.as_str()) {
            return Err(WechatError::Config(format!(
                "{field} list cannot contain duplicates"
            )));
        }
    }
    Ok(())
}

fn validate_unique_payment_urls(values: &[String], field: &str, max_items: usize) -> Result<()> {
    if values.len() > max_items {
        return Err(WechatError::Config(format!(
            "{field} list cannot contain more than {max_items} items"
        )));
    }
    let mut seen = std::collections::HashSet::with_capacity(values.len());
    for value in values {
        validate_https_url(value, field, 2048, true)?;
        if !seen.insert(value.as_str()) {
            return Err(WechatError::Config(format!(
                "{field} list cannot contain duplicates"
            )));
        }
    }
    Ok(())
}

fn validate_payment_rfc3339(value: &str, field: &str) -> Result<()> {
    validate_payment_text(value, field, 64)?;
    chrono::DateTime::parse_from_rfc3339(value)
        .map_err(|_| WechatError::Config(format!("{field} must use RFC3339 format")))?;
    Ok(())
}

fn validate_https_url(value: &str, field: &str, max_chars: usize, allow_query: bool) -> Result<()> {
    validate_payment_text(value, field, max_chars)?;
    let parsed = url::Url::parse(value)
        .map_err(|_| WechatError::Config(format!("{field} must be an absolute HTTPS URL")))?;
    if parsed.scheme() != "https"
        || parsed.host_str().is_none()
        || !parsed.username().is_empty()
        || parsed.password().is_some()
        || parsed.fragment().is_some()
        || (!allow_query && parsed.query().is_some())
    {
        return Err(WechatError::Config(format!(
            "{field} must be an absolute HTTPS URL without credentials or fragments{}",
            if allow_query {
                ""
            } else {
                " or query parameters"
            }
        )));
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplaintMessageInfo {
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub custom_data: Option<String>,
    #[serde(default, flatten)]
    pub extra: Value,
}

impl ComplaintMessageInfo {
    pub fn validate(&self) -> Result<()> {
        if self.content.is_none() && self.custom_data.is_none() {
            return Err(WechatError::Config(
                "complaint send-message action requires content or custom_data".to_string(),
            ));
        }
        if let Some(content) = self.content.as_deref() {
            validate_payment_text(content, "complaint action message content", 2000)?;
        }
        if let Some(custom_data) = self.custom_data.as_deref() {
            validate_payment_text(custom_data, "complaint action custom data", 2048)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplaintNotificationRequest {
    pub url: String,
}

impl ComplaintNotificationRequest {
    pub fn validate(&self) -> Result<()> {
        validate_https_url(&self.url, "complaint notification URL", 255, false)?;
        let parsed = url::Url::parse(&self.url).map_err(|_| {
            WechatError::Config(
                "complaint notification URL must be an absolute HTTPS URL".to_string(),
            )
        })?;
        if parsed.path().is_empty() || parsed.path() == "/" {
            return Err(WechatError::Config(
                "complaint notification URL must include a callback path".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplaintNotificationResponse {
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default, rename = "mchid")]
    pub mch_id: Option<String>,
    #[serde(default)]
    pub url: String,
    #[serde(default, flatten)]
    pub extra: Value,
}

impl ComplaintNotificationResponse {
    pub fn validate(&self) -> Result<()> {
        if let Some(code) = self.code.as_deref().filter(|code| !code.trim().is_empty()) {
            return Err(WechatError::Config(format!(
                "complaint notification API error {code}: {}",
                self.message.as_deref().unwrap_or("unknown error")
            )));
        }
        let mch_id = self.mch_id.as_deref().ok_or_else(|| {
            WechatError::Config("complaint notification merchant id is required".to_string())
        })?;
        validate_payment_identifier(mch_id, "complaint notification merchant id", 64)?;
        validate_https_url(&self.url, "complaint notification URL", 255, false)?;
        let parsed = url::Url::parse(&self.url).map_err(|_| {
            WechatError::Config(
                "complaint notification URL must be an absolute HTTPS URL".to_string(),
            )
        })?;
        if parsed.path().is_empty() || parsed.path() == "/" {
            return Err(WechatError::Config(
                "complaint notification URL must include a callback path".to_string(),
            ));
        }
        Ok(())
    }

    pub fn is_configured(&self) -> bool {
        self.code.is_none() && !self.url.trim().is_empty()
    }

    pub fn has_api_error(&self) -> bool {
        self.code
            .as_deref()
            .is_some_and(|code| !code.trim().is_empty())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplaintReplyRequest {
    pub complainted_mchid: String,
    pub response_content: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub response_images: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jump_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jump_url_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mini_program_jump_info: Option<ComplaintMiniProgramJumpInfo>,
}

impl ComplaintReplyRequest {
    pub fn validate(&self) -> Result<()> {
        validate_payment_identifier(&self.complainted_mchid, "complainted merchant id", 64)?;
        validate_payment_text(&self.response_content, "complaint response content", 500)?;
        validate_unique_payment_strings(
            &self.response_images,
            "complaint response image media id",
            4,
        )?;

        match (self.jump_url.as_deref(), self.jump_url_text.as_deref()) {
            (Some(jump_url), Some(jump_url_text)) => {
                validate_https_url(jump_url, "complaint response jump URL", 512, true)?;
                validate_payment_text(jump_url_text, "complaint response jump URL text", 10)?;
            }
            (None, None) => {}
            _ => {
                return Err(WechatError::Config(
                    "complaint response jump URL and text must be supplied together".to_string(),
                ));
            }
        }

        if let Some(mini_program) = self.mini_program_jump_info.as_ref() {
            mini_program.validate()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplaintCompleteRequest {
    pub complainted_mchid: String,
}

impl ComplaintCompleteRequest {
    pub fn validate(&self) -> Result<()> {
        validate_payment_identifier(&self.complainted_mchid, "complainted merchant id", 64)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplaintRefundProgressRequest {
    pub action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_refund_day: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reject_media_list: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplaintRefundAction {
    Approve,
    Reject,
    Other,
}

impl ComplaintRefundAction {
    pub fn from_code(value: &str) -> Self {
        if value.eq_ignore_ascii_case("APPROVE") {
            Self::Approve
        } else if value.eq_ignore_ascii_case("REJECT") {
            Self::Reject
        } else {
            Self::Other
        }
    }
}

impl ComplaintRefundProgressRequest {
    pub fn approve(launch_refund_day: i64) -> Self {
        Self {
            action: "APPROVE".to_string(),
            launch_refund_day: Some(launch_refund_day),
            reject_reason: None,
            reject_media_list: Vec::new(),
            remark: None,
        }
    }

    pub fn reject(reason: impl Into<String>) -> Self {
        Self {
            action: "REJECT".to_string(),
            launch_refund_day: None,
            reject_reason: Some(reason.into()),
            reject_media_list: Vec::new(),
            remark: None,
        }
    }

    pub fn with_remark(mut self, remark: impl Into<String>) -> Self {
        self.remark = Some(remark.into());
        self
    }

    pub fn validate(&self) -> Result<()> {
        validate_unique_payment_strings(
            &self.reject_media_list,
            "complaint refund rejection media id",
            4,
        )?;
        if let Some(remark) = self.remark.as_deref() {
            validate_payment_text(remark, "complaint refund remark", 200)?;
        }

        match self.action_kind() {
            ComplaintRefundAction::Approve => {
                if self.launch_refund_day.is_none_or(|days| days < 0) {
                    return Err(WechatError::Config(
                        "approved complaint refunds require a non-negative launch_refund_day"
                            .to_string(),
                    ));
                }
                if self
                    .reject_reason
                    .as_deref()
                    .is_some_and(|reason| !reason.trim().is_empty())
                    || !self.reject_media_list.is_empty()
                {
                    return Err(WechatError::Config(
                        "approved complaint refunds cannot include rejection evidence".to_string(),
                    ));
                }
            }
            ComplaintRefundAction::Reject => {
                let reason = self.reject_reason.as_deref().ok_or_else(|| {
                    WechatError::Config(
                        "rejected complaint refunds require a rejection reason".to_string(),
                    )
                })?;
                validate_payment_text(reason, "complaint refund rejection reason", 200)?;
                if self.launch_refund_day.is_some() {
                    return Err(WechatError::Config(
                        "rejected complaint refunds cannot include launch_refund_day".to_string(),
                    ));
                }
            }
            ComplaintRefundAction::Other => {
                return Err(WechatError::Config(
                    "complaint refund action must be APPROVE or REJECT".to_string(),
                ));
            }
        }
        Ok(())
    }

    pub fn action_kind(&self) -> ComplaintRefundAction {
        ComplaintRefundAction::from_code(&self.action)
    }
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
    pub location: Option<PayScoreLocation>,
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
    #[serde(default)]
    pub appid: Option<String>,
    #[serde(default)]
    pub mchid: Option<String>,
    #[serde(default)]
    pub out_order_no: Option<String>,
    #[serde(default)]
    pub service_id: Option<String>,
    #[serde(default)]
    pub service_introduction: Option<String>,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default)]
    pub state_description: Option<String>,
    #[serde(default)]
    pub post_payments: Vec<PayScorePostPayment>,
    #[serde(default)]
    pub post_discounts: Vec<PayScorePostDiscount>,
    #[serde(default)]
    pub risk_fund: Option<PayScoreRiskFund>,
    #[serde(default)]
    pub time_range: Option<PayScoreTimeRange>,
    #[serde(default)]
    pub location: Option<PayScoreLocation>,
    #[serde(default)]
    pub attach: Option<String>,
    #[serde(default)]
    pub notify_url: Option<String>,
    #[serde(default)]
    pub order_id: Option<String>,
    #[serde(default, rename = "package")]
    pub package_info: Option<String>,
    #[serde(default, flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayScoreLocation {
    #[serde(default)]
    pub start_location: Option<String>,
    #[serde(default)]
    pub end_location: Option<String>,
    #[serde(default)]
    pub start_latitude: Option<f64>,
    #[serde(default)]
    pub start_longitude: Option<f64>,
    #[serde(default)]
    pub end_latitude: Option<f64>,
    #[serde(default)]
    pub end_longitude: Option<f64>,
    #[serde(default)]
    pub start_address: Option<String>,
    #[serde(default)]
    pub end_address: Option<String>,
    #[serde(default, flatten)]
    pub extra: Value,
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
    #[serde(default)]
    pub original_type: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaymentNotificationEventKind {
    TransactionSuccess,
    RefundSuccess,
    RefundAbnormal,
    RefundClosed,
    Complaint,
    Other,
}

impl PaymentNotificationEventKind {
    pub fn from_code(value: &str) -> Self {
        match value.to_ascii_uppercase().as_str() {
            "TRANSACTION.SUCCESS" => Self::TransactionSuccess,
            "REFUND.SUCCESS" => Self::RefundSuccess,
            "REFUND.ABNORMAL" => Self::RefundAbnormal,
            "REFUND.CLOSED" => Self::RefundClosed,
            value if value.starts_with("COMPLAINT.") => Self::Complaint,
            _ => Self::Other,
        }
    }

    pub fn is_refund(self) -> bool {
        matches!(
            self,
            Self::RefundSuccess | Self::RefundAbnormal | Self::RefundClosed
        )
    }

    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::TransactionSuccess | Self::RefundSuccess | Self::RefundClosed
        )
    }
}

impl PaymentNotification {
    pub fn event_kind(&self) -> PaymentNotificationEventKind {
        PaymentNotificationEventKind::from_code(&self.event_type)
    }

    pub fn idempotency_key(&self) -> &str {
        &self.id
    }

    pub fn validate(&self) -> Result<()> {
        validate_payment_identifier(&self.id, "payment notification id", 36)?;
        validate_payment_text(&self.create_time, "payment notification create time", 32)?;
        chrono::DateTime::parse_from_rfc3339(&self.create_time).map_err(|_| {
            WechatError::Config(
                "payment notification create time must use RFC3339 format".to_string(),
            )
        })?;
        validate_payment_identifier(&self.event_type, "payment notification event type", 32)?;
        if self.resource_type != "encrypt-resource" {
            return Err(WechatError::Config(
                "payment notification resource type must be encrypt-resource".to_string(),
            ));
        }
        validate_payment_text(&self.summary, "payment notification summary", 64)?;
        self.resource.validate()
    }

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

    pub fn decrypt_transaction_resource(
        &self,
        api_v3_key: &str,
    ) -> Result<PaymentTransactionNotification> {
        self.validate_typed_resource(
            PaymentNotificationEventKind::TransactionSuccess,
            "transaction",
        )?;
        self.decrypt_resource(api_v3_key)
    }

    pub fn decrypt_refund_resource(&self, api_v3_key: &str) -> Result<PaymentRefundNotification> {
        self.validate()?;
        if !self.event_kind().is_refund() {
            return Err(WechatError::Config(format!(
                "payment notification event {} is not a refund event",
                self.event_type
            )));
        }
        self.validate_original_type("refund")?;
        self.decrypt_resource(api_v3_key)
    }

    pub fn decrypt_complaint_resource(
        &self,
        api_v3_key: &str,
    ) -> Result<ComplaintNotificationResource> {
        self.validate_typed_resource(PaymentNotificationEventKind::Complaint, "complaint")?;
        self.decrypt_resource(api_v3_key)
    }

    fn validate_typed_resource(
        &self,
        expected_event: PaymentNotificationEventKind,
        expected_original_type: &str,
    ) -> Result<()> {
        self.validate()?;
        if self.event_kind() != expected_event {
            return Err(WechatError::Config(format!(
                "payment notification event {} does not match the requested {:?} resource",
                self.event_type, expected_event
            )));
        }
        self.validate_original_type(expected_original_type)
    }

    fn validate_original_type(&self, expected: &str) -> Result<()> {
        let actual = self.resource.original_type.as_deref().ok_or_else(|| {
            WechatError::Config(
                "payment notification original resource type is missing".to_string(),
            )
        })?;
        if !actual.eq_ignore_ascii_case(expected) {
            return Err(WechatError::Config(format!(
                "payment notification original resource type {actual} does not match {expected}"
            )));
        }
        Ok(())
    }
}

impl PaymentResource {
    pub fn validate(&self) -> Result<()> {
        if self.algorithm != "AEAD_AES_256_GCM" {
            return Err(WechatError::Config(format!(
                "unsupported payment notification algorithm: {}",
                self.algorithm
            )));
        }
        validate_payment_text(
            &self.ciphertext,
            "payment notification ciphertext",
            1_048_576,
        )?;
        validate_payment_text(&self.nonce, "payment notification nonce", 32)?;
        if self.associated_data.chars().count() > 16 {
            return Err(WechatError::Config(
                "payment notification associated data cannot exceed 16 characters".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplaintNotificationResource {
    pub complaint_id: String,
    pub action_type: String,
    #[serde(default, flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplaintNotificationActionKind {
    CreateComplaint,
    ContinueComplaint,
    UserResponse,
    PlatformResponse,
    SellerRefund,
    MerchantResponse,
    MerchantConfirmComplete,
    UserApplyPlatformService,
    UserCancelPlatformService,
    PlatformServiceFinished,
    MerchantApproveRefund,
    MerchantRejectRefund,
    RefundSuccess,
    Other,
}

impl ComplaintNotificationActionKind {
    pub fn from_code(value: &str) -> Self {
        match value.to_ascii_uppercase().as_str() {
            "CREATE_COMPLAINT" => Self::CreateComplaint,
            "CONTINUE_COMPLAINT" => Self::ContinueComplaint,
            "USER_RESPONSE" => Self::UserResponse,
            "RESPONSE_BY_PLATFORM" => Self::PlatformResponse,
            "SELLER_REFUND" => Self::SellerRefund,
            "MERCHANT_RESPONSE" => Self::MerchantResponse,
            "MERCHANT_CONFIRM_COMPLETE" => Self::MerchantConfirmComplete,
            "USER_APPLY_PLATFORM_SERVICE" | "USER_APPLY_PLATFORM_SERIVCE" => {
                Self::UserApplyPlatformService
            }
            "USER_CANCEL_PLATFORM_SERVICE" => Self::UserCancelPlatformService,
            "PLATFORM_SERVICE_FINISHED" => Self::PlatformServiceFinished,
            "MERCHANT_APPROVE_REFUND" => Self::MerchantApproveRefund,
            "MERCHANT_REJECT_REFUND" => Self::MerchantRejectRefund,
            "REFUND_SUCCESS" => Self::RefundSuccess,
            _ => Self::Other,
        }
    }

    pub fn changes_complaint_state(self) -> bool {
        matches!(
            self,
            Self::MerchantApproveRefund | Self::MerchantRejectRefund | Self::RefundSuccess
        )
    }
}

impl ComplaintNotificationResource {
    pub fn action_kind(&self) -> ComplaintNotificationActionKind {
        ComplaintNotificationActionKind::from_code(&self.action_type)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentTransactionNotification {
    #[serde(default)]
    pub amount: Option<PaymentTransactionAmount>,
    #[serde(default)]
    pub appid: Option<String>,
    #[serde(default)]
    pub sp_appid: Option<String>,
    #[serde(default)]
    pub sp_mchid: Option<String>,
    #[serde(default)]
    pub sub_appid: Option<String>,
    #[serde(default)]
    pub sub_mchid: Option<String>,
    #[serde(default)]
    pub attach: Option<String>,
    #[serde(default)]
    pub bank_type: Option<String>,
    #[serde(default)]
    pub mchid: Option<String>,
    #[serde(default)]
    pub out_trade_no: Option<String>,
    #[serde(default)]
    pub payer: Option<PaymentTransactionPayer>,
    #[serde(default)]
    pub promotion_detail: Vec<PaymentPromotionDetail>,
    #[serde(default)]
    pub success_time: Option<String>,
    #[serde(default)]
    pub trade_state: Option<String>,
    #[serde(default)]
    pub trade_state_desc: Option<String>,
    #[serde(default)]
    pub trade_type: Option<String>,
    #[serde(default)]
    pub transaction_id: Option<String>,
    #[serde(default)]
    pub scene_info: Option<PaymentTransactionSceneInfo>,
    #[serde(default, flatten)]
    pub extra: Value,
}

impl PaymentTransactionNotification {
    pub fn trade_state_kind(&self) -> Option<PaymentTradeStateKind> {
        self.trade_state
            .as_deref()
            .map(PaymentTradeStateKind::from_code)
    }

    pub fn is_paid(&self) -> bool {
        self.trade_state_kind()
            .is_some_and(PaymentTradeStateKind::is_success)
    }

    pub fn effective_appid(&self) -> Option<&str> {
        self.sub_appid
            .as_deref()
            .or(self.appid.as_deref())
            .or(self.sp_appid.as_deref())
    }

    pub fn effective_mchid(&self) -> Option<&str> {
        self.sub_mchid
            .as_deref()
            .or(self.mchid.as_deref())
            .or(self.sp_mchid.as_deref())
    }

    pub fn promotion_total(&self) -> Result<i64> {
        payment_promotion_total(&self.promotion_detail)
    }

    pub fn assert_amount_reconciles(&self) -> Result<()> {
        let amount = self.amount.as_ref().ok_or_else(|| {
            WechatError::Config("payment notification amount is missing".to_string())
        })?;
        amount.assert_reconciles(&self.promotion_detail)
    }

    pub fn verify_paid_transaction(
        &self,
        expected_mchid: &str,
        expected_out_trade_no: &str,
        expected_total: i64,
        expected_currency: &str,
    ) -> Result<()> {
        verify_paid_transaction(
            self.trade_state.as_deref(),
            self.effective_mchid(),
            self.out_trade_no.as_deref(),
            self.transaction_id.as_deref(),
            self.amount.as_ref(),
            &self.promotion_detail,
            expected_mchid,
            expected_out_trade_no,
            expected_total,
            expected_currency,
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentTransactionAmount {
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub payer_currency: Option<String>,
    #[serde(default)]
    pub payer_total: Option<i64>,
    #[serde(default)]
    pub total: Option<i64>,
    #[serde(default, flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentTransactionPayer {
    #[serde(default)]
    pub openid: Option<String>,
    #[serde(default)]
    pub sub_openid: Option<String>,
    #[serde(default)]
    pub sp_openid: Option<String>,
    #[serde(default, flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentTransactionSceneInfo {
    #[serde(default)]
    pub device_id: Option<String>,
    #[serde(default, flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentPromotionDetail {
    #[serde(default)]
    pub coupon_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub scope: Option<String>,
    #[serde(default, rename = "type")]
    pub promotion_type: Option<String>,
    #[serde(default)]
    pub amount: Option<i64>,
    #[serde(default)]
    pub stock_id: Option<String>,
    #[serde(default)]
    pub wechatpay_contribute: Option<i64>,
    #[serde(default)]
    pub merchant_contribute: Option<i64>,
    #[serde(default)]
    pub other_contribute: Option<i64>,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub goods_detail: Vec<PaymentPromotionGoodsDetail>,
    #[serde(default, flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentPromotionGoodsDetail {
    #[serde(default)]
    pub goods_id: Option<String>,
    #[serde(default)]
    pub quantity: Option<i64>,
    #[serde(default)]
    pub unit_price: Option<i64>,
    #[serde(default)]
    pub discount_amount: Option<i64>,
    #[serde(default)]
    pub goods_remark: Option<String>,
    #[serde(default, flatten)]
    pub extra: Value,
}

impl PaymentTransactionAmount {
    pub fn assert_reconciles(&self, promotions: &[PaymentPromotionDetail]) -> Result<()> {
        let total = required_non_negative_payment_amount(self.total, "payment total")?;
        let payer_total =
            required_non_negative_payment_amount(self.payer_total, "payment payer total")?;
        if payer_total > total {
            return Err(WechatError::Config(
                "payment payer total cannot exceed the order total".to_string(),
            ));
        }
        if let (Some(currency), Some(payer_currency)) =
            (self.currency.as_deref(), self.payer_currency.as_deref())
        {
            if !currency.eq_ignore_ascii_case(payer_currency) {
                return Err(WechatError::Config(format!(
                    "payment currency {currency} does not match payer currency {payer_currency}"
                )));
            }
        }
        let promotion_total = payment_promotion_total(promotions)?;
        let reconciled = payer_total.checked_add(promotion_total).ok_or_else(|| {
            WechatError::Config("payment reconciliation amount exceeds i64".to_string())
        })?;
        if reconciled != total {
            return Err(WechatError::Config(format!(
                "payment payer total {payer_total} plus promotion total {promotion_total} does not match order total {total}"
            )));
        }
        Ok(())
    }
}

fn required_non_negative_payment_amount(value: Option<i64>, field: &str) -> Result<i64> {
    let value = value
        .ok_or_else(|| WechatError::Config(format!("{field} is required for reconciliation")))?;
    if value < 0 {
        return Err(WechatError::Config(format!("{field} cannot be negative")));
    }
    Ok(value)
}

fn payment_promotion_total(promotions: &[PaymentPromotionDetail]) -> Result<i64> {
    promotions.iter().try_fold(0_i64, |total, promotion| {
        let amount =
            required_non_negative_payment_amount(promotion.amount, "payment promotion amount")?;
        total
            .checked_add(amount)
            .ok_or_else(|| WechatError::Config("payment promotion total exceeds i64".to_string()))
    })
}

#[allow(clippy::too_many_arguments)]
fn verify_paid_transaction(
    trade_state: Option<&str>,
    actual_mchid: Option<&str>,
    actual_out_trade_no: Option<&str>,
    transaction_id: Option<&str>,
    amount: Option<&PaymentTransactionAmount>,
    promotions: &[PaymentPromotionDetail],
    expected_mchid: &str,
    expected_out_trade_no: &str,
    expected_total: i64,
    expected_currency: &str,
) -> Result<()> {
    if trade_state.map(PaymentTradeStateKind::from_code) != Some(PaymentTradeStateKind::Success) {
        return Err(WechatError::Config(format!(
            "payment transaction is not successful: {}",
            trade_state.unwrap_or("missing trade state")
        )));
    }
    validate_payment_identifier(expected_mchid, "expected payment merchant id", 32)?;
    validate_payment_identifier(
        expected_out_trade_no,
        "expected payment merchant order number",
        32,
    )?;
    if actual_mchid != Some(expected_mchid) {
        return Err(WechatError::Config(format!(
            "payment merchant id {} does not match expected {expected_mchid}",
            actual_mchid.unwrap_or("missing")
        )));
    }
    if actual_out_trade_no != Some(expected_out_trade_no) {
        return Err(WechatError::Config(format!(
            "payment merchant order number {} does not match expected {expected_out_trade_no}",
            actual_out_trade_no.unwrap_or("missing")
        )));
    }
    validate_payment_identifier(
        transaction_id.unwrap_or_default(),
        "payment transaction id",
        32,
    )?;
    if expected_total <= 0 {
        return Err(WechatError::Config(
            "expected payment total must be positive".to_string(),
        ));
    }
    validate_payment_identifier(expected_currency, "expected payment currency", 16)?;
    let amount = amount
        .ok_or_else(|| WechatError::Config("payment transaction amount is missing".to_string()))?;
    if amount.total != Some(expected_total) {
        return Err(WechatError::Config(format!(
            "payment total {} does not match expected {expected_total}",
            amount
                .total
                .map_or_else(|| "missing".to_string(), |value| value.to_string())
        )));
    }
    if !amount
        .currency
        .as_deref()
        .is_some_and(|currency| currency.eq_ignore_ascii_case(expected_currency))
    {
        return Err(WechatError::Config(format!(
            "payment currency {} does not match expected {expected_currency}",
            amount.currency.as_deref().unwrap_or("missing")
        )));
    }
    amount.assert_reconciles(promotions)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentRefundNotification {
    #[serde(default)]
    pub sp_mchid: Option<String>,
    #[serde(default)]
    pub sub_mchid: Option<String>,
    #[serde(default)]
    pub mchid: Option<String>,
    #[serde(default)]
    pub transaction_id: Option<String>,
    #[serde(default)]
    pub out_trade_no: Option<String>,
    #[serde(default)]
    pub refund_id: Option<String>,
    #[serde(default)]
    pub out_refund_no: Option<String>,
    #[serde(default)]
    pub refund_status: Option<String>,
    #[serde(default)]
    pub success_time: Option<String>,
    #[serde(default)]
    pub user_received_account: Option<String>,
    #[serde(default)]
    pub amount: Option<PaymentRefundNotificationAmount>,
    #[serde(default, flatten)]
    pub extra: Value,
}

impl PaymentRefundNotification {
    pub fn refund_status_kind(&self) -> Option<PaymentRefundStatusKind> {
        self.refund_status
            .as_deref()
            .map(PaymentRefundStatusKind::from_code)
    }

    pub fn is_success(&self) -> bool {
        self.refund_status_kind()
            .is_some_and(PaymentRefundStatusKind::is_success)
    }

    pub fn effective_mchid(&self) -> Option<&str> {
        self.sub_mchid.as_deref().or(self.mchid.as_deref())
    }

    pub fn verify_successful_refund(
        &self,
        expected_mchid: &str,
        expected_out_trade_no: &str,
        expected_out_refund_no: &str,
        expected_total: i64,
        expected_refund: i64,
    ) -> Result<()> {
        if !self.is_success() {
            return Err(WechatError::Config(format!(
                "payment refund is not successful: {}",
                self.refund_status.as_deref().unwrap_or("missing status")
            )));
        }
        verify_payment_refund_identity(
            self.effective_mchid(),
            self.out_trade_no.as_deref(),
            self.out_refund_no.as_deref(),
            self.transaction_id.as_deref(),
            self.refund_id.as_deref(),
            expected_mchid,
            expected_out_trade_no,
            expected_out_refund_no,
        )?;
        let amount = self.amount.as_ref().ok_or_else(|| {
            WechatError::Config("payment refund notification amount is missing".to_string())
        })?;
        amount.verify(expected_total, expected_refund)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentRefundNotificationAmount {
    #[serde(default)]
    pub total: Option<i64>,
    #[serde(default)]
    pub refund: Option<i64>,
    #[serde(default)]
    pub payer_total: Option<i64>,
    #[serde(default)]
    pub payer_refund: Option<i64>,
    #[serde(default, flatten)]
    pub extra: Value,
}

impl PaymentRefundNotificationAmount {
    pub fn verify(&self, expected_total: i64, expected_refund: i64) -> Result<()> {
        if expected_total <= 0 || expected_refund <= 0 || expected_refund > expected_total {
            return Err(WechatError::Config(
                "expected payment refund amounts must be positive and refund cannot exceed total"
                    .to_string(),
            ));
        }
        if self.total != Some(expected_total) {
            return Err(WechatError::Config(format!(
                "payment refund total {} does not match expected {expected_total}",
                self.total
                    .map_or_else(|| "missing".to_string(), |value| value.to_string())
            )));
        }
        if self.refund != Some(expected_refund) {
            return Err(WechatError::Config(format!(
                "payment refund amount {} does not match expected {expected_refund}",
                self.refund
                    .map_or_else(|| "missing".to_string(), |value| value.to_string())
            )));
        }
        let payer_total =
            required_non_negative_payment_amount(self.payer_total, "payment refund payer total")?;
        let payer_refund =
            required_non_negative_payment_amount(self.payer_refund, "payment refund payer refund")?;
        if payer_total > expected_total
            || payer_refund > payer_total
            || payer_refund > expected_refund
        {
            return Err(WechatError::Config(
                "payment refund payer amounts exceed their corresponding totals".to_string(),
            ));
        }
        Ok(())
    }
}

#[allow(clippy::too_many_arguments)]
fn verify_payment_refund_identity(
    actual_mchid: Option<&str>,
    actual_out_trade_no: Option<&str>,
    actual_out_refund_no: Option<&str>,
    transaction_id: Option<&str>,
    refund_id: Option<&str>,
    expected_mchid: &str,
    expected_out_trade_no: &str,
    expected_out_refund_no: &str,
) -> Result<()> {
    validate_payment_identifier(expected_mchid, "expected payment merchant id", 32)?;
    validate_payment_identifier(
        expected_out_trade_no,
        "expected payment merchant order number",
        32,
    )?;
    validate_payment_identifier(
        expected_out_refund_no,
        "expected payment merchant refund number",
        64,
    )?;
    for (actual, expected, field) in [
        (actual_mchid, expected_mchid, "payment merchant id"),
        (
            actual_out_trade_no,
            expected_out_trade_no,
            "payment merchant order number",
        ),
        (
            actual_out_refund_no,
            expected_out_refund_no,
            "payment merchant refund number",
        ),
    ] {
        if actual != Some(expected) {
            return Err(WechatError::Config(format!(
                "{field} {} does not match expected {expected}",
                actual.unwrap_or("missing")
            )));
        }
    }
    validate_payment_identifier(
        transaction_id.unwrap_or_default(),
        "payment transaction id",
        32,
    )?;
    validate_payment_identifier(refund_id.unwrap_or_default(), "payment refund id", 32)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentTransferBillNotification {
    pub out_bill_no: String,
    pub transfer_bill_no: String,
    pub state: String,
    #[serde(default, rename = "mch_id")]
    pub mch_id: Option<String>,
    #[serde(default)]
    pub transfer_amount: Option<i64>,
    #[serde(default, alias = "open_id")]
    pub openid: Option<String>,
    #[serde(default)]
    pub fail_reason: Option<String>,
    #[serde(default)]
    pub create_time: Option<String>,
    #[serde(default)]
    pub update_time: Option<String>,
    #[serde(default, flatten)]
    pub extra: Value,
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use sha1::Digest as _;

    use crate::crypto;

    use super::{
        build_merchant_media_upload_body, build_sandbox_sign_key_xml,
        checked_payment_download_size, encode_payment_path_segment, multipart_quoted,
        split_payment_download_url, verify_payment_download_hash, Amount, AppPayParams,
        Applyment4SubQueryResponse, Applyment4SubRequest, Applyment4SubResponse, BillRequest,
        BillResponse, CertificateListResponse, CodepayAmount, CodepayPayer, CodepayRequest,
        CodepaySettleInfo, CombineAmount, CombineAppPrepayRequest, CombinePayerInfo,
        CombineSceneInfo, CombineSettleInfo, CombineSubOrder, ComplaintAdditionalInfoType,
        ComplaintCompleteRequest, ComplaintDetailResponse, ComplaintListRequest,
        ComplaintListResponse, ComplaintMedia, ComplaintMediaType, ComplaintMessageActionType,
        ComplaintMessageBlockType, ComplaintMessageSenderIdentity, ComplaintMiniProgramJumpInfo,
        ComplaintNegotiationHistoryRequest, ComplaintNegotiationHistoryResponse,
        ComplaintNegotiationOperateType, ComplaintNotificationActionKind,
        ComplaintNotificationRequest, ComplaintNotificationResource, ComplaintNotificationResponse,
        ComplaintProblemType, ComplaintRefundAction, ComplaintRefundProgressRequest,
        ComplaintReplyRequest, ComplaintServiceOrderStateKind, ComplaintStateKind,
        ComplaintUserTag, CouponStockCreateRequest, CouponStockListRequest,
        CouponStockListResponse, CouponStockOperationRequest, CouponStockResponse,
        FundAppElecSignResponse, FundAppTransferBillRequest, FundAppTransferBillResponse,
        FundFlowBillRequest, H5PrepayResponse, JsapiPayParams, LegacyProfitSharingReturnRequest,
        LegacyProfitSharingReturnResponse, LegacyTransferInfoResponse, MerchantFundBalanceResponse,
        MerchantMediaUploadRequest, MerchantMediaUploadResponse, MicropayRequest,
        MiniProgramRedpackRequest, NativePrepayRequest, NativePrepayResponse,
        PartnerCloseOrderRequest, PartnerH5PrepayRequest, PartnerJsapiPrepayRequest,
        PartnerOrderQuery, PartnerPayer, PartnerRefundQuery, PartnerTransactionQuery,
        PayScoreLocation, PayScoreRiskFund, PayScoreServiceOrderQuery, PayScoreServiceOrderRequest,
        PayScoreServiceOrderResponse, PayScoreTimeRange, PaymentBillDownloadRequest,
        PaymentCredentials, PaymentDownloadHasher, PaymentDownloadedBill,
        PaymentDownloadedBillFile, PaymentNotification, PaymentNotificationEventKind,
        PaymentOrderResponse, PaymentRefundNotification, PaymentRefundStatusKind, PaymentResource,
        PaymentStatusResponse, PaymentTradeStateKind, PaymentTransactionNotification,
        PaymentTransferBillNotification, PrepayResponse, ProfitSharingBillRequest,
        ProfitSharingOrderRequest, ProfitSharingReceiver, ProfitSharingReceiverRequest,
        ProfitSharingReturnOrderQuery, ProfitSharingReturnOrderRequest,
        ProfitSharingUnfreezeRequest, QueryRedpackRequest, QueryWorkRedpackRequest,
        RedpackInfoResponse, RedpackResponse, RefundAmount, RefundDetailResponse, RefundRequest,
        ReverseOrderRequest, SandboxSignKeyResponse, SendCouponRequest, SendCouponResponse,
        SendGroupRedpackRequest, SendRedpackRequest, TaxCardTemplateInformation,
        TaxCardTemplateRequest, TaxCustomCell, TemporaryFileGuard, TransferBatchQuery,
        TransferBatchRequest, TransferBillReceiptResponse, TransferDetailInput,
        TransferDetailReceiptQuery, TransferDetailReceiptRequest, TransferDetailReceiptResponse,
        TransferSceneReportInfo, TransferToBalanceRequest, TransferToBalanceResponse,
        UserCouponListRequest, UserCouponListResponse, UserCouponResponse, WorkRedpackRequest,
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
                original_type: Some("transaction".to_string()),
            },
            summary: "success".to_string(),
        };

        let value: serde_json::Value = notification.decrypt_resource(key).unwrap();
        assert_eq!(value, json!({ "trade_state": "SUCCESS" }));
        let transaction = notification.decrypt_transaction_resource(key).unwrap();
        assert!(transaction.is_paid());
        assert_eq!(
            notification.event_kind(),
            PaymentNotificationEventKind::TransactionSuccess
        );
        assert!(notification.event_kind().is_terminal());
        assert_eq!(notification.idempotency_key(), "id");
        assert_eq!(
            notification.resource.original_type.as_deref(),
            Some("transaction")
        );
    }

    #[test]
    fn rejects_mismatched_payment_notification_resources() {
        let key = "0123456789abcdef0123456789abcdef";
        let nonce = "nonce-123456";
        let ciphertext =
            crypto::payment_v3_encrypt_for_test(key, nonce, "", br#"{"refund_status":"SUCCESS"}"#)
                .unwrap();
        let mut notification = PaymentNotification {
            id: "notice-1".to_string(),
            create_time: "2026-07-20T10:00:00+08:00".to_string(),
            event_type: "REFUND.SUCCESS".to_string(),
            resource_type: "encrypt-resource".to_string(),
            resource: PaymentResource {
                algorithm: "AEAD_AES_256_GCM".to_string(),
                ciphertext,
                nonce: nonce.to_string(),
                associated_data: String::new(),
                original_type: Some("refund".to_string()),
            },
            summary: "refund succeeded".to_string(),
        };

        assert_eq!(
            notification.event_kind(),
            PaymentNotificationEventKind::RefundSuccess
        );
        assert!(notification.event_kind().is_refund());
        let refund = notification.decrypt_refund_resource(key).unwrap();
        assert!(refund.is_success());
        let event_error = notification
            .decrypt_transaction_resource(key)
            .expect_err("refund event cannot be decoded as a transaction");
        assert!(event_error.to_string().contains("does not match"));

        notification.resource.original_type = Some("transaction".to_string());
        let type_error = notification
            .decrypt_refund_resource(key)
            .expect_err("refund event with transaction resource must fail");
        assert!(type_error.to_string().contains("does not match refund"));

        notification.resource.algorithm = "AES-CBC".to_string();
        let algorithm_error = notification
            .validate()
            .expect_err("unsupported algorithms must fail");
        assert!(algorithm_error.to_string().contains("unsupported"));

        notification.resource.algorithm = "AEAD_AES_256_GCM".to_string();
        notification.create_time = "not-a-time".to_string();
        let time_error = notification
            .validate()
            .expect_err("invalid notification time must fail");
        assert!(time_error.to_string().contains("RFC3339"));
    }

    #[test]
    fn decrypts_typed_complaint_notification_resource() {
        let key = "0123456789abcdef0123456789abcdef";
        let nonce = "nonce-123456";
        let aad = "complaint";
        let ciphertext = crypto::payment_v3_encrypt_for_test(
            key,
            nonce,
            aad,
            br#"{"complaint_id":"complaint-1","action_type":"REFUND_SUCCESS","trace_id":"trace-1"}"#,
        )
        .unwrap();
        let notification = PaymentNotification {
            id: "notice-1".to_string(),
            create_time: "2026-07-17T10:00:00+08:00".to_string(),
            event_type: "COMPLAINT.STATE_CHANGE".to_string(),
            resource_type: "encrypt-resource".to_string(),
            resource: PaymentResource {
                algorithm: "AEAD_AES_256_GCM".to_string(),
                ciphertext,
                nonce: nonce.to_string(),
                associated_data: aad.to_string(),
                original_type: Some("complaint".to_string()),
            },
            summary: "complaint changed".to_string(),
        };

        let resource: ComplaintNotificationResource =
            notification.decrypt_complaint_resource(key).unwrap();
        assert_eq!(resource.complaint_id, "complaint-1");
        assert_eq!(
            resource.action_kind(),
            ComplaintNotificationActionKind::RefundSuccess
        );
        assert!(resource.action_kind().changes_complaint_state());
        assert_eq!(resource.extra["trace_id"], "trace-1");
    }

    #[test]
    fn deserializes_payment_order_and_prepay_responses() {
        let status: PaymentStatusResponse = serde_json::from_value(json!({
            "code": "SUCCESS",
            "message": "ok",
            "request_id": "status-request"
        }))
        .unwrap();
        assert_eq!(status.code.as_deref(), Some("SUCCESS"));
        assert_eq!(status.extra["request_id"], "status-request");

        let prepay: PrepayResponse = serde_json::from_value(json!({
            "prepay_id": "prepay-id",
            "request_id": "prepay-request"
        }))
        .unwrap();
        assert_eq!(prepay.prepay_id, "prepay-id");
        assert_eq!(prepay.extra["request_id"], "prepay-request");

        let h5: H5PrepayResponse = serde_json::from_value(json!({
            "h5_url": "https://pay.example.com/h5",
            "request_id": "h5-request"
        }))
        .unwrap();
        assert_eq!(h5.h5_url, "https://pay.example.com/h5");
        assert_eq!(h5.extra["request_id"], "h5-request");

        let native: NativePrepayResponse = serde_json::from_value(json!({
            "code_url": "weixin://wxpay/bizpayurl?pr=abc",
            "request_id": "native-request"
        }))
        .unwrap();
        assert_eq!(native.code_url, "weixin://wxpay/bizpayurl?pr=abc");
        assert_eq!(native.extra["request_id"], "native-request");

        let order: PaymentOrderResponse = serde_json::from_value(json!({
            "appid": "wx-app",
            "mchid": "mchid",
            "sp_appid": "sp-app",
            "sp_mchid": "sp-mchid",
            "sub_appid": "sub-app",
            "sub_mchid": "sub-mchid",
            "out_trade_no": "out-1",
            "transaction_id": "tx-1",
            "trade_type": "JSAPI",
            "trade_state": "SUCCESS",
            "trade_state_desc": "paid",
            "bank_type": "OTHERS",
            "attach": "attach",
            "success_time": "2026-07-17T10:00:00+08:00",
            "amount": {
                "total": 100,
                "payer_total": 90,
                "currency": "CNY",
                "settlement_rate": "1.0"
            },
            "payer": {
                "openid": "openid",
                "sub_openid": "sub-openid",
                "payer_client_ip": "127.0.0.1"
            },
            "scene_info": {
                "device_id": "device-1",
                "store_id": "store-1"
            },
            "promotion_detail": [{
                "coupon_id": "coupon-1",
                "type": "CASH",
                "amount": 10,
                "promotion_extra": "retained",
                "goods_detail": [{
                    "goods_id": "sku-1",
                    "quantity": 1,
                    "unit_price": 100,
                    "discount_amount": 10,
                    "goods_extra": "retained"
                }]
            }],
            "order_extra": "retained"
        }))
        .unwrap();

        assert_eq!(order.trade_state.as_deref(), Some("SUCCESS"));
        assert_eq!(
            order.trade_state_kind(),
            Some(PaymentTradeStateKind::Success)
        );
        assert!(order.is_paid());
        order
            .verify_paid_order("sub-mchid", "out-1", 100, "CNY")
            .unwrap();
        order.assert_amount_reconciles().unwrap();
        assert_eq!(order.promotion_total().unwrap(), 10);
        assert_eq!(order.effective_mchid(), Some("sub-mchid"));
        assert_eq!(order.effective_appid(), Some("sub-app"));
        assert!(order.trade_state_kind().expect("trade state").is_terminal());
        assert_eq!(
            PaymentTradeStateKind::from_code("notpay"),
            PaymentTradeStateKind::NotPay
        );
        assert!(PaymentTradeStateKind::NotPay.needs_polling());
        assert!(PaymentTradeStateKind::UserPaying.needs_polling());
        assert!(PaymentTradeStateKind::Accept.needs_polling());
        assert!(PaymentTradeStateKind::Closed.is_terminal());
        assert!(PaymentTradeStateKind::Revoked.is_terminal());
        assert!(PaymentTradeStateKind::PayError.is_terminal());
        assert_eq!(
            PaymentTradeStateKind::from_code("UNKNOWN"),
            PaymentTradeStateKind::Other
        );
        assert_eq!(order.sp_mchid.as_deref(), Some("sp-mchid"));
        assert_eq!(order.sub_mchid.as_deref(), Some("sub-mchid"));
        assert_eq!(
            order.amount.as_ref().and_then(|amount| amount.total),
            Some(100)
        );
        assert_eq!(
            order.amount.as_ref().unwrap().extra["settlement_rate"],
            "1.0"
        );
        assert_eq!(
            order
                .payer
                .as_ref()
                .and_then(|payer| payer.sub_openid.as_deref()),
            Some("sub-openid")
        );
        assert_eq!(
            order.payer.as_ref().unwrap().extra["payer_client_ip"],
            "127.0.0.1"
        );
        assert_eq!(
            order.scene_info.as_ref().unwrap().extra["store_id"],
            "store-1"
        );
        assert_eq!(
            order.promotion_detail[0].extra["promotion_extra"],
            "retained"
        );
        assert_eq!(
            order.promotion_detail[0].goods_detail[0].extra["goods_extra"],
            "retained"
        );
        assert_eq!(order.extra["order_extra"], "retained");
    }

    #[test]
    fn rejects_payment_transaction_identity_and_amount_mismatches() {
        let order: PaymentOrderResponse = serde_json::from_value(json!({
            "mchid": "merchant-1",
            "out_trade_no": "order-1",
            "transaction_id": "transaction-1",
            "trade_state": "SUCCESS",
            "amount": {
                "total": 100,
                "payer_total": 80,
                "currency": "CNY",
                "payer_currency": "CNY"
            },
            "promotion_detail": [{
                "coupon_id": "coupon-1",
                "amount": 10
            }]
        }))
        .unwrap();

        let reconciliation_error = order
            .assert_amount_reconciles()
            .expect_err("unbalanced payer and promotion amounts must fail");
        assert!(reconciliation_error
            .to_string()
            .contains("does not match order total"));

        let merchant_error = order
            .verify_paid_order("merchant-2", "order-1", 100, "CNY")
            .expect_err("merchant mismatch must fail");
        assert!(merchant_error.to_string().contains("merchant id"));

        let order_error = order
            .verify_paid_order("merchant-1", "order-2", 100, "CNY")
            .expect_err("merchant order mismatch must fail");
        assert!(order_error.to_string().contains("merchant order number"));

        let currency_error = order
            .verify_paid_order("merchant-1", "order-1", 100, "USD")
            .expect_err("currency mismatch must fail");
        assert!(currency_error.to_string().contains("currency"));

        let overflow: PaymentOrderResponse = serde_json::from_value(json!({
            "amount": {
                "total": 9223372036854775807_i64,
                "payer_total": 0,
                "currency": "CNY"
            },
            "promotion_detail": [
                {"amount": 9223372036854775807_i64},
                {"amount": 1}
            ]
        }))
        .unwrap();
        let overflow_error = overflow
            .promotion_total()
            .expect_err("promotion aggregation overflow must fail");
        assert!(overflow_error.to_string().contains("exceeds i64"));
    }

    #[test]
    fn deserializes_payment_notify_payloads() {
        let transaction: PaymentTransactionNotification = serde_json::from_value(json!({
            "appid": "wx-app",
            "mchid": "mchid",
            "out_trade_no": "out-1",
            "transaction_id": "tx-1",
            "trade_type": "JSAPI",
            "trade_state": "SUCCESS",
            "amount": {
                "total": 100,
                "payer_total": 90,
                "currency": "CNY",
                "settlement_rate": "1.0"
            },
            "payer": {
                "openid": "openid",
                "payer_client_ip": "127.0.0.1"
            },
            "scene_info": {
                "device_id": "device-1",
                "store_id": "store-1"
            },
            "promotion_detail": [{
                "coupon_id": "coupon-1",
                "type": "CASH",
                "amount": 10,
                "promotion_extra": "retained",
                "goods_detail": [{
                    "goods_id": "sku-1",
                    "quantity": 1,
                    "unit_price": 100,
                    "discount_amount": 10,
                    "goods_extra": "retained"
                }]
            }],
            "transaction_extra": "retained"
        }))
        .unwrap();
        assert_eq!(transaction.trade_state.as_deref(), Some("SUCCESS"));
        assert_eq!(
            transaction.trade_state_kind(),
            Some(PaymentTradeStateKind::Success)
        );
        assert!(transaction.is_paid());
        transaction
            .verify_paid_transaction("mchid", "out-1", 100, "CNY")
            .unwrap();
        transaction.assert_amount_reconciles().unwrap();
        assert_eq!(transaction.promotion_total().unwrap(), 10);
        assert_eq!(transaction.effective_mchid(), Some("mchid"));
        assert_eq!(transaction.effective_appid(), Some("wx-app"));
        let amount = transaction.amount.as_ref().expect("amount");
        assert_eq!(amount.total, Some(100));
        assert_eq!(amount.extra["settlement_rate"], "1.0");
        let payer = transaction.payer.as_ref().expect("payer");
        assert_eq!(payer.openid.as_deref(), Some("openid"));
        assert_eq!(payer.extra["payer_client_ip"], "127.0.0.1");
        let scene_info = transaction.scene_info.as_ref().expect("scene info");
        assert_eq!(scene_info.device_id.as_deref(), Some("device-1"));
        assert_eq!(scene_info.extra["store_id"], "store-1");
        assert_eq!(
            transaction.promotion_detail[0].promotion_type.as_deref(),
            Some("CASH")
        );
        assert_eq!(
            transaction.promotion_detail[0].extra["promotion_extra"],
            "retained"
        );
        assert_eq!(
            transaction.promotion_detail[0].goods_detail[0].extra["goods_extra"],
            "retained"
        );
        assert_eq!(transaction.extra["transaction_extra"], "retained");

        let refund: PaymentRefundNotification = serde_json::from_value(json!({
            "mchid": "mchid",
            "transaction_id": "tx-1",
            "out_trade_no": "out-1",
            "refund_id": "refund-1",
            "out_refund_no": "out-refund-1",
            "refund_status": "SUCCESS",
            "success_time": "2026-07-16T10:00:00+08:00",
            "user_received_account": "微信零钱",
            "amount": {
                "total": 100,
                "refund": 100,
                "payer_total": 100,
                "payer_refund": 100,
                "settlement_refund": 100
            },
            "refund_extra": "retained"
        }))
        .unwrap();
        assert_eq!(refund.refund_status.as_deref(), Some("SUCCESS"));
        assert_eq!(
            refund.refund_status_kind(),
            Some(PaymentRefundStatusKind::Success)
        );
        assert!(refund.is_success());
        refund
            .verify_successful_refund("mchid", "out-1", "out-refund-1", 100, 100)
            .unwrap();
        let refund_amount = refund.amount.as_ref().expect("refund amount");
        assert_eq!(refund_amount.payer_refund, Some(100));
        assert_eq!(refund_amount.extra["settlement_refund"], 100);
        assert_eq!(refund.extra["refund_extra"], "retained");

        let transfer: PaymentTransferBillNotification = serde_json::from_value(json!({
            "out_bill_no": "bill-1",
            "transfer_bill_no": "transfer-1",
            "state": "SUCCESS",
            "mch_id": "mchid",
            "transfer_amount": 100,
            "openid": "openid",
            "create_time": "2026-07-16T10:00:00+08:00"
        }))
        .unwrap();
        assert_eq!(transfer.out_bill_no, "bill-1");
        assert_eq!(transfer.openid.as_deref(), Some("openid"));
    }

    #[test]
    fn rejects_payment_refund_identity_and_amount_mismatches() {
        let refund: PaymentRefundNotification = serde_json::from_value(json!({
            "mchid": "merchant-1",
            "transaction_id": "transaction-1",
            "out_trade_no": "order-1",
            "refund_id": "refund-1",
            "out_refund_no": "merchant-refund-1",
            "refund_status": "SUCCESS",
            "amount": {
                "total": 100,
                "refund": 50,
                "payer_total": 100,
                "payer_refund": 50
            }
        }))
        .unwrap();

        let identity_error = refund
            .verify_successful_refund("merchant-1", "order-1", "merchant-refund-2", 100, 50)
            .expect_err("merchant refund number mismatch must fail");
        assert!(identity_error
            .to_string()
            .contains("merchant refund number"));

        let amount_error = refund
            .verify_successful_refund("merchant-1", "order-1", "merchant-refund-1", 100, 40)
            .expect_err("refund amount mismatch must fail");
        assert!(amount_error.to_string().contains("refund amount"));

        let invalid_payer_amounts: PaymentRefundNotification = serde_json::from_value(json!({
            "mchid": "merchant-1",
            "transaction_id": "transaction-1",
            "out_trade_no": "order-1",
            "refund_id": "refund-1",
            "out_refund_no": "merchant-refund-1",
            "refund_status": "SUCCESS",
            "amount": {
                "total": 100,
                "refund": 50,
                "payer_total": 100,
                "payer_refund": 60
            }
        }))
        .unwrap();
        let payer_error = invalid_payer_amounts
            .verify_successful_refund("merchant-1", "order-1", "merchant-refund-1", 100, 50)
            .expect_err("payer refund above refund total must fail");
        assert!(payer_error.to_string().contains("payer amounts"));
    }

    #[test]
    fn deserializes_pay_score_service_order_response() {
        let response: PayScoreServiceOrderResponse = serde_json::from_value(json!({
            "appid": "wx-app",
            "mchid": "mchid",
            "out_order_no": "out-order-1",
            "service_id": "service-id",
            "service_introduction": "rental",
            "state": "CREATED",
            "state_description": "created",
            "post_payments": [{
                "name": "fee",
                "amount": 100,
                "description": "fee",
                "count": 1
            }],
            "post_discounts": [{
                "name": "discount",
                "amount": 10,
                "description": "discount",
                "count": 1
            }],
            "risk_fund": {
                "name": "deposit",
                "amount": 100,
                "description": "deposit"
            },
            "time_range": {
                "start_time": "2026-07-16T10:00:00+08:00",
                "end_time": "2026-07-16T11:00:00+08:00"
            },
            "location": {
                "start_location": "A",
                "end_location": "B",
                "start_latitude": 31.2304,
                "start_longitude": 121.4737,
                "poi_id": "poi-1"
            },
            "order_id": "order-id",
            "package": "prepay_id=xxx"
        }))
        .unwrap();

        assert_eq!(response.state.as_deref(), Some("CREATED"));
        assert_eq!(response.post_payments[0].name, "fee");
        let location = response.location.unwrap();
        assert_eq!(location.end_location.as_deref(), Some("B"));
        assert_eq!(location.start_latitude, Some(31.2304));
        assert_eq!(location.extra["poi_id"], "poi-1");
        assert_eq!(response.package_info.as_deref(), Some("prepay_id=xxx"));
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
    fn serializes_micropay_request() {
        let value = serde_json::to_value(MicropayRequest {
            appid: "appid".to_string(),
            mchid: "mchid".to_string(),
            description: "desc".to_string(),
            out_trade_no: "out".to_string(),
            auth_code: "134000000000000000".to_string(),
            amount: Amount {
                total: 100,
                currency: "CNY".to_string(),
            },
            attach: Some("attach".to_string()),
            goods_tag: None,
            detail: None,
            scene_info: Some(json!({ "device_id": "device-1" })),
        })
        .unwrap();

        assert_eq!(value["appid"], "appid");
        assert_eq!(value["auth_code"], "134000000000000000");
        assert_eq!(value["amount"]["total"], 100);
        assert_eq!(value["scene_info"]["device_id"], "device-1");
        assert!(value.get("goods_tag").is_none());
    }

    #[test]
    fn builds_sandbox_sign_key_xml() {
        let credentials = PaymentCredentials {
            mch_id: "1900000109".to_string(),
            serial_no: "serial".to_string(),
            private_key_pem: "pem".to_string(),
        };
        let xml = build_sandbox_sign_key_xml(&credentials, "secret", "abc");
        let expected_sign = crypto::payment_legacy_sign(
            &[
                ("mch_id".to_string(), "1900000109".to_string()),
                ("nonce_str".to_string(), "abc".to_string()),
            ],
            "secret",
        );

        assert!(xml.contains("<mch_id><![CDATA[1900000109]]></mch_id>"));
        assert!(xml.contains("<nonce_str><![CDATA[abc]]></nonce_str>"));
        assert!(xml.contains(&format!("<sign><![CDATA[{expected_sign}]]></sign>")));
    }

    #[test]
    fn parses_sandbox_sign_key_response_xml() {
        let response: SandboxSignKeyResponse = quick_xml::de::from_str(
            "<xml><return_code><![CDATA[SUCCESS]]></return_code><mch_id><![CDATA[1900000109]]></mch_id><sandbox_signkey><![CDATA[key]]></sandbox_signkey></xml>",
        )
        .unwrap();

        assert_eq!(response.return_code, "SUCCESS");
        assert_eq!(response.mch_id.as_deref(), Some("1900000109"));
        assert_eq!(response.sandbox_sign_key.as_deref(), Some("key"));
    }

    #[test]
    fn builds_merchant_media_upload_body() {
        let request = MerchantMediaUploadRequest {
            file_name: "pay\"logo.png".to_string(),
            sha256: crypto::sha256_hex(b"image-bytes"),
            data: b"image-bytes".to_vec(),
        };
        let (content_type, body) = build_merchant_media_upload_body(&request);
        let text = String::from_utf8(body).unwrap();

        assert_eq!(
            content_type,
            "multipart/form-data; boundary=----roze-wechat-pay-v3-media-upload"
        );
        assert!(text.contains("name=\"meta\""));
        assert!(text.contains("\"filename\":\"pay\\\"logo.png\""));
        assert!(text.contains("\"sha256\":\""));
        assert!(text.contains("name=\"file\"; filename=\"pay\\\"logo.png\""));
        assert!(text.contains("image-bytes"));
        assert!(text.ends_with("----roze-wechat-pay-v3-media-upload--\r\n"));
    }

    #[test]
    fn escapes_multipart_filename() {
        assert_eq!(multipart_quoted("a\\b\"c.png"), "a\\\\b\\\"c.png");
    }

    #[test]
    fn deserializes_merchant_media_upload_response() {
        let response: MerchantMediaUploadResponse = serde_json::from_value(json!({
            "media_id": "media-1",
            "upload_scene": "merchant-service"
        }))
        .unwrap();

        assert_eq!(response.media_id, "media-1");
        assert_eq!(response.extra["upload_scene"], "merchant-service");
    }

    #[test]
    fn serializes_coupon_stock_create_request() {
        let value = serde_json::to_value(CouponStockCreateRequest {
            value: json!({
                "stock_name": "coupon-stock",
                "belong_merchant": "10000098",
                "comment": "campaign",
                "stock_type": "NORMAL",
                "available_begin_time": "2026-07-07T00:00:00+08:00",
                "available_end_time": "2026-07-31T23:59:59+08:00",
                "stock_use_rule": {
                    "max_coupons": 100,
                    "max_amount": 10000,
                    "max_amount_by_day": 1000,
                    "max_coupons_per_user": 1
                },
                "pattern_info": {
                    "description": "discount",
                    "merchant_logo": "https://example.com/logo.png",
                    "merchant_name": "merchant"
                }
            }),
        })
        .unwrap();

        assert_eq!(value["stock_name"], "coupon-stock");
        assert_eq!(value["stock_use_rule"]["max_coupons"], 100);
        assert_eq!(value["pattern_info"]["merchant_name"], "merchant");
    }

    #[test]
    fn serializes_coupon_stock_operation_request() {
        let value = serde_json::to_value(CouponStockOperationRequest {
            stock_creator_mchid: Some("10000098".to_string()),
        })
        .unwrap();

        assert_eq!(value["stock_creator_mchid"], "10000098");
    }

    #[test]
    fn builds_coupon_stock_list_query() {
        let query = CouponStockListRequest {
            stock_creator_mchid: "10000098".to_string(),
            offset: 0,
            limit: 20,
            status: Some("running".to_string()),
            stock_id: Some("stock-1".to_string()),
            create_start_time: None,
            create_end_time: Some("2026-07-31T23:59:59+08:00".to_string()),
        }
        .into_query();

        assert!(query.contains(&("stock_creator_mchid".to_string(), "10000098".to_string())));
        assert!(query.contains(&("offset".to_string(), "0".to_string())));
        assert!(query.contains(&("limit".to_string(), "20".to_string())));
        assert!(query.contains(&("status".to_string(), "running".to_string())));
        assert!(query.contains(&("stock_id".to_string(), "stock-1".to_string())));
        assert!(query.contains(&(
            "create_end_time".to_string(),
            "2026-07-31T23:59:59+08:00".to_string()
        )));
    }

    #[test]
    fn serializes_send_coupon_request() {
        let value = serde_json::to_value(SendCouponRequest {
            appid: "wx-app".to_string(),
            stock_id: "stock-1".to_string(),
            out_request_no: "request-1".to_string(),
            stock_creator_mchid: "10000098".to_string(),
            coupon_value: Some(100),
            coupon_minimum: Some(1000),
        })
        .unwrap();

        assert_eq!(value["appid"], "wx-app");
        assert_eq!(value["stock_id"], "stock-1");
        assert_eq!(value["coupon_value"], 100);
    }

    #[test]
    fn builds_user_coupon_list_query() {
        let query = UserCouponListRequest {
            appid: "wx-app".to_string(),
            stock_id: Some("stock-1".to_string()),
            coupon_state: Some("SENDED".to_string()),
            creator_mchid: Some("10000098".to_string()),
            sender_mchid: None,
            offset: Some(0),
            limit: Some(10),
        }
        .into_query();

        assert!(query.contains(&("appid".to_string(), "wx-app".to_string())));
        assert!(query.contains(&("stock_id".to_string(), "stock-1".to_string())));
        assert!(query.contains(&("coupon_state".to_string(), "SENDED".to_string())));
        assert!(query.contains(&("offset".to_string(), "0".to_string())));
        assert!(query.contains(&("limit".to_string(), "10".to_string())));
    }

    #[test]
    fn deserializes_coupon_stock_responses() {
        let stock: CouponStockResponse = serde_json::from_value(json!({
            "stock_id": "stock-1",
            "stock_name": "coupon-stock",
            "status": "running"
        }))
        .unwrap();
        let list: CouponStockListResponse = serde_json::from_value(json!({
            "total_count": 1,
            "limit": 20,
            "offset": 0,
            "data": [{ "stock_id": "stock-1" }]
        }))
        .unwrap();

        assert_eq!(stock.value["stock_id"], "stock-1");
        assert_eq!(list.total_count, Some(1));
        assert_eq!(list.data[0]["stock_id"], "stock-1");
    }

    #[test]
    fn deserializes_coupon_user_responses() {
        let sent: SendCouponResponse =
            serde_json::from_value(json!({ "coupon_id": "coupon-1" })).unwrap();
        let list: UserCouponListResponse = serde_json::from_value(json!({
            "total_count": 1,
            "data": [{ "coupon_id": "coupon-1", "stock_id": "stock-1" }]
        }))
        .unwrap();
        let coupon: UserCouponResponse = serde_json::from_value(json!({
            "coupon_id": "coupon-1",
            "coupon_state": "SENDED"
        }))
        .unwrap();

        assert_eq!(sent.coupon_id, "coupon-1");
        assert_eq!(list.data[0]["stock_id"], "stock-1");
        assert_eq!(coupon.value["coupon_state"], "SENDED");
    }

    #[test]
    fn builds_redpack_params_with_default_total_num() {
        let request: SendRedpackRequest = serde_json::from_value(json!({
            "mch_billno": "bill-1",
            "wxappid": "wx-app",
            "send_name": "merchant",
            "re_openid": "openid",
            "total_amount": 100,
            "wishing": "thanks",
            "client_ip": "127.0.0.1",
            "act_name": "campaign",
            "remark": "remark",
            "scene_id": "PRODUCT_2"
        }))
        .unwrap();
        let params = request.into_params();

        assert!(params.contains(&("mch_billno".to_string(), "bill-1".to_string())));
        assert!(params.contains(&("total_num".to_string(), "1".to_string())));
        assert!(params.contains(&("scene_id".to_string(), "PRODUCT_2".to_string())));
        assert!(!params.iter().any(|(key, _)| key == "mch_id"));
        assert!(!params.iter().any(|(key, _)| key == "sign"));
    }

    #[test]
    fn builds_group_redpack_params() {
        let params = SendGroupRedpackRequest {
            mch_billno: "bill-1".to_string(),
            wxappid: "wx-app".to_string(),
            send_name: "merchant".to_string(),
            re_openid: "openid".to_string(),
            total_amount: 300,
            total_num: 3,
            amt_type: "ALL_RAND".to_string(),
            wishing: "thanks".to_string(),
            act_name: "campaign".to_string(),
            remark: "remark".to_string(),
            scene_id: None,
            risk_info: Some("posttime=1700000000".to_string()),
        }
        .into_params();

        assert!(params.contains(&("amt_type".to_string(), "ALL_RAND".to_string())));
        assert!(params.contains(&("total_num".to_string(), "3".to_string())));
        assert!(params.contains(&("risk_info".to_string(), "posttime=1700000000".to_string())));
    }

    #[test]
    fn builds_query_redpack_params_with_default_bill_type() {
        let params = QueryRedpackRequest {
            mch_billno: "bill-1".to_string(),
            appid: "wx-app".to_string(),
            bill_type: None,
        }
        .into_params();

        assert!(params.contains(&("appid".to_string(), "wx-app".to_string())));
        assert!(params.contains(&("bill_type".to_string(), "MCHT".to_string())));
    }

    #[test]
    fn builds_work_and_mini_program_redpack_params() {
        let work = WorkRedpackRequest {
            mch_billno: "bill-1".to_string(),
            wxappid: "wx-app".to_string(),
            sender_name: "merchant".to_string(),
            sender_header_media_id: "media".to_string(),
            re_openid: "openid".to_string(),
            total_amount: 100,
            wishing: "thanks".to_string(),
            act_name: "campaign".to_string(),
            remark: "remark".to_string(),
            scene_id: Some("PRODUCT_2".to_string()),
            workwx_sign: Some("work-sign".to_string()),
        }
        .into_params();
        assert!(work.contains(&("sender_header_media_id".to_string(), "media".to_string())));
        assert!(work.contains(&("workwx_sign".to_string(), "work-sign".to_string())));

        let query = QueryWorkRedpackRequest {
            mch_billno: "bill-1".to_string(),
            appid: "wx-app".to_string(),
        }
        .into_params();
        assert_eq!(
            query,
            vec![
                ("mch_billno".to_string(), "bill-1".to_string()),
                ("appid".to_string(), "wx-app".to_string())
            ]
        );

        let mini = MiniProgramRedpackRequest {
            mch_billno: "bill-2".to_string(),
            wxappid: "wx-mini".to_string(),
            send_name: "merchant".to_string(),
            re_openid: "openid".to_string(),
            total_amount: 200,
            total_num: 1,
            wishing: "thanks".to_string(),
            act_name: "campaign".to_string(),
            remark: "remark".to_string(),
            notify_way: "MINI_PROGRAM_JSAPI".to_string(),
            scene_id: None,
        }
        .into_params();
        assert!(mini.contains(&("notify_way".to_string(), "MINI_PROGRAM_JSAPI".to_string())));
        assert!(!mini.iter().any(|(key, _)| key == "scene_id"));
    }

    #[test]
    fn builds_transfer_to_balance_params() {
        let credentials = PaymentCredentials {
            mch_id: "1900000109".to_string(),
            serial_no: "serial".to_string(),
            private_key_pem: "pem".to_string(),
        };
        let params = TransferToBalanceRequest {
            mch_appid: "wx-app".to_string(),
            device_info: None,
            partner_trade_no: "partner-1".to_string(),
            openid: "openid".to_string(),
            check_name: "FORCE_CHECK".to_string(),
            re_user_name: Some("Alice".to_string()),
            amount: 100,
            desc: "bonus".to_string(),
            spbill_create_ip: "127.0.0.1".to_string(),
            scene: Some("PRODUCT_2".to_string()),
            brand_id: Some(1000),
            finder_template_id: None,
        }
        .into_params(&credentials);

        assert!(params.contains(&("mch_appid".to_string(), "wx-app".to_string())));
        assert!(params.contains(&("mchid".to_string(), "1900000109".to_string())));
        assert!(params.contains(&("partner_trade_no".to_string(), "partner-1".to_string())));
        assert!(params.contains(&("re_user_name".to_string(), "Alice".to_string())));
        assert!(params.contains(&("brand_id".to_string(), "1000".to_string())));
        assert!(!params.iter().any(|(key, _)| key == "mch_id"));
    }

    #[test]
    fn parses_legacy_transfer_responses() {
        let queried: LegacyTransferInfoResponse = quick_xml::de::from_str(
            "<xml><return_code>SUCCESS</return_code><result_code>SUCCESS</result_code><mch_id>1900000109</mch_id><appid>wx-app</appid><detail_id>detail-1</detail_id><partner_trade_no>partner-1</partner_trade_no><status>SUCCESS</status><payment_amount>100</payment_amount><openid>openid</openid><transfer_time>2026-07-10 10:00:00</transfer_time><transfer_name>Alice</transfer_name><desc>bonus</desc></xml>",
        )
        .unwrap();
        assert_eq!(queried.status.as_deref(), Some("SUCCESS"));
        assert_eq!(queried.payment_amount.as_deref(), Some("100"));

        let created: TransferToBalanceResponse = quick_xml::de::from_str(
            "<xml><return_code>SUCCESS</return_code><result_code>SUCCESS</result_code><mch_appid>wx-app</mch_appid><mchid>1900000109</mchid><partner_trade_no>partner-1</partner_trade_no><payment_no>pay-1</payment_no><payment_time>2026-07-10 10:00:00</payment_time></xml>",
        )
        .unwrap();
        assert_eq!(created.payment_no.as_deref(), Some("pay-1"));
        assert_eq!(created.mchid.as_deref(), Some("1900000109"));
    }

    #[test]
    fn parses_redpack_response_xml() {
        let response: RedpackResponse = quick_xml::de::from_str(
            "<xml><return_code><![CDATA[SUCCESS]]></return_code><result_code><![CDATA[SUCCESS]]></result_code><mch_billno><![CDATA[bill-1]]></mch_billno><mch_id><![CDATA[1900000109]]></mch_id><wxappid><![CDATA[wx-app]]></wxappid><re_openid><![CDATA[openid]]></re_openid><total_amount>100</total_amount><send_listid><![CDATA[list-1]]></send_listid></xml>",
        )
        .unwrap();

        assert_eq!(response.return_code, "SUCCESS");
        assert_eq!(response.result_code.as_deref(), Some("SUCCESS"));
        assert_eq!(response.total_amount, Some(100));
        assert_eq!(response.send_list_id.as_deref(), Some("list-1"));
    }

    #[test]
    fn parses_redpack_info_response_xml() {
        let response: RedpackInfoResponse = quick_xml::de::from_str(
            "<xml><return_code><![CDATA[SUCCESS]]></return_code><result_code><![CDATA[SUCCESS]]></result_code><mch_billno><![CDATA[bill-1]]></mch_billno><mch_id><![CDATA[1900000109]]></mch_id><detail_id><![CDATA[detail-1]]></detail_id><status><![CDATA[RECEIVED]]></status><send_type><![CDATA[API]]></send_type><hb_type><![CDATA[NORMAL]]></hb_type><total_num>1</total_num><total_amount>100</total_amount><send_time><![CDATA[2026-07-06 12:00:00]]></send_time><wishing><![CDATA[thanks]]></wishing><act_name><![CDATA[campaign]]></act_name><remark><![CDATA[remark]]></remark><hblist><hbinfo><openid><![CDATA[openid]]></openid><amount>100</amount><rcv_time><![CDATA[2026-07-06 12:01:00]]></rcv_time></hbinfo></hblist></xml>",
        )
        .unwrap();

        assert_eq!(response.status.as_deref(), Some("RECEIVED"));
        assert_eq!(response.total_num, Some(1));
        let receivers = response.hblist.expect("receiver list").receivers;
        assert_eq!(receivers.len(), 1);
        assert_eq!(receivers[0].openid, "openid");
        assert_eq!(receivers[0].amount, 100);
    }

    #[test]
    fn serializes_applyment4_sub_request() {
        let value = serde_json::to_value(Applyment4SubRequest {
            business_code: "business-1".to_string(),
            contact_info: json!({
                "contact_type": "LEGAL",
                "contact_name": "encrypted-name"
            }),
            subject_info: json!({
                "subject_type": "SUBJECT_TYPE_ENTERPRISE"
            }),
            business_info: json!({
                "merchant_shortname": "merchant"
            }),
            settlement_info: json!({
                "settlement_id": "716"
            }),
            bank_account_info: json!({
                "bank_account_type": "BANK_ACCOUNT_TYPE_CORPORATE"
            }),
            addition_info: Some(json!({
                "legal_person_commitment": "https://example.com/file"
            })),
        })
        .unwrap();

        assert_eq!(value["business_code"], "business-1");
        assert_eq!(value["contact_info"]["contact_type"], "LEGAL");
        assert_eq!(
            value["subject_info"]["subject_type"],
            "SUBJECT_TYPE_ENTERPRISE"
        );
        assert_eq!(
            value["addition_info"]["legal_person_commitment"],
            "https://example.com/file"
        );
    }

    #[test]
    fn deserializes_applyment4_sub_responses() {
        let created: Applyment4SubResponse =
            serde_json::from_value(json!({ "applyment_id": 2000002124775691_i64 })).unwrap();
        assert_eq!(created.applyment_id, Some(2000002124775691));

        let queried: Applyment4SubQueryResponse = serde_json::from_value(json!({
            "business_code": "business-1",
            "applyment_state": "APPLYMENT_STATE_FINISHED",
            "sign_url": "https://example.com/sign"
        }))
        .unwrap();

        assert_eq!(queried.value["business_code"], "business-1");
        assert_eq!(queried.value["applyment_state"], "APPLYMENT_STATE_FINISHED");
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
    fn serializes_combine_app_transaction_request() {
        let value = serde_json::to_value(CombineAppPrepayRequest {
            combine_appid: "wx-combine".to_string(),
            combine_mchid: "combine-mch".to_string(),
            combine_out_trade_no: "combine-out".to_string(),
            notify_url: "https://example.com/notify".to_string(),
            scene_info: Some(CombineSceneInfo {
                device_id: None,
                payer_client_ip: "127.0.0.1".to_string(),
            }),
            sub_orders: vec![CombineSubOrder {
                mchid: "sub-mch".to_string(),
                out_trade_no: "sub-out".to_string(),
                description: "desc".to_string(),
                amount: CombineAmount {
                    total_amount: 100,
                    currency: "CNY".to_string(),
                },
                attach: None,
                goods_tag: Some("goods".to_string()),
                settle_info: Some(CombineSettleInfo {
                    profit_sharing: true,
                    subsidy_amount: Some(10),
                }),
            }],
            combine_payer_info: Some(CombinePayerInfo {
                openid: "openid".to_string(),
            }),
            time_start: None,
            time_expire: None,
        })
        .unwrap();

        assert_eq!(value["combine_appid"], "wx-combine");
        assert_eq!(value["sub_orders"][0]["amount"]["total_amount"], 100);
        assert_eq!(value["sub_orders"][0]["amount"]["currency"], "CNY");
        assert_eq!(
            value["sub_orders"][0]["settle_info"]["profit_sharing"],
            true
        );
        assert_eq!(value["combine_payer_info"]["openid"], "openid");
        assert!(value["scene_info"].get("device_id").is_none());
    }

    #[test]
    fn serializes_codepay_transaction_request() {
        let value = serde_json::to_value(CodepayRequest {
            appid: "wx-app".to_string(),
            mchid: "mch".to_string(),
            description: "desc".to_string(),
            out_trade_no: "out".to_string(),
            attach: "attach".to_string(),
            payer: CodepayPayer {
                auth_code: "auth-code".to_string(),
            },
            amount: CodepayAmount {
                total: 100,
                currency: "CNY".to_string(),
            },
            goods_tag: None,
            support_fapiao: Some(true),
            scene_info: None,
            detail: None,
            settle_info: Some(CodepaySettleInfo {
                profit_sharing: true,
            }),
        })
        .unwrap();

        assert_eq!(value["payer"]["auth_code"], "auth-code");
        assert_eq!(value["amount"]["currency"], "CNY");
        assert_eq!(value["support_fapiao"], true);
        assert_eq!(value["settle_info"]["profit_sharing"], true);
        assert!(value.get("goods_tag").is_none());
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
    fn builds_partner_transaction_query() {
        let query = PartnerTransactionQuery {
            transaction_id: "transaction".to_string(),
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
    fn builds_partner_refund_query() {
        let query = PartnerRefundQuery {
            out_refund_no: "refund-1".to_string(),
            sub_mchid: "sub_mchid".to_string(),
        }
        .into_query();

        assert_eq!(
            query,
            vec![("sub_mchid".to_string(), "sub_mchid".to_string())]
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
        let request = BillRequest {
            bill_date: "2026-07-04".to_string(),
            bill_type: Some("ALL".to_string()),
            tar_type: None,
        };
        assert!(request.validate_trade().is_ok());
        let query = request.into_query();

        assert_eq!(
            query,
            vec![
                ("bill_date".to_string(), "2026-07-04".to_string()),
                ("bill_type".to_string(), "ALL".to_string())
            ]
        );
    }

    #[test]
    fn builds_fund_flow_bill_queries_with_account_type() {
        let compatibility_request = BillRequest {
            bill_date: "2026-07-04".to_string(),
            bill_type: Some("BASIC".to_string()),
            tar_type: Some("GZIP".to_string()),
        };
        assert!(compatibility_request.validate_fund_flow().is_ok());
        let compatibility_query = compatibility_request.into_fund_flow_query();
        assert_eq!(
            compatibility_query,
            vec![
                ("bill_date".to_string(), "2026-07-04".to_string()),
                ("account_type".to_string(), "BASIC".to_string()),
                ("tar_type".to_string(), "GZIP".to_string())
            ]
        );

        let typed_request = FundFlowBillRequest {
            bill_date: "2026-07-04".to_string(),
            account_type: Some("OPERATION".to_string()),
            tar_type: None,
        };
        assert!(typed_request.validate().is_ok());
        let typed_query = typed_request.into_query();
        assert_eq!(
            typed_query,
            vec![
                ("bill_date".to_string(), "2026-07-04".to_string()),
                ("account_type".to_string(), "OPERATION".to_string())
            ]
        );
    }

    #[test]
    fn deserializes_bill_response_with_download_hash() {
        let response: BillResponse = serde_json::from_value(json!({
            "download_url": "https://api.mch.weixin.qq.com/v3/billdownload/file?token=abc",
            "hash_type": "SHA256",
            "hash_value": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
        }))
        .unwrap();

        assert_eq!(
            response.download_url,
            "https://api.mch.weixin.qq.com/v3/billdownload/file?token=abc"
        );
        assert_eq!(response.hash_type.as_deref(), Some("SHA256"));
        assert!(response.validate().is_ok());

        let request = PaymentBillDownloadRequest {
            download_url: response.download_url,
            hash_type: response.hash_type,
            hash_value: response.hash_value,
        };
        assert!(request.validate(true).is_ok());
        assert_eq!(request.hash_value.as_deref().map(str::len), Some(64));
    }

    #[test]
    fn validates_payment_bill_query_and_download_contracts() {
        for invalid in ["", "2026-02-30", "20260704", "2026-7-04"] {
            assert!(BillRequest {
                bill_date: invalid.to_string(),
                bill_type: Some("ALL".to_string()),
                tar_type: None,
            }
            .validate_trade()
            .is_err());
        }

        let mut trade = BillRequest {
            bill_date: "2026-07-04".to_string(),
            bill_type: Some("UNKNOWN".to_string()),
            tar_type: None,
        };
        assert!(trade.validate_trade().is_err());
        trade.bill_type = Some("SUCCESS".to_string());
        trade.tar_type = Some("ZIP".to_string());
        assert!(trade.validate_trade().is_err());

        let mut fund_flow = FundFlowBillRequest {
            bill_date: "2026-07-04".to_string(),
            account_type: Some("UNKNOWN".to_string()),
            tar_type: None,
        };
        assert!(fund_flow.validate().is_err());
        fund_flow.account_type = Some("FEES".to_string());
        fund_flow.tar_type = Some("GZIP".to_string());
        assert!(fund_flow.validate().is_ok());

        let mut profit_sharing = ProfitSharingBillRequest {
            bill_date: "2026-07-04".to_string(),
            tar_type: Some("GZIP".to_string()),
            sub_mchid: Some("1900000109".to_string()),
        };
        assert!(profit_sharing.validate().is_ok());
        profit_sharing.sub_mchid = Some(" ".to_string());
        assert!(profit_sharing.validate().is_err());

        let valid_hash = "a".repeat(64);
        let valid = PaymentBillDownloadRequest {
            download_url: "https://api.mch.weixin.qq.com/v3/billdownload/file?token=abc"
                .to_string(),
            hash_type: Some("SHA256".to_string()),
            hash_value: Some(valid_hash),
        };
        assert!(valid.validate(true).is_ok());

        for invalid in [
            PaymentBillDownloadRequest {
                download_url: "http://api.mch.weixin.qq.com/file".to_string(),
                hash_type: Some("SHA256".to_string()),
                hash_value: Some("a".repeat(64)),
            },
            PaymentBillDownloadRequest {
                download_url: "https://api.mch.weixin.qq.com/file".to_string(),
                hash_type: Some("MD5".to_string()),
                hash_value: Some("a".repeat(32)),
            },
            PaymentBillDownloadRequest {
                download_url: "https://api.mch.weixin.qq.com/file".to_string(),
                hash_type: Some("SHA256".to_string()),
                hash_value: None,
            },
            PaymentBillDownloadRequest {
                download_url: "https://api.mch.weixin.qq.com/file".to_string(),
                hash_type: Some("SHA1".to_string()),
                hash_value: Some("not-a-valid-hash".to_string()),
            },
        ] {
            assert!(invalid.validate(false).is_err());
        }

        let unhashed = PaymentBillDownloadRequest {
            download_url: "https://api.mch.weixin.qq.com/file".to_string(),
            hash_type: None,
            hash_value: None,
        };
        assert!(unhashed.validate(false).is_ok());
        assert!(unhashed.validate(true).is_err());

        for value in [
            json!({
                "download_url": "http://api.mch.weixin.qq.com/file",
                "hash_type": "SHA256",
                "hash_value": "a".repeat(64)
            }),
            json!({
                "download_url": "https://api.mch.weixin.qq.com/file",
                "hash_type": "SHA256"
            }),
            json!({
                "download_url": "https://api.mch.weixin.qq.com/file",
                "hash_type": "SHA256",
                "hash_value": "invalid"
            }),
        ] {
            let response: BillResponse = serde_json::from_value(value).unwrap();
            assert!(response.validate().is_err());
        }
    }

    #[test]
    fn converts_downloadable_payment_artifact_responses() {
        let receipt = TransferBillReceiptResponse {
            out_batch_no: "batch-1".to_string(),
            signature_no: Some("signature-1".to_string()),
            signature_status: "FINISHED".to_string(),
            hash_type: Some("SHA256".to_string()),
            hash_value: Some("hash".to_string()),
            download_url: Some("https://example.com/receipt.pdf".to_string()),
            create_time: "2026-07-18T00:00:00+08:00".to_string(),
            update_time: "2026-07-18T00:01:00+08:00".to_string(),
        };
        let download: PaymentBillDownloadRequest = receipt.try_into().unwrap();
        assert_eq!(download.hash_type.as_deref(), Some("SHA256"));
        assert_eq!(download.download_url, "https://example.com/receipt.pdf");

        let sign = FundAppElecSignResponse {
            state: "FINISHED".to_string(),
            create_time: "2026-07-18T00:00:00+08:00".to_string(),
            update_time: "2026-07-18T00:01:00+08:00".to_string(),
            hash_type: "SHA256".to_string(),
            hash_value: "hash".to_string(),
            download_url: "https://example.com/sign.pdf".to_string(),
        };
        let download: PaymentBillDownloadRequest = sign.into();
        assert_eq!(download.hash_value.as_deref(), Some("hash"));
    }

    #[test]
    fn rejects_payment_receipt_without_download_url() {
        let receipt = TransferDetailReceiptResponse {
            accept_type: "BATCH_TRANSFER".to_string(),
            out_batch_no: "batch-1".to_string(),
            out_detail_no: "detail-1".to_string(),
            signature_no: None,
            signature_status: "PROCESSING".to_string(),
            hash_type: None,
            hash_value: None,
            download_url: None,
        };
        let error = PaymentBillDownloadRequest::try_from(receipt).unwrap_err();
        assert!(error.to_string().contains("PROCESSING"));
    }

    #[test]
    fn splits_payment_download_url_and_verifies_hash() {
        let (path, query) = split_payment_download_url(
            "https://api.mch.weixin.qq.com/v3/billdownload/file?token=a%2Bb&nonce=n",
        )
        .unwrap();
        assert_eq!(path, "/v3/billdownload/file");
        assert_eq!(
            query,
            vec![
                ("token".to_string(), "a%2Bb".to_string()),
                ("nonce".to_string(), "n".to_string())
            ]
        );

        let (relative_path, relative_query) =
            split_payment_download_url("/v3/billdownload/file?token=abc").unwrap();
        assert_eq!(relative_path, "/v3/billdownload/file");
        assert_eq!(
            relative_query,
            vec![("token".to_string(), "abc".to_string())]
        );

        let sha256 = crypto::sha256_hex(b"bill-bytes");
        verify_payment_download_hash(b"bill-bytes", Some("SHA256"), Some(&sha256)).unwrap();

        let mut sha1_hasher = sha1::Sha1::new();
        sha1_hasher.update(b"bill-bytes");
        let sha1 = hex::encode(sha1_hasher.finalize());
        verify_payment_download_hash(b"bill-bytes", Some("SHA1"), Some(&sha1)).unwrap();
        assert!(verify_payment_download_hash(b"bill-bytes", Some("SHA256"), Some("bad")).is_err());
    }

    #[test]
    fn hashes_payment_download_chunks_incrementally() {
        let mut sha1 = PaymentDownloadHasher::new(Some("SHA1")).unwrap();
        sha1.update(b"bill-");
        sha1.update(b"bytes");
        let mut expected_sha1 = sha1::Sha1::new();
        expected_sha1.update(b"bill-bytes");
        assert_eq!(sha1.finalize(), hex::encode(expected_sha1.finalize()));

        let mut sha256 = PaymentDownloadHasher::new(Some("SHA-256")).unwrap();
        sha256.update(b"bill-");
        sha256.update(b"bytes");
        assert_eq!(sha256.finalize(), crypto::sha256_hex(b"bill-bytes"));
        assert!(PaymentDownloadHasher::new(Some("MD5")).is_err());
    }

    #[test]
    fn enforces_payment_download_size_before_writing_chunks() {
        assert_eq!(checked_payment_download_size(4, 3, Some(7)).unwrap(), 7);
        let error = checked_payment_download_size(4, 4, Some(7))
            .expect_err("download over the configured limit should fail");
        assert!(error.to_string().contains("7 byte limit"));
        assert_eq!(checked_payment_download_size(4, 4, None).unwrap(), 8);
    }

    #[test]
    fn removes_abandoned_payment_download_temporary_file() {
        let path =
            std::env::temp_dir().join(format!("roze-wechat-payment-{}.part", uuid::Uuid::now_v7()));
        std::fs::write(&path, b"partial bill").unwrap();

        {
            let _guard = TemporaryFileGuard::new(path.clone());
            assert!(path.exists());
        }

        assert!(!path.exists());
    }

    #[tokio::test]
    async fn verifies_and_reads_committed_payment_bill_files() {
        let bytes = b"transaction_id,out_trade_no,amount\n4200001,order-1,100\nsum,1,100\n";
        let path =
            std::env::temp_dir().join(format!("roze-wechat-payment-{}.csv", uuid::Uuid::now_v7()));
        tokio::fs::write(&path, bytes).await.unwrap();
        let file = PaymentDownloadedBillFile {
            path: path.clone(),
            bytes_written: u64::try_from(bytes.len()).unwrap(),
            hash_type: "SHA256".to_string(),
            hash_value: crypto::sha256_hex(bytes),
        };

        let statement = file.statement(1024).await.unwrap();
        assert_eq!(statement.records.len(), 1);
        assert_eq!(statement.sum_i64("amount").unwrap(), 100);

        let limit_error = file
            .read_verified(10)
            .await
            .expect_err("file over the read limit should fail");
        assert!(limit_error.to_string().contains("byte read limit"));

        tokio::fs::write(&path, b"changed").await.unwrap();
        let length_error = file
            .read_verified(1024)
            .await
            .expect_err("changed file length should fail");
        assert!(length_error.to_string().contains("file length changed"));
        tokio::fs::remove_file(path).await.unwrap();
    }

    #[tokio::test]
    async fn rejects_tampered_payment_bill_files() {
        let bytes = b"header\nrow\nsummary\n";
        let path =
            std::env::temp_dir().join(format!("roze-wechat-payment-{}.csv", uuid::Uuid::now_v7()));
        tokio::fs::write(&path, bytes).await.unwrap();
        let file = PaymentDownloadedBillFile {
            path: path.clone(),
            bytes_written: u64::try_from(bytes.len()).unwrap(),
            hash_type: "SHA256".to_string(),
            hash_value: crypto::sha256_hex(b"different same-length data"),
        };

        let error = file
            .read_verified(1024)
            .await
            .expect_err("hash mismatch should fail");
        assert!(error.to_string().contains("hash mismatch"));
        tokio::fs::remove_file(path).await.unwrap();
    }

    #[test]
    fn builds_structured_payment_downloaded_bill() {
        let bill = PaymentDownloadedBill::from_verified_bytes(
            bytes::Bytes::from_static(
                b"transaction_id,out_trade_no,amount\n4200001,order-1,100\nsum,1,100\n",
            ),
            Some("SHA256".to_string()),
            Some("abc".to_string()),
        )
        .unwrap();

        assert_eq!(
            bill.header.as_deref(),
            Some("transaction_id,out_trade_no,amount")
        );
        assert_eq!(bill.summary.as_deref(), Some("sum,1,100"));
        assert_eq!(bill.line_count, 3);
        assert_eq!(
            bill.data_rows().collect::<Vec<_>>(),
            vec!["4200001,order-1,100"]
        );
        let records = bill.data_records().unwrap();
        assert_eq!(records[0].get("transaction_id"), Some("4200001"));
        assert_eq!(records[0].get("out_trade_no"), Some("order-1"));
        assert_eq!(records[0].get("amount"), Some("100"));
        assert_eq!(records[0].require("out_trade_no").unwrap(), "order-1");
        assert_eq!(records[0].get_i64("amount").unwrap(), Some(100));
        assert_eq!(records[0].require_i64("amount").unwrap(), 100);
        assert_eq!(records[0].raw, "4200001,order-1,100");
        let statement = bill.statement().unwrap();
        assert_eq!(
            statement.headers,
            vec![
                "transaction_id".to_string(),
                "out_trade_no".to_string(),
                "amount".to_string()
            ]
        );
        assert_eq!(statement.records, records);
        assert_eq!(statement.summary.raw, "sum,1,100");
        assert_eq!(statement.summary.get(0), Some("sum"));
        assert_eq!(statement.summary.require(0).unwrap(), "sum");
        assert_eq!(statement.summary.get_i64(1).unwrap(), Some(1));
        assert_eq!(statement.summary.get_i64(2).unwrap(), Some(100));
        assert_eq!(statement.summary.require_i64(1).unwrap(), 1);
        assert_eq!(statement.column_index("amount"), Some(2));
        statement
            .require_columns(&["transaction_id", "out_trade_no", "amount"])
            .unwrap();
        assert_eq!(statement.assert_record_count_matches_summary(1).unwrap(), 1);
        assert_eq!(statement.sum_i64("amount").unwrap(), 100);
        assert_eq!(
            statement.assert_sum_matches_summary("amount", 2).unwrap(),
            100
        );
        let by_order = statement.index_by_unique_column("out_trade_no").unwrap();
        assert_eq!(
            by_order
                .get("order-1")
                .and_then(|record| record.get("transaction_id")),
            Some("4200001")
        );
        let grouped = statement.group_sum_i64("out_trade_no", "amount").unwrap();
        assert_eq!(grouped.get("order-1"), Some(&100));
        assert_eq!(statement.sum_i64("missing").unwrap(), 0);
        assert_eq!(statement.non_empty_count("transaction_id"), 1);
        assert_eq!(
            statement
                .assert_non_empty_count_matches_summary("transaction_id", 1)
                .unwrap(),
            1
        );
        assert_eq!(statement.non_empty_count("missing"), 0);
        assert_eq!(bill.hash_type.as_deref(), Some("SHA256"));
        assert_eq!(bill.bytes.len(), 65);
    }

    #[test]
    fn parses_payment_bill_records_with_quoted_and_excel_escaped_cells() {
        let bill = PaymentDownloadedBill::from_verified_bytes(
            bytes::Bytes::from_static(
                b"\xef\xbb\xbftransaction_id,out_trade_no,description\n`4200001,`order-1,\"fee, service\"\nsummary,1,100\n",
            ),
            None,
            None,
        )
        .unwrap();

        let records = bill.data_records().unwrap();
        assert_eq!(records[0].get("transaction_id"), Some("4200001"));
        assert_eq!(records[0].get("out_trade_no"), Some("order-1"));
        assert_eq!(records[0].get("description"), Some("fee, service"));
    }

    #[test]
    fn rejects_payment_bill_record_field_count_mismatch() {
        let bill = PaymentDownloadedBill::from_verified_bytes(
            bytes::Bytes::from_static(b"a,b\n1\nsum,1\n"),
            None,
            None,
        )
        .unwrap();

        let err = bill
            .data_records()
            .expect_err("mismatched row should be rejected");
        assert!(err.to_string().contains("field count mismatch"));
    }

    #[test]
    fn rejects_payment_bill_record_unterminated_quote() {
        let bill = PaymentDownloadedBill::from_verified_bytes(
            bytes::Bytes::from_static(b"a,b\n\"1,2\nsum,1\n"),
            None,
            None,
        )
        .unwrap();

        let err = bill
            .data_records()
            .expect_err("unterminated quote should be rejected");
        assert!(err.to_string().contains("unterminated quoted field"));
    }

    #[test]
    fn rejects_invalid_payment_bill_numeric_helpers() {
        let bill = PaymentDownloadedBill::from_verified_bytes(
            bytes::Bytes::from_static(b"amount\nnot-number\nsum,total\n"),
            None,
            None,
        )
        .unwrap();

        let statement = bill.statement().unwrap();
        let field_err = statement.records[0]
            .get_i64("amount")
            .expect_err("invalid record amount should fail");
        assert!(field_err.to_string().contains("not a valid i64"));
        let sum_err = statement
            .sum_i64("amount")
            .expect_err("invalid summed amount should fail");
        assert!(sum_err.to_string().contains("not a valid i64"));
        let summary_err = statement
            .summary
            .get_i64(1)
            .expect_err("invalid summary total should fail");
        assert!(summary_err.to_string().contains("not a valid i64"));
        let required_summary_err = statement
            .summary
            .require_i64(1)
            .expect_err("invalid required summary total should fail");
        assert!(required_summary_err.to_string().contains("not a valid i64"));
    }

    #[test]
    fn rejects_payment_bill_sum_overflow() {
        let bill = PaymentDownloadedBill::from_verified_bytes(
            bytes::Bytes::from_static(
                b"state,group,amount\nSUCCESS,A,9223372036854775807\nSUCCESS,A,1\nsum,2,0\n",
            ),
            None,
            None,
        )
        .unwrap();
        let statement = bill.statement().unwrap();

        let sum_error = statement
            .sum_i64("amount")
            .expect_err("total overflow should fail");
        assert!(sum_error.to_string().contains("i64 range"));
        let filtered_error = statement
            .sum_i64_where("state", "SUCCESS", "amount")
            .expect_err("filtered overflow should fail");
        assert!(filtered_error.to_string().contains("i64 range"));
        let grouped_error = statement
            .group_sum_i64("group", "amount")
            .expect_err("grouped overflow should fail");
        assert!(grouped_error.to_string().contains("i64 range"));
        let grouped_filtered_error = statement
            .group_sum_i64_where("state", "SUCCESS", "group", "amount")
            .expect_err("filtered grouped overflow should fail");
        assert!(grouped_filtered_error.to_string().contains("i64 range"));
    }

    #[test]
    fn validates_payment_bill_required_columns_and_summary_totals() {
        let bill = PaymentDownloadedBill::from_verified_bytes(
            bytes::Bytes::from_static(
                b"transaction_id,out_trade_no,amount,trade_state,mchid\n4200001,order-1,100,SUCCESS,1900000109\n4200002,order-2,50,REFUND,1900000109\n4200003,,25,SUCCESS,1900000110\nsum,3,175\n",
            ),
            None,
            None,
        )
        .unwrap();

        let statement = bill.statement().unwrap();
        assert_eq!(statement.column_index("out_trade_no"), Some(1));
        statement
            .require_columns(&[
                "transaction_id",
                "out_trade_no",
                "amount",
                "trade_state",
                "mchid",
            ])
            .unwrap();
        assert_eq!(
            statement.assert_sum_matches_summary("amount", 2).unwrap(),
            175
        );
        let grouped = statement.group_sum_i64("out_trade_no", "amount").unwrap();
        assert_eq!(grouped.get("order-1"), Some(&100));
        assert_eq!(grouped.get("order-2"), Some(&50));
        assert_eq!(grouped.get(""), Some(&25));
        let grouped_counts = statement.group_count("trade_state").unwrap();
        assert_eq!(grouped_counts.get("SUCCESS"), Some(&2));
        assert_eq!(grouped_counts.get("REFUND"), Some(&1));
        let grouped_success_counts = statement
            .group_count_where("trade_state", "SUCCESS", "mchid")
            .unwrap();
        assert_eq!(grouped_success_counts.get("1900000109"), Some(&1));
        assert_eq!(grouped_success_counts.get("1900000110"), Some(&1));
        assert_eq!(
            statement
                .group_count_where("trade_state", "NOT_EXIST", "mchid")
                .unwrap()
                .len(),
            0
        );
        let grouped_success_order_counts = statement
            .group_count_where_all(&[("trade_state", "SUCCESS")], "out_trade_no")
            .unwrap();
        assert_eq!(grouped_success_order_counts.get(""), Some(&1));
        let grouped_non_empty = statement
            .group_non_empty_count("trade_state", "out_trade_no")
            .unwrap();
        assert_eq!(grouped_non_empty.get("SUCCESS"), Some(&1));
        assert_eq!(grouped_non_empty.get("REFUND"), Some(&1));
        assert_eq!(
            statement
                .records_matching("trade_state", "SUCCESS")
                .unwrap()
                .len(),
            2
        );
        assert_eq!(
            statement
                .records_matching("trade_state", "SUCCESS")
                .unwrap()[0]
                .get("transaction_id"),
            Some("4200001")
        );
        assert_eq!(
            statement
                .records_matching("trade_state", "NOT_EXIST")
                .unwrap()
                .len(),
            0
        );
        assert_eq!(
            statement
                .records_matching_all(&[("trade_state", "SUCCESS"), ("out_trade_no", "")])
                .unwrap()[0]
                .get("transaction_id"),
            Some("4200003")
        );
        assert!(statement
            .records_matching_all(&[("trade_state", "SUCCESS"), ("amount", "999")])
            .unwrap()
            .is_empty());
        assert_eq!(
            statement
                .sum_i64_where("trade_state", "SUCCESS", "amount")
                .unwrap(),
            125
        );
        assert_eq!(
            statement
                .sum_i64_where("trade_state", "REFUND", "amount")
                .unwrap(),
            50
        );
        assert_eq!(
            statement
                .sum_i64_where_all(
                    &[("trade_state", "SUCCESS"), ("out_trade_no", "")],
                    "amount"
                )
                .unwrap(),
            25
        );
        let grouped_success_sum = statement
            .group_sum_i64_where("trade_state", "SUCCESS", "mchid", "amount")
            .unwrap();
        assert_eq!(grouped_success_sum.get("1900000109"), Some(&100));
        assert_eq!(grouped_success_sum.get("1900000110"), Some(&25));
        let grouped_success_empty_order_sum = statement
            .group_sum_i64_where_all(
                &[("trade_state", "SUCCESS"), ("out_trade_no", "")],
                "mchid",
                "amount",
            )
            .unwrap();
        assert_eq!(grouped_success_empty_order_sum.get("1900000110"), Some(&25));
        assert_eq!(
            statement
                .non_empty_count_where("trade_state", "SUCCESS", "out_trade_no")
                .unwrap(),
            1
        );
        assert_eq!(
            statement
                .index_by_unique_column("transaction_id")
                .unwrap()
                .len(),
            3
        );
        assert_eq!(
            statement
                .record_by_unique_column("transaction_id", "4200002")
                .unwrap()
                .and_then(|record| record.get("out_trade_no")),
            Some("order-2")
        );
        assert!(statement
            .record_by_unique_column("transaction_id", "missing")
            .unwrap()
            .is_none());
        assert_eq!(
            statement
                .assert_non_empty_count_matches_summary("transaction_id", 1)
                .unwrap(),
            3
        );
        assert_eq!(statement.assert_record_count_matches_summary(1).unwrap(), 3);

        let missing_column = statement
            .require_columns(&["transaction_id", "missing"])
            .expect_err("missing required columns should fail");
        assert!(missing_column.to_string().contains("missing"));
        let missing_filter_column = statement
            .records_matching("missing", "SUCCESS")
            .expect_err("missing filter column should fail");
        assert!(missing_filter_column.to_string().contains("missing"));
        let missing_multi_filter_column = statement
            .records_matching_all(&[("trade_state", "SUCCESS"), ("missing", "value")])
            .expect_err("missing multi filter column should fail");
        assert!(missing_multi_filter_column.to_string().contains("missing"));
        let missing_sum_column = statement
            .sum_i64_where("trade_state", "SUCCESS", "missing")
            .expect_err("missing sum column should fail");
        assert!(missing_sum_column.to_string().contains("missing"));
        let missing_multi_sum_column = statement
            .sum_i64_where_all(&[("missing", "SUCCESS")], "amount")
            .expect_err("missing multi sum column should fail");
        assert!(missing_multi_sum_column.to_string().contains("missing"));
        let missing_count_column = statement
            .non_empty_count_where("trade_state", "SUCCESS", "missing")
            .expect_err("missing count column should fail");
        assert!(missing_count_column.to_string().contains("missing"));
        let missing_group_column = statement
            .group_count("missing")
            .expect_err("missing group column should fail");
        assert!(missing_group_column.to_string().contains("missing"));
        let missing_group_count_column = statement
            .group_non_empty_count("trade_state", "missing")
            .expect_err("missing group count column should fail");
        assert!(missing_group_count_column.to_string().contains("missing"));

        let mismatched = PaymentDownloadedBill::from_verified_bytes(
            bytes::Bytes::from_static(
                b"transaction_id,out_trade_no,amount\n4200001,order-1,100\nsum,1,99\n",
            ),
            None,
            None,
        )
        .unwrap()
        .statement()
        .unwrap();
        let mismatch_err = mismatched
            .assert_sum_matches_summary("amount", 2)
            .expect_err("summary mismatch should fail");
        assert!(mismatch_err.to_string().contains("does not match summary"));

        let count_mismatch = PaymentDownloadedBill::from_verified_bytes(
            bytes::Bytes::from_static(
                b"transaction_id,out_trade_no,amount\n4200001,order-1,100\nsum,2,100\n",
            ),
            None,
            None,
        )
        .unwrap()
        .statement()
        .unwrap();
        let count_mismatch_err = count_mismatch
            .assert_record_count_matches_summary(1)
            .expect_err("record count mismatch should fail");
        assert!(count_mismatch_err.to_string().contains("record count"));

        let non_empty_count_mismatch = PaymentDownloadedBill::from_verified_bytes(
            bytes::Bytes::from_static(
                b"transaction_id,out_trade_no,amount\n4200001,order-1,100\n,order-2,50\nsum,2,150\n",
            ),
            None,
            None,
        )
        .unwrap()
        .statement()
        .unwrap();
        let non_empty_count_err = non_empty_count_mismatch
            .assert_non_empty_count_matches_summary("transaction_id", 1)
            .expect_err("non-empty count mismatch should fail");
        assert!(non_empty_count_err.to_string().contains("non-empty count"));

        let duplicate_key = PaymentDownloadedBill::from_verified_bytes(
            bytes::Bytes::from_static(
                b"transaction_id,out_trade_no,amount\n4200001,order-1,100\n4200001,order-2,50\nsum,2,150\n",
            ),
            None,
            None,
        )
        .unwrap()
        .statement()
        .unwrap();
        let duplicate_err = duplicate_key
            .index_by_unique_column("transaction_id")
            .expect_err("duplicate unique key should fail");
        assert!(duplicate_err.to_string().contains("duplicate value"));
        let duplicate_lookup_err = duplicate_key
            .record_by_unique_column("transaction_id", "4200001")
            .expect_err("duplicate unique lookup should fail");
        assert!(duplicate_lookup_err.to_string().contains("duplicate value"));

        let empty_key = PaymentDownloadedBill::from_verified_bytes(
            bytes::Bytes::from_static(
                b"transaction_id,out_trade_no,amount\n,order-1,100\nsum,1,100\n",
            ),
            None,
            None,
        )
        .unwrap()
        .statement()
        .unwrap();
        let empty_key_err = empty_key
            .index_by_unique_column("transaction_id")
            .expect_err("empty unique key should fail");
        assert!(empty_key_err.to_string().contains("empty value"));

        let missing_summary = statement
            .summary
            .require(10)
            .expect_err("missing summary field should fail");
        assert!(missing_summary
            .to_string()
            .contains("summary field 10 is missing"));

        let missing_required = statement.records[0]
            .require("missing")
            .expect_err("missing required field should fail");
        assert!(missing_required
            .to_string()
            .contains("required field missing"));

        let invalid_required_i64 = PaymentDownloadedBill::from_verified_bytes(
            bytes::Bytes::from_static(b"amount\nnot-number\nsum,total\n"),
            None,
            None,
        )
        .unwrap()
        .statement()
        .unwrap();
        let invalid_required_err = invalid_required_i64.records[0]
            .require_i64("amount")
            .expect_err("invalid required i64 should fail");
        assert!(invalid_required_err.to_string().contains("not a valid i64"));
    }

    #[test]
    fn rejects_non_utf8_payment_downloaded_bill() {
        let err = PaymentDownloadedBill::from_verified_bytes(
            bytes::Bytes::from_static(&[0xff]),
            None,
            None,
        )
        .expect_err("non UTF-8 bill should fail");

        assert!(err.to_string().contains("not valid UTF-8"));
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
    fn deserializes_refund_detail_response() {
        let response: RefundDetailResponse = serde_json::from_value(json!({
            "refund_id": "refund-id",
            "out_refund_no": "refund-1",
            "transaction_id": "transaction-1",
            "out_trade_no": "order-1",
            "channel": "ORIGINAL",
            "user_received_account": "零钱",
            "success_time": "2026-07-10T10:00:00+08:00",
            "create_time": "2026-07-10T09:59:00+08:00",
            "status": "SUCCESS",
            "funds_account": "AVAILABLE",
            "amount": {
                "refund": 50,
                "total": 100,
                "currency": "CNY",
                "from": [{ "account": "AVAILABLE", "amount": 50, "account_extra": "retained" }],
                "payer_total": 100,
                "payer_refund": 50,
                "settlement_refund": 50,
                "settlement_total": 100,
                "discount_refund": 0,
                "amount_extra": "retained"
            },
            "promotion_detail": [{
                "promotion_id": "promo-1",
                "scope": "GLOBAL",
                "type": "COUPON",
                "amount": 10,
                "refund_amount": 5,
                "promotion_extra": "retained",
                "goods_detail": [{
                    "merchant_goods_id": "sku-1",
                    "wechatpay_goods_id": "wx-sku-1",
                    "goods_name": "product",
                    "unit_price": 100,
                    "refund_amount": 5,
                    "refund_quantity": 1,
                    "goods_extra": "retained"
                }]
            }],
            "refund_extra": "retained"
        }))
        .unwrap();

        assert_eq!(response.refund_id, "refund-id");
        assert_eq!(response.status_kind(), PaymentRefundStatusKind::Success);
        assert!(response.is_success());
        assert!(response.status_kind().is_terminal());
        assert_eq!(
            PaymentRefundStatusKind::from_code("processing"),
            PaymentRefundStatusKind::Processing
        );
        assert!(!PaymentRefundStatusKind::Processing.is_terminal());
        assert!(PaymentRefundStatusKind::Closed.needs_attention());
        assert!(PaymentRefundStatusKind::Abnormal.needs_attention());
        assert!(PaymentRefundStatusKind::Other.needs_attention());
        assert_eq!(response.amount.from[0].account, "AVAILABLE");
        assert_eq!(response.amount.from[0].extra["account_extra"], "retained");
        assert_eq!(response.amount.payer_refund, Some(50));
        assert_eq!(response.amount.extra["amount_extra"], "retained");
        assert_eq!(response.promotion_detail[0].kind, "COUPON");
        assert_eq!(
            response.promotion_detail[0].extra["promotion_extra"],
            "retained"
        );
        assert_eq!(
            response.promotion_detail[0].goods_detail[0]
                .wechatpay_goods_id
                .as_deref(),
            Some("wx-sku-1")
        );
        assert_eq!(
            response.promotion_detail[0].goods_detail[0].extra["goods_extra"],
            "retained"
        );
        assert_eq!(response.extra["refund_extra"], "retained");
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
    fn serializes_profit_sharing_return_and_unfreeze_requests() {
        let returns = serde_json::to_value(ProfitSharingReturnOrderRequest {
            sub_mchid: Some("sub-mch".to_string()),
            order_id: None,
            out_order_no: Some("share-1".to_string()),
            out_return_no: "return-1".to_string(),
            return_mchid: "1900000109".to_string(),
            amount: 30,
            description: "return commission".to_string(),
        })
        .unwrap();
        assert_eq!(returns["sub_mchid"], "sub-mch");
        assert_eq!(returns["out_order_no"], "share-1");
        assert_eq!(returns["out_return_no"], "return-1");
        assert!(returns.get("order_id").is_none());

        let unfreeze = serde_json::to_value(ProfitSharingUnfreezeRequest {
            transaction_id: "4200000000".to_string(),
            out_order_no: "share-1".to_string(),
            description: "finish sharing".to_string(),
            sub_mchid: None,
        })
        .unwrap();
        assert_eq!(unfreeze["transaction_id"], "4200000000");
        assert_eq!(unfreeze["description"], "finish sharing");
        assert!(unfreeze.get("sub_mchid").is_none());
    }

    #[test]
    fn builds_profit_sharing_queries_and_legacy_return_params() {
        let query = ProfitSharingReturnOrderQuery {
            out_order_no: "share-1".to_string(),
            sub_mchid: Some("sub-mch".to_string()),
        }
        .into_query();
        assert_eq!(
            query,
            vec![
                ("out_order_no".to_string(), "share-1".to_string()),
                ("sub_mchid".to_string(), "sub-mch".to_string())
            ]
        );

        let bill = ProfitSharingBillRequest {
            bill_date: "2026-07-10".to_string(),
            tar_type: Some("GZIP".to_string()),
            sub_mchid: None,
        }
        .into_query();
        assert_eq!(
            bill,
            vec![
                ("bill_date".to_string(), "2026-07-10".to_string()),
                ("tar_type".to_string(), "GZIP".to_string())
            ]
        );

        let legacy = LegacyProfitSharingReturnRequest {
            appid: "wxappid".to_string(),
            out_order_no: "share-1".to_string(),
            out_return_no: "return-1".to_string(),
            return_account_type: "MERCHANT_ID".to_string(),
            return_account: "1900000109".to_string(),
            return_amount: "30".to_string(),
            description: "return commission".to_string(),
        }
        .into_params();
        assert!(legacy.contains(&("appid".to_string(), "wxappid".to_string())));
        assert!(legacy.contains(&("return_amount".to_string(), "30".to_string())));
        assert!(!legacy.iter().any(|(key, _)| key == "mch_id"));
        assert!(!legacy.iter().any(|(key, _)| key == "nonce_str"));
    }

    #[test]
    fn parses_legacy_profit_sharing_return_response() {
        let response: LegacyProfitSharingReturnResponse = quick_xml::de::from_str(
            "<xml><return_code>SUCCESS</return_code><mch_id>1900000109</mch_id><appid>wxappid</appid><order_id>order-1</order_id><out_order_no>share-1</out_order_no><out_return_no>return-1</out_return_no><return_no>wechat-return-1</return_no><result>SUCCESS</result></xml>",
        )
        .unwrap();

        assert_eq!(response.return_code, "SUCCESS");
        assert_eq!(response.out_return_no.as_deref(), Some("return-1"));
        assert_eq!(response.result.as_deref(), Some("SUCCESS"));
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
    fn serializes_transfer_detail_receipt_request_and_query() {
        let value = serde_json::to_value(TransferDetailReceiptRequest {
            accept_type: "BATCH_TRANSFER".to_string(),
            out_batch_no: "batch-1".to_string(),
            out_detail_no: "detail-1".to_string(),
        })
        .unwrap();

        assert_eq!(value["accept_type"], "BATCH_TRANSFER");
        assert_eq!(value["out_batch_no"], "batch-1");
        assert_eq!(value["out_detail_no"], "detail-1");

        let query = TransferDetailReceiptQuery {
            accept_type: "BATCH_TRANSFER".to_string(),
            out_batch_no: "batch-1".to_string(),
            out_detail_no: "detail-1".to_string(),
        }
        .into_query();

        assert_eq!(
            query,
            vec![
                ("accept_type".to_string(), "BATCH_TRANSFER".to_string()),
                ("out_batch_no".to_string(), "batch-1".to_string()),
                ("out_detail_no".to_string(), "detail-1".to_string())
            ]
        );
    }

    #[test]
    fn deserializes_transfer_receipt_responses() {
        let bill: TransferBillReceiptResponse = serde_json::from_value(json!({
            "out_batch_no": "batch-1",
            "signature_no": "signature-1",
            "signature_status": "FINISHED",
            "hash_type": "SHA256",
            "hash_value": "hash",
            "download_url": "https://example.com/receipt.pdf",
            "create_time": "2026-07-10T10:00:00+08:00",
            "update_time": "2026-07-10T10:01:00+08:00"
        }))
        .unwrap();
        assert_eq!(bill.signature_status, "FINISHED");
        assert_eq!(
            bill.download_url.as_deref(),
            Some("https://example.com/receipt.pdf")
        );

        let detail: TransferDetailReceiptResponse = serde_json::from_value(json!({
            "accept_type": "BATCH_TRANSFER",
            "out_batch_no": "batch-1",
            "out_detail_no": "detail-1",
            "signature_no": "signature-2",
            "signature_status": "PROCESSING",
            "hash_type": "SHA256",
            "hash_value": "hash",
            "download_url": "https://example.com/detail.pdf"
        }))
        .unwrap();
        assert_eq!(detail.out_detail_no, "detail-1");
        assert_eq!(detail.signature_no.as_deref(), Some("signature-2"));
    }

    #[test]
    fn builds_complaint_list_query() {
        let request = ComplaintListRequest::first_page("2026-07-01", "2026-07-06", 20)
            .for_complainted_merchant("mchid");
        assert!(request.validate().is_ok());
        let query = request.into_query();

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

        assert!(
            ComplaintListRequest::next_page("2026-07-01", "2026-07-31", 50, 50)
                .validate()
                .is_ok()
        );
        assert!(
            ComplaintListRequest::first_page("2026-07-31", "2026-07-01", 20)
                .validate()
                .is_err()
        );
        assert!(
            ComplaintListRequest::first_page("2026-07-01", "2026-08-01", 20)
                .validate()
                .is_err()
        );
        assert!(
            ComplaintListRequest::first_page("2026-02-30", "2026-03-01", 20)
                .validate()
                .is_err()
        );
        assert!(
            ComplaintListRequest::first_page("2026-07-01", "2026-07-01", 51)
                .validate()
                .is_err()
        );
        assert!(
            ComplaintListRequest::next_page("2026-07-01", "2026-07-01", 20, -1)
                .validate()
                .is_err()
        );
        assert!(
            ComplaintListRequest::first_page("2026-07-01", "2026-07-01", 20)
                .for_complainted_merchant("bad merchant")
                .validate()
                .is_err()
        );
    }

    #[test]
    fn builds_complaint_negotiation_history_query() {
        let request = ComplaintNegotiationHistoryRequest::next_page(10, 20);
        assert!(request.validate().is_ok());
        let query = request.into_query();

        assert_eq!(
            query,
            vec![
                ("limit".to_string(), "10".to_string()),
                ("offset".to_string(), "20".to_string())
            ]
        );
        assert!(ComplaintNegotiationHistoryRequest::first_page(300)
            .validate()
            .is_ok());
        assert!(ComplaintNegotiationHistoryRequest::first_page(301)
            .validate()
            .is_err());
        assert!(ComplaintNegotiationHistoryRequest::next_page(10, -1)
            .validate()
            .is_err());
        assert_eq!(
            encode_payment_path_segment("complaint/id"),
            "complaint%2Fid"
        );
    }

    #[test]
    fn deserializes_complaint_detail_and_list_response() {
        let detail: ComplaintDetailResponse = serde_json::from_value(json!({
            "complaint_id": "complaint-1",
            "complaint_time": "2026-07-16T10:00:00+08:00",
            "complaint_detail": "item not received",
            "complaint_state": "PENDING",
            "payer_phone": "13800000000",
            "payer_openid": "openid-1",
            "complaint_order_info": [{
                "transaction_id": "transaction-1",
                "out_trade_no": "order-1",
                "amount": 100,
                "merchant_extra_order_id": "extra-order-1"
            }],
            "complaint_full_refunded": false,
            "incoming_user_response": true,
            "user_complaint_times": 2,
            "complaint_media_list": [{
                "media_id": "media-detail-1",
                "media_type": "USER_COMPLAINT_IMAGE",
                "media_url": ["https://example.com/image.jpg"],
                "thumbnail_url": "https://example.com/thumb.jpg",
                "duration": 10
            }],
            "problem_description": "shipping issue",
            "problem_type": "SERVICE_NOT_WORK",
            "apply_refund_amount": 100,
            "user_tag_list": ["HIGH_RISK"],
            "service_order_info": [{
                "order_id": "service-order-1",
                "out_order_no": "out-service-1",
                "state": "DOING",
                "service_extra_state": "WAITING"
            }],
            "additional_info": {
                "type": "SHARE_POWER_BANK",
                "share_power_info": {
                    "return_time": "2026-07-16T11:00:00+08:00",
                    "is_returned_to_same_machine": true,
                    "return_status": "RETURNED",
                    "return_address_info": {
                        "return_address": "Shanghai",
                        "longitude": "121.47",
                        "latitude": "31.23",
                        "poi_id": "poi-1"
                    }
                }
            },
            "in_platform_service": true,
            "need_immediate_service": true,
            "is_agent_mode": false,
            "merchant_extra_detail": "retained"
        }))
        .unwrap();

        assert_eq!(detail.complaint_id.as_deref(), Some("complaint-1"));
        assert_eq!(
            detail.complaint_state_kind(),
            Some(ComplaintStateKind::Pending)
        );
        assert_eq!(
            detail.complaint_order_info[0].transaction_id.as_deref(),
            Some("transaction-1")
        );
        assert_eq!(
            detail.complaint_media_list[0].media_url[0],
            "https://example.com/image.jpg"
        );
        assert_eq!(
            detail.complaint_media_list[0].media_id.as_deref(),
            Some("media-detail-1")
        );
        assert_eq!(
            detail.complaint_media_list[0].media_kind(),
            Some(ComplaintMediaType::UserComplaintImage)
        );
        assert!(detail.complaint_media_list[0].is_image());
        assert!(!detail.complaint_media_list[0].is_video());
        assert_eq!(
            detail.complaint_media_list[0].thumbnail_url.as_deref(),
            Some("https://example.com/thumb.jpg")
        );
        assert_eq!(
            detail.complaint_order_info[0].extra["merchant_extra_order_id"],
            "extra-order-1"
        );
        assert_eq!(detail.complaint_media_list[0].extra["duration"], 10);
        assert_eq!(
            detail.service_order_info[0].extra["service_extra_state"],
            "WAITING"
        );
        assert_eq!(
            detail.service_order_info[0].state_kind(),
            Some(ComplaintServiceOrderStateKind::Doing)
        );
        assert_eq!(detail.extra["merchant_extra_detail"], "retained");
        assert_eq!(detail.payer_openid.as_deref(), Some("openid-1"));
        assert_eq!(detail.in_platform_service, Some(true));
        assert_eq!(detail.need_immediate_service, Some(true));
        assert_eq!(detail.is_agent_mode, Some(false));
        assert_eq!(
            detail.problem_type_kind(),
            Some(ComplaintProblemType::ServiceNotWork)
        );
        assert_eq!(
            detail.user_tag_kinds().collect::<Vec<_>>(),
            vec![ComplaintUserTag::HighRisk]
        );
        assert!(detail.has_identity());
        assert!(!detail.is_fully_refunded());
        assert!(detail.has_pending_user_response());
        assert_eq!(detail.identified_order_count(), 1);
        assert_eq!(detail.known_order_amount().unwrap(), 100);
        assert!(!detail.is_refund_request());
        assert!(detail.needs_merchant_response());
        assert!(detail.needs_priority_attention());
        assert!(detail.validate().is_ok());
        let single_media_detail: ComplaintDetailResponse = serde_json::from_value(json!({
            "complaint_media_list": {
                "media_type": "VIDEO",
                "media_url": "https://example.com/video.mp4"
            }
        }))
        .unwrap();
        assert_eq!(
            single_media_detail.complaint_media_list[0]
                .media_type
                .as_deref(),
            Some("VIDEO")
        );
        assert_eq!(
            single_media_detail.complaint_media_list[0].media_kind(),
            Some(ComplaintMediaType::Video)
        );
        assert!(single_media_detail.complaint_media_list[0].is_video());
        let other_media = ComplaintMedia {
            media_id: None,
            media_type: Some("AUDIO".to_string()),
            media_url: Vec::new(),
            thumbnail_url: None,
            extra: serde_json::Value::Null,
        };
        assert_eq!(other_media.media_kind(), Some(ComplaintMediaType::Other));
        assert_eq!(
            detail
                .additional_info
                .as_ref()
                .and_then(|info| info.info_type_kind()),
            Some(ComplaintAdditionalInfoType::SharePower)
        );
        assert_eq!(
            detail
                .additional_info
                .as_ref()
                .and_then(|info| info.share_power_info.as_ref())
                .and_then(|info| info.return_address_info.as_ref())
                .and_then(|info| info.return_address.as_deref()),
            Some("Shanghai")
        );
        assert_eq!(
            detail
                .additional_info
                .as_ref()
                .and_then(|info| info.share_power_info.as_ref())
                .and_then(|info| info.is_returned_to_same_machine),
            Some(true)
        );
        assert_eq!(
            detail
                .additional_info
                .as_ref()
                .and_then(|info| info.share_power_info.as_ref())
                .map(|info| &info.extra["return_status"]),
            Some(&json!("RETURNED"))
        );
        assert_eq!(
            detail
                .additional_info
                .as_ref()
                .and_then(|info| info.share_power_info.as_ref())
                .and_then(|info| info.return_address_info.as_ref())
                .map(|info| &info.extra["poi_id"]),
            Some(&json!("poi-1"))
        );

        let list: ComplaintListResponse = serde_json::from_value(json!({
            "total_count": 1,
            "limit": 10,
            "offset": 0,
            "data": [detail],
            "next_key": "cursor-1"
        }))
        .unwrap();

        assert_eq!(list.total_count, Some(1));
        assert_eq!(list.data[0].complaint_state.as_deref(), Some("PENDING"));
        assert_eq!(
            list.data[0].complaint_state_kind(),
            Some(ComplaintStateKind::Pending)
        );
        assert_eq!(list.extra["next_key"], "cursor-1");
        assert_eq!(list.has_more(), Some(false));
        assert_eq!(list.next_offset(), None);
        assert_eq!(list.identified_count(), 1);
        assert_eq!(list.pending_response_count(), 1);
        assert_eq!(list.priority_attention_count(), 1);
        assert!(list.find_complaint("complaint-1").is_some());
        let next_page: ComplaintListResponse = serde_json::from_value(json!({
            "total_count": 3,
            "offset": 1,
            "data": [{}]
        }))
        .unwrap();
        assert_eq!(next_page.has_more(), Some(true));
        assert_eq!(next_page.next_offset(), Some(2));
        let no_phone: ComplaintDetailResponse = serde_json::from_value(json!({})).unwrap();
        assert_eq!(no_phone.decrypt_payer_phone("not parsed").unwrap(), None);
        let completed_refund: ComplaintDetailResponse = serde_json::from_value(json!({
            "complaint_state": "PROCESSED",
            "problem_type": "REFUND",
            "incoming_user_response": true,
            "need_immediate_service": true,
            "user_tag_list": ["HIGH_RISK"]
        }))
        .unwrap();
        assert!(completed_refund.is_refund_request());
        assert!(!completed_refund.needs_merchant_response());
        assert!(!completed_refund.needs_priority_attention());
        assert_eq!(
            ComplaintStateKind::from_code("processed"),
            ComplaintStateKind::Processed
        );
        assert_eq!(
            ComplaintStateKind::from_code("COMPLETED"),
            ComplaintStateKind::Processed
        );
        assert!(ComplaintStateKind::Processed.is_terminal());
        assert_eq!(
            ComplaintStateKind::from_code("unknown"),
            ComplaintStateKind::Other
        );
        assert_eq!(
            ComplaintServiceOrderStateKind::from_code("waitpay"),
            ComplaintServiceOrderStateKind::WaitPay
        );
        assert!(ComplaintServiceOrderStateKind::Done.is_terminal());
        assert_eq!(
            ComplaintServiceOrderStateKind::from_code("unknown"),
            ComplaintServiceOrderStateKind::Other
        );
    }

    #[test]
    fn serializes_complaint_notification_request() {
        let request = ComplaintNotificationRequest {
            url: "https://example.com/complaints".to_string(),
        };
        assert!(request.validate().is_ok());
        let value = serde_json::to_value(request).unwrap();

        assert_eq!(value["url"], "https://example.com/complaints");
        assert!(ComplaintNotificationRequest {
            url: "http://example.com/complaints".to_string()
        }
        .validate()
        .is_err());
        assert!(ComplaintNotificationRequest {
            url: "https://example.com/complaints?token=secret".to_string()
        }
        .validate()
        .is_err());
        assert!(ComplaintNotificationRequest {
            url: "https://example.com/".to_string()
        }
        .validate()
        .is_err());
    }

    #[test]
    fn deserializes_complaint_notification_response() {
        let response: ComplaintNotificationResponse = serde_json::from_value(json!({
            "mchid": "1900000109",
            "url": "https://example.com/complaints",
            "notify_scene": "merchant-service"
        }))
        .unwrap();

        assert_eq!(response.mch_id.as_deref(), Some("1900000109"));
        assert_eq!(response.url, "https://example.com/complaints");
        assert_eq!(response.extra["notify_scene"], "merchant-service");
        assert!(response.is_configured());
        assert!(!response.has_api_error());

        let error: ComplaintNotificationResponse = serde_json::from_value(json!({
            "code": "INVALID_REQUEST",
            "message": "bad request",
            "request_id": "notify-error"
        }))
        .unwrap();
        assert_eq!(error.code.as_deref(), Some("INVALID_REQUEST"));
        assert_eq!(error.message.as_deref(), Some("bad request"));
        assert_eq!(error.url, "");
        assert_eq!(error.extra["request_id"], "notify-error");
        assert!(!error.is_configured());
        assert!(error.has_api_error());
    }

    #[test]
    fn validates_merchant_service_response_contracts() {
        let complaint = json!({
            "complaint_id": "complaint-1",
            "complaint_time": "2026-07-20T10:00:00+08:00",
            "complaint_state": "PENDING",
            "complaint_order_info": [{
                "transaction_id": "transaction-1",
                "out_trade_no": "order-1",
                "amount": 100
            }],
            "complaint_media_list": [{
                "media_id": "media-1",
                "media_type": "IMAGE",
                "media_url": ["https://example.com/image.jpg"]
            }],
            "user_tag_list": ["HIGH_RISK"],
            "service_order_info": [{
                "order_id": "service-order-1",
                "state": "DOING"
            }]
        });
        let list: ComplaintListResponse = serde_json::from_value(json!({
            "data": [complaint.clone()],
            "limit": 10,
            "offset": 0,
            "total_count": 1
        }))
        .unwrap();
        assert!(list.validate().is_ok());

        let inconsistent_page: ComplaintListResponse = serde_json::from_value(json!({
            "data": [complaint.clone()],
            "limit": 10,
            "offset": 1,
            "total_count": 1
        }))
        .unwrap();
        assert!(inconsistent_page.validate().is_err());

        let duplicate_complaints: ComplaintListResponse = serde_json::from_value(json!({
            "data": [complaint.clone(), complaint.clone()],
            "limit": 10,
            "offset": 0,
            "total_count": 2
        }))
        .unwrap();
        assert!(duplicate_complaints.validate().is_err());

        let negative_amount: ComplaintDetailResponse = serde_json::from_value(json!({
            "complaint_id": "complaint-2",
            "complaint_time": "2026-07-20T10:00:00Z",
            "complaint_state": "PENDING",
            "complaint_order_info": [{
                "transaction_id": "transaction-2",
                "amount": -1
            }]
        }))
        .unwrap();
        assert!(negative_amount.validate().is_err());

        let invalid_time: ComplaintDetailResponse = serde_json::from_value(json!({
            "complaint_id": "complaint-3",
            "complaint_time": "2026-07-20 10:00:00",
            "complaint_state": "PENDING"
        }))
        .unwrap();
        assert!(invalid_time.validate().is_err());

        let history: ComplaintNegotiationHistoryResponse = serde_json::from_value(json!({
            "data": [{
                "log_id": "log-1",
                "operator": "MERCHANT",
                "operate_time": "2026-07-20T10:10:00+08:00",
                "operate_type": "MERCHANT_RESPONSE",
                "image_list": ["https://example.com/reply.jpg"]
            }],
            "limit": 10,
            "offset": 0,
            "total_count": 1
        }))
        .unwrap();
        assert!(history.validate().is_ok());

        let duplicate_history: ComplaintNegotiationHistoryResponse =
            serde_json::from_value(json!({
                "data": [{
                    "log_id": "log-1",
                    "operator": "MERCHANT",
                    "operate_time": "2026-07-20T10:10:00+08:00",
                    "operate_type": "MERCHANT_RESPONSE"
                }, {
                    "log_id": "log-1",
                    "operator": "USER",
                    "operate_time": "2026-07-20T10:11:00+08:00",
                    "operate_type": "USER_RESPONSE"
                }],
                "limit": 10,
                "offset": 0,
                "total_count": 2
            }))
            .unwrap();
        assert!(duplicate_history.validate().is_err());

        let callback: ComplaintNotificationResponse = serde_json::from_value(json!({
            "mchid": "1900000109",
            "url": "https://example.com/complaints"
        }))
        .unwrap();
        assert!(callback.validate().is_ok());
        let callback_api_error: ComplaintNotificationResponse = serde_json::from_value(json!({
            "code": "INVALID_REQUEST",
            "message": "bad request"
        }))
        .unwrap();
        assert!(callback_api_error.validate().is_err());
        let insecure_callback: ComplaintNotificationResponse = serde_json::from_value(json!({
            "mchid": "1900000109",
            "url": "http://example.com/complaints"
        }))
        .unwrap();
        assert!(insecure_callback.validate().is_err());
    }

    #[test]
    fn serializes_complaint_complete_request() {
        let request = ComplaintCompleteRequest {
            complainted_mchid: "1900000109".to_string(),
        };
        assert!(request.validate().is_ok());
        let value = serde_json::to_value(request).unwrap();
        assert_eq!(value["complainted_mchid"], "1900000109");
        assert!(ComplaintCompleteRequest {
            complainted_mchid: " ".to_string()
        }
        .validate()
        .is_err());
    }

    #[test]
    fn serializes_complaint_reply_request() {
        let request = ComplaintReplyRequest {
            complainted_mchid: "1900000109".to_string(),
            response_content: "handled".to_string(),
            response_images: vec!["media-1".to_string()],
            jump_url: Some("https://example.com/detail".to_string()),
            jump_url_text: Some("detail".to_string()),
            mini_program_jump_info: Some(ComplaintMiniProgramJumpInfo {
                appid: "wx-app".to_string(),
                path: "/pages/complaint".to_string(),
                text: Some("complaint".to_string()),
                extra: serde_json::Value::Null,
            }),
        };
        assert!(request.validate().is_ok());
        let value = serde_json::to_value(request).unwrap();

        assert_eq!(value["complainted_mchid"], "1900000109");
        assert_eq!(value["response_content"], "handled");
        assert_eq!(value["response_images"][0], "media-1");
        assert_eq!(value["jump_url_text"], "detail");
        assert_eq!(value["mini_program_jump_info"]["appid"], "wx-app");

        let unpaired_link = ComplaintReplyRequest {
            complainted_mchid: "1900000109".to_string(),
            response_content: "handled".to_string(),
            response_images: Vec::new(),
            jump_url: Some("https://example.com/detail".to_string()),
            jump_url_text: None,
            mini_program_jump_info: None,
        };
        assert!(unpaired_link.validate().is_err());
        let duplicate_images = ComplaintReplyRequest {
            complainted_mchid: "1900000109".to_string(),
            response_content: "handled".to_string(),
            response_images: vec!["media".to_string(), "media".to_string()],
            jump_url: None,
            jump_url_text: None,
            mini_program_jump_info: None,
        };
        assert!(duplicate_images.validate().is_err());
        let missing_mini_program_text = ComplaintReplyRequest {
            complainted_mchid: "1900000109".to_string(),
            response_content: "handled".to_string(),
            response_images: Vec::new(),
            jump_url: None,
            jump_url_text: None,
            mini_program_jump_info: Some(ComplaintMiniProgramJumpInfo {
                appid: "wx-app".to_string(),
                path: "/pages/complaint".to_string(),
                text: None,
                extra: serde_json::Value::Null,
            }),
        };
        assert!(missing_mini_program_text.validate().is_err());
    }

    #[test]
    fn serializes_complaint_refund_progress_request() {
        let request = ComplaintRefundProgressRequest::approve(3).with_remark("refund accepted");
        assert!(request.validate().is_ok());
        assert_eq!(request.action_kind(), ComplaintRefundAction::Approve);
        let value = serde_json::to_value(request).unwrap();

        assert_eq!(value["action"], "APPROVE");
        assert_eq!(value["launch_refund_day"], 3);
        assert_eq!(value["remark"], "refund accepted");
        assert!(value.get("reject_media_list").is_none());

        let rejected = ComplaintRefundProgressRequest::reject("not eligible");
        assert!(rejected.validate().is_ok());
        assert_eq!(rejected.action_kind(), ComplaintRefundAction::Reject);
        assert!(ComplaintRefundProgressRequest::approve(-1)
            .validate()
            .is_err());
        assert!(ComplaintRefundProgressRequest::reject(" ")
            .validate()
            .is_err());
        let mixed = ComplaintRefundProgressRequest {
            action: "REJECT".to_string(),
            launch_refund_day: Some(1),
            reject_reason: Some("not eligible".to_string()),
            reject_media_list: Vec::new(),
            remark: None,
        };
        assert!(mixed.validate().is_err());
    }

    #[test]
    fn deserializes_complaint_negotiation_history_response() {
        let response: ComplaintNegotiationHistoryResponse = serde_json::from_value(json!({
            "total_count": 1,
            "limit": 10,
            "offset": 0,
            "data": [{
                "log_id": "log-1",
                "operator": "MERCHANT",
                "operate_type": "MERCHANT_RESPONSE",
                "merchant_history_state": "OPEN",
                "image_list": "https://example.com/history-inline.jpg",
                "complaint_media_list": [{
                    "media_id": "media-1",
                    "media_type": "IMAGE",
                    "media_url": "https://example.com/history.jpg",
                    "thumbnail_url": "https://example.com/history-thumb.jpg"
                }],
                "normal_message": {
                    "sender_identity": "MANUAL",
                    "custom_data": "merchant-data",
                    "blocks": [{
                        "type": "TEXT",
                        "text": {
                            "text": "handled",
                            "color": "DEFAULT",
                            "is_bold": true
                        }
                    }, {
                        "type": "BUTTON",
                        "button": {
                            "text": "details",
                            "action": {
                                "action_type": "ACTION_TYPE_JUMP_MINI_PROGRAM",
                                "mini_program_jump_info": {
                                    "appid": "wx-app",
                                    "path": "/pages/detail"
                                },
                                "action_id": "action-1"
                            }
                        }
                    }]
                },
                "click_message": {
                    "message_content": "details",
                    "action_id": "action-1",
                    "clicked_log_id": "log-0"
                }
            }],
            "next_key": "history-cursor-1"
        }))
        .unwrap();

        assert_eq!(response.total_count, Some(1));
        assert_eq!(response.has_more(), Some(false));
        assert_eq!(response.next_offset(), None);
        assert_eq!(response.identified_count(), 1);
        assert_eq!(response.refund_event_count(), 0);
        assert_eq!(response.system_event_count(), 0);
        assert!(response.find_log("log-1").is_some());
        assert_eq!(response.data[0].log_id.as_deref(), Some("log-1"));
        assert_eq!(
            response.data[0].operate_type_kind(),
            Some(ComplaintNegotiationOperateType::MerchantResponse)
        );
        assert_eq!(
            response.data[0].complaint_media_list[0]
                .media_type
                .as_deref(),
            Some("IMAGE")
        );
        assert_eq!(
            response.data[0].complaint_media_list[0].media_url[0],
            "https://example.com/history.jpg"
        );
        assert_eq!(
            response.data[0].image_list[0],
            "https://example.com/history-inline.jpg"
        );
        assert_eq!(response.extra["next_key"], "history-cursor-1");
        assert_eq!(response.data[0].extra["merchant_history_state"], "OPEN");
        assert_eq!(
            response.data[0].complaint_media_list[0].media_id.as_deref(),
            Some("media-1")
        );
        assert_eq!(
            response.data[0].complaint_media_list[0]
                .thumbnail_url
                .as_deref(),
            Some("https://example.com/history-thumb.jpg")
        );
        let normal = response.data[0].normal_message.as_ref().unwrap();
        assert!(normal.validate().is_ok());
        assert_eq!(normal.sender_identity.as_deref(), Some("MANUAL"));
        assert_eq!(
            normal.sender_identity_kind(),
            Some(ComplaintMessageSenderIdentity::Manual)
        );
        assert_eq!(
            normal.blocks[0].block_type_kind(),
            Some(ComplaintMessageBlockType::Text)
        );
        assert_eq!(
            normal.blocks[0]
                .text
                .as_ref()
                .and_then(|text| text.text.as_deref()),
            Some("handled")
        );
        assert_eq!(
            normal.blocks[1]
                .button
                .as_ref()
                .and_then(|button| button.action.as_ref())
                .and_then(|action| action.mini_program_jump_info.as_ref())
                .map(|jump| jump.appid.as_str()),
            Some("wx-app")
        );
        assert_eq!(
            normal.blocks[1]
                .button
                .as_ref()
                .and_then(|button| button.action.as_ref())
                .and_then(|action| action.action_type_kind()),
            Some(ComplaintMessageActionType::JumpMiniProgram)
        );
        assert_eq!(
            response.data[0]
                .click_message
                .as_ref()
                .and_then(|message| message.clicked_log_id.as_deref()),
            Some("log-0")
        );
        assert!(response.data[0]
            .click_message
            .as_ref()
            .unwrap()
            .validate()
            .is_ok());
        assert_eq!(
            ComplaintNegotiationOperateType::from_code("USER_COMFIRM_COMPLAINT"),
            ComplaintNegotiationOperateType::UserConfirmComplaint
        );
        assert_eq!(
            ComplaintNegotiationOperateType::from_code("COMPLAINT_PARTIAL_REFUNDED_SYSTEM_MESSAGE"),
            ComplaintNegotiationOperateType::ComplaintPartialRefunded
        );
        assert!(ComplaintNegotiationOperateType::ComplaintPartialRefunded.is_refund_event());
        assert!(ComplaintNegotiationOperateType::ComplaintPartialRefunded.is_system_event());
        assert_eq!(
            ComplaintNotificationActionKind::from_code("USER_APPLY_PLATFORM_SERIVCE"),
            ComplaintNotificationActionKind::UserApplyPlatformService
        );
    }

    #[test]
    fn validates_merchant_service_nested_response_contracts() {
        let valid_detail: ComplaintDetailResponse = serde_json::from_value(json!({
            "complaint_id": "complaint-nested-1",
            "complaint_time": "2026-07-20T10:00:00+08:00",
            "complaint_state": "PENDING",
            "additional_info": {
                "type": "SHARE_POWER_BANK",
                "share_power_info": {
                    "return_time": "2026-07-20T10:30:00+08:00",
                    "return_address_info": {
                        "return_address": "Shanghai",
                        "longitude": "121.4737",
                        "latitude": "31.2304"
                    }
                }
            }
        }))
        .unwrap();
        assert!(valid_detail.validate().is_ok());

        for additional_info in [
            json!({ "type": "SHARE_POWER_BANK" }),
            json!({
                "type": "SHARE_POWER_TYPE",
                "share_power_info": {
                    "return_time": "not-rfc3339",
                    "return_address_info": {
                        "return_address": "Shanghai",
                        "longitude": "121.47",
                        "latitude": "31.23"
                    }
                }
            }),
            json!({
                "type": "SHARE_POWER_TYPE",
                "share_power_info": {
                    "return_time": "2026-07-20T10:30:00+08:00",
                    "return_address_info": {
                        "return_address": "Shanghai",
                        "longitude": "181",
                        "latitude": "31.23"
                    }
                }
            }),
        ] {
            let detail: ComplaintDetailResponse = serde_json::from_value(json!({
                "complaint_id": "complaint-invalid",
                "complaint_time": "2026-07-20T10:00:00+08:00",
                "complaint_state": "PENDING",
                "additional_info": additional_info
            }))
            .unwrap();
            assert!(detail.validate().is_err());
        }

        let valid_history: ComplaintNegotiationHistoryResponse = serde_json::from_value(json!({
            "data": [{
                "log_id": "log-nested-1",
                "operator": "MERCHANT",
                "operate_time": "2026-07-20T10:10:00+08:00",
                "operate_type": "MERCHANT_RESPONSE",
                "normal_message": {
                    "sender_identity": "MANUAL",
                    "blocks": [{
                        "type": "BUTTON",
                        "button": {
                            "text": "details",
                            "action": {
                                "action_type": "ACTION_TYPE_JUMP_URL",
                                "jump_url": "https://example.com/details",
                                "action_id": "action-1"
                            }
                        }
                    }]
                },
                "click_message": {
                    "message_content": "details",
                    "action_id": "action-1",
                    "clicked_log_id": "log-parent"
                }
            }],
            "limit": 10,
            "offset": 0,
            "total_count": 1
        }))
        .unwrap();
        assert!(valid_history.validate().is_ok());

        for normal_message in [
            json!({ "blocks": [] }),
            json!({
                "blocks": [{
                    "type": "TEXT",
                    "image": { "media_id": "media-1" }
                }]
            }),
            json!({
                "blocks": [{
                    "type": "BUTTON",
                    "button": {
                        "text": "details",
                        "action": {
                            "action_type": "ACTION_TYPE_JUMP_URL",
                            "jump_url": "http://example.com/details"
                        }
                    }
                }]
            }),
        ] {
            let history: ComplaintNegotiationHistoryResponse = serde_json::from_value(json!({
                "data": [{
                    "log_id": "log-invalid",
                    "operator": "MERCHANT",
                    "operate_time": "2026-07-20T10:10:00+08:00",
                    "operate_type": "MERCHANT_RESPONSE",
                    "normal_message": normal_message
                }],
                "limit": 10,
                "offset": 0,
                "total_count": 1
            }))
            .unwrap();
            assert!(history.validate().is_err());
        }

        let invalid_click: ComplaintNegotiationHistoryResponse = serde_json::from_value(json!({
            "data": [{
                "log_id": "log-invalid-click",
                "operator": "USER",
                "operate_time": "2026-07-20T10:10:00+08:00",
                "operate_type": "USER_CLICK_RESPONSE",
                "click_message": { "action_id": "action-1" }
            }],
            "limit": 10,
            "offset": 0,
            "total_count": 1
        }))
        .unwrap();
        assert!(invalid_click.validate().is_err());

        let missing_click: ComplaintNegotiationHistoryResponse = serde_json::from_value(json!({
            "data": [{
                "log_id": "log-missing-click",
                "operator": "USER",
                "operate_time": "2026-07-20T10:10:00+08:00",
                "operate_type": "USER_CLICK_RESPONSE"
            }],
            "limit": 10,
            "offset": 0,
            "total_count": 1
        }))
        .unwrap();
        assert!(missing_click.validate().is_err());
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
            location: Some(PayScoreLocation {
                start_location: Some("A".to_string()),
                end_location: Some("B".to_string()),
                start_latitude: Some(31.2304),
                start_longitude: Some(121.4737),
                end_latitude: None,
                end_longitude: None,
                start_address: Some("Start address".to_string()),
                end_address: None,
                extra: serde_json::Value::Null,
            }),
            attach: None,
        })
        .unwrap();

        assert_eq!(value["appid"], "wxappid");
        assert_eq!(value["service_id"], "service-id");
        assert_eq!(value["out_order_no"], "score-order-1");
        assert_eq!(value["time_range"]["start_time_remark"], "start");
        assert_eq!(value["risk_fund"]["amount"], 100);
        assert_eq!(value["need_user_confirm"], true);
        assert_eq!(value["location"]["start_location"], "A");
        assert_eq!(value["location"]["start_latitude"], 31.2304);
        assert_eq!(value["location"]["start_address"], "Start address");
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
    fn deserializes_merchant_fund_balance_response() {
        let response: MerchantFundBalanceResponse = serde_json::from_value(json!({
            "available_amount": 1000,
            "pending_amount": 200
        }))
        .unwrap();

        assert_eq!(response.available_amount, 1000);
        assert_eq!(response.pending_amount, 200);
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
