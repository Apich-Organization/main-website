//! Route table for the Apich Organization website.
//!
//! The site is a statically generated single-page application: every route is
//! rendered to HTML at build time by `src/bin/ssg.rs` and then hydrated in the
//! browser. Routes are therefore plain data — no server, no runtime router
//! dependency — which keeps the hydration tree identical on both sides.

use std::fmt;

/// Stable identifiers for the projects that live on this site.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Project {
    Dtact,
    Ensia,
    Bincode,
    Rssn,
    CargoSlide,
    Virtue,
    Aevia,
}

impl Project {
    pub const ALL: [Project; 7] = [
        Project::Dtact,
        Project::Ensia,
        Project::Bincode,
        Project::Rssn,
        Project::CargoSlide,
        Project::Virtue,
        Project::Aevia,
    ];

    /// URL slug, also used as the legacy subdomain label.
    pub const fn slug(self) -> &'static str {
        match self {
            Project::Dtact => "dtact",
            Project::Ensia => "ensia",
            Project::Bincode => "bincode",
            Project::Rssn => "rssn",
            Project::CargoSlide => "cargo-slide",
            Project::Virtue => "virtue",
            Project::Aevia => "aevia",
        }
    }

    /// Display name used in navigation and headings.
    pub const fn name(self) -> &'static str {
        match self {
            Project::Dtact => "dtact",
            Project::Ensia => "ensia",
            Project::Bincode => "bincode-next",
            Project::Rssn => "rssn",
            Project::CargoSlide => "cargo-slide",
            Project::Virtue => "virtue",
            Project::Aevia => "aevia",
        }
    }

    pub fn from_slug(slug: &str) -> Option<Project> {
        match slug {
            "dtact" => Some(Project::Dtact),
            "ensia" => Some(Project::Ensia),
            "bincode" | "bincode-next" => Some(Project::Bincode),
            "rssn" => Some(Project::Rssn),
            "cargo-slide" => Some(Project::CargoSlide),
            "virtue" | "virtue-next" => Some(Project::Virtue),
            "aevia" => Some(Project::Aevia),
            _ => None,
        }
    }

    /// Hosts that used to serve this project's site, before consolidation.
    pub const fn legacy_hosts(self) -> &'static [&'static str] {
        match self {
            Project::Dtact => &["dtact.apich.org"],
            Project::Ensia => &["ensia.apich.org"],
            Project::Bincode => &["bincode-next.apich.org", "bincode.apich.org"],
            Project::Rssn => &["rssn.apich.org"],
            Project::CargoSlide => &["cargo-slide.apich.org"],
            Project::Virtue => &["virtue.apich.org", "virtue-next.apich.org"],
            Project::Aevia => &[],
        }
    }
}

impl fmt::Display for Project {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// A page on the site.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Route {
    Home,
    Projects,
    Project(Project),
    Research,
    Security,
    Blog,
    Post(&'static str),
    Sponsor,
    Contact,
    NotFound,
}

impl Route {
    /// Canonical, absolute-path form of the route (always starts with `/`,
    /// and always ends with `/` so GitHub Pages serves the directory index).
    pub fn path(self) -> &'static str {
        match self {
            Route::Home => "/",
            Route::Projects => "/projects/",
            Route::Project(Project::Dtact) => "/projects/dtact/",
            Route::Project(Project::Ensia) => "/projects/ensia/",
            Route::Project(Project::Bincode) => "/projects/bincode/",
            Route::Project(Project::Rssn) => "/projects/rssn/",
            Route::Project(Project::CargoSlide) => "/projects/cargo-slide/",
            Route::Project(Project::Virtue) => "/projects/virtue/",
            Route::Project(Project::Aevia) => "/projects/aevia/",
            Route::Research => "/research/",
            Route::Security => "/security/",
            Route::Blog => "/blog/",
            Route::Post(slug) => match slug {
                "sponsorships-moving-to-open-collective" => {
                    "/blog/sponsorships-moving-to-open-collective/"
                }
                "hello-apich" => "/blog/hello-apich/",
                _ => "/blog/",
            },
            Route::Sponsor => "/sponsor/",
            Route::Contact => "/contact/",
            Route::NotFound => "/404/",
        }
    }

    /// Resolves a request path (with or without trailing slash, with or
    /// without a `#fragment`/`?query`) to a route.
    pub fn from_path(path: &str) -> Route {
        let path = path.split(['?', '#']).next().unwrap_or("/");
        let trimmed = path.trim_matches('/');

        if trimmed.is_empty() {
            return Route::Home;
        }

        let mut parts = trimmed.split('/');
        let head = parts.next().unwrap_or("");
        let second = parts.next();

        match (head, second) {
            ("projects", None) => Route::Projects,
            ("projects", Some(slug)) => Project::from_slug(slug)
                .map(Route::Project)
                .unwrap_or(Route::NotFound),
            ("research", _) => Route::Research,
            ("security", _) => Route::Security,
            ("sponsor", _) => Route::Sponsor,
            ("contact", _) => Route::Contact,
            ("blog", None) => Route::Blog,
            ("blog", Some(slug)) => match slug {
                "sponsorships-moving-to-open-collective" => {
                    Route::Post("sponsorships-moving-to-open-collective")
                }
                "hello-apich" => Route::Post("hello-apich"),
                _ => Route::NotFound,
            },
            ("aevia", _) => Route::Project(Project::Aevia),
            ("virtue", _) | ("virtue-next", _) => Route::Project(Project::Virtue),
            ("dtact", _) => Route::Project(Project::Dtact),
            ("ensia", _) => Route::Project(Project::Ensia),
            ("bincode", _) | ("bincode-next", _) => Route::Project(Project::Bincode),
            ("rssn", _) => Route::Project(Project::Rssn),
            ("cargo-slide", _) => Route::Project(Project::CargoSlide),
            _ => Route::NotFound,
        }
    }

    /// Whether this route should be linked in the primary navigation.
    pub fn nav_section(self) -> &'static str {
        match self {
            Route::Home => "home",
            Route::Projects | Route::Project(_) => "projects",
            Route::Research => "research",
            Route::Security => "security",
            Route::Blog | Route::Post(_) => "blog",
            Route::Sponsor => "sponsor",
            Route::Contact => "contact",
            Route::NotFound => "",
        }
    }
}

/// Every route that gets a generated HTML file, in sitemap order.
pub fn all_routes() -> Vec<Route> {
    let mut routes = vec![
        Route::Home,
        Route::Projects,
        Route::Research,
        Route::Security,
        Route::Blog,
        Route::Post("hello-apich"),
        Route::Post("sponsorships-moving-to-open-collective"),
        Route::Sponsor,
        Route::Contact,
    ];
    routes.extend(Project::ALL.into_iter().map(Route::Project));
    routes
}
