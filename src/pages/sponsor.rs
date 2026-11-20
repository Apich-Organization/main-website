use leptos::prelude::*;

use crate::components::ui::{Icon, SectionHead, Stat};
use crate::content::projects;
use crate::route::Project;
use crate::site::*;

#[component]
pub fn SponsorPage() -> impl IntoView {
    let tiers = ORG_TIERS
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let featured = i == 2;
            view! {
                <div class="tier" class:tier--feature=featured>
                    {featured.then(|| view! { <span class="tier-flag badge badge-brand">"Most common"</span> })}
                    <span class="tier-name">{t.name}</span>
                    <div class="tier-price">
                        {t.price}
                        <small>" / month"</small>
                    </div>
                    <p class="tier-blurb">{t.blurb}</p>
                    <a class="tier-link" href=OPEN_COLLECTIVE target="_blank" rel="noopener noreferrer">
                        "Contribute"
                        <Icon name="arrow-up-right" class="icon-xs" />
                    </a>
                </div>
            }
        })
        .collect_view();

    let collectives = Project::ALL
        .into_iter()
        .filter(|p| !projects::get(*p).funding.ends_with("apich-organization"))
        .count();

    let funds = Project::ALL
        .into_iter()
        .filter(|p| !projects::get(*p).funding.ends_with("apich-organization"))
        .map(|p| {
            let c = projects::get(p);
            let accent = format!("accent-{}", p.slug());
            view! {
                <a class=format!("fund-card {accent}") href=c.funding target="_blank" rel="noopener noreferrer">
                    <span class="fund-name">{p.name()}</span>
                    <span class="fund-meta">
                        <i class="dot"></i>
                        {format!("from {} · monthly or one-off", c.funding_from)}
                    </span>
                </a>
            }
        })
        .collect_view();

    view! {
        <section class="pt-nav">
            <div class="page-wrap">
                <div class="page-head">
                    <p class="eyebrow">"Sponsorship"</p>
                    <h1>"Fund the maintenance"</h1>
                    <p class="lede">
                        "Financial hosting runs through Open Collective under the Open Source
                         Collective fiscal host, so every contribution and every expense appears
                         in a public ledger. Most projects have their own budget; the rest are
                         funded through the organization account. Recurring and one-off
                         contributions are both welcome — a one-off lands in the same budget."
                    </p>
                    <div class="row" style="margin-top:1.5rem">
                        <a class="btn btn-primary" href=OPEN_COLLECTIVE target="_blank" rel="noopener noreferrer">
                            <Icon name="heart" />
                            "Contribute on Open Collective"
                        </a>
                        <a class="btn btn-ghost" href=format!("mailto:{EMAIL_GENERAL}")>
                            "Talk to us first"
                        </a>
                    </div>
                </div>

                <div class="stat-row">
                    <Stat value=collectives.to_string() label="Project collectives" note="public budgets" />
                    <Stat value="$5" label="Lowest tier" note="monthly or one-off" />
                    <Stat value="100%" label="Expenses published" note="open ledger" />
                    <Stat value="0" label="Paywalled features" note="ever" />
                </div>
            </div>
        </section>

        <section class="section">
            <div class="page-wrap">
                <SectionHead
                    eyebrow="Organization tiers"
                    title="One account, everything general"
                    lead="Support infrastructure, security work, and the projects without a dedicated collective. Tiers are monthly and can be changed or cancelled at any time; one-off contributions go to the same place."
                />
                <div class="tier-grid">{tiers}</div>
            </div>
        </section>

        <section class="section">
            <div class="page-wrap">
                <SectionHead
                    eyebrow="Project collectives"
                    title="Fund one project directly"
                    lead="Project budgets cover that project's CI, benchmarks, and maintainer time. Each page shows the same public ledger, and takes both recurring and one-off contributions."
                />
                <div class="fund-grid">{funds}</div>
                <p class="contact-note" style="margin-top:1rem">
                    "virtue is funded through the organization account. If you need an invoice, a
                     purchase order, or a custom arrangement, write to "
                    <a href=format!("mailto:{EMAIL_GENERAL}") class="contact-mail">{EMAIL_GENERAL}</a>
                    "."
                </p>
            </div>
        </section>

        <section class="section">
            <div class="page-wrap">
                <SectionHead eyebrow="Where it goes" title="What the money pays for" />
                <div class="grid grid-4">
                    <div class="glass card">
                        <span class="pillar-icon"><Icon name="cpu" class="icon-lg" /></span>
                        <h3 style="margin-top:.6rem;font-size:1rem">"Compute"</h3>
                        <p style="margin-top:.3rem;font-size:.88rem">
                            "CI runners, fuzzing time, and the benchmark machines that produce the
                             numbers we publish."
                        </p>
                    </div>
                    <div class="glass card">
                        <span class="pillar-icon"><Icon name="lock" class="icon-lg" /></span>
                        <h3 style="margin-top:.6rem;font-size:1rem">"Release integrity"</h3>
                        <p style="margin-top:.3rem;font-size:.88rem">
                            "Code-signing keys, reproducible-build infrastructure, and artifact
                             hosting."
                        </p>
                    </div>
                    <div class="glass card">
                        <span class="pillar-icon"><Icon name="shield" class="icon-lg" /></span>
                        <h3 style="margin-top:.6rem;font-size:1rem">"Security work"</h3>
                        <p style="margin-top:.3rem;font-size:.88rem">
                            "External audits where they are warranted, and the time to run
                             coordinated disclosure properly."
                        </p>
                    </div>
                    <div class="glass card">
                        <span class="pillar-icon"><Icon name="users" class="icon-lg" /></span>
                        <h3 style="margin-top:.6rem;font-size:1rem">"Maintainer time"</h3>
                        <p style="margin-top:.3rem;font-size:.88rem">
                            "Reviews, releases, and documentation — the unglamorous work that keeps
                             a project usable."
                        </p>
                    </div>
                </div>
            </div>
        </section>

        <section class="section">
            <div class="page-wrap split">
                <div class="stack">
                    <p class="eyebrow">"Questions"</p>
                    <h2 class="section-title">"Before you contribute"</h2>
                    <div class="sec-list" style="margin-top:.4rem">
                        <div class="sec-item">
                            <span class="sec-num">"01"</span>
                            <div>
                                <strong>"Does sponsorship change the license?"</strong>
                                " No. Every project keeps its existing license and stays fully open."
                            </div>
                        </div>
                        <div class="sec-item">
                            <span class="sec-num">"02"</span>
                            <div>
                                <strong>"Can we get a logo on the site?"</strong>
                                " Yes, from the $100 Sponsor tier upward, subject to a quick content review."
                            </div>
                        </div>
                        <div class="sec-item">
                            <span class="sec-num">"03"</span>
                            <div>
                                <strong>"Do you offer support contracts?"</strong>
                                " The Enterprise Sponsor and Member tiers include priority support; consultancy is arranged separately."
                            </div>
                        </div>
                        <div class="sec-item">
                            <span class="sec-num">"04"</span>
                            <div>
                                <strong>"Can we fund a specific feature?"</strong>
                                " Often. Write to us with the scope before contributing so we can confirm it is maintainable."
                            </div>
                        </div>
                    </div>
                </div>
                <div class="glass card">
                    <p class="aside-head">"Fiscal host"</p>
                    <h3>"Open Source Collective"</h3>
                    <p style="margin-top:.5rem;font-size:.9rem">
                        "A non-profit fiscal host that handles payments, accounting, and expense
                         payouts for open-source collectives. Contributions are not tax-deductible
                         unless the host states otherwise for your jurisdiction."
                    </p>
                    <div class="row" style="margin-top:1.1rem">
                        <a class="btn btn-ghost btn-sm" href=OPEN_COLLECTIVE_PROJECTS target="_blank" rel="noopener noreferrer">
                            "All collectives"
                            <Icon name="arrow-up-right" class="icon-xs" />
                        </a>
                        <a class="btn btn-ghost btn-sm" href="/contact/">
                            "Contact"
                            <Icon name="arrow-right" class="icon-xs" />
                        </a>
                    </div>
                </div>
            </div>
        </section>
    }
}
