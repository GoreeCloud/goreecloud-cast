# GoreeCloud Cast Specifications

Version: v0.1  
Status: Development

The current repository implementation is limited to the Phase 0 core contracts described in `docs/architecture/phase-0-core.md`. The full planned product specification is represented in `FEATURE-ROADMAP.md` v0.2.

Implemented repository contracts:

- Rust core package version `0.1.0`.
- Protocol schema version `0.1` constants.
- Receiver-first Cast-mode negotiation.
- Privacy-minimized unauthenticated discovery shape.
- Versioned, size-bounded State Capsule.
- Explicit State Capsule privacy classes.
- Fail-closed Identity/Privacy/Security decision consumption.
- Session state and state-authority types.
- Policy-constrained local-first transport selection.
- Unified handoff preflight and normalized failure reasons.

Not implemented: networking, pairing cryptography, persistent trust, receiver daemon, media playback/streaming, screen capture, Glaze UI, remote relay, Integral Platform System adapters, deployment, release, or Stable qualification.
