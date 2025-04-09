//! Tests for the MCP Server integration.

#[cfg(test)]
mod tests {
    use crate::models::{FunctionDefinition, Tool};
    use crate::Integration;
    use async_trait::async_trait;
    use serde_json::json;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    // Mock server for testing
    struct MockServer {
        tools: Vec<Tool>,
        function_results: std::collections::HashMap<String, String>,
    }

    impl MockServer {
        fn new() -> Self {
            let mut function_results = std::collections::HashMap::new();
            function_results.insert(
                "get_weather".to_string(),
                json!({
                    "location": "San Francisco, CA",
                    "temperature": 22,
                    "unit": "celsius",
                    "condition": "sunny",
                    "forecast": ["sunny", "partly cloudy", "sunny"]
                })
                .to_string(),
            );

            Self {
                tools: vec![Tool {
                    r#type: "function".to_string(),
                    function: FunctionDefinition {
                        name: "get_weather".to_string(),
                        description: "Get the current weather in a given location".to_string(),
                        parameters: json!({
                            "type": "object",
                            "properties": {
                                "location": {
                                    "type": "string",
                                    "description": "The city and state, e.g. San Francisco, CA"
                                },
                                "unit": {
                                    "type": "string",
                                    "enum": ["celsius", "fahrenheit"],
                                    "description": "The temperature unit to use"
                                }
                            },
                            "required": ["location"]
                        }),
                    },
                }],
                function_results,
            }
        }

        async fn handle_discover(&self) -> Vec<Tool> {
            self.tools.clone()
        }

        async fn handle_execute(&self, function_name: &str, _arguments: &str) -> Option<String> {
            self.function_results.get(function_name).cloned()
        }
    }

    // Mock reqwest client for testing
    struct MockClient {
        server: Arc<Mutex<MockServer>>,
    }

    impl MockClient {
        fn new(server: Arc<Mutex<MockServer>>) -> Self {
            Self { server }
        }

        async fn get(&self, url: &str) -> MockResponse {
            if url.ends_with("/discover") {
                let server = self.server.lock().await;
                let tools = server.handle_discover().await;
                MockResponse {
                    status: 200,
                    body: serde_json::to_string(&tools).unwrap(),
                }
            } else {
                MockResponse {
                    status: 404,
                    body: "Not found".to_string(),
                }
            }
        }

        async fn post(&self, url: &str, body: &str) -> MockResponse {
            if url.ends_with("/execute") {
                let server = self.server.lock().await;
                let request: serde_json::Value = serde_json::from_str(body).unwrap();
                let function = request["function"].as_str().unwrap();
                let arguments = request["arguments"].as_str().unwrap();

                if let Some(result) = server.handle_execute(function, arguments).await {
                    MockResponse {
                        status: 200,
                        body: result,
                    }
                } else {
                    MockResponse {
                        status: 404,
                        body: "Function not found".to_string(),
                    }
                }
            } else {
                MockResponse {
                    status: 404,
                    body: "Not found".to_string(),
                }
            }
        }
    }

    struct MockResponse {
        status: u16,
        body: String,
    }

    impl MockResponse {
        fn status(&self) -> MockStatus {
            MockStatus(self.status)
        }

        async fn json<T: serde::de::DeserializeOwned>(&self) -> T {
            serde_json::from_str(&self.body).unwrap()
        }

        async fn text(&self) -> String {
            self.body.clone()
        }
    }

    struct MockStatus(u16);

    impl MockStatus {
        fn is_success(&self) -> bool {
            self.0 >= 200 && self.0 < 300
        }

        fn as_u16(&self) -> u16 {
            self.0
        }
    }

    // Mock MCP Server integration for testing
    struct MockMCPServerIntegration {
        name: String,
        base_url: String,
        client: MockClient,
    }

    impl MockMCPServerIntegration {
        fn new(name: String, base_url: String, server: Arc<Mutex<MockServer>>) -> Self {
            Self {
                name,
                base_url,
                client: MockClient::new(server),
            }
        }
    }

    #[async_trait]
    impl Integration for MockMCPServerIntegration {
        fn name(&self) -> &str {
            &self.name
        }

        fn description(&self) -> &str {
            "MCP Server Integration"
        }

        async fn discover(&self) -> Result<Vec<Tool>, crate::IntegrationError> {
            let url = format!("{}/discover", self.base_url);
            let response = self.client.get(&url).await;

            if !response.status().is_success() {
                return Err(crate::IntegrationError::FunctionExecutionFailed(
                    "discover".to_string(),
                    format!("Server returned status: {}", response.status().as_u16()),
                ));
            }

            let tools: Vec<Tool> = response.json().await;

            // Prefix each tool's function name with the integration name
            let tools = tools
                .into_iter()
                .map(|mut tool| {
                    let original_name = tool.function.name.clone();
                    tool.function.name = format!("{}.{}", self.name, original_name);
                    tool
                })
                .collect();

            Ok(tools)
        }

        async fn execute(
            &self,
            function_name: &str,
            arguments: &str,
        ) -> Result<String, crate::IntegrationError> {
            let url = format!("{}/execute", self.base_url);

            let request = serde_json::json!({
                "function": function_name,
                "arguments": arguments,
            });

            let response = self
                .client
                .post(&url, &serde_json::to_string(&request).unwrap())
                .await;

            if !response.status().is_success() {
                return Err(crate::IntegrationError::FunctionExecutionFailed(
                    function_name.to_string(),
                    format!("Server returned status: {}", response.status().as_u16()),
                ));
            }

            let result = response.text().await;

            Ok(result)
        }
    }

    #[tokio::test]
    async fn test_mcp_server_integration_discover() {
        let server = Arc::new(Mutex::new(MockServer::new()));
        let integration = MockMCPServerIntegration::new(
            "weather".to_string(),
            "http://localhost:8080".to_string(),
            server,
        );

        let tools = integration.discover().await.unwrap();
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0].function.name, "weather.get_weather");
    }

    #[tokio::test]
    async fn test_mcp_server_integration_execute() {
        let server = Arc::new(Mutex::new(MockServer::new()));
        let integration = MockMCPServerIntegration::new(
            "weather".to_string(),
            "http://localhost:8080".to_string(),
            server,
        );

        let arguments = json!({
            "location": "San Francisco, CA",
            "unit": "celsius"
        })
        .to_string();

        let result = integration
            .execute("get_weather", &arguments)
            .await
            .unwrap();
        let result_json: serde_json::Value = serde_json::from_str(&result).unwrap();

        assert_eq!(result_json["location"], "San Francisco, CA");
        assert_eq!(result_json["temperature"], 22);
        assert_eq!(result_json["unit"], "celsius");
    }
}
