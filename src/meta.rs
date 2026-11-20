//! Page metadata: titles, descriptions, canonical paths, social cards and
//! structured data. The SSG writes these into the document head; the client
//! router rewrites them on navigation so deep links stay accurate.

use crate::content::{posts, projects};
use crate::route::{Project, Route};
use crate::site::SITE_URL;

pub struct PageMeta {
    pub title: String,
    pub description: String,
    pub og_type: &'static str,
    pub image: String,
    pub updated: &'static str,
    /// True for pages that must not be indexed (404).
    pub noindex: bool,
}

fn absolute(path: &str) -> String {
    format!("{SITE_URL}{path}")
}

impl PageMeta {
    fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        PageMeta {
            title: title.into(),
            description: description.into(),
            og_type: "website",
            image: "/assets/img/og/og-default.jpg".to_string(),
            updated: "2026-11-19",
            noindex: false,
        }
    }

    fn image(mut self, path: impl Into<String>) -> Self {
        self.image = path.into();
        self
    }
}

/// A shorter tagline for the `<title>` of projects whose positioning line is
/// too long to survive a search-result truncation.
fn title_tagline(p: Project, tagline: &'static str) -> &'static str {
    match p {
        Project::Aevia => "a dimensional language for science",
        Project::CargoSlide => "presentations rendered natively",
        Project::Bincode => "binary serialization for Rust",
        _ => tagline,
    }
}

/// Hand-written meta descriptions. They are deliberately not generated from the
/// summaries: search engines truncate mid-sentence, and a clipped description
/// reads badly in results.
fn seo_description(p: Project) -> &'static str {
    match p {
        Project::Dtact => "A stackful coroutine runtime with a lock-free context arena, peer-to-peer mesh scheduling and assembly context switchers. Rust, C and C++.",
        Project::Ensia => "An LLVM obfuscation framework for LLVM 21 and later: fifteen passes in a twelve-stage cascade, with no central interpreter to devirtualize.",
        Project::Bincode => "A binary encoder and decoder for Rust with SIMD varint scanning, zero-copy decoding, bit packing, schema fingerprints and an RFC 8949 CBOR front end.",
        Project::Rssn => "A computer algebra system, numerical kernels and physics simulations behind one API: symbolic rewriting on an e-graph, with dimensions in every term.",
        Project::CargoSlide => "A presentation engine pairing Typst typesetting with a native Rust runtime: 60 FPS playback, a WYSIWYG editor, and three ways to deliver a deck.",
        Project::Virtue => "A small helper for writing proc-macro derives: parse an item, describe the impl you want, and let the type system catch the rest. Zero required dependencies.",
        Project::Aevia => "A functional language for scientific computing that rejects dimensionally inconsistent expressions before the program runs, with RSSN as its engine.",
    }
}

