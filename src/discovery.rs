use crate::{
    identity::DeviceId,
    PROTOCOL_MAJOR, PROTOCOL_MINOR,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceCategory {
    Display,
    Speaker,
    Computer,
    Mobile,
    Embedded,
    Browser,
    Other,
}

/// Minimum information suitable for unauthenticated local discovery.
///
/// It intentionally excludes user identity, account identity, friendly device name,
/// room, trust level, ownership relationship, media history, and network credentials.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscoveryAdvertisement {
    pub rotating_discovery_id: String,
    pub category: DeviceCategory,
    pub protocol_major: u16,
    pub protocol_minor: u16,
    pub capability_digest: u64,
    pub requires_confirmation: bool,
}

impl DiscoveryAdvertisement {
    pub fn new(
        rotating_discovery_id: impl Into<String>,
        category: DeviceCategory,
        capability_digest: u64,
        requires_confirmation: bool,
    ) -> Result<Self, DiscoveryError> {
        let rotating_discovery_id = rotating_discovery_id.into();
        if !(16..=96).contains(&rotating_discovery_id.len())
            || rotating_discovery_id.chars().any(char::is_control)
        {
            return Err(DiscoveryError::InvalidRotatingId);
        }
        Ok(Self {
            rotating_discovery_id,
            category,
            protocol_major: PROTOCOL_MAJOR,
            protocol_minor: PROTOCOL_MINOR,
            capability_digest,
            requires_confirmation,
        })
    }
}

/// Receiver metadata that may be revealed only after the peer has authenticated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthenticatedDiscoveryDetails {
    pub rotating_discovery_id: String,
    pub device_id: DeviceId,
    pub friendly_name: String,
    pub category: DeviceCategory,
    pub capability_digest: u64,
    pub requires_confirmation: bool,
}

impl AuthenticatedDiscoveryDetails {
    pub fn new(
        rotating_discovery_id: impl Into<String>,
        device_id: DeviceId,
        friendly_name: impl Into<String>,
        category: DeviceCategory,
        capability_digest: u64,
        requires_confirmation: bool,
    ) -> Result<Self, DiscoveryError> {
        let rotating_discovery_id = rotating_discovery_id.into();
        if !(16..=96).contains(&rotating_discovery_id.len())
            || rotating_discovery_id.chars().any(char::is_control)
        {
            return Err(DiscoveryError::InvalidRotatingId);
        }

        let friendly_name = friendly_name.into();
        if friendly_name.is_empty()
            || friendly_name.len() > 96
            || friendly_name.chars().any(char::is_control)
        {
            return Err(DiscoveryError::InvalidFriendlyName);
        }

        Ok(Self {
            rotating_discovery_id,
            device_id,
            friendly_name,
            category,
            capability_digest,
            requires_confirmation,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiscoveryError {
    InvalidRotatingId,
    InvalidFriendlyName,
}
