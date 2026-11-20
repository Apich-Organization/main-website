use leptos::prelude::*;

use crate::components::blocks::{AllReposCard, CodeBlock, ProjectCard};
use crate::components::ui::{Icon, SectionHead, Stat};
use crate::route::Project;
use crate::site::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let pillars = PILLARS
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let icon = match i {
                0 => "compass",
                1 => "package",
                2 => "shield",
                _ => "users",
            };
            view! {
                <a class="glass card-lift pillar" href=p.href>
                    <span class="pillar-icon"><Icon name=icon class="icon-lg" /></span>
                    <span class="pillar-role">{p.role}</span>
                    <h3>{p.name}</h3>
                    <p>{p.body}</p>
                </a>
            }
        })
        .collect_view();

    let cards = Project::ALL
        .into_iter()
        .map(|p| view! { <ProjectCard project=p /> })
        .collect_view();

    view! {
        // ── Hero ───────────────────────────────────────────────────────────
        <section class="hero">
            <div class="page-wrap hero-grid">
                <div class="hero-copy">
                    <p class="eyebrow">"Open source · Scientific computing · Security"</p>
                    <h1>"Open infrastructure for "<em>"science"</em>" and security."</h1>
                    <p class="hero-lead">
                        "We maintain the runtimes, compilers, serialization formats and numerical engines that scientific software is built on — released in the open, and checked against a published security baseline before each release."
                    </p>
                    <div class="hero-actions">
                        <a class="btn btn-primary btn-lg" href="/projects/">
                            "Explore the projects"
                            <Icon name="arrow-right" />
                        </a>
                        <a class="btn btn-ghost btn-lg" href="/sponsor/">
                            <Icon name="heart" />
                            "Sponsor our work"
                        </a>
                    </div>
                    <div class="hero-meta">
                        <span><strong>{Project::ALL.len()}</strong>" maintained"</span>
                        <span><strong>"4"</strong>" teams"</span>
                        <span><strong>"OSI-approved licenses"</strong></span>
                        <span><strong>"Open Collective"</strong>" funded"</span>
                    </div>
                </div>

                <div class="hero-visual" aria-hidden="true">
                    <div class="orb-ring r1"></div>
                    <div class="orb-ring r2"></div>
                    <div class="orb-ring r3"></div>
                    <div class="orb-core">
                        <span class="orb-orbit"></span>
                        <span class="orb-dot"></span>
                    </div>
                    <span class="orb-node n1 accent-dtact"><i class="dot"></i>"dtact"</span>
                    <span class="orb-node n2 accent-ensia"><i class="dot"></i>"ensia"</span>
                    <span class="orb-node n3 accent-rssn"><i class="dot"></i>"rssn"</span>
                </div>
            </div>
        </section>

        // ── Pillars ────────────────────────────────────────────────────────
        <section class="section">
            <div class="page-wrap">
                <SectionHead
                    eyebrow="The organization"
                    title="Four teams, one engineering standard"
                    lead="Research proposes, engineering builds, security reviews, and consultancy keeps the work grounded in production reality."
                />
                <div class="grid grid-4">{pillars}</div>
            </div>
        </section>

        // ── Projects ───────────────────────────────────────────────────────
        <section class="section">
            <div class="page-wrap">
                <SectionHead
                    eyebrow="Projects"
                    title="Projects we maintain"
                    lead="The libraries and tools we ship today, from the coroutine runtime upwards — each one maintained, documented, and independently funded. Work in the lab is not listed until it earns a release."
                />
                <div class="proj-grid">{cards}<AllReposCard /></div>
                <div class="row" style="margin-top:2rem">
                    <a class="btn btn-ghost" href="/projects/">
                        "Compare all projects"
                        <Icon name="arrow-right" />
                    </a>
                    <span class="block-note">"API documentation and a changelog, where the ecosystem provides them."</span>
                </div>
            </div>
        </section>

        // ── Get started ────────────────────────────────────────────────────
        <section class="section">
            <div class="page-wrap split">
                <div class="stack">
                    <p class="eyebrow">"Get started"</p>
                    <h2 class="section-title">"Add the stack to your project"</h2>
                    <p class="section-lead">
                        "The Rust crates are published to crates.io and build with a stable toolchain. No build scripts that phone home, no vendored blobs, no telemetry."
                    </p>
                    <ul class="stack check-list" style="gap:.55rem;margin-top:.5rem">
                        <li class="row" style="gap:.55rem">
                            <Icon name="check" class="icon-sm" />
                            <span class="muted">"Each crate declares its own minimum Rust version in its metadata."</span>
                        </li>
                        <li class="row" style="gap:.55rem">
                            <Icon name="check" class="icon-sm" />
                            <span class="muted">"bincode-next keeps the wire format stable across the 3.x series."</span>
                        </li>
                        <li class="row" style="gap:.55rem">
                            <Icon name="check" class="icon-sm" />
                            <span class="muted">"rssn ships C and C++ headers alongside the Rust API."</span>
                        </li>
                    </ul>
                </div>
                <CodeBlock
                    title="shell — cargo"
                    plain="cargo add dtact\ncargo add bincode-next\ncargo add rssn"
                    html="<span class=\"c\"># A stackful coroutine runtime for systems work</span>\ncargo add dtact\n\n<span class=\"c\"># Binary serialization with SIMD varint scanning</span>\ncargo add bincode-next\n\n<span class=\"c\"># Computer algebra, numerical kernels, physics</span>\ncargo add rssn"
                />
            </div>
        </section>

        // ── Numbers ────────────────────────────────────────────────────────
        <section class="section section--tight">
            <div class="page-wrap">
                <SectionHead eyebrow="By the numbers" title="Small organization, measured work" />
                <div class="stat-row">
                    <Stat value=Project::ALL.len().to_string() label="Maintained projects" note="documented in the open" />
                    <Stat value="6" label="Open Collective budgets" note="public ledgers" />
                    <Stat value="79" label="Benchmark targets in ensia" note="60 defeated at max" />
                    <Stat value="0" label="Third-party requests" note="this site" />
                </div>
            </div>
        </section>

        // ── Teams ──────────────────────────────────────────────────────────
        <section class="section">
            <div class="page-wrap">
                <SectionHead
                    eyebrow="Research & security"
                    title="The two teams behind the standards"
                    lead="One produces the ideas, the other makes sure shipping them is safe."
                />
                <div class="grid grid-2">
                    <a class="glass card-lift spot card" href="/research/">
                        <span class="pillar-icon"><Icon name="compass" class="icon-lg" /></span>
                        <h3 style="margin-top:.6rem">"Science & Research Team"</h3>
                        <p style="margin-top:.5rem">
                            "An independent unit working on physics-informed neural networks, AI for science, and high-performance computing — with extended work in cybersecurity and theoretical physics. Two preprints are in progress and two are in review."
                        </p>
                        <div class="row" style="margin-top:1.1rem">
                            <span class="badge badge-brand">"PINN"</span>
                            <span class="badge badge-brand">"AI4S"</span>
                            <span class="badge badge-brand">"HPC"</span>
                        </div>
                    </a>
                    <a class="glass card-lift spot card" href="/security/">
                        <span class="pillar-icon"><Icon name="shield" class="icon-lg" /></span>
                        <h3 style="margin-top:.6rem">"Security Team"</h3>
                        <p style="margin-top:.5rem">
                            "Coordinated disclosure with a seven-day response commitment, a documented baseline for developer machines, and dual-use review for every obfuscation and compiler feature we ship."
                        </p>
                        <div class="row" style="margin-top:1.1rem">
                            <span class="badge badge-green">"7-day response"</span>
                            <span class="badge badge-green">"PGP published"</span>
                            <span class="badge badge-green">"Dual-use review"</span>
                        </div>
                    </a>
                </div>
            </div>
        </section>

        // ── Sponsor ────────────────────────────────────────────────────────
        <section class="section">
            <div class="page-wrap">
                <div class="cta-band">
                    <div>
                        <p class="eyebrow">"Sponsorship"</p>
                        <h2>"Fund the maintenance, not the marketing."</h2>
                        <p>
                            "Contributions run through Open Collective with a public budget: compute for CI and benchmarks, code-signing and release infrastructure, security audits, and maintainer time."
                        </p>
                    </div>
                    <div class="cta-actions">
                        <a class="btn btn-primary" href="/sponsor/">
                            "See the tiers"
                            <Icon name="arrow-right" />
                        </a>
                        <a class="btn btn-ghost" href=OPEN_COLLECTIVE target="_blank" rel="noopener noreferrer">
                            "Open Collective"
                            <Icon name="arrow-up-right" class="icon-sm" />
                        </a>
                    </div>
                </div>
            </div>
        </section>

        // ── Community ──────────────────────────────────────────────────────
        <section class="section section--tight">
            <div class="page-wrap">
                <SectionHead
                    eyebrow="Community"
                    title="Where the work happens"
                    lead="Issues and pull requests on GitHub, longer discussions on Zulip, and everything else on Discord."
                />
                <div class="link-grid">
                    <a class="link-card" href=GITHUB_ORG target="_blank" rel="noopener noreferrer">
                        <Icon name="github" />
                        <span class="label">"GitHub"</span>
                        <span class="sub">"code"</span>
                    </a>
                    <a class="link-card" href=ZULIP target="_blank" rel="noopener noreferrer">
                        <Icon name="chat" />
                        <span class="label">"Zulip"</span>
                        <span class="sub">"discussion"</span>
                    </a>
                    <a class="link-card" href=DISCORD target="_blank" rel="noopener noreferrer">
                        <Icon name="discord" />
                        <span class="label">"Discord"</span>
                        <span class="sub">"chat"</span>
                    </a>
                    <a class="link-card" href=STATUS_PAGE target="_blank" rel="noopener noreferrer">
                        <Icon name="zap" />
                        <span class="label">"Status"</span>
                        <span class="sub">"uptime"</span>
                    </a>
                </div>
            </div>
        </section>
    }
}
