# PowerWeChat Method-Depth Audit

Audit date: 2026-07-18.

The submodule-level coverage matrix is currently green. This means every
PowerWeChat product submodule has an explicit Roze WeChat boundary and tested
typed wrappers for the core paths.

It does not mean every PowerWeChat public Go method has a one-to-one Rust
wrapper yet. The generic `PlatformClient` can still call uncovered endpoints,
but these areas should be expanded for stricter production parity.

## Snapshot

| Family | PowerWeChat public methods | Roze public async wrappers | Update need |
| --- | ---: | ---: | --- |
| Work | 363 | 370 | high |
| Payment | 165 | 110 | high |
| Open Platform | 76 | 69 | medium |
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

Implemented on 2026-07-18 in Roze WeChat Open Work license depth:

- License activation records now expose typed merge and upstream/downstream
  sharing details while retaining unknown response fields.
- License account status covers unbound, active, expired, pending-transfer,
  merged, and downstream-shared states.
- License order semantics now cover migration and multi-corporation orders,
  and distinguish refunding, refunded, refund-rejected, and invalid states.
- Order/account pagination and application license-check status expose
  production-oriented semantic helpers.

Implemented on 2026-07-18 in Roze WeChat Open Work server depth:

- Server callback parsing now recognizes contact user/department/tag changes,
  shared-agent and shared-chain changes, organization authorization, special
  authorization approval/cancellation, and application-admin changes.
- Enterprise WeChat `AuthCode` and `TimeStamp` XML names are accepted alongside
  existing aliases, and typed callback fields retain event-specific payloads.

Implemented on 2026-07-18 in Roze WeChat suite authorization depth:

- Authorized applications now expose typed contact privileges, shared-source
  corporations, customized-app flags, and enterprise location.
- Permanent-code and authorization responses expose typed edition agents,
  edition IDs/names, paid/trial status, user limits, expiration, virtual
  editions, and cross-corporation sharing state.
- Privilege levels and application edition states provide semantic write,
  active-entitlement, payment, and shared-install helpers.

Implemented on 2026-07-18 in Roze WeChat component/base depth:

- Component query-auth and authorizer-info responses now reuse the shared,
  verified authorization and authorizer aggregate contracts exposed by Open
  Platform, with Open Work-specific public type aliases.
- Authorization scopes, confirmation state, account metadata, mini-program
  domains/categories, and nested extension fields are no longer raw JSON.

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

Implemented on 2026-07-18 in Roze WeChat Work message endpoint depth:

- Application-message statistics now expose typed today/yesterday requests,
  per-application counters, total aggregation, agent lookup, and
  forward-compatible response fields.
- Legacy interactive task cards now support typed send payloads and a
  convenience sender, including replacement text, color, and bold button
  options.
- Task-card state updates now expose typed recipients, task/button identifiers,
  invalid-user parsing, and delivery-failure semantics.
- The current WxJava Work `user`, `message`, and `media` exact-path comparison
  is now green.

Implemented on 2026-07-18 in Roze WeChat Work OA exact endpoint depth:

- Approval template copying and the OA-prefixed meeting-detail route are now
  covered with typed responses.
- Meeting-room booking through schedules or meetings and booking-ID detail
  lookup expose typed conflict and schedule data.
- WeDoc content reads/edits, document-image multipart upload, and advanced
  account add/delete/list operations are now covered.
- WeDoc smart-sheet content-permission get/update operations preserve
  compatibility aliases and expose semantic effective-value helpers.
- The current WxJava Work OA exact-path comparison is green after excluding its
  duplicated-slash `book_by_meeting` constant typo.

Implemented on 2026-07-18 in Roze WeChat Work external-contact operations:

- External-contact operations now cover service-provider ID conversion and
  migration completion, served-contact pagination, on-job group ownership
  transfer, legacy group-message result compatibility, single unassigned
  contact transfer, moment-task cancellation, external-contact OpenID
  conversion, and school-notification subscription QR-code/mode management.
- Served contacts and legacy group-message results use typed records with
  pagination and forward-compatible extension fields; current message delivery
  status semantics are reused for legacy result records. The current WxJava
  external-contact exact-path comparison is now green.

Implemented on 2026-07-18 in Roze WeChat Work external-contact message depth:

- enterprise group-message `tag_filter` is now a typed group/tag DTO instead
  of arbitrary JSON, with constructors, tag counting, non-empty ID checks, and
  the upstream 100-tags-per-group limit;
- group-message template requests expose semantic `single`/`group` chat-type
  classification and are validated before the HTTP request is sent;
- validation enforces audience-field separation, required sender rules,
  10,000-customer and 2,000-chat audience limits, the nine-attachment limit,
  and a non-empty message payload;
- `ExternalContactCursorPage` now provides a shared `next_cursor`/`has_more`
  contract across 20 served-contact, detail, contact-way, group-chat,
  acquisition, product, group-message, transfer, unassigned, strategy, and
  moment response types, treating blank cursors as terminal pages.

Implemented on 2026-07-18 in Roze WeChat Work external-contact management:

