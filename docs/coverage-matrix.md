# Coverage Matrix

Status values:

- `foundation`: shared SDK/service foundation exists.
- `module`: module boundary and product entry exist.
- `typed`: typed request/response wrappers implemented.
- `tested`: typed wrappers plus protocol or API tests implemented.
- `pending`: not implemented yet.

| Family | PowerWeChat module | Roze WeChat module | Status |
| --- | --- | --- | --- |
| Basic Service | contentSecurity | `basic_service::content_security` | tested |
| Basic Service | jssdk | `basic_service::jssdk` | tested |
| Basic Service | media | `basic_service::media` | tested |
| Basic Service | qrCode | `basic_service::qr_code` | tested |
| Basic Service | subscribeMessage | `basic_service::subscribe_message` | tested |
| Basic Service | url | `basic_service::url` | tested |
| Channels | eCommerce | `channels::e_commerce` | tested |
| Mini Program | auth | `mini_program::auth` | tested |
| Mini Program | base | `mini_program::base` | tested |
| Mini Program | b2b | `mini_program::b2b` | tested |
| Mini Program | customerServiceMessage | `mini_program::customer_service_message` | tested |
| Mini Program | dataCube | `mini_program::data_cube` | tested |
| Mini Program | device | `mini_program::device` | tested |
| Mini Program | express | `mini_program::express` | tested |
| Mini Program | image | `mini_program::image` | tested |
| Mini Program | immediateDelivery | `mini_program::immediate_delivery` | tested |
| Mini Program | internet | `mini_program::internet` | tested |
| Mini Program | liveBroadcast | `mini_program::live_broadcast` | tested |
| Mini Program | nearbyPoi | `mini_program::nearby_poi` | tested |
| Mini Program | ocr | `mini_program::ocr` | tested |
| Mini Program | operation | `mini_program::operation` | tested |
| Mini Program | phoneNumber | `mini_program::phone_number` | tested |
| Mini Program | plugin | `mini_program::plugin` | tested |
| Mini Program | riskControl | `mini_program::risk_control` | tested |
| Mini Program | search | `mini_program::search` | tested |
| Mini Program | security | `mini_program::security` | tested |
| Mini Program | server | `mini_program::server` | tested |
| Mini Program | serviceMarket | `mini_program::service_market` | tested |
| Mini Program | soter | `mini_program::soter` | tested |
| Mini Program | subscribe/uniform/updatable messages | `mini_program::messages` | tested |
| Mini Program | urlScheme/urlLink/shortLink | `mini_program::url` | tested |
| Mini Program | virtualPayment | `mini_program::virtual_payment` | tested |
| Mini Program | wxaCode | `mini_program::wxa_code` | tested |
| Mini Program | wxa/sec/order | `mini_program::wxa_sec_order` | tested |
| Official Account | auth/oauth | `official_account::oauth` | tested |
| Official Account | base | `official_account::base` | tested |
| Official Account | broadcasting | `official_account::broadcasting` | tested |
| Official Account | card | `official_account::card` | tested |
| Official Account | customerService | `official_account::customer_service` | tested |
| Official Account | jssdk | `official_account::jssdk` | tested |
| Official Account | material | `official_account::material` | tested |
| Official Account | menu | `official_account::menu` | tested |
| Official Account | server | `official_account::server` | tested |
| Official Account | templateMessage | `official_account::template_message` | tested |
| Official Account | user/tag | `official_account::user` | tested |
| Open Platform | auth | `open_platform::auth` | tested |
| Open Platform | authorizer | `open_platform::authorizer` | tested |
| Open Platform | codeTemplate | `open_platform::code_template` | tested |
| Open Platform | component | `open_platform::component` | tested |
| Open Platform | server | `open_platform::server` | tested |
| Open Work | provider/suitAuth/corp | `open_work` | tested |
| Payment | apply4Sub | `payment::apply4_sub` | tested |
| Payment | bill | `payment::bill` | tested |
| Payment | base | `payment::base` | tested |
| Payment | fundApp | `payment::fund_app` | tested |
| Payment | jssdk | `payment::jssdk` | tested |
| Payment | merchant | `payment::merchant` | tested |
| Payment | merchantService | `payment::merchant_service` | tested |
| Payment | notify | `payment::notify` | tested |
| Payment | order | `payment::order` | tested |
| Payment | partner | `payment::partner` | tested |
| Payment | payScore | `payment::pay_score` | tested |
| Payment | profitSharing | `payment::profit_sharing` | tested |
| Payment | promotion | `payment::promotion` | tested |
| Payment | redpack | `payment::redpack` | tested |
| Payment | refund | `payment::refund` | tested |
| Payment | reverse | `payment::reverse` | tested |
| Payment | sandbox | `payment::sandbox` | tested |
| Payment | security | `payment::security` | tested |
| Payment | tax | `payment::tax` | tested |
| Payment | transfer | `payment::transfer` | tested |
| Work | agent | `work::agent` | tested |
| Work | auth/oauth | `work::oauth` | tested |
| Work | department/user/tag | `work::contact` | tested |
| Work | externalContact | `work::external_contact` | tested |
| Work | groupRobot | `work::group_robot` | tested |
| Work | jssdk | `work::jssdk` | tested |
| Work | media | `work::media` | tested |
| Work | message | `work::message` | tested |
| Work | msgAudit | `work::msg_audit` | tested |
| Work | oa | `work::oa` | tested |
| Work | server | `work::server` | tested |

The generic `PlatformClient` can call every WeChat endpoint before a typed
wrapper exists. Production readiness for a module requires `typed` or `tested`.
