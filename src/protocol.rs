use core::fmt;

use crate::{
    auth::{SessionCredential, SessionCredentialBinding},
    discovery::{AuthenticatedDiscoveryDetails, DeviceCategory},
    identity::DeviceId,
    PROTOCOL_MAJOR, PROTOCOL_MINOR,
};

const MAGIC: &[u8; 4] = b"GCC0";
pub const MAX_FRAME_PAYLOAD_BYTES: usize = 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageKind {
    DiscoveryDetailsRequest,
    DiscoveryDetailsResponse,
    SessionOpen,
    SessionAccepted,
    Command,
    SessionClose,
}

impl MessageKind {
    fn wire_code(self) -> u16 {
        match self {
            Self::DiscoveryDetailsRequest => 1,
            Self::DiscoveryDetailsResponse => 2,
            Self::SessionOpen => 3,
            Self::SessionAccepted => 4,
            Self::Command => 5,
            Self::SessionClose => 6,
        }
    }

    fn from_wire_code(value: u16) -> Result<Self, ProtocolError> {
        match value {
            1 => Ok(Self::DiscoveryDetailsRequest),
            2 => Ok(Self::DiscoveryDetailsResponse),
            3 => Ok(Self::SessionOpen),
            4 => Ok(Self::SessionAccepted),
            5 => Ok(Self::Command),
            6 => Ok(Self::SessionClose),
            _ => Err(ProtocolError::UnknownMessageKind),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtocolFrame {
    pub kind: MessageKind,
    pub correlation_id: u64,
    pub payload: Vec<u8>,
}

impl ProtocolFrame {
    pub fn new(
        kind: MessageKind,
        correlation_id: u64,
        payload: Vec<u8>,
    ) -> Result<Self, ProtocolError> {
        if payload.len() > MAX_FRAME_PAYLOAD_BYTES {
            return Err(ProtocolError::FramePayloadTooLarge);
        }
        Ok(Self {
            kind,
            correlation_id,
            payload,
        })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut output = Vec::with_capacity(22 + self.payload.len());
        output.extend_from_slice(MAGIC);
        output.extend_from_slice(&PROTOCOL_MAJOR.to_be_bytes());
        output.extend_from_slice(&PROTOCOL_MINOR.to_be_bytes());
        output.extend_from_slice(&self.kind.wire_code().to_be_bytes());
        output.extend_from_slice(&self.correlation_id.to_be_bytes());
        output.extend_from_slice(&(self.payload.len() as u32).to_be_bytes());
        output.extend_from_slice(&self.payload);
        output
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, ProtocolError> {
        let mut cursor = Cursor::new(bytes);
        if cursor.take_exact(4)? != MAGIC {
            return Err(ProtocolError::BadMagic);
        }

        let major = cursor.take_u16()?;
        let minor = cursor.take_u16()?;
        if major != PROTOCOL_MAJOR || minor > PROTOCOL_MINOR {
            return Err(ProtocolError::UnsupportedVersion);
        }

        let kind = MessageKind::from_wire_code(cursor.take_u16()?)?;
        let correlation_id = cursor.take_u64()?;
        let payload_len = cursor.take_u32()? as usize;
        if payload_len > MAX_FRAME_PAYLOAD_BYTES {
            return Err(ProtocolError::FramePayloadTooLarge);
        }
        let payload = cursor.take_exact(payload_len)?.to_vec();
        cursor.finish()?;

        Ok(Self {
            kind,
            correlation_id,
            payload,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionOpenPayload {
    pub binding: SessionCredentialBinding,
    pub credential_handle: String,
}

impl SessionOpenPayload {
    pub fn from_credential(credential: &SessionCredential) -> Self {
        Self {
            binding: credential.binding.clone(),
            credential_handle: credential.handle.clone(),
        }
    }
}

pub fn encode_authenticated_discovery_details(
    details: &AuthenticatedDiscoveryDetails,
) -> Result<Vec<u8>, ProtocolError> {
    let mut output = Vec::new();
    push_string(&mut output, &details.rotating_discovery_id)?;
    push_string(&mut output, details.device_id.as_str())?;
    push_string(&mut output, &details.friendly_name)?;
    output.push(category_to_wire(details.category));
    output.extend_from_slice(&details.capability_digest.to_be_bytes());
    output.push(u8::from(details.requires_confirmation));
    Ok(output)
}

pub fn decode_authenticated_discovery_details(
    bytes: &[u8],
) -> Result<AuthenticatedDiscoveryDetails, ProtocolError> {
    let mut cursor = Cursor::new(bytes);
    let rotating_discovery_id = cursor.take_string()?;
    let device_id = DeviceId::parse(cursor.take_string()?)
        .map_err(|_| ProtocolError::InvalidField)?;
    let friendly_name = cursor.take_string()?;
    let category = category_from_wire(cursor.take_u8()?)?;
    let capability_digest = cursor.take_u64()?;
    let requires_confirmation = match cursor.take_u8()? {
        0 => false,
        1 => true,
        _ => return Err(ProtocolError::InvalidField),
    };
    cursor.finish()?;

    AuthenticatedDiscoveryDetails::new(
        rotating_discovery_id,
        device_id,
        friendly_name,
        category,
        capability_digest,
        requires_confirmation,
    )
    .map_err(|_| ProtocolError::InvalidField)
}

pub fn encode_session_open(payload: &SessionOpenPayload) -> Result<Vec<u8>, ProtocolError> {
    let mut output = Vec::new();
    push_string(&mut output, &payload.binding.session_id)?;
    push_string(&mut output, payload.binding.controller.as_str())?;
    push_string(&mut output, payload.binding.receiver.as_str())?;
    push_string(&mut output, &payload.credential_handle)?;
    Ok(output)
}

pub fn decode_session_open(bytes: &[u8]) -> Result<SessionOpenPayload, ProtocolError> {
    let mut cursor = Cursor::new(bytes);
    let session_id = cursor.take_string()?;
    let controller = DeviceId::parse(cursor.take_string()?)
        .map_err(|_| ProtocolError::InvalidField)?;
    let receiver = DeviceId::parse(cursor.take_string()?)
        .map_err(|_| ProtocolError::InvalidField)?;
    let credential_handle = cursor.take_string()?;
    cursor.finish()?;

    let binding = SessionCredentialBinding::new(session_id, controller, receiver)
        .map_err(|_| ProtocolError::InvalidField)?;
    if !(16..=256).contains(&credential_handle.len())
        || credential_handle.chars().any(char::is_control)
    {
        return Err(ProtocolError::InvalidField);
    }

    Ok(SessionOpenPayload {
        binding,
        credential_handle,
    })
}

fn push_string(output: &mut Vec<u8>, value: &str) -> Result<(), ProtocolError> {
    let length = u16::try_from(value.len()).map_err(|_| ProtocolError::FieldTooLarge)?;
    output.extend_from_slice(&length.to_be_bytes());
    output.extend_from_slice(value.as_bytes());
    Ok(())
}

fn category_to_wire(category: DeviceCategory) -> u8 {
    match category {
        DeviceCategory::Display => 1,
        DeviceCategory::Speaker => 2,
        DeviceCategory::Computer => 3,
        DeviceCategory::Mobile => 4,
        DeviceCategory::Embedded => 5,
        DeviceCategory::Browser => 6,
        DeviceCategory::Other => 7,
    }
}

fn category_from_wire(value: u8) -> Result<DeviceCategory, ProtocolError> {
    match value {
        1 => Ok(DeviceCategory::Display),
        2 => Ok(DeviceCategory::Speaker),
        3 => Ok(DeviceCategory::Computer),
        4 => Ok(DeviceCategory::Mobile),
        5 => Ok(DeviceCategory::Embedded),
        6 => Ok(DeviceCategory::Browser),
        7 => Ok(DeviceCategory::Other),
        _ => Err(ProtocolError::InvalidField),
    }
}

struct Cursor<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Cursor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    fn take_exact(&mut self, length: usize) -> Result<&'a [u8], ProtocolError> {
        let end = self
            .offset
            .checked_add(length)
            .ok_or(ProtocolError::UnexpectedEof)?;
        let value = self
            .bytes
            .get(self.offset..end)
            .ok_or(ProtocolError::UnexpectedEof)?;
        self.offset = end;
        Ok(value)
    }

    fn take_u8(&mut self) -> Result<u8, ProtocolError> {
        Ok(self.take_exact(1)?[0])
    }

    fn take_u16(&mut self) -> Result<u16, ProtocolError> {
        let bytes: [u8; 2] = self
            .take_exact(2)?
            .try_into()
            .map_err(|_| ProtocolError::UnexpectedEof)?;
        Ok(u16::from_be_bytes(bytes))
    }

    fn take_u32(&mut self) -> Result<u32, ProtocolError> {
        let bytes: [u8; 4] = self
            .take_exact(4)?
            .try_into()
            .map_err(|_| ProtocolError::UnexpectedEof)?;
        Ok(u32::from_be_bytes(bytes))
    }

    fn take_u64(&mut self) -> Result<u64, ProtocolError> {
        let bytes: [u8; 8] = self
            .take_exact(8)?
            .try_into()
            .map_err(|_| ProtocolError::UnexpectedEof)?;
        Ok(u64::from_be_bytes(bytes))
    }

    fn take_string(&mut self) -> Result<String, ProtocolError> {
        let length = self.take_u16()? as usize;
        let bytes = self.take_exact(length)?.to_vec();
        String::from_utf8(bytes).map_err(|_| ProtocolError::InvalidUtf8)
    }

    fn finish(self) -> Result<(), ProtocolError> {
        if self.offset == self.bytes.len() {
            Ok(())
        } else {
            Err(ProtocolError::TrailingBytes)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolError {
    BadMagic,
    UnsupportedVersion,
    UnknownMessageKind,
    UnexpectedEof,
    InvalidUtf8,
    InvalidField,
    FieldTooLarge,
    FramePayloadTooLarge,
    TrailingBytes,
}

impl fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for ProtocolError {}
