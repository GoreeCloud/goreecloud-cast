# GoreeCloud Cast — Repository Notes

Version: v0.3  
Status: Development

- The repository is GoreeCloud-created.
- Rust remains the Phase 0 core language because it provides a small native cross-platform library surface with memory-safety guarantees and conventional test tooling; future platform adapters are not locked to Rust.
- Implementation version `0.3.0` introduces one material third-party runtime dependency: `ed25519-dalek` exactly `3.0.0`, used only for Ed25519 device-challenge signatures behind the Cast identity-provider traits.
- `ed25519-dalek` 3.0.0 is recorded as BSD-3-Clause with upstream provenance and replacement/security boundaries in `THIRD-PARTY-NOTICES.md`.
- Cast still does not own a production key lifecycle. `Ed25519DeviceSigner::from_secret_bytes` accepts key material from a higher-level approved key-management boundary; production private-key generation, secure persistence, rotation, backup/recovery policy, and revocation distribution are still unimplemented.
- Pairing confirmation and credential lease stores are deterministic/in-memory Development components, not persistent or production authorization systems.
- Drive roadmap v0.5 remains the authoritative project roadmap until this repository branch is reviewed/merged and roadmap authority is formally reconciled.
- Exact source `2bab9042542937efbf96fa9539031a3ba1c4ded9` passed GoreeCloud Cast Core CI run `35158442444` on September 16, 2026, including formatting and all current tests. This is Development source/test evidence only.
