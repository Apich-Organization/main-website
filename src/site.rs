//! Organization-level constants: identity, contact addresses, and outbound
//! links. Everything the site claims about the organization lives here so that
//! pages, JSON-LD, and the machine-readable exports never drift apart.

/// Canonical origin. All absolute URLs are derived from this.
pub const SITE_URL: &str = "https://apich.org";
pub const SITE_NAME: &str = "Apich Organization";
pub const SITE_SHORT: &str = "Apich";
pub const TAGLINE: &str = "Open source for scientific computing and a more resilient cyberspace";
pub const DESCRIPTION: &str = "Apich Organization is an open-source initiative advancing the Rust ecosystem, with a primary focus on scientific computing and cultivating a more secure, resilient cyberspace.";

pub const EMAIL_GENERAL: &str = "info@apich.org";
pub const EMAIL_SECURITY: &str = "security@apich.org";
pub const EMAIL_RESEARCH: &str = "research@apich.org";
pub const EMAIL_ADMIN: &str = "admin@apich.org";

pub const GITHUB_ORG: &str = "https://github.com/Apich-Organization";
pub const OPEN_COLLECTIVE: &str = "https://opencollective.com/apich-organization";
pub const OPEN_COLLECTIVE_PROJECTS: &str = "https://opencollective.com/apich-organization/projects";
pub const DISCORD: &str = "https://discord.gg/D5e2czMTT9";
pub const ZULIP: &str = "https://apich.zulipchat.com/";
pub const STATUS_PAGE: &str = "https://status.apich.org/";
pub const BLOG_LEGACY: &str = "https://blog.apich.org/";

/// Open Collective tier shared by every project collective.
pub struct Tier {
    pub name: &'static str,
    pub price: &'static str,
    pub blurb: &'static str,
}

/// The organization-wide tiers (the main collective).
pub const ORG_TIERS: &[Tier] = &[
    Tier {
        name: "Backer",
        price: "$5",
        blurb: "A Sponsor badge on your Open Collective profile.",
    },
    Tier {
        name: "Donator",
        price: "$25",
        blurb: "A mention in our release notes.",
    },
    Tier {
        name: "Sponsor",
        price: "$100",
        blurb: "Your logo or name on the project website.",
    },
    Tier {
        name: "Team Sponsor",
        price: "$500",
        blurb: "Direct collaboration and internal workspace access.",
    },
    Tier {
        name: "Enterprise Sponsor",
        price: "$1,000",
        blurb: "Long-term stability, performance, and security guarantees for production users.",
    },
    Tier {
        name: "Member",
        price: "$10,000",
        blurb: "Architectural influence and mission-critical support.",
    },
];

/// A pillar of the organization, shown on the home page.
pub struct Pillar {
    pub name: &'static str,
    pub role: &'static str,
    pub body: &'static str,
    pub href: &'static str,
}

pub const PILLARS: &[Pillar] = &[
    Pillar {
        name: "Science & Research",
        role: "Research team",
        body: "PINN, AI4S and HPC research that regularly matures into the software we ship.",
        href: "/research/",
    },
    Pillar {
        name: "Open Source Development",
        role: "Maintainers",
        body: "Long-term stewardship of runtimes, compilers, serialization and scientific tooling.",
        href: "/projects/",
    },
    Pillar {
        name: "Security",
        role: "Security team",
        body: "Vulnerability handling, hardening standards, and dual-use review for every release.",
        href: "/security/",
    },
    Pillar {
        name: "Consultancy",
        role: "Engineering",
        body: "Applied work with organizations that depend on our stack in production.",
        href: "/contact/",
    },
];
