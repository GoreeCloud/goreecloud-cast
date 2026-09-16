use goreecloud_cast_core::{
    auth::{SessionCredential, SessionCredentialBinding},
    credentials::{CredentialLease, InMemoryCredentialLeaseStore},
    crypto::{Ed25519DeviceSigner, Ed25519DeviceVerifier},
    endpoints::{ControllerEndpoint, EndpointError, ReceiverEndpoint},
    identity::{DeviceId, TrustLevel},
    local_transport::{
        InMemoryLocalLink, LocalFrameReceiver, LocalFrameTransmitter, LocalTransportError,
    },
    protocol::{MessageKind, ProtocolFrame},
    session::SessionState,
    trust::{
        DevelopmentTrustPersistence, InMemoryTrustStore, PersistentTrustStore, TrustRecord,
    },
};

fn controller() -> DeviceId {
    DeviceId::parse("device:controller:0001").unwrap()
}

fn receiver() -> DeviceId {
    DeviceId::parse("device:receiver:0001").unwrap()
}

fn security_material() -> (
    Ed25519DeviceSigner,
    Ed25519DeviceSigner,
    Ed25519DeviceVerifier,
) {
    let controller_signer = Ed25519DeviceSigner::from_secret_bytes(controller(), [7; 32]);
    let receiver_signer = Ed25519DeviceSigner::from_secret_bytes(receiver(), [9; 32]);
    let mut verifier = Ed25519DeviceVerifier::new();
    verifier
        .register(controller(), controller_signer.verifying_key_bytes())
        .unwrap();
    verifier
        .register(receiver(), receiver_signer.verifying_key_bytes())
        .unwrap();
    (controller_signer, receiver_signer, verifier)
}

fn trusted_store() -> InMemoryTrustStore {
    let mut store = InMemoryTrustStore::new();
    store
        .upsert(
            TrustRecord::new(controller(), TrustLevel::ApprovedDevice, 1, 100).unwrap(),
        )
        .unwrap();
    store
}

fn credential_and_store() -> (SessionCredential, InMemoryCredentialLeaseStore) {
    let binding =
        SessionCredentialBinding::new("session:000000000001", controller(), receiver()).unwrap();
    let credential = SessionCredential::new("credential-handle-0001", binding).unwrap();
    let lease = CredentialLease::new(credential.clone(), 100, 10_000).unwrap();
    let mut store = InMemoryCredentialLeaseStore::new();
    store.register(lease).unwrap();
    (credential, store)
}

#[test]
fn authenticated_local_transport_rejects_tamper_and_replay_without_advancing_state() {
    let (controller_signer, _, verifier) = security_material();
    let mut transmitter = LocalFrameTransmitter::new(controller(), &controller_signer);
    let mut receiver_transport = LocalFrameReceiver::new(receiver(), &verifier);
    let frame = ProtocolFrame::new(MessageKind::Command, 7, b"play".to_vec()).unwrap();
    let packet = transmitter.seal(&receiver(), &frame).unwrap();

    let mut tampered = packet.clone();
    let last = tampered.len() - 1;
    tampered[last] ^= 0x01;
    assert_eq!(
        receiver_transport.open(&tampered),
        Err(LocalTransportError::AuthenticationRejected)
    );

    let opened = receiver_transport.open(&packet).unwrap();
    assert_eq!(opened.sender, controller());
    assert_eq!(opened.frame, frame);
    assert_eq!(
        receiver_transport.open(&packet),
        Err(LocalTransportError::Replay)
    );
    assert_eq!(
        receiver_transport.open(b"not-a-cast-packet"),
        Err(LocalTransportError::MalformedPacket)
    );
}

#[test]
fn stale_active_snapshot_cannot_resurrect_revoked_trust() {
    let mut store = trusted_store();
    let mut active_backup = DevelopmentTrustPersistence::new();
    active_backup.persist(&store).unwrap();

    store.revoke(&controller(), 500, 2).unwrap();
    let mut revoked_backup = DevelopmentTrustPersistence::new();
    revoked_backup.persist(&store).unwrap();

    let mut restored = InMemoryTrustStore::new();
    revoked_backup.restore_into(&mut restored).unwrap();
    active_backup.restore_into(&mut restored).unwrap();

    let record = restored.lookup(&controller()).unwrap();
    assert!(!record.is_active());
    assert_eq!(record.revoked_at_ms, Some(500));
}

