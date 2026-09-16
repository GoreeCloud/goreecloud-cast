use core::fmt;

use crate::{
    auth::{
        AdapterError, AuthenticationChallenge, DeviceIdentitySigner, DeviceIdentityVerifier,
        PairingAuthorizer, PairingGrant, PairingRequest, SessionCredential,
        SessionCredentialAuthority, SessionCredentialBinding,
    },
    discovery::AuthenticatedDiscoveryDetails,
    identity::DeviceId,
    protocol::{
        MessageKind, ProtocolError, ProtocolFrame, SessionOpenPayload,
        decode_authenticated_discovery_details, decode_session_open,
        encode_authenticated_discovery_details, encode_session_open,
    },
    session::{SessionDescriptor, SessionError, SessionState, StateAuthority},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandshakeRequest {
    pub controller: DeviceId,
    pub receiver: DeviceId,
    pub session_id: String,
    pub rotating_discovery_id: String,
    pub challenge_nonce: [u8; 32],
}

impl HandshakeRequest {
    pub fn new(
        controller: DeviceId,
        receiver: DeviceId,
        session_id: impl Into<String>,
        rotating_discovery_id: impl Into<String>,
        challenge_nonce: [u8; 32],
    ) -> Result<Self, HandshakeError> {
        let session_id = session_id.into();
        if !(16..=128).contains(&session_id.len()) || session_id.chars().any(char::is_control) {
            return Err(HandshakeError::InvalidRequest);
        }
        let rotating_discovery_id = rotating_discovery_id.into();
        if !(16..=96).contains(&rotating_discovery_id.len())
            || rotating_discovery_id.chars().any(char::is_control)
        {
            return Err(HandshakeError::InvalidRequest);
        }
        Ok(Self {
            controller,
            receiver,
            session_id,
            rotating_discovery_id,
            challenge_nonce,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandshakeOutcome {
    pub session: SessionDescriptor,
    pub pairing_grant: PairingGrant,
    pub credential: SessionCredential,
    pub receiver_details: AuthenticatedDiscoveryDetails,
    pub transcript: Vec<ProtocolFrame>,
}

#[derive(Debug)]
pub struct InMemoryHandshakeHarness<'a> {
    signer: &'a dyn DeviceIdentitySigner,
    verifier: &'a dyn DeviceIdentityVerifier,
    pairing: &'a dyn PairingAuthorizer,
    credentials: &'a dyn SessionCredentialAuthority,
}

impl<'a> InMemoryHandshakeHarness<'a> {
    pub fn new(
        signer: &'a dyn DeviceIdentitySigner,
        verifier: &'a dyn DeviceIdentityVerifier,
        pairing: &'a dyn PairingAuthorizer,
        credentials: &'a dyn SessionCredentialAuthority,
    ) -> Self {
        Self {
            signer,
            verifier,
            pairing,
            credentials,
        }
    }

    pub fn establish(
        &self,
        request: &HandshakeRequest,
        receiver_details: &AuthenticatedDiscoveryDetails,
    ) -> Result<HandshakeOutcome, HandshakeError> {
        if receiver_details.device_id != request.receiver
            || receiver_details.rotating_discovery_id != request.rotating_discovery_id
        {
            return Err(HandshakeError::ReceiverMismatch);
        }

        let challenge = AuthenticationChallenge::new(
            request.challenge_nonce,
            "goreecloud-cast:pairing-and-discovery",
        )?;
        let proof = self
            .signer
            .sign_challenge(&request.controller, &challenge)?;
        if proof.device_id != request.controller
            || !self.verifier.verify_challenge(&challenge, &proof)?
        {
            return Err(HandshakeError::IdentityRejected);
        }

        let mut transcript = Vec::new();
        let discovery_request = round_trip_frame(ProtocolFrame::new(
            MessageKind::DiscoveryDetailsRequest,
            1,
            request.rotating_discovery_id.as_bytes().to_vec(),
        )?)?;
        transcript.push(discovery_request);

        let discovery_payload = encode_authenticated_discovery_details(receiver_details)?;
        let discovery_response = round_trip_frame(ProtocolFrame::new(
            MessageKind::DiscoveryDetailsResponse,
            1,
            discovery_payload,
        )?)?;
        let decoded_details = decode_authenticated_discovery_details(&discovery_response.payload)?;
        if decoded_details != *receiver_details {
            return Err(HandshakeError::ProtocolRejected);
        }
        transcript.push(discovery_response);

        let pairing_grant = self.pairing.authorize_pairing(&PairingRequest {
            controller: request.controller.clone(),
            receiver: request.receiver.clone(),
        })?;
        if pairing_grant.controller != request.controller
            || pairing_grant.receiver != request.receiver
        {
            return Err(HandshakeError::PairingRejected);
        }

        let mut session = SessionDescriptor::new(
            request.session_id.clone(),
            request.controller.clone(),
            request.receiver.clone(),
            StateAuthority::Receiver,
        )?;
        session.state = SessionState::Authorized;

        let binding = SessionCredentialBinding::new(
            request.session_id.clone(),
            request.controller.clone(),
            request.receiver.clone(),
        )?;
        let credential = self.credentials.issue(&binding)?;
        if credential.binding != binding || !self.credentials.validate(&credential)? {
            return Err(HandshakeError::CredentialRejected);
        }

        let session_open_payload = SessionOpenPayload::from_credential(&credential);
        let session_open = round_trip_frame(ProtocolFrame::new(
            MessageKind::SessionOpen,
            2,
            encode_session_open(&session_open_payload)?,
        )?)?;
        let decoded_open = decode_session_open(&session_open.payload)?;
        if decoded_open != session_open_payload {
            return Err(HandshakeError::ProtocolRejected);
        }
        transcript.push(session_open);

        session.state = SessionState::Connecting;
        let session_accepted = round_trip_frame(ProtocolFrame::new(
            MessageKind::SessionAccepted,
            2,
            request.session_id.as_bytes().to_vec(),
        )?)?;
        transcript.push(session_accepted);
        session.state = SessionState::Active;

        Ok(HandshakeOutcome {
            session,
            pairing_grant,
            credential,
            receiver_details: decoded_details,
            transcript,
        })
    }
}

fn round_trip_frame(frame: ProtocolFrame) -> Result<ProtocolFrame, ProtocolError> {
    ProtocolFrame::decode(&frame.encode())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandshakeError {
    InvalidRequest,
    ReceiverMismatch,
    IdentityRejected,
    PairingRejected,
    CredentialRejected,
    ProtocolRejected,
    AdapterFailure,
    ProtocolFailure,
    SessionFailure,
}

impl From<AdapterError> for HandshakeError {
    fn from(_: AdapterError) -> Self {
        Self::AdapterFailure
    }
}

impl From<ProtocolError> for HandshakeError {
    fn from(_: ProtocolError) -> Self {
        Self::ProtocolFailure
    }
}

impl From<SessionError> for HandshakeError {
    fn from(_: SessionError) -> Self {
        Self::SessionFailure
    }
}

impl fmt::Display for HandshakeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for HandshakeError {}
