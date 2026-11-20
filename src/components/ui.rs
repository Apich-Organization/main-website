//! Small, reusable presentational pieces shared across pages.

use leptos::prelude::*;

/// Line icon set. Kept as inline SVG so icons are crisp, themeable and free of
/// extra requests.
#[component]
pub fn Icon(
    name: &'static str,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let paths: &[&str] = match name {
        "sun" => &[
            "M12 3v1.5M12 19.5V21M4.2 4.2l1.1 1.1M18.7 18.7l1.1 1.1M3 12h1.5M19.5 12H21M4.2 19.8l1.1-1.1M18.7 5.3l1.1-1.1",
            "M12 16.5a4.5 4.5 0 1 0 0-9 4.5 4.5 0 0 0 0 9Z",
        ],
        "moon" => &["M20 14.5A8.5 8.5 0 0 1 9.5 4a8.5 8.5 0 1 0 10.5 10.5Z"],
        "github" => &[
            "M9 19c-4 1.5-4-2.5-6-3m12 5v-3.5c0-1 .1-1.4-.5-2 2.8-.3 5.5-1.4 5.5-6a4.6 4.6 0 0 0-1.3-3.2 4.3 4.3 0 0 0-.1-3.2s-1.4-.4-4.6 1.7a11.4 11.4 0 0 0-6 0C4.8 2.7 3.4 3.1 3.4 3.1a4.3 4.3 0 0 0-.1 3.2A4.6 4.6 0 0 0 2 9.5c0 4.6 2.7 5.7 5.5 6-.6.6-.6 1.2-.5 2V21",
        ],
        "heart" => &["M20.8 5.6a5.2 5.2 0 0 0-7.4 0L12 7l-1.4-1.4a5.2 5.2 0 1 0-7.4 7.4l8.8 8.8 8.8-8.8a5.2 5.2 0 0 0 0-7.4Z"],
        "menu" => &["M4 7h16M4 12h16M4 17h16"],
        "close" => &["M6 6l12 12M18 6L6 18"],
        "arrow-right" => &["M5 12h14M13 6l6 6-6 6"],
        "arrow-up-right" => &["M7 17 17 7M9 7h8v8"],
        "arrow-up" => &["M12 19V5M6 11l6-6 6 6"],
        "external" => &["M14 4h6v6M20 4l-9 9M18 14v5a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1V7a1 1 0 0 1 1-1h5"],
        "mail" => &["M3 6.5h18v11H3z", "M3 7l9 6 9-6"],
        "discord" => &["M8.5 8.5a9 9 0 0 1 7 0M7 17c-1.5-2.5-2-5-1.5-8 1-.8 2.5-1.5 3.5-1.8l.7 1.4a9.6 9.6 0 0 1 4.6 0l.7-1.4c1 .3 2.5 1 3.5 1.8.5 3 0 5.5-1.5 8-1.2.9-2.5 1.4-3.8 1.6l-.7-1.4a7 7 0 0 1-2.2 0L9.8 18.6c-1.3-.2-2.6-.7-3.8-1.6Z"],
        "chat" => &["M21 12a8 8 0 0 1-8 8H8l-5 3 1.4-4.6A8 8 0 1 1 21 12Z"],
        "shield" => &["M12 3l7.5 3v6c0 4.5-3 7.7-7.5 9-4.5-1.3-7.5-4.5-7.5-9V6L12 3Z", "M9 12l2 2 4-4"],
        "book" => &["M4 5.5A2.5 2.5 0 0 1 6.5 3H20v15H6.5A2.5 2.5 0 0 0 4 20.5Z", "M4 5.5v15"],
        "terminal" => &["M4 5h16v14H4z", "M8 10l2.5 2L8 14M13 14h3"],
        "package" => &["M12 3l8 4.5v9L12 21l-8-4.5v-9L12 3Z", "M4 7.5l8 4.5 8-4.5M12 12v9"],
        "cpu" => &["M7 7h10v10H7z", "M4 10h3M4 14h3M17 10h3M17 14h3M10 4v3M14 4v3M10 17v3M14 17v3"],
        "layers" => &["M12 3l9 5-9 5-9-5 9-5Z", "M3 13l9 5 9-5M3 17l9 5 9-5"],
        "sparkles" => &["M12 3l1.8 4.7L18.5 9.5 13.8 11.3 12 16l-1.8-4.7L5.5 9.5l4.7-1.8L12 3Z", "M18.5 15l.9 2.3 2.3.9-2.3.9-.9 2.3-.9-2.3-2.3-.9 2.3-.9.9-2.3Z"],
        "check" => &["M4.5 12.5l5 5 10-11"],
        "copy" => &["M9 9h11v11H9z", "M15 5H4v11"],
        "rss" => &["M5 19h.01M4 11a9 9 0 0 1 9 9M4 5a15 15 0 0 1 15 15"],
        "calendar" => &["M4 6h16v14H4z", "M4 10h16M9 4v4M15 4v4"],
        "users" => &["M16 20v-2a4 4 0 0 0-4-4H7a4 4 0 0 0-4 4v2", "M9.5 10a3.5 3.5 0 1 0 0-7 3.5 3.5 0 0 0 0 7Z", "M21 20v-2a4 4 0 0 0-3-3.9M16.5 3.2a3.5 3.5 0 0 1 0 6.6"],
        "globe" => &["M12 21a9 9 0 1 0 0-18 9 9 0 0 0 0 18Z", "M3.5 9h17M3.5 15h17", "M12 3c2.5 2.5 3.5 5.5 3.5 9s-1 6.5-3.5 9c-2.5-2.5-3.5-5.5-3.5-9S9.5 5.5 12 3Z"],
        "zap" => &["M13 3 5 14h6l-1 7 8-11h-6l1-7Z"],
        "lock" => &["M6 10h12v10H6z", "M8.5 10V7.5a3.5 3.5 0 0 1 7 0V10"],
        "chart" => &["M4 20V4M4 20h16", "M8 16v-4M12.5 16V8M17 16v-6"],
        "git" => &["M6 3v12", "M6 21a3 3 0 1 0 0-6 3 3 0 0 0 0 6Z", "M18 9a3 3 0 1 0 0-6 3 3 0 0 0 0 6Z", "M18 9a6 6 0 0 1-6 6H6"],
        "quote" => &["M9 7H5v5h4c0 2.5-1.5 4-4 5", "M19 7h-4v5h4c0 2.5-1.5 4-4 5"],
        "file" => &["M6 3h8l4 4v14H6z", "M14 3v4h4"],
        "grid" => &["M4 4h7v7H4zM13 4h7v7h-7zM4 13h7v7H4zM13 13h7v7h-7z"],
        "compass" => &["M12 21a9 9 0 1 0 0-18 9 9 0 0 0 0 18Z", "M15.5 8.5l-2 5-5 2 2-5 5-2Z"],
        _ => &["M12 21a9 9 0 1 0 0-18 9 9 0 0 0 0 18Z"],
    };

    let inner = paths
        .iter()
        .map(|d| view! { <path d=*d /> })
        .collect_view();

    view! {
        <svg
            class=format!("icon {class}")
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="1.7"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
            focusable="false"
        >
            {inner}
        </svg>
    }
}

