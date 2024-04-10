use leptos::*;
use leptos_meta::*;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/web-ui.css"/>

        <main>
            <div class="drawer lg:drawer-open">
                <input id="my-drawer-2" type="checkbox" class="drawer-toggle" />
                <div class="drawer-content">
                    {children()}
                </div>
                <div class="drawer-side">
                    <label for="my-drawer-2" aria-label="close sidebar" class="drawer-overlay"></label>
                    <ul class="menu p-4 w-80 min-h-full bg-base-200 text-base-content">
                        <li><a href="/console"><img src="accounts.svg" width="16" height="16" /> The Console</a></li>
                        <li><a href="/">The Keys</a></li>
                    </ul>
                </div>
            </div>
        </main>
    }
}
