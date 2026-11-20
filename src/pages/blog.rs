use leptos::prelude::*;

use crate::components::ui::{Badge, Icon};
use crate::content::posts;
use crate::site::ZULIP;

#[component]
pub fn BlogPage() -> impl IntoView {
    let rows = posts::ALL
        .iter()
        .map(|p| {
            let href = format!("/blog/{}/", p.slug);
            view! {
                <a class="post-row" href=href>
                    <span class="post-date">{p.date}</span>
                    <span>
                        <h2 class="post-title">{p.title}</h2>
                        <span class="post-excerpt">{p.summary}</span>
                    </span>
                    <span class="arrow"><Icon name="arrow-right" class="icon-sm" /></span>
                </a>
            }
        })
        .collect_view();

    view! {
        <section class="pt-nav">
            <div class="page-wrap">
                <div class="page-head">
                    <p class="eyebrow">"Blog"</p>
                    <h1>"Announcements and engineering notes"</h1>
                    <p class="lede">
                        "Release notes, design decisions, security advisories, and research
                         updates. Short, dated, and permanent — we do not rewrite published posts."
                    </p>
                    <div class="row" style="margin-top:1.4rem">
                        <a class="btn btn-ghost" href="/blog/feed.xml">
                            <Icon name="rss" class="icon-sm" />
                            "RSS feed"
                        </a>
                        <a class="btn btn-ghost" href=ZULIP target="_blank" rel="noopener noreferrer">
                            <Icon name="chat" class="icon-sm" />
                            "Discuss on Zulip"
                        </a>
                    </div>
                </div>
            </div>
        </section>

        <section class="section section--tight">
            <div class="page-wrap">
                <div class="post-list">{rows}</div>
            </div>
        </section>
    }
}

#[component]
pub fn PostPage(slug: &'static str) -> impl IntoView {
    let post = posts::get(slug);

    let Some(post) = post else {
        return view! {
            <section class="pt-nav">
                <div class="page-wrap">
                    <div class="page-head">
                        <h1>"Post not found"</h1>
                        <p class="lede">"That article does not exist."</p>
                        <a class="btn btn-ghost" href="/blog/">"Back to the blog"</a>
                    </div>
                </div>
            </section>
        }
        .into_any();
    };

    let others = posts::ALL
        .iter()
        .filter(|p| p.slug != post.slug)
        .map(|p| {
            let href = format!("/blog/{}/", p.slug);
            view! {
                <a class="related-card" href=href>
                    <span>
                        <span class="related-name" style="font-size:1rem">{p.title}</span>
                        <span class="related-cat" style="display:block;margin-top:.2rem">{p.date}</span>
                    </span>
                    <Icon name="arrow-right" class="icon-sm" />
                </a>
            }
        })
        .collect_view();

    view! {
        <article>
            <header class="post-hero">
                <div class="page-wrap">
                    <nav class="breadcrumb" aria-label="Breadcrumb">
                        <a href="/">"Home"</a>
                        <span class="sep">"/"</span>
                        <a href="/blog/">"Blog"</a>
                        <span class="sep">"/"</span>
                        <span aria-current="page">{post.tag}</span>
                    </nav>
                    <Badge text=post.tag.to_string() tone="brand" />
                    <h1>{post.title}</h1>
                    <p class="lede" style="max-width:56ch">{post.summary}</p>
                    <div class="post-byline">
                        <span>{post.date}</span>
                        <span>"·"</span>
                        <span>{post.reading_time}</span>
                        <span>"·"</span>
                        <span>"Apich Organization"</span>
                    </div>
                </div>
            </header>

            <div class="page-wrap post-body">
                <div class="prose" inner_html=post.body></div>

                <div class="post-foot">
                    <span>"Published " {post.date} " · Licensed CC BY 4.0"</span>
                    <a class="tier-link" href="/blog/feed.xml">
                        <Icon name="rss" class="icon-xs" />
                        "Subscribe"
                    </a>
                </div>

                <div class="post-nav">{others}</div>
            </div>
        </article>
    }
    .into_any()
}
