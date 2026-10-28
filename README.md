# iommu

Contract-only snapshot of the Shuyra IOMMU declarations, public protocol, and
the existing reference-holding ledger, with their owned host tests.

This repository is the public IOMMU provider prerequisite for later Kernel and
Source consumers. It is **not** a standalone IOMMU Driver Unit, server,
runtime, image build, or composition catalog. It contains no register access,
queue programming, page-table runtime, loader, or `prog-iommu` image target.
Source still owns those bodies and the actual image targets until the full
driver migration.

The Kernel-owned DMA-confirmation tests genuinely need the real IOMMU API and
declaration when their Source phase moves to public packages, and Source
`kernel/tests/health` currently uses Source-local `iommu-hold` as `retention_core`
through the normal `kernel/Cargo.toml` dependency. Those consumers stay in Source;
their dependency moves to this provider only after its reviewed public revision exists.

## Layout

- `declaration/` — `iommu-declaration` 0.1.0. The one `UnitFile` declaration
  (`PROGRAM`, name `iommu`), retention/region supply bindings, and entry mark.
- `api/` — `iommu-api` 0.1.0. `Establish`, `Acquire`, `Step`, `Said`, `Wire`,
  `Call`, the generated `Fail` code table, and entry/back/party constants.
  API sibling paths such as `../declaration` stay inside this provider.
- `hold/` — `iommu-hold` 0.1.0, depends on `env` only. Preserves the existing
  `Holds` ledger, the private `Memory` record in its own file, range/scope/
  phase/failure/platform/effects types, and the existing unit range case.
- `api/tests/provider_wire.rs` — the four existing public-API frame tests moved
  from Source `programs/tests/iommu-pages`; only the normal `env`/`iommu_api`
  import adaptation changed.
- `api/tests/golden.rs` — seven added independent golden/boundary cases using
  only the public API.
- `hold/tests/` — the existing range case plus the six moved `Holds` ordering
  cases and their three existing support fixtures.

Each ordinary data type remains in its own file. No production Rust file was
refactored, renamed, reformatted, visibility-widened, or given a new facade.
The private `Memory` record, private `wire::step`, and generated `Fail`
conversion constants remain as they were.

## Contract preserved

The six public operations keep their existing effect boundaries:

- `ESTABLISH` establishes or reuses a persistent Domain.
- `ACQUIRE` obtains a private IOVA mapping for a full Domain identity plus the
  full Hub binding identity.
- `PUBLISH`, `INVALIDATE`, `QUIESCE`, and `RELEASE` are separate effects and
  are not merged.

Messages carry the engine-issued Domain id and the full Hub binding id, not a
trusted caller. Sender identity comes from the real Kernel at the door. Golden
lengths are `Establish` 13, `Acquire` 61, `Step` 42, `Wire` receive buffer 64,
`Said` message 38 in a 48-byte response buffer.

The hold ledger preserves its safety contract:

- `Holds` is the single actual-holds ledger; `Memory` stays private to its own
  module.
- `register` reserves metadata before `MemoryEffects::acquire`, so a failed
  acquire cannot leave an unowned reference.
- `publish_begin` marks reachability before the caller performs publication;
  `publish_end` converts a failed publication into `Closing(Fault)`.
- Normal close and `close_all` ask the platform to synchronize. Fault close
  and death reported through `offline` ask it to isolate; an existing Fault
  remains Fault during `close_all`. Pending retains references for retry, and
  a failed release retains that scope for a later retry.
- Scopes progress independently: one failed scope does not freeze another, and
  partial publication failure still reaches real isolation.
- `forget` only removes an empty scope; occupied/reference/failure accessors do
  not create a second reference account.

The host cases exercise data types, wire bytes, and the ledger's deterministic
safety behavior. They do not prove real hardware authorization, register or
queue effects, or Kernel lifetime handling; those remain in Source/Kernel
tests until their own phases.

## Public pins

| Component | Repository | Revision |
| --- | --- | --- |
| `env` | `https://github.com/shuyra-arche/env.git` | `d3000253cd5d13d466e91fafd4dbb27a5ec5aaae` |
| `unit` | `https://github.com/shuyra-arche/unit.git` | `90396f0fda376e0693fb1b9553aa21864d241057` |
| `system-api` | `https://github.com/shuyra-arche/system.git` | `9e5d63ca536a176583cf1b0054b4ecf7ddd88257` |
| `wire` | `https://github.com/shuyra-arche/wire.git` | `0ff77dc4573fa2620d2fe995a18c01949d9c7e57` |
| `mold` | `https://github.com/shuyra-arche/mold.git` | `f52501f4c91b686eded8b4bcd743b4e96084b7f9` |
| `hub-api` | `https://github.com/shuyra-arche/hub.git` | `cccef3f325198c8390007ae88b064bae0f21f93e` |

There is no PCI-to-IOMMU dependency. The two providers can be published in
either order.

## Build and test

```sh
cargo build --locked --target x86_64-unknown-linux-gnu --lib
cargo build --locked --target riscv64gc-unknown-none-elf --lib
cargo build --locked --release --target riscv64gc-unknown-none-elf --lib
cargo test -p iommu-api --locked --target x86_64-unknown-linux-gnu
cargo test -p iommu-hold --locked --target x86_64-unknown-linux-gnu
```

Add `--offline` after the dependency cache is prepared. The API packages set
`test = false` on the library, so all API cases execute through integration
test targets. `iommu-hold` runs its library range case and its integration
ordering target.

The test targets' direct dependencies are `env`, `wire`, `iommu-api`,
`iommu-declaration`, and `hub-api` (plus `iommu-hold` for the ledger cases) — a
subset of the production closure. The full 15-package provider closure is 3
local packages plus `env`, `unit`, `system-api`, `wire`, `mold`, `hub-api`,
`bitflags`, `erra`, `proc-macro2`, `quote`, `syn`, and `unicode-ident`; there
are no test-only external dependencies.

## Provenance and license

See `PROVENANCE.md` for the frozen Source commit, authors, old-to-new file
hashes, test mapping, and exact limits. Source had no tracked license or
manifest license/authors field; this snapshot adds none and asserts no
copyright permission.
