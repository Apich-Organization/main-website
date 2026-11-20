use leptos::prelude::*;

use crate::components::blocks::{AllReposCard, ProjectCard};
use crate::components::ui::{Icon, SectionHead};
use crate::content::projects;
use crate::route::Project;
use crate::site::OPEN_COLLECTIVE_PROJECTS;

#[component]
pub fn ProjectsPage() -> impl IntoView {
    let cards = Project::ALL
        .into_iter()
        .map(|p| view! { <ProjectCard project=p /> })
        .collect_view();

    let rows = Project::ALL
        .into_iter()
        .map(|p| {
            let c = projects::get(p);
            let href = format!("/projects/{}/", p.slug());
            let href2 = href.clone();
            let accent = format!("accent-{}", p.slug());
            view! {
                <tr>
                    <td>
                        <a class=format!("related-name {accent}") href=href>{p.name()}</a>
                        <div class="muted" style="font-size:.78rem">{c.tagline}</div>
                    </td>
                    <td>{c.category}</td>
                    <td class="num">{format!("v{}", c.version)}</td>
                    <td class="num">{c.license}</td>
                    <td class="num">{c.languages.join(", ")}</td>
                    <td>
                        <a href=href2 class="tier-link">
                            "Details"
                            <Icon name="arrow-right" class="icon-xs" />
                        </a>
                    </td>
                </tr>
            }
        })
        .collect_view();

    view! {
        <section class="pt-nav">
            <div class="page-wrap">
                <div class="page-head">
                    <p class="eyebrow">"Projects"</p>
                    <h1>"Projects we maintain"</h1>
                    <p class="lede">
                        "These are the projects with a release, a maintainer, and a place in the
                         security baseline. Each has its own license, cadence, and — where it makes
                         sense — its own Open Collective budget. Experimental work stays out of this
                         list until it is ready to be supported, so the count is a floor, not a
                         ceiling."
                    </p>
                </div>
            </div>
        </section>

        <section class="section section--tight">
            <div class="page-wrap">
                <h2 class="sr-only">"Maintained projects"</h2>
                <div class="proj-grid">{cards}<AllReposCard /></div>
            </div>
        </section>

        <section class="section">
            <div class="page-wrap">
                <SectionHead
                    eyebrow="At a glance"
                    title="Everything in one table"
                    lead="Versions reflect the latest published release. Licenses are those of the source repository."
                />
                <div class="table-wrap" tabindex="0" role="region" aria-label="Project comparison">
                    <table class="data">
                        <caption>
                            "Maintained projects, sorted by stack position: runtime, compiler tooling, serialization, numerics, tooling, libraries, language."
                        </caption>
                        <thead>
                            <tr>
                                <th scope="col">"Project"</th>
                                <th scope="col">"Category"</th>
                                <th scope="col">"Version"</th>
                                <th scope="col">"License"</th>
                                <th scope="col">"Written in"</th>
                                <th scope="col">""</th>
                            </tr>
                        </thead>
                        <tbody>{rows}</tbody>
                    </table>
                </div>
                <p class="block-note" style="margin-top:1rem">
                    "Rust version support: each project caps its MSRV at either the current stable
                     release or stable − 3, depending on its policy. The cap is the most a release
                     may ever require — the actual minimum is often lower, and raising it up to the
                     cap is not treated as a breaking change."
                </p>
            </div>
        </section>

        <section class="section">
            <div class="page-wrap">
                <SectionHead
                    eyebrow="How we maintain"
                    title="The same bar for every repository"
                />
                <div class="grid grid-3">
                    <div class="glass card">
                        <span class="pillar-icon"><Icon name="book" class="icon-lg" /></span>
                        <h3 style="margin-top:.6rem">"Documented, not just released"</h3>
                        <p style="margin-top:.4rem">
                            "Public API documentation on docs.rs, a changelog per release, and a
                             README that states what the project does not do."
                        </p>
                    </div>
                    <div class="glass card">
                        <span class="pillar-icon"><Icon name="shield" class="icon-lg" /></span>
                        <h3 style="margin-top:.6rem">"Security review before release"</h3>
                        <p style="margin-top:.4rem">
                            "Dependency review, fuzzing where it applies, and a coordinated
                             disclosure path with a seven-day response commitment."
                        </p>
                    </div>
                    <div class="glass card">
                        <span class="pillar-icon"><Icon name="heart" class="icon-lg" /></span>
                        <h3 style="margin-top:.6rem">"Funded in the open"</h3>
                        <p style="margin-top:.4rem">
                            "Most projects have a dedicated Open Collective budget. The rest are
                             funded through the organization account, from recurring and one-off
                             contributions alike."
                        </p>
                    </div>
                </div>

                <div class="row" style="margin-top:2rem">
                    <a class="btn btn-ghost" href=OPEN_COLLECTIVE_PROJECTS target="_blank" rel="noopener noreferrer">
                        "Project collectives"
                        <Icon name="arrow-up-right" class="icon-sm" />
                    </a>
                    <a class="btn btn-ghost" href="/contact/">
                        "Propose a project"
                        <Icon name="arrow-right" class="icon-sm" />
                    </a>
                </div>
            </div>
        </section>
    }
}
