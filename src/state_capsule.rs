use core::fmt;

pub const MAX_STATE_ENTRIES: usize = 64;
pub const MAX_STATE_BYTES: usize = 64 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrivacyClass {
    Public,
    SessionMetadata,
    Sensitive,
    Restricted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateEntry {
    pub key: String,
    pub value: Vec<u8>,
    pub privacy_class: PrivacyClass,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateCapsule {
    pub schema_major: u16,
    pub schema_minor: u16,
    pub application_id: String,
    entries: Vec<StateEntry>,
}

impl StateCapsule {
    pub fn new(application_id: impl Into<String>) -> Result<Self, CapsuleError> {
        let application_id = application_id.into();
        if application_id.is_empty() || application_id.len() > 128 {
            return Err(CapsuleError::InvalidApplicationId);
        }
        Ok(Self {
            schema_major: 0,
            schema_minor: 1,
            application_id,
            entries: Vec::new(),
        })
    }

    pub fn push(&mut self, entry: StateEntry) -> Result<(), CapsuleError> {
        if entry.key.is_empty()
            || entry.key.len() > 96
            || self
                .entries
                .iter()
                .any(|existing| existing.key == entry.key)
        {
            return Err(CapsuleError::InvalidOrDuplicateKey);
        }
        if self.entries.len() >= MAX_STATE_ENTRIES {
            return Err(CapsuleError::TooManyEntries);
        }
        let current = self
            .entries
            .iter()
            .map(|entry| entry.value.len())
            .sum::<usize>();
        if current.saturating_add(entry.value.len()) > MAX_STATE_BYTES {
            return Err(CapsuleError::PayloadTooLarge);
        }
        self.entries.push(entry);
        Ok(())
    }

    pub fn entries(&self) -> &[StateEntry] {
        &self.entries
    }

    pub fn transferable_entries<'a>(
        &'a self,
        privacy: &PlatformPrivacyDecision,
    ) -> Result<Vec<&'a StateEntry>, CapsuleError> {
        if !privacy.allowed {
            return Err(CapsuleError::PrivacyAuthorizationRequired);
        }
        Ok(self
            .entries
            .iter()
            .filter(|entry| match entry.privacy_class {
                PrivacyClass::Public | PrivacyClass::SessionMetadata => true,
                PrivacyClass::Sensitive => privacy.allow_sensitive,
                PrivacyClass::Restricted => false,
            })
            .collect())
    }
}

/// Bounded decision supplied by the external Privacy Shield integration.
/// Cast consumes this decision; it does not mint or reinterpret it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformPrivacyDecision {
    pub allowed: bool,
    pub allow_sensitive: bool,
}

impl PlatformPrivacyDecision {
    pub const DENY: Self = Self {
        allowed: false,
        allow_sensitive: false,
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapsuleError {
    InvalidApplicationId,
    InvalidOrDuplicateKey,
    TooManyEntries,
    PayloadTooLarge,
    PrivacyAuthorizationRequired,
}

impl fmt::Display for CapsuleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for CapsuleError {}
