use crate::error_template::{AppError, ErrorTemplate};
use crate::pages::api_keys;
use crate::pages::console;
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
        <Stylesheet id="leptos" href="/pkg/web-server.css"/>

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
                <div class="drawer lg:drawer-open">
                    <input id="my-drawer-2" type="checkbox" class="drawer-toggle" />
                    <div class="drawer-content">
                        <Routes>
                            <Route path="api_keys" view=api_keys::IndexPage/>
                            <Route path="console" view=console::IndexPage/>
                        </Routes>
                    </div>
                    <div class="drawer-side">
                        <label for="my-drawer-2" aria-label="close sidebar" class="drawer-overlay"></label>
                        <ul class="menu p-4 w-80 min-h-full bg-base-200 text-base-content">
                            <li><a href="/console"><img src="accounts.svg" width="16" height="16" /> The Console</a></li>
                            <li><a href="/api_keys">The Keys</a></li>
                        </ul>
                    </div>
                </div>
            </main>
        </Router>
    }
}
