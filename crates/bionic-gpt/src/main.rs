use std::net::SocketAddr;

use ssg_whiz::{ScriptAsset, Section, SiteAssets, SiteBuilder, SiteConfig};

use bionic_gpt::{
    architect_course_summary, blog_summary, docs_summary, generator, pages_summary,
    site_header::site_header,
    ui_links::{footer_links, navigation_links},
};
use ssg_whiz::summaries::DocumentSite;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let docs_summary = docs_summary::summary();
    let architect_summary = architect_course_summary::summary();
    let blog_summary = blog_summary::summary();
    let pages_summary = pages_summary::summary();
    let tailwind_stylesheet =
        std::env::var("TAILWIND_STYLESHEET").unwrap_or_else(|_| "/tailwind.css".to_string());

    let run_server = std::env::var("DO_NOT_RUN_SERVER").is_err();
    let config = SiteConfig {
        dist_dir: "dist".into(),
        run_server,
        addr: SocketAddr::from(([0, 0, 0, 0], 8080)),
        live_reload: true,
        navigation_links: navigation_links(),
        footer_links: footer_links(),
        site_meta: bionic_gpt::ui_links::site_meta(),
        site_header: Some(site_header),
        site_assets: SiteAssets {
            stylesheets: vec![tailwind_stylesheet, "https://cdn.jsdelivr.net/npm/daisyui@5".into()],
            head_scripts: vec![
                ScriptAsset {
                    src: "/goat-counter.js".to_string(),
                    script_type: None,
                    async_load: true,
                    integrity: None,
                    data_goatcounter: Some(
                        "https://bionicgpt.goatcounter.com/count".to_string(),
                    ),
                },
                ScriptAsset {
                    src: "/copy-paste.js".to_string(),
                    script_type: None,
                    async_load: true,
                    integrity: None,
                    data_goatcounter: None,
                },
                ScriptAsset {
                    src: "https://cdn.jsdelivr.net/npm/@justinribeiro/lite-youtube@1/lite-youtube.min.js"
                        .to_string(),
                    script_type: Some("module".to_string()),
                    async_load: false,
                    integrity: None,
                    data_goatcounter: None,
                },
            ],
            body_scripts: vec![ScriptAsset {
                src: "https://instant.page/5.2.0".to_string(),
                script_type: Some("module".to_string()),
                async_load: false,
                integrity: Some(
                    "sha384-jnZyxPjiipYXnSU0ygqeac2q7CVYMbh84q0uHVRRxEtvFPiQYbXWUorga2aqZJ0z"
                        .to_string(),
                ),
                data_goatcounter: None,
            }],
            head_inline_scripts: vec![],
            body_inline_scripts: vec![],
        },
    };

    SiteBuilder::new(config)
        .blog(blog_summary)
        .pages(pages_summary)
        .documents(vec![
            DocumentSite {
                summary: docs_summary,
                section: Section::Docs,
            },
            DocumentSite {
                summary: architect_summary,
                section: Section::ArchitectCourse,
            },
        ])
        .static_pages(generator::generate_static_pages)
        .build()
        .await
        .expect("Failed to generate website");
}
