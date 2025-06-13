use db::Json;

/// Parses an OpenAPI specification from JSON string.
///
/// This function will:
/// 1. Parse the provided JSON string using the oas3 crate
/// 2. Return the parsed specification as JSON or an error
pub fn parse_openapi_spec(spec_json: &str) -> Result<Json<oas3::OpenApiV3Spec>, String> {
    match oas3::from_json(spec_json) {
        Ok(spec) => {
            let mut missing_ops = Vec::new();
            for (path, method, operation) in spec.operations() {
                if operation.operation_id.is_none() {
                    missing_ops.push(format!("{} {}", method, path));
                }
            }

            if !missing_ops.is_empty() {
                return Err(format!(
                    "Every operation must have an operationId. Missing for: {}",
                    missing_ops.join(", ")
                ));
            }

            Ok(Json(spec))
        }
        Err(e) => Err(format!("Invalid OpenAPI JSON: {}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_openapi_spec_missing_operation_id() {
        let spec_json = json!({
            "openapi": "3.0.0",
            "info": {"title": "Test API", "version": "1.0"},
            "paths": {"/users": {"get": {"summary": "list"}}}
        })
        .to_string();

        let result = parse_openapi_spec(&spec_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_openapi_spec_valid() {
        let spec_json = json!({
            "openapi": "3.0.0",
            "info": {"title": "Test API", "version": "1.0"},
            "paths": {"/users": {"get": {"operationId": "listUsers"}}}
        })
        .to_string();

        let result = parse_openapi_spec(&spec_json);
        assert!(result.is_ok());
    }
}
