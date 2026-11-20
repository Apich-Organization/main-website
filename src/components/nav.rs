use leptos::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;

use crate::components::ui::Icon;
use crate::route::Route;
use crate::site::{GITHUB_ORG, OPEN_COLLECTIVE};

/// Primary navigation. Links are plain anchors pointing at real, prerendered
/// URLs; on the client the router upgrades them to instant navigations.
#[component]
pub fn Nav() -> impl IntoView {
    let current = expect_context::<RwSignal<Route>>();
    let menu_open = RwSignal::new(false);

    let section = move || current.get().nav_section();

    // Close the mobile drawer whenever the route changes.
    Effect::new(move |_| {
        current.get();
        menu_open.set(false);
    });

    view! {
        <header class="nav" id="site-nav" class:open=move || menu_open.get()>
            <div class="nav-inner">
                <a class="brand" href="/" aria-label="Apich Organization — home">
                    <span class="brand-text">
                        <span class="brand-name">"Apich"</span>
                        <span class="brand-sub">"Organization"</span>
                    </span>
                </a>

                <nav class="nav-links" aria-label="Primary">
                    <a class="nav-link" href="/projects/" class:active=move || section() == "projects">
                        "Projects"
                    </a>
                    <a class="nav-link" href="/research/" class:active=move || section() == "research">
                        "Research"
                    </a>
                    <a class="nav-link" href="/security/" class:active=move || section() == "security">
                        "Security"
                    </a>
                    <a class="nav-link" href="/blog/" class:active=move || section() == "blog">
                        "Blog"
                    </a>
                    <a class="nav-link" href="/sponsor/" class:active=move || section() == "sponsor">
                        "Sponsor"
                    </a>
                </nav>

                <div class="nav-actions">
                    <button
                        class="icon-btn theme-toggle"
                        type="button"
                        aria-label="Switch between light and dark theme"
                        title="Switch between light and dark theme"
                        on:click=move |_| toggle_theme()
                    >
                        <Icon name="sun" class="icon-sun" />
                        <Icon name="moon" class="icon-moon" />
                    </button>
                    <a
                        class="icon-btn nav-gh"
                        href=GITHUB_ORG
                        target="_blank"
                        rel="noopener noreferrer"
                        aria-label="Apich Organization on GitHub"
                        title="GitHub"
                    >
                        <Icon name="github" />
                    </a>
                    <a
                        class="btn btn-primary btn-sm nav-cta"
                        href=OPEN_COLLECTIVE
                        target="_blank"
                        rel="noopener noreferrer"
                        aria-label="Support us on Open Collective"
                        title="Support us on Open Collective"
                    >
                        <Icon name="heart" />
                        <span>"Support us"</span>
                    </a>
                    <button
                        class="icon-btn nav-burger"
                        type="button"
                        aria-label="Menu"
                        aria-expanded=move || menu_open.get().to_string()
                        aria-controls="nav-drawer"
                        on:click=move |_| menu_open.update(|v| *v = !*v)
                    >
                        <Icon name="menu" class="icon-menu" />
                        <Icon name="close" class="icon-close" />
                    </button>
                </div>
            </div>

            <div id="nav-drawer" class="nav-drawer" class:open=move || menu_open.get()>
                <a class="drawer-link" href="/projects/" on:click=move |_| menu_open.set(false)>"Projects"</a>
                <a class="drawer-link" href="/research/" on:click=move |_| menu_open.set(false)>"Research"</a>
                <a class="drawer-link" href="/security/" on:click=move |_| menu_open.set(false)>"Security"</a>
                <a class="drawer-link" href="/blog/" on:click=move |_| menu_open.set(false)>"Blog"</a>
                <a class="drawer-link" href="/sponsor/" on:click=move |_| menu_open.set(false)>"Sponsor"</a>
                <a class="drawer-link" href="/contact/" on:click=move |_| menu_open.set(false)>"Contact"</a>
                <div class="drawer-foot">
                    <a class="btn btn-ghost btn-sm" href=GITHUB_ORG target="_blank" rel="noopener noreferrer">
                        <Icon name="github" />"GitHub"
                    </a>
                    <a class="btn btn-primary btn-sm" href=OPEN_COLLECTIVE target="_blank" rel="noopener noreferrer">
                        <Icon name="heart" />"Open Collective"
                    </a>
                </div>
            </div>
        </header>
    }
}

/// Cycles light → dark, persisting the choice. The initial paint is handled by
/// the inline bootstrap script in the document head, so this never causes a
/// flash or a hydration mismatch.
fn toggle_theme() {
    #[cfg(feature = "hydrate")]
    {
        let Some(window) = web_sys::window() else { return };
        let Some(document) = window.document() else { return };
        let Some(root) = document.document_element() else { return };
        let current = root.get_attribute("data-theme").unwrap_or_else(|| "light".into());
        let next = if current == "dark" { "light" } else { "dark" };
        let _ = root.set_attribute("data-theme", next);
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.set_item("apich-theme", next);
        }
        // Keep the browser UI (address bar / form controls) in sync.
        if let Ok(Some(meta)) = document.query_selector("meta[name=\"theme-color\"]") {
            let color = if next == "dark" { "#05070f" } else { "#f3f6fc" };
            let _ = meta.set_attribute("content", color);
        }
        sync_theme_button(&document);
    }
}

