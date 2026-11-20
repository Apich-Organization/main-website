use leptos::prelude::*;

use crate::components::ui::{Icon, SectionHead, Stat};
use crate::site::EMAIL_RESEARCH;

struct Paper {
    title: &'static str,
    doi: &'static str,
    status: &'static str,
    tone: &'static str,
    venue: &'static str,
}

const PAPERS: &[Paper] = &[
    Paper {
        title: "3D Morphogenesis via Symmetry-Protected Langevin Dynamics: A Renormalization Group Perspective on Generative Geometry in LLM",
        doi: "https://doi.org/10.17605/OSF.IO/7RWBG",
        status: "In progress",
        tone: "badge-violet",
        venue: "OSF",
    },
    Paper {
        title: "One Tick, One Link: A Non-Autoregressive Challenge–Response Memory in Which Depth Is Bought with the World's Silence",
        doi: "https://doi.org/10.5281/zenodo.22307390",
        status: "In progress",
        tone: "badge-violet",
        venue: "Zenodo",
    },
    Paper {
        title: "Scheduling with Bounded Information: Correctness, Progress, and Near-Optimal Performance in DTA",
        doi: "https://doi.org/10.5281/zenodo.20998767",
        status: "In review",
        tone: "badge-accent",
        venue: "Zenodo",
    },
    Paper {
        title: "Ensia (OLLVM-Next): A Chaos-Based High-Entropy Obfuscation Framework for LLVM IR with Dual-Use Security Protocols",
        doi: "https://doi.org/10.5281/zenodo.20149843",
        status: "In review",
        tone: "badge-accent",
        venue: "Zenodo",
    },
];

#[component]
pub fn ResearchPage() -> impl IntoView {
    let papers = PAPERS
        .iter()
        .map(|p| {
            view! {
                <div class="pub">
                    <div>
                        <a class="pub-title" href=p.doi target="_blank" rel="noopener noreferrer">
                            {p.title}
                        </a>
                        <div class="pub-meta">
                            <span>{p.venue}</span>
                            <span>"DOI"</span>
                        </div>
                    </div>
                    <div class="pub-status">
                        <span class=format!("badge {}", p.tone)>{p.status}</span>
                    </div>
                </div>
            }
        })
        .collect_view();

    view! {
        <section class="pt-nav">
            <div class="page-wrap">
                <div class="page-head">
                    <p class="eyebrow">"Science & Research Team"</p>
                    <h1>"Research that ships"</h1>
                    <p class="lede">
                        "An independent research unit that grew out of the organization's core
                         consulting branch. We work on physics-informed neural networks, AI for
                         science, and high-performance computing — with extended expertise in
                         cybersecurity and theoretical physics."
                    </p>
                    <div class="row" style="margin-top:1.5rem">
                        <a class="btn btn-primary" href=format!("mailto:{EMAIL_RESEARCH}")>
                            <Icon name="mail" />
                            "research@apich.org"
                        </a>
                        <a class="btn btn-ghost" href="/projects/">
                            "Software we maintain"
                            <Icon name="arrow-right" class="icon-sm" />
                        </a>
                    </div>
                </div>

                <div class="stat-row">
                    <Stat value="2" label="Preprints in progress" note="OSF · Zenodo" />
                    <Stat value="2" label="Manuscripts in review" note="peer review" />
                    <Stat value="3" label="Core fields" note="PINN · AI4S · HPC" />
                    <Stat value="1" label="Security baseline" note="required for members" />
                </div>
            </div>
        </section>

        <section class="section">
            <div class="page-wrap">
                <SectionHead
                    eyebrow="Focus"
                    title="Three fields, one method"
                    lead="We treat numerical method, systems performance, and security as one problem: a model that cannot be reproduced on real hardware, or cannot be trusted with real data, is not a result."
                />
                <div class="grid grid-3">
                    <div class="glass card card-lift">
                        <span class="pillar-icon"><Icon name="layers" class="icon-lg" /></span>
                        <h3 style="margin-top:.6rem">"Physics-informed neural networks"</h3>
                        <p style="margin-top:.4rem">
                            "Hybrid solvers that constrain learned models with the governing
                             equations, evaluated against classical integrators rather than
                             against themselves."
                        </p>
                    </div>
                    <div class="glass card card-lift">
                        <span class="pillar-icon"><Icon name="cpu" class="icon-lg" /></span>
                        <h3 style="margin-top:.6rem">"AI for science"</h3>
                        <p style="margin-top:.4rem">
                            "Representation learning for scientific data — symmetry, scale
                             separation, and the renormalization group as an architectural prior."
                        </p>
                    </div>
                    <div class="glass card card-lift">
                        <span class="pillar-icon"><Icon name="chart" class="icon-lg" /></span>
                        <h3 style="margin-top:.6rem">"High-performance computing"</h3>
                        <p style="margin-top:.4rem">
                            "Scheduling, memory topology, and kernel design — the work that turns a
                             method into something that runs at scale on real machines."
                        </p>
                    </div>
                </div>
            </div>
        </section>

        <section class="section">
            <div class="page-wrap">
                <SectionHead
                    eyebrow="Publications"
                    title="Preprints and manuscripts"
                    lead="Every entry links to its permanent identifier. Preprints are citable before peer review; we do not withdraw them after publication."
                />
                <div class="pub-list">{papers}</div>
            </div>
        </section>

        <section class="section">
            <div class="page-wrap split">
                <div class="stack">
                    <p class="eyebrow">"How we work"</p>
                    <h2 class="section-title">"Autonomous, not isolated"</h2>
                    <p class="section-lead">
                        "The team is autonomous, but it works in close synergy with the Open
                         Source Development Team. We contribute architectural concepts and use the
                         projects maintained by the community; code that starts as a research
                         prototype often matures into software the maintenance team stewards
                         long-term."
                    </p>
                    <div class="callout" style="margin-top:.6rem">
                        <Icon name="shield" />
                        <div>
                            <strong>"Compliance."</strong>
                            " The team adheres to the Apich Organization Security Team standards and
                             to international academic and dual-use research frameworks."
                        </div>
                    </div>
                </div>
                <div class="glass card">
                    <p class="aside-head">"Research → software"</p>
                    <div class="sec-list">
                        <div class="sec-item">
                            <span class="sec-num">"01"</span>
                            <div><strong>"Concept."</strong>" A method is formalised and tested against a reference implementation."</div>
                        </div>
                        <div class="sec-item">
                            <span class="sec-num">"02"</span>
                            <div><strong>"Prototype."</strong>" Code is written in-tree, benchmarked, and reviewed for dual-use risk."</div>
                        </div>
                        <div class="sec-item">
                            <span class="sec-num">"03"</span>
                            <div><strong>"Handover."</strong>" The maintenance team takes stewardship: docs, releases, long-term support."</div>
                        </div>
                    </div>
                </div>
            </div>
        </section>

        <section class="section">
            <div class="page-wrap">
                <div class="cta-band">
                    <div>
                        <p class="eyebrow">"Collaboration"</p>
                        <h2>"Work with the team"</h2>
                        <p>
                            "We collaborate with academic groups and with organizations that need
                             the methods applied to a real problem. Write to us with the question,
                             the data shape, and the constraint that makes it hard."
                        </p>
                    </div>
                    <div class="cta-actions">
                        <a class="btn btn-primary" href=format!("mailto:{EMAIL_RESEARCH}")>
                            "Start a conversation"
                            <Icon name="arrow-right" class="icon-sm" />
                        </a>
                    </div>
                </div>
            </div>
        </section>
    }
}
