use goreecloud_cast_core::{
    auth::{
        AdapterError, AuthenticationChallenge, DeviceIdentitySigner, DeviceIdentityVerifier,
        IdentityProof, PairingAuthorizer, PairingGrant, PairingRequest, SessionCredential,
        SessionCredentialAuthority, SessionCredentialBinding,
    },
    command::{CommandDisposition, CommandEnvelope, CommandSequencer},
    discovery::{AuthenticatedDiscoveryDetails, DeviceCategory},
    handshake::{HandshakeError, HandshakeRequest, InMemoryHandshakeHarness},
    identity::{DeviceId, TrustLevel},
    protocol::{
        decode_authenticated_discovery_details, encode_authenticated_discovery_details, MessageKind,
        ProtocolFrame,
    },
    session::SessionState,
};

#[derive(Debug)]
struct TestIdentity;

impl DeviceIdentitySigner for TestIdentity {
    fn sign_challenge(
        &self,
        device_id: &DeviceId,
        challenge: &AuthenticationChallenge,
    ) -> Result<IdentityProof, AdapterError> {
        let mut proof = b"development-test-proof:".to_vec();
        proof.extend_from_slice(&challenge.nonce);
        IdentityProof::new(device_id.clone(), proof)
    }
}

impl DeviceIdentityVerifier for TestIdentity {
    fn verify_challenge(
        &self,
        challenge: &AuthenticationChallenge,
        proof: &IdentityProof,
    ) -> Result<bool, AdapterError> {
        let mut expected = b"development-test-proof:".to_vec();
        expected.extend_from_slice(&challenge.nonce);
        Ok(proof.proof == expected)
    }
}

#[derive(Debug)]
struct RejectingVerifier;

impl DeviceIdentityVerifier for RejectingVerifier {
    fn verify_challenge(
        &self,
        _challenge: &AuthenticationChallenge,
        _proof: &IdentityProof,
    ) -> Result<bool, AdapterError> {
        Ok(false)
    }
}

#[derive(Debug)]
struct TestPairing;

impl PairingAuthorizer for TestPairing {
    fn authorize_pairing(&self, request: &PairingRequest) -> Result<PairingGrant, AdapterError> {
        PairingGrant::new(
            "pairing-grant-0001",
            request.controller.clone(),
            request.receiver.clone(),
            TrustLevel::ApprovedDevice,
        )
    }
}

#[derive(Debug)]
struct TestCredentials;

impl SessionCredentialAuthority for TestCredentials {
    fn issue(
        &self,
        binding: &SessionCredentialBinding,
    ) -> Result<SessionCredential, AdapterError> {
        SessionCredential::new("session-credential-0001", binding.clone())
    }

    fn validate(&self, credential: &SessionCredential) -> Result<bool, AdapterError> {
        Ok(credential.handle == "session-credential-0001")
    }

    fn revoke(&self, _credential: &SessionCredential) -> Result<(), AdapterError> {
        Ok(())
    }
}

fn controller() -> DeviceId {
    DeviceId::parse("device:controller:0001").unwrap()
}

fn receiver() -> DeviceId {
    DeviceId::parse("device:receiver:0001").unwrap()
}

fn receiver_details() -> AuthenticatedDiscoveryDetails {
    AuthenticatedDiscoveryDetails::new(
        "rotating-id-0123456789",
        receiver(),
        "Living Room Display",
        DeviceCategory::Display,
        0xA11CE,
        true,
    )
    .unwrap()
}

fn handshake_request() -> HandshakeRequest {
    HandshakeRequest::new(
        controller(),
        receiver(),
        "session:000000000001",
        "rotating-id-0123456789",
        [7; 32],
    )
    .unwrap()
}

#[test]
fn in_memory_handshake_reaches_active_only_after_identity_pairing_and_credential_checks() {
    let identity = TestIdentity;
    let pairing = TestPairing;
    let credentials = TestCredentials;
    let harness = InMemoryHandshakeHarness::new(
        &identity,
        &identity,
        &pairing,
        &credentials,
    );

    let outcome = harness
        .establish(&handshake_request(), &receiver_details())
        .unwrap();

    assert_eq!(outcome.session.state, SessionState::Active);
    assert_eq!(outcome.pairing_grant.trust_level, TrustLevel::ApprovedDevice);
    assert_eq!(outcome.receiver_details.friendly_name, "Living Room Display");
    assert_eq!(outcome.transcript.len(), 4);
    assert_eq!(outcome.transcript[0].kind, MessageKind::DiscoveryDetailsRequest);
    assert_eq!(outcome.transcript[1].kind, MessageKind::DiscoveryDetailsResponse);
    assert_eq!(outcome.transcript[2].kind, MessageKind::SessionOpen);
    assert_eq!(outcome.transcript[3].kind, MessageKind::SessionAccepted);
}

#[test]
fn authenticated_discovery_details_are_not_released_when_identity_verification_fails() {
    let identity = TestIdentity;
    let verifier = RejectingVerifier;
    let pairing = TestPairing;
    let credentials = TestCredentials;
    let harness = InMemoryHandshakeHarness::new(
        &identity,
        &verifier,
        &pairing,
        &credentials,
    );

    assert_eq!(
        harness.establish(&handshake_request(), &receiver_details()),
        Err(HandshakeError::IdentityRejected)
    );
}

#[test]
fn protocol_frames_and_authenticated_discovery_payloads_round_trip_deterministically() {
    let details = receiver_details();
    let payload = encode_authenticated_discovery_details(&details).unwrap();
    let frame = ProtocolFrame::new(MessageKind::DiscoveryDetailsResponse, 17, payload).unwrap();

    let first = frame.encode();
    let second = frame.encode();
    assert_eq!(first, second);

    let decoded_frame = ProtocolFrame::decode(&first).unwrap();
    assert_eq!(decoded_frame, frame);
    assert_eq!(
        decode_authenticated_discovery_details(&decoded_frame.payload).unwrap(),
        details
    );
}

#[test]
fn command_sequencer_rejects_out_of_order_and_conflicting_replays() {
    let session_id = "session:000000000001";
    let mut sequencer = CommandSequencer::new(session_id).unwrap();
    let first = CommandEnvelope::new(session_id, 1, "command-0001", 10, b"play".to_vec()).unwrap();

    assert_eq!(sequencer.accept(&first), CommandDisposition::Applied);
    assert_eq!(sequencer.accept(&first), CommandDisposition::Duplicate);

    let conflict =
        CommandEnvelope::new(session_id, 1, "command-0001", 10, b"pause".to_vec()).unwrap();
    assert_eq!(
        sequencer.accept(&conflict),
        CommandDisposition::ConflictingReplay
    );

    let out_of_order =
        CommandEnvelope::new(session_id, 3, "command-0003", 10, b"seek".to_vec()).unwrap();
    assert_eq!(
        sequencer.accept(&out_of_order),
        CommandDisposition::OutOfOrder {
            expected: 2,
            received: 3,
        }
    );

    let second =
        CommandEnvelope::new(session_id, 2, "command-0002", 10, b"pause".to_vec()).unwrap();
    assert_eq!(sequencer.accept(&second), CommandDisposition::Applied);
    assert_eq!(sequencer.next_sequence(), Some(3));
}
