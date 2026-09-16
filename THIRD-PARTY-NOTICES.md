# GoreeCloud Cast — Third-Party Dependency Record

Version: v0.1  
Status: Development

## ed25519-dalek 3.0.0

- Purpose: Ed25519 signing and strict verification for GoreeCloud Cast device-identity challenge proofs.
- Version policy: pinned exactly to `3.0.0` for this Development increment.
- License: BSD-3-Clause.
- Authoritative upstream: `dalek-cryptography/curve25519-dalek`, `ed25519-dalek` crate.
- Maintenance/update source: crates.io release metadata and the authoritative upstream repository.
- Architectural boundary: the dependency is used only by `src/crypto.rs` behind the existing `DeviceIdentitySigner` and `DeviceIdentityVerifier` contracts. Cast does not use its hazardous or legacy-compatibility APIs.
- Key-management boundary: Cast does not generate, persist, recover, rotate, synchronize, or remotely escrow production device private keys in this increment. `Ed25519DeviceSigner::from_secret_bytes` accepts key material supplied by a higher-level approved key-management boundary.
- Verification policy: registered public keys are rejected when weak, and challenge signatures are checked using strict verification.
- Privacy/security implications: public verification keys identify trusted device identities and therefore must not be exposed through unauthenticated discovery. Private key bytes are security-sensitive and must be supplied and protected by an approved platform key store or equivalent provider before production use.
- Replacement considerations: the Cast domain layer is not coupled to Ed25519-specific APIs; alternative approved cryptographic providers can implement the same identity traits if migration becomes necessary.
- Production qualification: this dependency and wrapper are Development evidence only until target-platform key storage, rotation/revocation, runtime isolation, dependency/security review, and Identity/Wardveil acceptance are verified.

No dependency record changes the repository's own licensing status or grants rights to GoreeCloud Cast source code.
