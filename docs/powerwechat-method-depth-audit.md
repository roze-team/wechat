# PowerWeChat Method-Depth Audit

Audit date: 2026-07-10.

The submodule-level coverage matrix is currently green. This means every
PowerWeChat product submodule has an explicit Roze WeChat boundary and tested
typed wrappers for the core paths.

It does not mean every PowerWeChat public Go method has a one-to-one Rust
wrapper yet. The generic `PlatformClient` can still call uncovered endpoints,
but these areas should be expanded for stricter production parity.

## Snapshot

| Family | PowerWeChat public methods | Roze public async wrappers | Update need |
| --- | ---: | ---: | --- |
| Work | 363 | 190 | high |
| Payment | 165 | 93 | high |
| Open Platform | 76 | 23 | high |
| Mini Program | 214 | 165 | medium |
| Official Account | 283 | 222 | medium |
| Open Work | 57 | 36 | medium |
| Basic Service | 33 | 16 | low |
| Channels | 6 | 4 | low |

Counts are directional because Roze sometimes merges several PowerWeChat helper
methods into one typed wrapper, and PowerWeChat includes non-endpoint helpers.

## Priority Updates

1. Work method-depth parity:
   continue expanding `externalContact`, `oa`, `user`, `message`, and `media`.
   These still have the largest endpoint surface compared with current Roze
   wrappers.

2. Payment method-depth parity:
   continue expanding `merchantService`, `notify`, `order`, and remaining
   download/statement helper variants. These are production-sensitive and
   should keep strong typed request/response DTOs plus signature/decryption
   tests where applicable.

3. Open Platform authorizer depth:
   PowerWeChat has many authorizer mini-program/official-account aggregate
   clients for code release, audit, tester, account, privacy, setting, domain,
   material, and aggregate account flows.

4. Mini Program depth:
   expand `liveBroadcast`, `industry/miniDrama/vod`, `express`,
   `immediateDelivery`, `b2b`, `dataCube`, `operation`, and `wxa`.

5. Official Account depth:
   submodules are now covered, but `broadcasting`, `customerService`, `material`,
   `user/tag`, `card`, `menu`, `templateMessage`, and `publish` still deserve
   more one-to-one helper wrappers.

6. Open Work depth:
   `license`, `suitAuth`, and `server` should be expanded toward PowerWeChat's
   method surface.

## Endpoint Audit

The following exact endpoint comparison was generated from the latest
PowerWeChat checkout on 2026-07-10. It is intentionally conservative: dynamic
paths can be reported as missing when PowerWeChat uses `%s` formatting and Roze
uses Rust `format!` placeholders.

| Family | PowerWeChat endpoints found | Exact endpoints not found in Roze | Highest-impact update areas |
| --- | ---: | ---: | --- |
| Work | 261 | 113 | external contact, check-in, department/user batch/export, message variants, OA |
| Mini Program | 151 | 41 | live broadcast goods/roles/room operations, uniform/updatable messages, business/security paths |
| Open Platform | 48 | 36 | authorizer mini-program code/audit/privacy/domain/tester/account flows |
| Official Account | 200 | 22 | user tags, customer-service sessions/message records, card update/list, base callback/quota |
| Basic Service | 12 | 7 | subscribe message template management |
| Open Work | 35 | 7 | component authorizer management and quota paths |
| Channels | 2 | 0 | none from exact endpoint scan |

Implemented on 2026-07-16 in Roze WeChat Work external contact depth:

- contact-way list/update/delete and temporary-chat close wrappers;
- external contact remark wrapper;
- corp-tag list/add/edit/delete and customer tag-mark wrappers;
- external group-chat list/get/transfer wrappers.
- customer-acquisition link list/get/create/update/delete wrappers;
- external-contact group message template, task/result, welcome, remind, and
  cancel wrappers;
- external-customer transfer, transfer-result, unassigned-list, and resigned
  transfer/result wrappers.
- external-contact moment list/task/customer/send-result/comment/create/result
  wrappers;
- external-contact user-behavior and group-chat statistics wrappers;
- external-contact customer-strategy list/get/range/create/edit/delete wrappers.

Payment uses dedicated v3/v2 request helpers in PowerWeChat, so it needs a
separate path scan rather than the generic `HttpPostJson` endpoint extractor.
The approximate payment scan found 69 payment paths and 37 paths that still
need review. Some are formatting false positives, but the real update areas are:

- remaining payment stream-download helpers, statement helpers, and deeper
  merchant-service response DTO normalization;

Implemented on 2026-07-10 in Roze WeChat payment depth:

- legacy balance transfer query/create and bank-card transfer query:
  `gettransferinfo`, `promotion/transfers`, `query_bank`;
- Work redpack and mini-program redpack paths;
- profit sharing return orders, unfreeze, bills, transaction amount query, and
  legacy `secapi/pay/profitsharingreturn`.
- partner combine app transaction, partner transaction-id query, and payment
  codepay;
- merchant fund balance;
- fund-app electronic-sign transfer-bill-no apply/query variants.
- typed refund detail responses, partner out-refund query, and transfer bill
  receipt/electronic receipt apply/query wrappers.

## Concrete Next Batch

Recommended implementation order:

1. Payment remaining stream-download/statement helper depth, because these
   touch money movement and need stronger typed DTOs plus signing tests.
2. Work `externalContact` depth, especially contact way, customer acquisition,
   group chat, group message, tag, moment, strategy, and transfer endpoints.
3. Open Platform `authorizer` depth for mini-program release/audit/domain and
   privacy operations.
4. Mini Program `liveBroadcast` depth, especially goods, room assistant,
   sub-anchor, role, replay/comment/KF toggles, and follower/message helpers.
5. Official Account `user/tag` and `customerService` depth.

## Documentation Update Needed

Keep `docs/powerwechat-gap-analysis.md` as the submodule-level view, but do not
use it as the final production parity signal. This method-depth audit should be
updated whenever a family reaches one-to-one endpoint coverage or PowerWeChat
adds new methods.
