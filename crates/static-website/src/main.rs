use std::{fs, net::SocketAddr, path::Path};

use axum::Router;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

use static_website::{
    architect_course_summary, blog_summary, components::navigation::Section, docs_summary,
    generator, pages_summary,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    fs::create_dir_all("dist").expect("Couldn't create dist folder");
    generator::generate_marketing().await;
    generator::generate_product().await;
    generator::generate_solutions().await;
    generator::generate_docs(docs_summary::summary(), Section::Docs);
    generator::generate_docs(
        architect_course_summary::summary(),
        Section::ArchitectCourse,
    );
    generator::generate(blog_summary::summary());
    generator::generate_pages(pages_summary::summary()).await;
    generator::generate_blog_list(blog_summary::summary()).await;
    let src = Path::new("assets");
    let dst = Path::new("dist");
    generator::copy_folder(src, dst).expect("Couldn't copy folder");

    if std::env::var("DO_NOT_RUN_SERVER").is_err() {
        let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

        // build our application with a route
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
