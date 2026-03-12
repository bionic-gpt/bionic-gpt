use ssg_whiz::SitePage;

use crate::pages;

fn output_page(path: &str, html: String) -> SitePage {
    SitePage {
        path: path.to_string(),
        html,
    }
}

pub async fn generate_product() -> Vec<SitePage> {
    vec![
        output_page("product/assistants", pages::product::assistants::page()),
        output_page("product/automations", pages::product::automations::page()),
        output_page("product/chat", pages::product::chat::page()),
        output_page("product/developers", pages::product::developers::page()),
        output_page("product/integrations", pages::product::integrations::page()),
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
    let mut pages = Vec::new();
    pages.extend(generate_marketing().await);
    pages.extend(generate_product().await);
    pages.extend(generate_solutions().await);
    pages
}
