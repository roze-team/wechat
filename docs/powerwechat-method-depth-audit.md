# PowerWeChat Method-Depth Audit

Audit date: 2026-07-09.

The submodule-level coverage matrix is currently green. This means every
PowerWeChat product submodule has an explicit Roze WeChat boundary and tested
typed wrappers for the core paths.

It does not mean every PowerWeChat public Go method has a one-to-one Rust
wrapper yet. The generic `PlatformClient` can still call uncovered endpoints,
but these areas should be expanded for stricter production parity.

## Snapshot

| Family | PowerWeChat public methods | Roze public async wrappers | Update need |
| --- | ---: | ---: | --- |
| Work | 363 | 144 | high |
| Payment | 165 | 67 | high |
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
   `externalContact`, `oa`, `user`, `message`, and `media` still have the
   largest endpoint surface compared with current Roze wrappers.

2. Payment method-depth parity:
   expand `transfer`, `partner`, `merchantService`, `profitSharing`, `notify`,
   and `order`. These are production-sensitive and should keep strong typed
   request/response DTOs plus signature/decryption tests where applicable.

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

## Documentation Update Needed

Keep `docs/powerwechat-gap-analysis.md` as the submodule-level view, but do not
use it as the final production parity signal. This method-depth audit should be
updated whenever a family reaches one-to-one endpoint coverage or PowerWeChat
adds new methods.
