use crate::identity::DeviceId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    Proposed,
    Authorized,
    Connecting,
    Active,
    Recovering,
    Ended,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateAuthority {
    Receiver,
    Controller,
    Shared,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionDescriptor {
    pub session_id: String,
    pub controller: DeviceId,
    pub receiver: DeviceId,
    pub state: SessionState,
    pub authority: StateAuthority,
}

impl SessionDescriptor {
    pub fn new(
        session_id: impl Into<String>,
        controller: DeviceId,
        receiver: DeviceId,
        authority: StateAuthority,
    ) -> Result<Self, SessionError> {
        let session_id = session_id.into();
        if !(16..=128).contains(&session_id.len()) {
            return Err(SessionError::InvalidSessionId);
        }
        Ok(Self {
            session_id,
            controller,
            receiver,
            state: SessionState::Proposed,
            authority,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionError {
    InvalidSessionId,
}
