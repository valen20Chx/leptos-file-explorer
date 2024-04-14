use crate::{
    error_template::{AppError, ErrorTemplate},
    list_dir::ListView,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

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
                    <Route path="/" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[derive(Params, PartialEq)]
struct SearchParams {
    path: Option<String>,
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let query = use_query::<SearchParams>();

    let path = move || {
        query.with(|params| {
            params
                .as_ref()
                .map(|params| params.path.clone())
                .unwrap_or_default()
        })
    };

    view! {
        <ListView path={path()}/>
    }
}
