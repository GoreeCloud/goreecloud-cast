# GoreeCloud Cast Specifications

Version: v0.3  
Status: Development

The current repository implementation is limited to the Phase 0 contracts described in `docs/architecture/phase-0-core.md`. The full planned product specification is represented in `FEATURE-ROADMAP.md` v0.5.

Implemented repository contracts:

- Rust core package version `0.3.0`.
- Protocol schema version `0.2` constants.
- Receiver-first Cast-mode negotiation.
- Privacy-minimized unauthenticated discovery shape.
- Authenticated receiver-detail model.
- Versioned, size-bounded State Capsule and explicit transfer privacy classes.
- Fail-closed Identity/Privacy/Security decision consumption.
- Session state and state-authority types.
- Policy-constrained local-first transport selection.
- Unified handoff preflight and normalized failure reasons.
- Device-identity signer/verifier interfaces.
- Concrete Development Ed25519 challenge signer/verifier using pinned `ed25519-dalek` 3.0.0 and strict verification.
- Pairing authorization contracts and explicit pairing confirmation state transitions with deadlines.
- Session-credential authority contracts and in-memory credential lease expiry/revocation semantics.
- Deterministic protocol framing and typed payload serialization.
- Command sequencing/idempotency primitives.
- In-memory controller/receiver handshake orchestration and executable contract tests.

Not implemented: approved production key storage/rotation/recovery, full secure pairing UX/runtime, persistent trust storage, authenticated network transport, receiver/controller endpoint processes, media playback/streaming, screen capture, Glaze UI, remote relay, accepted Integral Platform System runtime adapters, deployment, release, or Stable qualification.
