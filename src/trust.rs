use std::collections::HashMap;
use std::fmt;

use crate::identity::{DeviceId, TrustLevel};

const SNAPSHOT_MAGIC: &[u8; 4] = b"GCT1";
const SNAPSHOT_VERSION: u16 = 1;
const MAX_TRUST_RECORDS: usize = 4096;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrustRecord {
    pub device_id: DeviceId,
    pub trust_level: TrustLevel,
    pub revision: u64,
    pub paired_at_ms: u64,
    pub revoked_at_ms: Option<u64>,
}

impl TrustRecord {
    pub fn new(
        device_id: DeviceId,
        trust_level: TrustLevel,
        revision: u64,
        paired_at_ms: u64,
    ) -> Result<Self, TrustError> {
        if revision == 0 || trust_level == TrustLevel::Untrusted {
            return Err(TrustError::InvalidRecord);
        }
        Ok(Self {
            device_id,
            trust_level,
            revision,
            paired_at_ms,
            revoked_at_ms: None,
        })
    }

    pub fn is_active(&self) -> bool {
        self.revoked_at_ms.is_none()
    }

    pub fn revoke(&mut self, revoked_at_ms: u64, revision: u64) -> Result<(), TrustError> {
        if revision <= self.revision {
            return Err(TrustError::StaleRevision);
        }
        self.revision = revision;
        self.revoked_at_ms = Some(revoked_at_ms);
        Ok(())
    }
}

pub trait PersistentTrustStore: fmt::Debug {
    fn lookup(&self, device_id: &DeviceId) -> Option<TrustRecord>;
    fn upsert(&mut self, record: TrustRecord) -> Result<(), TrustError>;
    fn revoke(
        &mut self,
        device_id: &DeviceId,
        revoked_at_ms: u64,
        revision: u64,
    ) -> Result<(), TrustError>;
    fn snapshot(&self) -> TrustSnapshot;
    fn restore(&mut self, snapshot: &TrustSnapshot) -> Result<(), TrustError>;
}

#[derive(Debug, Clone, Default)]
pub struct InMemoryTrustStore {
    records: HashMap<DeviceId, TrustRecord>,
    generation: u64,
}

impl InMemoryTrustStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_trusted(&self, device_id: &DeviceId) -> bool {
        self.records
            .get(device_id)
            .is_some_and(TrustRecord::is_active)
    }

    fn merge_record(&mut self, incoming: TrustRecord) -> Result<(), TrustError> {
        match self.records.get(&incoming.device_id) {
            None => {
                self.records.insert(incoming.device_id.clone(), incoming);
            }
            Some(current) if current.revoked_at_ms.is_some() && incoming.revoked_at_ms.is_none() => {
                return Ok(());
            }
            Some(current) if incoming.revoked_at_ms.is_some() => {
                let mut merged = incoming;
                if merged.revision <= current.revision {
                    merged.revision = current.revision.saturating_add(1);
                }
                self.records.insert(merged.device_id.clone(), merged);
            }
            Some(current) if incoming.revision > current.revision => {
                self.records.insert(incoming.device_id.clone(), incoming);
            }
            Some(current) if incoming.revision == current.revision && incoming == *current => {}
            Some(current) if incoming.revision == current.revision => {
                return Err(TrustError::ConflictingRevision);
            }
            Some(_) => return Err(TrustError::StaleRevision),
        }
        Ok(())
    }
}

impl PersistentTrustStore for InMemoryTrustStore {
    fn lookup(&self, device_id: &DeviceId) -> Option<TrustRecord> {
        self.records.get(device_id).cloned()
    }

    fn upsert(&mut self, record: TrustRecord) -> Result<(), TrustError> {
        self.merge_record(record)?;
        self.generation = self.generation.saturating_add(1);
        Ok(())
    }

    fn revoke(
        &mut self,
        device_id: &DeviceId,
        revoked_at_ms: u64,
        revision: u64,
    ) -> Result<(), TrustError> {
        let record = self
            .records
            .get_mut(device_id)
            .ok_or(TrustError::UnknownDevice)?;
        record.revoke(revoked_at_ms, revision)?;
        self.generation = self.generation.saturating_add(1);
        Ok(())
    }

    fn snapshot(&self) -> TrustSnapshot {
        let mut records: Vec<_> = self.records.values().cloned().collect();
        records.sort_by(|left, right| left.device_id.as_str().cmp(right.device_id.as_str()));
        TrustSnapshot {
            generation: self.generation,
            records,
        }
    }

