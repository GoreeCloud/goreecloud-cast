# Phase 0 Core Architecture

Status: Development  
Version: v0.3  
Last updated: 2026-09-16

This document describes the currently verified GoreeCloud Cast Phase 0 implementation boundary.

## Scope

The `goreecloud-cast-core` crate is a transport-independent domain and Development security-provider layer. It currently implements:

- bounded device/session identity value types;
- normalized Cast capability representation and preferred-mode selection;
- privacy-minimized unauthenticated discovery advertisements;
- authenticated receiver-detail models;
- a bounded, versioned State Capsule with transfer classifications;
- fail-closed platform decisions for Identity, Privacy Shield, and Wardveil Security outcomes;
- local-first transport-profile selection;
- a unified handoff preflight that produces either a concrete plan or a normalized failure;
- device identity, pairing, and session-credential adapter contracts;
- deterministic protocol `0.2` framing and typed payload serialization;
- command ordering and idempotency primitives;
- an in-memory authenticated controller/receiver handshake harness;
- a concrete Development Ed25519 challenge signer/verifier using pinned `ed25519-dalek` 3.0.0;
- explicit pairing confirmation states with deadline, deny, cancel, and finalized-state behavior; and
- in-memory credential leases with expiration, revocation, and binding-substitution rejection.

It intentionally does **not** implement a production network stack, media codecs, approved device-key storage/rotation, complete secure pairing UX/runtime, durable trust storage, receiver/controller endpoint processes, UI, remote relay, deployed platform-service adapters, or production authorization.

## Authority boundaries

Cast consumes decisions from GoreeCloud Identity, Privacy Shield, and Wardveil Security; it does not replace those authorities. An `Unknown` decision fails closed. `Restricted` State Capsule entries are never transferable by the core, even when sensitive transfer is allowed.

Ed25519 challenge verification proves only that the presented signature matches a registered device verification key for the domain-separated Cast challenge message. It does not independently prove account identity, pairing consent, privacy authorization, device posture, or transport confidentiality.

## Device identity cryptography

`src/crypto.rs` implements the existing device-identity traits using Ed25519 from `ed25519-dalek` 3.0.0. The signed message binds:

- a GoreeCloud Cast device-identity domain separator;
- the Cast `DeviceId`;
- the challenge context; and
- the 32-byte challenge nonce.

The verifier rejects unregistered devices, malformed signatures, weak public keys at registration, and signatures that fail strict verification. Private signing key material is supplied to the signer by the caller; this crate does not yet define the production key store or key lifecycle.

## Pairing and session credentials

`src/pairing.rs` models a confirmation request that can remain pending only until its explicit deadline. It may become Confirmed, Denied, Cancelled, or Expired, and finalized/expired requests cannot be revived. A pairing grant must match the original controller/receiver binding.

`src/credentials.rs` models registered credential leases separately from credential generation. Leases have explicit issue/expiry timestamps, can be revoked, and reject a presented credential whose handle matches but session/controller/receiver binding differs. This proves lease semantics only; it is not yet a production credential issuer or persistent store.

## Preferred mode order

`ReceiverPlayback -> DirectMediaStream -> ApplicationCast -> DisplayMirror`

This codifies the receiver-first architecture while still permitting policy to remove modes.

## Transport order

Local direct transport is preferred, followed by an authorized local relay. Remote direct and remote relay remain disabled by the default `LOCAL_ONLY` policy. No authenticated socket/network implementation exists yet.

## Verified evidence

- Exact source `c421ce127059a0a1cd8ba33740e9c2188bbf58a3` passed Core CI run `35157077714` for the protocol/handshake adapter increment.
- Exact source `2bab9042542937efbf96fa9539031a3ba1c4ded9` passed Core CI run `35158442444` for Ed25519 device identity, pairing confirmation, and credential-lease contracts.

Both are Development source/test evidence only.

## Next implementation boundary

The next bounded Phase 0 increment should add an authenticated local frame-transport prototype and revocation-safe persistent trust-store interface, then exercise interrupted handshake, trust revocation, restart/recovery, malformed-frame, and credential-revocation paths across separate controller/receiver endpoint harnesses. Target-platform key storage and Integral Platform System acceptance must remain separate gates.
