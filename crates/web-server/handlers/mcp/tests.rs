use super::*;
use http_body_util::BodyExt;
use serde_json::json;
use std::sync::MutexGuard;
use tokio::task::JoinHandle;
use tower::ServiceExt;

struct ResolverGuard {
    _lock: MutexGuard<'static, ()>,
}

impl ResolverGuard {
    fn new<F>(resolver: F) -> Self
    where
        F: Fn(&str, Uuid) -> IntegrationContext + Send + Sync + 'static,
    {
        let lock = RESOLVER_LOCK.get_or_init(|| Mutex::new(()));
        let guard = lock.lock().unwrap();
        set_mock_resolver(resolver);
        Self { _lock: guard }
    }
}

impl Drop for ResolverGuard {
    fn drop(&mut self) {
        clear_mock_resolver();
    }
}

struct ApiKeyGuard {
    key: String,
}

impl ApiKeyGuard {
    fn new(key: &str, team_id: i32) -> Self {
        register_mock_api_key(key, team_id);
        Self {
            key: key.to_string(),
        }
    }
}

impl Drop for ApiKeyGuard {
    fn drop(&mut self) {
        remove_mock_api_key(&self.key);
    }
}

struct EnvVarGuard {
    key: &'static str,
    original: Option<String>,
}

impl EnvVarGuard {
    fn set(key: &'static str, value: &str) -> Self {
        let original = std::env::var(key).ok();
        std::env::set_var(key, value);
        Self { key, original }
    }
}

impl Drop for EnvVarGuard {
    fn drop(&mut self) {
        if let Some(original) = &self.original {
            std::env::set_var(self.key, original);
        } else {
            std::env::remove_var(self.key);
        }
    }
}

fn create_test_pool() -> Pool {
    db::create_pool("postgres://postgres:postgres@127.0.0.1:1/postgres")
}

fn test_router(pool: Pool) -> Router {
    Router::new().merge(routes()).layer(Extension(pool))
}

fn sample_spec(base_url: &str, slug: &str) -> Value {
    json!({
        "openapi": "3.0.0",
        "info": {
            "title": "Test API",
            "version": "1.0.0",
            "bionic-slug": slug,
            "x-bionic-slug": slug,
        },
        "servers": [
            { "url": base_url }
        ],
        "paths": {
            "/ping": {
                "get": {
                    "operationId": "ping",
                    "summary": "Ping",
                    "description": "Ping endpoint",
                    "responses": {
                        "200": {
                            "description": "success"
                        }
                    }
                }
            }
        }
    })
}

fn minimal_spec(slug: &str) -> Value {
    json!({
        "openapi": "3.0.0",
        "info": {
            "title": "Minimal API",
            "version": "1.0.0",
            "bionic-slug": slug,
            "x-bionic-slug": slug,
        },
        "servers": [
            { "url": "http://example.com" }
        ],
        "paths": {}
    })
}

async fn spawn_ping_service(response: Value) -> (String, JoinHandle<()>) {
    let app = Router::new().route(
        "/ping",
        axum::routing::get(move || {
            let payload = response.clone();
            async move { Json(payload) }
        }),
    );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("listener");
    let addr = listener.local_addr().unwrap();
    let handle = tokio::spawn(async move {
        axum::serve(listener, app.into_make_service())
            .await
            .expect("server");
    });
    (format!("http://{}", addr), handle)
}

