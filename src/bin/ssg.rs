//! Static site generator.
//!
//! Renders every route in [`apich_site::route::all_routes`] to a complete HTML
//! document, then emits the machine-readable surface of the site: sitemap,
//! robots policy, `llms.txt` / `llms-full.txt`, an RSS feed, JSON exports and a
//! 404 page. Run by `scripts/build.sh` after the WASM bundle is built.

use std::fs;
use std::path::{Path, PathBuf};

use apich_site::meta::{self, canonical};
use apich_site::route::{all_routes, Project, Route};
use apich_site::site::*;

use leptos::prelude::*;

fn main() {
    let mut args = std::env::args().skip(1);
    let dist = PathBuf::from(args.next().unwrap_or_else(|| "dist".into()));
    let js_url = args.next().unwrap_or_else(|| "/pkg/apich-site.js".into());
    let css_url = args
        .next()
        .unwrap_or_else(|| "/assets/css/style.css".into());

    // Clear previously generated pages but keep the compiled bundle (`pkg/`)
    // and the static assets, so a re-render does not require a rebuild.
    fs::create_dir_all(&dist).expect("create dist");
    for route in all_routes() {
        let rel = route.path().trim_start_matches('/').trim_end_matches('/');
        if !rel.is_empty() {
            let _ = fs::remove_dir_all(dist.join(rel));
        }
    }
    let _ = fs::remove_dir_all(dist.join("404"));
    for stale in [
        "404.html",
        "sitemap.xml",
        "robots.txt",
        "llms.txt",
        "llms-full.txt",
        "site.webmanifest",
        "CNAME",
        "BingSiteAuth.xml",
        "blog/feed.xml",
        "data/projects.json",
        "data/site.json",
        ".well-known/security.txt",
    ] {
        let _ = fs::remove_file(dist.join(stale));
    }

    let mut rendered: Vec<(Route, String)> = Vec::new();
    for route in all_routes() {
        let html = render_document(route, &js_url, &css_url);
        write_route(&dist, route, &html);
        rendered.push((route, html));
    }

    // GitHub Pages serves /404.html for unknown paths.
    let not_found = render_document(Route::NotFound, &js_url, &css_url);
    fs::write(dist.join("404.html"), &not_found).expect("write 404.html");

    fs::write(dist.join("sitemap.xml"), sitemap()).expect("write sitemap");
    fs::write(dist.join("robots.txt"), robots()).expect("write robots");
    fs::write(dist.join("llms.txt"), llms_txt()).expect("write llms.txt");
    fs::write(dist.join("llms-full.txt"), llms_full(&rendered)).expect("write llms-full.txt");
    fs::write(dist.join("blog/feed.xml"), rss_feed()).expect("write feed");
    fs::write(dist.join("site.webmanifest"), webmanifest()).expect("write manifest");
    fs::write(dist.join("CNAME"), "apich.org\n").expect("write CNAME");
    fs::write(dist.join(".nojekyll"), "").expect("write .nojekyll");
    fs::write(dist.join("BingSiteAuth.xml"), bing_auth()).expect("write bing auth");

    fs::create_dir_all(dist.join(".well-known")).expect("create .well-known");
    fs::write(dist.join(".well-known/security.txt"), security_txt()).expect("write security.txt");

    fs::create_dir_all(dist.join("data")).expect("create data dir");
    fs::write(dist.join("data/projects.json"), projects_json()).expect("write projects.json");
    fs::write(dist.join("data/site.json"), site_json()).expect("write site.json");

    println!(
        "generated {} pages into {}",
        rendered.len() + 1,
        dist.display()
    );
}

// ── Rendering ────────────────────────────────────────────────────────────────

