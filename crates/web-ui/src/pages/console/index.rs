use leptos::*;
use crate::islands::Counter;
pub use axum::{
    body::Body,
    extract::{Path, State},
    http::Request,
    response::{IntoResponse, Response, Html},
    routing::get,
    Router,
    Extension
};
use super::super::Layout;

pub async fn index(
    Extension(options): Extension<LeptosOptions>, 
    req: Request<Body>) 
    -> Response{
    let handler = leptos_axum::render_app_to_stream((options).clone(),
        || view! {
            <Layout>
                <IndexPage />
            </Layout>
        }
    );
    handler(req).await.into_response()
}

#[component]
pub fn IndexPage() -> impl IntoView {
    view! {
        <div class="navbar bg-base-100">
            <div class="flex-none">
            <button class="btn btn-square btn-ghost">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-5 h-5 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path></svg>
            </button>
            </div>
            <div class="flex-1">
                <a class="btn btn-ghost text-xl">Console</a>
            </div>
            <div class="flex-none">
            </div>
        </div>
        <div class="m-5 mb-0">
            <h1>"Welcome to Leptos!"</h1>
            <Counter/>
        </div>
    }
}