- External-contact sensitive-word interception now covers add, update, delete,
  list, and detail operations. Applicable ranges, semantic interception rules,
  summaries, details, and interception behavior are typed with semantic enums.
- External-contact product albums now cover add, update, delete, list, and
  detail operations. Product prices, codes, timestamps, image attachments, and
  pagination are typed, with an image-attachment constructor and forward-
  compatible extension fields.

Implemented on 2026-07-18 in Roze WeChat Work external-contact depth:

- Customer acquisition now covers quota monitoring, customer attribution
  pagination, link usage statistics, and repeated-message chat details through
  four typed wrappers. Quota batches, attributed customers, chat status,
  conversion counters, and chat details are typed while preserving unknown
  upstream fields.
- Quota responses expose exhausted and next-expiring-batch helpers, and
  attributed customers expose semantic chat-status classification.

Implemented on 2026-07-18 in Roze WeChat Work user depth:

- Work member authentication now covers the current two-factor verification
  lifecycle: `auth/get_tfa_info` exchanges the single-use authorization code
  for a member ID and `tfa_code`, while `user/tfa_succ` submits the verified
  result. Both requests and the information response use typed DTOs, and the
  response preserves unknown upstream fields.

Implemented on 2026-07-18 in Roze WeChat Work message depth:

- Work application-message responses now expose `unlicenseduser` and semantic
  list helpers for invalid users, departments, tags, and unlicensed users,
  including a single delivery-failure check for partial-send handling.
- Work group robots now support typed `markdown_v2` and voice payloads with
  constructors and semantic message-type detection, matching the current
  webhook message contract.

Implemented on 2026-07-18 in Roze WeChat Work media depth:

- Work media now covers the current asynchronous CDN upload lifecycle:
  `media/upload_by_url`, `media/get_upload_by_url_result`, and
  `upload_media_job_finish` callback job IDs. Scene and supported media types,
  task status, result details, media IDs, timestamps, and upstream extension
  fields are typed; terminal and successful-result helpers avoid raw status
  checks in application code.

Implemented on 2026-07-18 in Roze WeChat Work message-audit depth:

- Single-chat agreement queries now use typed internal-member/external-contact
  conversation pairs and reject empty query sets or blank identifiers before
  sending a request.
- Agreement responses now retain the upstream `status_change_time`, expose
  semantic agree/disagree/unknown status classification, and provide
  all-agreed and any-disagreement helpers for compliance decisions.
- The historical upstream `exteranalopenid` spelling remains the wire contract,
  while Rust callers use the correctly named `external_openid` field; group
  agreement responses remain valid without a member `userid`.

Implemented on 2026-07-17 in Roze WeChat Work WeDoc depth:

- WeDoc document batch updates now expose typed operations for replacing,
  inserting, and deleting text/content; inserting images, page breaks,
  paragraphs, and tables; and updating text properties.
- Document locations and ranges are typed, range end positions are checked,
  and future operation objects remain round-trip compatible through extension
  fields.
- Requests are validated before transport for non-empty document IDs,
  non-negative versions and locations, the 30-operation batch limit, the
  50-range text-operation limit, paired positive image dimensions, and the
  upstream 100-row/60-column/1,000-cell table constraints.
- OA WeDoc now covers document create/rename/delete/base-info/share and
  collection-form create/modify/info/statistics/answer lifecycles. The
  collection-form create/modify paths use the current upstream
  `create_collect` and `modify_collect` endpoints instead of PowerWeChat's
  stale `create_form` path.
- OA WeDoc form definitions now expose typed questions, options, fill and
  manager ranges, repeat settings, statistics, submitters, unfilled members,
  answers, option/file/department/member/duration replies, and pagination
  metadata. Polymorphic question extension settings remain structured JSON,
  while response wrappers and nested records preserve unknown upstream fields.
- OA WeDoc permission management now covers typed access rules, internal and
  external defaults, administrator-only approvals, external-sharing controls,
  member and department permissions, readonly behavior, and watermark
  settings.
- OA WeDoc advanced-feature account management uses the current
  `wedoc/vip/batch_add`, `wedoc/vip/batch_del`, and `wedoc/vip/list` contracts,
  including typed success/failure lists and cursor pagination, instead of the
  older per-document administrator endpoints.
- OA WeDoc document content supports versioned reads and batch updates through
  the current `wedoc/document/get` and `wedoc/document/batch_update`
  endpoints. Polymorphic document blocks and update operations remain
  structured JSON while their versioned envelopes are typed.
- OA WeDoc spreadsheet support now covers worksheet properties, A1 range
  reads, and ordered batch updates for adding/deleting sheets, updating cell
  ranges, and deleting row/column dimensions. Range and batch responses follow
  the upstream `data.result` and `data.responses` wrappers with typed cells,
  links, text formats, colors, operation results, and extension fields.
- OA WeDoc smart-sheet support now covers sub-sheet, view, field, and record
  create/read/update/delete lifecycles through 16 current
  `wedoc/smartsheet/*` endpoints. Stable identifiers, pagination, versions,
  timestamps, and response envelopes are typed and preserve upstream extension
  fields. Polymorphic properties, cell values, sorts, and filters remain
  structured JSON so every current and future field type can round-trip without
  lossy SDK updates.
