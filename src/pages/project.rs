use leptos::prelude::*;

use crate::components::blocks::{Block, CodeBlock};
use crate::components::ui::{Badge, Icon, Stat};
use crate::content::details;
use crate::content::projects;
use crate::content::versions;
use crate::route::Project;

#[component]
pub fn ProjectPage(project: Project) -> impl IntoView {
    let c = projects::get(project);
    let d = details::get(project);
    let accent = format!("accent-{}", project.slug());
    let initial = project
        .name()
        .chars()
        .next()
        .unwrap_or('A')
        .to_uppercase()
        .to_string();

    let stats = c
        .stats
        .iter()
        .map(|(value, label, note)| {
            view! {
                <Stat
                    value=versions::fill(value, c.version)
                    label=label.to_string()
                    note=note.to_string()
                />
            }
        })
        .collect_view();

    let features = c
        .features
        .iter()
        .map(|f| {
            view! {
                <div class="feature">
                    <span class="feature-icon"><Icon name=f.icon class="icon-sm" /></span>
                    <h3>{f.title}</h3>
                    <p>{f.body}</p>
                </div>
            }
        })
        .collect_view();

    let mechanisms = d
        .mechanisms
        .iter()
        .enumerate()
        .map(|(i, (title, body))| {
            view! {
                <div class="mech">
                    <span class="mech-index">{format!("{:02}", i + 1)}</span>
                    <div>
                        <h3>{*title}</h3>
                        <p>{*body}</p>
                    </div>
                </div>
            }
        })
        .collect_view();

    let paragraphs = d
        .intro
        .iter()
        .map(|p| view! { <p>{*p}</p> })
        .collect_view();

    let spec_rows = {
        let mut rows: Vec<AnyView> = Vec::new();
        rows.push(
            view! {
                <div class="spec-row"><dt>"Version"</dt><dd class="num">{c.version}</dd></div>
            }
            .into_any(),
        );
        rows.push(
            view! {
                <div class="spec-row"><dt>"License"</dt><dd>{c.license}</dd></div>
            }
            .into_any(),
        );
        rows.push(
            view! {
                <div class="spec-row"><dt>"Language"</dt><dd>{c.languages.join(", ")}</dd></div>
            }
            .into_any(),
        );
        rows.push(
            view! {
                <div class="spec-row"><dt>"Platforms"</dt><dd>{c.platforms.join(", ")}</dd></div>
            }
            .into_any(),
        );
        if !c.crate_url.is_empty() {
            rows.push(
                view! {
                    <div class="spec-row">
                        <dt>"crates.io"</dt>
                        <dd><a href=c.crate_url target="_blank" rel="noopener noreferrer">{crate_name(project)}</a></dd>
                    </div>
                }
                .into_any(),
            );
        }
        rows.push(
            view! {
                <div class="spec-row">
                    <dt>"Source"</dt>
                    <dd><a href=c.repo target="_blank" rel="noopener noreferrer">"GitHub"</a></dd>
                </div>
            }
            .into_any(),
        );
        if !c.docs.is_empty() {
            rows.push(
                view! {
                    <div class="spec-row">
                        <dt>"Docs"</dt>
                        <dd><a href=c.docs target="_blank" rel="noopener noreferrer">{docs_label(c.docs)}</a></dd>
                    </div>
                }
                .into_any(),
            );
        }
        if !c.doi.is_empty() {
            rows.push(
                view! {
                    <div class="spec-row">
                        <dt>"DOI"</dt>
                        <dd><a href=c.doi target="_blank" rel="noopener noreferrer">{doi_label(c.doi)}</a></dd>
                    </div>
                }
                .into_any(),
            );
        }
        rows
    };

    let links = d
        .links
        .iter()
        .map(|(label, sub, href)| {
            view! {
                <a class="link-card" href=*href target="_blank" rel="noopener noreferrer">
                    <Icon name="arrow-up-right" class="icon-sm" />
                    <span class="label">{*label}</span>
                    <span class="sub">{*sub}</span>
                </a>
            }
        })
        .collect_view();

    let related = Project::ALL
        .into_iter()
        .filter(|p| *p != project)
        .take(3)
        .map(|p| {
            let pc = projects::get(p);
            let href = format!("/projects/{}/", p.slug());
            let accent = format!("accent-{}", p.slug());
            view! {
                <a class=format!("related-card {accent}") href=href>
                    <span>
                        <span class="related-name">{p.name()}</span>
                        <span class="related-cat" style="display:block;margin-top:.15rem">{pc.category}</span>
                    </span>
                    <Icon name="arrow-right" class="icon-sm" />
                </a>
            }
        })
        .collect_view();

    let figures = d
        .figures
        .iter()
        .map(|f| {
            view! {
                <figure class="figure">
                    <img src=f.src alt=f.alt width="1484" height="613" loading="eager" decoding="async" />
                    <figcaption>{f.caption}</figcaption>
                </figure>
            }
        })
        .collect_view();

    let bench = d.table.map(|b| {
        let rows = b
            .rows
            .iter()
            .map(|(what, value, note)| {
                view! {
                    <tr>
                        <td>{*what}</td>
                        <td class="num">{*value}</td>
                        <td>{*note}</td>
                    </tr>
                }
            })
            .collect_view();
        view! {
            <div class="table-wrap" tabindex="0" role="region" aria-label="Project data">
                <table class="data">
                    <caption>{b.caption}</caption>
                    <thead>
                        <tr>
                            <th scope="col">{b.headers[0]}</th>
                            <th scope="col">{b.headers[1]}</th>
                            <th scope="col">{b.headers[2]}</th>
                        </tr>
                    </thead>
                    <tbody>{rows}</tbody>
                </table>
            </div>
        }
    });

    view! {
        <div class=accent.clone()>
            <section class="proj-hero">
                <div class="page-wrap">
                    <nav class="breadcrumb" aria-label="Breadcrumb">
                        <a href="/">"Home"</a>
                        <span class="sep">"/"</span>
                        <a href="/projects/">"Projects"</a>
                        <span class="sep">"/"</span>
                        <span aria-current="page">{project.name()}</span>
                    </nav>

                    <div class="proj-hero-grid">
                        <div>
                            <div class="proj-eyebrow">
                                <span class="proj-mark" aria-hidden="true">{initial}</span>
                                <Badge text=c.category.to_string() tone="neutral" />
                                <Badge text=projects::version_label(project) tone="brand" />
                            </div>
                            <h1 class="proj-title">{project.name()}</h1>
                            <p class="proj-tag">{c.tagline}</p>
                            <div class="proj-actions">
                                <a class="btn btn-primary" href=c.repo target="_blank" rel="noopener noreferrer">
                                    <Icon name="github" />
                                    "View source"
                                </a>
                                {(!c.docs.is_empty()).then(|| view! {
                                    <a class="btn btn-ghost" href=c.docs target="_blank" rel="noopener noreferrer">
                                        <Icon name="book" class="icon-sm" />
                                        "Documentation"
                                    </a>
                                })}
                                <a class="btn btn-ghost" href="/sponsor/">
                                    <Icon name="heart" class="icon-sm" />
                                    "Sponsor"
                                </a>
                            </div>
                            <div class="proj-meta">
                                <span class="chip"><Icon name="git" class="icon-xs" />{c.license}</span>
                                <span class="chip"><Icon name="terminal" class="icon-xs" />{c.languages.join(" · ")}</span>
                                {(!d.platform_note.is_empty()).then(|| view! {
                                    <span class="chip"><Icon name="cpu" class="icon-xs" />{d.platform_note}</span>
                                })}
                            </div>
                        </div>

                        <div class="proj-art" aria-hidden="true">
                            <span class="proj-art-ring"></span>
                            <span class="proj-art-ring ring-b"></span>
                            <span class="proj-art-mark">{project.name()}</span>
                            <span class="proj-art-note">{d.art_note}</span>
                        </div>
                    </div>
                </div>
            </section>

            {(!c.stats.is_empty()).then(|| view! {
                <section class="section section--tight">
                    <div class="page-wrap">
                        <div class="stat-row">{stats}</div>
                    </div>
                </section>
            })}

            <section class="section section--tight">
                <div class="page-wrap proj-body">
                    <div class="proj-main">
                        <section class="block" id="overview">
                            <div class="block-head"><h2 class="block-title">"Overview"</h2></div>
                            <div class="prose">{paragraphs}</div>
                        </section>

                        {(!c.features.is_empty()).then(|| view! {
                            <Block id="capabilities" title="Capabilities" note="what it does">
                                <div class="feature-grid">{features}</div>
                            </Block>
                        })}

                        {(!d.mechanisms.is_empty()).then(|| view! {
                            <Block id="design" title="How it works" note="design notes">
                                <div class="mech-list">{mechanisms}</div>
                            </Block>
                        })}

                        {d.table.map(|t| view! {
                            <Block id="data" title=t.title note=t.note>
                                {bench}
                                {figures}
                            </Block>
                        })}

                        <Block id="install" title="Install" note="cargo">
                            <CodeBlock
                                title=d.install_title
                                html=versions::fill(d.install_html, c.version)
                                plain=versions::fill(d.install_plain, c.version)
                            />
                            <p class="contact-note">{versions::fill(d.install_note, c.version)}</p>
                        </Block>

                        {(!d.links.is_empty()).then(|| view! {
                            <Block id="links" title="Elsewhere" note="canonical links">
                                <div class="link-grid">{links}</div>
                            </Block>
                        })}

                        <Block id="related" title="Other projects" note="same standard">
                            <div class="related">{related}</div>
                        </Block>
                    </div>

                    <aside class="proj-aside">
                        <nav class="glass spec-card toc-card" aria-label="On this page">
                            <p class="aside-head">"On this page"</p>
                            <ul class="toc">
                                <li><a href="#overview">"Overview"</a></li>
                                {(!c.features.is_empty()).then(|| view! {
                                    <li><a href="#capabilities">"Capabilities"</a></li>
                                })}
                                {(!d.mechanisms.is_empty()).then(|| view! {
                                    <li><a href="#design">"How it works"</a></li>
                                })}
                                {d.table.map(|t| view! {
                                    <li><a href="#data">{t.title}</a></li>
                                })}
                                <li><a href="#install">"Install"</a></li>
                                {(!d.links.is_empty()).then(|| view! {
                                    <li><a href="#links">"Elsewhere"</a></li>
                                })}
                                <li><a href="#related">"Other projects"</a></li>
                            </ul>
                        </nav>
                        <div class="glass spec-card">
                            <p class="aside-head">"At a glance"</p>
                            <dl class="spec-list">{spec_rows}</dl>
                        </div>
                        <div class="support-card">
                            <p class="aside-head">"Support"</p>
                            <div class="support-price">
                                {c.funding_from}
                                <span>" / month"</span>
                            </div>
                            <p style="margin-top:.5rem">
                                {if c.funding.ends_with("apich-organization") {
                                    "This project is funded through the organization's collective, alongside infrastructure and security work."
                                } else {
                                    "Fund this project directly through its own Open Collective budget."
                                }}
                            </p>
                            <a class="btn btn-primary btn-sm" href=c.funding target="_blank" rel="noopener noreferrer">
                                <Icon name="heart" class="icon-xs" />
                                "Contribute"
                            </a>
                        </div>
                    </aside>
                </div>
            </section>
        </div>
    }
}

fn crate_name(p: Project) -> &'static str {
    match p {
        Project::Dtact => "dtact",
        Project::Ensia => "ensia",
        Project::Bincode => "bincode-next",
        Project::Rssn => "rssn",
        Project::CargoSlide => "cargo-slide",
        Project::Virtue => "virtue",
        Project::Aevia => "aevia",
    }
}

/// Registry name for a documentation URL.
fn docs_label(url: &str) -> &'static str {
    if url.contains("docs.rs") {
        "docs.rs"
    } else if url.contains("github.com") {
        "README"
    } else {
        "Website"
    }
}

/// Registry name for a DOI, derived from the registrant prefix.
fn doi_label(url: &str) -> &'static str {
    if url.contains("10.6084") {
        "Figshare"
    } else if url.contains("10.5281") {
        "Zenodo"
    } else {
        "DOI"
    }
}
