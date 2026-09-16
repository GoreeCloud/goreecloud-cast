use core::fmt;

use crate::identity::{DeviceId, TrustLevel};

pub const AUTHENTICATION_CHALLENGE_BYTES: usize = 32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthenticationChallenge {
    pub nonce: [u8; AUTHENTICATION_CHALLENGE_BYTES],
    pub context: String,
}

impl AuthenticationChallenge {
    pub fn new(
        nonce: [u8; AUTHENTICATION_CHALLENGE_BYTES],
        context: impl Into<String>,
    ) -> Result<Self, AdapterError> {
        let context = context.into();
        if context.is_empty() || context.len() > 96 || context.chars().any(char::is_control) {
            return Err(AdapterError::InvalidChallenge);
        }
        Ok(Self { nonce, context })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentityProof {
    pub device_id: DeviceId,
    pub proof: Vec<u8>,
}

impl IdentityProof {
    pub fn new(device_id: DeviceId, proof: Vec<u8>) -> Result<Self, AdapterError> {
        if !(16..=4096).contains(&proof.len()) {
            return Err(AdapterError::InvalidIdentityProof);
        }
        Ok(Self { device_id, proof })
    }
}

pub trait DeviceIdentitySigner: fmt::Debug {
    fn sign_challenge(
        &self,
        device_id: &DeviceId,
        challenge: &AuthenticationChallenge,
    ) -> Result<IdentityProof, AdapterError>;
}

pub trait DeviceIdentityVerifier: fmt::Debug {
    fn verify_challenge(
        &self,
        challenge: &AuthenticationChallenge,
        proof: &IdentityProof,
    ) -> Result<bool, AdapterError>;
}

pub trait FrameSigner: fmt::Debug {
    fn sign_frame(
        &self,
        sender: &DeviceId,
        receiver: &DeviceId,
        transport_sequence: u64,
        frame_bytes: &[u8],
    ) -> Result<Vec<u8>, AdapterError>;
}

pub trait FrameVerifier: fmt::Debug {
    fn verify_frame(
        &self,
        sender: &DeviceId,
        receiver: &DeviceId,
        transport_sequence: u64,
        frame_bytes: &[u8],
        signature: &[u8],
    ) -> Result<bool, AdapterError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PairingRequest {
    pub controller: DeviceId,
    pub receiver: DeviceId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PairingGrant {
    pub grant_id: String,
    pub controller: DeviceId,
    pub receiver: DeviceId,
    pub trust_level: TrustLevel,
}

impl PairingGrant {
    pub fn new(
        grant_id: impl Into<String>,
        controller: DeviceId,
        receiver: DeviceId,
        trust_level: TrustLevel,
    ) -> Result<Self, AdapterError> {
        let grant_id = grant_id.into();
        if !(16..=192).contains(&grant_id.len()) || grant_id.chars().any(char::is_control) {
            return Err(AdapterError::InvalidPairingGrant);
        }
        Ok(Self {
            grant_id,
            controller,
            receiver,
            trust_level,
        })
    }
}

pub trait PairingAuthorizer: fmt::Debug {
    fn authorize_pairing(&self, request: &PairingRequest) -> Result<PairingGrant, AdapterError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionCredentialBinding {
    pub session_id: String,
    pub controller: DeviceId,
    pub receiver: DeviceId,
}

impl SessionCredentialBinding {
    pub fn new(
        session_id: impl Into<String>,
        controller: DeviceId,
        receiver: DeviceId,
    ) -> Result<Self, AdapterError> {
        let session_id = session_id.into();
        if !(16..=128).contains(&session_id.len()) || session_id.chars().any(char::is_control) {
            return Err(AdapterError::InvalidCredentialBinding);
        }
        Ok(Self {
            session_id,
            controller,
            receiver,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionCredential {
    pub handle: String,
    pub binding: SessionCredentialBinding,
}

impl SessionCredential {
    pub fn new(
        handle: impl Into<String>,
        binding: SessionCredentialBinding,
    ) -> Result<Self, AdapterError> {
        let handle = handle.into();
        if !(16..=256).contains(&handle.len()) || handle.chars().any(char::is_control) {
            return Err(AdapterError::InvalidSessionCredential);
        }
        Ok(Self { handle, binding })
    }
}

pub trait SessionCredentialAuthority: fmt::Debug {
    fn issue(&self, binding: &SessionCredentialBinding) -> Result<SessionCredential, AdapterError>;

    fn validate(&self, credential: &SessionCredential) -> Result<bool, AdapterError>;

    fn revoke(&self, credential: &SessionCredential) -> Result<(), AdapterError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdapterError {
    InvalidChallenge,
    InvalidIdentityProof,
    InvalidFrameProof,
    InvalidPairingGrant,
    InvalidCredentialBinding,
    InvalidSessionCredential,
    Denied,
    Unavailable,
}

impl fmt::Display for AdapterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for AdapterError {}
