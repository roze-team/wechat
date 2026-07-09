# PowerWeChat Gap Analysis

This document compares Roze WeChat against the PowerWeChat source tree at a
submodule level. The current `coverage-matrix.md` tracks the first production
slice of core typed/tested wrappers. It does not mean every PowerWeChat
submodule has a dedicated typed wrapper yet.

The generic `PlatformClient` can still call uncovered endpoints, but these
items are not considered fully aligned until they have explicit module
entries, request/response DTOs, and tests.

For method-level depth beyond submodule coverage, see
`docs/powerwechat-method-depth-audit.md`.

## Summary

| Family | PowerWeChat submodules | Explicit Roze typed/tested coverage | Remaining gaps |
| --- | ---: | ---: | ---: |
| Basic Service | 6 | 6 | 0 |
| Channels | 2 | 2 | 0 |
| Mini Program | 32 | 32 | 0 |
| Official Account | 24 | 24 | 0 |
| Open Platform | 6 | 6 | 0 |
| Open Work | 8 | 8 | 0 |
| Payment | 20 | 20 | 0 |
| Work | 33 | 33 | 0 |

Total remaining PowerWeChat submodule gaps: 0.

## Remaining Gaps

No remaining submodule-level PowerWeChat gaps are currently tracked.

## Implementation Priority

1. Keep submodule parity intact as PowerWeChat and WeChat APIs evolve.
Each completed submodule should add:

- a public `DomainModule` entry when the boundary is new;
- typed request/response structures for common endpoints;
- unit tests for wire-format serialization/deserialization;
- protocol tests for signing, encryption, callback, or payment behavior when
  security-sensitive;
- an update to this document and `coverage-matrix.md`.
