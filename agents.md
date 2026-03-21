# Agent North Star: Instagram Private API TypeScript → Rust Migration

## 1) Description

This repository currently contains a TypeScript implementation of an Instagram private API client. The mission is to migrate it to Rust while preserving feature parity across client behavior, repositories, feeds, services, request semantics, error mapping, and state/session serialization.

The migration should prioritize correctness and compatibility first, then ergonomics and optimization. Architecture parity with the existing implementation is the default strategy.

## 2) Technical Architecture

### Current TypeScript surface (to preserve)

- Core runtime: client, state, request, repository/feed base classes, feed factory.
- Repositories: low-level one-request operations (account, media, upload, direct, live, etc.).
- Feeds: paginated endpoint abstractions with retry + iterator semantics.
- Services: multi-step workflows (publish/simulate/search/story/insights).
- Response and type models: broad typed contracts for Instagram API payloads.
- Error hierarchy: typed error mapping from response semantics.

### Target Rust architecture

- `src/client/`: high-level API client (`IgApiClient` equivalent).
- `src/state/`: session/auth/device/cookie lifecycle and persistence.
- `src/http/`: request signing, default mobile headers, retries, response parsing.
- `src/repositories/`: endpoint modules with one-request-per-method convention.
- `src/feeds/`: paginated feed abstractions + serializable pagination state.
- `src/services/`: orchestration flows (publish, simulate, story, search, insights).
- `src/models/responses/`: `serde` response models.
- `src/models/types/`: request/domain types.
- `src/errors/`: typed client/network/protocol/domain errors.

### Core migration principles

- Preserve protocol fidelity:
  - signature generation (`ig_sig_key_version`, HMAC payload signing),
  - header semantics,
  - cookie/state behavior,
  - error mapping behavior.
- Port in dependency tiers (core first, then repositories/feeds/services).
- Validate parity with explicit endpoint/method checklists and tests.

## 3) List of Open Tasks

- [x] ✅ Initialize Rust crate/workspace mirroring TypeScript module boundaries.
- [x] ✅ Implement Rust `State` with deterministic device generation and session serialization.
- [x] ✅ Implement request layer with signing, headers, retries, and typed error mapping.
- [ ] Build model generation workflow from TS response/type declarations.
- [ ] Port Tier 1 repositories (auth/session/content essentials).
- [ ] Port feed framework and feed factory equivalents.
- [ ] Port services (`publish`, `simulate`, `story`, `search`, `insights`).
- [ ] Port advanced repositories (live, music, igtv, highlights, ads, etc.).
- [ ] Create parity matrix and test harness against TS behavior.
- [ ] Add Rust examples equivalent to current TypeScript examples.
- [ ] Stage rollout with dual-run validation before TS deprecation.

## 4) File Structure

```text
.
├─ agents.md
├─ .agents/
│  ├─ memories/
│  │  ├─ context/
│  │  │  └─ context-YYYYmmDDHHMMss.md
│  │  └─ plans/
│  │     └─ name-of-plan-YYYYmmDDHHMMss.md
│  └─ qna.md
├─ src/
│  ├─ client/
│  ├─ state/
│  ├─ http/
│  ├─ repositories/
│  ├─ feeds/
│  ├─ services/
│  ├─ models/
│  │  ├─ responses/
│  │  └─ types/
│  └─ errors/
└─ ...
```

## 5) Agent Instructions

- Realign to `agents.md` to set your true north.
- Add a context summary in `.agents/memories/context` after each prompt to avoid context drift. `agents.md` takes precedence over memories and sets the true north. Use filenames like `context-YYYYmmDDHHMMss.md`.
- Document plans in `.agents/memories/plans` using names like `name-of-plan-YYYYmmDDHHMMss.md`.
- Always follow secure security practices and keep any sensitive information away from this directory, including untracked files in git.
- Commit changes following conventional commit rules after each sensible change.
  - Before committing: run linters/formatters, standardize code, and update progress on completed items using checkmark emojis.
  - Make best effort to keep the git worktree clean.
- Proceed with any non-blocking tasks.
  - For blocking tasks, add questions in `.agents/qna.md` using a reddit-style thread per question and include status so answers can be provided asynchronously.
  - After posting a blocking question, continue with the next non-blocking task.
- Check `./agents/qna.md` for updates and follow up on them.

## 6) Agent Forbids

- Never delete files.

## 7) Repository Origin

- Upstream reference: `https://github.com/dilame/instagram-private-api`
- Current repository package identity: `instagram-private-api` (TypeScript origin, migration target Rust).
