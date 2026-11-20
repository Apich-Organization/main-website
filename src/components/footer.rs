use leptos::prelude::*;

use crate::components::ui::Icon;
use crate::route::{Project, Route};
use crate::site::*;

#[component]
pub fn Footer() -> impl IntoView {
    let projects = Project::ALL
        .into_iter()
        .map(|p| {
            let href = format!("/projects/{}/", p.slug());
            view! { <li><a class="footer-link" href=href>{p.name()}</a></li> }
        })
        .collect_view();

    view! {
        <footer class="footer">
            <div class="footer-rings" aria-hidden="true">
                <span class="ring ring-1"></span>
                <span class="ring ring-2"></span>
                <span class="ring ring-3"></span>
            </div>

            <div class="page-wrap footer-grid">
                <div class="footer-brand">
                    <a class="brand brand--lockup" href="/" aria-label="Apich Organization — home">
                        <img
                            class="brand-logo brand-logo--light"
                            src="/assets/img/brand/apich-logo.png"
                            alt="Apich Organization"
                            width="640"
                            height="424"
                            decoding="async"
                        />
                        <img
                            class="brand-logo brand-logo--dark"
                            src="/assets/img/brand/apich-logo-dark.png"
                            alt=""
                            width="640"
                            height="424"
                            decoding="async"
                        />
                    </a>
                    <p class="footer-blurb">{TAGLINE}</p>
                    <div class="footer-social">
                        <a class="icon-btn" href=GITHUB_ORG target="_blank" rel="noopener noreferrer" aria-label="GitHub">
                            <Icon name="github" />
                        </a>
                        <a class="icon-btn" href=DISCORD target="_blank" rel="noopener noreferrer" aria-label="Discord">
                            <Icon name="discord" />
                        </a>
                        <a class="icon-btn" href=ZULIP target="_blank" rel="noopener noreferrer" aria-label="Zulip">
                            <Icon name="chat" />
                        </a>
                        <a class="icon-btn" href=format!("mailto:{EMAIL_GENERAL}") aria-label="Email">
                            <Icon name="mail" />
                        </a>
                    </div>
                </div>

                <nav class="footer-col" aria-label="Projects">
                    <h3 class="footer-head">"Projects"</h3>
                    <ul>{projects}</ul>
                </nav>

                <nav class="footer-col" aria-label="Organization">
                    <h3 class="footer-head">"Organization"</h3>
                    <ul>
                        <li><a class="footer-link" href="/research/">"Research team"</a></li>
                        <li><a class="footer-link" href="/security/">"Security team"</a></li>
                        <li><a class="footer-link" href="/blog/">"Blog"</a></li>
                        <li><a class="footer-link" href="/sponsor/">"Sponsorship"</a></li>
                        <li><a class="footer-link" href="/contact/">"Contact"</a></li>
                    </ul>
                </nav>

                <nav class="footer-col" aria-label="Elsewhere">
                    <h3 class="footer-head">"Elsewhere"</h3>
                    <ul>
                        <li>
                            <a class="footer-link" href=OPEN_COLLECTIVE target="_blank" rel="noopener noreferrer">
                                "Open Collective"
                            </a>
                        </li>
                        <li>
                            <a class="footer-link" href=STATUS_PAGE target="_blank" rel="noopener noreferrer">
                                "Status"
                            </a>
                        </li>
                        <li>
                            <a class="footer-link" href=ZULIP target="_blank" rel="noopener noreferrer">
                                "Zulip chat"
                            </a>
                        </li>
                        <li>
                            <a class="footer-link" href=format!("mailto:{EMAIL_SECURITY}")>
                                "security@apich.org"
                            </a>
                        </li>
                        <li>
                            <a class="footer-link" href="/blog/feed.xml">"RSS feed"</a>
                        </li>
                    </ul>
                </nav>
            </div>

            <div class="page-wrap footer-bottom">
                <p class="footer-legal">
                    "© 2026 Apich Organization. Content licensed CC BY 4.0; code under the license of each project."
                </p>
                <p class="footer-meta">
                    <a href="/sitemap.xml">"Sitemap"</a>
                    <span class="dot-sep">"·"</span>
                    <a href="/llms.txt">"llms.txt"</a>
                    <span class="dot-sep">"·"</span>
                    <span>"Static site, generated with Leptos"</span>
                </p>
            </div>
        </footer>
    }
}

/// Keeps `Route` in scope for the footer's type inference in some builds.
#[allow(dead_code)]
fn _route_marker(_r: Route) {}
