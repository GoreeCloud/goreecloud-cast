---
title: "GoreeCloud Cast — Feature Roadmap"
product: "GoreeCloud Cast"
document_type: "Repository Feature Roadmap"
status: "Proposed / Planned"
version: "v0.3"
classification: "Internal"
implementation_status: "Phase 0 Development started; roadmap is not product completion evidence"
last_updated: "2026-09-16"
---

# GoreeCloud Cast — Feature Roadmap

This repository roadmap is materially synchronized with the GoreeCloud Drive roadmap under `GoreeCloud/Feature Roadmap/GoreeCloud Cast/goreecloud-cast.md`. The Drive record remains authoritative until this branch is reviewed and merged and roadmap authority is formally reconciled.

## Product direction

GoreeCloud Cast is intended to become a policy-aware, state-aware, transport-independent cross-device continuation platform. The governing design principle is **Transfer experiences between devices, not merely pixels.**

Preferred delivery order:

**Native receiver playback → direct media transfer → application streaming → display mirroring**

## Current implementation state

Phase 0 has started on draft PR #1 with implementation version `0.1.0`. The initial executable boundary covers capability negotiation, minimal discovery advertisements, bounded State Capsules, fail-closed consumption of Identity/Privacy/Security decisions, session authority types, local-first transport selection, and handoff preflight.

Exact source revision `f165832a3e2e0be546415672a916f706e21890b8` passed GoreeCloud Cast Core CI run `35155854625`, including `cargo fmt --check` and `cargo test --all-targets`. This is Development source/test evidence for the bounded core only.

Phase 0 is **not complete**. Production cryptography, network transport, persistent trust, receiver/controller processes, UI, platform adapters, runtime acceptance, deployment, release, and Stable qualification remain unimplemented or unverified.

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

## Active next boundary

Complete the remaining Phase 0 core foundation by adding cryptographic identity adapter interfaces, secure pairing/session credential interfaces, deterministic protocol serialization, authenticated discovery detail exchange, command ordering/idempotency primitives, and an in-memory controller/receiver handshake harness. None of those items should be represented as production networking until runtime evidence exists.

## Verified Phase 0 progress — September 16, 2026

Implemented on draft PR #1:

- native Rust core package `goreecloud-cast-core` version `0.1.0`;
- receiver-first Cast mode selection;
- minimized unauthenticated discovery shape;
- bounded State Capsule with Restricted entries excluded from transfer;
- explicit session state and state authority types;
- fail-closed Identity, Privacy Shield, and Wardveil Security gate consumption;
- local-first transport policy and selection;
- unified handoff preflight and normalized failures;
- executable core contract tests;
- mandatory repository documentation baseline; and
- Contract 0.2 platform declaration with incomplete integrations explicitly blocked or migration-required.

The initial CI attempt stopped on rustfmt drift before compilation; the exact formatting changes reported by CI were applied. The subsequent exact-source run `35155854625` passed formatting and all current tests at revision `f165832a3e2e0be546415672a916f706e21890b8`.

No production license grant is inferred from public repository visibility; package publication is disabled and the repository rights notice does not grant reuse rights absent separate GoreeCloud terms.