#[test]
fn separate_endpoints_establish_session_and_restart_revalidates_authority() {
    let (controller_signer, receiver_signer, verifier) = security_material();
    let trust = trusted_store();
    let (credential, credentials) = credential_and_store();
    let mut persistence = DevelopmentTrustPersistence::new();
    persistence.persist(&trust).unwrap();

    let mut link = InMemoryLocalLink::new();
    let mut controller_endpoint =
        ControllerEndpoint::new(controller(), &controller_signer, &verifier);
    let mut receiver_endpoint = ReceiverEndpoint::new(
        receiver(),
        &receiver_signer,
        &verifier,
        &trust,
        &credentials,
    );

    controller_endpoint
        .send_session_open(&mut link, &receiver(), &credential, 11)
        .unwrap();
    let session = receiver_endpoint.process_next(&mut link, 1_000).unwrap().unwrap();
    assert_eq!(session.state, SessionState::Active);
    assert_eq!(
        controller_endpoint.receive_session_accepted(&mut link).unwrap(),
        Some("session:000000000001".to_string())
    );

    drop(receiver_endpoint);
    drop(controller_endpoint);

    let mut restored_trust = InMemoryTrustStore::new();
    persistence.restore_into(&mut restored_trust).unwrap();
    let restored_credentials = credentials.clone();
    let mut restarted_link = InMemoryLocalLink::new();
    let mut restarted_controller =
        ControllerEndpoint::new(controller(), &controller_signer, &verifier);
    let mut restarted_receiver = ReceiverEndpoint::new(
        receiver(),
        &receiver_signer,
        &verifier,
        &restored_trust,
        &restored_credentials,
    );

    restarted_controller
        .send_session_open(&mut restarted_link, &receiver(), &credential, 12)
        .unwrap();
    let recovered = restarted_receiver
        .process_next(&mut restarted_link, 2_000)
        .unwrap()
        .unwrap();
    assert_eq!(recovered.state, SessionState::Active);
}

#[test]
fn receiver_endpoint_fails_closed_after_trust_or_credential_revocation() {
    let (controller_signer, receiver_signer, verifier) = security_material();
    let (credential, credentials) = credential_and_store();

    let mut revoked_trust = trusted_store();
    revoked_trust.revoke(&controller(), 500, 2).unwrap();
    let mut link = InMemoryLocalLink::new();
    let mut controller_endpoint =
        ControllerEndpoint::new(controller(), &controller_signer, &verifier);
    let mut receiver_endpoint = ReceiverEndpoint::new(
        receiver(),
        &receiver_signer,
        &verifier,
        &revoked_trust,
        &credentials,
    );
    controller_endpoint
        .send_session_open(&mut link, &receiver(), &credential, 21)
        .unwrap();
    assert_eq!(
        receiver_endpoint.process_next(&mut link, 1_000),
        Err(EndpointError::UntrustedController)
    );

    let trusted = trusted_store();
    let mut revoked_credentials = credentials.clone();
    revoked_credentials.revoke(&credential.handle, 700).unwrap();
    let mut link = InMemoryLocalLink::new();
    let mut controller_endpoint =
        ControllerEndpoint::new(controller(), &controller_signer, &verifier);
    let mut receiver_endpoint = ReceiverEndpoint::new(
        receiver(),
        &receiver_signer,
        &verifier,
        &trusted,
        &revoked_credentials,
    );
    controller_endpoint
        .send_session_open(&mut link, &receiver(), &credential, 22)
        .unwrap();
    assert_eq!(
        receiver_endpoint.process_next(&mut link, 1_000),
        Err(EndpointError::CredentialRejected)
    );
}

#[test]
fn malformed_interrupted_endpoint_input_fails_closed() {
    let (_, receiver_signer, verifier) = security_material();
    let trust = trusted_store();
    let (_, credentials) = credential_and_store();
    let mut link = InMemoryLocalLink::new();
    link.send_raw(receiver(), b"partial-frame".to_vec());
    let mut receiver_endpoint = ReceiverEndpoint::new(
        receiver(),
        &receiver_signer,
        &verifier,
        &trust,
        &credentials,
    );

    assert_eq!(
        receiver_endpoint.process_next(&mut link, 1_000),
        Err(EndpointError::TransportFailure)
    );
}
