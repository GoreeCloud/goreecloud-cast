# Phase 0 Core Architecture

Status: Development  
Version: v0.2  
Last updated: 2026-09-16

This document describes the current executable GoreeCloud Cast Phase 0 implementation boundary.

## Scope

The `goreecloud-cast-core` crate is a transport-independent domain layer. It currently implements:

- bounded device/session identity value types;
- normalized Cast capability representation and preferred-mode selection;
- privacy-minimized unauthenticated discovery advertisements;
- authenticated receiver-detail value models;
- a bounded, versioned State Capsule with transfer classifications;
- fail-closed platform decisions for Identity, Privacy Shield, and Wardveil Security outcomes;
- local-first transport-profile selection;
- a unified handoff preflight that produces either a concrete plan or a normalized failure;
- interfaces for device-identity signing and verification;
- interfaces for pairing authorization and opaque session-credential issuance, validation, and revocation;
- deterministic protocol frames with explicit protocol version, message kind, correlation ID, payload length, and bounded payload;
- deterministic typed payload serialization for authenticated discovery details and session opening;
- command sequencing/idempotency with duplicate recognition, conflicting-replay rejection, out-of-order rejection, wrong-session rejection, and sequence exhaustion handling; and
- an in-memory handshake harness that exercises identity verification, authenticated discovery-detail exchange, pairing authorization, session-credential issuance/validation, protocol round trips, and session activation without creating a network or production security implementation.

The core still intentionally does **not** implement production cryptography, sockets or other network transport, codecs, persistent pairing/trust storage, receiver/controller processes, UI, remote relay, deployed platform-service adapters, or production authorization. Those remain active Phase 0/roadmap obligations.

## Authority boundaries

Cast consumes decisions from GoreeCloud Identity, Privacy Shield, and Wardveil Security; it does not replace those authorities. An `Unknown` decision fails closed. `Restricted` State Capsule entries are never transferable by the core, even when sensitive transfer is allowed.

The identity, pairing, and session-credential traits added in implementation version `0.2.0` are provider boundaries. They deliberately contain no built-in cryptographic algorithm, key store, trust database, or production credential format. The in-memory test providers validate orchestration only.

## Discovery privacy boundary

Unauthenticated `DiscoveryAdvertisement` remains minimized and excludes friendly name and stable device identity. `AuthenticatedDiscoveryDetails` may carry the stable `DeviceId` and friendly name, but the in-memory handshake does not construct or return the authenticated detail response until the controller proof has verified and the advertisement correlation has matched.

This source-level ordering is not equivalent to a secure network protocol until a real authenticated transport and production identity provider are integrated and validated.

## Protocol boundary

Protocol version `0.2` introduces deterministic frame serialization with the `GCC0` magic value, explicit major/minor version fields, message kind, correlation ID, bounded payload length, and exact trailing-byte checks. The current typed payloads cover authenticated discovery details and session opening. Future protocol messages must preserve version negotiation, strict parsing, bounded allocations, and fail-closed validation.

## Command authority boundary

Commands are scoped to one session. Sequence numbers begin at one. New commands must arrive in order; retries with the same idempotency key and same operation payload are recognized as duplicates; reuse of an idempotency key for different operation data is rejected as a conflicting replay. This is a source-level ordering primitive, not complete network replay protection.

## Preferred mode order

`ReceiverPlayback -> DirectMediaStream -> ApplicationCast -> DisplayMirror`

This codifies the receiver-first architecture while still permitting policy to remove modes.

## Transport order

Local direct transport is preferred, followed by an authorized local relay. Remote direct and remote relay remain disabled by the default `LOCAL_ONLY` policy.

## Verification

Exact source `c421ce127059a0a1cd8ba33740e9c2188bbf58a3` passed GoreeCloud Cast Core CI run `35157077714`, including `cargo fmt --check` and `cargo test --all-targets`. The earlier source commit `101fa022595cec14c85481d84fa63afc958cd5f0` stopped at rustfmt-only drift before compilation; the CI-provided formatting changes were applied without behavioral changes before the passing run.

## Next implementation boundary

The next Phase 0 increment should connect these interfaces to concrete Development-grade providers and process boundaries without prematurely claiming production security: a reviewed cryptographic identity dependency or platform facility, explicit pairing confirmation state machine, expiring/revocable session credentials, authenticated local transport prototype, persistent trust-store interface with revocation semantics, receiver/controller endpoint processes, and failure-mode tests for malformed frames, replay, revocation, restart, and interrupted handshakes.
