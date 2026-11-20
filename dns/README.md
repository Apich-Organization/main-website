# DNS and legacy host handling

`apich.org` is served by GitHub Pages from the [`main-website`](../) repository.
Project sites were consolidated into paths on that origin, so every retired
project subdomain has to end up at the matching page.

Two mechanisms do that, and they are complementary:

1. **DNS (required).** Every retired subdomain becomes a `CNAME` to the Pages
   host, so the request lands on this site instead of a dead repository.
2. **Edge redirect (recommended).** A 301 rule at the CDN/DNS provider sends
   `dtact.apich.org/anything` to `apich.org/projects/dtact/` with a real status
   code. Search engines consolidate the old URLs immediately.
3. **In-page fallback (shipped).** If no edge rule exists, the site's bootstrap
   script rewrites the location before first paint, so a human still lands on the
   right page. Crawlers prefer the 301, which is why step 2 matters.

## Records to create

Replace `apich-organization.github.io` with the Pages host shown in the
repository settings if it differs.

| Host | Type | Value | TTL | Note |
| --- | --- | --- | --- | --- |
| `apich.org` | A | `185.199.108.153` | 3600 | GitHub Pages |
| `apich.org` | A | `185.199.109.153` | 3600 | |
| `apich.org` | A | `185.199.110.153` | 3600 | |
| `apich.org` | A | `185.199.111.153` | 3600 | |
| `apich.org` | AAAA | `2606:50c0:8000::153` | 3600 | IPv6 |
| `apich.org` | AAAA | `2606:50c0:8001::153` | 3600 | |
| `apich.org` | AAAA | `2606:50c0:8002::153` | 3600 | |
| `apich.org` | AAAA | `2606:50c0:8003::153` | 3600 | |
| `www.apich.org` | CNAME | `apich-organization.github.io` | 3600 | canonical host is apex |
| `dtact.apich.org` | CNAME | `apich-organization.github.io` | 3600 | → `/projects/dtact/` |
| `ensia.apich.org` | CNAME | `apich-organization.github.io` | 3600 | → `/projects/ensia/` |
| `bincode-next.apich.org` | CNAME | `apich-organization.github.io` | 3600 | → `/projects/bincode/` |
| `rssn.apich.org` | CNAME | `apich-organization.github.io` | 3600 | → `/projects/rssn/` |
| `cargo-slide.apich.org` | CNAME | `apich-organization.github.io` | 3600 | → `/projects/cargo-slide/` |
| `virtue.apich.org` | CNAME | `apich-organization.github.io` | 3600 | → `/projects/virtue/` |
| `research.apich.org` | CNAME | `apich-organization.github.io` | 3600 | → `/research/` |
| `security.apich.org` | CNAME | `apich-organization.github.io` | 3600 | → `/security/` |
| `blog.apich.org` | CNAME | `apich-organization.github.io` | 3600 | → `/blog/` |
| `aevia.apich.org` | CNAME | *(unchanged)* | 3600 | separate platform, not part of this consolidation |

`aevia.apich.org` keeps pointing at the platform deployment. This site only
carries its overview page and links out to it.

Machine-readable copy of the table: [`records.csv`](records.csv).

## 301 rules (recommended)

If the zone is on Cloudflare, add Redirect Rules rather than relying on the
JavaScript fallback. One rule per retired host, all with status `301`, preserve
query string, and *do not* append the path — the old sites were hash-routed, so
the path carries no meaning:

```
dtact.apich.org/*        → https://apich.org/projects/dtact/
ensia.apich.org/*        → https://apich.org/projects/ensia/
bincode-next.apich.org/* → https://apich.org/projects/bincode/
rssn.apich.org/*         → https://apich.org/projects/rssn/
cargo-slide.apich.org/*  → https://apich.org/projects/cargo-slide/
virtue.apich.org/*       → https://apich.org/projects/virtue/
research.apich.org/*     → https://apich.org/research/
security.apich.org/*     → https://apich.org/security/
blog.apich.org/*         → https://apich.org/blog/
www.apich.org/*          → https://apich.org/$1   (path preserved)
```

The site also ships `www.apich.org` → apex handling, but a 301 at the edge is
cleaner than a meta refresh in the browser.

## Things that must move

* **dtact benchmark reports.** The old site published Criterion output at
  `dtact.apich.org/bench/report/`. That host now serves this site, so the
  publishing workflow needs a new home — either `bench.dtact.apich.org` or a
  Pages subpath owned by the dtact repository. Until then the dtact page links
  to the workflow that produces the report rather than to the report itself.
* **Blog.** `blog.apich.org` used a separate repository and a separate
  deployment. Posts now live in this repository (`src/content/posts.rs`) and the
  old repository's Pages deployment can be retired.

## Verifying

```bash
# Records resolve to Pages
dig +short dtact.apich.org CNAME
dig +short apich.org A

# The certificate covers every legacy host (Pages issues one cert for the apex
# plus the custom domains added to the repository)
curl -sI https://dtact.apich.org/ | head -5

# Canonical host wins
curl -sI https://www.apich.org/ | grep -i location
```

## Checklist when adding a project

1. Add the project to `src/route.rs` and `src/content/projects.rs`.
2. Add its host to the `H` map in the bootstrap script (`src/bin/ssg.rs`) and to
   the table above.
3. Create the `CNAME` record and, if available, a 301 rule.
4. Confirm `sitemap.xml` picked up the new page (`python3 scripts/verify.py dist`).
