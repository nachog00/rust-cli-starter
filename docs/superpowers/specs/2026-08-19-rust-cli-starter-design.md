# Rust CLI Starter — Design Spec

**Date:** 2026-08-19
**Status:** Approved (design), pending implementation
**Distributed as:** a [`cargo-generate`](https://cargo-generate.github.io/cargo-generate/) template

## Purpose

A reusable starting point for new Rust CLI projects that captures the
hexagonal (ports & adapters) architecture from `godchat`: a pure `core`,
a `domain` of services depending only on ports, swappable adapters, and a
thin binary composition root. `cargo generate --git <url>` produces a
renamed, compiling, test-green workspace in one step.

Derived from `godchat`'s crate layout and seams:
- **core** — pure types (parse-don't-validate newtypes, module-per-type) +
  ports split into **driven** (outbound) and **driving** (inbound) traits +
  in-memory mocks behind a `test-support` feature.
- **domain** — services implementing driving ports, depending only on
  `Arc<dyn DrivenPort>`, unit-tested against the mocks.
- **adapters** — concrete implementations of driven ports (config, installer).
- **binary** — the composition root: resolves paths, builds concrete
  adapters, wires services, dispatches CLI commands. Nothing else knows the
  concrete types.

## Scope decisions (approved)

| Decision | Choice | Rationale |
|---|---|---|
| Template mechanism | `cargo-generate` | Automatic crate/binary renaming on generate. |
| Example depth | Bare skeleton, seams only | One trivial *live* thread, no throwaway domain entity. |
| Adapters shipped | TOML config + installer/paths | No SQLite, no MCP — documented as extension points instead. |
| Persistence crate | **Excluded** | A store crate with no consumer is dead code; README documents where a `crates/store` adapter plugs in. |
| Example newtype | **Included** (`EnvName`) | Two-line teach of parse-don't-validate at the config boundary. |
| Devcontainer | **Included** (minimal, podman-agnostic, editor-neutral) | Container-first tooling preference; no lifecycle scripts. |
| Editor support | **Helix** | `rust-analyzer` shipped as a rustup component (on PATH for any LSP client) + project-local `.helix/languages.toml` for format-on-save. No VSCode assumptions. |

## The demonstrated thread

Every shipped crate is *live* — the workspace compiles and `cargo test`
passes with no throwaway code. A single `status` command runs end-to-end
through every seam:

```
$ <project> status
environment: dev
version:     0.1.0
checked at:  2026-08-19T12:00:00Z
```

Seams it exercises:
- **driven port** `Clock { fn now(&self) -> DateTime<Utc> }` — an outbound
  dependency that is injected into the service and mocked (`FixedClock`)
  in unit tests.
- **driving port** `Health { fn report(&self) -> HealthReport }` — the
  inbound surface the CLI calls through `Arc<dyn Health>`.
- **service** `HealthService { clock: Arc<dyn Clock> }` implements `Health`;
  a unit test asserts the report carries the injected time.
- **newtype** `EnvName` — parsed (parse-don't-validate) at the **config**
  boundary from `environment = "..."` and carried into the report.
- **config** adapter loads/parses the TOML into a typed struct.
- **installer** resolves paths and `setup` writes a default `config.toml`;
  `uninstall` removes the integration points.

## Crate layout

Lib crate **directories are static**; only **package names and `use` paths**
are templated. No templated file/directory names → most robust
`cargo-generate` setup.

```
{{project-name}}/
├── cargo-generate.toml              placeholders + post-gen hook (cargo build)
├── Cargo.toml                       workspace: edition 2024, workspace.dependencies
├── rust-toolchain.toml              pin stable
├── .gitignore
├── README.md                        architecture + "where does X go?" extension guide
├── .devcontainer/
│   └── devcontainer.json            rust image, ignore-scripts, podman-agnostic
├── docs/…                           this spec (ignored by cargo-generate)
└── crates/
    ├── core/         {{project-name}}-core
    │   └── src/
    │       ├── lib.rs               pub mod types; pub mod ports; #[cfg(test-support)] pub mod mocks;
    │       ├── types.rs             re-exports
    │       ├── types/env_name.rs    EnvName newtype + InvalidEnvName + tests
    │       ├── types/health.rs      HealthReport struct
    │       ├── ports.rs             re-exports driven + driving
    │       ├── ports/driven.rs      trait Clock
    │       ├── ports/driving.rs     trait Health
    │       ├── mocks.rs             re-exports (test-support)
    │       ├── mocks/clock.rs       FixedClock
    │       └── mocks/fixtures.rs    helpers
    ├── domain/       {{project-name}}-domain
    │   └── src/
    │       ├── lib.rs
    │       ├── services.rs
    │       └── services/health.rs   HealthService + unit tests (uses mocks)
    ├── config/       {{project-name}}-config
    │   └── src/
    │       ├── lib.rs               load(path) -> Result<Config, ConfigError>; re-exports
    │       └── config.rs            typed Config { environment: EnvName } + ConfigError
    ├── installer/    {{project-name}}-installer
    │   └── src/
    │       ├── lib.rs               setup() / uninstall() -> Report
    │       └── paths.rs             Paths::resolve() (XDG-style)
    ├── cli/          {{project-name}}-cli
    │   └── src/
    │       ├── lib.rs               pub use Cli, Command, Ctx
    │       ├── app.rs               clap Parser/Subcommand
    │       ├── context.rs           Ctx { health: Arc<dyn Health> }
    │       ├── format.rs            output helpers
    │       ├── commands.rs
    │       └── commands/status.rs   run(args, ctx)
    └── app/          {{project-name}}   (package name; [[bin]] name = {{project-name}})
        └── src/main.rs              composition root: paths → config → adapters → services → dispatch
```

Follows the user convention: **no `mod.rs`** — always `foo.rs` + `foo/`.

## Ports (the seams)

**Driven (outbound, implemented by adapters / mocks):**
```rust
pub trait Clock: Send + Sync {
    fn now(&self) -> DateTime<Utc>;
}
```

**Driving (inbound, implemented by domain services, consumed by CLI):**
```rust
pub trait Health: Send + Sync {
    fn report(&self) -> HealthReport;
}
```

`HealthReport { environment: EnvName, version: &'static str, checked_at: DateTime<Utc> }`.

The CLI depends only on `Arc<dyn Health>` via `Ctx`; the binary is the only
place that names `HealthService`, `FileConfig`, real `Clock`, etc.

## Composition root (binary)

```rust
fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Command::Setup     => run_setup(),
        Command::Uninstall => run_uninstall(),
        Command::Status(a) => with_ctx(|ctx| { commands::status::run(a, ctx); Ok(()) }),
    }
}

fn with_ctx(f: impl FnOnce(&Ctx) -> Result<()>) -> Result<()> {
    let paths  = Paths::resolve()?;
    let config = config::load(&paths.config_file())?;   // parses EnvName here
    let clock: Arc<dyn Clock> = Arc::new(SystemClock);
    let health = Arc::new(HealthService::new(config.environment, clock));
    f(&Ctx { health })
}
```

## cargo-generate mechanics

`cargo-generate.toml`:
- Built-in placeholders: `project-name` (kebab → repo/binary/package prefix),
  `crate_name` (snake → `use {{crate_name}}_core` etc.).
- Prompted placeholders: `authors`, `description` for Cargo metadata.
- `[template] ignore = [...]` excludes `docs/` (this spec) from generated projects.

No post-generation rhai hook is shipped (avoids a fragile cross-platform
script); the README instructs running `cargo test --workspace` after generate.
The template is verified green by substituting the placeholders into a throwaway
project and running `cargo fmt --check && cargo clippy -D warnings && cargo test`
— all pass, and the `status` thread runs end to end.

## Testing strategy

- **core**: newtype parse tests (valid/invalid `EnvName`), colocated `#[cfg(test)]`.
- **domain**: `HealthService` test with `FixedClock` asserting the report's
  `checked_at` equals the injected instant and `environment` round-trips.
- **config/installer**: `tempfile`-based round-trip tests (load a written
  config; `setup` then `uninstall` on a temp dir).
- **CI-ready**: `cargo test --workspace` and
  `cargo test -p {{project-name}}-core --features test-support`.

## Extension guide (README content, not built)

The README documents where the deliberately-omitted seams plug in:
- **Persistence**: add `crates/store` implementing a `Repo` driven port
  (in-memory first, then SQLite via `rusqlite`), wire it in `app`.
- **MCP surface**: add `crates/mcp` (rmcp) that adapts the *driving* ports,
  add `tokio` + an `mcp` subcommand to the binary.
- **New feature**: type → driven/driving port → service (+ test) → adapter →
  CLI command → wire in `with_ctx`.

## Out of scope

SQLite persistence, MCP server, knowledge/markdown store, typestate task
lifecycle, multi-project domain — all present in `godchat` but omitted here
as domain-specific. The README points at `godchat` as the fuller reference.
