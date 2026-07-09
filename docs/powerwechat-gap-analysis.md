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
| Channels | 2 | 2 | 0 |
| Mini Program | 32 | 32 | 0 |
| Official Account | 24 | 15 | 10 |
| Open Platform | 6 | 6 | 0 |
| Open Work | 8 | 8 | 0 |
| Payment | 20 | 20 | 0 |
| Work | 33 | 27 | 7 |

Total remaining PowerWeChat submodule gaps: 17.

## Remaining Gaps

### Official Account

- `autoReply`
- `device`
- `goods`
- `guide`
- `ocr`
- `poi`
- `semantic`
- `shakeAround`
- `store`
- `wifi`

### Work

- `accountService`
- `aibot`
- `oa/living`
- `oa/meeting`
- `oa/meetingroom`
- `oa/wedoc`
- `oa/wedrive`

## Implementation Priority

1. Work enterprise service surface:
   `accountService`, `oa/approval`, `oa/meeting`, `oa/wedoc`, `oa/wedrive`.
2. Official Account long-tail modules:
   `guide`, `semantic`, `shakeAround`, `store`, `wifi`, `poi`, `device`,
   `goods`, `autoReply`, `ocr`.
Each completed submodule should add:

- a public `DomainModule` entry when the boundary is new;
- typed request/response structures for common endpoints;
- unit tests for wire-format serialization/deserialization;
- protocol tests for signing, encryption, callback, or payment behavior when
  security-sensitive;
- an update to this document and `coverage-matrix.md`.
