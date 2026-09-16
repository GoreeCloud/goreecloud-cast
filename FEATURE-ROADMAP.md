---
title: "GoreeCloud Cast — Feature Roadmap"
product: "GoreeCloud Cast"
document_type: "Repository Feature Roadmap"
status: "Proposed / Planned"
version: "v0.4"
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

Phase 0 is active on draft PR #1 with implementation version `0.2.0` and protocol version `0.2`.

The current executable boundary includes capability negotiation, minimized unauthenticated discovery, authenticated receiver-detail models, bounded State Capsules, fail-closed consumption of Identity/Privacy/Security decisions, session authority types, local-first transport selection, handoff preflight, device-identity adapter interfaces, pairing/session-credential adapter interfaces, deterministic protocol framing and typed payload serialization, per-session command ordering/idempotency, and an in-memory authenticated handshake harness.

Exact source revision `c421ce127059a0a1cd8ba33740e9c2188bbf58a3` passed GoreeCloud Cast Core CI run `35157077714`, including `cargo fmt --check` and `cargo test --all-targets`. This is Development source/test evidence for the bounded core only.

Phase 0 is **not complete**. Production cryptography, real secure pairing, expiring/revocable credential providers, persistent trust, authenticated network transport, receiver/controller endpoint processes, UI, deployed platform adapters, runtime acceptance, deployment, release, and Stable qualification remain unimplemented or unverified.

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

Initial bounded core work established:

- native Rust core package;
- receiver-first Cast mode selection;
- minimized unauthenticated discovery shape;
- bounded State Capsule with Restricted entries excluded from transfer;
- explicit session state and state authority types;
- fail-closed Identity, Privacy Shield, and Wardveil Security gate consumption;
- local-first transport policy and selection;
- unified handoff preflight and normalized failures;
- mandatory repository documentation baseline; and
- Contract 0.2 platform declaration with incomplete integrations explicitly blocked or migration-required.

The next verified increment, implementation version `0.2.0`, added:

- device-identity signing and verification adapter traits without an embedded cryptographic implementation;
- pairing authorization and opaque session-credential authority interfaces;
- authenticated discovery-detail models tied to rotating discovery identifiers;
- deterministic protocol `0.2` frames with bounded payloads and strict version/trailing-byte validation;
- deterministic typed serialization for authenticated discovery details and session opening;
- per-session command sequence and idempotency primitives with duplicate recognition, conflicting-replay rejection, wrong-session rejection, out-of-order rejection, and sequence exhaustion handling;
- an in-memory controller/receiver handshake harness that orders identity verification before authenticated receiver-detail release, then pairing authorization, credential issuance/validation, protocol round trips, and session activation; and
- executable tests proving the Development harness succeeds only when the identity verifier accepts, authenticated detail payloads round-trip deterministically, and command ordering/replay rules behave as specified.

The first CI run for this increment, `35156893548`, stopped at rustfmt-only drift before tests. The exact CI formatting patch was applied without behavioral changes. Exact source `c421ce127059a0a1cd8ba33740e9c2188bbf58a3` then passed run `35157077714` at 2026-09-16T22:19:33Z.

No production license grant is inferred from public repository visibility; package publication remains disabled and the repository rights notice does not grant reuse rights absent separate GoreeCloud terms.

## Active next boundary

Continue Phase 0 by replacing test-only security orchestration with concrete Development-grade provider and endpoint boundaries while preserving independent platform authority. The next bounded increment should cover:

- a reviewed cryptographic identity provider backed by an approved dependency or platform facility rather than custom cryptography;
- an explicit pairing confirmation state machine with timeout/cancellation behavior;
- expiring and revocable session-credential provider semantics;
- an authenticated local transport prototype carrying the existing deterministic frames;
- a persistent trust-store interface with revocation-safe restore semantics;
- separate controller and receiver endpoint processes or process-level harnesses;
- malformed-frame, interrupted-handshake, credential-revocation, replay, restart, and recovery tests; and
- continued Identity, Privacy Shield, Wardveil Security, Everkeep, Mesh, Manager, and Glaze UI integration work without claiming runtime conformance before evidence exists.

None of these next items should be represented as production-ready until their concrete runtime providers, target-environment behavior, failure modes, and platform acceptance are verified.
