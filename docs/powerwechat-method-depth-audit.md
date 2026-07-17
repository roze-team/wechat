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
| Work | 363 | 283 | high |
| Payment | 165 | 96 | high |
| Open Platform | 76 | 61 | medium |
| Mini Program | 214 | 178 | medium |
| Official Account | 283 | 244 | medium |
| Open Work | 57 | 43 | medium |
| Basic Service | 33 | 24 | low |
| Channels | 6 | 4 | low |

Counts are directional because Roze sometimes merges several PowerWeChat helper
methods into one typed wrapper, and PowerWeChat includes non-endpoint helpers.

## Priority Updates

1. Work method-depth parity:
   exact endpoint coverage is now green against the current PowerWeChat Work
   scan, but the method surface still needs deeper typed DTO normalization and
   semantic helper polish across `externalContact`, `oa`, `user`, `message`,
   `media`, and webhook/token flows.

2. Payment method-depth parity:
   stream-download bill helpers and merchant-service complaint DTOs have been
   deepened. Continue expanding `notify`, `order`, and remaining payment method
   variants with strong typed request/response DTOs plus signature/decryption
   tests where applicable.

3. Open Platform authorizer depth:
   exact endpoint coverage is now green against the current PowerWeChat scan.
   Continue DTO normalization for authorizer mini-program release/audit,
   privacy, domain, tester, material, and aggregate account flows.

4. Mini Program depth:
   exact endpoint coverage is now green after filtering scanner-only
   documentation paths. Continue DTO normalization across `liveBroadcast`,
   `industry/miniDrama/vod`, `express`, `immediateDelivery`, `b2b`,
   `dataCube`, `operation`, and `wxa`.

5. Official Account depth:
   exact endpoint coverage is now green against the current PowerWeChat scan.
   Continue DTO normalization for `broadcasting`, `material`, `menu`,
   `templateMessage`, and `publish`.

6. Open Work depth:
   exact endpoint coverage is now green against the current PowerWeChat scan.
   Continue DTO normalization for `license`, `suitAuth`, `server`, and
   component/base authorization helpers.

## Endpoint Audit

The following exact endpoint comparison was generated from the latest
PowerWeChat checkout on 2026-07-10. It is intentionally conservative: dynamic
paths can be reported as missing when PowerWeChat uses `%s` formatting and Roze
uses Rust `format!` placeholders.

| Family | PowerWeChat endpoints found | Exact endpoints not found in Roze | Highest-impact update areas |
| --- | ---: | ---: | --- |
| Work | 261 | 0 | exact endpoint scan green; continue method/DTO depth review |
| Mini Program | 151 | 0 | exact endpoint scan green after filtering documentation-path false positives; continue method/DTO depth review |
| Open Platform | 48 | 0 | exact endpoint scan green; continue method/DTO depth review |
| Official Account | 201 | 0 | exact endpoint scan green; continue method/DTO depth review |
| Basic Service | 14 | 0 | exact endpoint scan green; continue method/DTO depth review |
| Open Work | 38 | 0 | exact endpoint scan green; continue method/DTO depth review |
| Channels | 2 | 0 | none from exact endpoint scan |

Implemented on 2026-07-16 in Roze WeChat Work external contact depth:

- contact-way list/update/delete and temporary-chat close wrappers;
- external contact remark wrapper;
- corp-tag list/add/edit/delete and customer tag-mark wrappers;
- external group-chat list/get/transfer wrappers.
- contact-way add/get/list responses now preserve unknown upstream fields
  across wrappers, list items, and detail payloads.
- external group-chat join-way add/get wrappers now preserve unknown upstream
  fields alongside typed join-way details.
- external group welcome-template add/get wrappers now preserve unknown upstream
  fields alongside typed message payload details.
- customer-acquisition link list/get/create/update/delete wrappers;
- customer-acquisition link create/get responses now expose typed link DTOs
  with range, priority, timestamp, URL, and extension-field preservation.
- customer-acquisition link list/create/get responses now preserve unknown
  upstream fields across wrappers, range, and priority-option payloads.
- external group-chat join-way get responses now expose typed config, QR code,
  scene, room, chat-list, state, and extension fields instead of raw JSON.
