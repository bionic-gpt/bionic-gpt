use oas3::spec::{ObjectOrReference, SecurityScheme};

/// Check if an integration has OAuth2 support
pub fn has_oauth2_support(integration: &db::queries::integrations::Integration) -> bool {
    if let Some(definition) = &integration.definition {
        if let Ok(spec) = oas3::from_json(definition.to_string()) {
            return has_oauth2_security(&spec);
        }
    }
    false
}

/// Check if an OpenAPI spec has OAuth2 security schemes
pub fn has_oauth2_security(spec: &oas3::OpenApiV3Spec) -> bool {
    if let Some(components) = &spec.components {
        for scheme_ref in components.security_schemes.values() {
            if let Ok(scheme) = scheme_ref.resolve(spec) {
                if matches!(scheme, SecurityScheme::OAuth2 { .. }) {
                    return true;
                }
            }
        }
    }
    false
}

/// OAuth2 configuration extracted from a security scheme
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OAuth2Config {
    pub authorization_url: String,
    pub token_url: String,
    pub scopes: Vec<String>,
}

/// Retrieve OAuth2 configuration from an OpenAPI spec
pub fn get_oauth2_config(spec: &oas3::OpenApiV3Spec) -> Option<OAuth2Config> {
    let components = spec.components.as_ref()?;
    for scheme_ref in components.security_schemes.values() {
        if let Ok(SecurityScheme::OAuth2 { flows, .. }) = scheme_ref.resolve(spec) {
            if let Some(flow) = flows.authorization_code {
                let scopes = flow.scopes.keys().cloned().collect();
                return Some(OAuth2Config {
                    authorization_url: flow.authorization_url.to_string(),
                    token_url: flow.token_url.to_string(),
                    scopes,
                });
            }
        }
    }
    None
}
