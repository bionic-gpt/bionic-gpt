pub mod blog_summary;
pub mod docs_summary;
pub mod generator;
pub mod layouts;
pub mod mcp_specs;
pub mod pages;
pub mod pages_summary;

pub use static_website::{components, markdown, render};

use axum::Router;
use std::{fs, net::SocketAddr, path::Path};
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

pub mod routes {
    pub const SIGN_IN_UP: &str = "https://app.deploy-mcp.com";

    pub mod blog {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/blog/")]
        pub struct Index {}
    }

    pub mod product {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/product/chat/")]
        pub struct Chat {}

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/product/assistants/")]
        pub struct Assistants {}

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/product/integrations/")]
        pub struct Integrations {}

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/product/automations/")]
        pub struct Automations {}

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/product/developers/")]
        pub struct Developers {}
    }

    pub mod solutions {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/solutions/education/")]
        pub struct Education {}

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/solutions/support/")]
        pub struct Support {}
    }

    pub mod marketing {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/")]
        pub struct Index {}

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/enterprise/")]
        pub struct Enterprise {}

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/terms/")]
        pub struct Terms {}

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/privacy/")]
        pub struct Privacy {}

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/pricing/")]
        pub struct Pricing {}

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/mcp-servers/")]
        pub struct McpServers {}

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/contact/")]
        pub struct Contact {}

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/services/")]
        pub struct ServicesPage {}
    }

    pub mod docs {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/docs/")]
        pub struct Index {}
    }

    pub mod mcp_servers {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/mcp-servers/{slug}/")]
        pub struct Detail {
            pub slug: String,
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    fs::create_dir_all("dist").expect("Couldn't create dist folder");
    generator::generate_marketing();
    generator::generate_mcp_servers();
    generator::generate_docs(docs_summary::summary());
    generator::generate_blog_posts(blog_summary::summary());
    generator::generate_pages(pages_summary::summary());

    let src = Path::new("assets");
    let dst = Path::new("dist");
    generator::copy_folder(src, dst).expect("Couldn't copy assets");

    if std::env::var("DO_NOT_RUN_SERVER").is_err() {
        let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

        let app = Router::new()
            .fallback_service(ServeDir::new("dist"))
            .layer(LiveReloadLayer::new());

        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        tracing::info!("listening on http://{}", &addr);
        axum::serve(listener, app.into_make_service())
            .await
            .unwrap();
    }
}
