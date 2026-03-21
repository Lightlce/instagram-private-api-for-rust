# Plan: state and http foundation

1. Continue with non-blocking migration tasks after workspace bootstrap.
2. Implement deterministic `State` generation and JSON session serialization.
3. Implement request-layer foundations: payload signing, default headers, retry policy, and status error mapping.
4. Add unit tests for deterministic behavior, serialization round-trips, and retry/signing logic.
5. Run formatting/checks/tests, commit, and prepare PR metadata.
