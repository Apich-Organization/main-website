//! Blog posts. Bodies are stored as HTML fragments so the server render and the
//! hydrated client render are byte-identical and no markdown parser ships to
//! the browser.

#[derive(Clone, Copy)]
pub struct Post {
    pub slug: &'static str,
    pub title: &'static str,
    pub summary: &'static str,
    /// Human-readable date, e.g. "20 June 2026".
    pub date: &'static str,
    /// RFC 822 date for the RSS feed.
    pub rfc822: &'static str,
    /// ISO date for structured data and the sitemap.
    pub iso: &'static str,
    pub reading_time: &'static str,
    pub tag: &'static str,
    pub body: &'static str,
}

pub static ALL: &[Post] = &[SPONSORSHIPS, HELLO];

static SPONSORSHIPS: Post = Post {
    slug: "sponsorships-moving-to-open-collective",
    title: "Sponsorships are moving to Open Collective",
    summary: "Financial hosting for the organization and for six of our projects now runs through Open Collective, with transparent budgets and per-project collectives.",
    date: "20 June 2026",
    rfc822: "Sat, 20 Jun 2026 09:00:00 +0000",
    iso: "2026-06-20",
    reading_time: "3 min read",
    tag: "Announcement",
    body: r#"
<p>To better support our mission of fostering a transparent and collaborative open-source environment, the Apich Organization has moved its financial hosting to <a href="https://opencollective.com/apich-organization" target="_blank" rel="noopener noreferrer">Open Collective</a>, under the fiscal host <a href="https://opencollective.com/opensource" target="_blank" rel="noopener noreferrer">Open Source Collective</a>.</p>

<h2>What changes</h2>
<p>Every contribution now lands in a public budget. You can see what comes in, what is spent, and which project it supports. Nothing about the projects themselves changes: the licenses stay the same, the repositories stay where they are, and no feature is moved behind a paywall.</p>

<h2>Where to contribute</h2>
<p>The organization collective funds general maintenance, infrastructure, security work and the projects that do not have a dedicated page:</p>
<ul>
  <li><a href="https://opencollective.com/apich-organization" target="_blank" rel="noopener noreferrer">apich-organization</a> — general contributions, infrastructure, and projects without a dedicated collective.</li>
  <li><a href="https://opencollective.com/apich-organization/projects/aevia" target="_blank" rel="noopener noreferrer">aevia</a></li>
  <li><a href="https://opencollective.com/apich-organization/projects/rssn" target="_blank" rel="noopener noreferrer">rssn</a></li>
  <li><a href="https://opencollective.com/apich-organization/projects/cargo-slide" target="_blank" rel="noopener noreferrer">cargo-slide</a></li>
  <li><a href="https://opencollective.com/apich-organization/projects/ensia" target="_blank" rel="noopener noreferrer">ensia</a></li>
  <li><a href="https://opencollective.com/apich-organization/projects/dtact" target="_blank" rel="noopener noreferrer">dtact</a></li>
  <li><a href="https://opencollective.com/apich-organization/projects/bincode-next" target="_blank" rel="noopener noreferrer">bincode-next</a></li>
</ul>

<h2>Tiers</h2>
<p>Project collectives start at $5 per month (Backer) and scale to Sponsor ($50), Enterprise Sponsor ($500) and Project Member ($5,000). The organization collective adds a $25 Donator tier and a $100 Sponsor tier that places your logo on this site.</p>

<h2>What we spend it on</h2>
<p>Compute for continuous integration and benchmarks, code-signing and release infrastructure, security audits, and the maintainers' time. We publish updates on the collective as those costs are incurred.</p>

<p>Thank you to everyone who has contributed code, reviews, bug reports, or money. If your organization depends on our work in production, the <a href="/sponsor/">sponsorship page</a> explains which tier fits.</p>
"#,
};

static HELLO: Post = Post {
    slug: "hello-apich",
    title: "Hello, and welcome to the Apich blog",
    summary: "Why we are consolidating our web presence, what this blog will cover, and how to follow along.",
    date: "20 February 2026",
    rfc822: "Fri, 20 Feb 2026 09:00:00 +0000",
    iso: "2026-02-20",
    reading_time: "2 min read",
    tag: "Meta",
    body: r#"
<p>This is the start of the official blog of the Apich Organization. We build open-source infrastructure for scientific computing and for a more resilient cyberspace, and until now most of that work was documented in READMEs scattered across several repositories.</p>

<h2>What changes</h2>
<p>Every project site now lives here, under <code>apich.org</code>. One design, one navigation, and one place to find out what a project is, what it does, and how to use it. The old project subdomains forward to the matching pages, so existing links still land somewhere sensible.</p>
<p>Two things stay separate. <strong>aevia</strong> has its own platform and its own site at <a href="https://aevia.apich.org/" target="_blank" rel="noopener noreferrer">aevia.apich.org</a>. Our <strong>team pages</strong> for research and security now live here as well.</p>

<h2>What we will publish</h2>
<ul>
  <li>Release notes for dtact, ensia, bincode-next, rssn, cargo-slide and virtue.</li>
  <li>Engineering notes: scheduler design, obfuscation passes, numerical kernels, serialization formats.</li>
  <li>Security advisories and disclosure timelines.</li>
  <li>Research preprints from the Science &amp; Research Team.</li>
</ul>

<h2>Following along</h2>
<p>There is an <a href="/blog/feed.xml">RSS feed</a>, a <a href="https://apich.zulipchat.com/" target="_blank" rel="noopener noreferrer">Zulip</a> for discussion, and a <a href="https://discord.gg/D5e2czMTT9" target="_blank" rel="noopener noreferrer">Discord</a> for everything else. Code lives on <a href="https://github.com/Apich-Organization" target="_blank" rel="noopener noreferrer">GitHub</a>.</p>
"#,
};

pub fn get(slug: &str) -> Option<&'static Post> {
    ALL.iter().find(|p| p.slug == slug)
}
