# Overcore

This is Overdaw's audio engine which makes use of a highly modular and dynamic,
graph-based DSP chain as processing model, written in Rust to be safe and
efficient on resources usage. Can be also used as a library for another audio
system. Written to be abstract of execution environment.

## Status

Pre-Alpha. Contributions are very welcome!

## Dependencies

This crate only depends on Rust's Standard Library and some cross-platform
crates. Platform integration is done by plugins as features, so the engine
itself is platform-agnostic. It's been designed this way to be usable on
platforms such as Web through WASM, Android using NDK or just like a normal
desktop application.
