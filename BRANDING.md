# Sigma Tactical Group — brand & artwork

© Sigma Tactical Group. **All rights reserved.**

## Proprietary brand and artwork

The following are **proprietary** to Sigma Tactical Group and are **not** licensed under MIT, Apache-2.0, or any other open-source terms that may apply to source code in this repository:

- **Names and wordmarks** — “Sigma Tactical Group”, “Sigma”, product names, and nav titles
- **Logos and marks** — favicons, the sigma eye / background SVG, dial artwork, and related marks
- **Artwork and visual identity** — site chrome in `assets/static/css/`, Bootstrap theme overrides, Keycloak login theme under `assets/keycloak/sigma/`, illustrations, and presentation assets
- **Compiled front-end bundles** — `/static/js/*` built from `ts/` (behavior and presentation together)

Do **not** use these assets in other products, forks, demos, or marketing without **written permission** from Sigma Tactical Group.

Presence of brand assets in this repository for product use does **not** grant a license to reuse them.

## Source code license

Rust crates and application code that depend on sigma-theme may be licensed under **MIT OR Apache-2.0** (see `LICENSE-MIT` and `LICENSE-APACHE` in this directory). That license applies to **source code**, not to the proprietary brand and artwork above.

## Web footer

Public sites render:

> © {year} Sigma Tactical Group. All rights reserved.

via `sigma_theme::copyright_years()` and the shared Askama base template.

## Questions

Brand or trademark use: contact Sigma Tactical Group before redistribution or external use.
