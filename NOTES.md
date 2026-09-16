# GoreeCloud Cast — Repository Notes

Version: v0.1  
Status: Development

- The repository is GoreeCloud-created and currently has no third-party runtime dependencies.
- Rust was selected for the Phase 0 core because it provides a small native cross-platform library surface with memory-safety guarantees and conventional test tooling; this does not lock future platform adapters to Rust.
- No cryptographic primitive is implemented in-house by this increment. Production cryptography must use an approved, reviewed dependency or platform facility with provenance and evidence.
- Drive roadmap v0.2 remains the current authoritative project roadmap until this repository branch is reviewed/merged and roadmap authority is reconciled.
