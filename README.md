# AIGOS 1.0 — Canonical Registry & World Model Library

`aigos` is the canonical world-model library for the AI Governance Operating System.
It defines the authoritative list of governance layers, canonical names, and metadata that are embedded into `aigosd` at compile-time.

`aigos` is not a runtime component.
It is a **build-time dependency** that shapes how the AIGOS daemon interprets the world.

## Purpose

This crate provides:
- the canonical registry of all governance layers
- compile-time constants used by `aigosd`
- definitions for layer naming, validation, and mesh resolution
- the extensible structure for adding new layers or modifying existing ones

When `aigosd` is compiled, it imports this crate and embeds the registry into its binary.
After compilation, no further crawling or loading of `aigos` occurs.

## How it Works

1. Modify or extend layer definitions in this crate.
2. Build the library `(cargo build --release)`.
3. Build `aigosd`, which links against the compiled library.
4. The resulting `aigosd` binary contains the complete canonical worldview.

At runtime, `aigosd` matches layer names in `config.yaml` against this embedded registry.

## Canonical Layers

This crate defines the ten core governance layers:
- `dio`
- `zt-aas`
- `icae`
- `poc`
- `fak`
- `are`
- `jib`
- `icl`
- `gsas`
- `able`

These names must match the on-disk folder names and binaries used by the daemon.

## Extending the Registry

To introduce new layers or rename existing ones:
1. Add or modify the entries in the registry module.
2. Rebuild the library.
3. Recompile `aigosd`.
4. Provide corresponding layer binaries in the working directory at runtime.

Because the daemon embeds these definitions at compile-time, users must rebuild it to recognize any new or altered layers.

## Repository Structure

<pre>
src/
  lib.rs          # Public-facing registry module
  registry.rs     # Canonical layer definitions and mappings
Cargo.toml
README.md
</pre>

`aigos` should remain a minimal, sharply-defined crate with no runtime branching or external dependencies beyond `serde/serde_derive` for compile-time serialization.

## Compilation

To build the library:
```bash
cargo build --release
```

This produces:
```bash
target/release/libaigos.rlib
```

## Relationship to AIGOSD

- `aigos` provides the world. `aigosd` executes it.
- `aigos` defines the allowed layers. `aigosd` launches them.
- `aigos` is static, compile-time truth. `aigosd` is the runtime operating system.

## License

AIGOS is also available for enterprise and institutional licensing.