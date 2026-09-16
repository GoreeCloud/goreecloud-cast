use core::fmt;

use crate::auth::{PairingGrant, PairingRequest};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PairingState {
    AwaitingConfirmation,
    Confirmed,
    Denied,
    Cancelled,
    Expired,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PairingConfirmation {
    request: PairingRequest,
    created_at_ms: u64,
    expires_at_ms: u64,
    state: PairingState,
    grant: Option<PairingGrant>,
}

impl PairingConfirmation {
    pub fn new(
        request: PairingRequest,
        created_at_ms: u64,
        expires_at_ms: u64,
    ) -> Result<Self, PairingFlowError> {
        if expires_at_ms <= created_at_ms {
            return Err(PairingFlowError::InvalidDeadline);
        }
        Ok(Self {
            request,
            created_at_ms,
            expires_at_ms,
            state: PairingState::AwaitingConfirmation,
            grant: None,
        })
    }

    pub fn request(&self) -> &PairingRequest {
        &self.request
    }

    pub fn created_at_ms(&self) -> u64 {
        self.created_at_ms
    }

    pub fn expires_at_ms(&self) -> u64 {
        self.expires_at_ms
    }

    pub fn state(&mut self, now_ms: u64) -> PairingState {
        self.expire_if_needed(now_ms);
        self.state
    }

    pub fn grant(&mut self, now_ms: u64) -> Option<&PairingGrant> {
        self.expire_if_needed(now_ms);
        self.grant.as_ref()
    }

    pub fn confirm(
        &mut self,
        now_ms: u64,
        grant: PairingGrant,
    ) -> Result<(), PairingFlowError> {
        self.ensure_pending(now_ms)?;
        if grant.controller != self.request.controller || grant.receiver != self.request.receiver {
            return Err(PairingFlowError::GrantBindingMismatch);
        }
        self.grant = Some(grant);
        self.state = PairingState::Confirmed;
        Ok(())
    }

    pub fn deny(&mut self, now_ms: u64) -> Result<(), PairingFlowError> {
        self.ensure_pending(now_ms)?;
        self.state = PairingState::Denied;
        Ok(())
    }

    pub fn cancel(&mut self, now_ms: u64) -> Result<(), PairingFlowError> {
        self.ensure_pending(now_ms)?;
        self.state = PairingState::Cancelled;
        Ok(())
    }

    fn ensure_pending(&mut self, now_ms: u64) -> Result<(), PairingFlowError> {
        self.expire_if_needed(now_ms);
        match self.state {
            PairingState::AwaitingConfirmation => Ok(()),
            PairingState::Expired => Err(PairingFlowError::Expired),
            _ => Err(PairingFlowError::Finalized),
        }
    }

    fn expire_if_needed(&mut self, now_ms: u64) {
        if self.state == PairingState::AwaitingConfirmation && now_ms >= self.expires_at_ms {
            self.state = PairingState::Expired;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PairingFlowError {
    InvalidDeadline,
    Expired,
    Finalized,
    GrantBindingMismatch,
}

impl fmt::Display for PairingFlowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for PairingFlowError {}
