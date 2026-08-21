use std::fs;
use std::path::Path;

use ssg_whiz::SitePage;

use crate::pages;

const EVAL_SPEC_SOURCE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../infra-as-code/eval-mocks/openapi/specs"
);
const EVAL_SPEC_OUTPUT_DIR: &str = "dist/architect-course/enterprise-evals";
const EVAL_SPEC_FILES: [&str; 2] = ["email-integration.openapi.yaml", "web-search.openapi.yaml"];
const POSTGRES_MCP_SPEC_SOURCE: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/../postgres-mcp/postgres.json");
const POSTGRES_MCP_SPEC_OUTPUT: &str = "postgres.openapi.json";

fn output_page(path: &str, html: String) -> SitePage {
    SitePage {
        path: path.to_string(),
        html,
    }
}

pub async fn generate_product() -> Vec<SitePage> {
    vec![
        output_page("product/assistants", pages::product::assistants::page()),
        output_page("product/chat", pages::product::chat::page()),
        output_page("product/datasets", pages::product::datasets::page()),
        output_page("product/developers", pages::product::developers::page()),
        output_page("product/integrations", pages::product::integrations::page()),
        output_page("product/skills", pages::product::skills::page()),
    ]
}

pub async fn generate_solutions() -> Vec<SitePage> {
    vec![
        output_page("solutions/education", pages::solutions::education::page()),
        output_page("solutions/support", pages::solutions::support::page()),
    ]
}

pub async fn generate_marketing() -> Vec<SitePage> {
    vec![
        output_page("pricing", pages::pricing::pricing()),
        output_page("partners", pages::partners::partners_page()),
        output_page("contact", pages::contact::contact_page()),
        output_page("", pages::home::home_page()),
    ]
}

pub async fn generate_static_pages() -> Vec<SitePage> {
    copy_enterprise_eval_specs();

    let mut pages = Vec::new();
    pages.extend(generate_marketing().await);
    pages.extend(generate_product().await);
    pages.extend(generate_solutions().await);
    pages
}

fn copy_enterprise_eval_specs() {
    let output_dir = Path::new(EVAL_SPEC_OUTPUT_DIR);
    fs::create_dir_all(output_dir).expect("failed to create enterprise eval spec output directory");

    for file_name in EVAL_SPEC_FILES {
        let source = Path::new(EVAL_SPEC_SOURCE_DIR).join(file_name);
        let destination = output_dir.join(file_name);
        fs::copy(&source, &destination).unwrap_or_else(|error| {
            panic!(
                "failed to copy enterprise eval spec from {} to {}: {error}",
                source.display(),
                destination.display()
            )
        });
    }

    let postgres_destination = output_dir.join(POSTGRES_MCP_SPEC_OUTPUT);
    fs::copy(POSTGRES_MCP_SPEC_SOURCE, &postgres_destination).unwrap_or_else(|error| {
        panic!(
            "failed to copy Postgres MCP spec from {} to {}: {error}",
            POSTGRES_MCP_SPEC_SOURCE,
            postgres_destination.display()
        )
    });
}
