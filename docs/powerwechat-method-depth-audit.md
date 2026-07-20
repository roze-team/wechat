# PowerWeChat Method-Depth Audit

Audit date: 2026-07-20.

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
| Mini Program | 214 | 227 | low |
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
   deepened, including response-boundary contract validation. Continue expanding
   `notify`, `order`, and remaining payment method variants with strong typed
   request/response DTOs plus signature/decryption tests where applicable.

3. Open Platform authorizer depth:
   exact endpoint coverage is now green against the current PowerWeChat scan.
   Release/audit, privacy, domain, tester, and permanent-material workflows
   now have production response guards. Continue aggregate account and
   authorizer Official Account workflow depth.

4. Mini Program depth:
   exact endpoint coverage is now green after filtering scanner-only
   documentation paths. `liveBroadcast` now has one-to-one PowerWeChat method
   coverage and production request/response guards. `express` retains all
   16 PowerWeChat methods and adds typed production variants for the four
   upstream open-map operations. The current PowerWeChat `b2b`, `dataCube`,
   `industry/miniDrama/vod`, and `wxa/sec/order` surfaces now have one-to-one
   method coverage, production request/response guards, semantic models, and
   pagination or time-range helpers where applicable.

5. Official Account depth:
   exact endpoint coverage is now green against the current PowerWeChat scan.
   Continue DTO normalization for `broadcasting`, `material`, `menu`,
   `templateMessage`, and `publish`.

6. Open Work depth:
   exact endpoint coverage is now green against the current PowerWeChat scan.
   Continue DTO normalization for `license`, `suitAuth`, `server`, and
   component/base authorization helpers.

Implemented on 2026-07-21 in Work group-robot message depth:

- The group-robot send path now validates webhook keys, request payloads, and
  WeCom API status responses instead of returning unchecked success values.
- Text, Markdown, Markdown V2, image, news, file, voice, and template-card
  messages enforce a single type-matched payload with production size,
  identifier, article, URL, base64, and MD5 constraints.
- Typed constructors now cover mobile mentions, files, news, template cards,
  and byte-backed images; image construction uses the shared base64 and MD5
  libraries so callers cannot accidentally submit a mismatched digest.

Implemented on 2026-07-21 in Work identity-conversion response depth:

- All five `id_convert` network paths now validate WeCom API status and typed
  response contracts before returning data to callers.
- External-user/pending, user/open-user, open-user/user, and external-tag/open-
  tag batches reject missing identities, duplicate sources or targets,
  success/failure overlap, request-external mappings, and silently omitted
  inputs.
- Conversion responses expose successful counts, source lookup, invalid lists,
  and missing-input helpers for production reconciliation and retry handling.

Implemented on 2026-07-21 in Work corpgroup and mini-program session depth:

- Corpgroup application sharing, subordinate-corporation tokens, transferred
  sessions, and Work mini-program code-to-session now validate inputs, WeCom
  API status, and required response identities before returning.
- The Work mini-program login request now sends PowerWeChat's documented
  `grant_type=authorization_code` query parameter in addition to `js_code`.
- Shared installations reject duplicate `(corpid, agentid)` pairs; subordinate
  tokens expose checked expiry/refresh helpers, and both session responses
  expose validated identity tuples for authentication boundaries.

Implemented on 2026-07-21 in Work department contract depth:

- Department create, update, delete, full list, simple list, and detail paths
  now validate requests and WeCom responses before returning data.
- Department detail now follows PowerWeChat's actual nested
  `{ department: {...} }` response instead of incorrectly treating department
  fields as top-level values; malformed legacy-flat responses are rejected.
- Create/update semantics now support safe partial updates, omit unset name and
  parent fields, and reject empty mutations, self-parenting, invalid hierarchy
  identifiers, oversized names, and invalid ordering values.
- Full/simple department responses enforce unique IDs and valid hierarchy
  records, with root, child, lookup, leader, and required-detail helpers.

Implemented on 2026-07-21 in Work JSSDK credential depth:

- Corporation and agent JSSDK ticket network exits now validate WeCom API
  status, required ticket values, and positive expiration before returning.
- Ticket DTOs expose checked ticket, expiry, and safety-margin refresh helpers
  matching PowerWeChat's early-refresh cache behavior.
- JSSDK config construction is now fallible and validates corporation IDs,
  tickets, nonces, timestamps, absolute HTTP(S) URLs, and unique API names.
- Signature input strips the browser-only URL fragment while preserving the
  exact pre-fragment URL bytes; deterministic construction and signature
  verification helpers are covered by a fixed WeChat-format SHA-1 vector.

Implemented on 2026-07-21 in Work external-contact profile depth:

- External-contact detail and batch-query exits now validate nested external
  profile attributes before returning customer data to application code.
- Known text, web, and mini-program attribute types require exactly one
  matching payload with their required fields; type/payload mismatches,
  multiple payloads, blank values, and duplicate attribute names are rejected.
- Profile web links require absolute HTTP(S) URLs without credentials or URL
  fragments, while unknown future attribute types remain forward-compatible
  through flattened extension fields.
- Profile lookup and per-kind count helpers expose the typed attribute data
  without forcing callers back to generic JSON traversal.

Implemented on 2026-07-21 in Work media integrity depth:

- URL-upload requests can now compute the required MD5 digest directly from
  local content and verify that content again before submission, using the
  project's shared MD5 dependency instead of caller-specific implementations.
- Remote upload URLs reject embedded credentials and fragments in addition to
  requiring absolute HTTP(S), preventing secrets or browser-only URL state
  from entering WeCom's server-side downloader.
- Download responses expose canonical MD5 calculation and checked digest
  verification helpers; malformed expected digests and content mismatches are
  reported before application code persists the media.
- Binary download validation now rejects JSON content types, including
  structured `+json` variants, so a successful JSON envelope cannot be
  mistaken for media bytes.

Implemented on 2026-07-21 in Work application-message response depth:

- Generic and convenience template-card sends now preserve the card type
  through response validation; button, vote, and multiple-choice interaction
  cards require the one-time `response_code` needed by the update endpoint.
- Checked response-code accessors distinguish an update-capable interactive
  send from ordinary cards without ad hoc optional-string handling.
- Message IDs and template-card response codes require printable values within
  a 512-byte boundary, rejecting blank, control-character, and oversized
  upstream identifiers.
- Pipe-delimited invalid user, unlicensed user, department, and tag lists now
  enforce the same 1000/100/100 response boundaries as their request scopes;
  task-card update failures enforce the 1000-user limit as well.

Implemented on 2026-07-21 in Work user batch-job result depth:

- Known contact-import jobs now require total and percentage lifecycle fields;
  finished jobs require 100 percent progress and result rows cannot exceed the
  declared total.
- Sync, replace-user, and invite results require a user identity, while
  replace-department results require a positive department identity; mixed or
  duplicate identities are rejected before returning polling results.
- Known result items require an explicit error code, and failed rows require a
  non-empty error message; success/failure counts plus user and department
  lookup helpers support reconciliation workers without raw scans.
- Unknown future job types remain forward-compatible, while batch callback
  URLs now reject credentials and fragments in addition to requiring absolute
  HTTP(S).

Implemented on 2026-07-21 in Work media path-upload parity:

- Work image, temporary image/voice/video/file, and external-contact
  attachment uploads now expose async path-based entry points alongside the
  existing byte APIs, matching PowerWeChat's production file workflow without
  requiring callers to read files themselves.
- Path uploads validate the media type before I/O, require a regular file and
  a UTF-8 file name, and reject files above the endpoint-specific limit from
  metadata before allocating the upload body.
- The shared reader rechecks the actual bytes after the asynchronous read so a
  file that grows between metadata inspection and reading cannot bypass the
  image, voice, video, or generic-file size ceiling.

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

Implemented on 2026-07-20 in Mini Program liveBroadcast lifecycle depth:

- all 37 methods in the current PowerWeChat `liveBroadcast` client now have
  dedicated Rust wrappers, including room editing, push/share URLs,
  assistants, sub-anchors, room feature switches, room-goods operations,
  warehouse add/update/delete/audit, approved-goods listing, roles, followers,
  and event push;
- create/edit room requests use distinct wire contracts and validate required
  media/member fields, ordered positive schedules, room modes, and all 0/1
  feature flags before network I/O;
- goods, assistant, follower, role, sort, and message batches enforce positive
  IDs, nonblank identities, bounded collection sizes, and uniqueness; goods
  creation/update distinguish required create fields from nonempty patches and
  reject invalid/non-finite prices;
- live-info, replay, goods, and follower pagination expose checked next-page
  helpers that detect negative offsets, integer overflow, and stalled
  non-terminal pages;
- room, goods warehouse, assistant, role, follower, shared-code, push-address,
  video, create, and audit responses validate API errors and required
  identities, reject inconsistent totals/duplicates, and retain extension
  fields for forward compatibility;
- room status helpers distinguish terminal and operator-attention states,
  while goods audit helpers expose approved inventory without treating future
  numeric values as approved.

Implemented on 2026-07-20 in Mini Program express production depth:

- all 16 current PowerWeChat express methods remain covered, while typed
  add-order, preview-template, provider-business-audit, and provider-path
  wrappers replace open JSON maps for production callers without removing the
  compatibility entry points;
- typed add-order contracts follow the current WeChat schema for source modes,
  App/H5 identity, sender/receiver contacts, shop single/multi-goods details,
  cargo dimensions/details, insurance, service, pickup expectations, tags,
  and settlement modes;
