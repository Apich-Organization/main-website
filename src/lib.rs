//! The Apich Organization website.
//!
//! One codebase, two builds:
//!
//! * `--features ssr` — the `ssg` binary renders every route to a complete HTML
//!   document at build time (real markup, real meta tags, no client needed).
//! * `--features hydrate` — the WASM bundle takes over that markup and turns
//!   the site into a client-side router with instant navigation.
//!
//! Because both builds render the *same* component tree from the same route
//! value, hydration is exact and the site works with JavaScript disabled.

pub mod components;
pub mod content;
pub mod meta;
pub mod pages;
pub mod route;
pub mod site;

use leptos::prelude::*;
use route::Route;

use crate::components::{footer::Footer, nav::Nav};

/// Application shell. `initial` is the route resolved from the request path on
/// the server, and from `window.location` on the client — the same value, so
/// the first client render matches the server HTML byte for byte.
#[component]
pub fn App(initial: Route) -> impl IntoView {
    let current = RwSignal::new(initial);
    provide_context(current);

    #[cfg(feature = "hydrate")]
    crate::components::nav::install_client_router(current);

    view! {
        <a class="skip-link" href="#main">"Skip to main content"</a>
        <Nav />
        <main id="main" tabindex="-1">
            {move || pages::page_view(current.get())}
        </main>
        <Footer />
        <a class="to-top" id="to-top" href="#top" aria-label="Back to top">
            <span aria-hidden="true">"↑"</span>
        </a>
    }
}

/// WASM entry point. `#[wasm_bindgen(start)]` runs automatically when the
/// module is instantiated by the loader in `index.html`.
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    let initial = route::Route::from_path(&client_path());
    leptos::mount::hydrate_body(move || view! { <App initial=initial /> });
    // Signals to tooling (and to us, in devtools) that the bundle took over.
    if let Some(root) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.document_element())
    {
        let _ = root.set_attribute("data-hydrated", "true");
    }
}

/// Current path as seen by the browser.
#[cfg(feature = "hydrate")]
pub fn client_path() -> String {
    web_sys::window()
        .and_then(|w| w.location().pathname().ok())
        .unwrap_or_else(|| "/".to_string())
}
