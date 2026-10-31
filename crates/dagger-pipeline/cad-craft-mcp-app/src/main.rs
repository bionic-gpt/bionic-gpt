use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
};
use serde_json::{Value, json};
use std::sync::Arc;
use tower_http::{cors::CorsLayer, services::ServeDir};
const URI: &str = "ui://cadcraft/editor.html";
const MIME: &str = "text/html;profile=mcp-app";
struct App {
    html: String,
    origin: String,
}
async fn rpc(State(app): State<Arc<App>>, Json(request): Json<Value>) -> Response {
    let Some(id) = request.get("id") else {
        return StatusCode::ACCEPTED.into_response();
    };
    let result = match request["method"].as_str().unwrap_or("") {
        "initialize" => json!({"protocolVersion":"2025-03-26","capabilities":{"tools":{},"resources":{}},"serverInfo":{"name":"CADCraft","version":"0.1.0"}}),
        "ping" => json!({}),
        "tools/list" => json!({"tools":[{"name":"open_cadcraft","description":"Open the interactive CADCraft editor","inputSchema":{"type":"object","properties":{},"additionalProperties":false},"_meta":{"ui":{"resourceUri":URI}}}]}),
        "tools/call" if request["params"]["name"] == "open_cadcraft" => json!({"content":[{"type":"text","text":"CADCraft editor opened"}]}),
        "resources/list" => json!({"resources":[{"uri":URI,"name":"CADCraft editor","mimeType":MIME}]}),
        "resources/read" if request["params"]["uri"] == URI => json!({"contents":[{"uri":URI,"mimeType":MIME,"text":app.html,"_meta":{"ui":{"csp":{"resourceDomains":[app.origin],"connectDomains":[app.origin]}}}}]}),
        _ => return Json(json!({"jsonrpc":"2.0","id":id,"error":{"code":-32601,"message":"Unknown method or resource"}})).into_response(),
    };
    Json(json!({"jsonrpc":"2.0","id":id,"result":result})).into_response()
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base = url::Url::parse(&std::env::var("PUBLIC_BASE_URL")?)?;
    if !matches!(base.scheme(), "http" | "https")
        || base.host_str().is_none()
        || base.query().is_some()
        || base.fragment().is_some()
        || !base.username().is_empty()
        || base.password().is_some()
    {
        return Err("Invalid PUBLIC_BASE_URL".into());
    }
    let assets = base.join("assets/")?.to_string();
    let html = tokio::fs::read_to_string("/app/assets/index.html")
        .await?
        .replace("./", &assets);
    let state = Arc::new(App {
        html,
        origin: base.origin().ascii_serialization(),
    });
    let router = Router::new()
        .route("/mcp", post(rpc))
        .route("/healthz", get(|| async { "ok" }))
        .route(
            "/",
            get(|State(app): State<Arc<App>>| async move { Html(app.html.clone()) }),
        )
        .nest_service("/assets", ServeDir::new("/app/assets"))
        .layer(CorsLayer::permissive())
        .with_state(state);
    axum::serve(tokio::net::TcpListener::bind("0.0.0.0:8080").await?, router).await?;
    Ok(())
}
