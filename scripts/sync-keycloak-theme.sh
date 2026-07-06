#!/usr/bin/env bash
# Copy sigma-theme Keycloak login assets into the platform Keycloak manifest tree.
# Run from a monorepo checkout with theme/ and platform/ as siblings under it/.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SRC="${ROOT}/assets/keycloak/sigma/login"
DEST="${SIGMA_PLATFORM_DIR:-$(cd "$ROOT/../platform" && pwd)}/services/identity/keycloak/base/theme/sigma/login"

if [[ ! -d "$SRC" ]]; then
  echo "error: missing Keycloak theme source at $SRC" >&2
  exit 1
fi

mkdir -p "$DEST"
cp -a "$SRC/." "$DEST/"
echo "Synced Keycloak theme to $DEST"