pub fn meta_for(route: Route) -> PageMeta {
    match route {
        Route::Home => PageMeta::new(
            "Apich Organization — open source for science and security",
            "An open-source organization building runtimes, compilers, serialization formats and numerical engines for scientific computing and a more resilient cyberspace.",
        ),
        Route::Projects => PageMeta::new(
            "Projects — the open-source tools we maintain",
            "A coroutine runtime, an LLVM obfuscator, a serialization format, a computer algebra system, a presentation engine, a derive helper and a dimensional language.",
        ),
        Route::Project(p) => {
            let c = projects::get(p);
            PageMeta::new(
                format!("{} — {}", p.name(), title_tagline(p, c.tagline)),
                seo_description(p),
            )
            .image(format!("/assets/img/og/{}.jpg", p.slug()))
        }
        Route::Research => PageMeta::new(
            "Science & Research Team — PINN, AI4S and HPC research",
            "An independent research unit working on physics-informed neural networks, AI for science and high-performance computing, with preprints on OSF and Zenodo.",
        ),
        Route::Security => PageMeta::new(
            "Security policy and vulnerability disclosure",
            "Supported versions, coordinated disclosure with a seven-day response commitment, the published PGP key, and the hardening baseline we hold ourselves to.",
        ),
        Route::Blog => PageMeta::new(
            "Blog — announcements and engineering notes",
            "Release notes, design decisions, security advisories and research updates from the Apich Organization. Short, dated, and permanent.",
        ),
        Route::Post(slug) => {
            let post = posts::get(slug);
            let mut m = PageMeta::new(
                post.map(|p| p.title.to_string()).unwrap_or_else(|| "Blog".into()),
                post.map(|p| p.summary.to_string()).unwrap_or_default(),
            );
            m.og_type = "article";
            m.updated = post.map(|p| p.iso).unwrap_or("2026-11-19");
            m
        }
        Route::Sponsor => PageMeta::new(
            "Sponsorship — fund the maintenance",
            "Open Collective tiers from $5, per-project budgets and a public ledger. Recurring or one-off, and here is exactly what the money pays for.",
        ),
        Route::Contact => PageMeta::new(
            "Contact — general, security, research and admin",
            "Four addresses, four teams: info@, security@, research@ and admin@apich.org, with the response target and the right channel for each.",
        ),
        Route::NotFound => {
            let mut m = PageMeta::new(
                "Page not found — Apich Organization",
                "This page moved or never existed. Project sites were consolidated under apich.org.",
            );
            m.noindex = true;
            m
        }
    }
}

/// Absolute URL for a route, used in canonicals and structured data.
pub fn canonical(route: Route) -> String {
    absolute(route.path())
}

