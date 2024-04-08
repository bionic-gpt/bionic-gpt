pub mod error_template;
#[cfg(feature = "ssr")]
pub mod fileserv;
pub mod config;
#[cfg(feature = "ssr")]
pub mod app;
pub mod pages;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::leptos_dom::HydrationCtx::stop_hydrating();
}
