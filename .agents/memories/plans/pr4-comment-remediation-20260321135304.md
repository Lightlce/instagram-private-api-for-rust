# Plan: PR #4 comment remediation

1. Address HTTP header parity review comment (`X-IG-Device-ID`/`X-IG-Android-ID`).
2. Refactor client/repository wiring to share mutable runtime state.
3. Update tests to validate header semantics and shared state wiring.
4. Move/duplicate Q&A log into `.agents/qna.md` and update agent guidance pointer.
5. Run Rust tests, commit, push to PR branch, and reply on review threads.