- OA WeDoc smart-sheet field groups now cover add/update/list/delete with typed
  field membership and offset pagination. Content-permission reads expose typed
  rules, per-sheet capabilities, record ranges, field ranges, field-specific
  and default permissions, while preserving mixed-format rule identifiers and
  future extension fields.

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
- OA approval template names, controls, properties, selector configurations,
  application approvers, summaries, filters, form contents, table rows,
  aggregate fields, approval records, comments, process nodes, and legacy
  approval records now use typed DTOs instead of generic JSON values while
  preserving upstream extension fields.
- OA journal record filters, submitters, receivers, read receipts, form
  contents, and comments now use typed DTOs; journal statistics now follow the
  upstream `stat_list` array contract with typed report/white ranges, fixed
  receivers, leaders, reported users, unreported users, and report items.
- OA living create/modify requests now model optional wire fields and typed
  activity details; living information exposes the documented viewer, online,
  replay, reservation, microphone, stream, subscription, and comment fields.
  Watch statistics now follow the upstream internal `users` and
  `external_users` contract with typed viewer records and the integer `ending`
  pagination marker. Living IDs and viewing codes use their documented string
  wire types, and living-detail retrieval uses the documented GET query.
- OA WeDrive space/file ACL requests and space-member responses now expose
  typed member/department permission records. Space creation supports the
  documented subtype, space details expose typed member, quit-user, and
  security settings, and the current `new_space_info` endpoint is available.
- OA WeDrive file list, rename, and move responses now use the upstream
  `{ item: [...] }` wrapper and typed file size, timestamps, type, status,
  creator/updater, digest, URL, and extension fields instead of fabricated
  flat lists or success/failure move records. The current typed `file_info`
  retrieval endpoint is also available.
- OA meeting create/update requests and meeting detail responses now expose
  typed internal-member, external-user, and device attendee DTOs instead of
  generic maps.
- OA meeting-room add/edit/list responses now expose typed latitude/longitude
  coordinates; location, equipment, booking filters, subject, attendees, and
  schedule-retention fields follow their optional wire semantics.
- OA calendar add/update requests now expose typed organizers, colors,
  descriptions, shares, and readonly settings; calendar details expose
  administrators, public ranges, and corporate-calendar metadata.
- OA schedule add/update/detail payloads now expose typed administrators,
  attendees, response status, reminders, repeat/custom-repeat rules, timezones,
  excluded occurrences, locations, calendar IDs, and sequence numbers.
- OA schedule supports typed pagination through
  `cgi-bin/oa/schedule/get_by_calendar`.
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
- account-service servicer add/delete requests now support both member and
  department assignments; operation/list records expose department and stop
  details, and servicer/event/message-failure/session-change codes provide
  forward-compatible semantic helpers.
- account-service synchronized messages now expose semantic message-origin and
  message-type helpers, servicer ids, menu/link metadata, and typed Channels
  shop product/order payloads; service-state requests/responses expose
  0-through-4 state helpers and typed transition message codes.
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
- group robot image and news message payloads now expose typed DTOs.
- Work app sends, group robots, and template-card updates now share typed card
  DTOs for source/action menus, titles, quote/emphasis/content areas, jumps,
  images, selection controls, buttons, checkboxes, submit controls, and select
  lists while preserving card extension fields.
- Work app, linked-corp, school, appchat, and group-robot messages now expose
  semantic message-type helpers for text, media, rich, mini-program, and
  template-card dispatch while retaining upstream `msgtype` strings.
- Work AppChat sends now expose typed text, image, voice, video, file,
  text-card, news, mpnews, and markdown payloads plus explicit confidential
  message selection instead of relying on flattened raw JSON.
- Work template-card sends, update requests, and group-robot payloads now
  expose semantic card-type helpers for text notice, news notice, and button
  / vote / multiple interaction cards while retaining upstream `card_type`
  strings; update button replacement is also an explicit DTO rather than raw
  JSON.
- Work upload-media responses now expose semantic media-type helpers for image,
  voice, video, and file while retaining the original upstream `type` string.
- Work temporary-media downloads now expose HTTP status, headers, content type,
  filename, length, byte-range metadata, and resumable range requests; binary
  download paths also surface WeChat JSON error payloads as API errors.
- Work invoice status requests and batch invoice responses now expose semantic
  reimbursement-status helpers for init, locked, and closure states while
  retaining the original upstream string values.
- Work remaining raw request payloads have been removed: moment-strategy
  create/edit, linked-corp and external-contact school message sends,
  template-card update, check-in option add/update, approval apply-event, and
  vacation quota update now accept explicit request DTOs.
- Work user batch job results now expose semantic async job-type helpers for
  contact import and export job families while retaining upstream `type`
  strings.
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
- external-contact moment task and customer records now expose semantic
  publish-status helpers for unpublished/published states while retaining
  upstream numeric status values.
- external-pay merchant and bill-list responses now expose typed use-scope and
  bill DTOs.