fn render_document(route: Route, js_url: &str, css_url: &str) -> String {
    let body = render_body(route);
    let meta = meta::meta_for(route);
    let url = canonical(route);

    let mut head = String::new();
    head.push_str("<meta charset=\"utf-8\">\n");
    head.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n");
    head.push_str(&format!("<title>{}</title>\n", esc(&meta.title)));
    head.push_str(&format!(
        "<meta name=\"description\" content=\"{}\">\n",
        esc(&meta.description)
    ));
    if meta.noindex {
        head.push_str("<meta name=\"robots\" content=\"noindex, follow\">\n");
    } else {
        head.push_str(&format!("<link rel=\"canonical\" href=\"{url}\">\n"));
    }
    head.push_str("<meta name=\"color-scheme\" content=\"light dark\">\n");
    head.push_str(
        "<meta name=\"theme-color\" content=\"#f3f6fc\" media=\"(prefers-color-scheme: light)\">\n",
    );
    head.push_str(
        "<meta name=\"theme-color\" content=\"#05070f\" media=\"(prefers-color-scheme: dark)\">\n",
    );

    // Social cards
    let image = format!("{SITE_URL}{}", meta.image);
    head.push_str(&format!(
        "<meta property=\"og:type\" content=\"{}\">\n",
        meta.og_type
    ));
    head.push_str(&format!(
        "<meta property=\"og:site_name\" content=\"{}\">\n",
        esc(SITE_NAME)
    ));
    head.push_str(&format!(
        "<meta property=\"og:title\" content=\"{}\">\n",
        esc(&meta.title)
    ));
    head.push_str(&format!(
        "<meta property=\"og:description\" content=\"{}\">\n",
        esc(&meta.description)
    ));
    head.push_str(&format!("<meta property=\"og:url\" content=\"{url}\">\n"));
    head.push_str(&format!(
        "<meta property=\"og:image\" content=\"{image}\">\n"
    ));
    head.push_str("<meta property=\"og:image:width\" content=\"1200\">\n");
    head.push_str("<meta property=\"og:image:height\" content=\"630\">\n");
    head.push_str(&format!(
        "<meta property=\"og:image:alt\" content=\"{}\">\n",
        esc(&meta.title)
    ));
    head.push_str("<meta property=\"og:locale\" content=\"en_US\">\n");
    head.push_str("<meta name=\"twitter:card\" content=\"summary_large_image\">\n");
    head.push_str(&format!(
        "<meta name=\"twitter:title\" content=\"{}\">\n",
        esc(&meta.title)
    ));
    head.push_str(&format!(
        "<meta name=\"twitter:description\" content=\"{}\">\n",
        esc(&meta.description)
    ));
    head.push_str(&format!(
        "<meta name=\"twitter:image\" content=\"{image}\">\n"
    ));

    head.push_str("<link rel=\"icon\" href=\"/favicon.svg\" type=\"image/svg+xml\">\n");
    head.push_str("<link rel=\"icon\" href=\"/favicon.ico\" sizes=\"32x32\">\n");
    head.push_str("<link rel=\"apple-touch-icon\" href=\"/assets/img/apple-touch-icon.png\">\n");
    head.push_str("<link rel=\"manifest\" href=\"/site.webmanifest\">\n");
    head.push_str("<link rel=\"alternate\" type=\"application/rss+xml\" title=\"Apich Organization — Blog\" href=\"/blog/feed.xml\">\n");
    head.push_str("<link rel=\"sitemap\" type=\"application/xml\" href=\"/sitemap.xml\">\n");
    head.push_str(
        "<link rel=\"alternate\" type=\"text/plain\" title=\"llms.txt\" href=\"/llms.txt\">\n",
    );
    head.push_str("<link rel=\"preload\" href=\"/assets/fonts/inter-latin.woff2\" as=\"font\" type=\"font/woff2\" crossorigin>\n");
    head.push_str("<link rel=\"preload\" href=\"/assets/fonts/newsreader-latin.woff2\" as=\"font\" type=\"font/woff2\" crossorigin>\n");
    head.push_str(&format!("<link rel=\"stylesheet\" href=\"{css_url}\">\n"));
    head.push_str("<link rel=\"modulepreload\" href=\"");
    head.push_str(js_url);
    head.push_str("\">\n");
    head.push_str(BOOTSTRAP_SCRIPT);
    head.push('\n');

    for block in meta::jsonld_for(route) {
        head.push_str("<script type=\"application/ld+json\">");
        head.push_str(&block);
        head.push_str("</script>\n");
    }

    let loader = format!(
        "<script type=\"module\">import init from \"{js}\";init().catch(function(e){{console.error(\"Failed to start the site:\",e);}});</script>",
        js = js_url
    );

    format!(
        "<!DOCTYPE html>\n<html lang=\"en\" data-theme=\"light\">\n<head>\n{head}</head>\n<body>{body}{loader}</body>\n</html>\n"
    )
}