- every dedicated order/account/printer/path request now validates required
  identities, bind/unbind and print modes, positive event fields, nonempty
  object compatibility payloads, and unique batches of at most 100 orders
  before network I/O;
- order creation and cancellation distinguish WeChat API errors from carrier
  result errors, require returned order/waybill identities, and validate
  unique typed waybill data;
- batch orders, accounts, carriers, order details, paths, printers, quotas,
  contacts, and preview responses validate decoded counts, uniqueness,
  non-negative values, ordered tracking events, required contacts, and
  rendered artifacts while preserving extension fields;
- printer counts, quotas, and contact waybill ids accept the string-or-number
  wire variants used by current PowerWeChat/WeChat responses.

Implemented on 2026-07-20 in Mini Program immediateDelivery production depth:

- all 14 valid current PowerWeChat immediate-delivery business operations
  remain covered, while nine typed production variants cover add/pre-add,
  cancel/pre-cancel, add-tip, abnormal-return confirmation, reorder, and
  sandbox/real-environment status simulation without removing open-map
  compatibility entry points;
- the common crypto module now exposes ordered SHA-1 hashing, and signed order
  identities derive `delivery_sign` from `shopid + shop_order_id + appSecret`
  without retaining or serializing the carrier secret;
- typed order contracts cover carrier/shop/openid identity, sender and receiver
  coordinates, nested cargo/goods detail, scheduling, insurance, cash
  collection, pickup/delivery codes, logistics notification shop metadata,
  cancellation, tips, status simulation, and delivery tokens;
- request validation rejects blank and oversized identities, malformed SHA-1
  signatures, invalid coordinates, non-cent monetary values, impossible cargo
  dimensions, empty goods, inconsistent scheduled/insured orders, invalid
  flags and statuses, non-HTTPS images, and empty compatibility payloads before
  network I/O;
- responses distinguish WeChat `errcode` failures from carrier `resultcode`
  failures, accept documented string-or-number drift for identifiers and
  status fields, preserve extension fields, expose provider/shop lookup and
  order-state helpers, and reconcile actual delivery fees against delivery
  fees minus coupons.

Implemented on 2026-07-20 in Mini Program operation production depth:

- all 11 current PowerWeChat operation methods remain covered, while typed
  domain, JS-error list/detail/legacy-search, performance, and real-time-log
  variants lift production callers off open JSON without removing compatibility
  entry points;
- typed requests enforce documented camelCase/snake_case wire fields, real
  calendar dates, ordered positive timestamps, MD5 identities, query types,
  sort modes, pagination limits, performance dimensions, supported network
  types, same-day real-time-log windows in China Standard Time, and log levels
  before network I/O;
- the typed real-time-log path uses the official GET query contract, including
  `traceId` and `filterMsg`, while the PowerWeChat-compatible open-map POST path
  remains available;
- JS-error detail/list responses now model real `errorMsg`, `errorStack`,
  `errorMsgMd5`, `errorStackMd5`, `TimeStamp`, version, device, count, UV/PV,
  route, plugin, and identity fields with pagination and consistency checks;
- real-time logs now decode the official nested `data.list` shape, numeric
  aggregate/message levels, arbitrary structured log arguments, platform,
  versions, route, trace and filter metadata, while retaining historical
  top-level-list compatibility;
- domain, feedback, gray-release, performance, scene, version, and log
  responses preserve extension fields and expose API-error, duplicate,
  percentage, schedule, JSON payload, semantic status/version, and next-page
  validation helpers.

Implemented on 2026-07-20 in Mini Program `wxa/sec/order` production depth:

- all 9 current PowerWeChat security-order methods remain covered, with
  request validation applied before upload, combined upload, order query,
  list query, receipt confirmation, message-path, trade-management, and
  special-order network calls;
- order keys now enforce the documented merchant-order versus WeChat
  transaction identifier modes, while shipment validation covers logistics
  and delivery modes, split-delivery completion, express identifiers,
  non-empty item descriptions, RFC 3339 upload times, payer identity, and
  duplicate combined sub-orders;
- order/list requests enforce complete lookup identities, positive ordered
  payment ranges, order states, page sizes up to 50, nonblank cursors, and
  positive receipt/special-order timestamps;
- order responses now cover PowerWeChat's flat paid amount, trade creation
  time, complaint state, shipping-completion counters, goods descriptions,
  and per-item Unix upload time while preserving the existing amount and
  extension-field compatibility surface;
- response guards reject WeChat API failures, missing or duplicate order
  identities, invalid states, negative or inconsistent amounts/timestamps,
  malformed shipping data, missing trade-management results, and `has_more`
  pages without a cursor; a typed next-page helper preserves the current
  filters.

Implemented on 2026-07-20 in Mini Program `industry/miniDrama/vod` production
depth:

- all 27 current PowerWeChat VOD, drama-audit, CDN, package, authorization,
  account-authorization, and flush-drama workflows remain covered, and every
  request-bearing path now validates before network or multipart I/O;
- URL and file uploads validate HTTPS sources, supported media/cover formats,
  non-empty bounded payloads, media names, and source context; multipart
  uploads omit absent cover parts and propagate source context;
- chunked uploads enforce resource types, 1-100 part numbers, 5 MiB chunks,
  non-empty etags, unique contiguous completion manifests, and distinct media
  replacements;
- media queries, playback links, drama submissions and updates validate
  positive identities, ordered time windows, pagination, the two-hour
  playback expiry, referrer-domain lists, episode-count consistency,
  conditional qualification evidence/costs, actor counts and metadata, audit
  modes, CDN intervals, authorization expiry, and flush-list uniqueness;
- responses now model PowerWeChat task type/error/finish data, media expiry,
  file size and original/MP4/HLS URLs, audit evidence, full drama metadata and
  episode references, CDN log dates, and the actual traffic-package schema;
  `package_id` is string-typed with numeric drift compatibility, and per-drama
  authorization results decode the upstream `errcode` field;
- response guards reject outer and per-item API failures, invalid task/audit
  states, missing completed-task media, duplicate media/dramas/packages or
  authorization objects, malformed URLs, impossible timestamps/counts,
  out-of-order CDN usage, and over-consumed traffic packages; semantic status,
  package remaining, and next-page helpers are available.

Implemented on 2026-07-20 in Mini Program `b2b` production depth:

- all 14 current PowerWeChat payment-construction, order/refund,
  profit-sharing account/order/refund, remaining-amount, finish, and bill
  workflows remain covered; all 13 network request types now validate before
  signing or HTTP I/O;
- payment construction rejects empty session/app keys and non-object payloads;
  query/refund identities enforce the documented exclusive alternatives,
  amounts must be positive, environment flags are bounded, account pagination
  is validated, and bill dates use a valid `YYYYMMDD` date;
- refund source and reason now serialize as the numeric PowerWeChat wire enums
  instead of the previous incompatible string source, while profit-sharing
  order status and account timestamps decode as their upstream numeric types;
- payment response metadata covers `return_code`, `result_code`, payment error,
  and newer `code`/`message` shapes, preserves unknown fields, and is checked
  before a typed network response is returned;
- order/refund/payment semantic status helpers, pagination helpers, required
  identity checks, amount/currency consistency, duplicate-account detection,
  non-negative balances/timestamps, HTTPS bill URLs, and ended-day balance
  reconciliation provide production response guards.

Implemented on 2026-07-20 in Mini Program `dataCube` production depth:

- all 11 current PowerWeChat daily-summary, performance, daily/weekly/monthly
  visit-trend, visit-distribution, daily/weekly/monthly retention, visit-page,
  and user-portrait methods now have one-to-one Rust wrappers; the previously
  missing summary, distribution, weekly-retention, and monthly-retention
  endpoints are present;
- every request validates before HTTP I/O; date ranges require valid ordered
  `YYYYMMDD` values and expose an inclusive-day helper;
- performance requests now use PowerWeChat's current nested
  `time/module/params` wire contract, with positive ordered timestamps,
  required module/parameter values, and duplicate-field rejection, replacing
  the incompatible legacy flat fields;
- typed visit-distribution dimensions/items and user-portrait
  dimensions/items replace unstructured payloads while preserving extension
  fields; retention and page responses now retain their upstream `ref_date`;
- response guards cover API failures, supported daily/monthly/range reference
  periods, non-negative finite metrics, duplicate periods/pages/distribution
  indexes/items/portrait items, retention keys, required identities, and
  object-shaped performance data.

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

- remaining payment notify/order/refund variants;

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
3. Payment remaining notify/order/refund DTO normalization and helper variants.
4. Continue cross-family DTO hardening where endpoint coverage is already green.

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

Implemented on 2026-07-20 in Work external-contact contact-way response depth:

- add/get/list network methods now validate their typed response contracts
  before returning data to application code, including non-zero API error
  propagation for manually deserialized responses;
- add responses require a bounded configuration id and absolute HTTP(S)
  QR-code URL, while get responses require a structurally valid contact-way
  detail;
- contact-way details validate required identity/type/scene, style, text,
  unique users and departments, positive temporary durations, QR-code URLs,
  and nested conclusion contracts while retaining unknown enum values and
  extension fields;
- list responses reject missing or duplicate configuration ids and expose
  normalized terminal-cursor, `has_more`, and configuration lookup helpers;
- conclusion image, link image, and destination URLs now require absolute
  HTTP(S) URLs without credentials or fragments.

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

- synchronous image, temporary-media, attachment, and group-robot upload
  boundaries now reject upstream API errors and structurally incomplete
  success responses before returning them to callers;
- upload responses require non-empty media identifiers, matching known media
  types, positive creation timestamps, and absolute HTTP(S) URLs when present;
  creation timestamps accept both string and numeric WeChat response forms;