- external-contact group message template, task/result, welcome, remind, and
  cancel wrappers;
- external-customer transfer, transfer-result, unassigned-list, and resigned
  transfer/result wrappers.
- external-contact moment list/task/customer/send-result/comment/create/result
  wrappers;
- external-contact user-behavior and group-chat statistics wrappers;
- external-contact customer-strategy list/get/range/create/edit/delete wrappers.
- linked-corp message send and external-contact school notice message send
  wrappers.
- user batch sync/replace/replace-party/get-result wrappers;
- user export simple-user/user/department/tag-user/get-result wrappers.
- user create/update/delete/batch-delete/list-id/mobile/email/auth-success,
  invite, join-qrcode, and active-stat wrappers.
- linked-corp permission, user detail, user simple/detail list, and department
  list wrappers.
- OA check-in corp/user option, record, day/month data, schedule list/set,
  face, option add/update/delete wrappers;
- OA approval template/detail/list/data/apply wrappers and vacation
  config/quota/update wrappers.
- external-contact strategy-tag list/add/edit/delete wrappers, group welcome
  template add/edit/get/delete wrappers, group-chat join-way add/get/update/delete
  wrappers, and opengid-to-chatid wrapper.
- external-contact list/detail/follow-user/batch-detail responses now expose
  typed contact, profile, follow-info, and tag DTOs.
- account-service sync-message responses now expose typed text/media/link/menu
  and event message-body DTOs.
- agent scope/workbench wrappers, auth exact-path user info/detail wrappers,
  department get/simple-list wrappers, external-contact new-id and union-id
  conversion wrappers, moment-strategy list/range/create/edit/delete wrappers,
  template-card update wrapper, msg-audit agree/robot-info wrappers, tag user
  removal wrapper, group robot send/upload wrappers, and Work access-token
  wrapper.
- Work media upload-image, temp-media, attachment, and group-robot upload
  responses now preserve upstream extension fields while keeping typed media ids.
- Work agent list/detail and department list/simple-list/detail responses now
  expose typed agent, allow-scope, department, and department-leader DTOs.
- Work agent, department, base IP/token, corpgroup, and mini-program session
  response wrappers now preserve unknown upstream fields across wrappers and
  response-only nested records.
- Work agent scope, workbench-template, and workbench-data mutation paths now
  accept typed request DTOs instead of raw JSON request payloads.
- Work appchat create/get and OAuth/Auth user info/detail responses now expose
  typed chat, user-ticket, identity, and profile DTOs.
- Work invoice, external-pay, menu, and appchat response DTOs now preserve
  upstream extension fields across wrappers and response-only nested records.
- Work OAuth/Auth user info/detail responses now preserve unknown upstream
  identity and profile fields.
- Work msg-audit permit-user, chat-data, room, agree-status, and robot-info
  responses now expose typed audit metadata DTOs.
- Work msg-audit response wrappers and nested chat-data, room-member,
  agree-info, and robot-info records now preserve unknown upstream fields.
- Work agent workbench-template response now exposes typed key-data, image,
  list, and webview template DTOs.
- typed Work message audience and helper wrappers for markdown, image, voice,
  file, video, text-card, news, mpnews, and mini-program notice sends.
- linked-corp and external-contact school message requests now include typed
  builder helpers for text/media payloads and recipient targeting, reducing
  raw JSON construction at call sites while keeping extension fields.
- external-contact contact-way add/get/list responses and conclusions are now
  typed instead of generic JSON values.
- external-contact base/detail/batch DTOs now preserve unknown upstream fields
  across contact, profile, attribute, follow-info, tag, and list wrappers.
- external-contact base/detail DTOs now expose semantic helper enums for
  contact type, gender, and external-profile attribute type while retaining the
  original upstream numeric fields.
- external-contact group-chat list/get/transfer responses now expose typed
  chat summaries, chat details, members, admins, and failed-transfer records.
- external-contact group-chat list/get/transfer/open-gid DTOs now preserve
  unknown upstream fields across wrappers, summaries, members, admins, and
  failed-transfer records.
