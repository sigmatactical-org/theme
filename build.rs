//! Publishes this crate's asset directories to dependents' build scripts.
//!
//! Consumers extend the templates in `assets/templates` from their own
//! templates, which means Askama needs a filesystem path to them at compile
//! time. That path is only knowable here: when this crate is consumed over git,
//! it lives in Cargo's checkout directory under a rev-specific hash, not at any
//! path a consumer could hardcode.
//!
//! The `links = "sigma-theme"` key in `Cargo.toml` turns the values below into
//! `DEP_SIGMA_THEME_TEMPLATES` and `DEP_SIGMA_THEME_ASSETS` in the environment
//! of every direct dependent's build script. See `docs` in the README for the
//! consumer side, which writes `askama.toml` from it.

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("cargo always sets CARGO_MANIFEST_DIR for build scripts");
    println!("cargo::metadata=templates={manifest_dir}/assets/templates");
    println!("cargo::metadata=assets={manifest_dir}/assets");
    println!("cargo::rerun-if-changed=build.rs");
}