- asynchronous URL-upload creation now requires a non-empty job id, while
  polling results validate positive statuses and apply distinct processing,
  completed, and failed contracts; completed results require media identity
  and creation time, failed results require a non-zero detail error, and
  unknown positive statuses remain forward compatible;
- checked accessors expose verified image URLs, media/job identifiers, and
  creation timestamps for production callers and polling workers;
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
- complete-media, ranged-media, and JSSDK download network exits now enforce
  successful non-empty responses, parseable content lengths and ranges, and
  exact body-length consistency before returning bytes; ranged downloads also
  require status 206, the requested start offset, and an end offset no later
  than requested, preventing ignored or mismatched range responses from being
  appended to resumable files;
- attachment image/video uploads now enforce the same minimum and 2/10 MiB
  maximum byte boundaries as their corresponding temporary-media kinds before
  multipart network I/O;
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

Implemented on 2026-07-20 in Work OA calendar and schedule response contracts:

- calendar and schedule add/get plus mutation responses now validate API
  success before leaving the HTTP boundary, including both direct schedule
  lookup and calendar-scoped listing;
- create responses require usable calendar or schedule ids and expose checked
  identity accessors, while list responses enforce the 1000-resource boundary
  and reject duplicate or unidentified resources;
- calendar details validate unique administrators and shares, optional
  `#RRGGBB` colors, binary flags, share access modes, and unique positive
  public-range principals while preserving unknown upstream fields;
- schedule details require positive ordered timestamps, reuse the complete
  administrator, attendee, reminder, calendar, and organizer contract, reject
  negative status/sequence values, and expose lookup and duration helpers;
  calendar responses expose public-calendar and principal-count helpers.

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

Implemented on 2026-07-20 in Open Platform authorizer release workflow depth:

- submitted and latest audit responses expose checked positive audit IDs,
  while audit states distinguish terminal, releasable, and operator-attention
  outcomes without treating future unknown states as approved;
- audit status and latest-status responses validate required status data,
  require a reason for rejected audits, expose rejection reasons, and provide
  an explicit release-readiness guard;
- audit quota responses validate non-negative remaining/limit pairs, reject
  impossible remaining values, and expose submit/speedup availability plus
  used-quota helpers;
- rollback release versions validate percentages, timestamps, version labels,
  and unique application versions, with latest-version and application-version
  lookup helpers for recovery workflows;
- gray-release plans validate timestamps, percentages, and running-state
  consistency, while plan responses expose a direct active-release helper;
- support-version responses reject blank current versions, negative UV counts,
  and percentages outside zero through one hundred before rollout decisions.

Implemented on 2026-07-20 in Open Platform authorizer domain, tester, privacy,
and shared permanent-material response depth:

- domain mutation responses normalize string/object result variants, expose
  configured and invalid result sets, distinguish non-ICP failures, validate
  required domain identities, preserve future fields, and provide an explicit
  all-applied guard for deployment workflows;
- tester bind responses require a nonblank `userstr`, while tester lists
  validate member identities, timestamps, unique `userstr` and WeChat-id
  values, and expose direct lookup by either identity;
- privacy setting responses validate API errors, code-state flags, versions,
  timestamps, setting/SDK/description uniqueness, and absolute evidence URLs;
  code-detected privacy keys can be compared against declared settings before
  an audit submission;
- privacy extension uploads and interface applications require their returned
  media/audit ids, while interface lists validate unique API identities and
  expose lookup, action-required, and all-approved semantics without rejecting
  future status codes;
- the shared Official Account/Open Platform material DTOs now validate API
  errors, media ids, URLs, timestamps, decoded page counts, unique items, and
  non-negative statistics; checked next-page and aggregate-count helpers
  reject stalled pagination and integer overflow.

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

Implemented on 2026-07-20 in Work external-contact corporate and strategy tag
response contracts:

- corporate and strategy tag list/add/edit/delete plus customer tag-marking
  operations now surface nonzero Work API errors at the HTTP boundary;
- list responses require unique nonblank group and tag ids, complete bounded
  names, non-negative ordering, positive optional creation times, and
  consistent positive strategy ownership for strategy-tag groups;
- add responses require a group with at least one returned tag while accepting
  the documented compact result shape, and expose checked group/tag lookup
  helpers without discarding extension fields;
- malformed duplicate groups/tags, mixed strategy ids, missing create results,
  and corporate groups carrying strategy ownership are rejected;
- the delete wrapper continues to use the documented
  `del_strategy_tag` endpoint, correcting PowerWeChat's implementation that
  routes strategy-tag deletion to `edit_strategy_tag`.

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

Implemented on 2026-07-20 in Work external-contact group-chat response
contracts:

- list/get, resigned and on-job transfer, openGID conversion, and join-way
  add/get methods now validate typed responses before returning application
  data;
- list responses enforce the 1000-row boundary, required unique chat ids, and
  non-negative statuses, while exposing normalized cursor and chat lookup
  helpers;
- group details require chat/owner identity, positive creation time, members,
  unique member/admin ids, positive member type/join timestamps, valid inviter
  identity, and bounded state metadata;
- transfer responses require unique failed-chat records with non-zero error
  codes, openGID responses require a resolved chat id, and non-zero top-level
  errors are surfaced as `WechatError::Api`;
- join-way responses require configuration identity, an absolute HTTP(S)
  QR-code URL, scene and 0/1 auto-create semantics, bounded remark/state,
  unique chat ids, and complete automatic-room metadata;
- unknown positive status, member, join-scene, and join-way scene values plus
  extension fields remain available for forward-compatible inspection.

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

Implemented on 2026-07-20 in Work external-contact moment-strategy response
contracts:

- list, detail, range, create, and edit operations now validate successful
  responses at the HTTP boundary and surface nonzero Work API codes;
- strategy lists enforce the 1000-row boundary, positive unique strategy ids,
  nonblank names, non-negative parent ids, unique administrators, and positive
  optional creation timestamps while exposing normalized cursor and lookup
  helpers;
- detail responses require a usable strategy object, and range responses
  validate unique user/department selections, positive department ids, and
  the 1000-row boundary with user/department membership helpers;
- future positive range types remain available through extension fields,
  whereas zero or negative types are rejected as malformed payloads;
- create responses require a positive strategy id and expose a checked
  accessor, while edit responses remain compatible with status-only success
  payloads and validate a strategy id when one is returned.

Implemented on 2026-07-20 in Work external-contact customer-strategy response
contracts:

- list, detail, range, create, edit, and delete operations now validate Work
  API errors and typed payloads before returning from the HTTP boundary;
- customer strategies require positive unique ids, nonblank names,
  non-negative parent ids, unique administrators, and positive optional
  creation/update timestamps, with detail and create helpers requiring their
  primary result values;
- list and range responses enforce the 1000-row boundary, normalize blank
  cursors, expose strategy lookup and user/department membership helpers, and
  reject duplicate known ranges while preserving future positive range types;
- this replaces PowerWeChat's HashMap-based strategy/range payloads and its
  incorrectly HashMap-typed `strategy_id` create result with production-safe
  typed contracts while retaining unknown extension fields.

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

Implemented on 2026-07-20 in Work external-contact moment response contracts:

- moment list, member-task, selected-customer, send-result, interaction,
  creation, and asynchronous-result operations now validate successful
  responses at the HTTP boundary and surface nonzero Work API codes;
- list responses enforce their documented 100, 1000, and 5000-row boundaries,
  reject duplicate moment, member, customer, comment, and like identities,
  normalize blank cursors, and expose checked lookup and aggregate helpers;
- moment summaries require stable identity, creator, timestamp, creation and
  visibility metadata, plus real text or media content; image, video, link,
  location, media-id, URL, coordinate, and item-count contracts are checked;
- task and customer records accept both current `publish_status` and
  compatibility `status` fields, reject missing or negative values, retain
  unknown non-negative states, and expose published-count helpers;
- task creation requires a bounded job id, while asynchronous results
  distinguish in-progress and finished payloads, require a moment id only for
  successful completion, preserve typed business failures, and report invalid
  sender, department, tag, and chat counts.

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

Implemented on 2026-07-21 in Work OA meeting and meeting-room response
contracts:

- meeting create, update, cancel, user-list, current detail, and legacy OA
  detail boundaries now reject upstream API errors and structurally invalid
  success responses before returning them to callers;
- create responses require a meeting id, excess-user and meeting-id lists
  require unique non-empty values, pagination exposes checked cursor helpers,
  and meeting details validate creator, title, reservation time, optional
  lifecycle fields, settings, reminders, and typed member/external/device
  attendees;
- meeting-room add/edit/delete/list, booking-list, direct booking, linked
  schedule/meeting booking, cancellation, and booking-detail boundaries now
  enforce response contracts after every HTTP call;
- room records require positive unique ids, valid profiles, equipment, and
  coordinates; booking records validate linked meeting/schedule ids, ordered
  time ranges, bookers, attendees, and duplicate identities;
- linked bookings distinguish conflict-only responses from completed bookings,
  require a booking id when no conflicts exist, and validate unique positive
  conflict timestamps; checked accessors expose verified meeting, room,
  booking, schedule, and creator identities.

Implemented on 2026-07-20 in Work OA checkin depth:

- added the missing official hardware-checkin endpoint
  `hardware/get_hardware_checkin_data`, including typed request builders for
  checkin-time and upload-time filtering plus typed device name/serial response
  records;
- added the self-built-application punch-correction endpoint with optional
  scheduled checkin offsets and remarks, matching the current
  `checkin/punch_correction` contract;