- external-contact group-chat member DTOs now expose semantic member-kind
  helpers for enterprise members and external contacts while retaining the
  original upstream numeric member type.
- Work user, linked-corp user, async import/export job, exported-user, and
  external-contact group-chat status fields now expose semantic helper enums
  while retaining original upstream numeric fields and `Other(i64)` fallbacks.
- external-contact group-message template add/list/task/send-result responses
  now expose typed fail IDs, message text/attachments, tasks, and send results.
- external-contact group-message task/send-result records now expose semantic
  status helpers for unsent, sent, customer-not-friend, duplicate-delivery, and
  receive-limit outcomes while retaining the original numeric `status` code.
- external-contact group-message template/list/task/send-result DTOs now
  preserve unknown upstream fields across wrappers, message payloads,
  attachments, tasks, and send-result records.
- external-contact customer-strategy list/get/range/create responses now expose
  typed strategy IDs, strategy metadata, privileges, and ranges.
- external-contact corp-tag, strategy-tag, and moment-strategy responses now
  expose typed tag-group, tag, strategy, range, and strategy-id DTOs.
- external-contact corp-tag, strategy-tag, moment-strategy, and customer-strategy
  DTOs now preserve unknown upstream fields across wrappers, groups, tags,
  strategies, and ranges.
- external-contact moment and statistics responses now expose typed moment,
  task, customer, comment/like, task-result, behavior, and group-chat statistic
  DTOs.
- external-contact moment list/task/customer/comment/task-result DTOs now
  preserve unknown upstream fields across response wrappers and per-record
  payloads.
- external-contact user-behavior and group-chat statistics DTOs now preserve
  unknown upstream fields across response wrappers, items, and data payloads.
- external-customer transfer and unassigned-customer responses now expose typed
  customer transfer records and unassigned customer metadata.
- external-customer transfer-result records now expose semantic status helpers
  for completed, pending, refused, takeover-limit, and no-record outcomes while
  retaining the original numeric `status` code.
- external-customer transfer-result and unassigned-customer DTOs now preserve
  unknown upstream fields across response wrappers and per-customer records.
- account-service responses now expose typed customer-service accounts,
  customers, synchronized messages, servicer operation results, and servicer
  list records.
- Work status/ticket, AI Bot long-connection, and account-service sync/send,
  servicer, state, and tag responses now preserve unknown upstream fields
  across wrappers and response-only nested records.
- account-service account, contact-way, customer batch, customer profile,
  enter-session context, and upgrade-config responses now preserve unknown
  upstream fields.
- linked-corp user/department responses and user batch/export job results now
  expose typed user, department, import-result, and export-data DTOs.
- Work user get/simple-list/detail-list responses now expose typed user,
  department, and external-profile DTOs.
- Work user create/update and batch-delete mutation paths now use typed request
  DTOs covering department/order, leader, contact, invite, avatar, external
  profile, and extended-attribute payloads.
- Work user extended attributes and batch import/replace callbacks now use
  typed DTOs instead of raw JSON payloads.
- Work user list/detail/invite/join-QR and batch import/export result DTOs now
  preserve upstream extension fields for forward-compatible enterprise data.
- Work user ID conversion and active-stat response DTOs now preserve upstream
  extension fields across wrappers and per-item records.
- Work user detail and linked-corp permission/user/department response DTOs now
  preserve upstream extension fields across wrappers and nested records.
- Work app message send payloads now expose typed text, markdown, and text-card
  message DTOs on the primary `WorkMessage` request instead of generic JSON.
- Work app message media, video, news, mpnews, and mini-program notice payloads
  now expose typed DTOs on the primary `WorkMessage` request and helper paths.
- Work app, linked-corp, and external-contact school message send responses now
  preserve upstream extension fields alongside typed invalid-recipient fields.
- linked-corp and external-contact school message payloads now reuse typed text,
  media, video, news, mpnews, and mini-program notice DTOs instead of raw JSON.
- account-service send-message and send-on-event requests now reuse typed text,
  media, video, link, mini-program, menu, location, and customer-acquisition link DTOs.
- group robot text, markdown, and file message payloads now expose typed DTOs
  instead of generic JSON values.
