//! OpenApiTool - HTTP request execution tool for OpenAPI operations
//!
//! This module provides the OpenApiTool struct that executes HTTP requests
//! based on OpenAPI operation definitions.

use crate::token_providers::TokenProvider;
use crate::tool::ToolInterface;
use async_trait::async_trait;
use oas3::{
    self,
    spec::{ObjectOrReference, Operation, Parameter, ParameterIn},
};

use openai_api::BionicToolDefinition;
use reqwest::{Client, Method, StatusCode, Url};
use serde_json::Value;
use std::collections::HashSet;
use std::sync::{Arc, Mutex, OnceLock};

#[async_trait]
pub trait HttpClient: Send + Sync + 'static {
    async fn send(
        &self,
        method: Method,
        url: Url,
        headers: Vec<(String, String)>,
        body: Option<Value>,
    ) -> Result<HttpResponse, String>;
}

#[derive(Clone, Debug)]
pub struct HttpResponse {
    pub status: StatusCode,
    pub body: String,
}

#[derive(Clone)]
struct ReqwestHttpClient {
    client: Client,
}

static HTTP_CLIENT_OVERRIDE: OnceLock<Mutex<Option<Arc<dyn HttpClient>>>> = OnceLock::new();
static DEFAULT_HTTP_CLIENT: OnceLock<Arc<dyn HttpClient>> = OnceLock::new();

impl ReqwestHttpClient {
    fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

#[async_trait]
impl HttpClient for ReqwestHttpClient {
    async fn send(
        &self,
        method: Method,
        url: Url,
        headers: Vec<(String, String)>,
        body: Option<Value>,
    ) -> Result<HttpResponse, String> {
        let mut request = self.client.request(method, url);
        for (name, value) in headers {
            request = request.header(name, value);
        }
        if let Some(body) = body {
            request = request.json(&body);
        }
        let response = request.send().await.map_err(|e| e.to_string())?;
        let status = response.status();
        let text = response.text().await.map_err(|e| e.to_string())?;
        Ok(HttpResponse { status, body: text })
    }
}

fn current_http_client() -> Arc<dyn HttpClient> {
    if let Some(override_lock) = HTTP_CLIENT_OVERRIDE.get() {
        if let Some(client) = override_lock.lock().unwrap().clone() {
            return client;
        }
    }

    DEFAULT_HTTP_CLIENT
        .get_or_init(|| Arc::new(ReqwestHttpClient::new()) as Arc<dyn HttpClient>)
        .clone()
}

/// Guard that clears the global HTTP client override when dropped.
pub struct HttpClientOverrideGuard;

impl Drop for HttpClientOverrideGuard {
    fn drop(&mut self) {
        clear_http_client_override();
    }
}

/// Override the HTTP client used by OpenApiTool. Primarily for tests.
pub fn set_http_client_override(client: Arc<dyn HttpClient>) -> HttpClientOverrideGuard {
    let lock = HTTP_CLIENT_OVERRIDE.get_or_init(|| Mutex::new(None));
    *lock.lock().unwrap() = Some(client);
    HttpClientOverrideGuard
}

/// Clear the global HTTP client override if one is set.
pub fn clear_http_client_override() {
    if let Some(lock) = HTTP_CLIENT_OVERRIDE.get() {
        *lock.lock().unwrap() = None;
    }
}

/// A tool that executes external integrations based on OpenAPI definitions
pub struct OpenApiTool {
    /// The tool definition
    definition: BionicToolDefinition,
    /// The base URL for the API
    base_url: String,
    /// The HTTP client
    client: Arc<dyn HttpClient>,
    /// The OpenAPI specification
    spec: oas3::OpenApiV3Spec,
    /// The operation ID for this tool
    operation_id: String,
    /// The header name to pass the token in
    auth_header_name: String,
    /// Token provider for authenticated requests
    token_provider: Option<Arc<dyn TokenProvider>>,
}

/// Start a simple scheduler that logs token refresh events
impl OpenApiTool {
    pub fn new(
        definition: BionicToolDefinition,
        base_url: String,
        spec: oas3::OpenApiV3Spec,
        operation_id: String,
        auth_header_name: String,
        token_provider: Option<Arc<dyn TokenProvider>>,
    ) -> Self {
        let client = current_http_client();
        Self::with_http_client(
            definition,
            base_url,
            spec,
            operation_id,
            auth_header_name,
            token_provider,
            client,
        )
    }

