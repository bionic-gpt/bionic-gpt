use std::fs;
use std::io::{Cursor, Write};
use std::path::Path;

use ssg_whiz::SitePage;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

use crate::pages;

const EVAL_SPEC_SOURCE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../infra-as-code/eval-mocks/openapi/specs"
);
const EVAL_SPEC_OUTPUT_DIR: &str = "dist/architect-course/enterprise-evals";
const EVAL_SPEC_FILES: [&str; 2] = ["email-integration.openapi.yaml", "web-search.openapi.yaml"];
const DOCUMENT_VALIDATION_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/content/architect-course/enterprise-evals/document-validation"
);
const DOCUMENT_VALIDATION_OUTPUT_DIR: &str =
    "dist/architect-course/enterprise-evals/document-validation";
const POSTGRES_MCP_SPEC_SOURCE: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/../postgres-mcp/postgres.json");
const POSTGRES_MCP_SPEC_OUTPUT: &str = "postgres.openapi.json";
const DASHBOARD_SKILL_SOURCE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/content/architect-course/enterprise-evals/dashboard-builder/package"
);
const DASHBOARD_SALES_CSV_SOURCE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/content/architect-course/enterprise-evals/dashboard-builder/quarterly-sales.csv"
);
const DASHBOARD_SKILL_OUTPUT_DIR: &str = "dist/architect-course/enterprise-evals/dashboard-builder";
const DASHBOARD_SKILL_ZIP: &str =
    "dist/architect-course/enterprise-evals/dashboard-builder/dashboard-builder.zip";

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
    copy_document_validation_assets();
    copy_dashboard_skill_package();

    let mut pages = Vec::new();
    pages.extend(generate_marketing().await);
    pages.extend(generate_product().await);
    pages.extend(generate_solutions().await);
    pages
}

fn copy_document_validation_assets() {
    let source_dir = Path::new(DOCUMENT_VALIDATION_DIR);
    let output_dir = Path::new(DOCUMENT_VALIDATION_OUTPUT_DIR);
    fs::create_dir_all(output_dir).expect("failed to create document validation output directory");
    for file_name in [
        "vendor-agreement.docx",
        "vendor-service-schedule.xlsx",
        "procurement-security-rubric.pdf",
    ] {
        let source = source_dir.join(file_name);
        let destination = output_dir.join(file_name);
        fs::copy(&source, &destination).unwrap_or_else(|error| {
            panic!(
                "failed to copy document validation asset from {} to {}: {error}",
                source.display(),
                destination.display()
            )
        });
    }
}

fn copy_dashboard_skill_package() {
    let source_dir = Path::new(DASHBOARD_SKILL_SOURCE_DIR);
    let output_dir = Path::new(DASHBOARD_SKILL_OUTPUT_DIR);
    fs::create_dir_all(output_dir.join("bin"))
        .expect("failed to create dashboard skill output directory");
    fs::copy(
        DASHBOARD_SALES_CSV_SOURCE,
        output_dir.join("quarterly-sales.csv"),
    )
    .expect("failed to copy quarterly sales CSV");

    let files = [
        ("SKILL.md", "SKILL.md"),
        ("bin/render_dashboard.py", "bin/render_dashboard.py"),
    ];
    let mut archive = ZipWriter::new(Cursor::new(Vec::new()));
    let options = SimpleFileOptions::default();

    for (source_name, archive_name) in files {
        let source = source_dir.join(source_name);
        let contents = fs::read(&source).unwrap_or_else(|error| {
            panic!(
                "failed to read dashboard skill file {}: {error}",
                source.display()
            )
        });
        fs::write(output_dir.join(source_name), &contents).unwrap_or_else(|error| {
            panic!(
                "failed to copy dashboard skill file {}: {error}",
                source.display()
            )
        });
        archive
            .start_file(format!("dashboard-builder/{archive_name}"), options)
            .expect("failed to add dashboard skill file to archive");
        archive
            .write_all(&contents)
            .expect("failed to write dashboard skill file to archive");
    }

    let archive = archive
        .finish()
        .expect("failed to finish dashboard skill archive")
        .into_inner();
    if let Some(parent) = Path::new(DASHBOARD_SKILL_ZIP).parent() {
        fs::create_dir_all(parent).expect("failed to create dashboard skill archive directory");
    }
    fs::write(DASHBOARD_SKILL_ZIP, archive).expect("failed to write dashboard skill archive");
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