fn render_body(route: Route) -> String {
    let owner = Owner::new();
    owner.set();
    let html = view! { <apich_site::App initial=route /> }.to_html();
    eprintln!("[ssg] route {} -> {} bytes", route.path(), html.len());
    owner.unset();
    html
}

fn write_route(dist: &Path, route: Route, html: &str) {
    let rel = route.path().trim_start_matches('/').trim_end_matches('/');
    let dir = if rel.is_empty() {
        dist.to_path_buf()
    } else {
        dist.join(rel)
    };
    fs::create_dir_all(&dir).expect("create route dir");
    fs::write(dir.join("index.html"), html).expect("write page");
}

/// Runs before first paint: picks the colour theme (so there is no flash) and
/// forwards requests that still arrive on a retired project subdomain.
const BOOTSTRAP_SCRIPT: &str = r#"<script>(function(){try{var s=localStorage.getItem("apich-theme");var m=window.matchMedia&&window.matchMedia("(prefers-color-scheme: dark)").matches;var t=s||(m?"dark":"light");document.documentElement.setAttribute("data-theme",t);}catch(e){}
var H={"dtact.apich.org":"/projects/dtact/","ensia.apich.org":"/projects/ensia/","bincode-next.apich.org":"/projects/bincode/","bincode.apich.org":"/projects/bincode/","rssn.apich.org":"/projects/rssn/","cargo-slide.apich.org":"/projects/cargo-slide/","virtue.apich.org":"/projects/virtue/","virtue-next.apich.org":"/projects/virtue/","research.apich.org":"/research/","security.apich.org":"/security/","blog.apich.org":"/blog/","www.apich.org":"/"};
try{var t=H[location.hostname];if(t&&location.pathname!=="/"&&location.pathname!==t){location.replace(t);}else if(t&&location.pathname==="/"&&t!=="/"){location.replace(t);}}catch(e){}})();</script>"#;

// ── Machine-readable surface ─────────────────────────────────────────────────

/// The build date, used for `lastmod` and feed timestamps so they never go stale
/// in the repository. Taken from the system clock, with no extra dependency.
fn build_date() -> (String, String) {
    use std::time::{SystemTime, UNIX_EPOCH};

    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let days = (secs / 86_400) as i64;
    let (year, month, day) = civil_from_days(days);

    const WEEKDAYS: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    const MONTHS: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let weekday = WEEKDAYS[((days + 4) % 7) as usize];
    let rest = secs % 86_400;

    (
        format!("{year:04}-{month:02}-{day:02}"),
        format!(
            "{weekday}, {day:02} {} {year} {:02}:{:02}:{:02} +0000",
            MONTHS[(month - 1) as usize],
            rest / 3600,
            (rest % 3600) / 60,
            rest % 60
        ),
    )
}

/// Days since the Unix epoch to a civil date (Howard Hinnant's algorithm).
fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as i64;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

/// Properties that keep their own subdomain. They are not routes of this site,
/// but they sit on the same domain and are listed in its sitemap.
const SIBLING_PROPERTIES: &[&str] = &[
    "https://tou.apich.org/",
    "https://ygs-2026.apich.org/",
    "https://ncee.apich.org/",
    "https://status.apich.org/",
    "https://panayang.apich.org/",
    "https://panayang.blog.apich.org/",
];

