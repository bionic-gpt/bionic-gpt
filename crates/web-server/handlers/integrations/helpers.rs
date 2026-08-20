use db::Json;

/// Parses an OpenAPI specification from JSON or YAML string.
///
/// This function will:
/// 1. Parse the provided string using the oas3 crate
/// 2. Return the parsed specification or an error
pub fn parse_openapi_spec(spec_text: &str) -> Result<Json<oas3::OpenApiV3Spec>, String> {
    let spec = oas3::from_json(spec_text)
        .or_else(|_| oas3::from_yaml(spec_text).map_err(|error| error.to_string()))
        .map_err(|error| format!("Invalid OpenAPI JSON or YAML: {}", error))?;

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

pub fn parse_openapi_spec_json_value(spec_text: &str) -> Result<serde_json::Value, String> {
    let Json(spec) = parse_openapi_spec(spec_text)?;
    serde_json::to_value(spec).map_err(|error| format!("Invalid OpenAPI JSON or YAML: {}", error))
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

    #[test]
    fn test_parse_openapi_spec_valid_yaml() {
        let spec_yaml = r#"
openapi: 3.0.0
info:
  title: Test API
  version: "1.0"
paths:
  /users:
    get:
      operationId: listUsers
"#;

        let result = parse_openapi_spec(spec_yaml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_openapi_spec_json_value_normalizes_yaml() {
        let spec_yaml = r#"
openapi: 3.0.0
info:
  title: Test API
  version: "1.0"
paths:
  /users:
    get:
      operationId: listUsers
"#;

        let result = parse_openapi_spec_json_value(spec_yaml).unwrap();
        assert_eq!(result["info"]["title"], "Test API");
        assert_eq!(result["paths"]["/users"]["get"]["operationId"], "listUsers");
    }

    #[test]
    fn test_parse_openapi_spec_invalid_text() {
        let result = parse_openapi_spec("not: [valid");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid OpenAPI JSON or YAML"));
    }

    #[test]
    fn test_parse_openapi_spec_yaml_missing_operation_id() {
        let spec_yaml = r#"
openapi: 3.0.0
info:
  title: Test API
  version: "1.0"
paths:
  /users:
    get:
      summary: list
"#;

        let result = parse_openapi_spec(spec_yaml);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("operationId"));
    }

    #[test]
    fn test_parse_eval_web_search_openapi_spec() {
        let spec_yaml = include_str!(
            "../../../../infra-as-code/eval-mocks/openapi/specs/web-search.openapi.yaml"
        );

        let Json(parsed) = parse_openapi_spec(spec_yaml).unwrap();
        let operation_ids = parsed
            .operations()
            .filter_map(|(_, _, operation)| operation.operation_id.as_deref())
            .collect::<Vec<_>>();

        assert_eq!(operation_ids, vec!["searchWeb"]);
    }

    #[test]
    fn test_parse_eval_email_openapi_spec() {
        let spec_yaml = include_str!(
            "../../../../infra-as-code/eval-mocks/openapi/specs/email-integration.openapi.yaml"
        );

        let Json(parsed) = parse_openapi_spec(spec_yaml).unwrap();
        let operation_ids = parsed
            .operations()
            .filter_map(|(_, _, operation)| operation.operation_id.as_deref())
            .collect::<Vec<_>>();

        assert!(operation_ids.contains(&"listEmails"));
        assert!(operation_ids.contains(&"getEmail"));
        assert!(operation_ids.contains(&"createDraft"));
        assert!(operation_ids.contains(&"sendDraft"));
    }
}