- group robot image, news, and template-card message payloads now expose typed
  DTOs while preserving card extension fields.
- Work upload-media responses now expose semantic media-type helpers for image,
  voice, video, and file while retaining the original upstream `type` string.
- Work invoice status requests and batch invoice responses now expose semantic
  reimbursement-status helpers for init, locked, and closure states while
  retaining the original upstream string values.
- Work remaining raw request payloads have been removed: moment-strategy
  create/edit, linked-corp and external-contact school message sends,
  template-card update, check-in option add/update, approval apply-event, and
  vacation quota update now accept explicit request DTOs.
- invoice info/batch responses and account-service tag detail/list responses
  now expose typed invoice user-info, line-item, invoice batch-item, tag-user,
  and tag DTOs.
- external-contact group-message template, welcome-message, and moment-task
  requests now reuse typed text/attachment DTOs with constructors for image,
  link, mini-program, video, and file attachments instead of raw JSON vectors.
- external group welcome-template request and response payloads now reuse typed
  text, image, link, mini-program, file, and video DTOs instead of raw JSON.
- external-contact group-chat owner filters, statistic owner filters, and
  moment-task visible ranges now use typed DTOs instead of raw JSON values.
- external-pay merchant and bill-list responses now expose typed use-scope and
  bill DTOs.
- corp-group app-share responses now expose typed shared-corp records.
- OA check-in option, record, data, and schedule responses now expose typed
  group, user-option, record, data-item, and schedule DTOs.
- OA check-in, approval, and vacation response DTOs now preserve unknown
  upstream fields across wrappers and nested records.
- OA calendar, dial, PSTNCC, and schedule responses now expose typed calendar,
  dial-record, call-state, and schedule DTOs.
- OA calendar, dial, PSTNCC, journal, and schedule response DTOs now preserve
  unknown upstream fields across wrappers and nested records.
- OA meeting-room and WeDrive responses now expose typed room, booking, space,
  file, and move-result DTOs.
- OA meeting, meeting-room, and WeDoc response DTOs now preserve unknown
  upstream fields across wrappers and nested room/booking records.
- OA journal detail/stat and living info/watch-stat responses now expose typed
  journal, statistic, live-info, and watch-stat DTOs.
- OA living and WeDrive response DTOs now preserve unknown upstream fields
  across wrappers, info payloads, file records, and move failures.

Implemented on 2026-07-16 in Roze WeChat Open Platform authorizer depth:

- authorizer mini-program code release/audit wrappers: commit, QR code bytes,
  category/page list, submit/get/latest audit, release, withdraw, rollback,
  visit status, gray release/revert/plan, support version, quota, and speedup;
- authorizer mini-program domain, tester, privacy setting, privacy ext-file,
  privacy interface apply/get, and jscode2session wrappers;
- authorizer account basic info, head image, signature, material bytes, open
  account create/bind/unbind/get, fast-registration URL, component login URL,
  and fast-register reuse wrappers.
- authorizer info/option/list and code-template draft/list public wrappers now
  return typed Open Platform DTOs instead of raw JSON values.
- authorizer mini-program submit-audit requests, category responses, rollback
  versions, gray-release plans, support-version UV info, domain mutation
  results, tester members, and privacy setting payloads now expose typed DTOs
  while keeping extension fields for future WeChat additions.
- authorizer mini-program code/audit category, page, submit, status, latest,
  rollback, gray-release, support-version, and quota response wrappers now
  preserve unknown upstream fields alongside typed DTOs.
- authorizer mini-program privacy-interface responses now expose typed
  interface status/audit DTOs and preserve upstream extension fields across
  list and apply responses.
- authorizer account basic-info responses now expose typed verification,
  signature, and head-image DTOs while preserving upstream extension fields.
- authorizer mini-program category, code-audit, latest-audit, and privacy
  interface DTOs now expose semantic status helper enums while retaining the
  original upstream numeric status fields.
- authorizer mini-program gray-release plans now expose semantic release-state
  helpers for initial/running/paused/finished/deleted states while retaining
  the original upstream numeric status field.

Implemented on 2026-07-16 in Roze WeChat Mini Program base/message/live depth:

