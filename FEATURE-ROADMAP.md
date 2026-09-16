---
title: "GoreeCloud Cast — Feature Roadmap"
product: "GoreeCloud Cast"
document_type: "Repository Feature Roadmap"
status: "Proposed / Planned"
version: "v0.5"
classification: "Internal"
implementation_status: "Phase 0 Development active; roadmap is not product completion evidence"
last_updated: "2026-09-16"
---

# GoreeCloud Cast — Feature Roadmap

This repository roadmap is materially synchronized with the GoreeCloud Drive roadmap under `GoreeCloud/Feature Roadmap/GoreeCloud Cast/goreecloud-cast.md`. The Drive record remains authoritative until this branch is reviewed and merged and roadmap authority is formally reconciled.

## Product direction

GoreeCloud Cast is intended to become a policy-aware, state-aware, transport-independent cross-device continuation platform. The governing design principle is **Transfer experiences between devices, not merely pixels.**

Preferred delivery order:

**Native receiver playback → direct media transfer → application streaming → display mirroring**

## Current implementation state

Phase 0 is active on draft PR #1 with implementation version `0.3.0` and protocol version `0.2`.

The current executable boundary includes capability negotiation, minimized unauthenticated discovery, authenticated receiver-detail models, bounded State Capsules, fail-closed consumption of Identity/Privacy/Security decisions, session authority types, local-first transport selection, handoff preflight, device-identity/pairing/session-credential adapter interfaces, deterministic protocol framing and typed payload serialization, command ordering/idempotency, an in-memory authenticated handshake harness, a concrete Development Ed25519 device-challenge provider, pairing confirmation state transitions, and expiring/revocable in-memory credential leases.

Exact source revision `2bab9042542937efbf96fa9539031a3ba1c4ded9` passed GoreeCloud Cast Core CI run `35158442444`, including `cargo fmt --check` and `cargo test --all-targets`. This is Development source/test evidence for the bounded core only.

Phase 0 is **not complete**. Approved production key storage/rotation/recovery, complete secure pairing runtime, persistent trust, authenticated network transport, receiver/controller endpoint processes, UI, deployed platform adapters, runtime acceptance, deployment, release, and Stable qualification remain unimplemented or unverified.

## Planned phases

1. **Architecture and Security Foundation** — threat model, device identity, pairing, trust, permissions, session authority, discovery protocol, capability schema, transport abstraction, receiver interface, API contracts, Experience Handoff Contract, State Capsule, error model, tests, and Glaze interaction prototype.
2. **Core Local Cast** — local discovery/pairing, trusted-device storage, media handoff, basic playback control, metadata, queue, picker, receiver confirmation, and reconnection.
3. **Session Continuity** — persistent sessions, system controller, controller migration, multiple controllers, queue synchronization, `Continue on…`, receiver-authoritative playback state, command ordering/idempotency, and recovery.
4. **Audio Cast** — application/device audio routing, audio-only receivers, latency management, and background operation.
5. **Application and Display Casting** — application/window/display casting, virtual displays, adaptive quality, notification privacy, protected surfaces, and persistent privacy indicators.
6. **Receiver Groups** — synchronized multi-room audio, drift correction, group and per-device volume, calibration, and dynamic membership.
7. **Advanced Multi-Display** — synchronized visual receiver groups, presentations, dashboards, signage, and advanced diagnostics.
8. **Guest Cast** — temporary pairing, expiring credentials, bounded permissions, approval, cleanup, and administration.
9. **Remote Cast** — trusted cross-network receivers, optional minimized relay, strict remote permissions, indicators, revocation, and auditing after local trust/recovery maturity.
10. **Developer Platform** — stable controller/receiver APIs, integration library, receiver extensions, samples, compatibility suite, developer documentation, and automated conformance tests.
11. **Ecosystem Integration and Hardening** — unified iconography, shared device picker/controller/settings, performance, accessibility, security, recovery, long-duration, and interoperability validation.
12. **Experience Handoff Contract** — portable versioned experience state and deterministic preflight across compatible applications.
13. **Integral Platform System Integration** — evidence-backed integration with exactly seven systems: Manager, Privacy Shield, Wardveil Security, Everkeep, Glaze UI, Mesh, and Identity. GoreeCloud Sync remains separately governed.
14. **Cross-Application Handoff** — shared continuation across appropriate GoreeCloud Music, Video, Photos, Gallery, Browser, Reader, Documents, Home, Location, and Notify surfaces.
15. **Protocol Hardening and Conformance** — version negotiation, malicious participant handling, protected content, mixed-version interoperability, conformance kits, and capability-level maturity states.