    fn restore(&mut self, snapshot: &TrustSnapshot) -> Result<(), TrustError> {
        if snapshot.records.len() > MAX_TRUST_RECORDS {
            return Err(TrustError::TooManyRecords);
        }
        for record in &snapshot.records {
            self.merge_record(record.clone())?;
        }
        self.generation = self.generation.max(snapshot.generation);
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrustSnapshot {
    pub generation: u64,
    pub records: Vec<TrustRecord>,
}

impl TrustSnapshot {
    pub fn encode(&self) -> Result<Vec<u8>, TrustError> {
        if self.records.len() > MAX_TRUST_RECORDS {
            return Err(TrustError::TooManyRecords);
        }
        let mut output = Vec::new();
        output.extend_from_slice(SNAPSHOT_MAGIC);
        output.extend_from_slice(&SNAPSHOT_VERSION.to_be_bytes());
        output.extend_from_slice(&self.generation.to_be_bytes());
        output.extend_from_slice(&(self.records.len() as u32).to_be_bytes());
        for record in &self.records {
            push_string(&mut output, record.device_id.as_str())?;
            output.push(trust_level_to_wire(record.trust_level));
            output.extend_from_slice(&record.revision.to_be_bytes());
            output.extend_from_slice(&record.paired_at_ms.to_be_bytes());
            match record.revoked_at_ms {
                Some(value) => {
                    output.push(1);
                    output.extend_from_slice(&value.to_be_bytes());
                }
                None => output.push(0),
            }
        }
        Ok(output)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, TrustError> {
        let mut cursor = Cursor::new(bytes);
        if cursor.take_exact(4)? != SNAPSHOT_MAGIC {
            return Err(TrustError::MalformedSnapshot);
        }
        if cursor.take_u16()? != SNAPSHOT_VERSION {
            return Err(TrustError::UnsupportedSnapshotVersion);
        }
        let generation = cursor.take_u64()?;
        let count = cursor.take_u32()? as usize;
        if count > MAX_TRUST_RECORDS {
            return Err(TrustError::TooManyRecords);
        }
        let mut records = Vec::with_capacity(count);
        for _ in 0..count {
            let device_id = DeviceId::parse(cursor.take_string()?)
                .map_err(|_| TrustError::MalformedSnapshot)?;
            let trust_level = trust_level_from_wire(cursor.take_u8()?)?;
            let revision = cursor.take_u64()?;
            let paired_at_ms = cursor.take_u64()?;
            let revoked_at_ms = match cursor.take_u8()? {
                0 => None,
                1 => Some(cursor.take_u64()?),
                _ => return Err(TrustError::MalformedSnapshot),
            };
            if revision == 0 || trust_level == TrustLevel::Untrusted {
                return Err(TrustError::MalformedSnapshot);
            }
            records.push(TrustRecord {
                device_id,
                trust_level,
                revision,
                paired_at_ms,
                revoked_at_ms,
            });
        }
        cursor.finish()?;
        Ok(Self {
            generation,
            records,
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct DevelopmentTrustPersistence {
    snapshot_bytes: Option<Vec<u8>>,
}

impl DevelopmentTrustPersistence {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn persist(&mut self, store: &dyn PersistentTrustStore) -> Result<(), TrustError> {
        self.snapshot_bytes = Some(store.snapshot().encode()?);
        Ok(())
    }

    pub fn restore_into(&self, store: &mut dyn PersistentTrustStore) -> Result<(), TrustError> {
        let bytes = self
            .snapshot_bytes
            .as_ref()
            .ok_or(TrustError::NoSnapshot)?;
        let snapshot = TrustSnapshot::decode(bytes)?;
        store.restore(&snapshot)
    }
}

fn trust_level_to_wire(level: TrustLevel) -> u8 {
    match level {
        TrustLevel::Owner => 1,
        TrustLevel::Household => 2,
        TrustLevel::ApprovedDevice => 3,
        TrustLevel::Guest => 4,
        TrustLevel::Untrusted => 5,
    }
}

fn trust_level_from_wire(value: u8) -> Result<TrustLevel, TrustError> {
    match value {
        1 => Ok(TrustLevel::Owner),
        2 => Ok(TrustLevel::Household),
        3 => Ok(TrustLevel::ApprovedDevice),
        4 => Ok(TrustLevel::Guest),
        5 => Ok(TrustLevel::Untrusted),
        _ => Err(TrustError::MalformedSnapshot),
    }
}

fn push_string(output: &mut Vec<u8>, value: &str) -> Result<(), TrustError> {
    let length = u16::try_from(value.len()).map_err(|_| TrustError::MalformedSnapshot)?;
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

    fn take_exact(&mut self, length: usize) -> Result<&'a [u8], TrustError> {
        let end = self
            .offset
            .checked_add(length)
            .ok_or(TrustError::MalformedSnapshot)?;
        let value = self
            .bytes
            .get(self.offset..end)
            .ok_or(TrustError::MalformedSnapshot)?;
        self.offset = end;
        Ok(value)
    }

    fn take_u8(&mut self) -> Result<u8, TrustError> {
        Ok(self.take_exact(1)?[0])
    }

    fn take_u16(&mut self) -> Result<u16, TrustError> {
        Ok(u16::from_be_bytes(
            self.take_exact(2)?
                .try_into()
                .map_err(|_| TrustError::MalformedSnapshot)?,
        ))
    }

    fn take_u32(&mut self) -> Result<u32, TrustError> {
        Ok(u32::from_be_bytes(
            self.take_exact(4)?
                .try_into()
                .map_err(|_| TrustError::MalformedSnapshot)?,
        ))
    }

    fn take_u64(&mut self) -> Result<u64, TrustError> {
        Ok(u64::from_be_bytes(
            self.take_exact(8)?
                .try_into()
                .map_err(|_| TrustError::MalformedSnapshot)?,
        ))
    }

    fn take_string(&mut self) -> Result<String, TrustError> {
        let length = self.take_u16()? as usize;
        let bytes = self.take_exact(length)?.to_vec();
        String::from_utf8(bytes).map_err(|_| TrustError::MalformedSnapshot)
    }

    fn finish(self) -> Result<(), TrustError> {
        if self.offset == self.bytes.len() {
            Ok(())
        } else {
            Err(TrustError::MalformedSnapshot)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrustError {
    InvalidRecord,
    UnknownDevice,
    StaleRevision,
    ConflictingRevision,
    TooManyRecords,
    MalformedSnapshot,
    UnsupportedSnapshotVersion,
    NoSnapshot,
}

impl fmt::Display for TrustError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for TrustError {}
