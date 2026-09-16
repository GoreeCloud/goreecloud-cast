# GoreeCloud Cast — Current Features

Version: v0.3  
Status: Development

Current executable functionality remains intentionally bounded:

- capability sets and preferred Cast-mode selection;
- State Capsule construction, bounds enforcement, duplicate-key prevention, and privacy-filtered transfer views;
- minimal unauthenticated discovery advertisements;
- authenticated receiver-detail value models released by the Development handshake harness only after identity proof verifies;
- session identity/state authority models;
- fail-closed platform gate aggregation;
- local-first transport selection;
- handoff preflight producing a plan only when identity, privacy, security, capability, state, and transport requirements are all satisfied;
- device-identity signer/verifier interfaces plus a concrete Ed25519 Development provider using pinned `ed25519-dalek` 3.0.0;
- weak-public-key rejection and strict Ed25519 challenge-signature verification;
- pairing authorization contracts plus an explicit confirmation state machine with timeout, deny, cancel, finalized-state, and grant-binding checks;
- opaque session-credential authority contracts plus in-memory credential leases with expiry, revocation, and binding-substitution rejection;
- deterministic versioned protocol frames plus typed authenticated-discovery and session-open payload serialization;
- per-session command ordering, duplicate detection, conflicting-replay rejection, and idempotency keys; and
- an in-memory controller/receiver handshake harness covering identity proof, authenticated receiver details, pairing authorization, session credential issuance/validation, protocol round trips, and transition to an Active Development session.

No user-facing casting feature, production key lifecycle, persistent trust store, or real authenticated network/media path is complete yet.
