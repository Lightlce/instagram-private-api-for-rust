use instagram_private_api_rust::state::State;

#[test]
fn deterministic_device_fixture_matches_expected_values() {
    let state = State::from_seed("example_user");

    assert_eq!(state.device.phone_id, "8f70705e-25ba-a00c-2db8-a925d48e779a");
    assert_eq!(state.device.uuid, "e9b87a55-77b9-0d53-39c3-fef0111e1f40");
    assert_eq!(state.device.device_id, "android-8f70705e25baa00c");
    assert_eq!(state.device.adid, "2db8a925-d48e-779a-e9b8-7a5577b90d53");
}

#[test]
fn session_roundtrip_fixture_is_stable() {
    let mut state = State::from_seed("fixture");
    state.session.ds_user_id = Some("12345".into());
    state.session.sessionid = Some("session-token".into());
    state.session.csrftoken = Some("csrf-token".into());

    let serialized = state.to_json().expect("state serializes");
    let restored = State::from_json(&serialized).expect("state deserializes");

    assert_eq!(restored.session.ds_user_id.as_deref(), Some("12345"));
    assert_eq!(restored.session.sessionid.as_deref(), Some("session-token"));
    assert_eq!(restored.session.csrftoken.as_deref(), Some("csrf-token"));
    assert_eq!(restored, state);
}