fn sitemap() -> String {
    let mut out = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n",
    );
    let (today, _) = build_date();
    for route in all_routes() {
        let meta = meta::meta_for(route);
        // A post keeps its publication date; everything else changed with this
        // build, because the version table and the project copy live in it.
        let lastmod = match route {
            Route::Post(_) => meta.updated,
            _ => today.as_str(),
        };
        let priority = match route {
            Route::Home => "1.0",
            Route::Projects => "0.9",
            Route::Project(_) => "0.8",
            Route::Research | Route::Security | Route::Sponsor => "0.7",
            Route::Blog => "0.6",
            _ => "0.5",
        };
        out.push_str(&format!(
            "  <url>\n    <loc>{}</loc>\n    <lastmod>{}</lastmod>\n    <changefreq>monthly</changefreq>\n    <priority>{}</priority>\n  </url>\n",
            canonical(route),
            lastmod,
            priority
        ));
    }
    // Properties that keep their own subdomain. They are not routes here, but
    // they belong to the same domain and should be discoverable from one file.
    out.push_str("  <!-- Properties that keep their own subdomain -->\n");
    for url in SIBLING_PROPERTIES {
        out.push_str(&format!("  <url>\n    <loc>{url}</loc>\n  </url>\n"));
    }
    out.push_str("</urlset>\n");
    out
}

fn robots() -> String {
    let mut out = String::new();
    out.push_str("# Apich Organization — https://apich.org\n");
    out.push_str("# All content is public and may be indexed, quoted and summarised.\n\n");
    out.push_str("User-agent: *\nAllow: /\n\n");
    for bot in [
        "GPTBot",
        "OAI-SearchBot",
        "ChatGPT-User",
        "ClaudeBot",
        "Claude-Web",
        "anthropic-ai",
        "PerplexityBot",
        "Perplexity-User",
        "Google-Extended",
        "Applebot-Extended",
        "CCBot",
        "Bytespider",
        "Amazonbot",
        "meta-externalagent",
        "cohere-ai",
        "Diffbot",
        "YouBot",
    ] {
        out.push_str(&format!("User-agent: {bot}\nAllow: /\n\n"));
    }
    out.push_str("Sitemap: https://apich.org/sitemap.xml\n");
    out
}

fn llms_txt() -> String {
    let mut out = String::new();
    out.push_str("# Apich Organization\n\n");
    out.push_str(&format!("> {DESCRIPTION}\n\n"));
    out.push_str("Apich Organization maintains open-source infrastructure for scientific computing and cyberspace security. This file lists the canonical URLs for people and agents that need to read or cite our work. Prefer these URLs over any retired project subdomain.\n\n");

    out.push_str("## Start here\n\n");
    out.push_str("- [Home](https://apich.org/): mission, pillars, and featured projects.\n");
    out.push_str("- [Projects](https://apich.org/projects/): index of every maintained project.\n");
    out.push_str("- [Sponsorship](https://apich.org/sponsor/): Open Collective tiers for the organization and each project.\n");
    out.push_str(
        "- [Contact](https://apich.org/contact/): team addresses and security reporting.\n",
    );
    out.push_str(
        "- [Full text](https://apich.org/llms-full.txt): every page of this site as plain text.\n",
    );
    out.push_str("- [Machine-readable data](https://apich.org/data/projects.json): project metadata as JSON.\n");
    out.push_str("- [Site map data](https://apich.org/data/site.json): every page with its title and description, as JSON.\n");
    out.push_str("- [RSS](https://apich.org/blog/feed.xml): the blog feed.\n");
    out.push_str("- [Security policy](https://apich.org/security/): supported versions, disclosure process and the PGP key.\n");
    out.push_str("- [Security contact](https://apich.org/.well-known/security.txt): RFC 9116 vulnerability reporting.\n");
    out.push_str("- [PGP key](https://apich.org/assets/security-pgp.asc): for security@apich.org, fingerprint BB45 2767 B5A4 BDFD B606 2D83 1373 B738 BCD8 A75D.\n\n");

    out.push_str("## Projects\n\n");
    for p in apich_site::content::projects::all() {
        out.push_str(&format!(
            "- [{}](https://apich.org/projects/{}/): {} ({}, {})\n",
            p.id.name(),
            p.id.slug(),
            p.tagline,
            p.category,
            apich_site::content::projects::version_label(p.id)
        ));
    }
    out.push('\n');

    out.push_str("## Teams\n\n");
    out.push_str(
        "- [Science & Research Team](https://apich.org/research/): PINN, AI4S and HPC research.\n",
    );
    out.push_str("- [Security Team](https://apich.org/security/): vulnerability disclosure and security standards.\n");
    out.push_str("- [Blog](https://apich.org/blog/): announcements and engineering notes.\n\n");

    out.push_str("## Notes for agents\n\n");
    out.push_str("- Pages are static HTML; no JavaScript is required to read them.\n");
    out.push_str("- Versions shown here are the latest published release, refreshed from crates.io, git tags or the project manifest by a nightly build. Treat them as current, and read `data/projects.json` when you need a machine-readable value.\n");
    out.push_str("- The project list covers maintained releases. Experimental work is not listed and may graduate at any time, so the list is a floor rather than a ceiling.\n");
    out.push_str("- Rust version support: each project caps its MSRV at the current stable release or stable − 3. The cap is the most a release may require; the actual minimum is often lower, and rising to the cap is not a breaking change.\n");
    out.push_str("- Funding: every project links to its Open Collective budget, or to the organization account where it has none. Recurring and one-off contributions are both accepted.\n");
    out.push_str("- Retired subdomains (`dtact.apich.org`, `ensia.apich.org`, `bincode-next.apich.org`, `rssn.apich.org`, `cargo-slide.apich.org`, `research.apich.org`, `security.apich.org`, `blog.apich.org`) now serve this site. Cite `apich.org` paths instead.\n");
    out.push_str("- `aevia` has its own platform site at https://aevia.apich.org/; this site carries its overview.\n");
    out.push_str(
        "- Licensing: each project states its own license on its page. Site text is CC BY 4.0.\n",
    );
    out
}