- base access-token, paid-union-id, and encrypted-data check wrappers;
- customer-service temporary media download/upload wrappers;
- uniform-message and updatable-message domain helpers and send/create wrappers;
- data-cube performance data wrapper;
- live-business goods warehouse, follower list, and push-message wrappers;
- image security multipart upload wrapper.
- liveBroadcast create-room, live-info, replay, goods warehouse, and follower
  responses now expose typed room/goods/replay/follower DTOs.
- liveBroadcast response DTOs now preserve unknown upstream fields across
  create-room, room, goods, replay, warehouse, and follower payloads.
- express order, account, delivery, waybill, path, and contact responses now
  expose typed logistics DTOs.
- express logistics response DTOs now preserve unknown upstream fields across
  order, waybill, account, delivery, path, printer, quota, contact, and
  preview-template payloads.
- express batch order query requests now use typed order-id/openid/delivery-id
  and waybill-id DTOs instead of raw JSON order entries.
- immediateDelivery bind-account, provider-list, and order-detail responses now
  expose typed shop/provider/order DTOs instead of raw JSON arrays.
- immediateDelivery bind-account, provider-list, cancel/pre-cancel, pre-add,
  order-detail, and re-order responses now preserve unknown upstream fields
  across wrappers and nested shop/provider payloads.
- operation feedback, gray-release, JS-error, scene, client-version, and
  real-time-log responses now expose typed operation DTOs.
- wxa sec-order detail/list responses now expose typed order, amount, shipping,
  and shipping-item DTOs.
- wxa sec-order detail/list and trade-management response DTOs now preserve
  unknown upstream fields across wrappers, orders, amount, shipping, and
  shipping-item payloads.
- dataCube visit-trend, retain, visit-page, user-portrait, performance, and
  security check responses now expose typed Mini Program DTOs.
- uniform-message send, updatable-message send, and dataCube performance-data
  request paths now accept typed Mini Program DTOs instead of raw JSON request
  payloads.
- wxa code QR/code/unlimited JSON response paths now expose typed ticket, URL,
  buffer, and error metadata DTOs alongside existing bytes helpers.
- miniDrama/VOD upload-task, media-list/info/link, drama-list/info,
  latest-audit, CDN usage/log, package-list, and authorization responses now
  expose typed DTOs instead of generic JSON arrays/objects.
- Channels shop/store basic info now share the typed basic-info response DTO.
- Official Account module response surfaces no longer expose `Result<Value>`;
  OAuth user info, broadcasting, customer-service account lists, material,
  menu, semantic, template-message, user-list, and blacklist paths now return
  explicit typed response DTOs with compatibility extension fields where the
  upstream payload remains open-ended.
- Official Account menu get/current/try-match responses now expose typed menu
  and button DTOs instead of raw JSON values, including conditional-menu
  match-rule metadata.
- Official Account mass-preview now accepts a typed request DTO with explicit
  preview recipient and message type fields.
- Official Account mass-send status responses now expose semantic status
  helpers for sending/success/failure/deleted states while retaining the
  original upstream `msg_status` string.
- Official Account publish draft/list/status/article response DTOs now preserve
  unknown upstream fields across wrappers, content blocks, article details, and
  news items.
- Official Account card base-info and free-publish status DTOs now expose
  semantic status helper enums while retaining the original upstream status
  fields.

The raw Mini Program endpoint scanner still reports 23 entries, but they are
documentation or path-template noise such as `*.html`, `express/response`, and
`wxa/sec/order/request` rather than callable PowerWeChat endpoints.

Implemented on 2026-07-17 in Roze WeChat Mini Program response depth:

- live room responses now expose semantic helper enums for `liveStatus`, live
  goods `priceType`, and warehouse `auditStatus`, while retaining raw upstream
  numeric fields and unknown-field capture.
- immediate-delivery order/reorder responses now expose semantic order-status
  helpers, including success/failure terminal checks and `Other(i64)` fallback.
- WXA security order-shipping responses now expose semantic order-state helpers
  for pending shipment, shipped, confirmed received, completed, and refunded
  states while retaining raw `order_state`.

Implemented on 2026-07-16 in Roze WeChat Official Account exact endpoint depth:

