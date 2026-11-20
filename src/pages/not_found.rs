use leptos::prelude::*;

use crate::components::ui::Icon;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <section class="notfound">
            <div class="page-wrap">
                <img
                    class="nf-mascot"
                    src="/assets/img/sugie.svg"
                    alt="Sugie, the Apich sugar glider, gliding"
                    width="512"
                    height="512"
                />
                <p class="nf-code">"404"</p>
                <h1 style="margin-top:1rem">"This page moved or never existed"</h1>
                <p class="lede" style="margin:1rem auto 0;max-width:44ch">
                    "Project sites were consolidated under apich.org. If you followed a link from
                     an old subdomain, the page you want is probably one of these."
                </p>
                <div class="row" style="justify-content:center;margin-top:1.8rem">
                    <a class="btn btn-primary" href="/projects/">
                        "All projects"
                        <Icon name="arrow-right" class="icon-sm" />
                    </a>
                    <a class="btn btn-ghost" href="/">"Home"</a>
                    <a class="btn btn-ghost" href="/contact/">"Contact"</a>
                </div>
                <div class="link-grid link-grid--2" style="margin:2.2rem auto 0;max-width:44rem">
                    <a class="link-card" href="/projects/dtact/">
                        <span class="label">"dtact"</span>
                        <span class="sub">"runtime"</span>
                    </a>
                    <a class="link-card" href="/projects/ensia/">
                        <span class="label">"ensia"</span>
                        <span class="sub">"obfuscator"</span>
                    </a>
                    <a class="link-card" href="/projects/rssn/">
                        <span class="label">"rssn"</span>
                        <span class="sub">"science"</span>
                    </a>
                    <a class="link-card" href="/projects/bincode/">
                        <span class="label">"bincode-next"</span>
                        <span class="sub">"serialization"</span>
                    </a>
                </div>
            </div>
        </section>
    }
}