- rule lookup, raw records, daily/monthly reports, schedule lookup, schedule
  mutation, face upload, rule mutation, rule deletion, hardware records, and
  punch correction now validate before network I/O;
- record and report queries enforce valid type/filter codes, positive ordered
  timestamps, the upstream 30-day or one-month window, and batches of 1 to 100
  unique users;
- schedule mutation validates positive rule ids, `YYYYMM`, actual calendar
  days including leap years, non-negative schedule ids including the documented
  rest-day value zero, and duplicate user/day assignments;
- face enrollment validates standard Base64 and the official 1 MiB decoded
  image limit, while punch correction validates user identity, timestamps, and
  the optional within-day scheduled offset;
- checkin rule mutations reject empty payloads and invalid group/range
  identifiers while retaining forward-compatible unknown fields.

Implemented on 2026-07-21 in Work OA checkin response contracts:

- corporation/user rule lookup, raw and hardware records, daily/monthly
  reports, and schedule lookup now validate API success and response structure
  at all seven query HTTP boundaries; schedule, face, correction, rule-add,
  rule-update, and rule-delete status responses are also checked before return;
- rule groups require stable unique ids and names and recursively validate
  weekdays, time/rest sections, special dates, Wi-Fi/location/range entries,
  reporters, schedules, overtime settings, correction reminders, identities,
  timestamps, counters, and durations while preserving unknown fields;
- raw records require a user, checkin type, positive timestamp, paired valid
  coordinates, unique media, and non-negative linked ids; hardware records add
  checked device name/serial and typed `upload_time`, while raw pagination now
  exposes typed `has_more` semantics;
- daily reports validate real `YYYYMMDD` dates, unique user/date records,
  rule/schedule profiles, ordered summary timestamps, non-negative counts and
  durations, localized leave approvals, and overtime details; monthly reports
  enforce unique users, valid rule identities, non-negative totals, and
  consistent exception/work-day counts;
- schedule responses validate `YYYYMM`, actual month lengths including leap
  years, unique user/month and day entries, active/rest schedule semantics, and
  complete non-negative time sections; focused tests cover API errors,
  duplicates, missing required fields, half coordinates, impossible dates and
  totals, reversed timestamps, and invalid calendar days.

Implemented on 2026-07-20 in Work OA journal depth:

- added the two current enterprise-WeChat journal document export endpoints,
  `oa/journal/export_doc` and `oa/journal/get_export_doc_result`, which are
  absent from the compared PowerWeChat journal client;
- document exports use typed request, task, and polling-result DTOs, preserve
  unknown upstream fields, and expose completed-result and download-URL
  helpers for production polling workers;
- record-list and statistics timestamps now serialize as the numeric Unix
  values required by the official wire contract instead of JSON strings;
- record queries validate positive ordered timestamps, the one-month window,
  non-negative cursors, the 1-to-100 page boundary, supported unique filters,
  and non-empty filter values before network I/O;
- detail, statistics, export, and export-result paths validate required
  record, template, document, and job identifiers, while statistics queries
  enforce the documented one-year maximum window.

Implemented on 2026-07-21 in Work OA journal response contracts:

- record-list, detail, statistics, document-export, and export-result methods
  now reject API errors and structurally invalid success responses at the HTTP
  boundary;
- record pagination accepts both PowerWeChat boolean and Enterprise WeChat
  integer `endflag` forms, enforces the 100-row limit, unique non-empty record
  ids, and positive continuation cursors only when more data exists;
- journal details require stable record, template, report-time, submitter,
  receiver, comment, and typed form-content contracts, while statistics
  validate template identities, report/white ranges, receivers, leader levels,
  cycle/stat time ranges, and per-user report records;
- export creation requires a usable job id; polling exposes forward-compatible
  processing/completed/failed status semantics and requires a validated
  absolute HTTP(S) download URL only for completed tasks.

Implemented on 2026-07-20 in Work OA living depth:

- create, modify, cancel, replay deletion, WeChat viewing-code, member-list,
  detail, watch-stat, and share-attribution paths now validate their required
  identities and request contracts before network I/O;
- create and modify requests enforce positive start/duration values, the
  documented title and description boundaries, live types 0 through 4,
  non-negative reminder offsets, positive legacy agent ids, and non-empty
  changes for modifications;
- activity-only cover/share/detail fields require live type 4, and activity
  image lists enforce non-empty unique media ids plus the five-image limit;
- member live-list pagination exposes a first-page constructor and enforces
  the 1-to-100 row boundary, while list and watch-stat responses expose
  explicit continuation helpers;
- all nine living operations now validate successful Work API envelopes at
  the HTTP boundary, including mutation status responses and required payloads;
- create, viewing-code, and share responses accept both numeric and string
  wire representations used by PowerWeChat and Enterprise WeChat while
  exposing checked string accessors;
- live lists reject blank, duplicate, or oversized ID pages; detail responses
  reject impossible negative counters, timestamps, and non-binary replay flags;
- watch-stat responses validate continuation keys, required statistics,
  viewer identities, non-negative watch times, binary interaction flags, and
  duplicate viewers, and expose checked aggregate viewer counts;
- share-attribution responses require a live ID and at least one internal or
  external viewer identity while retaining inviter and unknown extension data;
- live type, lifecycle status, replay status, and external-viewer type now
  expose forward-compatible semantic enums and terminal/replay helpers;
- external watch-stat records now retain inviter member and external-contact
  identities in addition to unknown upstream extension fields.

Implemented on 2026-07-20 in Work OA approval depth:

- template detail/copy, application submission, modern approval-list/detail,
  legacy approval-data, and template create/update paths now validate required
  identifiers and request contracts before network I/O;
- modern list queries expose a first-page constructor and enforce positive
  ordered timestamps, the 31-day window, 1-to-100 page size, nonblank cursors,
  supported unique filters, and known approval-status filter values;
- legacy approval-data queries expose a first-page constructor and enforce
  the 30-day window plus a non-negative continuation approval number;
- application submission validates template/custom approver mode, approver
  relations and unique users, notifier settings, nonempty control values,
  nested table rows, positive date ranges, localized summaries of at most
  three 20-character rows, and the six-file aggregate attachment limit;
- template mutations validate localized names, nonempty unique controls,
  binary property flags, selector modes/options, and the single
  vacation-or-attendance control constraint;
- modern list and legacy data responses expose continuation helpers, while
  approval and approval-node status codes expose forward-compatible semantic
  enums with terminal and approved-state helpers.

Implemented on 2026-07-20 in Work OA approval response contracts:

- template detail, application submission, modern list/detail, legacy data,
  template create/copy, and template update responses now validate API success
  before leaving the HTTP boundary;
- template responses require localized names, valid typed content, and usable
  template ids, while application submission responses require a usable
  approval number and expose checked identity accessors;
- modern lists enforce the 100-row response boundary, unique nonblank approval
  numbers, and nonblank continuation cursors; details require stable approval,
  template, applicant, timestamp, content, record, comment, process-node, and
  batch-applicant identities while keeping future positive status codes and
  accepting empty values only for optional response controls;
- legacy data requires non-negative consistent count/total/continuation
  metadata, unique positive approval numbers, valid applicant and field
  identities, positive times, finite non-negative expense amounts, and ordered
  leave intervals; both modern and legacy responses retain unknown fields.

Implemented on 2026-07-20 in Work OA vacation depth:

- member quota reads now reject blank user ids, and quota mutations validate
  member identity, positive vacation ids, non-negative balances, supported
  day/hour time units, and nonblank remarks of at most 200 characters before
  network I/O;
- day balances enforce the documented 0.1-day (`8640` seconds) increment and
  1000-day ceiling, while hour balances enforce the 0.1-hour (`360` seconds)
  increment and 24000-hour ceiling;
- quota requests expose checked constructors in tenths of days or hours plus
  a remarks builder, avoiding unchecked unit conversion and integer overflow
  in application code;
- vacation configurations expose forward-compatible day/hour unit semantics,
  unit-second and minimum-increment helpers, and ID lookups;
- quota responses expose ID lookup plus assigned, used, remaining, and
  effective-assigned conversions for the selected day/hour unit while
  retaining exact upstream seconds.

Implemented on 2026-07-21 in Work OA vacation response contracts:

- corporation configuration and member quota queries now validate API success
  and nested response structure at the HTTP boundary, while quota mutations
  validate their status response before return;
- configurations require unique positive vacation ids, nonblank names,
  non-negative current or future unit/type codes, valid binary flags,
  non-negative durations, and checked automatic-reset and expiration dates;
- quota policies recursively validate reset metadata, nonempty unique service-
  or company-age ranges, the documented zero-ended unbounded range, at most
  one open-ended rule, and optional extended-expiration requirements;
- member quota responses require unique positive vacation ids, nonblank names,
  and non-negative assigned, used, remaining, and actual-assigned durations
  without assuming those independent upstream values must sum to each other;
- checked configuration/quota lookup, known-unit, depletion, and overflow-safe
  aggregate duration helpers are available; focused tests cover API errors,
  duplicate ids/ranges, future units, zero sentinel dates, invalid switches,
  reset/expiration dates, negative balances, official independent-balance
  semantics, and aggregate overflow.

Implemented on 2026-07-20 in Work OA dial and PSTNCC depth:

- public-phone record requests expose first-page, recent-30-days, and checked
  next-page constructors, and reject partial/invalid timestamps, ranges over
  30 days, negative offsets, and page sizes outside 1 through 100 before
  network I/O;
