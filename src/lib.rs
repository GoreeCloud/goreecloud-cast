#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]

//! GoreeCloud Cast Phase 0 core contracts.
//!
//! This crate intentionally contains no network stack, media codec, persistent
//! trust store, or platform-service implementation. Production cryptographic
//! operations are isolated behind explicit provider boundaries.

pub mod auth;
pub mod capability;
pub mod command;
pub mod credentials;
pub mod crypto;
pub mod discovery;
pub mod handoff;
pub mod handshake;
pub mod identity;
pub mod pairing;
pub mod protocol;
pub mod session;
pub mod state_capsule;
pub mod transport;

pub const IMPLEMENTATION_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PROTOCOL_MAJOR: u16 = 0;
pub const PROTOCOL_MINOR: u16 = 2;
