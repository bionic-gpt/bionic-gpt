// SSR Only
#[cfg(feature = "ssr")]
pub mod app;
#[cfg(feature = "ssr")]
pub mod error_template;
#[cfg(feature = "ssr")]
pub mod fileserv;
#[cfg(feature = "ssr")]
pub mod pages;
#[cfg(feature = "ssr")]
pub mod ssr;

// Islands - get compiled to the server and the front end
pub mod islands;

// Front end only
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::leptos_dom::HydrationCtx::stop_hydrating();
}
