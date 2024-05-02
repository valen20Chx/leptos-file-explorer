use crate::{
    error_template::{AppError, ErrorTemplate},
    list_dir::ExploreView,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

const EXPLORE_PATH_PREFIX: &str = "/explore";

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/file-exp.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path=format!("{}/*any", EXPLORE_PATH_PREFIX) view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let location = use_location();

    let path = move || {
        let temp_path = location.pathname.get().clone();
        let (_, path) = temp_path.split_at(EXPLORE_PATH_PREFIX.len());
        path.to_string()
    };

    view! {
        <ExploreView path=path()/>
    }
}
