#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]

//! GoreeCloud Cast Phase 0 core contracts.
//!
//! This crate now includes Development-only authenticated local frame transport,
//! revocation-safe trust persistence, and endpoint harnesses. It still contains
//! no production network stack, media codec, persistent credential service, or
//! accepted platform-service runtime integration.

pub mod auth;
pub mod capability;
pub mod command;
pub mod credentials;
pub mod crypto;
pub mod discovery;
pub mod endpoints;
pub mod handoff;
pub mod handshake;
pub mod identity;
pub mod key_management;
pub mod local_transport;
pub mod pairing;
pub mod protocol;
pub mod session;
pub mod state_capsule;
pub mod transport;
pub mod trust;

pub const IMPLEMENTATION_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PROTOCOL_MAJOR: u16 = 0;
pub const PROTOCOL_MINOR: u16 = 2;
