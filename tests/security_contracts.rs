use goreecloud_cast_core::{
    auth::{
        AuthenticationChallenge, DeviceIdentitySigner, DeviceIdentityVerifier, PairingGrant,
        PairingRequest, SessionCredential, SessionCredentialBinding,
    },
    credentials::{
        CredentialLease, CredentialLeaseStatus, InMemoryCredentialLeaseStore,
    },
    crypto::{Ed25519DeviceSigner, Ed25519DeviceVerifier},
    identity::{DeviceId, TrustLevel},
    pairing::{PairingConfirmation, PairingFlowError, PairingState},
};

fn controller() -> DeviceId {
    DeviceId::parse("device:controller:0001").unwrap()
}

fn receiver() -> DeviceId {
    DeviceId::parse("device:receiver:0001").unwrap()
}

#[test]
fn ed25519_device_identity_signs_and_strictly_verifies_bound_challenges() {
    let signer = Ed25519DeviceSigner::from_secret_bytes(controller(), [7; 32]);
    let mut verifier = Ed25519DeviceVerifier::new();
    verifier
        .register(controller(), signer.verifying_key_bytes())
        .unwrap();

    let challenge = AuthenticationChallenge::new([9; 32], "cast-pairing").unwrap();
    let proof = signer.sign_challenge(&controller(), &challenge).unwrap();

    assert!(verifier.verify_challenge(&challenge, &proof).unwrap());

    let tampered = AuthenticationChallenge::new([8; 32], "cast-pairing").unwrap();
    assert!(!verifier.verify_challenge(&tampered, &proof).unwrap());
}

#[test]
fn ed25519_verifier_fails_closed_for_unregistered_devices() {
    let signer = Ed25519DeviceSigner::from_secret_bytes(controller(), [3; 32]);
    let verifier = Ed25519DeviceVerifier::new();
    let challenge = AuthenticationChallenge::new([4; 32], "cast-session").unwrap();
    let proof = signer.sign_challenge(&controller(), &challenge).unwrap();

    assert!(!verifier.verify_challenge(&challenge, &proof).unwrap());
}

#[test]
fn pairing_confirmation_expires_and_cannot_be_resurrected() {
    let request = PairingRequest {
        controller: controller(),
        receiver: receiver(),
    };
    let mut confirmation = PairingConfirmation::new(request.clone(), 1_000, 2_000).unwrap();
    assert_eq!(confirmation.state(1_500), PairingState::AwaitingConfirmation);
    assert_eq!(confirmation.state(2_000), PairingState::Expired);

    let grant = PairingGrant::new(
        "pairing-grant-0001",
        request.controller,
        request.receiver,
        TrustLevel::ApprovedDevice,
    )
    .unwrap();
    assert_eq!(
        confirmation.confirm(2_001, grant),
        Err(PairingFlowError::Expired)
    );
}

#[test]
fn pairing_confirmation_rejects_mismatched_grants() {
    let request = PairingRequest {
        controller: controller(),
        receiver: receiver(),
    };
    let mut confirmation = PairingConfirmation::new(request, 10, 1_000).unwrap();
    let wrong_receiver = DeviceId::parse("device:receiver:9999").unwrap();
    let grant = PairingGrant::new(
        "pairing-grant-0002",
        controller(),
        wrong_receiver,
        TrustLevel::ApprovedDevice,
    )
    .unwrap();

    assert_eq!(
        confirmation.confirm(100, grant),
        Err(PairingFlowError::GrantBindingMismatch)
    );
    assert_eq!(confirmation.state(100), PairingState::AwaitingConfirmation);
}

#[test]
fn credential_leases_expire_revoke_and_reject_binding_substitution() {
    let binding = SessionCredentialBinding::new(
        "session:000000000001",
        controller(),
        receiver(),
    )
    .unwrap();
    let credential = SessionCredential::new("credential-handle-0001", binding.clone()).unwrap();
    let lease = CredentialLease::new(credential.clone(), 100, 200).unwrap();
    let mut store = InMemoryCredentialLeaseStore::new();
    store.register(lease).unwrap();

    assert_eq!(store.validate(&credential, 150), CredentialLeaseStatus::Valid);
    assert_eq!(store.validate(&credential, 200), CredentialLeaseStatus::Expired);

    store.revoke(&credential.handle, 175).unwrap();
    assert_eq!(store.validate(&credential, 176), CredentialLeaseStatus::Revoked);

    let substituted_binding = SessionCredentialBinding::new(
        "session:000000000002",
        controller(),
        receiver(),
    )
    .unwrap();
    let substituted =
        SessionCredential::new(credential.handle.clone(), substituted_binding).unwrap();
    assert_eq!(
        store.validate(&substituted, 150),
        CredentialLeaseStatus::BindingMismatch
    );
}
