use leptos::prelude::*;

use crate::components::ui::{Icon, SectionHead, Stat};
use crate::site::EMAIL_SECURITY;

#[component]
pub fn SecurityPage() -> impl IntoView {
    view! {
        <section class="pt-nav">
            <div class="page-wrap">
                <div class="page-head">
                    <p class="eyebrow">"Security Team"</p>
                    <h1>"Security is a release gate"</h1>
                    <p class="lede">
                        "Every project in this organization passes a security review before it is
                         published. This page states what we support, how to report a problem, and
                         the baseline we hold ourselves to."
                    </p>
                    <div class="row" style="margin-top:1.5rem">
                        <a class="btn btn-primary" href=format!("mailto:{EMAIL_SECURITY}")>
                            <Icon name="mail" />
                            "security@apich.org"
                        </a>
                        <a class="btn btn-ghost" href="/assets/security-pgp.asc" download="apich-security.asc">
                            <Icon name="lock" class="icon-sm" />
                            "PGP public key"
                        </a>
                    </div>
                    <div class="key-block">
                        <span class="key-label">"Key fingerprint"</span>
                        <code class="key-fingerprint">
                            "BB45 2767 B5A4 BDFD B606 2D83 1373 B738 BCD8 A75D"
                        </code>
                        <span class="key-meta">
                            "brainpoolP512r1 · created 2026-02-19 · expires 2030-02-19 · the same key is listed in "
                            <a href="/.well-known/security.txt">"security.txt"</a>
                        </span>
                    </div>
                </div>

                <div class="stat-row">
                    <Stat value="7 days" label="Response commitment" note="all reports" />
                    <Stat value="0" label="Issues accepted via GitHub" note="email only" />
                    <Stat value="PGP" label="Encrypted reports" note="key published" />
                    <Stat value="Dual-use" label="Review on obfuscation" note="per release" />
                </div>
            </div>
        </section>

        <section class="section">
            <div class="page-wrap">
                <SectionHead
                    eyebrow="Supported versions"
                    title="What we patch"
                    lead="Support follows the project's release stage. Forks are not covered; if you maintain one, we will help you understand a fix but cannot ship it for you."
                />
                <div class="table-wrap" tabindex="0" role="region" aria-label="Supported versions">
                    <table class="data">
                        <caption>"Support policy by release stage. “Current” means the latest published minor version."</caption>
                        <thead>
                            <tr>
                                <th scope="col">"Release stage"</th>
                                <th scope="col">"Version shape"</th>
                                <th scope="col">"Supported"</th>
                                <th scope="col">"Notes"</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td><strong>"Major"</strong></td>
                                <td class="num">"x.y.z"</td>
                                <td class="num">"current + previous"</td>
                                <td>"Current minor version and the one before it."</td>
                            </tr>
                            <tr>
                                <td><strong>"Initial development"</strong></td>
                                <td class="num">"0.x.y"</td>
                                <td class="num">"x"</td>
                                <td>"Only the current minor version."</td>
                            </tr>
                            <tr>
                                <td><strong>"Experimental"</strong></td>
                                <td class="num">"0.0.x"</td>
                                <td class="num">"latest patch"</td>
                                <td>"Only the latest patch release."</td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
        </section>

        <section class="section">
            <div class="page-wrap split">
                <div class="stack">
                    <p class="eyebrow">"Reporting"</p>
                    <h2 class="section-title">"Report privately, not in public"</h2>
                    <p class="section-lead">
                        "Please do not open a GitHub issue for a vulnerability. Send the report to
                         the security address, encrypted with the team's PGP key if the details are
                         sensitive. We acknowledge every report and commit to addressing
                         responsibly disclosed vulnerabilities within seven days of receipt."
                    </p>
                    <div class="callout" style="margin-top:.6rem">
                        <Icon name="mail" />
                        <div>
                            <strong>"Include:"</strong>
                            " affected version and commit, platform, a reproduction or proof of
                             concept, and the impact you believe it has. If you need a channel that
                             is not email, say so in the first message."
                        </div>
                    </div>
                </div>

                <div class="glass card">
                    <p class="aside-head">"Disclosure timeline"</p>
                    <div class="sec-list">
                        <div class="sec-item">
                            <span class="sec-num">"T+0"</span>
                            <div><strong>"Report received."</strong>" Acknowledged by a team member."</div>
                        </div>
                        <div class="sec-item">
                            <span class="sec-num">"≤ 7d"</span>
                            <div><strong>"Triage and fix."</strong>" Severity assessed, patch prepared, affected releases identified."</div>
                        </div>
                        <div class="sec-item">
                            <span class="sec-num">"≤ 30d"</span>
                            <div><strong>"Advisory."</strong>" Coordinated publication with credit, unless you ask to remain anonymous."</div>
                        </div>
                    </div>
                    <p class="contact-note" style="margin-top:1rem">
                        "Past advisories are recorded as CVE entries where a CVE identifier was assigned."
                    </p>
                </div>
            </div>
        </section>

        <section class="section">
            <div class="page-wrap">
                <SectionHead
                    eyebrow="Standards"
                    title="The baseline for organization machines"
                    lead="Mandatory for members, recommended for contributors. These are the controls we actually run, not an aspirational checklist."
                />
                <div class="grid grid-3">
                    <div class="glass card">
                        <span class="pillar-icon"><Icon name="terminal" class="icon-lg" /></span>
                        <h3 style="margin-top:.6rem">"Operating environment"</h3>
                        <ul class="stack" style="gap:.4rem;margin-top:.5rem;font-size:.88rem;color:var(--text-2)">
                            <li>"Fedora KDE Plasma Desktop or Qubes OS"</li>
                            <li>"GNU Emacs or Zed"</li>
                            <li>"LUKS disk encryption, enabled at install"</li>
                            <li>"Root logins and root SSH access disabled"</li>
                        </ul>
                    </div>
                    <div class="glass card">
                        <span class="pillar-icon"><Icon name="shield" class="icon-lg" /></span>
                        <h3 style="margin-top:.6rem">"Security suite"</h3>
                        <ul class="stack" style="gap:.4rem;margin-top:.5rem;font-size:.88rem;color:var(--text-2)">
                            <li>"Wazuh agent and ESET Endpoint Security"</li>
                            <li>"SELinux enforcing, MLS policy"</li>
                            <li>"ClamAV, rkhunter, chkrootkit, AIDE"</li>
                            <li>"Lynis, firewalld, daily Nmap scan"</li>
                        </ul>
                    </div>
                    <div class="glass card">
                        <span class="pillar-icon"><Icon name="lock" class="icon-lg" /></span>
                        <h3 style="margin-top:.6rem">"Identity"</h3>
                        <ul class="stack" style="gap:.4rem;margin-top:.5rem;font-size:.88rem;color:var(--text-2)">
                            <li>"Hardware 2FA on every organizational account"</li>
                            <li>"YubiKey 5 FIPS Series (FIPS 140-3)"</li>
                            <li>"FIDO2 via biometrics or PIN"</li>
                            <li>"No shared credentials, no exceptions"</li>
                        </ul>
                    </div>
                </div>
            </div>
        </section>

        <section class="section">
            <div class="page-wrap">
                <SectionHead
                    eyebrow="Kernel baseline"
                    title="Hardening, in eight groups"
                    lead="The full sysctl baseline is published in the security repository. The groups below are what it enforces."
                />
                <div class="grid grid-2">
                    <div class="sec-list">
                        <div class="sec-item">
                            <span class="sec-num">"A"</span>
                            <div><strong>"Kernel self-protection."</strong>" Strict ASLR, restricted kptr and dmesg, magic SysRq off, perf and kexec locked down."</div>
                        </div>
                        <div class="sec-item">
                            <span class="sec-num">"B"</span>
                            <div><strong>"Trace and execution."</strong>" Yama ptrace scope 3, BPF JIT hardening, unprivileged BPF disabled, TIOCSTI blocked."</div>
                        </div>
                        <div class="sec-item">
                            <span class="sec-num">"C"</span>
                            <div><strong>"Namespace boundaries."</strong>" Unprivileged user namespaces disabled to close local privilege-escalation paths."</div>
                        </div>
                        <div class="sec-item">
                            <span class="sec-num">"D"</span>
                            <div><strong>"Filesystem and dumps."</strong>" Symlink and hardlink protection, sticky-directory rules, suid core dumps suppressed."</div>
                        </div>
                    </div>
                    <div class="sec-list">
                        <div class="sec-item">
                            <span class="sec-num">"E"</span>
                            <div><strong>"Memory management."</strong>" Strict overcommit, expanded map count, controlled allocation failure."</div>
                        </div>
                        <div class="sec-item">
                            <span class="sec-num">"F"</span>
                            <div><strong>"IPv4."</strong>" Reverse-path filtering, martian logging, no source routing, no redirects."</div>
                        </div>
                        <div class="sec-item">
                            <span class="sec-num">"G"</span>
                            <div><strong>"Flood mitigation."</strong>" SYN cookies, RFC 1337, ICMP echo and broadcast suppression."</div>
                        </div>
                        <div class="sec-item">
                            <span class="sec-num">"H"</span>
                            <div><strong>"IPv6."</strong>" Stack disabled where unused, router advertisements and redirects refused."</div>
                        </div>
                    </div>
                </div>
            </div>
        </section>

        <section class="section">
            <div class="page-wrap">
                <div class="cta-band">
                    <div>
                        <p class="eyebrow">"Disclosure"</p>
                        <h2>"Found something? Tell us first."</h2>
                        <p>
                            "We will not pursue legal action against researchers who act in good
                             faith, stay within the scope of the affected project, and give us the
                             agreed window to ship a fix."
                        </p>
                    </div>
                    <div class="cta-actions">
                        <a class="btn btn-primary" href=format!("mailto:{EMAIL_SECURITY}")>
                            "Email the security team"
                            <Icon name="arrow-right" class="icon-sm" />
                        </a>
                    </div>
                </div>
            </div>
        </section>
    }
}
