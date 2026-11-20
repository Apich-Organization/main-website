#!/usr/bin/env bash
# Builds the static site into ./dist
#
#   ./scripts/build.sh          # optimised build (what CI runs)
#   ./scripts/build.sh dev      # fast build for local iteration
#
# The client bundle is produced by trunk, every page is prerendered by the
# `ssg` binary, then assets are copied into place.
set -euo pipefail
# Git Bash on Windows rewrites arguments that look like paths ("/pkg/app.js").
export MSYS_NO_PATHCONV=1
cd "$(dirname "$0")/.."

MODE="${1:-release}"

echo "▸ building client bundle (wasm)"
if [ "$MODE" = "dev" ]; then
  trunk build --features hydrate
else
  trunk build --release --features hydrate -M
fi
rm -f build/pkg/index.html

echo "▸ prerendering pages"
cargo run --quiet --features ssr --bin ssg -- dist /pkg/apich-site.js /assets/css/style.css

echo "▸ assembling dist"
mkdir -p dist/pkg
cp -r build/pkg/. dist/pkg/
rm -f dist/pkg/index.html
mkdir -p dist/assets
cp -r assets/. dist/assets/
cp assets/favicon.svg dist/favicon.svg
cp assets/favicon.ico dist/favicon.ico

# Report payload sizes so regressions are visible in CI logs.
echo "▸ payload"
du -h dist/pkg/*.wasm dist/pkg/*.js dist/assets/css/style.css 2>/dev/null | sed 's/^/   /'
echo "▸ done: $(find dist -name 'index.html' | wc -l) pages"
