use std::collections::HashMap;
use std::fmt;

use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};

use crate::{
    auth::{
        AdapterError, AuthenticationChallenge, DeviceIdentitySigner, DeviceIdentityVerifier,
        FrameSigner, FrameVerifier, IdentityProof,
    },
    identity::DeviceId,
};

const IDENTITY_DOMAIN: &[u8] = b"goreecloud-cast/device-identity/v1\0";
const FRAME_DOMAIN: &[u8] = b"goreecloud-cast/local-frame/v1\0";

#[derive(Clone)]
pub struct Ed25519DeviceSigner {
    device_id: DeviceId,
    signing_key: SigningKey,
}

impl Ed25519DeviceSigner {
    pub fn from_secret_bytes(device_id: DeviceId, secret_key: [u8; 32]) -> Self {
        Self {
            device_id,
            signing_key: SigningKey::from_bytes(&secret_key),
        }
    }

    pub fn device_id(&self) -> &DeviceId {
        &self.device_id
    }

    pub fn verifying_key_bytes(&self) -> [u8; 32] {
        self.signing_key.verifying_key().to_bytes()
    }
}

impl fmt::Debug for Ed25519DeviceSigner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Ed25519DeviceSigner")
            .field("device_id", &self.device_id)
            .finish_non_exhaustive()
    }
}

impl DeviceIdentitySigner for Ed25519DeviceSigner {
    fn sign_challenge(
        &self,
        device_id: &DeviceId,
        challenge: &AuthenticationChallenge,
    ) -> Result<IdentityProof, AdapterError> {
        if device_id != &self.device_id {
            return Err(AdapterError::Denied);
        }
        let message = challenge_message(device_id, challenge);
        let signature: Signature = self.signing_key.sign(&message);
        IdentityProof::new(device_id.clone(), signature.to_bytes().to_vec())
    }
}

impl FrameSigner for Ed25519DeviceSigner {
    fn sign_frame(
        &self,
        sender: &DeviceId,
        receiver: &DeviceId,
        transport_sequence: u64,
        frame_bytes: &[u8],
    ) -> Result<Vec<u8>, AdapterError> {
        if sender != &self.device_id || transport_sequence == 0 {
            return Err(AdapterError::Denied);
        }
        let message = frame_message(sender, receiver, transport_sequence, frame_bytes)?;
        let signature: Signature = self.signing_key.sign(&message);
        Ok(signature.to_bytes().to_vec())
    }
}

#[derive(Clone, Default)]
pub struct Ed25519DeviceVerifier {
    keys: HashMap<DeviceId, VerifyingKey>,
}

impl Ed25519DeviceVerifier {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(
        &mut self,
        device_id: DeviceId,
        verifying_key: [u8; 32],
    ) -> Result<(), AdapterError> {
        let verifying_key = VerifyingKey::from_bytes(&verifying_key)
            .map_err(|_| AdapterError::InvalidIdentityProof)?;
        if verifying_key.is_weak() {
            return Err(AdapterError::InvalidIdentityProof);
        }
        self.keys.insert(device_id, verifying_key);
        Ok(())
    }

    pub fn remove(&mut self, device_id: &DeviceId) -> bool {
        self.keys.remove(device_id).is_some()
    }

    pub fn contains(&self, device_id: &DeviceId) -> bool {
        self.keys.contains_key(device_id)
    }
}

impl fmt::Debug for Ed25519DeviceVerifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Ed25519DeviceVerifier")
            .field("registered_devices", &self.keys.len())
            .finish()
    }
}

impl DeviceIdentityVerifier for Ed25519DeviceVerifier {
    fn verify_challenge(
        &self,
        challenge: &AuthenticationChallenge,
        proof: &IdentityProof,
    ) -> Result<bool, AdapterError> {
        let Some(verifying_key) = self.keys.get(&proof.device_id) else {
            return Ok(false);
        };
        let signature = match Signature::from_slice(&proof.proof) {
            Ok(signature) => signature,
            Err(_) => return Ok(false),
        };
        let message = challenge_message(&proof.device_id, challenge);
        Ok(verifying_key.verify_strict(&message, &signature).is_ok())
    }
}

impl FrameVerifier for Ed25519DeviceVerifier {
    fn verify_frame(
        &self,
        sender: &DeviceId,
        receiver: &DeviceId,
        transport_sequence: u64,
        frame_bytes: &[u8],
        signature: &[u8],
    ) -> Result<bool, AdapterError> {
        let Some(verifying_key) = self.keys.get(sender) else {
            return Ok(false);
        };
        if transport_sequence == 0 {
            return Ok(false);
        }
        let signature = match Signature::from_slice(signature) {
            Ok(signature) => signature,
            Err(_) => return Ok(false),
        };
        let message = frame_message(sender, receiver, transport_sequence, frame_bytes)?;
        Ok(verifying_key.verify_strict(&message, &signature).is_ok())
    }
}

fn challenge_message(device_id: &DeviceId, challenge: &AuthenticationChallenge) -> Vec<u8> {
    let mut message = Vec::with_capacity(
        IDENTITY_DOMAIN.len()
            + device_id.as_str().len()
            + challenge.context.len()
            + challenge.nonce.len()
            + 8,
    );
    message.extend_from_slice(IDENTITY_DOMAIN);
    append_bounded(&mut message, device_id.as_str().as_bytes());
    append_bounded(&mut message, challenge.context.as_bytes());
    message.extend_from_slice(&challenge.nonce);
    message
}

fn frame_message(
    sender: &DeviceId,
    receiver: &DeviceId,
    transport_sequence: u64,
    frame_bytes: &[u8],
) -> Result<Vec<u8>, AdapterError> {
    let frame_len =
        u32::try_from(frame_bytes.len()).map_err(|_| AdapterError::InvalidFrameProof)?;
    let mut message = Vec::with_capacity(
        FRAME_DOMAIN.len()
            + sender.as_str().len()
            + receiver.as_str().len()
            + frame_bytes.len()
            + 20,
    );
    message.extend_from_slice(FRAME_DOMAIN);
    append_bounded(&mut message, sender.as_str().as_bytes());
    append_bounded(&mut message, receiver.as_str().as_bytes());
    message.extend_from_slice(&transport_sequence.to_be_bytes());
    message.extend_from_slice(&frame_len.to_be_bytes());
    message.extend_from_slice(frame_bytes);
    Ok(message)
}

fn append_bounded(out: &mut Vec<u8>, value: &[u8]) {
    let len = u16::try_from(value.len()).expect("validated identity fields fit in u16");
    out.extend_from_slice(&len.to_be_bytes());
    out.extend_from_slice(value);
}
