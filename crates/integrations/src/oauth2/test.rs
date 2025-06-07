#[cfg(test)]
mod tests {
    use super::detection::{get_oauth2_config, has_oauth2_security};
    use oas3::OpenApiV3;
    use serde_json;

    #[test]
    fn test_oauth2_detection() {
        let spec_json = r#"
        {
          "openapi": "3.0.0",
          "info": {
            "title": "Test API with OAuth2",
            "version": "1.0.0"
          },
          "components": {
            "securitySchemes": {
              "oauth2": {
                "type": "oauth2",
                "flows": {
                  "authorizationCode": {
                    "authorizationUrl": "https://example.com/oauth/authorize",
                    "tokenUrl": "https://example.com/oauth/token",
                    "scopes": {
                      "read": "Read access"
                    }
                  }
                }
              }
            }
          },
          "paths": {
            "/users": {
              "get": {
                "summary": "Get users",
                "security": [
                  {
                    "oauth2": ["read"]
                  }
                ],
                "responses": {
                  "200": {
                    "description": "Success"
                  }
                }
              }
            }
          }
        }
        "#;

        let spec: OpenApiV3 = serde_json::from_str(spec_json).unwrap();
        let has_oauth2 = has_oauth2_security(&spec);

        assert!(has_oauth2, "OAuth2 should be detected in the test spec");

        let config = get_oauth2_config(&spec).expect("config should be parsed");
        assert_eq!(
            config.authorization_url,
            "https://example.com/oauth/authorize"
        );
        assert_eq!(config.token_url, "https://example.com/oauth/token");
        assert_eq!(config.scopes, vec!["read".to_string()]);
    }

    #[test]
    fn test_no_oauth2_detection() {
        let spec_json = r#"
        {
          "openapi": "3.0.0",
          "info": {
            "title": "Test API without OAuth2",
            "version": "1.0.0"
          },
          "paths": {
            "/users": {
              "get": {
                "summary": "Get users",
                "responses": {
                  "200": {
                    "description": "Success"
                  }
                }
              }
            }
          }
        }
        "#;

        let spec: OpenApiV3 = serde_json::from_str(spec_json).unwrap();
        let has_oauth2 = has_oauth2_security(&spec);

        assert!(!has_oauth2, "OAuth2 should not be detected in spec without OAuth2");
        assert!(get_oauth2_config(&spec).is_none(), "No config should be returned");
    }
}