/// Breadcrumb trail for a route, as `(name, path)` pairs.
pub fn breadcrumbs(route: Route) -> Vec<(&'static str, String)> {
    let mut trail: Vec<(&'static str, String)> = vec![("Home", "/".to_string())];
    match route {
        Route::Home => {}
        Route::Projects => trail.push(("Projects", "/projects/".to_string())),
        Route::Project(p) => {
            trail.push(("Projects", "/projects/".to_string()));
            trail.push((p.name(), route.path().to_string()));
        }
        Route::Research => trail.push(("Research", "/research/".to_string())),
        Route::Security => trail.push(("Security", "/security/".to_string())),
        Route::Blog => trail.push(("Blog", "/blog/".to_string())),
        Route::Post(slug) => {
            trail.push(("Blog", "/blog/".to_string()));
            let name = posts::get(slug).map(|p| p.title).unwrap_or("Post");
            trail.push((name, route.path().to_string()));
        }
        Route::Sponsor => trail.push(("Sponsor", "/sponsor/".to_string())),
        Route::Contact => trail.push(("Contact", "/contact/".to_string())),
        Route::NotFound => {}
    }
    trail
}

/// Rewrites the document head after a client-side navigation, so that titles,
/// descriptions, canonicals and social cards always describe the visible page.
#[cfg(feature = "hydrate")]
pub fn apply_client(document: &web_sys::Document, route: Route) {
    let meta = meta_for(route);
    document.set_title(&meta.title);

    let set = |selector: &str, value: &str| {
        if let Ok(Some(el)) = document.query_selector(selector) {
            let _ = el.set_attribute("content", value);
        }
    };
    let set_link = |selector: &str, value: &str| {
        if let Ok(Some(el)) = document.query_selector(selector) {
            let _ = el.set_attribute("href", value);
        }
    };

    set("meta[name=\"description\"]", &meta.description);
    set("meta[property=\"og:title\"]", &meta.title);
    set("meta[property=\"og:description\"]", &meta.description);
    set("meta[property=\"og:url\"]", &canonical(route));
    set("meta[property=\"og:type\"]", meta.og_type);
    set("meta[property=\"og:image\"]", &absolute(&meta.image));
    set("meta[name=\"twitter:title\"]", &meta.title);
    set("meta[name=\"twitter:description\"]", &meta.description);
    set("meta[name=\"twitter:image\"]", &absolute(&meta.image));
    set_link("link[rel=\"canonical\"]", &canonical(route));
}

/// Structured data for a route: an organization node on every page, a
/// breadcrumb trail for anything below the root, and a page-type node
/// (software, article, …) where one applies.
#[cfg(feature = "ssr")]
pub fn jsonld_for(route: Route) -> Vec<String> {
    use serde_json::{json, Value};

    let org = json!({
        "@context": "https://schema.org",
        "@type": "Organization",
        "@id": "https://apich.org/#organization",
        "name": crate::site::SITE_NAME,
        "alternateName": "Apich",
        "url": crate::site::SITE_URL,
        "logo": {
            "@type": "ImageObject",
            "url": format!("{}/assets/img/brand/apich-logo.png", crate::site::SITE_URL),
            "width": 640,
            "height": 424
        },
        "image": format!("{}/assets/img/icon-512.png", crate::site::SITE_URL),
        "slogan": crate::site::TAGLINE,
        "description": crate::site::DESCRIPTION,
        "email": crate::site::EMAIL_GENERAL,
        "sameAs": [
            crate::site::GITHUB_ORG,
            crate::site::OPEN_COLLECTIVE,
            crate::site::ZULIP,
            crate::site::DISCORD
        ],
        "contactPoint": [
            { "@type": "ContactPoint", "contactType": "general", "email": crate::site::EMAIL_GENERAL, "availableLanguage": ["en"] },
            { "@type": "ContactPoint", "contactType": "security", "email": crate::site::EMAIL_SECURITY, "availableLanguage": ["en"] },
            { "@type": "ContactPoint", "contactType": "research", "email": crate::site::EMAIL_RESEARCH, "availableLanguage": ["en"] }
        ]
    });

    let mut blocks = vec![org.to_string()];

    if route == Route::Home {
        blocks.push(
            json!({
                "@context": "https://schema.org",
                "@type": "WebSite",
                "@id": "https://apich.org/#website",
                "url": crate::site::SITE_URL,
                "name": crate::site::SITE_NAME,
                "description": crate::site::DESCRIPTION,
                "publisher": { "@id": "https://apich.org/#organization" },
                "inLanguage": "en"
            })
            .to_string(),
        );
        blocks.push(
            json!({
                "@context": "https://schema.org",
                "@type": "ItemList",
                "name": "Apich Organization projects",
                "itemListElement": projects::all().enumerate().map(|(i, c)| json!({
                    "@type": "ListItem",
                    "position": i + 1,
                    "name": c.id.name(),
                    "url": format!("{}/projects/{}/", crate::site::SITE_URL, c.id.slug())
                })).collect::<Vec<Value>>()
            })
            .to_string(),
        );
    }

    // Breadcrumbs
    let trail = breadcrumbs(route);
    if trail.len() > 1 {
        let items: Vec<Value> = trail
            .iter()
            .enumerate()
            .map(|(i, (name, path))| {
                json!({
                    "@type": "ListItem",
                    "position": i + 1,
                    "name": name,
                    "item": format!("{}{}", crate::site::SITE_URL, path)
                })
            })
            .collect();
        blocks.push(
            json!({
                "@context": "https://schema.org",
                "@type": "BreadcrumbList",
                "itemListElement": items
            })
            .to_string(),
        );
    }

    match route {
        Route::Project(p) => {
            let c = projects::get(p);
            let mut same_as = vec![c.repo.to_string()];
            if !c.crate_url.is_empty() {
                same_as.push(c.crate_url.to_string());
            }
            if !c.doi.is_empty() {
                same_as.push(c.doi.to_string());
            }
            if !c.docs.is_empty() {
                same_as.push(c.docs.to_string());
            }
            blocks.push(
                json!({
                    "@context": "https://schema.org",
                    "@type": "SoftwareSourceCode",
                    "@id": format!("{}#software", canonical(route)),
                    "name": p.name(),
                    "description": c.summary,
                    "url": canonical(route),
                    "codeRepository": c.repo,
                    "programmingLanguage": c.languages,
                    "runtimePlatform": c.platforms,
                    "license": c.license,
                    "softwareVersion": c.version,
                    "applicationCategory": c.category,
                    "author": { "@id": "https://apich.org/#organization" },
                    "publisher": { "@id": "https://apich.org/#organization" },
                    "isAccessibleForFree": true,
                    "sameAs": same_as
                })
                .to_string(),
            );
        }
        Route::Post(slug) => {
            if let Some(post) = posts::get(slug) {
                blocks.push(
                    json!({
                        "@context": "https://schema.org",
                        "@type": "BlogPosting",
                        "@id": format!("{}#post", canonical(route)),
                        "headline": post.title,
                        "description": post.summary,
                        "datePublished": post.iso,
                        "dateModified": post.iso,
                        "url": canonical(route),
                        "mainEntityOfPage": { "@type": "WebPage", "@id": canonical(route) },
                        "author": { "@type": "Organization", "name": crate::site::SITE_NAME },
                        "publisher": { "@id": "https://apich.org/#organization" },
                        "inLanguage": "en",
                        "isPartOf": { "@type": "Blog", "@id": "https://apich.org/blog/#blog" }
                    })
                    .to_string(),
                );
            }
        }
        Route::Blog => {
            blocks.push(
                json!({
                    "@context": "https://schema.org",
                    "@type": "Blog",
                    "@id": "https://apich.org/blog/#blog",
                    "name": "Apich Organization — Blog",
                    "url": "https://apich.org/blog/",
                    "description": "Announcements, release notes and engineering notes from the Apich Organization.",
                    "publisher": { "@id": "https://apich.org/#organization" },
                    "inLanguage": "en"
                })
                .to_string(),
            );
        }
        Route::Research => {
            blocks.push(
                json!({
                    "@context": "https://schema.org",
                    "@type": "ResearchOrganization",
                    "@id": "https://apich.org/research/#team",
                    "name": "Apich Organization Science & Research Team",
                    "url": "https://apich.org/research/",
                    "description": "An independent research unit working on PINN, AI4S and HPC, with extended expertise in cybersecurity and theoretical physics.",
                    "parentOrganization": { "@id": "https://apich.org/#organization" },
                    "email": crate::site::EMAIL_RESEARCH
                })
                .to_string(),
            );
        }
        Route::Sponsor => {
            blocks.push(
                json!({
                    "@context": "https://schema.org",
                    "@type": "FAQPage",
                    "mainEntity": [
                        {
                            "@type": "Question",
                            "name": "Does sponsorship change the license?",
                            "acceptedAnswer": { "@type": "Answer", "text": "No. Every project keeps its existing license and stays fully open." }
                        },
                        {
                            "@type": "Question",
                            "name": "Can we get a logo on the site?",
                            "acceptedAnswer": { "@type": "Answer", "text": "Yes, from the $100 Sponsor tier upward, subject to a quick content review." }
                        },
                        {
                            "@type": "Question",
                            "name": "Do you offer support contracts?",
                            "acceptedAnswer": { "@type": "Answer", "text": "The Enterprise Sponsor and Member tiers include priority support; consultancy is arranged separately." }
                        },
                        {
                            "@type": "Question",
                            "name": "Can we fund a specific feature?",
                            "acceptedAnswer": { "@type": "Answer", "text": "Often. Write to info@apich.org with the scope before contributing so we can confirm it is maintainable." }
                        }
                    ]
                })
                .to_string(),
            );
        }
        _ => {}
    }

    blocks
        .into_iter()
        .map(|s| s.replace("</", "<\\/"))
        .collect()
}

/// Short label for a project's status, used in badges and structured data.
pub fn project_status(p: Project) -> &'static str {
    match p {
        Project::Dtact => "Stable",
        Project::Ensia => "Stable",
        Project::Bincode => "Stable",
        Project::Rssn => "Stable",
        Project::CargoSlide => "Stable",
        Project::Virtue => "Stable",
        Project::Aevia => "Platform preview",
    }
}