#[tokio::test]
async fn session_initialize_returns_capabilities() {
    let pool = create_test_pool();
    let slug = "test".to_string();
    let connection_id = Uuid::new_v4();
    let spec = sample_spec("http://example.com", &slug);

    let api_key_value = "test-capabilities-key";

    let context = IntegrationContext {
        definition: spec,
        integration_id: 7,
        user_id: 11,
        team_id: 21,
        user_openid_sub: Some("user-1".to_string()),
        connection: ConnectionAuth::ApiKey {
            connection_id: 42,
            api_key: "abc".to_string(),
        },
    };

    let _api_guard = ApiKeyGuard::new(api_key_value, context.team_id);

    let slug_for_guard = slug.clone();
    let _guard = ResolverGuard::new(move |requested_slug, requested_id| {
        assert_eq!(requested_slug, slug_for_guard);
        assert_eq!(requested_id, connection_id);
        context.clone()
    });

    let app = test_router(pool.clone());

    let payload = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "session.initialize",
        "params": {}
    });

    let response = app
        .oneshot(
            axum::http::Request::builder()
                .method("POST")
                .uri(format!("/v1/mcp/{}/{}", slug, connection_id))
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", api_key_value))
                .body(axum::body::Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["result"]["protocolVersion"], DEFAULT_PROTOCOL_VERSION);
    assert_eq!(
        json["result"]["capabilities"]["tools"]["listChanged"],
        false
    );
    assert_eq!(json["result"]["metadata"]["integrationId"], 7);
}

#[tokio::test]
async fn api_key_validation_can_be_disabled_via_env() {
    let _env_guard = EnvVarGuard::set(super::SKIP_API_KEY_ENV_VAR, "true");

    let pool = create_test_pool();
    let slug = "test".to_string();
    let connection_id = Uuid::new_v4();
    let spec = minimal_spec(&slug);

    let context = IntegrationContext {
        definition: spec,
        integration_id: 71,
        user_id: 91,
        team_id: 201,
        user_openid_sub: Some("user-env".to_string()),
        connection: ConnectionAuth::ApiKey {
            connection_id: 321,
            api_key: "env".to_string(),
        },
    };

    let slug_for_guard = slug.clone();
    let _guard = ResolverGuard::new(move |requested_slug, requested_id| {
        assert_eq!(requested_slug, slug_for_guard);
        assert_eq!(requested_id, connection_id);
        context.clone()
    });

    let app = test_router(pool.clone());

    let payload = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "session.initialize",
        "params": {}
    });

    let response = app
        .oneshot(
            axum::http::Request::builder()
                .method("POST")
                .uri(format!("/v1/mcp/{}/{}", slug, connection_id))
                .header("content-type", "application/json")
                .body(axum::body::Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["result"]["metadata"]["integrationId"], 71);
}

