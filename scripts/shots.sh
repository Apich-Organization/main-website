#!/usr/bin/env bash
# Captures full-page screenshots of the built site for design review.
#   ./scripts/shots.sh [base-url] [out-dir]
# Requires Playwright (`npx playwright`) and an installed Chromium or Edge.
set -euo pipefail
BASE="${1:-http://127.0.0.1:8080}"
OUT="${2:-.shots}"
mkdir -p "$OUT"

CHANNEL="${PLAYWRIGHT_CHANNEL:-msedge}"

shot() {
  local path="$1" name="$2" width="${3:-1440}" height="${4:-900}"
  npx --yes playwright screenshot \
    --channel="$CHANNEL" \
    --viewport-size="$width,$height" \
    --full-page \
    --wait-for-timeout=900 \
    "$BASE$path" "$OUT/$name.png" >/dev/null 2>&1 \
    && echo "  $name" || echo "  FAILED $name"
}

echo "capturing $BASE → $OUT"
shot "/" home
shot "/projects/" projects
shot "/projects/dtact/" dtact
shot "/projects/ensia/" ensia
shot "/projects/rssn/" rssn
shot "/research/" research
shot "/security/" security
shot "/sponsor/" sponsor
shot "/blog/" blog
shot "/blog/sponsorships-moving-to-open-collective/" post
shot "/blog/hello-apich/" post-two
shot "/contact/" contact
shot "/404.html" notfound
shot "/" home-mobile 420 900
shot "/projects/dtact/" dtact-mobile 420 900
echo "done"
