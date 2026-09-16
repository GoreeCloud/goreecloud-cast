# GoreeCloud Cast

GoreeCloud Cast is the planned cross-device experience transport layer for the GoreeCloud ecosystem: **continue this experience there**, rather than merely mirror pixels.

## Current implementation state

**Development — Phase 0 foundation.** The native Rust core now covers handoff contracts, capability negotiation, privacy-minimized discovery, authenticated receiver-detail models, bounded State Capsules, fail-closed platform decisions, session authority types, local-first transport selection, identity/pairing/session-credential adapter interfaces, deterministic protocol framing, command ordering/idempotency, and an in-memory controller/receiver handshake harness.

The harness and adapter traits are Development architecture evidence only. This repository does not yet provide production cryptography, a secure pairing implementation, durable trust, a usable Cast receiver, real network transport, media streaming, mirroring, remote casting, Glaze UI, deployed Integral Platform System adapters, or Stable platform integration.

Implementation version: **0.2.0**  
Roadmap version: **v0.4**  
Platform contract target: **0.2**

## Build and test

```bash
cargo test --all-targets
cargo fmt --check
```

## Repository map

- `src/` — native Cast core contracts and Development adapter boundaries.
- `tests/` — executable core, protocol, sequencing, and in-memory handshake contract tests.
- `docs/architecture/` — repository-coupled architecture records.
- `FEATURE-ROADMAP.md` — synchronized roadmap representation for the current Development branch.
- `goreecloud.platform.yaml` — truthful Platform Contract declaration; integrations remain incomplete until runtime evidence exists.

## Design boundaries

Local-first operation, receiver-first playback, explicit trust, minimum required access, durable session continuity, visible sharing state, and independent platform authority are foundational requirements. Discovery is not authorization. Authentication is not privacy consent. Reachability is not trust. Adapter interfaces are not production security implementations.