- public-phone records now match the actual upstream `call_time`,
  `total_duration`, structured `caller`, and structured `callee[]` wire
  contract instead of attempting to deserialize caller/callee objects as
  strings;
- caller and callee records preserve member ids, textual or numeric phone
  values, per-party durations, and unknown extension fields, while call types
  expose single-party/multi-party semantic helpers;
- PSTNCC call submission rejects empty, blank, or duplicate callee lists, and
  state queries reject blank member and call ids before network I/O;
- PSTNCC initiation results now model the documented `userid`, `callid`, and
  `code` fields, retain aliases for older field names, and expose per-call,
  aggregate-success, and failed-result helpers;
- PSTNCC query responses expose talked-state and forward-compatible reason
  semantics for ringing, answering, active calls, timeout, rejection,
  service, balance, rate-limit, line, cancellation, and unknown outcomes.

Implemented on 2026-07-21 in Work OA dial and PSTNCC response contracts:

- dial-record, PSTNCC call, and PSTNCC state methods now validate API success
  and response structure before returning from their HTTP boundaries;
- dial records require positive call times, non-negative durations, positive
  call types, a caller, and at least one uniquely identified callee; caller and
  callee records require a member or phone identity plus per-party duration;
- PSTNCC initiation responses require non-empty unique user states, non-negative
  result codes, and a call id for successful entries while retaining
  failure-only entries without call ids;
- state queries require a binary talked flag, positive call timestamp,
  non-negative talk duration and reason code, with checked call-id accessors
  and existing forward-compatible reason semantics preserved.

Implemented on 2026-07-20 in Work OA WeDrive lifecycle depth:

- existing space, file, ACL, sharing, listing, upload, move, rename, delete,
  download, and information methods now validate their request contracts
  before network I/O;
- identifiers and names reject blank or control-character values, while space
  subtype, sorting, pagination, sharing scope, and permission values enforce
  their supported ranges;
- ACL entries enforce matching member/department identities, positive
  department ids, unique principals, and permissions on member additions
  while allowing permission-free member removals;
- file listing exposes first-page and checked next-page helpers, accepts the
  documented empty parent id for a space root, and enforces non-negative
  offsets plus the 1-to-100 page boundary;
- direct uploads reject empty, malformed, and data-URL-prefixed Base64 before
  sending, while file moves enforce a nonempty unique batch of at most 100
  file ids;
- the legacy PowerWeChat surface was also compared against the current
  official WeDrive catalog, exposing newer chunked-upload, file-permission,
  file-security, professional-edition, and capacity APIs for the next
  endpoint-expansion batch.

Implemented on 2026-07-20 in current Work OA WeDrive endpoint depth:

- added the current official `file_upload_init`, `file_upload_part`,
  `file_upload_finish`, `get_file_permission`, `file_secure_setting`,
  `mng_pro_info`, and `mng_capacity` endpoints plus direct upload through a
  file-selector ticket;
- chunk initialization supports checked location and selected-ticket
  constructors, enforces their exclusive target contract, validates the
  2-MiB block count and cumulative 40-character SHA-1 states, and rejects
  empty files or files above 20 GiB;
- chunk parts reject invalid indices, malformed Base64, and decoded blocks
  above 2 MiB, while direct uploads now enforce the documented 10-MiB limit;
- file-permission responses now expose typed sharing ranges, security
  settings, inherited ACLs, direct members, and watermark settings while
  preserving future upstream fields;
- watermark updates require a real change and validate text and density,
  while professional-edition and capacity responses expose available-account
  and used-capacity helpers;
- corrected legacy wire contracts: file list sorting now accepts values 1
  through 6 with pages up to 1000, file creation serializes numeric file
  types, file sharing accepts scopes 1 through 5 with permissions 1 or 4,
  and file deletion sends a unique file-id array instead of a scalar.

Implemented on 2026-07-21 in Work OA WeDrive response-contract depth:

- all 29 WeDrive HTTP return boundaries now reject nonzero WeCom API status
  codes and validate successful typed or status responses before returning
  them to callers;
- space creation, detail, and sharing responses require usable identifiers,
  names, ACL members, settings, and absolute HTTP(S) share URLs, with checked
  accessors for created space ids, space details, and share URLs;
- file listings require an explicit pagination state and file container,
  reject negative continuation offsets, duplicate ids, oversized pages,
  invalid file metadata, reversed timestamps, and malformed optional URLs;
- upload responses require file ids, while chunk initialization enforces the
  exclusive deduplication contract: an existing file returns only `fileid`
  and a pending upload returns only `upload_key`; checked accessors expose
  each lifecycle branch;
- download responses require absolute HTTP(S) URLs and paired cookie fields;
  create, rename, move, info, and share responses now validate their required
  payloads before use;
- file-permission responses validate enabled sharing ranges, ACL identities
  and uniqueness, inherited members, ACL timestamps, and watermark density
  while retaining unknown positive upstream permission and enum values;
- professional-edition and capacity responses enforce required fields,
  positive expiry, used-account bounds, paired VIP capacity fields, and
  remaining-capacity bounds; normal and malformed response matrices cover
  API failures, missing payloads, contradictions, duplicates, invalid URLs,
  reversed timestamps, permissions, quotas, and capacity overflows.

Implemented on 2026-07-21 in Work account-service production-contract depth:

- all 20 public account-service methods now validate local inputs, and their
  18 distinct HTTP boundaries validate typed or status responses before
  returning; convenience servicer methods continue through the checked
  request-based implementations;
- account create/update/delete, list, contact-way, customer batch, upgrade,
  cancel-upgrade, message sync/send, servicer, and session-state requests now
  reject blank identifiers, invalid names/scenes, duplicate or oversized
  batches, malformed upgrade targets, unsupported flags, limits, voice
  formats, message types, and contradictory state assignments before I/O;
- outbound messages enforce exactly one payload matching `msgtype`; text,
  media, video, link, mini-program, menu, location, and event-send payloads
  validate their required fields, URLs, coordinates, menu identities, and
  supported event-message types;
- corrected the customer-acquisition link wire contract to use its dedicated
  `{ "link_url": ... }` payload instead of serializing ordinary link
  `title/url` fields under `ca_link`;
- account and customer responses validate required identities, unique records,
  avatar/contact URLs, session context structure, gender bounds, and
  successful/invalid customer-list disjointness while preserving unknown
  positive response values;
- synchronized-message responses enforce binary pagination state, checked
  cursor continuation, the 1000-message boundary, unique message ids, and a
  single body matching every known message type; future message types remain
  forward compatible through extension fields;
- synchronized text/media/location/link/business-card/mini-program/menu,
  Channels product/order, and event payloads now validate required content,
  URLs, coordinates, identities, and event-specific failure/servicer/session/
  recall fields;
- servicer mutations expose partial-failure iteration and reject missing,
  duplicated, or ambiguous user/department identities; servicer lists and
  session-state responses validate identity/status coherence, including the
  required servicer for human-service sessions;
- normal and malformed request/response matrices cover API failures, duplicate
  accounts/customers/messages/servicers, cursor contradictions, body/type
  mismatches, malformed URLs/config ranges, partial servicer failures, and
  invalid state transitions.

Implemented on 2026-07-21 in Work user-tag response-contract depth:

- added the explicit `user_tag()` module entry while retaining the historical
  account-service tag entry for compatibility; all 9 public tag operations
  now flow through 6 validated HTTP boundaries;
- create, update, delete, detail, add/remove members, and list responses reject
  nonzero WeCom API status codes before returning, while create responses
  expose a checked positive tag-id accessor;
- corrected partial-failure compatibility across the upstream add/remove
  shapes by accepting both `invaliduser` and `invalidlist`, preserving one
  normalized invalid-user helper for callers;
- tag details and lists require valid names and positive identities, reject
  duplicate tag/user/department ids, and validate optional user names while
  preserving unknown response fields;
- membership mutation responses retain partial success but validate unique,
  nonblank invalid users and unique positive invalid departments; tag names
  now also reject control characters before network I/O;
- normal and malformed matrices cover API errors, missing ids, duplicate
  users/departments/tags, both partial-failure wire names, invalid department
  ids, overlong names, and control-character names.

Implemented on 2026-07-20 in Work application-message depth:

- ordinary sends, statistics, recall, template-card and task-card updates,
  linked-corporation sends, and school notifications now validate Work API
  success before leaving the HTTP boundary;
- ordinary send responses require a checked message ID, while template-card
  update responses use a separate contract because the update endpoint does
  not return a new message ID;
- pipe-delimited invalid and unlicensed recipients now reject blank segments
  and duplicates, and linked-corporation plus school failure lists enforce
  their documented recipient limits and expose aggregate partial-failure
  counts;
- application-message statistics require unique positive agent IDs and
  non-negative counts, reject arithmetic overflow through a checked total
  helper, and keep a saturating compatibility total for existing callers;
- AppChat create, update, get, and send paths now validate requests and
  successful responses, including chat identity, required owners, unique
  members, owner membership, actual update patches, and disjoint add/remove
  sets;
- application-message audiences now enforce the documented 1000-member,
  100-department, and 100-tag limits, reject blank and duplicate recipients,
  and accept duplicate-check windows through the documented four-hour
  (`14400` seconds) maximum;
- text and Markdown bodies, media ids, video metadata, text cards, task cards,
  news, stored news, and mini-program notices validate required fields and
  documented UTF-8 byte, item-count, character-count, and absolute HTTP(S)
  URL constraints before network I/O;
- confidential-message validation now permits `safe=2` only for stored
  `mpnews`, while ordinary message types continue to accept only `0` or `1`;
