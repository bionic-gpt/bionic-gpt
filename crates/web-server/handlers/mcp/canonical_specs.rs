//! Bundled canonical MCP OpenAPI specifications for well-known slugs.

/// Canonical specification metadata.
pub struct CanonicalSpec {
    pub slug: &'static str,
    pub json: &'static str,
}

macro_rules! canonical_spec {
    ($slug:literal) => {
        CanonicalSpec {
            slug: $slug,
            json: include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../deploy-mcp/specs/",
                $slug,
                ".json"
            )),
        }
    };
}

static SPECS: &[CanonicalSpec] = &[
    canonical_spec!("airtable"),
    canonical_spec!("apollo-io"),
    canonical_spec!("blockchain"),
    canonical_spec!("companiesHouse"),
    canonical_spec!("contacts-scraper"),
    canonical_spec!("dropbox"),
    canonical_spec!("github-advisories"),
    canonical_spec!("goatcounter"),
    canonical_spec!("google-calendar"),
    canonical_spec!("google-drive"),
    canonical_spec!("google_people"),
    canonical_spec!("linkedin"),
    canonical_spec!("mysql"),
    canonical_spec!("openfda"),
    canonical_spec!("postgres"),
    canonical_spec!("reddit"),
    canonical_spec!("sap"),
    canonical_spec!("ukpolicedata"),
];

pub fn find_spec(slug: &str) -> Option<&'static CanonicalSpec> {
    SPECS
        .iter()
        .find(|spec| spec.slug.eq_ignore_ascii_case(slug))
}
