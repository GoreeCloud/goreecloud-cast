use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandEnvelope {
    pub session_id: String,
    pub sequence: u64,
    pub idempotency_key: String,
    pub operation_code: u16,
    pub payload: Vec<u8>,
}

impl CommandEnvelope {
    pub fn new(
        session_id: impl Into<String>,
        sequence: u64,
        idempotency_key: impl Into<String>,
        operation_code: u16,
        payload: Vec<u8>,
    ) -> Result<Self, CommandError> {
        let session_id = session_id.into();
        if !(16..=128).contains(&session_id.len())
            || session_id.chars().any(char::is_control)
        {
            return Err(CommandError::InvalidSessionId);
        }
        if sequence == 0 {
            return Err(CommandError::InvalidSequence);
        }
        let idempotency_key = idempotency_key.into();
        if !(8..=128).contains(&idempotency_key.len())
            || idempotency_key.chars().any(char::is_control)
        {
            return Err(CommandError::InvalidIdempotencyKey);
        }
        if payload.len() > 64 * 1024 {
            return Err(CommandError::PayloadTooLarge);
        }
        Ok(Self {
            session_id,
            sequence,
            idempotency_key,
            operation_code,
            payload,
        })
    }

    fn fingerprint(&self) -> Vec<u8> {
        let mut fingerprint = Vec::with_capacity(2 + self.payload.len());
        fingerprint.extend_from_slice(&self.operation_code.to_be_bytes());
        fingerprint.extend_from_slice(&self.payload);
        fingerprint
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandDisposition {
    Applied,
    Duplicate,
    OutOfOrder { expected: u64, received: u64 },
    ConflictingReplay,
    WrongSession,
    SequenceExhausted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandSequencer {
    session_id: String,
    next_sequence: u64,
    exhausted: bool,
    seen: BTreeMap<String, Vec<u8>>,
}

impl CommandSequencer {
    pub fn new(session_id: impl Into<String>) -> Result<Self, CommandError> {
        let session_id = session_id.into();
        if !(16..=128).contains(&session_id.len())
            || session_id.chars().any(char::is_control)
        {
            return Err(CommandError::InvalidSessionId);
        }
        Ok(Self {
            session_id,
            next_sequence: 1,
            exhausted: false,
            seen: BTreeMap::new(),
        })
    }

    pub fn next_sequence(&self) -> Option<u64> {
        (!self.exhausted).then_some(self.next_sequence)
    }

    pub fn accept(&mut self, command: &CommandEnvelope) -> CommandDisposition {
        if command.session_id != self.session_id {
            return CommandDisposition::WrongSession;
        }

        let fingerprint = command.fingerprint();
        if let Some(existing) = self.seen.get(&command.idempotency_key) {
            return if *existing == fingerprint {
                CommandDisposition::Duplicate
            } else {
                CommandDisposition::ConflictingReplay
            };
        }

        if self.exhausted {
            return CommandDisposition::SequenceExhausted;
        }
        if command.sequence != self.next_sequence {
            return CommandDisposition::OutOfOrder {
                expected: self.next_sequence,
                received: command.sequence,
            };
        }

        self.seen
            .insert(command.idempotency_key.clone(), fingerprint);
        match self.next_sequence.checked_add(1) {
            Some(next) => self.next_sequence = next,
            None => self.exhausted = true,
        }
        CommandDisposition::Applied
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandError {
    InvalidSessionId,
    InvalidSequence,
    InvalidIdempotencyKey,
    PayloadTooLarge,
}
