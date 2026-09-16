#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]

//! GoreeCloud Cast Phase 0 core contracts.
//!
//! This crate intentionally contains no network stack, media codec, production
//! cryptography, persistent trust store, or platform-service implementation. It
//! establishes fail-closed, transport-independent contracts and adapter boundaries
//! that later runtime implementations can use.

pub mod auth;
pub mod capability;
pub mod command;
pub mod discovery;
pub mod handoff;
pub mod handshake;
pub mod identity;
pub mod protocol;
pub mod session;
pub mod state_capsule;
pub mod transport;

pub const IMPLEMENTATION_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PROTOCOL_MAJOR: u16 = 0;
pub const PROTOCOL_MINOR: u16 = 2;