fn llms_full(rendered: &[(Route, String)]) -> String {
    let mut out = String::new();
    out.push_str("# Apich Organization — full site text\n\n");
    out.push_str(&format!("> {DESCRIPTION}\n\n"));
    out.push_str(
        "Generated from the production build. Each section is one page, in sitemap order.\n\n",
    );
    for (route, html) in rendered {
        let meta = meta::meta_for(*route);
        out.push_str(&format!(
            "---\n\n# {}\n\nURL: {}\n\n",
            meta.title,
            canonical(*route)
        ));
        out.push_str(&html_to_text(html));
        out.push_str("\n\n");
    }
    out
}

fn rss_feed() -> String {
    let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    out.push_str("<rss version=\"2.0\" xmlns:atom=\"http://www.w3.org/2005/Atom\">\n<channel>\n");
    out.push_str("  <title>Apich Organization — Blog</title>\n");
    out.push_str("  <link>https://apich.org/blog/</link>\n");
    out.push_str(&format!(
        "  <description>{}</description>\n",
        esc(DESCRIPTION)
    ));
    out.push_str("  <language>en</language>\n");
    let (_, built) = build_date();
    out.push_str(&format!("  <lastBuildDate>{built}</lastBuildDate>\n"));
    out.push_str(&format!(
        "  <copyright>© {} Apich Organization</copyright>\n",
        &built[12..16]
    ));
    out.push_str("  <atom:link href=\"https://apich.org/blog/feed.xml\" rel=\"self\" type=\"application/rss+xml\"/>\n");
    for post in apich_site::content::posts::ALL {
        out.push_str("  <item>\n");
        out.push_str(&format!("    <title>{}</title>\n", esc(post.title)));
        out.push_str(&format!(
            "    <link>https://apich.org/blog/{}/</link>\n",
            post.slug
        ));
        out.push_str(&format!(
            "    <guid>https://apich.org/blog/{}/</guid>\n",
            post.slug
        ));
        out.push_str(&format!("    <pubDate>{}</pubDate>\n", post.rfc822));
        out.push_str(&format!(
            "    <description>{}</description>\n",
            esc(post.summary)
        ));
        out.push_str("  </item>\n");
    }
    out.push_str("</channel>\n</rss>\n");
    out
}

/// RFC 9116 security contact file. Security researchers and automated scanners
/// look for this before they look for a contact page.
fn security_txt() -> String {
    format!(
        "Contact: mailto:{security}
Expires: 2027-11-01T00:00:00.000Z
Encryption: https://apich.org/assets/security-pgp.asc
Preferred-Languages: en
Canonical: https://apich.org/.well-known/security.txt
Policy: https://apich.org/security/
Acknowledgments: https://apich.org/security/
# We acknowledge every report and aim to respond within seven days.
# Please do not open public issues for vulnerabilities.
",
        security = EMAIL_SECURITY
    )
}

