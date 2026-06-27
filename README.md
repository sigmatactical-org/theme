# sigma-theme

Shared Sigma Tactical Group site chrome: static assets, Askama templates, copyright helpers, and optional axum / warp route builders.

Used by:

- `sigmatacticalgroup.com` (warp)
- `identity` (axum)

## Layout

```
assets/
  static/       CSS, JS, fonts, icons (embedded at compile time)
  templates/    Askama base + pages (index, 404, 500)
ts/             TypeScript sources (authoritative)
assets/static/js/  esbuild output (gitignored; build before cargo)
src/
  copyright.rs  copyright_years()
  templates.rs  render_index_html(), error pages
  axum.rs       router() — GET /, /static/*, /favicon.ico
  warp.rs       routes() — full warp filter stack
```

## Frontend (TypeScript)

Browser source is TypeScript only (`ts/src/`). esbuild writes gitignored bundles to `assets/static/js/` (required before `cargo build` because of `rust-embed`):

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

## Workspace

This crate lives in the `sigma/` Cargo workspace alongside `identity` and `sigmatacticalgroup.com`. Depend on it via:

```toml
sigma-theme = { path = "../sigma-theme" }
```
