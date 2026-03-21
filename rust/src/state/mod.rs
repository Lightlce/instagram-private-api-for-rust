use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::errors::IgApiError;

/// Deterministic device identity derived from a stable seed (e.g. username).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Device {
    pub phone_id: String,
    pub uuid: String,
    pub device_id: String,
    pub adid: String,
}

/// Serializable session state persisted between authenticated runs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Session {
    pub ds_user_id: Option<String>,
    pub sessionid: Option<String>,
    pub csrftoken: Option<String>,
}

/// Runtime state including deterministic device identity and optional session.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct State {
    pub seed: String,
    pub device: Device,
    pub session: Session,
}

impl State {
    /// Build a stable state from a deterministic seed.
    pub fn from_seed(seed: impl Into<String>) -> Self {
        let seed = seed.into();
        Self {
            device: Device::from_seed(&seed),
            seed,
            session: Session::default(),
        }
    }

    /// Serialize full state for persistence.
    pub fn to_json(&self) -> Result<String, IgApiError> {
        serde_json::to_string(self).map_err(|e| IgApiError::Serialization(e.to_string()))
    }

    /// Restore state from persisted JSON.
    pub fn from_json(raw: &str) -> Result<Self, IgApiError> {
        serde_json::from_str(raw).map_err(|e| IgApiError::Serialization(e.to_string()))
    }
}

impl Device {
    /// Generate deterministic device identifiers.
    pub fn from_seed(seed: &str) -> Self {
        let digest = Sha256::digest(seed.as_bytes());
        let hex = bytes_to_hex(&digest);

        Self {
            phone_id: format_uuid_like(&hex[0..32]),
            uuid: format_uuid_like(&hex[32..64]),
            device_id: format!("android-{}", &hex[0..16]),
            adid: format_uuid_like(&hex[16..48]),
        }
    }
}

fn format_uuid_like(raw32: &str) -> String {
    format!(
        "{}-{}-{}-{}-{}",
        &raw32[0..8],
        &raw32[8..12],
        &raw32[12..16],
        &raw32[16..20],
        &raw32[20..32]
    )
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::State;

    #[test]
    fn deterministic_device_generation() {
        let first = State::from_seed("example_user");
        let second = State::from_seed("example_user");

        assert_eq!(first.device, second.device);
    }

    #[test]
    fn state_round_trip_json() {
        let state = State::from_seed("example_user");
        let serialized = state.to_json().expect("state serializes");
        let restored = State::from_json(&serialized).expect("state deserializes");

        assert_eq!(state, restored);
    }
}
