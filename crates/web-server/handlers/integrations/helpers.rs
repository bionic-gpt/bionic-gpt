use db::Json;

/// Parses an OpenAPI specification from JSON string.
///
/// This function will:
/// 1. Parse the provided JSON string using the oas3 crate
/// 2. Return the parsed specification as JSON or an error
pub fn parse_openapi_spec(spec_json: &str) -> Result<Json<oas3::OpenApiV3Spec>, String> {
    match oas3::from_json(spec_json) {
        Ok(spec) => Ok(Json(spec)),
        Err(e) => Err(format!("Invalid OpenAPI JSON: {}", e)),
    }
}
