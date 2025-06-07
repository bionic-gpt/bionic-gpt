/// Check if an integration has OAuth2 support
pub fn has_oauth2_support(integration: &db::queries::integrations::Integration) -> bool {
    if let Some(definition) = &integration.definition {
        if let Ok(spec) = oas3::from_json(definition.to_string()) {
            if let Some(components) = &spec.components {
                for security_scheme in components.security_schemes.values() {
                    if let Ok(scheme_value) = serde_json::to_value(security_scheme) {
                        if let Some(scheme_type) = scheme_value.get("type").and_then(|t| t.as_str())
                        {
                            if scheme_type == "oauth2" {
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

/// Check if an OpenAPI spec has OAuth2 security schemes
pub fn has_oauth2_security(spec: &oas3::OpenApiV3Spec) -> bool {
    if let Some(components) = &spec.components {
        for security_scheme in components.security_schemes.values() {
            if let Ok(scheme_value) = serde_json::to_value(security_scheme) {
                if let Some(scheme_type) = scheme_value.get("type").and_then(|t| t.as_str()) {
                    if scheme_type == "oauth2" {
                        return true;
                    }
                }
            }
        }
    }
    false
}