- linked-corporation messages now expose `toall`, `safe`, and typed
  constructors for every supported payload, enforce recipient limits, and
  validate exactly one payload whose type matches `msgtype`;
- school notifications now serialize the documented `to_external_user` wire
  name, add `toall`, voice, video, file, news, stored-news, ID-translation,
  and duplicate-check fields, and validate recipient, receive-scope, payload,
  and four-hour duplicate-window constraints;
- application-chat messages now use the same payload and confidentiality
  checks before sending, while ordinary, linked-corporation, and school
  responses expose invalid-recipient counts and delivery-failure helpers.

Implemented on 2026-07-20 in Work user, tag, and identity-conversion depth:

- core member CRUD, department-member lists, user-ID pagination, batch
  import/export, lookup, invitation, join-code, active-stat, linked-corporation,
  and userid/openid conversion responses now validate Work API success before
  leaving the HTTP boundary;
- user detail and list responses require a usable plaintext or privacy-mode
  identity, validate unique positive departments, aligned department metadata,
  leader flags, main-department membership, status values, and duplicate users;
- user-ID pages validate up to 10000 entries, exact user/department
  duplication, and expose normalized cursor continuation helpers;
- import/export start responses require checked job IDs, while result
  responses validate status, job type, totals, progress, row data, and expose
  completion plus partial-failure helpers without rejecting future job kinds;
- invitation results validate invalid-member, department, and tag sets and
  expose aggregate failure counts; join QR codes require absolute HTTP(S)
  URLs, and active counts accept numeric or string wire forms with checked
  unsigned conversion;
- linked-corporation permission, user, user-list, and department-list
  responses validate required identities, hierarchy fields, duplicate entries,
  and forward-compatible positive statuses while retaining unknown fields;
- department member lists, authorization confirmation, linked-corporation
  member and department reads, join QR codes, mobile/email lookups, and active
  statistics now reject invalid identifiers, dimensions, formats, types, and
  dates before network I/O;
- active-stat dates use the documented `YYYY-MM-DD` format and are limited to
  today or the previous 30 days, while mobile and email checks prevent bad
  identity-lookup attempts from consuming upstream error and rate limits;
- userid/openid, unionid/external-userid, pending-id, open-userid, and external
  tag-id conversion requests now enforce required values, supported subject
  types, positive source agents, unique batches, and the 1000-item boundary;
- current conversion response fields `invalid_open_userid_list` and
  `invalid_external_tagid_list` are modeled with backward-compatible aliases,
  userid conversion now retains `invalid_userid_list`, and mapping lookup plus
  failure-count helpers expose partial-success semantics;
- service-provider `user/list_id` results now retain `open_userid` alongside
  plaintext `userid`, supporting both current privacy modes without dropping
  an identity;
- enterprise tag creation, updates, reads, deletion, and membership changes
  validate positive ids and 32-character names, omit optional zero tag ids on
  creation, restrict the compatibility endpoint argument to the two official
  member mutation paths, enforce unique batches, and serialize department ids
  as JSON integers while preserving the existing string-based API signature;
- tag membership results expose parsed invalid user ids, aggregate failure
  counts, and partial-failure detection.

Implemented on 2026-07-20 in Work external-contact group-message lifecycle
depth:

- group-message list, member-task, send-result, legacy result, welcome,
  reminder, and cancellation operations now validate their request contracts
  before network I/O;
- list requests enforce supported chat and filter types, ordered positive time
  ranges of at most one month, nonblank creators and cursors, and the
  documented 100-item page boundary;
- member-task and send-result requests now serialize the current official
  `cursor` field instead of PowerWeChat's legacy `msgcursorid`, enforce the
  1000-item page boundary, and reject blank message or member identities;
- template audiences and tag groups reject blank or duplicate identifiers,
  while text, image, link, mini-program, video, and file payloads enforce
  matching message types, required fields, limits, and absolute HTTP(S) URLs;
- welcome messages require a nonblank callback code and real text or
  attachment content, enforce the nine-attachment boundary, and reuse the
  payload validation applied to group-message templates;
- group-message creation, member-task, and delivery-result DTOs expose
  forward-compatible semantic status helpers plus aggregate pending, sent,
  and failure counts, treating a blank next cursor as the final page.

Implemented on 2026-07-20 in Work external-contact group-message response
contracts:

- message-template creation, group-message list, member-task, send-result, and
  legacy result operations now reject upstream API errors and malformed
  successful payloads at the HTTP boundary;
- successful template creation requires a bounded message id and unique,
  nonblank failed-recipient ids, while exposing checked message-id and
  partial-failure helpers;
- message lists enforce the 100-row boundary, unique message ids, required
  creator and creation metadata, bounded content, and forward-compatible
  non-negative creation types;
- task and delivery-result pages enforce the 1000-row boundary, required
  member and recipient identities, unique rows, non-negative statuses and
  timestamps, and a positive send timestamp for records explicitly marked as
  sent;
- all paged responses normalize blank cursors and expose pagination and lookup
  helpers, while unknown non-negative status values and unknown attachment
  kinds remain available for future WeCom extensions.

Implemented on 2026-07-20 in Work external-contact group welcome-template
depth:

- add, edit, get, and delete operations now validate their request contracts
  before network I/O, including nonblank bounded template ids and non-negative
  legacy suite agent ids;
- template bodies require real text or one attachment, permit text plus one
  attachment, and reject multiple non-text payloads instead of silently
  relying on the upstream image/link/mini-program/file/video priority order;
- text, image, link, mini-program, file, and video fields reuse the shared
  production payload validation for required media identifiers, absolute
  HTTP(S) URLs, and documented byte limits;
- checked text-only and attachment-only constructors, a replacement
  attachment builder, and an explicit legacy-agent builder make valid modern
  and legacy-suite requests straightforward while preserving the existing
  public DTO fields;
- modern delete and template requests omit `agentid` when it is zero, while
  positive legacy multi-application suite ids remain wire compatible;
- add/get responses retain template, agent, notification, and future extension
  fields and expose typed notification and attachment-kind helpers.

Implemented on 2026-07-20 in Work external-contact group welcome-template
response contracts:

- add and get operations now validate successful response DTOs at the HTTP
  boundary and convert nonzero Work API codes into typed API errors;
- creation responses require a nonblank bounded template id and expose a
  checked accessor, preventing a nominally successful but unusable template
  from entering application state;
- get responses validate optional template ids, non-negative legacy agent ids,
  notification values, required content, and the single non-text attachment
  invariant while reusing existing text, media-id, URL, and byte limits;
- the current PowerWeChat response shape, which can omit template, agent, and
  notification metadata, remains supported, while newer response metadata and
  unknown extension fields continue to round-trip;
- response helpers now expose content presence and reconstruct the typed
  attachment for downstream dispatch or inspection.

Implemented on 2026-07-20 in Work external-contact customer-acquisition
depth:

- link list/get/create/update/delete plus customer list, usage statistic, and
  chat-detail operations now validate their contracts before network I/O;
- link pages enforce the 100-item limit, customer pages enforce the 1000-item
  limit, and both reject blank continuation cursors while exposing checked
  first-page and next-page constructors;
- acquisition ranges now serialize department ids as JSON integers instead of
  PowerWeChat's string-array mismatch, require a nonempty unique member or
  department scope, and enforce the 100-principal request boundary;
- priority assignment exposes enterprise-wide and specified-member
  constructors, validates type 1/2 semantics, and rejects empty, duplicate,
  blank, or oversized member selections;
- partial link updates now preserve explicit `false` booleans, require at
  least one changed field, support the current optional `mark_source` setting,
  and validate link ids, names, ranges, and priority settings;
- link-detail responses model the actual top-level `range`, `skip_verify`,
  `priority_option`, and `mark_source` fields while retaining compatibility
  helpers for older nested response shapes;
- customer status now distinguishes not-messaged, messaged, unknown, and
  future values, with aggregate helpers for delivered and unknown records;
- quota, 30-day statistic, and chat-detail DTOs expose available/used quota,
  active-expiry, conversion-rate, customer-identity, and received-message
  semantics while preserving extension fields.

Implemented on 2026-07-20 in Work external-contact customer-acquisition
response contracts:

- link list/get/create, quota, attributed-customer, statistic, and chat-info
  network methods now validate typed responses before returning application
  data;
- links require bounded ids/names, absolute HTTP(S) URLs, positive ordered
  timestamps, valid ranges and priority options; list pages reject duplicate
  ids and expose normalized cursor/lookup helpers;
- quota responses require non-negative totals and balances with
  `balance <= total`, while quota entries reject negative balances and invalid
  expiry timestamps;
- customer pages validate member/external identities, non-negative statuses,
  bounded state values, 1000-row limits, and duplicate customers, with
  normalized cursor and lookup helpers;
- statistics enforce non-negative counts and prevent new-customer totals from
  exceeding link-click totals; chat attribution requires both identities,
  typed chat info, non-negative message counts, and valid link/state metadata;
- manually deserialized non-zero API responses now surface as
  `WechatError::Api`, while unknown chat-status values and extension fields
  remain forward compatible.

Implemented on 2026-07-20 in Work external-contact product-album depth:

- add, update, delete, get, and list operations now validate their request
  contracts before network I/O;
- descriptions enforce the documented 300-character boundary, prices enforce
  the 0 to 5,000,000-cent range, and optional product codes enforce the
  128-byte ASCII letter-or-digit contract while updates can explicitly clear
  an existing product code;
- product creation requires one to nine unique image media ids, update requests
  require at least one changed field, and request attachments serialize only
  the exact official `image` wire type;