- external-pay merchant use-scope requests, scope responses, and bill records
  now expose semantic scope/status helpers while retaining upstream string
  fields.
- corp-group app-share responses now expose typed shared-corp records.
- OA check-in option, record, data, and schedule responses now expose typed
  group, user-option, record, data-item, and schedule DTOs.
- OA check-in rules now expose typed fixed/special workdays, time sections,
  Wi-Fi and location constraints, personnel ranges, reporters, shifts,
  correction reminders, and overtime policies. Employee rules use the actual
  `userid + group` response shape; day and month reports have separate typed
  DTOs; personal schedules and schedule mutations no longer use raw JSON.
- OA vacation configuration now exposes typed accrual, reset, quota, and
  expiration policies. Member quota responses use a dedicated balance DTO,
  and quota updates use the endpoint's direct single-balance request shape.
- OA check-in, approval, and vacation response DTOs now preserve unknown
  upstream fields across wrappers and nested records.
- OA calendar, dial, PSTNCC, and schedule responses now expose typed calendar,
  dial-record, call-state, and schedule DTOs.
- OA calendar, dial, PSTNCC, journal, and schedule response DTOs now preserve
  unknown upstream fields across wrappers and nested records.
- OA meeting-room and WeDrive responses now expose typed room, booking, space,
  file, and move-result DTOs.
- OA meeting, meeting-room, and WeDoc response DTOs now preserve unknown
  upstream fields across wrappers and nested room, booking, document, form,
  statistic, submitter, answer, and reply records.
- OA journal detail/stat and living info/watch-stat responses now expose typed
  journal, statistic, live-info, and watch-stat DTOs.
- OA living and WeDrive response DTOs now preserve unknown upstream fields
  across wrappers, info payloads, file records, and move failures.

Implemented on 2026-07-16 in Roze WeChat Open Platform authorizer depth:

- Authorizer permanent-material support now covers generic and video upload,
  news creation/update, typed and byte downloads, deletion, paginated listing,
  and material counts. Shared material DTOs retain unknown upstream fields.
- Component query-auth and authorizer aggregate responses now expose typed
  authorization scopes, mini-program network domains, categories, visit
  status, and extension fields throughout nested response structures.
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

- remaining payment notify/order/refund variants and deeper merchant-service
  response DTO normalization;

Implemented on 2026-07-18 in Roze WeChat payment download depth:

- fund-flow bill compatibility requests now send PowerWeChat's required
  `account_type` query key instead of the trade-bill-only `bill_type` key;
- added a dedicated `FundFlowBillRequest` and typed fund-flow query entry point
  while preserving the existing `BillRequest` entry point;
- profit-sharing bill responses now use the shared typed download URL/hash DTO
  and expose verified bytes, structured statement, and atomic file-download
  convenience methods;
- transfer batch receipts, transfer detail receipts, and FundApp electronic
  signatures now expose direct atomic file-download helpers;
- receipt downloads reject pending responses without a download URL before
  starting I/O, then reuse the existing SHA-1/SHA-256 streaming verification,
  no-clobber temporary file, flush/sync, and atomic commit path.

Implemented on 2026-07-18 in Roze WeChat merchant-service response depth:

- the shared crypto module now supports WeChat Pay's RSA-OAEP-SHA1 sensitive
  response-field decryption with PKCS#8 merchant private keys;
- complaint details can decrypt the encrypted `payer_phone` field directly
  while preserving the original ciphertext in the response DTO;
- complaint details expose terminal-aware refund, pending-reply, and priority
  attention helpers for production complaint routing;
- complaint list and negotiation-history responses expose checked `has_more`
  and `next_offset` pagination helpers without advancing empty pages;
- negotiation operation semantics now cover revoke/confirm, satisfaction,
  platform-help, service-order completion/cancellation, full/partial/received/
  entrusted refund system events, and the upstream legacy misspellings;
- complaint callback action parsing accepts both the documented
  `USER_APPLY_PLATFORM_SERVICE` value and the historical
  `USER_APPLY_PLATFORM_SERIVCE` value.

Implemented on 2026-07-16 in Roze WeChat payment download and complaint depth:

- signed WeChat Pay v3 bill download bytes helper with absolute/relative
  download URL parsing;
- streaming trade and fund-flow bill downloads now write to a unique sibling
  temporary file, calculate SHA-1/SHA-256 incrementally, require a matching
  expected digest, and publish with an atomic no-clobber hard link only after
  the file is flushed and synchronized;
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
- merchant-service complaint notification queries preserve unknown upstream
  fields and tolerate code/message error payloads.
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
- merchant-service reply, completion, refund-approval, and callback deletion
  now follow the upstream 204 No Content contract; completion sends the
  required complained merchant id, and replies support typed mini-program
  jumps.
- complaint details now expose payer OpenID, platform-service flags, agent mode,
  immediate-service priority, and shared-power same-machine return status.
- complaint negotiation history now exposes typed normal/click messages,
  content blocks, actions, FAQ/buttons, and semantic operation kinds; encrypted
  complaint callbacks decrypt directly into typed complaint/action resources.

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
4. Payment remaining notify/order/refund DTO normalization and helper variants.
5. Continue cross-family DTO hardening where endpoint coverage is already green.

