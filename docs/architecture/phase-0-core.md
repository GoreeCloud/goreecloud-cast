# Phase 0 Core Architecture

Status: Development  
Version: v0.1  
Last updated: 2026-09-16

This document describes the first executable GoreeCloud Cast implementation boundary.

## Scope

The `goreecloud-cast-core` crate is a transport-independent domain layer. It currently implements:

- bounded device/session identity value types;
- normalized Cast capability representation and preferred-mode selection;
- privacy-minimized unauthenticated discovery advertisements;
- a bounded, versioned State Capsule with transfer classifications;
- fail-closed platform decisions for Identity, Privacy Shield, and Wardveil Security outcomes;
- local-first transport-profile selection; and
- a unified handoff preflight that produces either a concrete plan or a normalized failure.

It intentionally does **not** implement cryptography, networking, codecs, persistent pairing/trust storage, receiver processes, UI, remote relay, platform-service adapters, or production authorization. Those remain active Phase 0/roadmap obligations.

## Authority boundaries

Cast consumes decisions from GoreeCloud Identity, Privacy Shield, and Wardveil Security; it does not replace those authorities. An `Unknown` decision fails closed. `Restricted` State Capsule entries are never transferable by the core, even when sensitive transfer is allowed.

## Preferred mode order

`ReceiverPlayback -> DirectMediaStream -> ApplicationCast -> DisplayMirror`

This codifies the receiver-first architecture while still permitting policy to remove modes.

## Transport order

Local direct transport is preferred, followed by an authorized local relay. Remote direct and remote relay remain disabled by the default `LOCAL_ONLY` policy.

## Next implementation boundary

The next implementation increment should add cryptographic identity adapter interfaces, pairing/session credential derivation interfaces, authenticated discovery detail exchange, deterministic protocol serialization, command sequencing/idempotency, and a small in-memory receiver/controller handshake test harness without representing those as production networking.
