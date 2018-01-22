# core

This is Overdaw's audio engine which makes use of a highly modular and dynamic
graph-based DSP chain as processing core, written in Rust to be safe and
efficient on resources usage. This can be also used as a crate for another audio
system and written to be abstract of usage environment.

## Status

The API is not stable and needs to pass it's evolution process.

## Dependencies

This crate only depends on Rust's Standard Library and some cross-platform
crates.  Platform integration is done by plugins and used as features, so the
engine itself is platform-agnostic. It's been designed this way to be usable on
platforms such as Web through WASM.
