use ssg_whiz::SitePage;

use crate::pages;

fn output_page(path: &str, html: String) -> SitePage {
    SitePage {
        path: path.to_string(),
        html,
    }
}

pub async fn generate_static_pages() -> Vec<SitePage> {
    let mut pages = Vec::new();
    pages.extend(generate_marketing().await);
    pages.extend(generate_mcp_servers());
    pages
}

pub async fn generate_marketing() -> Vec<SitePage> {
    vec![
        output_page("", pages::home::home_page()),
        output_page("pricing", pages::pricing::pricing_page()),
        output_page("contact", pages::contact::contact_page()),
        output_page("enterprise", pages::enterprise::enterprise_page()),
    ]
}

pub fn generate_mcp_servers() -> Vec<SitePage> {
    let integrations = pages::mcp_servers::load_integration_specs();
    let mut pages_out = Vec::new();

    pages_out.push(output_page(
        "mcp-servers",
        pages::mcp_servers::index_page(&integrations),
    ));

    for integration in integrations {
        pages_out.push(output_page(
            &integration.folder_name(),
            pages::mcp_servers::detail_page(&integration),
        ));
    }

    pages_out
}
