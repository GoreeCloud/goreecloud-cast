# GoreeCloud Cast — Privacy Policy

Version: v0.2  
Status: Development / non-production

GoreeCloud Cast is currently an early Development implementation and does not operate a production casting service.

The implemented core follows data-minimization rules: unauthenticated discovery has no user/account identity, friendly device name, room, trust level, ownership relationship, media history, or credentials; State Capsules are explicitly classified; Restricted state is never transferable by the core; and transfer fails closed when required platform privacy authorization is absent or unknown.

Implementation version `0.2.0` adds an authenticated receiver-detail model that can contain a stable device identifier and friendly device name. In the in-memory Development handshake harness, those details are serialized and returned only after controller identity proof verifies and the rotating discovery identifier matches the requested receiver. This source-level ordering is not a production privacy or network-security guarantee until real Identity, Privacy Shield, transport, and receiver implementations are integrated and accepted.

Future network, telemetry, history, remote relay, camera, microphone, display-capture, persistent trust, and platform-service behavior must be documented and validated before production use. This file does not claim those capabilities are implemented.
