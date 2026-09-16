use std::collections::HashMap;
use std::fmt;

use crate::{
    auth::{AdapterError, FrameSigner, FrameVerifier, SessionCredential},
    credentials::{CredentialLeaseStatus, InMemoryCredentialLeaseStore},
    identity::DeviceId,
    local_transport::{
        InMemoryLocalLink, LocalFrameReceiver, LocalFrameTransmitter, LocalTransportError,
    },
    protocol::{
        MessageKind, ProtocolError, ProtocolFrame, SessionOpenPayload, decode_session_open,
        encode_session_open,
    },
    session::{SessionDescriptor, SessionError, SessionState, StateAuthority},
    trust::PersistentTrustStore,
};

#[derive(Debug)]
pub struct ControllerEndpoint<'a> {
    device_id: DeviceId,
    transmitter: LocalFrameTransmitter<'a>,
    receiver: LocalFrameReceiver<'a>,
}

impl<'a> ControllerEndpoint<'a> {
    pub fn new(
        device_id: DeviceId,
        signer: &'a dyn FrameSigner,
        verifier: &'a dyn FrameVerifier,
    ) -> Self {
        Self {
            transmitter: LocalFrameTransmitter::new(device_id.clone(), signer),
            receiver: LocalFrameReceiver::new(device_id.clone(), verifier),
            device_id,
        }
    }

    pub fn send_session_open(
        &mut self,
        link: &mut InMemoryLocalLink,
        receiver: &DeviceId,
        credential: &SessionCredential,
        correlation_id: u64,
    ) -> Result<(), EndpointError> {
        if credential.binding.controller != self.device_id || credential.binding.receiver != *receiver {
            return Err(EndpointError::BindingMismatch);
        }
        let payload = encode_session_open(&SessionOpenPayload::from_credential(credential))?;
        let frame = ProtocolFrame::new(MessageKind::SessionOpen, correlation_id, payload)?;
        let packet = self.transmitter.seal(receiver, &frame)?;
        link.send_raw(receiver.clone(), packet);
        Ok(())
    }

    pub fn receive_session_accepted(
        &mut self,
        link: &mut InMemoryLocalLink,
    ) -> Result<Option<String>, EndpointError> {
        let Some(packet) = link.receive_raw(&self.device_id) else {
            return Ok(None);
        };
        let opened = self.receiver.open(&packet)?;
        if opened.frame.kind != MessageKind::SessionAccepted {
            return Err(EndpointError::UnexpectedMessage);
        }
        let session_id = String::from_utf8(opened.frame.payload)
            .map_err(|_| EndpointError::ProtocolFailure)?;
        if !(16..=128).contains(&session_id.len()) || session_id.chars().any(char::is_control) {
            return Err(EndpointError::ProtocolFailure);
        }
        Ok(Some(session_id))
    }
}

#[derive(Debug)]
pub struct ReceiverEndpoint<'a> {
    device_id: DeviceId,
    transmitter: LocalFrameTransmitter<'a>,
    receiver: LocalFrameReceiver<'a>,
    trust: &'a dyn PersistentTrustStore,
    credentials: &'a InMemoryCredentialLeaseStore,
    sessions: HashMap<String, SessionDescriptor>,
}

impl<'a> ReceiverEndpoint<'a> {
    pub fn new(
        device_id: DeviceId,
        signer: &'a dyn FrameSigner,
        verifier: &'a dyn FrameVerifier,
        trust: &'a dyn PersistentTrustStore,
        credentials: &'a InMemoryCredentialLeaseStore,
    ) -> Self {
        Self {
            transmitter: LocalFrameTransmitter::new(device_id.clone(), signer),
            receiver: LocalFrameReceiver::new(device_id.clone(), verifier),
            device_id,
            trust,
            credentials,
            sessions: HashMap::new(),
        }
    }

    pub fn process_next(
        &mut self,
        link: &mut InMemoryLocalLink,
        now_ms: u64,
    ) -> Result<Option<SessionDescriptor>, EndpointError> {
        let Some(packet) = link.receive_raw(&self.device_id) else {
            return Ok(None);
        };
        let opened = self.receiver.open(&packet)?;
        if opened.frame.kind != MessageKind::SessionOpen {
            return Err(EndpointError::UnexpectedMessage);
        }

        let payload = decode_session_open(&opened.frame.payload)?;
        if payload.binding.controller != opened.sender || payload.binding.receiver != self.device_id {
            return Err(EndpointError::BindingMismatch);
        }
        let trust = self
            .trust
            .lookup(&opened.sender)
            .ok_or(EndpointError::UntrustedController)?;
        if !trust.is_active() {
            return Err(EndpointError::UntrustedController);
        }

        let credential = SessionCredential::new(
            payload.credential_handle,
            payload.binding.clone(),
        )?;
        if self.credentials.validate(&credential, now_ms) != CredentialLeaseStatus::Valid {
            return Err(EndpointError::CredentialRejected);
        }

        let mut session = SessionDescriptor::new(
            payload.binding.session_id.clone(),
            payload.binding.controller,
            payload.binding.receiver,
            StateAuthority::Receiver,
        )?;
        session.state = SessionState::Active;
        self.sessions
            .insert(session.session_id.clone(), session.clone());

        let accepted = ProtocolFrame::new(
            MessageKind::SessionAccepted,
            opened.frame.correlation_id,
            session.session_id.as_bytes().to_vec(),
        )?;
        let response = self.transmitter.seal(&opened.sender, &accepted)?;
        link.send_raw(opened.sender, response);
        Ok(Some(session))
    }

    pub fn session(&self, session_id: &str) -> Option<&SessionDescriptor> {
        self.sessions.get(session_id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndpointError {
    BindingMismatch,
    UntrustedController,
    CredentialRejected,
    UnexpectedMessage,
    TransportFailure,
    ProtocolFailure,
    AdapterFailure,
    SessionFailure,
}

impl From<LocalTransportError> for EndpointError {
    fn from(_: LocalTransportError) -> Self {
        Self::TransportFailure
    }
}

impl From<ProtocolError> for EndpointError {
    fn from(_: ProtocolError) -> Self {
        Self::ProtocolFailure
    }
}

impl From<AdapterError> for EndpointError {
    fn from(_: AdapterError) -> Self {
        Self::AdapterFailure
    }
}

impl From<SessionError> for EndpointError {
    fn from(_: SessionError) -> Self {
        Self::SessionFailure
    }
}

impl fmt::Display for EndpointError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for EndpointError {}
