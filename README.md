# apich.org

The Apich Organization website: one statically generated site for the
organization, its teams, and every project that used to have its own subdomain.

Built with Leptos 0.8 in two modes from one codebase — server-side rendering at
build time, hydration in the browser — and deployed to GitHub Pages.

```
cargo + trunk (hydrate)  →  wasm bundle
cargo run --bin ssg      →  one HTML document per route
                            sitemap, robots, llms.txt, RSS, JSON exports
```

## Quick start

```bash
./scripts/build.sh dev      # fast build, output in dist/
python3 -m http.server -d dist 8080
```

```bash
./scripts/build.sh          # optimised build (wasm-opt, minified)
python3 scripts/verify.py dist
node scripts/audit.mjs http://127.0.0.1:8080 .shots   # needs playwright-core
```

Requires Rust (stable, with the `wasm32-unknown-unknown` target) and
[trunk](https://trunkrs.dev). ImageMagick 7 is only needed to regenerate the
social cards (`scripts/make-og.sh`).

## Layout

| Path | What lives there |
| --- | --- |
| `src/route.rs` | The route table: one enum, path parsing, legacy host map |
| `src/site.rs` | Organization identity, contact addresses, funding tiers |
| `src/content/versions.rs` | Generated: the published version of each project |
| `src/content/projects.rs` | Project facts: licence, stats, features |
| `src/content/details.rs` | Long-form project content: prose, tables, install, links |
| `src/content/posts.rs` | Blog posts (HTML bodies, kept out of the WASM bundle) |
| `src/pages/` | One component per route |
| `src/components/` | Navigation, footer, cards, icons, code blocks |
| `src/meta.rs` | Titles, descriptions, canonical URLs, JSON-LD |
| `src/bin/ssg.rs` | The static site generator |
| `assets/css/style.css` | The design system (tokens, glass, layout, responsive) |
| `assets/fonts/` | Self-hosted variable fonts (Inter, Newsreader, JetBrains Mono) |
| `dns/` | Records and redirect rules for the retired subdomains |
| `scripts/` | Build, verification, social card generation, version refresh |

## How rendering works

`Route` is plain data. The SSG resolves it from the output path and renders
`<App initial=route/>` with `to_html_branching()`, which emits the hydration
markers Leptos needs. The WASM build resolves the *same* route from
`window.location.pathname`, so the first client render matches the server HTML
exactly and hydration is silent.

Client navigation is a ~120-line router in `src/components/nav.rs`: click
delegation, `history.pushState`, scroll and focus restoration, head updates and
a prefetch hint. Every link is a real `<a href>`, so the site works with
JavaScript disabled and crawlers never need to execute anything.

## Versions

Every version shown on the site — cards, spec tables, install snippets, JSON-LD,
`/data/projects.json`, `llms.txt` — comes from `src/content/versions.rs`. That
file is generated; do not edit it by hand.

```bash
python3 scripts/update_versions.py          # refresh from the release sources
python3 scripts/update_versions.py --check  # report drift, write nothing
```

Crates are read from the crates.io API (`max_stable_version`), ensia from its
git tags, and aevia from the workspace version in its `Cargo.toml`. The script
only moves a version forward: if a registry lags behind an announced release it
keeps what the site already shows and prints a note. `--force` overrides that.

`deploy.yml` runs the script on a nightly schedule and commits the result when
something moved, so a release is reflected without anyone touching the site.
A failed lookup is never fatal — the previous value stays.

## Adding a project

1. Add a variant to `Project` in `src/route.rs` (slug, name, legacy hosts) and
   to `all_routes()`.
2. Add a constant to `src/content/versions.rs` and an entry to `SOURCES` in
   `scripts/update_versions.py` (crate, tags or manifest).
3. Fill in `src/content/projects.rs` (facts, stats, features) and
   `src/content/details.rs` (prose, mechanisms, table, install, links).
4. Add the accent colour pair to `assets/css/style.css`
   (`--pc-<slug>` for light and dark).
5. Add the host to the bootstrap map in `src/bin/ssg.rs` and to `dns/README.md`.
6. `./scripts/build.sh && python3 scripts/verify.py dist`.

## Editorial rules

* Every number on a project page comes from that project's manifest, README or
  published benchmark. Figures read off a chart say so in the caption.
* No release stage, branch or beta language: the site presents shipped software.
* No promises about toolchain support. MSRV policy is stated as a ceiling —
  current stable or stable − 3, per project — never as the version actually
  required, which can be lower.
* Funding links point at the project's own Open Collective budget where one
  exists, and at the organization collective otherwise.
* Descriptions are 60–180 characters; titles are unique and under 75.

## Accessibility and performance notes

* Skip link, focus-visible rings, `aria-current` on breadcrumbs, labelled
  controls, `prefers-reduced-motion` support, and a print stylesheet.
* No third-party requests: fonts, styles, scripts and images are all same-origin.
* The only JavaScript is the WASM bundle plus one inline bootstrap script that
  sets the colour theme before first paint.

## Deployment

`.github/workflows/deploy.yml` runs on every push to `main`, on a nightly
schedule, and on manual dispatch:

1. **Refresh versions** (schedule and dispatch only). `scripts/update_versions.py`
   runs, and the result is committed back to `main` when a version moved. The
   step is `continue-on-error`, so a flaky registry never blocks a deploy.
2. **Build.** `bash scripts/build.sh` — trunk compiles the WASM bundle, `ssg`
   renders every route, assets are copied into `dist/`.
3. **Verify.** `python3 scripts/verify.py dist` fails the run on missing
   metadata, duplicate titles, broken internal links, a dead `llms.txt` entry,
   an unparseable JSON file, or a PGP key that is not a key.
4. **Deploy.** `actions/deploy-pages` publishes `dist/`.

Repository settings: **Pages → Source** must be *GitHub Actions*, and the custom
domain `apich.org`. `dist/CNAME` pins it. The workflow asks for
`contents: write` so the nightly run can commit the refreshed versions; no
secrets are needed.

`scripts/audit.mjs` is a local check only — it drives a real browser (hydration,
client navigation, the theme toggle, the mobile drawer) and needs
`playwright-core`, which CI does not install.

See [`dns/README.md`](dns/README.md) for the records that make the old project
subdomains resolve here.

## Security contact

The published key lives at `assets/security-pgp.asc` and is served at
`/assets/security-pgp.asc`; the fingerprint on `/security/` must match it:

```bash
gpg --show-keys assets/security-pgp.asc
```

## License

Site content is CC BY 4.0. Code in this repository is MIT OR Apache-2.0. Each
project keeps its own license, stated on its page.
