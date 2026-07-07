# PowerWeChat Gap Analysis

This document compares Roze WeChat against the PowerWeChat source tree at a
submodule level. The current `coverage-matrix.md` tracks the first production
slice of core typed/tested wrappers. It does not mean every PowerWeChat
submodule has a dedicated typed wrapper yet.

The generic `PlatformClient` can still call uncovered endpoints, but these
items are not considered fully aligned until they have explicit module
entries, request/response DTOs, and tests.

## Summary

| Family | PowerWeChat submodules | Explicit Roze typed/tested coverage | Remaining gaps |
| --- | ---: | ---: | ---: |
| Basic Service | 6 | 6 | 0 |
| Channels | 2 | 1 | 1 |
| Mini Program | 32 | 16 | 16 |
| Official Account | 24 | 12 | 13 |
| Open Platform | 6 | 5 | 1 |
| Open Work | 8 | 6 | 2 |
| Payment | 20 | 20 | 0 |
| Work | 33 | 14 | 20 |

Total remaining PowerWeChat submodule gaps: 53.

## Remaining Gaps

### Channels

- `eCommerce/store`

### Mini Program

- `b2b`
- `device`
- `express`
- `image`
- `immediateDelivery`
- `industry`
- `internet`
- `nearbyPoi`
- `operation`
- `plugin`
- `search`
- `server`
- `serviceMarket`
- `soter`
- `virtualPayment`
- `wxa`

### Official Account

- `autoReply`
- `comment`
- `dataCube`
- `device`
- `goods`
- `guide`
- `ocr`
- `poi`
- `publish`
- `semantic`
- `shakeAround`
- `store`
- `wifi`

### Open Platform

- `base`

### Open Work

- `base`
- `user`

### Work

- `accountService`
- `aibot`
- `base`
- `corpgroup`
- `externalPay`
- `idConvert`
- `invoice`
- `menu`
- `miniProgram`
- `oa/approval`
- `oa/calendar`
- `oa/dial`
- `oa/journal`
- `oa/living`
- `oa/meeting`
- `oa/meetingroom`
- `oa/pstncc`
- `oa/schedule`
- `oa/wedoc`
- `oa/wedrive`

## Implementation Priority

1. Mini Program transaction and operational surface:
   `wxa/sec`, `virtualPayment`, `immediateDelivery`, `plugin`,
   `nearbyPoi`, `search`.
2. Work enterprise service surface:
   `accountService`, `externalPay`, `invoice`, `idConvert`, `menu`,
   `oa/approval`, `oa/meeting`, `oa/wedoc`, `oa/wedrive`.
3. Official Account long-tail modules:
   `publish`, `comment`, `dataCube`, `guide`, `semantic`, `shakeAround`,
   `store`, `wifi`, `poi`, `device`, `goods`, `autoReply`, `ocr`.
4. Remaining platform/base wrappers:
   Channels `eCommerce/store`, Open Platform `base`, Open Work `base` and
   `user`.

Each completed submodule should add:

- a public `DomainModule` entry when the boundary is new;
- typed request/response structures for common endpoints;
- unit tests for wire-format serialization/deserialization;
- protocol tests for signing, encryption, callback, or payment behavior when
  security-sensitive;
- an update to this document and `coverage-matrix.md`.
