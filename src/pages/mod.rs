//! Page composition. Each route maps to exactly one component so that the
//! server render and the hydrated client render always agree.

pub mod blog;
pub mod contact;
pub mod home;
pub mod not_found;
pub mod project;
pub mod projects;
pub mod research;
pub mod security;
pub mod sponsor;

use leptos::prelude::*;

use crate::route::Route;

pub fn page_view(route: Route) -> AnyView {
    match route {
        Route::Home => view! { <home::HomePage /> }.into_any(),
        Route::Projects => view! { <projects::ProjectsPage /> }.into_any(),
        Route::Project(p) => view! { <project::ProjectPage project=p /> }.into_any(),
        Route::Research => view! { <research::ResearchPage /> }.into_any(),
        Route::Security => view! { <security::SecurityPage /> }.into_any(),
        Route::Blog => view! { <blog::BlogPage /> }.into_any(),
        Route::Post(slug) => view! { <blog::PostPage slug=slug /> }.into_any(),
        Route::Sponsor => view! { <sponsor::SponsorPage /> }.into_any(),
        Route::Contact => view! { <contact::ContactPage /> }.into_any(),
        Route::NotFound => view! { <not_found::NotFoundPage /> }.into_any(),
    }
}