#[tokio::test]
async fn initialize_returns_server_info() {
    let pool = create_test_pool();
    let slug = "test".to_string();
    let connection_id = Uuid::new_v4();
    let spec = sample_spec("http://example.com", &slug);

    let api_key_value = "test-initialize-key";

    let context = IntegrationContext {
        definition: spec,
        integration_id: 12,
        user_id: 44,
        team_id: 33,
        user_openid_sub: Some("user-7".to_string()),
        connection: ConnectionAuth::ApiKey {
            connection_id: 88,
            api_key: "stu".to_string(),
        },
    };

    let _api_guard = ApiKeyGuard::new(api_key_value, context.team_id);

    let slug_for_guard = slug.clone();
    let _guard = ResolverGuard::new(move |requested_slug, requested_id| {
        assert_eq!(requested_slug, slug_for_guard);
        assert_eq!(requested_id, connection_id);
        context.clone()
    });

    let app = test_router(pool.clone());

    let payload = json!({
        "jsonrpc": "2.0",
        "id": 99,
        "method": "initialize",
        "params": {
            "protocolVersion": "2025-06-18",
            "capabilities": {
                "elicitation": {}
            },
            "clientInfo": {
                "name": "example-client",
                "version": "1.0.0"
            }
        }
    });

    let response = app
        .oneshot(
            axum::http::Request::builder()
                .method("POST")
                .uri(format!("/v1/mcp/{}/{}", slug, connection_id))
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", api_key_value))
                .body(axum::body::Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["result"]["protocolVersion"], "2025-06-18");
    assert_eq!(
        json["result"]["capabilities"]["tools"]["listChanged"],
        false
    );
    assert_eq!(json["result"]["serverInfo"]["name"], SERVER_NAME);
    assert_eq!(json["result"]["serverInfo"]["version"], SERVER_VERSION);
    assert_eq!(json["result"]["metadata"]["integrationId"], 12);
}

#[tokio::test]
async fn initialize_supports_march_2025_protocol() {
    let pool = create_test_pool();
    let slug = "test".to_string();
    let connection_id = Uuid::new_v4();
    let spec = sample_spec("http://example.com", &slug);

    let api_key_value = "test-initialize-march-key";

    let context = IntegrationContext {
        definition: spec,
        integration_id: 13,
        user_id: 45,
        team_id: 34,
        user_openid_sub: Some("user-8".to_string()),
        connection: ConnectionAuth::ApiKey {
            connection_id: 89,
            api_key: "vwx".to_string(),
        },
    };

    let _api_guard = ApiKeyGuard::new(api_key_value, context.team_id);

    let slug_for_guard = slug.clone();
    let _guard = ResolverGuard::new(move |requested_slug, requested_id| {
        assert_eq!(requested_slug, slug_for_guard);
        assert_eq!(requested_id, connection_id);
        context.clone()
    });

    let app = test_router(pool.clone());

    let payload = json!({
        "jsonrpc": "2.0",
        "id": 101,
        "method": "initialize",
        "params": {
            "protocolVersion": "2025-03-26",
            "capabilities": {
                "elicitation": {}
            },
            "clientInfo": {
                "name": "example-client",
                "version": "1.0.0"
            }
        }
    });

    let response = app
        .oneshot(
            axum::http::Request::builder()
                .method("POST")
                .uri(format!("/v1/mcp/{}/{}", slug, connection_id))
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", api_key_value))
                .body(axum::body::Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["result"]["protocolVersion"], "2025-03-26");
    assert_eq!(
        json["result"]["capabilities"]["tools"]["listChanged"],
        false
    );
    assert_eq!(json["result"]["serverInfo"]["name"], SERVER_NAME);
    assert_eq!(json["result"]["serverInfo"]["version"], SERVER_VERSION);
    assert_eq!(json["result"]["metadata"]["integrationId"], 13);
}

#[tokio::test]
async fn notifications_initialized_returns_no_content() {
    let pool = create_test_pool();
    let slug = "test".to_string();
    let connection_id = Uuid::new_v4();
    let spec = sample_spec("http://example.com", &slug);

    let api_key_value = "test-notification-key";

    let context = IntegrationContext {
        definition: spec,
        integration_id: 19,
        user_id: 55,
        team_id: 43,
        user_openid_sub: Some("user-19".to_string()),
        connection: ConnectionAuth::ApiKey {
            connection_id: 101,
            api_key: "xyz".to_string(),
        },
    };

    let _api_guard = ApiKeyGuard::new(api_key_value, context.team_id);

    let slug_for_guard = slug.clone();
    let _guard = ResolverGuard::new(move |requested_slug, requested_id| {
        assert_eq!(requested_slug, slug_for_guard);
        assert_eq!(requested_id, connection_id);
        context.clone()
    });

    let app = test_router(pool.clone());

    let payload = json!({
        "jsonrpc": "2.0",
        "method": "notifications/initialized",
        "params": {}
    });

    let response = app
        .oneshot(
            axum::http::Request::builder()
                .method("POST")
                .uri(format!("/v1/mcp/{}/{}", slug, connection_id))
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", api_key_value))
                .body(axum::body::Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    assert!(body.is_empty());
}

#[tokio::test]
async fn notifications_initialized_with_id_returns_error() {
    let pool = create_test_pool();
    let slug = "test".to_string();
    let connection_id = Uuid::new_v4();
    let spec = sample_spec("http://example.com", &slug);

    let api_key_value = "test-notification-id-key";

    let context = IntegrationContext {
        definition: spec,
        integration_id: 20,
        user_id: 56,
        team_id: 44,
        user_openid_sub: Some("user-20".to_string()),
        connection: ConnectionAuth::ApiKey {
            connection_id: 102,
            api_key: "xyz".to_string(),
        },
    };

    let _api_guard = ApiKeyGuard::new(api_key_value, context.team_id);

    let slug_for_guard = slug.clone();
    let _guard = ResolverGuard::new(move |requested_slug, requested_id| {
        assert_eq!(requested_slug, slug_for_guard);
        assert_eq!(requested_id, connection_id);
        context.clone()
    });

    let app = test_router(pool.clone());

    let payload = json!({
        "jsonrpc": "2.0",
        "id": 7,
        "method": "notifications/initialized",
        "params": {}
    });

    let response = app
        .oneshot(
            axum::http::Request::builder()
                .method("POST")
                .uri(format!("/v1/mcp/{}/{}", slug, connection_id))
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", api_key_value))
                .body(axum::body::Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["error"]["code"], -32600);
    assert_eq!(
        json["error"]["message"],
        "notifications/initialized must not include an id"
    );
}

#[tokio::test]
async fn tools_list_returns_available_tools() {
    let pool = create_test_pool();
    let slug = "test".to_string();
    let connection_id = Uuid::new_v4();
    let spec = sample_spec("http://example.com", &slug);

    let api_key_value = "test-tools-list-key";

    let context = IntegrationContext {
        definition: spec,
        integration_id: 9,
        user_id: 22,
        team_id: 44,
        user_openid_sub: Some("user-2".to_string()),
        connection: ConnectionAuth::ApiKey {
            connection_id: 55,
            api_key: "def".to_string(),
        },
    };

    let _api_guard = ApiKeyGuard::new(api_key_value, context.team_id);
    let _guard = ResolverGuard::new(move |_, _| context.clone());
    let app = test_router(pool.clone());

    let payload = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/list",
        "params": {}
    });

    let response = app
        .oneshot(
            axum::http::Request::builder()
                .method("POST")
                .uri(format!("/v1/mcp/{}/{}", slug, connection_id))
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", api_key_value))
                .body(axum::body::Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["result"]["tools"].as_array().unwrap().len(), 1);
    assert_eq!(json["result"]["tools"][0]["name"], "ping");
}

#[tokio::test]
async fn tools_list_uses_canonical_spec_when_available() {
    let pool = create_test_pool();
    let slug = "dropbox".to_string();
    let connection_id = Uuid::new_v4();
    let spec = minimal_spec(&slug);

    let api_key_value = "test-canonical-key";

    let context = IntegrationContext {
        definition: spec,
        integration_id: 10,
        user_id: 33,
        team_id: 52,
        user_openid_sub: Some("user-4".to_string()),
        connection: ConnectionAuth::ApiKey {
            connection_id: 70,
            api_key: "jkl".to_string(),
        },
    };

    let _api_guard = ApiKeyGuard::new(api_key_value, context.team_id);
    let _guard = ResolverGuard::new(move |_, _| context.clone());
    let app = test_router(pool.clone());

    let payload = json!({
        "jsonrpc": "2.0",
        "id": 4,
        "method": "tools/list",
        "params": {}
    });

    let response = app
        .oneshot(
            axum::http::Request::builder()
                .method("POST")
                .uri(format!("/v1/mcp/{}/{}", slug, connection_id))
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", api_key_value))
                .body(axum::body::Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();
    let tools = json["result"]["tools"].as_array().unwrap();
    assert!(!tools.is_empty(), "expected canonical spec tools");
}

#[tokio::test]
async fn tools_call_executes_tool() {
    let pool = create_test_pool();
    let slug = "test".to_string();
    let connection_id = Uuid::new_v4();
    let (base_url, handle) = spawn_ping_service(json!({ "ok": true })).await;
    let spec = sample_spec(&base_url, &slug);

    let api_key_value = "test-tools-call-key";

    let context = IntegrationContext {
        definition: spec,
        integration_id: 3,
        user_id: 12,
        team_id: 65,
        user_openid_sub: Some("user-3".to_string()),
        connection: ConnectionAuth::ApiKey {
            connection_id: 60,
            api_key: "ghi".to_string(),
        },
    };

    let _api_guard = ApiKeyGuard::new(api_key_value, context.team_id);
    let _guard = ResolverGuard::new(move |_, _| context.clone());
    let app = test_router(pool.clone());

    let payload = json!({
        "jsonrpc": "2.0",
        "id": 3,
        "method": "tools/call",
        "params": {
            "name": "ping",
            "arguments": {}
        }
    });

    let response = app
        .oneshot(
            axum::http::Request::builder()
                .method("POST")
                .uri(format!("/v1/mcp/{}/{}", slug, connection_id))
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", api_key_value))
                .body(axum::body::Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    handle.abort();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["result"]["output"]["ok"], true);
}

#[tokio::test]
async fn notifications_without_id_are_ignored() {
    let pool = create_test_pool();
    let slug = "test".to_string();
    let connection_id = Uuid::new_v4();
    let app = test_router(pool.clone());

    let api_key_value = "test-notification-key";
    let _api_guard = ApiKeyGuard::new(api_key_value, 77);

    let payload = json!({
        "jsonrpc": "2.0",
        "method": "session.initialize",
        "params": {}
    });

    let response = app
        .oneshot(
            axum::http::Request::builder()
                .method("POST")
                .uri(format!("/v1/mcp/{}/{}", slug, connection_id))
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", api_key_value))
                .body(axum::body::Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn unknown_method_returns_error() {
    let pool = create_test_pool();
    let slug = "test".to_string();
    let connection_id = Uuid::new_v4();
    let spec = sample_spec("http://example.com", &slug);

    let api_key_value = "test-unknown-method-key";

    let context = IntegrationContext {
        definition: spec,
        integration_id: 8,
        user_id: 22,
        team_id: 80,
        user_openid_sub: Some("user-5".to_string()),
        connection: ConnectionAuth::ApiKey {
            connection_id: 75,
            api_key: "mno".to_string(),
        },
    };

    let _api_guard = ApiKeyGuard::new(api_key_value, context.team_id);
    let _guard = ResolverGuard::new(move |_, _| context.clone());
    let app = test_router(pool.clone());

    let payload = json!({
        "jsonrpc": "2.0",
        "id": "abc",
        "method": "unknown.method",
        "params": {}
    });

    let response = app
        .oneshot(
            axum::http::Request::builder()
                .method("POST")
                .uri(format!("/v1/mcp/{}/{}", slug, connection_id))
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", api_key_value))
                .body(axum::body::Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["error"]["code"], -32601);
    assert_eq!(json["error"]["message"], "Unknown method: unknown.method");
}

#[tokio::test]
async fn tools_call_with_invalid_params_returns_error() {
    let pool = create_test_pool();
    let slug = "test".to_string();
    let connection_id = Uuid::new_v4();
    let spec = sample_spec("http://example.com", &slug);

    let api_key_value = "test-invalid-params-key";

    let context = IntegrationContext {
        definition: spec,
        integration_id: 5,
        user_id: 18,
        team_id: 84,
        user_openid_sub: Some("user-6".to_string()),
        connection: ConnectionAuth::ApiKey {
            connection_id: 81,
            api_key: "pqr".to_string(),
        },
    };

    let _api_guard = ApiKeyGuard::new(api_key_value, context.team_id);
    let _guard = ResolverGuard::new(move |_, _| context.clone());
    let app = test_router(pool.clone());

    let payload = json!({
        "jsonrpc": "2.0",
        "id": 9,
        "method": "tools/call",
        "params": {}
    });

    let response = app
        .oneshot(
            axum::http::Request::builder()
                .method("POST")
                .uri(format!("/v1/mcp/{}/{}", slug, connection_id))
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", api_key_value))
                .body(axum::body::Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["error"]["code"], -32602);
    assert_eq!(
        json["error"]["message"],
        "Invalid parameters for tools/call"
    );
}
