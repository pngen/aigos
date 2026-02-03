# AIGOS 1.0 — Canonical Layer Definitions

`aigos` defines the canonical layer set and schema-aligned values used by the AI Governance Operating System.

It consists of two sources of truth:
- `src/lib.rs` — canonical layer definitions embedded into `aigosd` at compile time
- `schemas/config.schema.json` — validation for allowed layer names in `config.yaml`

`aigos` is not a runtime component.
It is a **build-time dependency** that determines which layers exist and are valid.

## Purpose

This crate provides:
- the canonical set of layer names
- compile-time constants consumed by `aigosd`
- definitions for layer naming and validation
- alignment between code and configuration
- a controlled surface for adding or modifying layers

When `aigosd` is compiled, these definitions are embedded into the binary.
After compilation, `aigos` is no longer referenced or loaded.

## Build & Execution Flow

1. Add or modify layer definitions in `src/lib.rs`.
2. Update the layer enum in `schemas/config.schema.json`.
3. Build `aigos` with `cargo build --release`.
4. Build `aigosd`.
5. Copy `sample.yaml` into the working directory as `config.yaml`.
6. Define mesh or meshes in `config.yaml` using canonical layer names.
7. Place the compiled `aigosd` binary into the working directory.
8. Provide each desired layer in its own same-named folder with a same-named binary.
9. Run `aigosd`.

At runtime, `aigosd` matches layer names in `config.yaml` against the embedded canonical layer set.

## Canonical Layers

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

These names must match across:
- `src/lib.rs`
- `schemas/config.schema.json`
- `config.yaml`
- on-disk folder names
- layer binary names

## Extending Layers

1. Update `src/lib.rs`.
2. Update `schemas/config.schema.json`.
3. Rebuild `aigos`.
4. Recompile `aigosd`.
5. Provide corresponding layer binaries at runtime.

Changes are only recognized after recompilation.

## Repository Structure

    src/
      lib.rs
    schemas/
      config.schema.json
    Cargo.toml
    README.md

## Runtime Layout

    /workdir/
      aigosd
      config.yaml
      dio/
        dio
      zt-aas/
        zt-aas
      icae/
        icae
      poc/
        poc

On Windows, binaries use `.exe`.

## Relationship to AIGOSD

- `aigos` defines the layer set
- `aigosd` executes it
- `aigos` is compile-time truth
- `aigosd` is the runtime system

## License

AIGOS and its extension layers are also available for enterprise and institutional licensing.