use leptos::prelude::*;

use crate::components::ui::{Icon, SectionHead};
use crate::site::*;

#[component]
pub fn ContactPage() -> impl IntoView {
    view! {
        <section class="pt-nav">
            <div class="page-wrap">
                <div class="page-head">
                    <p class="eyebrow">"Contact"</p>
                    <h1>"Write to the right team"</h1>
                    <p class="lede">
                        "Four addresses reach four different groups. Use the security address for
                         vulnerabilities — not GitHub issues, and not the general inbox."
                    </p>
                </div>

                <div class="contact-grid">
                    <div class="contact-card">
                        <span class="contact-label">"General"</span>
                        <span class="contact-role">"Everything else"</span>
                        <a class="contact-mail" href=format!("mailto:{EMAIL_GENERAL}")>{EMAIL_GENERAL}</a>
                        <p class="contact-note">
                            "Collaboration, press, sponsorship questions, and anything that does not
                             fit below."
                        </p>
                    </div>
                    <div class="contact-card">
                        <span class="contact-label">"Security"</span>
                        <span class="contact-role">"Vulnerability reports"</span>
                        <a class="contact-mail" href=format!("mailto:{EMAIL_SECURITY}")>{EMAIL_SECURITY}</a>
                        <p class="contact-note">
                            "Encrypted with the team PGP key where possible. We respond within seven
                             days."
                        </p>
                    </div>
                    <div class="contact-card">
                        <span class="contact-label">"Research"</span>
                        <span class="contact-role">"Science & Research Team"</span>
                        <a class="contact-mail" href=format!("mailto:{EMAIL_RESEARCH}")>{EMAIL_RESEARCH}</a>
                        <p class="contact-note">
                            "Academic collaboration, preprint questions, and applied research
                             projects."
                        </p>
                    </div>
                    <div class="contact-card">
                        <span class="contact-label">"Admin"</span>
                        <span class="contact-role">"Administration & conduct"</span>
                        <a class="contact-mail" href=format!("mailto:{EMAIL_ADMIN}")>{EMAIL_ADMIN}</a>
                        <p class="contact-note">
                            "Membership, access, and the Code of Conduct Committee."
                        </p>
                    </div>
                </div>
            </div>
        </section>

        <section class="section">
            <div class="page-wrap split">
                <div class="stack">
                    <p class="eyebrow">"Reporting a vulnerability"</p>
                    <h2 class="section-title">"Email, do not open an issue"</h2>
                    <p class="section-lead">
                        "A public issue tells everyone about the problem before a fix exists. Send
                         the details to the security address; if you want to encrypt the message,
                         use the published key."
                    </p>
                    <div class="row" style="margin-top:.8rem">
                        <a class="btn btn-ghost" href="/security/">
                            "Read the security policy"
                            <Icon name="arrow-right" class="icon-sm" />
                        </a>
                        <a class="btn btn-ghost" href="/assets/security-pgp.asc" download="apich-security.asc">
                            <Icon name="lock" class="icon-sm" />
                            "PGP key"
                        </a>
                    </div>
                </div>
                <div class="glass card">
                    <p class="aside-head">"Response times"</p>
                    <div class="spec-list">
                        <div class="spec-row"><dt>"Security"</dt><dd>"≤ 7 days"</dd></div>
                        <div class="spec-row"><dt>"Research"</dt><dd>"≤ 10 days"</dd></div>
                        <div class="spec-row"><dt>"General"</dt><dd>"≤ 5 working days"</dd></div>
                        <div class="spec-row"><dt>"Conduct"</dt><dd>"≤ 72 hours"</dd></div>
                    </div>
                    <p class="contact-note" style="margin-top:1rem">
                        "Times are targets, not guarantees. If a report is urgent, say so in the
                         subject line."
                    </p>
                </div>
            </div>
        </section>

        <section class="section">
            <div class="page-wrap">
                <SectionHead
                    eyebrow="Elsewhere"
                    title="Public channels"
                    lead="Most day-to-day conversation happens in the open. Prefer these over email for anything that is not sensitive."
                />
                <div class="link-grid">
                    <a class="link-card" href=GITHUB_ORG target="_blank" rel="noopener noreferrer">
                        <Icon name="github" />
                        <span class="label">"GitHub"</span>
                        <span class="sub">"issues · PRs"</span>
                    </a>
                    <a class="link-card" href=ZULIP target="_blank" rel="noopener noreferrer">
                        <Icon name="chat" />
                        <span class="label">"Zulip"</span>
                        <span class="sub">"recommended"</span>
                    </a>
                    <a class="link-card" href=DISCORD target="_blank" rel="noopener noreferrer">
                        <Icon name="discord" />
                        <span class="label">"Discord"</span>
                        <span class="sub">"community"</span>
                    </a>
                    <a class="link-card" href=OPEN_COLLECTIVE target="_blank" rel="noopener noreferrer">
                        <Icon name="heart" />
                        <span class="label">"Open Collective"</span>
                        <span class="sub">"funding"</span>
                    </a>
                </div>
            </div>
        </section>

        <section class="section">
            <div class="page-wrap">
                <div class="cta-band">
                    <div>
                        <p class="eyebrow">"Consultancy"</p>
                        <h2>"Need the team on a problem?"</h2>
                        <p>
                            "The consultancy branch takes on applied work: performance engineering,
                             numerical method selection, compiler hardening, and security review of
                             dual-use tooling. Describe the constraint that makes the problem hard."
                        </p>
                    </div>
                    <div class="cta-actions">
                        <a class="btn btn-primary" href=format!("mailto:{EMAIL_GENERAL}?subject=Consultancy%20inquiry")>
                            "Start an inquiry"
                            <Icon name="arrow-right" class="icon-sm" />
                        </a>
                    </div>
                </div>
            </div>
        </section>
    }
}