Implemented on 2026-07-18 in Work OA WeDoc smart-sheet field depth:

- smart-sheet field add/update requests now use typed field mutation and
  property DTOs instead of `Vec<Value>`;
- add/update paths apply distinct local validation for document/sheet
  identity, required add metadata, update field identity, non-empty changes,
  select options, and auto-number/decimal property basics;
- all 24 currently documented smart-sheet field types expose semantic kind
  mapping and typed request constructors while unknown response types remain
  forward compatible;
- number, select, date/time, user, checkbox, attachment, URL, location,
  work-group, currency, auto-number, formula, reference, and related property
  keys share one extensible typed response/request model.

Implemented on 2026-07-18 in Work OA WeDoc smart-sheet record depth:

- record add/update requests now use typed record mutations and field-value
  maps instead of `Vec<Value>`, with builders for text, number, checkbox,
  date/time, users, URL, select options, images, attachments, and locations;
- record query sort and filter specifications now use typed DTOs, including
  string, number, boolean, user, and date/time filter value shapes;
- add/update/delete enforce the documented 500-record operational boundary,
  update requires record ids, and all record operations validate document,
  sheet, record, and cell identities before transport;
- record queries enforce the 1000-row page limit and reject the unsupported
  sort-plus-filter combination; response values retain heterogeneous,
  forward-compatible JSON cells behind a typed field map.

Implemented on 2026-07-18 in Work OA WeDoc smart-sheet view depth:

- view add/update/query responses now use typed view kinds, date ranges, and
  properties instead of raw JSON;
- grid, Kanban, gallery, Gantt, and calendar constructors emit exact upstream
  view codes, while Gantt/calendar date-range requirements are checked before
  transport;
- view properties type automatic sorting, sort/group specifications, filters,
  field statistics, field visibility, and frozen-field count while preserving
  unknown upstream extensions;
- add/update/get/delete paths validate document, sheet, view identity, update
  changes, pagination, date-property compatibility, and nested property
  contracts locally.

Implemented on 2026-07-18 in Work OA WeDoc smart-sheet lifecycle depth:

- smart-sheet add/update requests now use typed sub-sheet properties instead
  of raw JSON, separating smart-sheet `title/index` from ordinary spreadsheet
  row/column properties;
- title-only, index-only, title-at-index, and rename constructors emit only
  fields accepted by the corresponding upstream endpoint;
- add/get/update/delete paths validate document and sheet identity, nonnegative
  insertion index, rename title, and the update endpoint's title-only contract;
- sub-sheet responses now expose typed visibility and semantic smart-sheet,
  dashboard, external, or unknown sheet kinds while preserving extensions.

Implemented on 2026-07-18 in Work OA WeDoc smart-sheet field-group and privilege depth:

- field-group add/update/list/delete requests now validate document and sheet
  scope, names, identifiers, pagination, uniqueness, and the documented
  150-field/150-group ceilings before network I/O;
- field-group updates distinguish an omitted child patch from an explicit empty
  list, so renaming cannot accidentally clear fields and callers can
  intentionally clear a group;
- content-permission queries expose semantic all-member/additional rule types,
  validate their rule-id requirements, and retain unknown response rule types;
- mixed string/integer response rule identifiers and field-type semantics are
  typed while upstream extension fields remain forward compatible.

Implemented on 2026-07-18 in Work OA WeDoc smart-sheet authorization depth:

- document/sub-sheet authorization reads and modifications now validate scope
  and required modification payloads before network I/O;
- authorization payloads use an object-or-list structural type instead of
  unconstrained JSON, rejecting invalid scalar payloads while preserving
  undocumented and future policy keys;
- document and sub-sheet constructors make omission of `sheet_id` explicit, and
  response fallback across `auth_info`, `field_auth`, and `record_auth` remains
  available through a typed helper.

Implemented on 2026-07-18 in Work external-contact contact-way depth:

- contact-way type and scene now use semantic numeric enums for single/multi
  recipients and mini-program/QR-code scenes, retaining unknown response codes;
- add requests now expose the previously missing `is_temp` and current
  `is_exclusive` fields, with `is_exclusive` also represented in detail/update
  contracts;
- add/list/update/get/delete paths validate identifiers, time ranges, page
  sizes, recipient cardinality, unique users/departments, scene-specific styles,
  temporary-session dependencies, positive durations, and non-empty patches;
- conclusion DTOs validate text/link/mini-program requirements and support both
  `media_id` and `pic_url` image forms used by upstream contracts.

Implemented on 2026-07-18 in Work external-contact customer-operation depth:

- customer list, detail, external-user conversion, temporary-chat close, batch
  detail, served-customer list, remark, and tag-mark paths now validate member
  and external-user identities before network I/O;
- batch detail enforces 1 to 100 unique member users and a 100-row page limit,
  while served-customer pagination accepts the upstream 1000-row boundary and
  rejects empty cursors or invalid limits;
