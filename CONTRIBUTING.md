# Contributing

Atlas is a Rust/GPUI desktop file manager.

## Local Check

Before opening a PR or commit you care about, run:

```powershell
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

CI runs the same three on Windows and Linux, plus clippy with
`--all-features` on Linux. The lint policy lives in `[workspace.lints]` in
`Cargo.toml`; when a lint is legitimately wrong for one spot, allow it there
with a `// reason:` comment rather than loosening the workspace policy.

## Build Profiles

Use `cargo devfast` for the smallest feature surface, `cargo devfull` when the
default archive/provider features are required, and `cargo rel` for the
production artifact. Both development commands and plain `cargo run` share the
same `target/debug` profile. Optional nightly compiler experiments and the
repeatable `atlas-ui` benchmark are documented in [docs/build-performance.md](docs/build-performance.md).

## Vendored Code

Atlas keeps modified GPUI and gpui-component sources under `vendor/`.
Changes to those forks should be committed in this repo with the app change that
needs them.
