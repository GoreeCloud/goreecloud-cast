# GoreeCloud Cast — Security

Version: v0.2  
Status: Development

## Current security boundary

The Phase 0 core is a pure domain library with no sockets, filesystem persistence, codec parsing, process sandboxing, or production cryptographic implementation. It consumes platform authorization decisions and fails closed when Identity, Privacy Shield, or Wardveil Security decisions are denied or unknown.

Restricted State Capsule entries cannot be transferred through the implemented API. Remote transports are disabled by the default local-only policy.

Version `0.2.0` adds explicit device-identity signer/verifier interfaces, pairing authorization and session-credential authority interfaces, deterministic protocol framing, authenticated receiver-detail exchange inside an in-memory harness, and command sequencing/idempotency primitives. These are architectural boundaries, not security-provider implementations. The deterministic identity behavior used by tests is deliberately a test double and is not cryptography.

## Not yet implemented

Concrete cryptographic device identity, secure production pairing, session-secret derivation and key rotation, encrypted/authenticated network transport, durable trust persistence and revocation, production replay protection across reconnect/recovery, malicious receiver/controller isolation, denial-of-service controls, deployed Identity/Privacy Shield/Wardveil integrations, and production audit evidence remain open requirements.

Do not report vulnerabilities through public issues when disclosure would expose sensitive exploit details or credentials; use the private security-reporting path configured for the repository when available.
