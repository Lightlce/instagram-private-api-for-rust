use instagram_private_api_rust::http::{HttpTransport, IG_SIG_KEY_VERSION};

#[test]
fn signing_matches_known_hmac_fixture() {
    let signed = HttpTransport::sign_payload("{}", "secret").expect("payload signs");

    assert_eq!(signed.ig_sig_key_version, IG_SIG_KEY_VERSION);
    assert_eq!(
        signed.signed_body,
        "77325902caca812dc259733aacd046b73817372c777b8d95b402647474516e13.{}"
    );
}

#[test]
fn retry_policy_matches_expected_matrix() {
    let matrix = [
        (429, 0, 3, true),
        (500, 1, 3, true),
        (503, 2, 3, true),
        (503, 3, 3, false),
        (400, 0, 3, false),
    ];

    for (status, attempt, max_retries, expected) in matrix {
        assert_eq!(
            HttpTransport::should_retry(status, attempt, max_retries),
            expected,
            "status={status}, attempt={attempt}, max_retries={max_retries}"
        );
    }
}