- list requests expose checked first-page and next-page constructors, enforce
  the 100-item page boundary, and reject blank continuation cursors;
- response DTOs expose product-presence, image-count, and price-format helpers,
  while optional image payloads and flattened extension fields preserve
  forward compatibility with future attachment and response fields.

Implemented on 2026-07-20 in Work external-contact intercept-rule depth:

- add, update, get, and delete operations now validate rule contracts and
  identifiers before network I/O, while the parameterless list operation
  remains unchanged;
- rule names enforce the 20-character boundary, sensitive-word lists enforce
  one to 300 unique entries of one to 32 characters, semantic rules accept
  only the three documented values, and intercept modes accept only warning
  plus block or warning-only;
- applicable ranges require at least one principal and cap each user or
  positive-department collection at 1000 unique entries, while updates reject
  empty changes or principals present in both add and remove ranges;
- checked constructors expose semantic enums for intercept modes and
  phone/email/red-packet rules, including an explicit clear-all semantic rule
  that serializes the required empty `semantics_list` array;
- detail responses now read the official nested
  `extra_rule.semantics_list` shape while retaining compatibility with the
  historical top-level field, and response helpers expose rule identity,
  principal totals, and identified-list counts.

Implemented on 2026-07-20 in Work external-contact customer-base depth:

- the external-contact unionid conversion path now applies the same unionid,
  openid, and subject-type validation as the shared ID-conversion endpoint,
  and school-notification subscription changes reject unsupported modes before
  network I/O;
- batch customer reads and served-contact lists expose checked first/next-page
  constructors with their documented 100-member and 1000-record limits and
  nonblank cursor checks;
- list, detail, follower, and batch responses expose identity, completeness,
  uniqueness, duplicate-detection, and follow-user aggregation helpers for
  production synchronization jobs;
- batch `follow_info.tag_id` and `follow_info.wechat_channels` are now typed
  instead of falling through to extension JSON, while detail-style tag
  objects and unknown response fields remain compatible;
- customer add-way semantics cover QR code, search/share, group and contact
  sources, video channels, calendar/meeting, hardware and on-site service,
  acquisition links, custom development, demand replies, presales/business
  sources, internal sharing, administrator assignment, and future values;
- unionid conversion and external-userid migration responses expose resolved,
  pending, mapped, changed, and lookup helpers without discarding extension
  fields.

Implemented on 2026-07-20 in Work external-contact customer-base response
contracts:

- customer list, served-contact list, external-userid-to-openid, detail,
  follow-user list, external-userid migration, unionid conversion, and batch
  detail operations now validate responses before leaving the HTTP boundary;
- customer and follower lists reject blank or duplicate identities, served
  entries require their customer or group-chat identity shape plus owner,
  temporary openid, and positive optional add time, and cursors normalize
  whitespace-only values;
- detail responses require an identified customer and unique valid follow
  users, while batch responses require complete customer/follow pairs and
  reject duplicate customers;
- follow records validate positive optional creation times, non-negative
  forward-compatible add-way/channel-source values, unique typed and compact
  tag ids, valid tag types, operator ids, and unique nonblank remark mobiles;
- openid conversion requires a usable openid, unionid conversion requires at
  least one resolved or pending identity while retaining the observed combined
  response form, and migration results require at most 1000 unique complete
  source-to-target mappings.

Implemented on 2026-07-20 in Work external-contact mutation-status response
contracts:

- `WorkStatusResponse` now exposes reusable checked success and operation-aware
  validation helpers, preserving extension fields while converting every
  nonzero Work API code into `WechatError::Api`;
- all 28 external-contact status mutations now validate at the HTTP boundary,
  covering school subscription, contact ways, temporary chats, remarks, tags,
  group-chat join ways, moment/customer strategies, welcome templates,
  customer-acquisition links, intercept rules, product albums, welcome/group
  messages, unassigned-customer transfer, and moment cancellation;
- omitted and explicit-zero error codes remain successful for compatibility,
  while error messages fall back to the concrete operation name when WeCom
  omits `errmsg`;
- group-chat external-userid migration now also applies the shared complete,
  unique, at-most-1000 mapping response contract before returning.

Implemented on 2026-07-20 in Work external-contact intercept-rule and
product-album response contracts:

- create, list, and detail operations now validate API success and resource
  invariants before responses leave the HTTP boundary;
- intercept-rule responses require valid identities, names, sensitive words,
  positive modes, applicable principals, positive optional timestamps, and
  unique semantic values while accepting future positive semantic and mode
  values;
- intercept-rule lists reject duplicate identities and expose checked lookup,
  while create/detail responses expose required-resource helpers;
- product responses require identities, descriptions, prices, one to nine
  attachments, valid known image media ids, unique image ids, and positive
  optional creation times while retaining future attachment types;
- product lists enforce the 100-item response boundary, reject duplicate
  product identities, normalize continuation cursors, and expose pagination
  plus checked lookup helpers.

Implemented on 2026-07-20 in Work external-customer transfer response
contracts:

- on-job and resigned transfer submission, result-query, and unassigned-list
  operations now validate successful responses before leaving the HTTP
  boundary;
- transfer submissions require one to 100 unique customer receipts with
  explicit per-customer error codes, while result pages accept up to 1000
  unique customers with positive forward-compatible statuses;
- completed transfers require positive takeover timestamps, optional handover
  and takeover identities are validated and kept distinct, and response
  helpers expose normalized cursors, lookup, and completed/pending/failure
  counts;
- unassigned pages require coherent `is_last` and continuation-cursor state,
  valid unique handover/customer pairs, and positive resignation timestamps,
  with checked pagination and pair lookup helpers.

Implemented on 2026-07-20 in Work external-contact statistics response
contracts:

- customer-behavior, group-chat summary, and group-chat daily-statistic
  operations now validate successful responses before leaving the HTTP
  boundary;
- behavior rows require unique user/day identities, positive statistic times,
  non-negative counters and reply times, and finite reply percentages from
  zero through 100;
- group-chat summary rows require unique owners, daily rows require unique
  positive statistic times, and both shapes require typed data while retaining
  unknown extension metrics;
- statistic metadata validates totals, offsets, and endpoint-specific item
  limits, while activity/new counts cannot exceed their corresponding totals;
- response helpers expose owner/day and user/day lookup plus reliable
  pagination state.

Implemented on 2026-07-20 in Work external-contact migration and school
subscription response contracts:

- service/corporation external-userid conversion in both directions now
  rejects API failures and requires a usable converted identity before leaving
  the HTTP boundary;
- migration completion now applies the shared Work mutation-status contract,
  including operation-aware fallback errors;
- school-subscription QR-code queries require all large, middle, and thumbnail
  absolute HTTP(S) URLs and expose them as one checked tuple;
- school-subscription mode queries require a positive mode, retain future
  positive values through the semantic enum, and expose a checked mode helper.

Implemented on 2026-07-20 in Payment merchant-service workflow depth:

- complaint list, detail, negotiation-history, reply, completion,
  notification-URL, and refund-approval operations now validate their request
  contracts before signing or network I/O;
- complaint list queries expose checked first/next-page constructors, validate
  real `YYYY-MM-DD` dates, ordered windows of at most 30 days, the current
  1-to-50 page boundary, non-negative offsets, and optional complained
  merchant identifiers;
- negotiation-history queries expose checked pagination with the current
  1-to-300 boundary, while all complaint path identifiers are length-checked
  and percent-encoded as path segments;
- callback configuration requires an absolute HTTPS URL of at most 255
  characters with a real callback path and no query, credentials, or fragment;
- complaint replies enforce merchant/content limits, unique batches of at
  most four image media ids, paired HTTPS jump URLs and 10-character labels,
  and complete mini-program jump information;
- refund approval and rejection constructors enforce supported actions,
  non-negative launch days, required rejection reasons, action-specific field
  separation, four-image evidence limits, and 200-character reason/remark
  boundaries;
- complaint list/detail/history and notification responses expose identity,
  lookup, pending-response, priority-attention, known-order-amount,
  refund/system-event, configured-callback, and API-error semantics while
  preserving upstream extension fields.

Implemented on 2026-07-20 in Payment merchant-service response contracts:

- complaint list and negotiation-history responses now validate required
  pagination metadata, page bounds, total-count consistency, item limits, and
  duplicate complaint/log identities before returning from network calls;
- complaint detail responses validate complaint identity, RFC3339 timestamps,
  state, non-negative counters/refund amounts, unique order/service-order
  identities, media URLs, tags, and nested record invariants;
- negotiation records validate log identity, operator, RFC3339 operation time,
  operation type, unique HTTPS image URLs, and typed media records;
- complaint callback create/query/update responses reject string-form WeChat
  API errors and require a valid merchant id plus an absolute HTTPS callback
  URL with a non-root path;
- all merchant-service query and callback response methods now enforce these
  contracts at the API boundary while retaining unknown extension fields.

Implemented on 2026-07-21 in Payment merchant-service nested response depth:

- complaint details now validate optional detail/problem/OpenID fields and
  recursively enforce shared-power additional information at the top-level
  response boundary;
- known shared-power payloads require an RFC3339 return time, return address,
  finite numeric longitude/latitude, and valid geographic coordinate ranges;
- negotiation normal/click messages now validate sender metadata, block
  cardinality, block-type payload matching, click action/log identities, and
  nested text, image, link, FAQ, button, and button-group contracts;
- known message actions now require their matching send-message, HTTPS URL, or
  mini-program payload, while unknown block/action types remain
  forward-compatible and retain extension fields;
- response tests cover malformed shared-power timestamps/coordinates, missing
  nested payloads, insecure action URLs, empty block lists, and incomplete
  click-message identities.

