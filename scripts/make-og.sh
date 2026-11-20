#!/usr/bin/env bash
# Generates the Open Graph cards (1200x630) for the site and each project.
# Requires ImageMagick 7. Run from the repository root.
#
# The card layout wants a high-contrast serif, a neutral sans and a mono. The
# fonts are looked up from the usual system locations; override any of them by
# exporting GEORGIA / SANS / MONO before running.
set -euo pipefail
cd "$(dirname "$0")/.."
mkdir -p assets/img/og

pick_font() {
  local name="$1"
  shift
  local preset="${!name:-}"
  if [ -n "$preset" ] && [ -f "$preset" ]; then
    printf '%s' "$preset"
    return
  fi
  for candidate in "$@"; do
    if [ -f "$candidate" ]; then
      printf '%s' "$candidate"
      return
    fi
  done
}

GEORGIA="$(pick_font GEORGIA \
  "C:/Windows/Fonts/georgia.ttf" \
  "/System/Library/Fonts/Supplemental/Georgia.ttf" \
  "/usr/share/fonts/truetype/msttcorefonts/Georgia.ttf" \
  "/usr/share/fonts/truetype/liberation/LiberationSerif-Regular.ttf")"
SANS="$(pick_font SANS \
  "C:/Windows/Fonts/segoeui.ttf" \
  "/System/Library/Fonts/Supplemental/Arial.ttf" \
  "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf" \
  "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf")"
MONO="$(pick_font MONO \
  "C:/Windows/Fonts/consola.ttf" \
  "/System/Library/Fonts/Menlo.ttc" \
  "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf" \
  "/usr/share/fonts/truetype/liberation/LiberationMono-Regular.ttf")"

if [ -z "$GEORGIA" ] || [ -z "$SANS" ] || [ -z "$MONO" ]; then
  echo "error: could not find the fonts the cards need." >&2
  echo "       set GEORGIA=, SANS= and MONO= to .ttf paths and re-run." >&2
  exit 1
fi

# The organization mark: three identical ellipses offset along a diagonal.
# Geometry follows the official artwork (rx:ry = 613:359, rotated 44.3 degrees).
card() {
  local out="$1" title="$2" tagline="$3" url="$4" accent="$5"
  magick -size 1200x630 radial-gradient:"#12294a-#05070f" \
    -strokewidth 1.7 -fill none \
    -stroke "${accent}80" -draw "translate 1022,340 rotate 44.3 ellipse 0,0 188,110 0,360" \
    -stroke "${accent}bf" -draw "translate 1056,330 rotate 44.3 ellipse 0,0 188,110 0,360" \
    -stroke "$accent" -draw "translate 1090,320 rotate 44.3 ellipse 0,0 188,110 0,360" \
    -stroke none \
    -font "$MONO" -pointsize 21 -kerning 5 -fill "#7ea6d8" -annotate +80+108 "APICH ORGANIZATION" \
    -font "$GEORGIA" -pointsize 92 -kerning 0 -fill "#f3f7ff" -annotate +78+296 "$title" \
    -font "$SANS" -pointsize 31 -fill "#a6b9d6" -annotate +80+362 "$tagline" \
    -font "$MONO" -pointsize 22 -fill "#6d84a6" -annotate +80+552 "$url" \
    -stroke "#ffffff14" -strokewidth 1 -draw "line 80,600 1120,600" \
    -stroke none \
    -strip -interlace Plane -sampling-factor 4:2:0 -quality 86 \
    "$out"
}

# The organization card carries the real lockup rather than a wordmark in a
# system font, so the logo is what people see in a link preview.
brand_card() {
  local out="$1"
  magick -size 1200x630 radial-gradient:"#12294a-#05070f" \
    \( assets/img/brand/apich-logo-dark.png -resize 520x \) \
    -gravity north -geometry +0+84 -composite \
    -font "$SANS" -pointsize 30 -fill "#a6b9d6" -gravity north \
    -annotate +0+486 "Open infrastructure for science and security" \
    -font "$MONO" -pointsize 20 -kerning 5 -fill "#6d84a6" -gravity south \
    -annotate +0+52 "APICH.ORG" \
    -strip -interlace Plane -sampling-factor 4:2:0 -quality 88 \
    "$out"
}

brand_card assets/img/og/og-default.jpg
card assets/img/og/dtact.jpg "dtact" \
  "The universal topology-affinity async runtime" "apich.org/projects/dtact" "#8f7bff"
card assets/img/og/ensia.jpg "ensia" \
  "The next-generation LLVM obfuscator" "apich.org/projects/ensia" "#e57a9c"
card assets/img/og/bincode.jpg "bincode-next" \
  "High-performance binary serialization for Rust" "apich.org/projects/bincode" "#3d8fd6"
card assets/img/og/rssn.jpg "rssn" \
  "Scientific computing for Rust" "apich.org/projects/rssn" "#2fc2d6"
card assets/img/og/cargo-slide.jpg "cargo-slide" \
  "Code-driven presentations, rendered natively" "apich.org/projects/cargo-slide" "#d9a35c"
card assets/img/og/virtue.jpg "virtue" \
  "A sinless derive macro helper" "apich.org/projects/virtue" "#4fbf93"
card assets/img/og/aevia.jpg "aevia" \
  "Dimensions in the type system" "apich.org/projects/aevia" "#a98bff"

echo "wrote $(ls assets/img/og | wc -l) cards"