/// Section header: eyebrow, title, optional lead paragraph.
#[component]
pub fn SectionHead(
    #[prop(into)] eyebrow: String,
    #[prop(into)] title: String,
    #[prop(optional, into)] lead: String,
    #[prop(optional)] center: bool,
) -> impl IntoView {
    view! {
        <div class="section-head" class:center=center>
            <p class="eyebrow">{eyebrow}</p>
            <h2 class="section-title">{title}</h2>
            {(!lead.is_empty()).then(|| view! { <p class="section-lead">{lead}</p> })}
        </div>
    }
}

/// A single figure with a caption, used in stat rows.
#[component]
pub fn Stat(
    #[prop(into)] value: String,
    #[prop(into)] label: String,
    #[prop(optional, into)] note: String,
) -> impl IntoView {
    view! {
        <div class="stat">
            <div class="stat-value">{value}</div>
            <div class="stat-label">{label}</div>
            {(!note.is_empty()).then(|| view! { <div class="stat-note">{note}</div> })}
        </div>
    }
}

/// Pill-shaped label.
#[component]
pub fn Badge(#[prop(into)] text: String, #[prop(optional, into)] tone: String) -> impl IntoView {
    let tone = if tone.is_empty() { "neutral".to_string() } else { tone };
    view! { <span class=format!("badge badge-{tone}")>{text}</span> }
}

/// Outbound link that opens in a new tab, with the conventional affordances.
#[component]
pub fn ExternalLink(
    href: &'static str,
    #[prop(into)] label: String,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <a class=format!("link-ext {class}") href=href target="_blank" rel="noopener noreferrer">
            <span>{label}</span>
            <Icon name="arrow-up-right" class="icon-xs" />
        </a>
    }
}
