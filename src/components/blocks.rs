//! Reusable blocks that several pages share: project cards, code blocks,
//! section scaffolding.

use leptos::prelude::*;

use crate::components::ui::{Badge, Icon};
use crate::content::projects;
use crate::route::Project;

/// Card used on the home page and the project index.
#[component]
pub fn ProjectCard(project: Project) -> impl IntoView {
    let c = projects::get(project);
    let href = format!("/projects/{}/", project.slug());
    let accent = format!("accent-{}", project.slug());
    let initial = project.name().chars().next().unwrap_or('A').to_uppercase().to_string();

    view! {
        <a class=format!("glass card-lift spot proj-card {accent}") href=href>
            <div class="proj-top">
                <span class="proj-mark" aria-hidden="true">{initial}</span>
                <Badge text=c.category.to_string() tone="neutral" />
            </div>
            <h3 class="proj-name">{project.name()}</h3>
            <p class="proj-tagline">{c.tagline}</p>
            <div class="proj-foot">
                <span>{format!("{} · {}", projects::version_label(project), c.license)}</span>
                <span class="arrow" aria-hidden="true"><Icon name="arrow-right" class="icon-sm" /></span>
            </div>
        </a>
    }
}

/// Final tile in the project grids: a dashed card that leads to the full
/// repository list. It keeps the grid balanced with an odd number of projects.
#[component]
pub fn AllReposCard() -> impl IntoView {
    view! {
        <a class="proj-card proj-card--more" href=crate::site::GITHUB_ORG target="_blank" rel="noopener noreferrer">
            <div class="proj-top">
                <span class="proj-mark proj-mark--ghost" aria-hidden="true">
                    <Icon name="github" class="icon-sm" />
                </span>
            </div>
            <h3 class="proj-name">"Everything else"</h3>
            <p class="proj-tagline">
                "Experiments in progress, archived prototypes, editor integrations, and the
                 smaller crates that support the projects above."
            </p>
            <div class="proj-foot">
                <span>"All repositories"</span>
                <span class="arrow" aria-hidden="true"><Icon name="arrow-up-right" class="icon-sm" /></span>
            </div>
        </a>
    }
}

/// Dark code panel with a copy button. `html` is pre-highlighted markup; `plain`
/// is what the copy button puts on the clipboard.
#[component]
pub fn CodeBlock(
    #[prop(into)] title: String,
    #[prop(into)] html: String,
    #[prop(into)] plain: String,
) -> impl IntoView {
    view! {
        <div class="code-block">
            <div class="code-head">
                <span class="code-dots" aria-hidden="true"><i></i><i></i><i></i></span>
                <span>{title}</span>
                <button class="copy-btn" type="button" data-copy=plain title="Copy to clipboard">
                    <Icon name="copy" class="icon-xs" />
                    <span aria-live="polite">"Copy"</span>
                </button>
            </div>
            <pre><code inner_html=html></code></pre>
        </div>
    }
}

/// A titled block used inside project pages.
#[component]
pub fn Block(
    #[prop(into)] id: String,
    #[prop(into)] title: String,
    #[prop(optional, into)] note: String,
    children: Children,
) -> impl IntoView {
    view! {
        <section class="block" id=id>
            <div class="block-head">
                <h2 class="block-title">{title}</h2>
                {(!note.is_empty()).then(|| view! { <span class="block-note">{note}</span> })}
            </div>
            {children()}
        </section>
    }
}
