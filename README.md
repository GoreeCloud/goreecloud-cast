# GoreeCloud Cast

GoreeCloud Cast is the planned cross-device experience transport layer for the GoreeCloud ecosystem: **continue this experience there**, rather than merely mirror pixels.

## Current implementation state

**Development — Phase 0 foundation.** The native Rust core now covers handoff contracts, capability negotiation, privacy-minimized discovery, authenticated receiver-detail models, bounded State Capsules, fail-closed platform decisions, session authority types, local-first transport selection, identity/pairing/session-credential adapter interfaces, deterministic protocol framing, command ordering/idempotency, and an in-memory controller/receiver handshake harness.

Implementation version `0.3.0` adds a concrete Development Ed25519 device-challenge provider backed by pinned `ed25519-dalek` 3.0.0, strict public-key/signature verification, an explicit pairing-confirmation state machine with expiry/cancel/deny behavior, and in-memory session-credential lease semantics for expiry, revocation, and binding-substitution rejection.

These additions are still bounded Development evidence. The repository does not yet provide approved production key storage/rotation, a complete secure pairing runtime, persistent trust, authenticated network transport, receiver/controller endpoint processes, media streaming, mirroring, remote casting, Glaze UI, deployed Integral Platform System adapters, production acceptance, release, or Stable qualification.

Implementation version: **0.3.0**  
Protocol version: **0.2**  
Roadmap version: **v0.5**  
Platform contract target: **0.2**

## Build and test

```bash
cargo test --all-targets
cargo fmt --check
```

## Repository map

- `src/` — native Cast core contracts and Development security/provider boundaries.
- `tests/` — executable core, protocol, sequencing, identity, pairing, credential-lease, and in-memory handshake contract tests.
- `docs/architecture/` — repository-coupled architecture records.
- `FEATURE-ROADMAP.md` — synchronized roadmap representation for the current Development branch.
- `THIRD-PARTY-NOTICES.md` — material dependency purpose, provenance, license, security boundary, and replacement considerations.
- `goreecloud.platform.yaml` — truthful Platform Contract declaration; integrations remain incomplete until runtime evidence exists.

## Design boundaries

Local-first operation, receiver-first playback, explicit trust, minimum required access, durable session continuity, visible sharing state, and independent platform authority are foundational requirements. Discovery is not authorization. Authentication is not privacy consent. Reachability is not trust. A cryptographic signature provider is not by itself a trusted key lifecycle, secure pairing flow, or authenticated transport.
