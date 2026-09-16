use std::collections::{HashMap, VecDeque};
use std::fmt;

use crate::{
    auth::{AdapterError, FrameSigner, FrameVerifier},
    identity::DeviceId,
    protocol::{MAX_FRAME_PAYLOAD_BYTES, ProtocolError, ProtocolFrame},
};

const TRANSPORT_MAGIC: &[u8; 4] = b"GCL1";
const TRANSPORT_VERSION: u16 = 1;
const MAX_SIGNATURE_BYTES: usize = 4096;
const MAX_PACKET_BYTES: usize = MAX_FRAME_PAYLOAD_BYTES + 8192;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthenticatedLocalPacket {
    pub sender: DeviceId,
    pub receiver: DeviceId,
    pub transport_sequence: u64,
    pub frame_bytes: Vec<u8>,
    pub signature: Vec<u8>,
}

impl AuthenticatedLocalPacket {
    pub fn encode(&self) -> Result<Vec<u8>, LocalTransportError> {
        if self.transport_sequence == 0
            || self.frame_bytes.len() > MAX_PACKET_BYTES
            || !(16..=MAX_SIGNATURE_BYTES).contains(&self.signature.len())
        {
            return Err(LocalTransportError::MalformedPacket);
        }
        let mut output = Vec::new();
        output.extend_from_slice(TRANSPORT_MAGIC);
        output.extend_from_slice(&TRANSPORT_VERSION.to_be_bytes());
        push_string(&mut output, self.sender.as_str())?;
        push_string(&mut output, self.receiver.as_str())?;
        output.extend_from_slice(&self.transport_sequence.to_be_bytes());
        output.extend_from_slice(&(self.frame_bytes.len() as u32).to_be_bytes());
        output.extend_from_slice(&self.frame_bytes);
        output.extend_from_slice(&(self.signature.len() as u16).to_be_bytes());
        output.extend_from_slice(&self.signature);
        if output.len() > MAX_PACKET_BYTES {
            return Err(LocalTransportError::PacketTooLarge);
        }
        Ok(output)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, LocalTransportError> {
        if bytes.len() > MAX_PACKET_BYTES {
            return Err(LocalTransportError::PacketTooLarge);
        }
        let mut cursor = Cursor::new(bytes);
        if cursor.take_exact(4)? != TRANSPORT_MAGIC {
            return Err(LocalTransportError::MalformedPacket);
        }
        if cursor.take_u16()? != TRANSPORT_VERSION {
            return Err(LocalTransportError::UnsupportedTransportVersion);
        }
        let sender = DeviceId::parse(cursor.take_string()?)
            .map_err(|_| LocalTransportError::MalformedPacket)?;
        let receiver = DeviceId::parse(cursor.take_string()?)
            .map_err(|_| LocalTransportError::MalformedPacket)?;
        let transport_sequence = cursor.take_u64()?;
        if transport_sequence == 0 {
            return Err(LocalTransportError::MalformedPacket);
        }
        let frame_len = cursor.take_u32()? as usize;
        if frame_len > MAX_PACKET_BYTES {
            return Err(LocalTransportError::PacketTooLarge);
        }
        let frame_bytes = cursor.take_exact(frame_len)?.to_vec();
        let signature_len = cursor.take_u16()? as usize;
        if !(16..=MAX_SIGNATURE_BYTES).contains(&signature_len) {
            return Err(LocalTransportError::MalformedPacket);
        }
        let signature = cursor.take_exact(signature_len)?.to_vec();
        cursor.finish()?;
        Ok(Self {
            sender,
            receiver,
            transport_sequence,
            frame_bytes,
            signature,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenedLocalFrame {
    pub sender: DeviceId,
    pub transport_sequence: u64,
    pub frame: ProtocolFrame,
}

#[derive(Debug)]
pub struct LocalFrameTransmitter<'a> {
    sender: DeviceId,
    signer: &'a dyn FrameSigner,
    next_sequence: u64,
}

impl<'a> LocalFrameTransmitter<'a> {
    pub fn new(sender: DeviceId, signer: &'a dyn FrameSigner) -> Self {
        Self {
            sender,
            signer,
            next_sequence: 1,
        }
    }

    pub fn seal(
        &mut self,
        receiver: &DeviceId,
        frame: &ProtocolFrame,
    ) -> Result<Vec<u8>, LocalTransportError> {
        let sequence = self.next_sequence;
        let frame_bytes = frame.encode();
        let signature = self.signer.sign_frame(
            &self.sender,
            receiver,
            sequence,
            &frame_bytes,
        )?;
        let packet = AuthenticatedLocalPacket {
            sender: self.sender.clone(),
            receiver: receiver.clone(),
            transport_sequence: sequence,
            frame_bytes,
            signature,
        };
        let bytes = packet.encode()?;
        self.next_sequence = self
            .next_sequence
            .checked_add(1)
            .ok_or(LocalTransportError::SequenceExhausted)?;
        Ok(bytes)
    }
}

#[derive(Debug)]
pub struct LocalFrameReceiver<'a> {
    receiver: DeviceId,
    verifier: &'a dyn FrameVerifier,
    next_by_sender: HashMap<DeviceId, u64>,
}

impl<'a> LocalFrameReceiver<'a> {
    pub fn new(receiver: DeviceId, verifier: &'a dyn FrameVerifier) -> Self {
        Self {
            receiver,
            verifier,
            next_by_sender: HashMap::new(),
        }
    }

    pub fn open(&mut self, bytes: &[u8]) -> Result<OpenedLocalFrame, LocalTransportError> {
        let packet = AuthenticatedLocalPacket::decode(bytes)?;
        if packet.receiver != self.receiver {
            return Err(LocalTransportError::WrongReceiver);
        }
        if !self.verifier.verify_frame(
            &packet.sender,
            &packet.receiver,
            packet.transport_sequence,
            &packet.frame_bytes,
            &packet.signature,
        )? {
            return Err(LocalTransportError::AuthenticationRejected);
        }

        let expected = self.next_by_sender.entry(packet.sender.clone()).or_insert(1);
        if packet.transport_sequence < *expected {
            return Err(LocalTransportError::Replay);
        }
        if packet.transport_sequence > *expected {
            return Err(LocalTransportError::OutOfOrder {
                expected: *expected,
                received: packet.transport_sequence,
            });
        }

        let frame = ProtocolFrame::decode(&packet.frame_bytes)?;
        *expected = expected
            .checked_add(1)
            .ok_or(LocalTransportError::SequenceExhausted)?;
        Ok(OpenedLocalFrame {
            sender: packet.sender,
            transport_sequence: packet.transport_sequence,
            frame,
        })
    }
}

#[derive(Debug, Default)]
pub struct InMemoryLocalLink {
    queues: HashMap<DeviceId, VecDeque<Vec<u8>>>,
}

impl InMemoryLocalLink {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn send_raw(&mut self, receiver: DeviceId, packet: Vec<u8>) {
        self.queues.entry(receiver).or_default().push_back(packet);
    }

    pub fn receive_raw(&mut self, receiver: &DeviceId) -> Option<Vec<u8>> {
        self.queues.get_mut(receiver).and_then(VecDeque::pop_front)
    }

    pub fn pending_for(&self, receiver: &DeviceId) -> usize {
        self.queues.get(receiver).map_or(0, VecDeque::len)
    }
}

fn push_string(output: &mut Vec<u8>, value: &str) -> Result<(), LocalTransportError> {
    let length = u16::try_from(value.len()).map_err(|_| LocalTransportError::MalformedPacket)?;
    output.extend_from_slice(&length.to_be_bytes());
    output.extend_from_slice(value.as_bytes());
    Ok(())
}

struct Cursor<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Cursor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    fn take_exact(&mut self, length: usize) -> Result<&'a [u8], LocalTransportError> {
        let end = self
            .offset
            .checked_add(length)
            .ok_or(LocalTransportError::MalformedPacket)?;
        let value = self
            .bytes
            .get(self.offset..end)
            .ok_or(LocalTransportError::MalformedPacket)?;
        self.offset = end;
        Ok(value)
    }

    fn take_u16(&mut self) -> Result<u16, LocalTransportError> {
        Ok(u16::from_be_bytes(
            self.take_exact(2)?
                .try_into()
                .map_err(|_| LocalTransportError::MalformedPacket)?,
        ))
    }

    fn take_u32(&mut self) -> Result<u32, LocalTransportError> {
        Ok(u32::from_be_bytes(
            self.take_exact(4)?
                .try_into()
                .map_err(|_| LocalTransportError::MalformedPacket)?,
        ))
    }

    fn take_u64(&mut self) -> Result<u64, LocalTransportError> {
        Ok(u64::from_be_bytes(
            self.take_exact(8)?
                .try_into()
                .map_err(|_| LocalTransportError::MalformedPacket)?,
        ))
    }

    fn take_string(&mut self) -> Result<String, LocalTransportError> {
        let length = self.take_u16()? as usize;
        let bytes = self.take_exact(length)?.to_vec();
        String::from_utf8(bytes).map_err(|_| LocalTransportError::MalformedPacket)
    }

    fn finish(self) -> Result<(), LocalTransportError> {
        if self.offset == self.bytes.len() {
            Ok(())
        } else {
            Err(LocalTransportError::MalformedPacket)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalTransportError {
    MalformedPacket,
    UnsupportedTransportVersion,
    PacketTooLarge,
    WrongReceiver,
    AuthenticationRejected,
    Replay,
    OutOfOrder { expected: u64, received: u64 },
    SequenceExhausted,
    AdapterFailure,
    ProtocolFailure,
}

impl From<AdapterError> for LocalTransportError {
    fn from(_: AdapterError) -> Self {
        Self::AdapterFailure
    }
}

impl From<ProtocolError> for LocalTransportError {
    fn from(_: ProtocolError) -> Self {
        Self::ProtocolFailure
    }
}

impl fmt::Display for LocalTransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for LocalTransportError {}
