use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::{errors::IgApiError, state::State};

type HmacSha256 = Hmac<Sha256>;

pub const IG_SIG_KEY_VERSION: &str = "4";

/// Signed body payload required by private API write endpoints.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedPayload {
    pub signed_body: String,
    pub ig_sig_key_version: String,
}

/// Request-level defaults used by the transport layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestContext {
    pub headers: Vec<(String, String)>,
    pub signed_payload: Option<SignedPayload>,
}

/// HTTP request signing, headers, retries, and response parsing.
#[derive(Debug, Default)]
pub struct HttpTransport;

impl HttpTransport {
    pub fn default_headers(state: &State) -> Vec<(String, String)> {
        vec![
            ("x-ig-device-id".to_string(), state.device.uuid.to_string()),
            (
                "x-ig-android-id".to_string(),
                state.device.device_id.to_string(),
            ),
            (
                "x-pigeon-session-id".to_string(),
                state.device.uuid.to_string(),
            ),
            ("x-ads-opt-out".to_string(), "0".to_string()),
            ("x-ig-app-locale".to_string(), "en_US".to_string()),
        ]
    }

    pub fn sign_payload(payload: &str, signing_key: &str) -> Result<SignedPayload, IgApiError> {
        let mut mac = HmacSha256::new_from_slice(signing_key.as_bytes())
            .map_err(|e| IgApiError::Signature(e.to_string()))?;
        mac.update(payload.as_bytes());
        let result = mac.finalize().into_bytes();
        let signature = bytes_to_hex(&result);

        Ok(SignedPayload {
            signed_body: format!("{signature}.{payload}"),
            ig_sig_key_version: IG_SIG_KEY_VERSION.to_string(),
        })
    }

    pub fn should_retry(status_code: u16, attempt: usize, max_retries: usize) -> bool {
        attempt < max_retries && matches!(status_code, 429 | 500..=599)
    }

    pub fn map_status_error(status_code: u16, body: &str) -> Result<(), IgApiError> {
        if status_code >= 400 {
            return Err(IgApiError::HttpResponse {
                status: status_code,
                body: body.to_string(),
            });
        }

        Ok(())
    }
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use crate::state::State;

    use super::HttpTransport;

    #[test]
    fn retry_policy_only_for_retryable_statuses() {
        assert!(HttpTransport::should_retry(500, 0, 3));
        assert!(HttpTransport::should_retry(429, 1, 3));
        assert!(!HttpTransport::should_retry(400, 0, 3));
        assert!(!HttpTransport::should_retry(500, 3, 3));
    }

    #[test]
    fn sign_payload_returns_expected_shape() {
        let signed = HttpTransport::sign_payload("{}", "secret").expect("payload signs");
        assert!(signed.signed_body.ends_with(".{}"));
        assert_eq!(signed.ig_sig_key_version, "4");
    }

    #[test]
    fn default_headers_use_uuid_for_x_ig_device_id() {
        let state = State::from_seed("demo");
        let headers = HttpTransport::default_headers(&state);

        assert!(headers
            .iter()
            .any(|(k, v)| k == "x-ig-device-id" && v == &state.device.uuid));
        assert!(headers
            .iter()
            .any(|(k, v)| k == "x-ig-android-id" && v == &state.device.device_id));
    }
}
