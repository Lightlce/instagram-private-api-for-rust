# Endpoint Parity Checklist (TypeScript -> Rust)

_Date: 2026-03-21_

## Repositories

- [x] account.repository.ts -> `rust/src/repositories/mod.rs` (`AccountRepository`)
- [x] media.repository.ts -> `rust/src/repositories/mod.rs` (`MediaRepository`)
- [x] user.repository.ts -> `rust/src/repositories/mod.rs` (`UserRepository`)
- [x] friendship.repository.ts -> `rust/src/repositories/mod.rs` (`FriendshipRepository`)
- [x] direct.repository.ts -> `rust/src/repositories/mod.rs` (`DirectRepository`)
- [x] direct-thread.repository.ts -> `rust/src/repositories/mod.rs` (`DirectThreadRepository`)
- [x] discover.repository.ts -> `rust/src/repositories/mod.rs` (`DiscoverRepository`)
- [x] tag.repository.ts -> `rust/src/repositories/mod.rs` (`TagRepository`)
- [x] location.repository.ts + location-search.repository.ts -> `rust/src/repositories/mod.rs` (`LocationRepository`)
- [x] upload.repository.ts -> `rust/src/repositories/mod.rs` (`UploadRepository`)
- [x] status.repository.ts -> `rust/src/repositories/mod.rs` (`StatusRepository`)
- [x] live.repository.ts -> `rust/src/repositories/mod.rs` (`LiveRepository`)
- [x] music.repository.ts -> `rust/src/repositories/mod.rs` (`MusicRepository`)
- [x] igtv.repository.ts -> `rust/src/repositories/mod.rs` (`IgtvRepository`)
- [x] highlights.repository.ts -> `rust/src/repositories/mod.rs` (`HighlightsRepository`)
- [x] ads.repository.ts -> `rust/src/repositories/mod.rs` (`AdsRepository`)
- [x] qe.repository.ts -> `rust/src/repositories/mod.rs` (`SessionRepository` compatibility role)
- [ ] Remaining repository modules in `src/repositories/*.ts` (14 files) are still unmapped to dedicated Rust modules.

## Feeds

- [x] Base feed pagination abstraction exists in `rust/src/feeds/mod.rs`.
- [ ] Concrete feed ports for `src/feeds/*.ts` (35 feed modules) are not yet implemented as endpoint-backed Rust feed structs.

## Services

- [x] Service registry and modules exist: publish/simulate/story/search/insights.
- [ ] Current services mostly provide bootstrap endpoint sequences; full workflow parity is still pending.

## Models

- [x] Generated manifest tracks TypeScript `src/responses/*.ts` and `src/types/*.ts` module names.
- [ ] Concrete generated Rust structs/enums per model module are still pending.

## Validation

- [x] Manifest/file-system parity assertions added in Rust tests.
- [x] Dual-run Rust + TypeScript validation script executes successfully (`bash rust/scripts/dual_run_validation.sh`).

## Endpoint Inventory

- [x] Canonical TypeScript endpoint inventory captured in `rust/parity/typescript-endpoints.txt` (182 unique URL templates extracted from `src/repositories`, `src/feeds`, and `src/services`).
- [x] TypeScript parity smoke tests assert the inventory stays in sync with source endpoint declarations (`tests/run.js`).
