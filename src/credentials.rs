use std::collections::HashMap;
use std::fmt;

use crate::auth::SessionCredential;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CredentialLeaseStatus {
    Valid,
    Expired,
    Revoked,
    Unknown,
    BindingMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CredentialLease {
    pub credential: SessionCredential,
    pub issued_at_ms: u64,
    pub expires_at_ms: u64,
    pub revoked_at_ms: Option<u64>,
}

impl CredentialLease {
    pub fn new(
        credential: SessionCredential,
        issued_at_ms: u64,
        expires_at_ms: u64,
    ) -> Result<Self, CredentialLeaseError> {
        if expires_at_ms <= issued_at_ms {
            return Err(CredentialLeaseError::InvalidDeadline);
        }
        Ok(Self {
            credential,
            issued_at_ms,
            expires_at_ms,
            revoked_at_ms: None,
        })
    }

    pub fn status_at(&self, now_ms: u64) -> CredentialLeaseStatus {
        if self.revoked_at_ms.is_some() {
            CredentialLeaseStatus::Revoked
        } else if now_ms >= self.expires_at_ms {
            CredentialLeaseStatus::Expired
        } else {
            CredentialLeaseStatus::Valid
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct InMemoryCredentialLeaseStore {
    leases: HashMap<String, CredentialLease>,
}

impl InMemoryCredentialLeaseStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, lease: CredentialLease) -> Result<(), CredentialLeaseError> {
        let handle = lease.credential.handle.clone();
        if self.leases.contains_key(&handle) {
            return Err(CredentialLeaseError::DuplicateHandle);
        }
        self.leases.insert(handle, lease);
        Ok(())
    }

    pub fn validate(&self, presented: &SessionCredential, now_ms: u64) -> CredentialLeaseStatus {
        let Some(lease) = self.leases.get(&presented.handle) else {
            return CredentialLeaseStatus::Unknown;
        };
        if lease.credential.binding != presented.binding {
            return CredentialLeaseStatus::BindingMismatch;
        }
        lease.status_at(now_ms)
    }

    pub fn revoke(&mut self, handle: &str, revoked_at_ms: u64) -> Result<(), CredentialLeaseError> {
        let lease = self
            .leases
            .get_mut(handle)
            .ok_or(CredentialLeaseError::UnknownHandle)?;
        if lease.revoked_at_ms.is_none() {
            lease.revoked_at_ms = Some(revoked_at_ms);
        }
        Ok(())
    }

    pub fn get(&self, handle: &str) -> Option<&CredentialLease> {
        self.leases.get(handle)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CredentialLeaseError {
    InvalidDeadline,
    DuplicateHandle,
    UnknownHandle,
}

impl fmt::Display for CredentialLeaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for CredentialLeaseError {}
