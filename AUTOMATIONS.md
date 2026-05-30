# Automations

This document describes all automations in the project, where they live, and what they enforce.

## Local automations (git hooks)

Git hooks are configured via `core.hooksPath` pointing to `.githooks/`. Run `./setup-hooks.sh` once to enable them.

| Hook file | Trigger | What it does |
|---|---|---|
| `.githooks/commit-msg` | `git commit` | Validates the commit message follows [Conventional Commits](https://www.conventionalcommits.org/). Rejects the commit if the first line doesn't match `type[(scope)][!]: description`. |

## CI (continuous integration)

All CI workflows live in `.github/workflows/`. They use standard Rust tooling — no extra task runners.

### `ci.yml` — Core CI

**Trigger:** every push and pull request (any branch).

| Job | Tool | What it checks |
|---|---|---|
| PR title | shell | PR title follows Conventional Commits (PRs only). |
| Commit message | shell | Last commit message follows Conventional Commits (pushes only). |
| Format | `cargo fmt --check` | All code is formatted with rustfmt. |
| Clippy | `cargo clippy -- -D warnings` | No lint warnings (all targets, all features). |
| Test | `cargo test --all` | All unit, integration, and doc tests pass. |
| Docs | `cargo doc` | Documentation builds without warnings. |
| Unused deps | `cargo-machete` | No unused dependencies declared in Cargo.toml files. |

### `security.yml` — Security and license audit

**Trigger:** every push and PR to `main`, plus every Monday at 06:00 UTC.

| Job | Tool | What it checks |
|---|---|---|
| cargo-deny | `cargo-deny` | Advisories (RUSTSEC), duplicate crate bans, license compatibility. Configured via `deny.toml`. |

### `binary-check.yml` — Binary size and compilation time

**Trigger:** pull requests targeting `main`.

| Job | Tool | What it checks |
|---|---|---|
| Size & compile time | `cargo build --release`, `cargo-bloat` | Compares binary size and wall-clock compilation time against the baseline from `main`. Fails if size grows >10% or compile time grows >20%. Posts a `cargo-bloat` report as a PR comment. |

### `binary-baseline.yml` — Baseline recording

**Trigger:** push to `main`.

| Job | Tool | What it does |
|---|---|---|
| Update baseline | `cargo build --release` | Records binary sizes (per executable) and compilation time into `baseline.json`, then stores it in the GitHub Actions cache for PR comparisons. |

## Releases

### `release.yml` — Release orchestration

**Trigger:** push to `main`.

Runs [release-plz](https://release-plz.ieni.dev/) which:

1. Reads commit messages since the last tag (using Conventional Commits).
2. Determines the next semantic version bump.
3. Creates or updates a release PR with `CHANGELOG.md` updates and crate version bumps.
4. When that PR is merged, release-plz creates a git tag and a GitHub release.

Configured via `release-plz.toml`. All crates share a single root `CHANGELOG.md`.

### `release-binary.yml` — Binary attachment

**Trigger:** GitHub release published.

| Job | Tool | What it does |
|---|---|---|
| Build & upload | `cargo build --release`, `action-gh-release` | Builds the `runtime` binary and attaches it to the GitHub release. |

## Configuration files

| File | Purpose |
|---|---|
| `deny.toml` | cargo-deny configuration: allowed licenses, advisory policy, duplicate crate bans. |
| `release-plz.toml` | release-plz configuration: workspace layout, changelog paths, git-only releases. |
| `.githooks/commit-msg` | Shell script enforcing Conventional Commits locally. |
| `setup-hooks.sh` | One-time script to enable git hooks via `core.hooksPath`. |
