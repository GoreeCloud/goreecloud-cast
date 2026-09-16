# GoreeCloud Cast Specifications

Version: v0.2  
Status: Development

The current repository implementation is limited to the Phase 0 core contracts described in `docs/architecture/phase-0-core.md`. The full planned product specification is represented in `FEATURE-ROADMAP.md` v0.4 and synchronized with the authoritative Drive roadmap.

Implemented repository contracts:

- Rust core package version `0.2.0`.
- Protocol schema constants at version `0.2`.
- Receiver-first Cast-mode negotiation.
- Privacy-minimized unauthenticated discovery shape.
- Authenticated receiver-detail model with identity-gated release in the in-memory handshake harness.
- Versioned, size-bounded State Capsule and explicit privacy classes.
- Fail-closed Identity/Privacy/Security decision consumption.
- Session state and state-authority types.
- Policy-constrained local-first transport selection.
- Unified handoff preflight and normalized failure reasons.
- Device-identity signing/verifying adapter contracts.
- Pairing authorization and opaque session-credential authority contracts.
- Deterministic protocol framing and typed discovery/session-open payload serialization.
- Per-session command sequencing, idempotency, duplicate handling, conflicting-replay rejection, and out-of-order rejection.
- In-memory handshake harness that validates the order of identity verification, authenticated detail exchange, pairing authorization, credential issuance/validation, protocol round trips, and session activation.

These interfaces and tests do not implement production cryptography or networking. Not implemented: cryptographic Identity provider integration, production pairing, session-secret derivation or transport encryption, persistent trust/revocation storage, receiver/controller processes, network transport, media playback/streaming, screen capture, Glaze UI, remote relay, deployed Integral Platform System adapters, deployment, release, or Stable qualification.