    pub fn with_http_client(
        definition: BionicToolDefinition,
        base_url: String,
        spec: oas3::OpenApiV3Spec,
        operation_id: String,
        auth_header_name: String,
        token_provider: Option<Arc<dyn TokenProvider>>,
        client: Arc<dyn HttpClient>,
    ) -> Self {
        Self {
            definition,
            base_url,
            client,
            spec,
            operation_id,
            auth_header_name,
            token_provider,
        }
    }

    /// Find operation details by operation_id in the OpenAPI spec
    fn find_operation_details(&self) -> Result<(String, String, &Operation), String> {
        for (path, method, operation) in self.spec.operations() {
            if let Some(op_id) = &operation.operation_id {
                if op_id == &self.operation_id {
                    return Ok((path.to_string(), method.to_string(), operation));
                }
            }
        }
        Err(format!(
            "Operation with ID '{}' not found in OpenAPI spec",
            self.operation_id
        ))
    }

    /// Produce Authorization header if bearer token is present
    async fn build_auth_header(&self) -> Option<(String, String)> {
        if let Some(provider) = &self.token_provider {
            if let Some(token) = provider.token().await {
                let preview = &token[..6.min(token.len())];
                tracing::debug!("Adding bearer token {}...", preview);
                let header_value = if self.auth_header_name.eq_ignore_ascii_case("Authorization")
                    && !token.to_lowercase().starts_with("basic ")
                {
                    format!("Bearer {}", token)
                } else {
                    token
                };
                return Some((self.auth_header_name.clone(), header_value));
            }
        }
        None
    }

    async fn collect_headers(&self) -> Vec<(String, String)> {
        let mut headers = Vec::new();
        if let Some(header) = self.build_auth_header().await {
            headers.push(header);
        }
        headers
    }
}

#[async_trait]
impl ToolInterface for OpenApiTool {
    fn get_tool(&self) -> BionicToolDefinition {
        self.definition.clone()
    }

