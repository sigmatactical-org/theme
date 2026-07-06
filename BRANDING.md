# Sigma Tactical Group — copyright & branding

© Sigma Tactical Group. **All rights reserved.**

## Proprietary branding

The following are **proprietary** to Sigma Tactical Group and are **not** open-source or freely reusable, even when source code in this repository is under MIT/Apache-2.0:

- **Name and wordmarks** — “Sigma Tactical Group”, product names, and nav titles
- **Logos and marks** — favicons, the sigma eye / background SVG, dial artwork
- **Visual identity** — site chrome in `assets/static/css/`, Bootstrap theme overrides, Keycloak login theme under `assets/keycloak/sigma/`
- **Compiled front-end bundles** — `/static/js/*` built from `ts/` (behavior and presentation together)

Do **not** use these assets in other products, forks, demos, or marketing without **written permission** from Sigma Tactical Group.

## Source code license

Rust crates and application code that depend on sigma-theme may be licensed under **MIT OR Apache-2.0** (see `LICENSE-MIT` and `LICENSE-APACHE` in this directory). That license applies to **source code**, not to the proprietary branding above.

## Web footer

Public sites render:

> © {year} Sigma Tactical Group. All rights reserved.

via `sigma_theme::copyright_years()` and the shared Askama base template.

## Questions

Brand or trademark use: contact Sigma Tactical Group before redistribution or external use.
