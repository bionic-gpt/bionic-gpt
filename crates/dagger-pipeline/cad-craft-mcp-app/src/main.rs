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
    manifest: Value,
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
        "resources/read" if request["params"]["uri"] == URI => json!({"contents":[{"uri":URI,"mimeType":MIME,"text":app.html,"_meta":{"bionic/assets":app.manifest}}]}),
        _ => return Json(json!({"jsonrpc":"2.0","id":id,"error":{"code":-32601,"message":"Unknown method or resource"}})).into_response(),
    };
    Json(json!({"jsonrpc":"2.0","id":id,"result":result})).into_response()
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut files = Vec::new();
    let mut directory = tokio::fs::read_dir("/app/assets").await?;
    while let Some(entry) = directory.next_entry().await? {
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.ends_with(".js") || name.ends_with(".wasm") {
            files.push(name);
        }
    }
    let js = files
        .iter()
        .find(|f| f.ends_with(".js") && f.as_str() != "mcp-bridge.js")
        .ok_or("Missing generated JS")?;
    let wasm = files
        .iter()
        .find(|f| f.ends_with(".wasm"))
        .ok_or("Missing generated WASM")?;
    let mut html = tokio::fs::read_to_string("/app/assets/index.html").await?;
    while let Some(start) = html.find("<script") {
        let end = html[start..]
            .find("</script>")
            .ok_or("Invalid build HTML")?
            + start
            + 9;
        html.replace_range(start..end, "");
    }
    while let Some(start) = html.find("<link") {
        let end = html[start..].find('>').ok_or("Invalid build HTML link")? + start + 1;
        html.replace_range(start..end, "");
    }
    let loader = include_str!("loader.js")
        .replace("__JS__", js)
        .replace("__WASM__", wasm);
    html = html.replace(
        "</body>",
        &format!("<script type=\"module\">{loader}</script></body>"),
    );
    let state = Arc::new(App {
        html,
        manifest: json!({"files":files}),
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
