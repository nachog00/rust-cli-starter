# {{project-name}}

{{description}}

A Rust CLI built on a **hexagonal (ports & adapters)** architecture. The
center is pure and testable; I/O lives in swappable adapters; a thin binary
wires it all together.

## Quick start

```sh
{{project-name}} setup      # write the default config
{{project-name}} status     # report health (environment, version, timestamp)
{{project-name}} uninstall  # remove the config directory
```

Build, test, lint:

```sh
cargo build
cargo test --workspace
cargo fmt --all --check      # formatting gate (clean on a fresh generate)
cargo clippy --workspace --all-targets -- -D warnings
```

### Editor / tooling

`rust-toolchain.toml` pins stable and installs `rust-analyzer` as a rustup
component, so it's on `PATH` for any LSP client — **Helix picks it up with no
config**. A project-local `.helix/languages.toml` turns on rustfmt format-on-save
for this workspace. The `.devcontainer` is editor-neutral (just the Rust image);
run it with `podman` via your preferred devcontainer wrapper.

## Architecture

```
          driving ports                         driven ports
          (inbound traits)                      (outbound traits)
                │                                       │
   ┌────────┐   │   ┌────────┐   ┌──────────┐   │   ┌───────────────────┐
   │  cli   │──▶│──▶│ domain │──▶│   core   │◀──│───│ config / installer│
   │ (clap) │  Health│services│  ports/types│  Clock│    (adapters)     │
   └────────┘       └────────┘   └──────────┘       └───────────────────┘
        ▲                                                     ▲
        └──────────────────  app (binary)  ──────────────────┘
                        composition root
```

- **`crates/core`** — the hexagon center. Pure value types (newtypes that
  parse-don't-validate, one module per type) and the port traits. Ships
  in-memory mocks behind a `test-support` feature.
  - `ports/driven.rs` — outbound traits the app *needs* (e.g. `Clock`).
  - `ports/driving.rs` — inbound traits the app *offers* (e.g. `Health`).
- **`crates/domain`** — services implementing driving ports using only
  driven ports (`Arc<dyn Trait>`). Unit-tested against the core mocks.
- **`crates/config`** — TOML adapter. Deserializes the raw file, then parses
  into core newtypes so invalid input fails at the boundary.
- **`crates/installer`** — path resolution (XDG-style) + `setup`/`uninstall`.
- **`crates/cli`** — clap delivery adapter. Calls driving ports via a `Ctx`;
  knows nothing of concrete services.
- **`crates/app`** — the binary. The *only* place that names concrete
  adapters: it resolves paths, loads config, builds adapters, wires services,
  and dispatches commands.

The dependency rule: everything points inward at `core`. Adapters depend on
`core`; the domain depends on `core`; nothing depends on an adapter except the
binary.

## Where does X go?

**A new feature (the full loop):**

1. **Type** → `crates/core/src/types/<name>.rs`, re-export from `types.rs`.
   Wrap primitives in a newtype with a `parse` constructor.
2. **Port** → add a method to an existing trait, or a new trait in
   `ports/driven.rs` (something the app needs) or `ports/driving.rs`
   (something it offers).
3. **Service** → `crates/domain/src/services/<name>.rs` implementing the
   driving port; unit-test it with mocks (`test-support`).
4. **Adapter** → implement any new driven port in a crate under `crates/`.
5. **Command** → `crates/cli/src/commands/<name>.rs` with a clap `Args` and
   `run(&Args, &Ctx)`; register it in `app.rs` + `commands.rs`.
6. **Wire it** → construct the adapter and service in
   `crates/app/src/main.rs` (`with_ctx`), add the field to `Ctx`.

**Persistence** (deliberately omitted): add `crates/store` with a `Repo`
driven port in `core` (start with an in-memory `HashMap` impl, later swap in
`rusqlite`). Wire it in the binary — no other crate changes. This is the
`Clock` seam applied to storage.

**An MCP surface** (omitted): add `crates/mcp` (e.g. `rmcp`) that adapts the
existing *driving* ports, add `tokio` + an `mcp` subcommand to the binary.
The domain doesn't change — MCP is just another delivery adapter alongside
the CLI.

> This template is a distilled slice of a larger hexagonal Rust project. If
> you want a fuller worked reference (SQLite repos, an MCP server, a typestate
> entity lifecycle, multi-entity domain), that's where the pattern scales to.

## Conventions

- **No `mod.rs`** — a module `foo` is `foo.rs` + a `foo/` directory.
- **Parse, don't validate** — newtypes are constructed through a fallible
  `parse`, so an invalid value is unrepresentable past the boundary.
- **Ports over implementations** — depend on `dyn Trait`; name concrete
  types only in the binary.

## Regenerating

This repo is a [`cargo-generate`](https://cargo-generate.github.io/cargo-generate/)
template:

```sh
cargo generate --git <this-repo-url>
```

It prompts for the project name (renames crates, binary, and `use` paths) and
a description. After generating, run `cargo test --workspace` to confirm a
green start.
