//! Static MCP OpenAPI specifications bundled for the marketing site.

/// A bundled MCP specification.
#[derive(Debug, Clone, Copy)]
pub struct McpSpec {
    pub slug: &'static str,
    pub json: &'static str,
}

macro_rules! mcp_spec {
    ($slug:literal) => {
        McpSpec {
            slug: $slug,
            json: include_str!(concat!("../specs/", $slug, ".json")),
        }
    };
}

/// All bundled MCP specifications available to the marketing site.
static SPECS: &[McpSpec] = &[
    mcp_spec!("airtable"),
    mcp_spec!("apollo-io"),
    mcp_spec!("blockchain"),
    mcp_spec!("companiesHouse"),
    mcp_spec!("contacts-scraper"),
    mcp_spec!("dropbox"),
    mcp_spec!("github-advisories"),
    mcp_spec!("goatcounter"),
    mcp_spec!("google-calendar"),
    mcp_spec!("google-drive"),
    mcp_spec!("google_people"),
    mcp_spec!("linkedin"),
    mcp_spec!("mysql"),
    mcp_spec!("openfda"),
    mcp_spec!("postgres"),
    mcp_spec!("reddit"),
    mcp_spec!("sap"),
    mcp_spec!("ukpolicedata"),
];

/// Returns every bundled MCP specification.
pub fn all_specs() -> &'static [McpSpec] {
    SPECS
}

/// Finds a specification by its slug.
#[allow(dead_code)]
pub fn find_spec(slug: &str) -> Option<&'static McpSpec> {
    SPECS
        .iter()
        .find(|spec| spec.slug.eq_ignore_ascii_case(slug))
}