- base callback/quota wrappers: clear quota, callback IP list, and callback URL
  network check;
- card batch list and update wrappers with typed responses;
- customer-service avatar upload, session list/waiting/create/close/get, and
  message-record wrappers with typed session/record DTOs;
- template subscribe-message send wrapper;
- user openid migration wrapper;
- user tag create/list/update/delete, user-tag IDs, users-of-tag, batch tag,
  and batch untag wrappers with typed tag/list responses.
- card get/code responses now expose typed card, card-type detail, base-info,
  date-info, SKU, and code-card DTOs instead of generic JSON values.

Implemented on 2026-07-16 in Roze WeChat Open Work exact endpoint depth:

- component/base pre-authorization code wrapper;
- component authorization query and authorizer info/list wrappers;
- component authorizer option get/set wrappers;
- component quota clear wrapper;
- typed component preauth, query-auth, authorizer info/list, and option
  responses.
- component/base preauth, query-auth, authorizer info/list, option wrappers,
  and authorizer-list summaries now preserve unknown upstream fields.
- suite pre-auth, permanent-code, auth-info, and corp-token public wrappers now
  return typed Open Work DTOs instead of raw JSON values.
- suite permanent-code/auth-info responses now expose typed auth-corp,
  auth-agent, auth-user, register-code, and dealer-corp DTOs.
- suite session/auth-agent DTOs now expose semantic helpers for official/test
  authorization type and admin/member authorization mode while retaining the
  original upstream numeric fields.
- suite pre-auth, permanent-code, and corp-token response wrappers now preserve
  unknown upstream fields across wrapper and nested authorization DTO payloads.
- license renew-order job invalid accounts, order list/detail, and trial-info
  responses now expose typed Open Work DTOs instead of generic JSON values.
- license order/account/user-active/auto-active DTOs now expose semantic helper
  enums for order type, order status, account type, user activation, and
  automatic activation while retaining original upstream numeric fields.
- license order/job/account/active/transfer/status response wrappers now
  preserve unknown upstream fields alongside typed license DTOs.
- server callback XML can now be parsed into typed Open Work suite-ticket,
  create/change/cancel-auth, and reset-permanent-code events with an unknown
  fallback for future WeCom event types.
- server callback events now expose semantic event-kind helpers for auth
  lifecycle and ticket/permanent-code refresh handling while preserving the
  original `InfoType` string.

Payment uses dedicated v3/v2 request helpers in PowerWeChat, so it needs a
separate path scan rather than the generic `HttpPostJson` endpoint extractor.
The approximate payment scan found 69 payment paths and 37 paths that still
need review. Some are formatting false positives, but the real update areas are:

- remaining payment stream-download helpers, statement helpers, and deeper
  merchant-service response DTO normalization;

Implemented on 2026-07-16 in Roze WeChat payment download and complaint depth:

- signed WeChat Pay v3 bill download bytes helper with absolute/relative
  download URL parsing;
- trade-bill and fund-flow-bill download convenience wrappers;
- optional SHA-1/SHA-256 download hash verification for returned bill bytes;
- structured downloaded-bill wrapper exposing verified bytes, UTF-8 text,
  non-empty line count, header, summary, and data-row iteration for statement
  ingestion code;
- downloaded bill statements now expose structured header-keyed records with
  CSV quote handling and WeChat/Excel cell-prefix cleanup for production
  ingestion;
- downloaded bill statements now expose a first-class statement helper with
  parsed headers, records, summary fields, and checked integer accessors for
  record/summary amounts;
- downloaded bill statements now provide checked column-sum and non-empty
  column-count helpers for reconciliation code;
- downloaded bill statements now provide required-field, required-column, and
  column-sum-vs-summary validation helpers for production reconciliation
  checks;
- downloaded bill statements now provide required summary-field helpers and
  data-row-count-vs-summary validation for production reconciliation checks;
- downloaded bill statements now provide unique-column indexing, grouped
  integer sums, and non-empty-column-count-vs-summary validation helpers for
  production reconciliation checks;
- downloaded bill statements now provide filter-by-column helpers with
  filtered amount sums and non-empty counts for partial reconciliation checks.
