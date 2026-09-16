use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeviceId(String);

impl DeviceId {
    pub fn parse(value: impl Into<String>) -> Result<Self, IdentityError> {
        let value = value.into();
        let valid_len = (16..=128).contains(&value.len());
        let valid_chars = value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b':'));
        if valid_len && valid_chars {
            Ok(Self(value))
        } else {
            Err(IdentityError::InvalidDeviceId)
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrustLevel {
    Owner,
    Household,
    ApprovedDevice,
    Guest,
    Untrusted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActorContext {
    pub device_id: DeviceId,
    pub trust_level: TrustLevel,
    pub authenticated: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdentityError {
    InvalidDeviceId,
}

impl fmt::Display for IdentityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDeviceId => f.write_str("invalid device identifier"),
        }
    }
}

impl std::error::Error for IdentityError {}
