# GoreeCloud Cast — Privacy Policy

Version: v0.3  
Status: Development / non-production

GoreeCloud Cast is currently an early Development implementation and does not operate a production casting service.

The implemented core follows data-minimization rules: unauthenticated discovery has no user/account identity, friendly device name, room, trust level, ownership relationship, media history, or credentials; State Capsules are explicitly classified; Restricted state is never transferable by the core; and transfer fails closed when required platform privacy authorization is absent or unknown.

Authenticated receiver details may contain a stable device identifier and friendly device name. The Development handshake architecture releases those details only after controller identity proof verifies and the rotating discovery identifier matches the requested receiver. Version `0.3.0` adds Ed25519 challenge signatures, but cryptographic authentication alone does not establish privacy authorization; Privacy Shield remains an independent required authority.

The new Ed25519 verification key is device identity material and must not be added to unauthenticated discovery. Private signing keys are security-sensitive and are not persisted, synchronized, backed up, or transmitted by this implementation. Production key lifecycle and privacy acceptance remain open.

Credential-lease and pairing-confirmation state added in `0.3.0` is in-memory Development state. It is not a production history, audit, or durable trust database and does not authorize backup or synchronization of secrets.

Future network, telemetry, history, remote relay, camera, microphone, display-capture, persistent trust, key-management, and platform-service behavior must be documented and validated before production use. This file does not claim those capabilities are implemented.