- downloaded bill statements now provide multi-column filters, filtered amount
  sums, unique-column lookup, grouped record counts, and grouped non-empty
  counts for production reconciliation code.
- downloaded bill statements now provide filtered grouped record counts and
  filtered grouped integer sums for merchant/app/state scoped reconciliation.
- generic payment status responses now preserve unknown upstream fields such
  as request ids across mutation endpoints.
- shared signed bytes GET support in `PlatformClient`;
- structured merchant-service complaint list/detail/negotiation DTOs covering
  order info, media lists, service-order info, user tags, refund amount, and
  additional shared-power return details.
- merchant-service complaint media fields now deserialize as typed media-list
  DTOs in both detail and negotiation-history responses while accepting
  upstream array and single-object shapes.
- merchant-service complaint list/detail/history/nested DTOs now preserve
  unknown upstream fields and accept both string and array media URLs.
- merchant-service complaint notification delete, user reply, completion, and
  refund-progress mutation paths now return dedicated response DTOs instead of
  the generic payment status response.
- merchant-service complaint notification query and action response DTOs now
  preserve unknown upstream fields across success and error payloads.
- merchant-service complaint image and merchant media upload responses now
  preserve unknown upstream fields alongside typed media ids.
- merchant-service complaint media DTOs now expose typed media id and thumbnail
  URL fields, negotiation image lists accept single-string or array shapes, and
  notification query responses tolerate WeChat code/message error payloads.
- merchant-service complaint media DTOs now expose semantic media-kind helpers
  for image/video/other classification while retaining the original upstream
  media type string.
- merchant-service complaint detail/list/complete and related service-order
  DTOs now expose semantic complaint/service-order state helpers while
  retaining original upstream strings and tolerating legacy completed-state
  spelling.

Implemented on 2026-07-16 in Roze WeChat payment notify and PayScore DTO depth:

- `PaymentResource.original_type` support for WeChat Pay v3 encrypted
  notifications;
- typed transaction-success, refund, and merchant-transfer bill notification
  payload DTOs for production webhook handlers;
- transaction/refund notification nested amount, payer, scene, promotion, and
  goods DTOs now preserve unknown upstream fields for forward compatibility.
- order query and transaction notification DTOs now expose semantic trade-state
  helpers, and refund detail/notification DTOs expose semantic refund-status
  helpers while retaining the original upstream status strings.
- refund detail responses now preserve unknown upstream fields across wrapper,
  amount, source-account, promotion, and goods DTO payloads.
- order and partner-order responses now expose typed transaction fields,
  amount, payer, scene, promotion, and goods DTOs while preserving unknown
  upstream fields; prepay/H5/native response wrappers also retain extensions.
- typed PayScore service-order response covering state, post-payment/discount,
  risk-fund, time-range, location, order id, and package fields.
- PayScore service-order request/response location payloads now share a typed
  location DTO with coordinate/address fields and extension support.

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

Implemented on 2026-07-16 in Roze WeChat Basic Service exact endpoint depth:

- JSSDK media bytes download wrapper;
- mini-program subscribe-message send wrapper;
- subscribe template add/delete, category, keyword, title, and personal-template
  list wrappers;
- typed subscribe-template add/category/keyword/title/list responses.

## Concrete Next Batch

Recommended implementation order:

1. Payment remaining method-depth review for `notify`, `order`, refunds,
   partner flows, statement variants, and typed response normalization.
2. Work `externalContact` depth, especially contact way, customer acquisition,
   group chat, group message, tag, moment, strategy, and transfer endpoints.
3. Mini Program DTO/method-depth review for `liveBroadcast`,
   `industry/miniDrama/vod`, `express`, `immediateDelivery`, `b2b`,
   `dataCube`, `operation`, and `wxa`.
4. Payment remaining statement/download DTO normalization and helper variants.
5. Continue cross-family DTO hardening where endpoint coverage is already green.

## Documentation Update Needed

Keep `docs/powerwechat-gap-analysis.md` as the submodule-level view, but do not
use it as the final production parity signal. This method-depth audit should be
updated whenever a family reaches one-to-one endpoint coverage or PowerWeChat
adds new methods.