- remark updates require an actual patch, enforce the documented 20-character
  remark/company and 150-character description limits, preserve empty strings
  for explicit clearing, and reject empty media ids or duplicate mobiles;
- tag marking requires a non-empty mutation with unique ids and rejects adding
  and removing the same tag, while follow-tag response types expose corporate,
  personal, rule-group, and forward-compatible unknown semantics.

Implemented on 2026-07-18 in Work media upload/download depth:

- temporary-media upload now exposes PowerWeChat-compatible image, voice,
  video, and file helpers, and attachment upload exposes dedicated image and
  video helpers while retaining the generic byte-upload methods;
- image upload and temporary-media upload now enforce the upstream byte
  boundaries before network I/O: image upload accepts 5 bytes through 2 MiB,
  while temporary image/voice/video/file uploads require more than 5 bytes and
  cap payloads at 2/2/10/20 MiB respectively; upload filenames also reject
  control characters, and multipart parts carry explicit JPG/PNG, AMR, MP4, or
  binary content types after validating the media filename format;
- media download, JSSDK download, image upload, temporary upload, attachment
  upload, URL-job lookup, and group-robot file upload reject blank identifiers,
  names, keys, attachment types, and empty byte payloads before network I/O;
- temporary upload accepts only image/voice/video/file and attachment upload
  accepts only image/video, with semantic media kinds providing stable wire
  codes;
- asynchronous URL upload validates its supported scene/type, absolute HTTP(S)
  source URL, filename, and 32-character hexadecimal MD5 before submission.
- media downloads now decode RFC 5987 UTF-8 filenames, reject malformed
  content ranges semantically, and expose expected body length, complete-body,
  successful-response, consistent-range, and resumable-download helpers;
- asynchronous URL-upload results classify invalid URL, download failure,
  oversized-file, MD5-mismatch, and forward-compatible unknown detail errors,
  with terminal and retryable-failure helpers for polling workers.

Implemented on 2026-07-18 in Work application-message validation depth:

- generic and convenience application-message sends now validate positive
  agent ids, non-empty pipe-delimited recipients, 0/1 switches, and the
  documented 1-to-10000-second duplicate-check interval before network I/O;
- generic sends require exactly one typed payload whose field matches
  `msgtype`; text/media/card/mini-program payloads enforce required fields and
  news/mpnews enforce the eight-article boundary;
- message recall rejects blank message ids, task-card updates enforce 1 to 1000
  unique users plus task/button identity, and template-card updates validate
  response codes, recipient uniqueness, at-all semantics, and the
  1000-user/100-department/100-tag limits;
- template-card updates require at least one button or replacement-card
  payload and validate both when supplied, preserving PowerWeChat-compatible
  update combinations.

Implemented on 2026-07-18 in Work user mutation and batch-job depth:

- member create/update/get/delete paths validate user identity locally; create
  requires name and department while update requires an actual patch;
- member mutation validation enforces the 64-character identity/name limits,
  up to 20 unique positive departments, aligned order/leader arrays, main
  department membership, gender semantics, and 0/1 enable flags;
- batch deletion enforces 1 to 200 unique users, invitation enforces a
  non-empty target with the 1000-user/100-department/100-tag limits, and member
  ID pagination enforces the 10000-row maximum;
- import/replace tasks validate media and optional callback credentials,
  callback URLs must be absolute HTTP(S), export jobs validate positive block
  sizes and 43-character base64 AES keys, and result lookups reject blank or
  over-64-character job ids.

Implemented on 2026-07-18 in Work OA calendar and schedule depth:

- calendar add/update/get/delete paths validate positive agent ids, organizer,
  identity, summary, `#RRGGBB` color, unique share users, readonly semantics,
  and 1-to-1000 unique lookup ids before network I/O;
- schedule add/update/get/delete paths validate identity, positive ordered time
  ranges, unique admins and attendees, attendee response states, optional
  calendar/organizer identity, and 1-to-1000 unique lookup ids;
- reminder DTOs validate 0/1 switches, unique relative offsets, non-negative
  before-event seconds, positive repeat intervals, future repeat termination,
  valid unique week/month days, and positive exclusion timestamps;
- calendar schedule queries expose a first-page constructor and enforce
  non-negative offsets plus the 1000-row page boundary.

Implemented on 2026-07-18 in Open Platform authorizer mini-program domain,
tester, and privacy depth:

- server and web-view domain operations use typed get/set/add/delete actions,
  validate action payload semantics, reject duplicate or blank entries, and
  enforce HTTPS, WSS, UDP, and TCP schemes before network I/O;
- tester binding rejects blank WeChat ids, while unbinding provides explicit
  WeChat-id/userstr constructors and requires exactly one non-empty identity;
- privacy owner settings now include extension-media and storage-expiration
  fields, and privacy requests/responses cover SDK privacy information,
  privacy lists, code state, update time, and structured privacy descriptions;
- privacy setting, extension-file upload, and privacy-interface applications
  validate contacts, keys, descriptions, SDK uniqueness, versions, byte
  payloads, scenes, and absolute HTTP(S) evidence URLs before submission;
