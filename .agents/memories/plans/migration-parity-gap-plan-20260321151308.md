# Plan: migration parity gap closure (2026-03-21)

## Objective

Run a practical parity check between the TypeScript implementation and the Rust migration, then define the missing work needed to reach endpoint/behavior parity.

## Parity Check Snapshot

### Surface coverage (file-count proxy)

- TypeScript repositories: **31** files (`src/repositories/*.ts`)
- Rust repositories: **1** file (`rust/src/repositories/mod.rs`)
- TypeScript feeds: **35** files (`src/feeds/*.ts`)
- Rust feeds: **1** file (`rust/src/feeds/mod.rs`)
- TypeScript services: **5** files (`src/services/*.ts`)
- Rust services: **1** file (`rust/src/services/mod.rs`)
- TypeScript errors: **25** files (`src/errors/*.ts`)
- Rust errors: **1** file (`rust/src/errors/mod.rs`)
- TypeScript responses/types declarations: **106/31** files
- Rust response/type model modules: manifest present, but runtime structs are still placeholders in `rust/src/models/responses/mod.rs` and `rust/src/models/types/mod.rs`.

### Validation status

- Rust unit + parity harness tests pass.
- Dual-run TypeScript parity validation is blocked because `mocha` is missing in the environment.

## Missing Parity Work (Execution Plan)

1. [ ] **Repository parity expansion (P0)**

   - Port remaining repository modules from `src/repositories/*.ts` into dedicated Rust modules.
   - Replace endpoint-only stubs with typed request/response methods.
   - Exit criteria: each TypeScript repository has a Rust counterpart with method-level mapping.

2. [ ] **Feed parity implementation (P0)**

   - Port concrete feed implementations from `src/feeds/*.ts` (cursor handling, pagination flags, request params).
   - Keep generic feed trait, but wire real endpoint-backed feed structs.
   - Exit criteria: all tracked TS feeds mapped and integration-tested.

3. [ ] **Service workflow parity (P0)**

   - Upgrade Rust services from bootstrap endpoint lists to full orchestration logic matching TS `publish/simulate/story/search/insights` flows.
   - Exit criteria: service methods perform equivalent multi-step behavior and payload composition.

4. [ ] **Model parity hardening (P0)**

   - Generate/commit concrete `serde` structs/enums per response/type module instead of manifest-only placeholders.
   - Exit criteria: models compile and deserialize golden fixtures for representative endpoints.

5. [ ] **Error taxonomy parity (P1)**

   - Port TS error hierarchy semantics into Rust error types + mapping rules.
   - Exit criteria: error mapping tests for auth/checkpoint/rate-limit/challenge/network cases.

6. [ ] **Core runtime parity completion (P1)**

   - Fill gaps for request semantics (headers/signing/cookies/device/session nuances), feed factory/entity factory equivalents, and retry policy parity.
   - Exit criteria: fixture-backed parity tests against TS request snapshots.

7. [ ] **Parity matrix and harness strengthening (P1)**

   - Upgrade `rust/parity/matrix.json` from path existence to method-level parity inventory.
   - Extend harness assertions beyond file existence (method presence, endpoint signatures, behavior fixtures).
   - Exit criteria: CI fails on any uncovered endpoint/method in tracked scope.

8. [x] ✅ **Dual-run validation enablement (P1 blocker removal)**

   - Install/lock TypeScript test dependencies (including `mocha`) in CI/runtime image.
   - Run Rust vs TypeScript dual-run comparisons and capture diffs.
   - Exit criteria: reproducible dual-run report with pass/fail and deltas.

9. [ ] **Rollout readiness report (P2)**
   - Publish final parity report summarizing completed coverage, remaining exceptions, and go/no-go recommendation.
   - Exit criteria: report links method-level matrix, test artifacts, and known risks.

## Progress Tracking

- **Status:** In progress
- **Completed:** 1 / 9
- **Current blocker:** None (dual-run script now executes across Rust and TypeScript checks).
