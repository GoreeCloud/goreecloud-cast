# GoreeCloud Cast — Security

Version: v0.1  
Status: Development

## Current security boundary

The Phase 0 core is a pure domain library with no sockets, filesystem persistence, codec parsing, process sandboxing, or cryptographic implementation. It consumes platform authorization decisions and fails closed when Identity, Privacy Shield, or Wardveil Security decisions are denied or unknown.

Restricted State Capsule entries cannot be transferred through the implemented API. Remote transports are disabled by the default local-only policy.

## Not yet implemented

Cryptographic device identity, secure pairing, session credential derivation, encrypted transport, replay protection, trust persistence/revocation, malicious receiver/controller isolation, denial-of-service controls, and production audit evidence remain open requirements.

Do not report vulnerabilities through public issues when disclosure would expose sensitive exploit details or credentials; use the private security-reporting path configured for the repository when available.