- domain, tester, privacy-setting, and extension-file responses retain unknown
  fields so newly added WeChat response metadata remains forward compatible.

Implemented on 2026-07-18 in Open Platform authorizer mini-program code,
audit, and release depth:

- code-commit requests expose a constructor and reject blank template/version/
  description values, malformed `ext_json`, and non-object extension payloads
  before uploading an experience build;
- audit requests expose a first-class constructor and validate non-empty unique
  pages, category names and positive ids, paired third-level categories,
  optional feedback/order fields, and unique non-empty preview media ids;
- UGC declarations now match PowerWeChat's array-shaped `scene` contract,
  validate positive unique scene/method codes, constrain the audit-team flag to
  0/1, and require its description when an audit team is declared;
- audit lookup and acceleration require positive audit ids, visit status uses
  typed open/close actions, gray release enforces a 1-to-100 percentage, and
  support-version updates reject blank versions before network I/O;
- rollback versions type `commit_time` and `app_version`, support-version usage
  items type `percentage`, and existing unknown-field preservation remains
  intact across release and audit responses.

Implemented on 2026-07-18 in Open Platform authorizer and Official Account
permanent-material depth:

- shared upload kinds restrict generic permanent uploads to image, voice, and
  thumbnail media, with PowerWeChat-style convenience methods for each kind;
  video uploads retain their required title/introduction metadata;
- authorizer material now includes the previously missing article-image upload,
  while Official Account exposes a binary download path for image/voice assets
  that cannot be decoded through the JSON material response;
- all upload paths reject blank filenames and empty bytes, material ids are
  validated before get/delete/update calls, and article updates enforce the
  zero-to-seven index range;
- news creation enforces one to eight valid articles, including required
  title/thumbnail/content, 0/1 cover/comment flags, comment dependencies, and
  absolute HTTP(S) source URLs when supplied;
- material listing exposes typed image/voice/video/news kinds and validates
  non-negative offsets plus the one-to-twenty page boundary;
- batch material items provide typed news-content decoding with timestamps and
  unknown-field retention, avoiding direct `Value` traversal for normal news
  list workflows.

Implemented on 2026-07-18 in Official Account draft, publish, and menu depth:

- draft add/update and draft/published list requests expose constructors and
  validate one-to-eight articles, required content/media fields, comment flag
  dependencies, source URLs, zero-to-seven indices, non-negative offsets,
  one-to-twenty page sizes, and 0/1 no-content flags;
- draft get/delete, publish submit, published article get/delete, and publish
  status lookup reject blank identifiers or a zero publish id before network
  I/O;
- publish status helpers now treat user deletion and system banning as terminal
  failures, expose terminal-state semantics, and normalize string-or-array
  `article_id` values;
- menu creation validates one to three top-level buttons, at most five
  sub-buttons, one nesting level, supported action types, action-specific
  key/URL/media/article/mini-program payloads, conflicting fields, and absolute
  HTTP(S) fallback URLs;
- typed menu constructors cover leaf, parent, and mini-program buttons, while
  conditional menus require a non-empty match rule and validate sex and client
  platform codes;
- menu-get, conditional-menu, current-self-menu, and try-match responses use
  response-specific DTOs for numeric match fields, object-shaped sub-button
  lists, news metadata, flexible menu ids, and forward-compatible unknown
  fields; switch/menu-open helpers expose their 0/1 state as booleans.

Implemented on 2026-07-18 in Official Account user, tag, and customer-service
depth:

- single and batch user lookups validate non-empty openids and supported
  `zh_CN`/`zh_TW`/`en` languages, while batch requests enforce one-to-one
  hundred unique users before network I/O;
- openid migration enforces the one-to-one hundred boundary, user remarks and
  tag names enforce their thirty-character boundary, and tag identifiers must
  be positive integers;
- tag assignment/removal rejects blank or duplicate openids and enforces the
  fifty-user boundary, while blacklist add/remove enforces its twenty-user
  boundary; optional pagination cursors may be empty but not whitespace-only;
- customer-service account, worker, session, recipient, avatar filename, and
  avatar bytes are validated before account or session operations;
- customer-service transcript requests validate positive ordered timestamps,
  non-negative cursors, and one-to-ten-thousand page sizes;
- customer-service messages validate supported message types, exactly one
  matching non-empty payload, type-specific required fields, and optional
  sender accounts; text, media, and sender-account constructors provide common
  production request paths without hand-building JSON.

Implemented on 2026-07-18 in Work external-contact corporate and strategy tag
depth:

- corporate tag list/add/edit/delete and strategy tag list/add/edit/delete now
  validate their request DTOs before network I/O while retaining the existing
  PowerWeChat-compatible wire contracts;
- list requests permit empty filters for a full tag library but reject blank or
  duplicate tag and group ids, while delete requests require at least one
  selected tag or group;
- add requests distinguish new tag groups from additions to existing groups,
  validate one to thirty unique named tags, enforce the thirty-character name
  boundary, and reject negative group or tag ordering values;
- edit requests validate tag-or-group identity, names and ordering, optional
  corporate application ids must be positive, and strategy operations require
  positive strategy ids;