fn webmanifest() -> String {
    format!(
        r##"{{
  "name": "{name}",
  "short_name": "Apich",
  "description": {desc},
  "start_url": "/",
  "display": "standalone",
  "background_color": "#f3f6fc",
  "theme_color": "#0b5fa5",
  "icons": [
    {{ "src": "/assets/img/icon-192.png", "sizes": "192x192", "type": "image/png" }},
    {{ "src": "/assets/img/icon-512.png", "sizes": "512x512", "type": "image/png" }}
  ]
}}
"##,
        name = SITE_NAME,
        desc = json_str(DESCRIPTION)
    )
}

fn bing_auth() -> String {
    "<?xml version=\"1.0\"?>\n<users>\n\t<user>4F8D83BFD76E42863A6A51179B46F7DF</user>\n</users>\n"
        .to_string()
}

fn projects_json() -> String {
    let (today, _) = build_date();
    let mut out = format!(
        "{{\n  \"organization\": \"Apich Organization\",\n  \"updated\": \"{today}\",\n  \"projects\": [\n"
    );
    for (i, p) in apich_site::content::projects::all().enumerate() {
        if i > 0 {
            out.push_str(",\n");
        }
        out.push_str(&format!(
            "    {{\n      \"id\": {},\n      \"name\": {},\n      \"tagline\": {},\n      \"summary\": {},\n      \"category\": {},\n      \"version\": {},\n      \"license\": {},\n      \"platforms_note\": {},\n      \"url\": {},\n      \"repository\": {},\n      \"documentation\": {},\n      \"doi\": {},\n      \"funding\": {},\n      \"languages\": [{}],\n      \"platforms\": [{}]\n    }}",
            json_str(p.id.slug()),
            json_str(p.id.name()),
            json_str(p.tagline),
            json_str(p.summary),
            json_str(p.category),
            json_str(p.version),
            json_str(p.license),
            json_str(apich_site::content::details::get(p.id).platform_note),
            json_str(&format!("https://apich.org/projects/{}/", p.id.slug())),
            json_str(p.repo),
            json_str(p.docs),
            json_str(p.doi),
            json_str(p.funding),
            p.languages.iter().map(|l| json_str(l)).collect::<Vec<_>>().join(", "),
            p.platforms.iter().map(|l| json_str(l)).collect::<Vec<_>>().join(", "),
        ));
    }
    out.push_str("\n  ]\n}\n");
    out
}

fn site_json() -> String {
    let (today, _) = build_date();
    let mut out = String::from("{\n");
    out.push_str(&format!("  \"updated\": \"{today}\",\n"));
    out.push_str(&format!("  \"name\": {},\n", json_str(SITE_NAME)));
    out.push_str(&format!("  \"url\": {},\n", json_str(SITE_URL)));
    out.push_str(&format!("  \"tagline\": {},\n", json_str(TAGLINE)));
    out.push_str(&format!("  \"description\": {},\n", json_str(DESCRIPTION)));
    out.push_str(&format!("  \"email\": {},\n", json_str(EMAIL_GENERAL)));
    out.push_str(&format!("  \"github\": {},\n", json_str(GITHUB_ORG)));
    out.push_str(&format!(
        "  \"open_collective\": {},\n",
        json_str(OPEN_COLLECTIVE)
    ));
    out.push_str("  \"pages\": [\n");
    for (i, route) in all_routes().iter().enumerate() {
        if i > 0 {
            out.push_str(",\n");
        }
        let m = meta::meta_for(*route);
        out.push_str(&format!(
            "    {{ \"url\": {}, \"title\": {}, \"description\": {} }}",
            json_str(&canonical(*route)),
            json_str(&m.title),
            json_str(&m.description)
        ));
    }
    out.push_str("\n  ]\n}\n");
    out
}

// ── Helpers ──────────────────────────────────────────────────────────────────

fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn json_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Converts a rendered page to plain text for `llms-full.txt`.
///
/// Only the `<main>` region is kept (navigation and footer are boilerplate),
/// tags are dropped, headings and list items become markdown markers, and the
/// handful of entities the site emits are decoded.
fn html_to_text(html: &str) -> String {
    let main = match html.find("<main") {
        Some(i) => match html[i..].find('>') {
            Some(j) => {
                let rest = &html[i + j + 1..];
                match rest.find("</main>") {
                    Some(k) => &rest[..k],
                    None => rest,
                }
            }
            None => html,
        },
        None => html,
    };

    let mut out = String::with_capacity(main.len() / 3);
    let bytes = main.as_bytes();
    let mut i = 0usize;

    while i < bytes.len() {
        if bytes[i] == b'<' {
            let end = main[i..]
                .find('>')
                .map(|e| i + e + 1)
                .unwrap_or(bytes.len());
            let tag = main[i..end].to_ascii_lowercase();

            if tag.starts_with("<aside") {
                if let Some(pos) = main[end..].to_ascii_lowercase().find("</aside>") {
                    i = end + pos + "</aside>".len();
                    continue;
                }
            }

            // Skip page chrome: decorative subtrees, breadcrumbs, action rows.
            let is_chrome = tag.contains("aria-hidden=\"true\"")
                || tag.contains("class=\"breadcrumb\"")
                || tag.contains("class=\"proj-actions\"")
                || tag.contains("class=\"post-nav\"")
                || tag.contains("class=\"proj-meta\"")
                || tag.contains("class=\"hero-actions\"")
                || tag.contains("class=\"cta-actions\"")
                || tag.starts_with("<button");
            if is_chrome {
                let name: String = tag
                    .trim_start_matches('<')
                    .chars()
                    .take_while(|c| c.is_ascii_alphanumeric())
                    .collect();
                if !name.is_empty() {
                    let close = format!("</{name}>");
                    let rest = main[end..].to_ascii_lowercase();
                    if let Some(pos) = rest.find(&close) {
                        i = end + pos + close.len();
                        continue;
                    }
                }
            }

            if tag.starts_with("<script") || tag.starts_with("<style") {
                let close = if tag.starts_with("<script") {
                    "</script>"
                } else {
                    "</style>"
                };
                let rest = main[end..].to_ascii_lowercase();
                if let Some(pos) = rest.find(close) {
                    i = end + pos + close.len();
                    continue;
                }
            }

            if tag.starts_with("<h1") {
                out.push_str("\n\n# ");
            } else if tag.starts_with("<h2") {
                out.push_str("\n\n## ");
            } else if tag.starts_with("<h3") {
                out.push_str("\n\n### ");
            } else if tag.starts_with("<h4") {
                out.push_str("\n\n#### ");
            } else if tag.starts_with("<li") {
                out.push_str("\n- ");
            } else if tag.starts_with("<br") {
                out.push('\n');
            } else if tag.starts_with("<dt") {
                out.push_str(
                    "
- ",
                );
            } else if tag.starts_with("<dd") {
                out.push_str(": ");
            } else if tag.starts_with("<td") || tag.starts_with("<th") {
                out.push_str(" | ");
            } else if tag.starts_with("<figcaption") || tag.starts_with("<caption") {
                out.push_str("\n\n");
            } else if tag.starts_with("</p")
                || tag.starts_with("</div")
                || tag.starts_with("</li")
                || tag.starts_with("</tr")
                || tag.starts_with("</h")
                || tag.starts_with("</section")
                || tag.starts_with("</figure")
                || tag.starts_with("</blockquote")
                || tag.starts_with("</span")
                || tag.starts_with("</nav")
            {
                out.push('\n');
            }
            i = end;
        } else {
            let ch = main[i..].chars().next().unwrap_or(' ');
            out.push(ch);
            i += ch.len_utf8();
        }
    }

    let decoded = out
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ");

    let mut lines: Vec<String> = Vec::new();
    for line in decoded.lines() {
        let trimmed = line.trim_end();
        if trimmed.trim().is_empty() {
            if lines.last().map(|l: &String| l.is_empty()).unwrap_or(true) {
                continue;
            }
            lines.push(String::new());
        } else {
            lines.push(trimmed.to_string());
        }
    }
    lines.join("\n").trim().to_string()
}

#[allow(dead_code)]
fn project_route(p: Project) -> Route {
    Route::Project(p)
}