    async fn execute(&self, arguments: &str) -> Result<serde_json::Value, serde_json::Value> {
        tracing::info!(
            "Executing OpenAPI tool {} with arguments: {}",
            self.name(),
            arguments
        );

        // Find operation details by operation_id
        let (path, method, operation) = self
            .find_operation_details()
            .map_err(|e| crate::json_error("Operation not found", e))?;

        // Parse arguments
        let args: Value = serde_json::from_str(arguments)
            .map_err(|e| crate::json_error("Failed to parse arguments", e))?;

        // Separate path, query, and request body parameters
        let (path_params, query_params, request_body_params) =
            separate_parameters(&args, operation)
                .map_err(|e| crate::json_error("Failed to separate parameters", e))?;

        tracing::debug!(
            "Separated parameters - Path: {}, Query: {}, Request Body: {}",
            serde_json::to_string(&path_params).unwrap_or_default(),
            serde_json::to_string(&query_params).unwrap_or_default(),
            serde_json::to_string(&request_body_params).unwrap_or_default()
        );

        // Substitute path parameters in the URL using only path params
        let path_with_params = substitute_path_parameters(&path, &path_params, operation)
            .map_err(|e| crate::json_error("Failed to substitute path parameters", e))?;

        // Construct the final URL and append query parameters
        let mut url = Url::parse(&format!("{}{}", self.base_url, path_with_params))
            .map_err(|e| crate::json_error("Invalid URL", e))?;
        if let Some(obj) = query_params.as_object() {
            if !obj.is_empty() {
                let mut pairs = url.query_pairs_mut();
                for (k, v) in obj {
                    let value = match v {
                        Value::String(s) => s.clone(),
                        Value::Number(n) => n.to_string(),
                        Value::Bool(b) => b.to_string(),
                        _ => v.to_string(),
                    };
                    pairs.append_pair(k, &value);
                }
            }
        }
        tracing::debug!("Making request to URL: {} using method: {}", url, method);

        // Determine if we should send a request body
        let body_obj = request_body_params.as_object();
        let has_request_body = body_obj.is_some_and(|obj| !obj.is_empty());
        if body_obj.is_none() {
            return Err(serde_json::json!({
                "error": "Malformed request body arguments"
            }));
        }

        // Parse the HTTP method
        let http_method: Method = method
            .parse()
            .map_err(|e| crate::json_error("Unsupported HTTP method", e))?;

        let body = if has_request_body {
            Some(request_body_params.clone())
        } else {
            None
        };

        let mut response = self
            .client
            .send(
                http_method.clone(),
                url.clone(),
                self.collect_headers().await,
                body.clone(),
            )
            .await
            .map_err(|e| crate::json_error("Failed to make request", e))?;

        if response.status == StatusCode::UNAUTHORIZED {
            if let Some(provider) = &self.token_provider {
                tracing::info!("Received 401 response; forcing token refresh and retrying");
                provider.force_refresh().await;
                response = self
                    .client
                    .send(http_method, url, self.collect_headers().await, body.clone())
                    .await
                    .map_err(|e| crate::json_error("Failed to make request", e))?;
            }
        }

        // Check if the request was successful
        if !response.status.is_success() {
            return Err(serde_json::json!({
                "error": "Request failed",
                "status": response.status.to_string()
            }));
        }

        // Parse the response
        let response_text = response.body;

        // Try to parse as JSON, fallback to text if it fails
        match serde_json::from_str::<serde_json::Value>(&response_text) {
            Ok(json_value) => Ok(json_value),
            Err(_) => Ok(serde_json::json!({
                "content": response_text,
                "content_type": "text"
            })),
        }
    }
}

/// Separate path, query, and request body parameters
fn separate_parameters(
    args: &Value,
    operation: &Operation,
) -> Result<(Value, Value, Value), String> {
    let mut path_params = serde_json::Map::new();
    let mut query_params = serde_json::Map::new();
    let mut request_body_params = serde_json::Map::new();

    // Get all arguments as an object
    let args_obj = args.as_object().ok_or("Arguments must be a JSON object")?;

    // Collect path and query parameter names from the operation
    let mut path_param_names = HashSet::new();
    let mut query_param_names = HashSet::new();

    for param in &operation.parameters {
        if let ObjectOrReference::Object(Parameter { name, location, .. }) = param {
            match *location {
                ParameterIn::Path => {
                    path_param_names.insert(name.clone());
                }
                ParameterIn::Query => {
                    query_param_names.insert(name.clone());
                }
                _ => {}
            }
        }
    }

    // Separate the arguments based on parameter type
    for (key, value) in args_obj {
        if path_param_names.contains(key) {
            path_params.insert(key.clone(), value.clone());
        } else if query_param_names.contains(key) {
            query_params.insert(key.clone(), value.clone());
        } else {
            request_body_params.insert(key.clone(), value.clone());
        }
    }

    Ok((
        Value::Object(path_params),
        Value::Object(query_params),
        Value::Object(request_body_params),
    ))
}

/// Substitute path parameters in a URL template with actual values
fn substitute_path_parameters(
    path: &str,
    args: &Value,
    operation: &Operation,
) -> Result<String, String> {
    let mut result_path = path.to_string();

    // Extract path parameters from the operation
    let args_obj = args.as_object().ok_or("Arguments must be a JSON object")?;

    for param in &operation.parameters {
        if let ObjectOrReference::Object(Parameter {
            name,
            location,
            required,
            ..
        }) = param
        {
            if *location == ParameterIn::Path {
                let placeholder = format!("{{{}}}", name);
                if let Some(value) = args_obj.get(name) {
                    let value_str = match value {
                        Value::String(s) => s.clone(),
                        Value::Number(n) => n.to_string(),
                        Value::Bool(b) => b.to_string(),
                        _ => {
                            return Err(format!("Invalid value type for path parameter: {}", name))
                        }
                    };
                    result_path = result_path.replace(&placeholder, &value_str);
                } else if required.unwrap_or(false) {
                    return Err(format!("Missing required path parameter: {}", name));
                }
            }
        }
    }

    Ok(result_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token_providers::StaticTokenProvider;
    use openai_api::ChatCompletionFunctionDefinition;
    use serde_json::json;
    use std::collections::VecDeque;
    use std::sync::Arc;

    fn create_test_openapi_spec() -> oas3::OpenApiV3Spec {
        let spec_json = json!({
            "openapi": "3.0.0",
            "info": {
                "title": "Test API",
                "version": "1.0.0"
            },
            "servers": [
                {
                    "url": "https://api.example.com"
                }
            ],
            "paths": {
                "/users": {
                    "get": {
                        "operationId": "getUsers",
                        "summary": "Get all users",
                        "description": "Retrieve a list of all users"
                    },
                    "post": {
                        "operationId": "createUser",
                        "summary": "Create a user",
                        "description": "Create a new user"
                    }
                },
                "/users/{id}": {
                    "get": {
                        "operationId": "getUserById",
                        "summary": "Get user by ID",
                        "description": "Retrieve a specific user by ID"
                    }
                }
            }
        });

        serde_json::from_value(spec_json).unwrap()
    }

    fn create_uk_police_api_spec() -> oas3::OpenApiV3Spec {
        let spec_json = json!({
            "openapi": "3.0.3",
            "info": {
                "title": "UK Police Forces API",
                "version": "1.0.0",
                "description": "Provides a list of UK police forces and detailed data for each force."
            },
            "servers": [
                {
                    "url": "https://data.police.uk",
                    "description": "Production server"
                }
            ],
            "paths": {
                "/api/forces": {
                    "get": {
                        "summary": "List all police forces",
                        "operationId": "getForces",
                        "responses": {
                            "200": {
                                "description": "A list of police forces"
                            }
                        }
                    }
                },
                "/api/forces/{id}": {
                    "get": {
                        "summary": "Get details of a UK police force",
                        "operationId": "getPoliceForceDetails",
                        "parameters": [
                            {
                                "in": "path",
                                "name": "id",
                                "required": true,
                                "schema": {
                                    "type": "string"
                                },
                                "description": "The identifier of the police force (e.g. leicestershire)"
                            }
                        ],
                        "responses": {
                            "200": {
                                "description": "Police force details"
                            }
                        }
                    }
                }
            }
        });

        serde_json::from_value(spec_json).unwrap()
    }

    #[test]
    fn test_openapi_tool_find_operation_details() {
        let spec = create_test_openapi_spec();
        let tool_def = BionicToolDefinition {
            r#type: "function".to_string(),
            function: ChatCompletionFunctionDefinition {
                name: "getUsers".to_string(),
                description: "Get all users".to_string(),
                parameters: json!({}),
            },
        };

        let tool = OpenApiTool::new(
            tool_def,
            "https://api.example.com".to_string(),
            spec,
            "getUsers".to_string(),
            "Authorization".to_string(),
            None,
        );

        let result = tool.find_operation_details();
        assert!(result.is_ok());

        let (path, method, operation) = result.unwrap();
        assert_eq!(path, "/users");
        assert_eq!(method, "GET");
        assert_eq!(operation.operation_id, Some("getUsers".to_string()));
    }

    #[test]
    fn test_openapi_tool_operation_not_found() {
        let spec = create_test_openapi_spec();
        let tool_def = BionicToolDefinition {
            r#type: "function".to_string(),
            function: ChatCompletionFunctionDefinition {
                name: "nonExistentOperation".to_string(),
                description: "Non-existent operation".to_string(),
                parameters: json!({}),
            },
        };

        let tool = OpenApiTool::new(
            tool_def,
            "https://api.example.com".to_string(),
            spec,
            "nonExistentOperation".to_string(),
            "Authorization".to_string(),
            None,
        );

        let result = tool.find_operation_details();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Operation with ID 'nonExistentOperation' not found"));
    }

    #[test]
    fn test_tool_name_returns_operation_id() {
        let spec = create_test_openapi_spec();
        let tool_def = BionicToolDefinition {
            r#type: "function".to_string(),
            function: ChatCompletionFunctionDefinition {
                name: "createUser".to_string(),
                description: "Create a user".to_string(),
                parameters: json!({}),
            },
        };

        let tool = OpenApiTool::new(
            tool_def,
            "https://api.example.com".to_string(),
            spec,
            "createUser".to_string(),
            "Authorization".to_string(),
            None,
        );

        assert_eq!(tool.name(), "createUser");
    }

    #[test]
    fn test_substitute_path_parameters() {
        let spec = create_uk_police_api_spec();

        // Find the getPoliceForceDetails operation
        let mut operation = None;
        for (_path, _method, op) in spec.operations() {
            if op.operation_id.as_ref() == Some(&"getPoliceForceDetails".to_string()) {
                operation = Some(op);
                break;
            }
        }

        let operation = operation.expect("Should find getPoliceForceDetails operation");
        let args = json!({"id": "leicestershire"});

        let result = substitute_path_parameters("/api/forces/{id}", &args, operation);
        assert_eq!(result.unwrap(), "/api/forces/leicestershire");
    }

    #[test]
    fn test_substitute_path_parameters_missing_required() {
        let spec = create_uk_police_api_spec();

        // Find the getPoliceForceDetails operation
        let mut operation = None;
        for (_path, _method, op) in spec.operations() {
            if op.operation_id.as_ref() == Some(&"getPoliceForceDetails".to_string()) {
                operation = Some(op);
                break;
            }
        }

        let operation = operation.expect("Should find getPoliceForceDetails operation");
        let args = json!({});

        let result = substitute_path_parameters("/api/forces/{id}", &args, operation);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Missing required path parameter: id"));
    }

    #[test]
    fn test_separate_parameters_with_query() {
        let spec_json = json!({
            "openapi": "3.0.0",
            "info": {"title": "Query API", "version": "1.0"},
            "paths": {
                "/items/{id}": {
                    "get": {
                        "operationId": "getItem",
                        "parameters": [
                            {"in": "path", "name": "id", "required": true, "schema": {"type": "string"}},
                            {"in": "query", "name": "filter", "schema": {"type": "string"}}
                        ],
                        "responses": {"200": {"description": "ok"}}
                    }
                }
            }
        });

        let spec: oas3::OpenApiV3Spec = serde_json::from_value(spec_json).unwrap();
        let mut operation = None;
        for (_p, _m, op) in spec.operations() {
            if op.operation_id.as_deref() == Some("getItem") {
                operation = Some(op);
                break;
            }
        }
        let operation = operation.expect("operation not found");
        let args = json!({"id": "123", "filter": "all", "name": "bob"});

        let (path_params, query_params, body_params) =
            separate_parameters(&args, operation).expect("separate params");

        assert_eq!(path_params, json!({"id": "123"}));
        assert_eq!(query_params, json!({"filter": "all"}));
        assert_eq!(body_params, json!({"name": "bob"}));
    }

    #[tokio::test]
    async fn test_add_auth_header_custom_name() {
        let spec = create_test_openapi_spec();
        let tool_def = BionicToolDefinition {
            r#type: "function".to_string(),
            function: ChatCompletionFunctionDefinition {
                name: "getUsers".to_string(),
                description: "Get all users".to_string(),
                parameters: json!({}),
            },
        };

        let provider = StaticTokenProvider::new("abc123".to_string());
        let tool = OpenApiTool::new(
            tool_def,
            "https://api.example.com".to_string(),
            spec,
            "getUsers".to_string(),
            "x-api-key".to_string(),
            Some(Arc::new(provider)),
        );

        let header = tool.build_auth_header().await.unwrap();
        assert_eq!(header.0, "x-api-key");
        assert_eq!(header.1, "abc123");
    }

    #[derive(Clone)]
    struct MockHttpClient {
        responses: Arc<tokio::sync::Mutex<VecDeque<HttpResponse>>>,
        captured_headers: Arc<tokio::sync::Mutex<Vec<Option<String>>>>,
    }

    impl MockHttpClient {
        fn new(responses: Vec<HttpResponse>) -> Self {
            Self {
                responses: Arc::new(tokio::sync::Mutex::new(VecDeque::from(responses))),
                captured_headers: Arc::new(tokio::sync::Mutex::new(Vec::new())),
            }
        }

        async fn captured_headers(&self) -> Vec<Option<String>> {
            self.captured_headers.lock().await.clone()
        }
    }

    #[async_trait]
    impl HttpClient for MockHttpClient {
        async fn send(
            &self,
            _method: Method,
            _url: Url,
            headers: Vec<(String, String)>,
            _body: Option<Value>,
        ) -> Result<HttpResponse, String> {
            let auth_header = headers.iter().find_map(|(name, value)| {
                if name.eq_ignore_ascii_case("authorization") {
                    Some(value.clone())
                } else {
                    None
                }
            });
            self.captured_headers.lock().await.push(auth_header);
            let mut responses = self.responses.lock().await;
            responses
                .pop_front()
                .ok_or_else(|| "No more mock responses".to_string())
        }
    }

    #[tokio::test]
    async fn test_execute_refresh_and_retry_on_401() {
        use serde_json::json;

        let spec_json = json!({
            "openapi": "3.0.0",
            "info": {"title": "Test", "version": "1.0"},
            "paths": {"/protected": {"get": {"operationId": "getProtected"}}}
        });
        let spec: oas3::OpenApiV3Spec = serde_json::from_value(spec_json).unwrap();

        struct MockTokenProvider {
            tokens: Vec<String>,
            idx: tokio::sync::Mutex<usize>,
        }

        impl MockTokenProvider {
            fn new(tokens: Vec<String>) -> Self {
                Self {
                    tokens,
                    idx: tokio::sync::Mutex::new(0),
                }
            }
        }

        #[async_trait]
        impl TokenProvider for MockTokenProvider {
            async fn token(&self) -> Option<String> {
                let idx = *self.idx.lock().await;
                Some(self.tokens[idx].clone())
            }

            async fn force_refresh(&self) {
                let mut idx = self.idx.lock().await;
                if *idx + 1 < self.tokens.len() {
                    *idx += 1;
                }
            }
        }

        let provider = Arc::new(MockTokenProvider::new(vec![
            "first".into(),
            "second".into(),
        ]));
        let tool_def = BionicToolDefinition {
            r#type: "function".to_string(),
            function: ChatCompletionFunctionDefinition {
                name: "getProtected".to_string(),
                description: "".to_string(),
                parameters: json!({}),
            },
        };

        let client = Arc::new(MockHttpClient::new(vec![
            HttpResponse {
                status: StatusCode::UNAUTHORIZED,
                body: String::new(),
            },
            HttpResponse {
                status: StatusCode::OK,
                body: "{\"ok\":true}".to_string(),
            },
        ]));

        let tool = OpenApiTool::with_http_client(
            tool_def,
            "http://mock.api".to_string(),
            spec,
            "getProtected".to_string(),
            "Authorization".to_string(),
            Some(provider),
            client.clone(),
        );

        let result = tool.execute("{}").await.unwrap();
        assert_eq!(result, json!({"ok": true}));

        let headers = client.captured_headers().await;
        assert_eq!(headers.len(), 2);
        assert_eq!(headers[0].as_deref(), Some("Bearer first"));
        assert_eq!(headers[1].as_deref(), Some("Bearer second"));
    }
}
