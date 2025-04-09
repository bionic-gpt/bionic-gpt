//! Tests for the integration registry.

#[cfg(test)]
mod tests {
    use crate::models::{FunctionDefinition, Tool};
    use crate::{Integration, IntegrationError, Registry};
    use async_trait::async_trait;
    use serde_json::json;
    use std::sync::Arc;

    // Mock integration for testing
    struct MockIntegration {
        name: String,
        description: String,
        tools: Vec<Tool>,
        function_results: std::collections::HashMap<String, String>,
    }

    impl MockIntegration {
        fn new(name: &str, description: &str) -> Self {
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
                name: name.to_string(),
                description: description.to_string(),
                tools: vec![Tool {
                    r#type: "function".to_string(),
                    function: FunctionDefinition {
                        name: format!("{}.get_weather", name),
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
    }

    #[async_trait]
    impl Integration for MockIntegration {
        fn name(&self) -> &str {
            &self.name
        }

        fn description(&self) -> &str {
            &self.description
        }

        async fn discover(&self) -> Result<Vec<Tool>, IntegrationError> {
            Ok(self.tools.clone())
        }

        async fn execute(
            &self,
            function_name: &str,
            _arguments: &str,
        ) -> Result<String, IntegrationError> {
            if let Some(result) = self.function_results.get(function_name) {
                Ok(result.clone())
            } else {
                Err(IntegrationError::FunctionNotFound(
                    function_name.to_string(),
                ))
            }
        }
    }

    // Mock registry for testing
    struct MockRegistry {
        integrations: Vec<Arc<dyn Integration>>,
    }

    impl MockRegistry {
        fn new() -> Self {
            Self {
                integrations: Vec::new(),
            }
        }

        fn register(&mut self, integration: Arc<dyn Integration>) {
            self.integrations.push(integration);
        }
    }

    #[async_trait]
    impl Registry for MockRegistry {
        async fn discover_all(&self) -> Vec<Tool> {
            let mut tools = Vec::new();
            for integration in &self.integrations {
                if let Ok(mut integration_tools) = integration.discover().await {
                    tools.append(&mut integration_tools);
                }
            }
            tools
        }

        async fn execute(
            &self,
            integration_name: &str,
            function_name: &str,
            arguments: &str,
        ) -> Result<String, IntegrationError> {
            for integration in &self.integrations {
                if integration.name() == integration_name {
                    return integration.execute(function_name, arguments).await;
                }
            }
            Err(IntegrationError::IntegrationNotFound(
                integration_name.to_string(),
            ))
        }
    }

    #[tokio::test]
    async fn test_registry_discover_all() {
        let mut registry = MockRegistry::new();

        let integration1 = MockIntegration::new("weather", "Weather integration");
        let integration2 = MockIntegration::new("calendar", "Calendar integration");

        registry.register(Arc::new(integration1));
        registry.register(Arc::new(integration2));

        let tools = registry.discover_all().await;
        assert_eq!(tools.len(), 2);

        let names: Vec<String> = tools.iter().map(|t| t.function.name.clone()).collect();
        assert!(names.contains(&"weather.get_weather".to_string()));
        assert!(names.contains(&"calendar.get_weather".to_string()));
    }

    #[tokio::test]
    async fn test_registry_execute() {
        let mut registry = MockRegistry::new();

        let integration = MockIntegration::new("weather", "Weather integration");
        registry.register(Arc::new(integration));

        let arguments = json!({
            "location": "San Francisco, CA",
            "unit": "celsius"
        })
        .to_string();

        let result = registry
            .execute("weather", "get_weather", &arguments)
            .await
            .unwrap();
        let result_json: serde_json::Value = serde_json::from_str(&result).unwrap();

        assert_eq!(result_json["location"], "San Francisco, CA");
        assert_eq!(result_json["temperature"], 22);
        assert_eq!(result_json["unit"], "celsius");
    }

    #[tokio::test]
    async fn test_registry_execute_unknown_integration() {
        let mut registry = MockRegistry::new();

        let integration = MockIntegration::new("weather", "Weather integration");
        registry.register(Arc::new(integration));

        let arguments = json!({
            "location": "San Francisco, CA",
            "unit": "celsius"
        })
        .to_string();

        let result = registry.execute("unknown", "get_weather", &arguments).await;
        assert!(result.is_err());

        match result {
            Err(IntegrationError::IntegrationNotFound(name)) => {
                assert_eq!(name, "unknown");
            }
            _ => panic!("Expected IntegrationNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_registry_execute_unknown_function() {
        let mut registry = MockRegistry::new();

        let integration = MockIntegration::new("weather", "Weather integration");
        registry.register(Arc::new(integration));

        let arguments = json!({
            "location": "San Francisco, CA",
            "unit": "celsius"
        })
        .to_string();

        let result = registry
            .execute("weather", "unknown_function", &arguments)
            .await;
        assert!(result.is_err());

        match result {
            Err(IntegrationError::FunctionNotFound(name)) => {
                assert_eq!(name, "unknown_function");
            }
            _ => panic!("Expected FunctionNotFound error"),
        }
    }
}