Implemented on 2026-07-20 in Payment bill download and statement depth:

- atomic file downloads now expose an explicit maximum-byte variant, enforce
  the limit before writing each response chunk, reject empty artifacts, and
  retain the existing compatibility entry point;
- committed bill files can be reopened with a caller-defined memory limit,
  checked against their recorded and observed lengths, reverified using their
  SHA-1 or SHA-256 digest, and parsed directly into a typed bill statement;
- post-download verification detects truncation, replacement, concurrent
  length changes, hash tampering, and oversized reads before reconciliation;
- statement total, filtered-total, grouped-total, and filtered-grouped-total
  helpers now use checked `i64` arithmetic and return explicit configuration
  errors instead of panicking in debug builds or wrapping in release builds;
- summary count reconciliation now uses checked `usize` to `i64` conversion.

Implemented on 2026-07-21 in Payment bill query and download contracts:

- trade, fund-flow, and profit-sharing bill queries now reject malformed
  calendar dates, unsupported trade/account/archive types, and invalid
  profit-sharing sub-merchant identities before signed network I/O;
- trade and fund-flow requests distinguish the upstream `bill_type` and
  `account_type` value sets while retaining the compatibility request shape;
- bill query responses now require an absolute HTTPS download URL together
  with a supported SHA-1/SHA-256 digest of the exact hexadecimal length before
  they can enter a download workflow;
- direct download requests validate URL and paired hash metadata before
  allocating files or sending signed requests, while the compatibility bytes
  path still permits explicitly unhashed caller-provided URLs and atomic file
  downloads continue to require integrity metadata.

Implemented on 2026-07-20 in Payment notification and order verification depth:

- notification envelopes now expose forward-compatible typed transaction,
  refund-success, refund-abnormal, refund-closed, complaint, and unknown event
  semantics plus stable notification IDs for idempotency keys;
- typed transaction, refund, and complaint resource decryptors validate the
  RFC3339 notification time, encrypted-resource marker, AES-256-GCM algorithm,
  field boundaries, event type, and original resource type before decryption,
  preventing a valid ciphertext from being decoded under the wrong workflow;
- order-query and transaction-notification responses expose effective
  app/merchant identities across direct and partner modes, checked promotion
  totals, and explicit payer-total plus promotion reconciliation;
- paid-order verification checks successful state, merchant identity,
  merchant order number, nonempty WeChat transaction number, exact total, and
  case-insensitive currency before application code accepts a payment;
- successful-refund verification checks merchant, order, merchant-refund,
  WeChat transaction, and WeChat refund identities together with total,
  refund, payer-total, and payer-refund boundaries;
- all promotion and reconciliation arithmetic is checked for overflow and
  returns typed configuration errors on inconsistent financial data.

Implemented on 2026-07-21 in Work OA WeDoc response-contract depth:

- 28 public boundaries across document lifecycle, sharing and authorization,
  VIP accounts, legacy content, image upload, administrators, document batch
  updates, spreadsheets, and collection forms now validate requests before
  transport and typed responses before returning application data;
- all typed WeDoc responses propagate nonzero WeChat `errcode` values, while
  successful create/read/share/upload operations require their documented
  identifiers, payload envelopes, versions, content, or absolute HTTP(S) URLs;
- document/form targets enforce exactly one wire identity, member and
  administrator changes reject missing or duplicate identities, permission
  departments and VIP lists require unique valid identifiers, and paginated
  responses require a cursor whenever `has_more` is true;
- spreadsheet reads require typed `data.result` envelopes, unique sheet ids,
  nonnegative dimensions and offsets, while batches require 1-to-100
  unambiguous operations and exactly one result variant per response item;
- form creation/modification, statistic queries, and answer reads validate
  titles, question and repeated ids, time/page ranges, positive unique answer
  ids, required response envelopes, nonnegative counters, timestamps, and
  duplicate records;
- realistic lifecycle/content/spreadsheet fixtures now execute the same
  validators used by network methods, with a dedicated failure matrix for API
  errors, missing payloads, malformed URLs, cursor breaks, duplicate
  identities, and ambiguous spreadsheet operations.

Implemented on 2026-07-21 in Work OA WeDoc smart-sheet response-contract depth:

- all 23 smart-sheet public network boundaries now validate typed requests
  before transport and validate either their dedicated response DTO or status
  response before returning application data;
- sheet, view, field, field-group, privilege, authorization, and record
  responses propagate WeChat API errors and require stable identifiers while
  preserving unknown future fields and unknown semantic type codes;
- list responses enforce nonnegative and internally consistent totals,
  continuation offsets whenever `has_more` is true, nonnegative versions and
  timestamps, required queried record values, and unique entity ids;
- field query/delete, view query/delete, and record query/delete requests now
  reject empty or duplicate identifiers, invalid versions, and out-of-range
  pagination at the client boundary;
- privilege responses validate mixed string/integer rule identities, per-rule
  unique sheet privileges, positive rule identities, and unique field rules;
  authorization reads require one of the supported typed authorization
  payload envelopes;
- realistic smart-sheet fixtures now execute production validators, and the
  failure matrix covers API errors, missing wrappers, duplicate entities,
  broken pagination, empty authorization data, missing record values, and
  duplicate deletion targets.

Implemented on 2026-07-21 in Work account-service customer-upgrade depth:

- customer upgrades now use typed member and group-chat targets instead of
  arbitrary JSON, matching PowerWeChat's `userid`/`chat_id` plus `wording`
  contracts;
- checked member and group-chat constructors select the exact upstream type,
  populate only the matching target, and reject blank identities, blank or
  control-character wording, overlong wording, and mixed target payloads before
  network I/O;
- upgrade-service configuration now exposes typed member and group-chat ranges,
  rejects blank or duplicate target identities, preserves nested extension
  fields, and provides direct eligibility lookup helpers;
- response and request tests cover both upgrade modes, malformed wire shapes,
  duplicate/blank ranges, incompatible targets, wording boundaries, and
  forward-compatible nested metadata.

Implemented on 2026-07-21 in Work user external-profile depth:

- member create/update and detail DTOs now include PowerWeChat's typed
  `wechat_channels` profile with nickname, status, semantic configured-state,
  and forward-compatible extension fields;
- request validation requires a bounded nonblank Channels nickname and rejects
  response-only status injection, while response validation rejects empty
  profile objects, negative statuses, blank nicknames, and malformed nested
  data before returning a member;
- external text, web, and mini-program attributes now require a type, bounded
  name, exactly one matching payload, required payload fields, and absolute
  HTTP(S) web URLs; profile attribute counts are capped at the upstream
  ten-item boundary;
- unknown future attribute types and nested profile metadata remain
  forward-compatible, while tests cover typed serialization, extension-field
  retention, mismatched payloads, relative URLs, empty/negative Channels
  states, and future attribute kinds.

Implemented on 2026-07-21 in Work application-menu depth:

- menu get/create/delete now reject non-positive agent ids, validate request
  trees before transport, and validate typed success or mutation responses
  before returning application data;
- create responses now use the shared typed menu-button DTO instead of
  `Vec<Value>`, preserving per-button and top-level extension fields;
- menu trees enforce one-to-three top-level buttons, one-to-five second-level
  buttons, no third level, bounded names, unique keys across the full tree, and
  parent/action-field separation;
- click, scan, picture, location, view, and mini-program buttons enforce their
  exact key/URL/appid/pagepath contracts; URLs require absolute credential-free
  HTTP(S), while unknown future button types remain forward-compatible;
- click, view, mini-program, and container constructors plus button-kind,
  total-count, and recursive key-lookup helpers remove routine raw field
  assembly from application code;
- request/response tests cover API errors, empty and oversized menus, duplicate
  keys, missing and mixed action fields, unsafe URLs, incomplete mini-program
  routes, invalid nesting, typed create responses, and future button payloads.

Implemented on 2026-07-21 in Work base and agent response-contract depth:

- access-token, callback-IP, API-domain-IP, agent list/detail, agent update,
  scope update, and workbench get/set network methods now validate requests and
  successful responses instead of returning deserialized data directly;
- access-token responses require a nonblank token and positive lifetime, while
  IP responses require nonempty unique valid IPv4/IPv6 addresses and expose
  membership lookup;
- agent summaries/details require positive unique ids, bounded names, valid
  logo/home URLs, valid redirect domains, 0/1 control flags, and unique valid
  user/department/tag scopes, with id lookup and closed-state helpers;
- `agent/set_scope` now uses PowerWeChat's exact flat `allow_user`,
  `allow_party`, and `allow_tag` wire fields instead of the previous incorrect
  response-shaped nested fields;
- agent updates require a real patch and validate ids, flags, names, media ids,
  domains, and URLs before transport; scope mutations enforce nonempty unique
  targets and the upstream user/department/tag boundaries;
- workbench requests and responses now enforce key-data, image, list, and
  webview type/payload matching, nested required fields, safe URLs, item
  boundaries, and typed future-response semantics; template requests also
  expose `replace_user_data`, and webviews retain jump URL/pagepath metadata;
- failure matrices cover API errors, missing tokens, invalid lifetimes,
  malformed/duplicate IPs and agents, bad flags/logos/scopes, empty updates,
  wrong workbench payloads, unsafe URLs, and forward-compatible future types.

## Documentation Update Needed

Keep `docs/powerwechat-gap-analysis.md` as the submodule-level view, but do not
use it as the final production parity signal. This method-depth audit should be
updated whenever a family reaches one-to-one endpoint coverage or PowerWeChat
adds new methods.
