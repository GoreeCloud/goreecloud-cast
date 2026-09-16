# GoreeCloud Cast — Current Features

Version: v0.2  
Status: Development

Current executable functionality remains intentionally bounded:

- capability sets and preferred Cast-mode selection;
- State Capsule construction, bounds enforcement, duplicate-key prevention, and privacy-filtered transfer views;
- minimal unauthenticated discovery advertisements;
- authenticated receiver-detail value models that are released by the handshake harness only after identity proof verifies;
- session identity/state authority models;
- fail-closed platform gate aggregation;
- local-first transport selection;
- handoff preflight producing a plan only when identity, privacy, security, capability, state, and transport requirements are all satisfied;
- device-identity signer/verifier interfaces without an embedded production cryptographic implementation;
- pairing authorization and opaque session-credential authority interfaces;
- deterministic versioned protocol frames plus typed authenticated-discovery and session-open payload serialization;
- per-session command ordering, duplicate detection, conflicting-replay rejection, and idempotency keys; and
- an in-memory controller/receiver handshake harness covering identity proof, authenticated receiver details, pairing authorization, session credential issuance/validation, protocol round trips, and transition to an Active Development session.

No user-facing casting feature, production security provider, persistent trust store, or real network/media path is complete yet.
