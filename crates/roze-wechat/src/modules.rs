#[path = "modules_impl/basic_service.rs"]
pub mod basic_service;
#[path = "modules_impl/channels.rs"]
pub mod channels;
#[path = "modules_impl/mini_program.rs"]
pub mod mini_program;
#[path = "modules_impl/official_account.rs"]
pub mod official_account;
#[path = "modules_impl/open_platform.rs"]
pub mod open_platform;
#[path = "modules_impl/open_work.rs"]
pub mod open_work;
#[path = "modules_impl/payment.rs"]
pub mod payment;
#[path = "modules_impl/platform_client.rs"]
mod platform_client;
#[path = "modules_impl/work.rs"]
pub mod work;

pub use platform_client::{DomainModule, PlatformClient};

use crate::{config::Platform, Client};

#[derive(Debug, Clone)]
pub struct Wechat {
    client: Client,
}

impl Wechat {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub fn basic_service(&self) -> basic_service::BasicService {
        basic_service::BasicService::new(self.client.clone(), Platform::BasicService)
    }

    pub fn channels(&self) -> channels::Channels {
        channels::Channels::new(self.client.clone(), Platform::Channels)
    }

    pub fn mini_program(&self) -> mini_program::MiniProgram {
        mini_program::MiniProgram::new(self.client.clone(), Platform::MiniProgram)
    }

    pub fn official_account(&self) -> official_account::OfficialAccount {
        official_account::OfficialAccount::new(self.client.clone(), Platform::OfficialAccount)
    }

    pub fn open_platform(&self) -> open_platform::OpenPlatform {
        open_platform::OpenPlatform::new(self.client.clone(), Platform::OpenPlatform)
    }

    pub fn open_work(&self) -> open_work::OpenWork {
        open_work::OpenWork::new(self.client.clone(), Platform::OpenWork)
    }

    pub fn payment(&self) -> payment::Payment {
        payment::Payment::new(self.client.clone(), Platform::Payment)
    }

    pub fn work(&self) -> work::Work {
        work::Work::new(self.client.clone(), Platform::Work)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Client, WechatConfig};

    use super::Wechat;

    #[test]
    fn exposes_product_module_entries() {
        let client = Client::new(WechatConfig::default()).unwrap();
        let wechat = Wechat::new(client);

        assert_eq!(
            wechat.official_account().menu().name(),
            "official_account.menu"
        );
        assert_eq!(wechat.mini_program().auth().name(), "mini_program.auth");
        assert_eq!(
            wechat.mini_program().wxa_sec_order().name(),
            "mini_program.wxa_sec_order"
        );
        assert_eq!(
            wechat.mini_program().risk_control().name(),
            "mini_program.risk_control"
        );
        assert_eq!(wechat.mini_program().ocr().name(), "mini_program.ocr");
        assert_eq!(wechat.mini_program().image().name(), "mini_program.image");
        assert_eq!(wechat.mini_program().device().name(), "mini_program.device");
        assert_eq!(
            wechat.mini_program().operation().name(),
            "mini_program.operation"
        );
        assert_eq!(wechat.mini_program().server().name(), "mini_program.server");
        assert_eq!(wechat.mini_program().search().name(), "mini_program.search");
        assert_eq!(
            wechat.mini_program().nearby_poi().name(),
            "mini_program.nearby_poi"
        );
        assert_eq!(wechat.mini_program().plugin().name(), "mini_program.plugin");
        assert_eq!(
            wechat.mini_program().virtual_payment().name(),
            "mini_program.virtual_payment"
        );
        assert_eq!(wechat.mini_program().b2b().name(), "mini_program.b2b");
        assert_eq!(
            wechat.mini_program().industry_mini_drama_vod().name(),
            "mini_program.industry_mini_drama_vod"
        );
        assert_eq!(
            wechat.mini_program().immediate_delivery().name(),
            "mini_program.immediate_delivery"
        );
        assert_eq!(
            wechat.mini_program().express().name(),
            "mini_program.express"
        );
        assert_eq!(wechat.mini_program().soter().name(), "mini_program.soter");
        assert_eq!(
            wechat.mini_program().service_market().name(),
            "mini_program.service_market"
        );
        assert_eq!(
            wechat.mini_program().internet().name(),
            "mini_program.internet"
        );
        assert_eq!(wechat.payment().notify().name(), "payment.notify");
        assert_eq!(wechat.payment().apply4_sub().name(), "payment.apply4_sub");
        assert_eq!(wechat.payment().base().name(), "payment.base");
        assert_eq!(wechat.payment().merchant().name(), "payment.merchant");
        assert_eq!(
            wechat.payment().merchant_service().name(),
            "payment.merchant_service"
        );
        assert_eq!(wechat.payment().fund_app().name(), "payment.fund_app");
        assert_eq!(wechat.payment().pay_score().name(), "payment.pay_score");
        assert_eq!(wechat.payment().promotion().name(), "payment.promotion");
        assert_eq!(wechat.payment().redpack().name(), "payment.redpack");
        assert_eq!(wechat.payment().security().name(), "payment.security");
        assert_eq!(wechat.payment().sandbox().name(), "payment.sandbox");
        assert_eq!(wechat.payment().tax().name(), "payment.tax");
        assert_eq!(wechat.work().message().name(), "work.message");
        assert_eq!(
            wechat.open_platform().component().name(),
            "open_platform.component"
        );
        assert_eq!(
            wechat
                .open_platform()
                .authorizer_official_account()
                .menu()
                .name(),
            "official_account.menu"
        );
        assert_eq!(wechat.open_platform().base().name(), "open_platform.base");
        assert_eq!(wechat.open_work().provider().name(), "open_work.provider");
        assert_eq!(wechat.open_work().base().name(), "open_work.base");
        assert_eq!(wechat.open_work().suit_auth().name(), "open_work.suit_auth");
        assert_eq!(wechat.open_work().corp().name(), "open_work.corp");
        assert_eq!(wechat.open_work().user().name(), "open_work.user");
        assert_eq!(
            wechat.open_work().external_contact().name(),
            "open_work.external_contact"
        );
        assert_eq!(wechat.open_work().license().name(), "open_work.license");
        assert_eq!(wechat.open_work().server().name(), "open_work.server");
    }
}
