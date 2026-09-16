use std::fmt;

use crate::identity::DeviceId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceKeyDescriptor {
    pub device_id: DeviceId,
    pub key_handle: String,
    pub generation: u64,
    pub verifying_key: [u8; 32],
}

impl DeviceKeyDescriptor {
    pub fn new(
        device_id: DeviceId,
        key_handle: impl Into<String>,
        generation: u64,
        verifying_key: [u8; 32],
    ) -> Result<Self, KeyManagementError> {
        let key_handle = key_handle.into();
        if !(16..=192).contains(&key_handle.len())
            || key_handle.chars().any(char::is_control)
            || generation == 0
        {
            return Err(KeyManagementError::InvalidDescriptor);
        }
        Ok(Self {
            device_id,
            key_handle,
            generation,
            verifying_key,
        })
    }
}

pub trait DeviceKeyCustody: fmt::Debug {
    fn active_key(&self, device_id: &DeviceId) -> Result<DeviceKeyDescriptor, KeyManagementError>;
    fn sign(&self, key_handle: &str, message: &[u8]) -> Result<Vec<u8>, KeyManagementError>;
    fn rotate(&mut self, device_id: &DeviceId) -> Result<DeviceKeyDescriptor, KeyManagementError>;
    fn revoke(&mut self, key_handle: &str) -> Result<(), KeyManagementError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyManagementError {
    InvalidDescriptor,
    UnknownDevice,
    UnknownKey,
    Denied,
    Unavailable,
}

impl fmt::Display for KeyManagementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for KeyManagementError {}