- new-group, existing-group, tag-item and full-list constructors provide valid
  defaults for common corporate and strategy tag workflows without ad hoc JSON.

Implemented on 2026-07-18 in Work external-contact group-chat and identifier
migration depth:

- external-userid migration now matches the upstream batch contract:
  ordinary customer migration sends `external_userid_list`, group-member
  migration sends `chatid` plus `external_userid_list`, and both parse typed
  `items` containing old and new external-user ids instead of a singular,
  incompatible response shape;
- migration constructors enforce one to one thousand unique non-empty ids and
  group migration additionally validates chat identity before network I/O;
- group-chat listing validates status filters, non-empty pagination cursors,
  the one-to-one-thousand page boundary, and one-to-one-hundred unique owner
  filters;
- group details validate chat identity and the 0/1 name flag, openGID conversion
  validates identity, and resigned/on-job transfer paths enforce one to one
  hundred unique chats plus a non-empty new owner;
- join-way requests expose existing-chat and automatic-room constructors,
  validate scene and 0/1 auto-create modes, remarks, state, room metadata, and
  at most five unique initial chats; get/update/delete validate configuration
  identity;
- response helpers type join-way and member join scenes, expose automatic-room
  state, preserve unknown fields, and distinguish fully successful transfers
  from responses containing failed chats.

Implemented on 2026-07-18 in Work external-contact strategy and customer
transfer depth:

- moment strategy coverage now includes the missing detail endpoint, typed
  view/send/profile privileges, typed user/department ranges, and corrected
  array-shaped range responses;
- moment and customer strategy list/range requests validate positive strategy
  ids, parent ids, non-blank cursors and names, the one-to-one-thousand page
  boundary, unique administrators, valid ranges, and disjoint range patches;
- strategy create/edit/delete paths validate before network I/O, while typed
  user and department constructors avoid ambiguous range payloads;
- customer strategy list and range responses expose `next_cursor` through the
  shared cursor-page helpers, preserving unknown response fields;
- on-job and resigned-customer transfer paths validate distinct handover and
  takeover users, one-to-one-hundred unique customers, optional message length,
  unassigned-customer pagination, and transfer-result cursors.

Implemented on 2026-07-18 in Work external-contact moment and statistics
depth:

- moment listing validates positive ascending timestamps, the official
  one-month query window, 0/1/2 enterprise/member/all filters, non-blank
  creators and cursors, and the one-to-one-hundred page boundary;
- moment task, selected-customer, and delivered-customer pagination use typed
  request DTOs with their respective one-thousand, one-thousand, and
  five-thousand limits, while comment, cancellation, and asynchronous-result
  lookups validate identifiers before network I/O;
- moment summaries type the official image, video, link, and location response
  shapes and expose enterprise/member creation plus partial/public visibility
  semantics instead of leaving these fields in generic JSON;
- task creation validates text-or-attachment content, image/video/link payload
  consistency, media identifiers, and user/department/tag visible ranges;
  visible-range builders now include the previously missing department path;
- asynchronous task results restore nested invalid user/department and tag
  records, retain result-level errors, and expose shared async-job completion
  and success helpers;
- contact behavior statistics enforce a non-empty unique user/department
  selection, the one-hundred target boundary, positive departments, and the
  official thirty-day window;
- group-chat statistics validate owner filters, time order, sort fields,
  direction, offsets, and the one-to-one-thousand page boundary; daily records
  expose `stat_time`, migration counts are typed, and offset responses expose
  a pagination helper.

Implemented on 2026-07-18 in Work OA meeting and meeting-room depth:

- meeting create/update requests now model the current `admin_userid`,
  `location`, `settings`, `cal_id`, and `reminders` contract; attendee lists
  serialize with the official `userid` key while accepting the historical
  `userids` key during deserialization;
- meeting settings type waiting-room, host-entry, mute, external-user,
  watermark, host, ring-user, and password controls, while reminder DTOs cover
  repeat cadence and advance notifications;
- create, update, cancel, list, and detail paths validate identifiers,
  timestamps, durations, agent ids, unique attendees, binary switches,
  passwords, repeat consistency, 30-day query windows, and 1-to-100 page
  limits before network I/O;
- create/update responses retain excess attendees and expose partial-success
  helpers; create meeting ids now use the current string representation, and
  meeting details type current administrator, location, agent, settings,
  calendar, and reminder fields while preserving older response fields;
- meeting-room add/edit/list/query/delete/book/link/cancel/detail paths validate
  positive room ids and capacities, the 30-character room-name boundary,
  all-or-none city/building/floor locations, unique equipment, finite bounded
  coordinates, ordered time ranges, unique attendees, non-empty linkage ids,
  and 30-minute booking boundaries;
- linked meeting-room booking responses expose conflict detection while all
  response DTOs continue preserving unknown upstream fields.

## Documentation Update Needed

Keep `docs/powerwechat-gap-analysis.md` as the submodule-level view, but do not
use it as the final production parity signal. This method-depth audit should be
updated whenever a family reaches one-to-one endpoint coverage or PowerWeChat
adds new methods.