## Verified Phase 0 progress — September 16, 2026

### Initial bounded core

Established receiver-first mode selection, minimized unauthenticated discovery, bounded State Capsules, explicit session authority, fail-closed platform gate consumption, local-first transport policy/selection, unified handoff preflight, mandatory repository documentation, and Contract 0.2 platform declaration.

### Implementation version 0.2.0

Added:

- device-identity signing/verifier adapter traits without embedded cryptography;
- pairing authorization and opaque session-credential authority interfaces;
- authenticated discovery-detail models tied to rotating discovery identifiers;
- deterministic protocol `0.2` frames with bounded payloads and strict version/trailing-byte validation;
- typed serialization for authenticated discovery details and session opening;
- per-session command sequence/idempotency primitives with duplicate recognition, conflicting-replay rejection, wrong-session rejection, out-of-order rejection, and sequence exhaustion handling; and
- an in-memory controller/receiver handshake harness that orders identity verification before authenticated receiver-detail release, then pairing authorization, credential issuance/validation, protocol round trips, and session activation.

The first CI attempt for this increment stopped at rustfmt-only drift. Exact source `c421ce127059a0a1cd8ba33740e9c2188bbf58a3` then passed run `35157077714`.

### Implementation version 0.3.0

Added:

- pinned `ed25519-dalek` 3.0.0 as the first material third-party runtime dependency, documented under `THIRD-PARTY-NOTICES.md` with BSD-3-Clause provenance and replacement/security boundaries;
- a concrete Development Ed25519 device-challenge signer accepting externally supplied secret key bytes;
- a public-key registry/verifier that rejects weak keys, unknown devices, malformed signatures, and signatures failing strict Ed25519 verification;
- domain-separated challenge messages bound to Cast device ID, challenge context, and nonce;
- a pairing-confirmation state machine with explicit deadline, confirm, deny, cancel, expiry, finalized-state, and controller/receiver grant-binding behavior;
- in-memory session-credential leases with explicit issue/expiry timestamps, revocation state, unknown-handle rejection, and binding-substitution rejection; and
- executable tests proving signature/tamper behavior, unknown-device fail-closed behavior, pairing expiry/finalization, grant-binding validation, credential expiry/revocation, and credential-binding isolation.

The first CI run for this increment, `35158343289`, stopped at rustfmt-only drift before tests. That formatting-only diff was applied without behavioral changes. Exact source `2bab9042542937efbf96fa9539031a3ba1c4ded9` then passed run `35158442444` at 2026-09-16T22:36:00Z.

A temporary empty placeholder commit was accidentally created while moving the feature ref during this increment. It was immediately displaced by resetting the feature branch to the intended implementation commit before validation; it is not present on the current PR branch and is not part of the implementation state.

No production license grant is inferred from public repository visibility; package publication remains disabled and the repository rights notice does not grant reuse rights absent separate GoreeCloud terms.

## Active next boundary

Continue Phase 0 from the verified `0.3.0` security-provider boundary with:

- an authenticated local frame-transport prototype carrying protocol `0.2` frames;
- a revocation-safe persistent trust-store interface and Development persistence harness;
- separate controller and receiver endpoint/process-level harnesses rather than one in-memory orchestration object;
- integration of credential lease validation and trust state into session establishment/recovery;
- malformed-frame, interrupted-handshake, trust-revocation, credential-revocation, restart, and recovery tests;
- target-platform device-key storage/rotation interfaces without embedding secrets in Cast configuration;
- continued Identity, Privacy Shield, Wardveil Security, Everkeep, Mesh, Manager, and Glaze UI integration work without claiming runtime conformance before evidence exists.

None of these next items should be represented as production-ready until concrete providers, target-environment behavior, failure modes, and platform acceptance are verified.
