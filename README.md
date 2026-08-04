# sigma-theme

[![CI](https://github.com/sigmatactical-org/theme/actions/workflows/ci.yml/badge.svg)](https://github.com/sigmatactical-org/theme/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![MSRV](https://img.shields.io/badge/MSRV-1.97.0-blue.svg)](https://www.rust-lang.org)

Shared Sigma Tactical Group site chrome: static assets, Askama templates, copyright helpers, and optional axum / warp route builders.

Used by:

- `sigmatacticalgroup.com` (warp)
- `identity` (axum)
- `store`, `catalog`, `cart`, `contact`, `accounting`, `info` (Askama + static assets)
- Keycloak login (CSS/JS/fonts under `assets/keycloak/`, synced to platform)

## Layout

```
assets/
  static/       CSS, JS, fonts, icons (embedded at compile time)
  templates/    Askama base + pages (index, 404, 500)
  keycloak/     Keycloak login theme (sigma branding)
ts/             TypeScript sources (authoritative)
assets/static/js/  esbuild output (committed; CI fails if stale vs ts/src)
build.rs        publishes the template/asset paths to dependents (links metadata)
src/
  copyright.rs  copyright_years()
  templates.rs  render_index_html(), error pages
  axum.rs       router() — GET /, /static/*, /favicon.ico
  warp.rs       routes() — full warp filter stack
```

Rust apps extend `assets/templates/base.html` and load `/static/css/site.css`.
Override `{% block nav_actions %}` in the navbar when an app needs auth controls
(store).

## Keycloak login theme

Source: `assets/keycloak/sigma/login/`. Sync into platform manifests after edits:

```bash
./scripts/sync-keycloak-theme.sh
```

Identity devcontainer mounts this directory into Keycloak at
`/opt/keycloak/themes/sigma/login`.

## Frontend (TypeScript)

Browser source is TypeScript only (`ts/src/`). esbuild writes bundles to
`assets/static/js/` (site apps) and `assets/keycloak/sigma/login/resources/js/`
(Keycloak login). After editing anything under `ts/src/`, rebuild and commit the
output:

```bash
cd ts && npm ci && npm run check && npm run build
cargo build -p sigma-theme   # or any dependent crate
```

The served URL stays `/static/js/sigma-dial.js` — that is compiled output, not a source file in the repo.

**The bundles are committed on purpose.** `rust-embed` compiles them into the
binary, and consumers depend on this crate over git, where they cannot run npm
first — an uncommitted bundle would simply be missing from their binary. The
`bundles` CI job rebuilds from `ts/src` and fails if the committed output differs,
so stale bundles are caught rather than shipped. esbuild's output is
deterministic, so a clean rebuild produces no diff.

## Rust API

```rust
// Templates
sigma_theme::templates::render_index_html()?;

// Copyright
sigma_theme::copyright_years();

// axum
Router::new().merge(sigma_theme::axum::router());

// warp
sigma_theme::warp::routes();
```

## Depend on this crate

Declare it as a git dependency pinned to an exact revision:

```toml
sigma-theme = { git = "https://github.com/sigmatactical-org/sigma-theme.git", rev = "…" }
```

`platform/scripts/pin-shared-revs.sh sigma-theme` sets that revision across every
consumer at once, after this repo is pushed. To build a consumer against a local
checkout of this crate instead, run the consumer's `scripts/prepare-local.sh`,
which patches the dependency to whichever sibling checkout it finds; don't hand-
edit the manifest to a `path`, because that form cannot be committed.

### Templates in dependents

A consumer's pages extend `assets/templates/base.html`, which Askama resolves from
the filesystem at compile time. That path is only knowable here — from git this
crate lives in Cargo's checkout directory under a rev-specific hash — so `build.rs`
publishes it as `links` metadata, and each consumer's own `build.rs` reads it:

```rust
let templates = std::env::var("DEP_SIGMA_THEME_TEMPLATES")?;
// write askama.toml with dirs = ["templates", templates]
```

`DEP_SIGMA_THEME_ASSETS` carries the `assets/` directory the same way. Both reach
direct dependents only, which is why the `links = "sigma-theme"` key exists in
`Cargo.toml` despite there being no native library.

Repository: https://github.com/sigmatactical-org/sigma-theme

## Workspace

This crate lives under `it/theme/` in the `sigma/` monorepo alongside `it/identity`, `it/sigmatacticalgroup.com`, and the other IT service repos.

## Brand & artwork

© Sigma Tactical Group. **All rights reserved.**

The Sigma Tactical Group name, logos, marks, artwork, and visual identity are **proprietary**. They are not covered by this repository's source-code license. See [BRANDING.md](BRANDING.md).

## License

Licensed **MIT OR Apache-2.0** for **source code** only (see `LICENSE-MIT` and `LICENSE-APACHE`). Branding remains proprietary as described above.
