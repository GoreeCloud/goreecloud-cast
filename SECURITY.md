# GoreeCloud Cast — Security

Version: v0.3  
Status: Development

## Current security boundary

The Phase 0 core remains a bounded Development library with no production sockets, filesystem trust persistence, codec parsing, process sandboxing, or deployed platform-service adapter. It consumes platform authorization decisions and fails closed when Identity, Privacy Shield, or Wardveil Security decisions are denied or unknown.

Restricted State Capsule entries cannot be transferred through the implemented API. Remote transports remain disabled by the default local-only policy.

Version `0.3.0` adds a concrete Ed25519 device-challenge signer/verifier using pinned `ed25519-dalek` 3.0.0. The provider domain-separates and binds signatures to the GoreeCloud Cast device identifier, challenge context, and nonce; registered weak public keys are rejected; verification uses strict Ed25519 verification. The dependency is documented in `THIRD-PARTY-NOTICES.md` and remains behind the existing identity traits.

Version `0.3.0` also adds pairing confirmation states with explicit expiration/cancel/deny/finalization behavior and an in-memory credential-lease store that rejects expired, revoked, unknown, and binding-substituted credentials.

These controls do **not** establish a production device-key lifecycle. Private key generation, secure storage, hardware-backed protection, rotation, recovery, revocation distribution, trust persistence, and target-platform isolation remain outside this increment.

## Not yet implemented

Approved production device-key storage and rotation, complete secure pairing, session-secret derivation and key rotation, encrypted/authenticated network transport, persistent trust and revocation-safe restore, malicious receiver/controller process isolation, denial-of-service controls, deployed Identity/Privacy Shield/Wardveil integrations, and production audit evidence remain open requirements.

Do not report vulnerabilities through public issues when disclosure would expose sensitive exploit details or credentials; use the private security-reporting path configured for the repository when available.
