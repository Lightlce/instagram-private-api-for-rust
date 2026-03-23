# Q&A Thread Log

## [closed] PR creation permissions are blocked

**u/codex-agent**
I can push branches to `origin`, but direct GitHub PR creation via `gh pr create` fails with:
`Resource not accessible by personal access token (createPullRequest)`.

This was blocking fully automated PR creation on GitHub from this environment.

**Resolution:** Use `gh pr create` for PR creation instead of the `make_pr` MCP workflow.
**Next action:** Continue non-blocking migration tasks and open PRs with `gh pr create`.

## [open] Prioritization needed for remaining migration parity backlog

**u/codex-agent**
Dual-run validation is now unblocked and passing, and parity-validation plan is complete. The remaining open plan (`migration-parity-gap-plan-20260321151308.md`) still has major scope (repositories, feeds, services, model structs, and error taxonomy).

To execute efficiently, please choose the next highest-priority implementation track:

1. Repository module expansion (largest endpoint coverage gain)
2. Feed concrete implementations
3. Generated response/type struct materialization
4. Error taxonomy + mapping parity

**Status:** waiting for prioritization decision.
**Next action while waiting:** proceed with repository expansion by default if no preference is provided.

**u/codex-agent (follow-up 2026-03-23)**
Proceeding with the default path while waiting for prioritization input: repository expansion continued by adding Rust endpoint primitives for `user`, `friendship`, `direct`, `direct-thread`, `discover`, `tag`, `location`, `upload`, and `status`.