/// Describes what the toggle will do next, rather than the state it is in.
#[cfg(feature = "hydrate")]
fn sync_theme_button(document: &web_sys::Document) {
    let Some(root) = document.document_element() else { return };
    let dark = root.get_attribute("data-theme").as_deref() == Some("dark");
    let label = if dark { "Switch to light theme" } else { "Switch to dark theme" };
    if let Ok(Some(btn)) = document.query_selector(".theme-toggle") {
        let _ = btn.set_attribute("aria-label", label);
        let _ = btn.set_attribute("title", label);
        let _ = btn.set_attribute("aria-pressed", if dark { "true" } else { "false" });
    }
}

/// Wires click delegation, history, scroll restoration, focus management, head
/// updates and the small interaction extras (copy buttons, card sheen).
#[cfg(feature = "hydrate")]
pub fn install_client_router(current: RwSignal<Route>) {
    use wasm_bindgen::prelude::*;

    let Some(window) = web_sys::window() else { return };
    let Some(document) = window.document() else { return };

    // ── Click delegation ────────────────────────────────────────────────
    let click_current = current;
    let on_click = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |ev: web_sys::MouseEvent| {
        if ev.default_prevented()
            || ev.button() != 0
            || ev.meta_key()
            || ev.ctrl_key()
            || ev.shift_key()
            || ev.alt_key()
        {
            return;
        }
        let Some(target) = ev.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else {
            return;
        };

        // Copy-to-clipboard buttons.
        if let Ok(Some(btn)) = target.closest("[data-copy]") {
            if let Some(text) = btn.get_attribute("data-copy") {
                copy_to_clipboard(&btn, &text);
                ev.prevent_default();
                return;
            }
        }

        let Ok(Some(anchor)) = target.closest("a[href]") else { return };
        if let Some(t) = anchor.get_attribute("target") {
            if t != "_self" {
                return;
            }
        }
        if anchor.has_attribute("download") {
            return;
        }
        let Some(href) = anchor.get_attribute("href") else { return };
        if href.starts_with('#') || href.starts_with("mailto:") || href.starts_with("tel:") {
            return;
        }
        let path = if let Some(rest) = href.strip_prefix("https://apich.org") {
            rest.to_string()
        } else if href.starts_with('/') {
            href.clone()
        } else {
            return;
        };

        // Only take over navigation for routes this build knows about; anything
        // else gets a normal request so the server's 404 page is used.
        if Route::from_path(&path) == Route::NotFound {
            return;
        }
        ev.prevent_default();
        push_route(click_current, &path);
    });
    let _ = document.add_event_listener_with_callback("click", on_click.as_ref().unchecked_ref());
    on_click.forget();

    // ── Back / forward ─────────────────────────────────────────────────
    let pop_current = current;
    let on_pop = Closure::<dyn FnMut(web_sys::Event)>::new(move |_ev: web_sys::Event| {
        let path = crate::client_path();
        pop_current.set(Route::from_path(&path));
        after_navigation(false);
    });
    let _ = window.add_event_listener_with_callback("popstate", on_pop.as_ref().unchecked_ref());
    on_pop.forget();

    // ── Prefetch on hover ──────────────────────────────────────────────
    let on_over = Closure::<dyn FnMut(web_sys::Event)>::new(move |ev: web_sys::Event| {
        let Some(target) = ev.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else {
            return;
        };
        let Ok(Some(anchor)) = target.closest("a[href^=\"/\"]") else { return };
        if let Some(href) = anchor.get_attribute("href") {
            if Route::from_path(&href) != Route::NotFound {
                prefetch(&href);
            }
        }
    });
    let _ = document.add_event_listener_with_callback("mouseover", on_over.as_ref().unchecked_ref());
    on_over.forget();

    // ── Navigation bar elevation ───────────────────────────────────────
    let nav = document.get_element_by_id("site-nav");
    let to_top = document.get_element_by_id("to-top");
    let on_scroll = Closure::<dyn FnMut(web_sys::Event)>::new(move |_ev: web_sys::Event| {
        let y = web_sys::window()
            .and_then(|w| w.scroll_y().ok())
            .unwrap_or(0.0);
        if let Some(nav) = &nav {
            let list = nav.class_list();
            if y > 8.0 {
                let _ = list.add_1("scrolled");
            } else {
                let _ = list.remove_1("scrolled");
            }
        }
        if let Some(top) = &to_top {
            let list = top.class_list();
            if y > 900.0 {
                let _ = list.add_1("visible");
            } else {
                let _ = list.remove_1("visible");
            }
        }
    });
    let _ = window.add_event_listener_with_callback("scroll", on_scroll.as_ref().unchecked_ref());
    on_scroll.forget();

    // ── Pointer-tracked sheen on cards ─────────────────────────────────
    let on_move = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |ev: web_sys::MouseEvent| {
        let Some(target) = ev.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else {
            return;
        };
        let Ok(Some(card)) = target.closest(".spot") else { return };
        let rect = card.get_bounding_client_rect();
        if rect.width() == 0.0 {
            return;
        }
        let x = ((ev.client_x() as f64 - rect.left()) / rect.width() * 100.0) as i32;
        let y = ((ev.client_y() as f64 - rect.top()) / rect.height() * 100.0) as i32;
        let _ = card.set_attribute("style", &format!("--mx:{x}%;--my:{y}%"));
    });
    let _ = document.add_event_listener_with_callback("mousemove", on_move.as_ref().unchecked_ref());
    on_move.forget();

    // ── Escape closes the mobile drawer ────────────────────────────────
    let on_key = Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(move |ev: web_sys::KeyboardEvent| {
        if ev.key() != "Escape" {
            return;
        }
        let Some(document) = web_sys::window().and_then(|w| w.document()) else { return };
        let Some(drawer) = document.get_element_by_id("nav-drawer") else { return };
        if !drawer.class_list().contains("open") {
            return;
        }
        if let Ok(Some(burger)) = document.query_selector(".nav-burger") {
            if let Some(btn) = burger.dyn_into::<web_sys::HtmlElement>().ok() {
                let _ = btn.click();
                let _ = btn.focus();
            }
        }
    });
    let _ = document.add_event_listener_with_callback("keydown", on_key.as_ref().unchecked_ref());
    on_key.forget();

    sync_theme_button(&document);
    after_navigation(false);
}

