# AIGOS 1.0 - Canonical World-Model Registry

`aigos` is the canonical world-model registry for the AI Governance Operating System. It defines which layer names exist, which layers are mandatory Core, and which future layers are optional extensions.

It consists of one runtime source of truth and one public Core schema:
- `src/lib.rs` - canonical Core and extension layer definitions embedded into `aigosd` at compile time
- `schemas/config.schema.json` - public Core configuration schema for external validation and editor tooling

`aigos` is not a runtime component. It is a **build-time dependency** that determines which layers are recognized by the supervisor.

## Layer Registry

AIGOS Core consists of ten mandatory layers:
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

The registry is split into:
- `CANONICAL_CORE_LAYERS` for the mandatory ten-layer Core substrate
- `CANONICAL_EXTENSION_LAYERS` for optional additive unlocks
- `canonical_layers()` for the derived Core + extension universe

Extensions are additive unlocks. Adding an extension requires adding its canonical name to `CANONICAL_EXTENSION_LAYERS`, rebuilding `aigos`, recompiling `aigosd`, and making the extension binary available in the runtime bundle.

## Relationship To AIGOSD

AIGOSD always starts the full ten-layer Core substrate for every configured mesh. `config.yaml` cannot subtract from Core.

`config.yaml` may omit `layers` for a Core-only runtime:

```yaml
meshes:
  mesh1: {}
```

When extension layers are unlocked in `CANONICAL_EXTENSION_LAYERS`, `config.yaml` may list them to run Core + extensions:

```yaml
meshes:
  mesh1:
    layers:
      - iam
      - sck
```

That means:
- run all ten Core layers first
- then run `iam`
- then run `sck`

For backward compatibility, a mesh may list all ten Core layers. AIGOSD still runs Core once and does not double-spawn Core.

## Build Flow

1. Add Core or extension definitions in `src/lib.rs`.
2. Update `schemas/config.schema.json` only for public Core schema changes. Private extension builds may ship their own unlocked schema separately.
3. Build `aigos`.
4. Recompile `aigosd`.
5. Place `aigosd`, `config.yaml`, all Core binaries, and every extension binary named by the compiled `CANONICAL_EXTENSION_LAYERS` registry in the runtime bundle.
6. Run `aigosd`.

Changes are only recognized after recompilation.

## Runtime Layout

Preferred flat layout:

```text
/workdir/
  aigosd
  config.yaml
  dio
  zt-aas
  icae
  poc
  fak
  are
  jib
  icl
  gsas
  able
```

On Windows, binaries use `.exe`.

## License

AIGOS and its extension layers are also available for enterprise and institutional licensing.
