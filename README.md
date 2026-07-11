# sigma-theme

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
assets/static/js/  esbuild output (gitignored; build before cargo)
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

Browser source is TypeScript only (`ts/src/`). esbuild writes gitignored bundles to
`assets/static/js/` (site apps) and `assets/keycloak/sigma/login/resources/js/`
(Keycloak login). Build before `cargo build` because of `rust-embed`:

```bash
cd ts && npm ci && npm run check && npm run build
cargo build -p sigma-theme   # or any dependent crate
```

The served URL stays `/static/js/sigma-dial.js` — that is compiled output, not a source file in the repo.

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

From GitHub (other repos / CI):

```toml
sigma-theme = { git = "ssh://git@github.com/sigmatactical-org/sigma-theme.git" }
```

Local monorepo checkout:

```toml
sigma-theme = { path = "../theme" }
```

Repository: https://github.com/sigmatactical-org/sigma-theme

## Workspace

This crate lives under `it/theme/` in the `sigma/` monorepo alongside `it/identity`, `it/sigmatacticalgroup.com`, and the other IT service repos.

## Brand & artwork

© Sigma Tactical Group. **All rights reserved.**

The Sigma Tactical Group name, logos, marks, artwork, and visual identity are **proprietary**. They are not covered by this repository's source-code license. See [BRANDING.md](BRANDING.md).

## License

Licensed **MIT OR Apache-2.0** for **source code** only (see `LICENSE-MIT` and `LICENSE-APACHE`). Branding remains proprietary as described above.