#[cfg(feature = "hydrate")]
fn copy_to_clipboard(btn: &web_sys::Element, text: &str) {
    use wasm_bindgen::prelude::Closure;

    let Some(window) = web_sys::window() else { return };
    let promise = window.navigator().clipboard().write_text(text);

    let btn = btn.clone();
    let ok = Closure::once(move |_: wasm_bindgen::JsValue| {
        let _ = btn.class_list().add_1("done");
        let previous = btn
            .query_selector("span")
            .ok()
            .flatten()
            .and_then(|label| label.text_content())
            .unwrap_or_else(|| "Copy".to_string());
        if let Ok(Some(label)) = btn.query_selector("span") {
            label.set_text_content(Some("Copied"));
        }
        let btn2 = btn.clone();
        let restore = Closure::once(move || {
            let _ = btn2.class_list().remove_1("done");
            if let Ok(Some(label)) = btn2.query_selector("span") {
                label.set_text_content(Some(&previous));
            }
        });
        if let Some(win) = web_sys::window() {
            let _ = win.set_timeout_with_callback_and_timeout_and_arguments_0(
                restore.as_ref().unchecked_ref(),
                1600,
            );
        }
        restore.forget();
    });
    let _ = promise.then(&ok);
    ok.forget();
}

#[cfg(feature = "hydrate")]
fn prefetch(href: &str) {
    let Some(document) = web_sys::window().and_then(|w| w.document()) else { return };
    let Ok(Some(head)) = document.query_selector("head") else { return };
    if document
        .query_selector(&format!("link[rel=\"prefetch\"][href=\"{href}\"]"))
        .ok()
        .flatten()
        .is_some()
    {
        return;
    }
    if let Ok(link) = document.create_element("link") {
        let _ = link.set_attribute("rel", "prefetch");
        let _ = link.set_attribute("href", href);
        let _ = link.set_attribute("as", "document");
        let _ = head.append_child(&link);
    }
}

/// Navigates to `path`, pushing a history entry and re-rendering.
#[cfg(feature = "hydrate")]
pub fn push_route(current: RwSignal<Route>, path: &str) {
    use wasm_bindgen::JsValue;
    let Some(window) = web_sys::window() else { return };
    if let Ok(history) = window.history() {
        let _ = history.push_state_with_url(&JsValue::NULL, "", Some(path));
    }
    current.set(Route::from_path(path));
    after_navigation(true);
}

/// Scroll, focus and head bookkeeping that runs after every navigation.
#[cfg(feature = "hydrate")]
fn after_navigation(scroll: bool) {
    let Some(window) = web_sys::window() else { return };
    let Some(document) = window.document() else { return };

    if scroll {
        let hash = window.location().hash().unwrap_or_default();
        if hash.len() > 1 {
            if let Ok(Some(el)) = document.query_selector(&hash) {
                let _ = el.scroll_into_view();
            }
        } else {
            window.scroll_to_with_x_and_y(0.0, 0.0);
        }
        // Replay the page-enter animation.
        if let Some(main) = document.get_element_by_id("main") {
            let list = main.class_list();
            let _ = list.remove_1("page-enter");
            let _ = list.add_1("page-enter");
        }
        // Move focus to the main landmark for screen-reader and keyboard users.
        if let Some(main) = document.get_element_by_id("main") {
            if let Some(el) = main.dyn_into::<web_sys::HtmlElement>().ok() {
                let _ = el.focus();
            }
        }
    }

    crate::meta::apply_client(&document, Route::from_path(&crate::client_path()));
}
