# GoreeCloud Cast

GoreeCloud Cast is the planned cross-device experience transport layer for the GoreeCloud ecosystem: **continue this experience there**, rather than merely mirror pixels.

## Current implementation state

**Development — Phase 0 foundation.** This repository now contains an initial native Rust core for handoff contracts, capability negotiation, privacy-minimized discovery, State Capsules, fail-closed platform decisions, session authority types, and local-first transport selection. It does not yet provide a usable Cast receiver, media streaming, mirroring, remote casting, production cryptography, or Stable platform integration.

Implementation version: **0.1.0**  
Roadmap version: **v0.2**  
Platform contract target: **0.2**

## Build and test

```bash
cargo test --all-targets
cargo fmt --check
```

## Repository map

- `src/` — native Cast core contracts.
- `tests/` — executable contract tests.
- `docs/architecture/` — repository-coupled architecture records.
- `FEATURE-ROADMAP.md` — synchronized roadmap representation for the current Development branch.
- `goreecloud.platform.yaml` — truthful Platform Contract declaration; integrations remain incomplete until evidence exists.

## Design boundaries

Local-first operation, receiver-first playback, explicit trust, minimum required access, durable session continuity, visible sharing state, and independent platform authority are foundational requirements. Discovery is not authorization. Authentication is not privacy consent. Reachability is not trust.
