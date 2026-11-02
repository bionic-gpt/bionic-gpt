mod backend;

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
};
use serde_json::{Value, json};
use std::sync::Arc;
use tower_http::services::ServeDir;
const URI: &str = "ui://cadcraft/editor.html";
const MIME: &str = "text/html;profile=mcp-app";
struct App {
    html: String,
    manifest: Value,
    tools: Value,
    drawings: backend::Drawings,
}

fn tool_error(error: impl ToString) -> Value {
    json!({"isError":true,"content":[{"type":"text","text":error.to_string()}]})
}

fn bound_snapshot(result: &mut Value) {
    // Bionic's MCP client accepts at most 8 MiB, including the JSON-RPC envelope.
    if serde_json::to_vec(result).is_ok_and(|bytes| bytes.len() > 7 * 1024 * 1024)
        && let Some(meta) = result.get_mut("_meta").and_then(Value::as_object_mut)
    {
        meta.remove("cadcraft/snapshot");
        meta.insert(
            "cadcraft/snapshotError".into(),
            json!(
                "Preview omitted because the combined tool result exceeds the response size limit"
            ),
        );
    }
}

fn tool_definitions(upstream: Value) -> Value {
    let mut tools = vec![
        json!({"name":"open_cadcraft","description":"Create or reopen a model-driven CADCraft drawing. Reuse its drawing_id for subsequent calls. The embedded view is a historical snapshot; manual edits are disabled. Active sessions expire after 24 hours idle or a server restart.","inputSchema":{"type":"object","properties":{"drawing_id":{"type":"string"}},"additionalProperties":false}}),
    ];
    for mut tool in upstream["tools"].as_array().into_iter().flatten().cloned() {
        if !backend::TOOLS.contains(&tool["name"].as_str().unwrap_or("")) {
            continue;
        }
        tool["inputSchema"]["properties"]["drawing_id"] =
            json!({"type":"string","description":"ID returned by open_cadcraft"});
        if !tool["inputSchema"]["required"].is_array() {
            tool["inputSchema"]["required"] = json!([]);
        }
        tool["inputSchema"]["required"]
            .as_array_mut()
            .unwrap()
            .push(json!("drawing_id"));
        tools.push(tool);
    }
    for tool in &mut tools {
        tool["_meta"] = json!({"ui":{"resourceUri":URI},"bionic/conversationScope":true});
    }
    json!({"tools":tools})
}

async fn call(app: &App, params: &Value) -> Result<Value, String> {
    let owner = params
        .pointer("/_meta/bionic~1conversation")
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty())
        .ok_or("Authenticated conversation context required; use Bionic's model tool call")?;
    let name = params["name"].as_str().ok_or("Missing tool name")?;
    let mut arguments = params
        .get("arguments")
        .cloned()
        .unwrap_or_else(|| json!({}));
    if !arguments.is_object() {
        return Err("Tool arguments must be an object".into());
    }
    let drawing_id = arguments.as_object_mut().unwrap().remove("drawing_id");
    let requested_id = drawing_id
        .as_ref()
        .map(|id| {
            id.as_str()
                .filter(|s| !s.is_empty())
                .ok_or("drawing_id must be a nonempty string")
        })
        .transpose()?;
    let (id, drawing) = if name == "open_cadcraft" {
        if !arguments.as_object().unwrap().is_empty() {
            return Err("Unexpected open_cadcraft argument".into());
        }
        app.drawings.open(owner, requested_id).await?
    } else {
        if !backend::TOOLS.contains(&name) {
            return Err("Unknown drawing tool".into());
        }
        let id = requested_id.ok_or("drawing_id required; call open_cadcraft first")?;
        (id.to_owned(), app.drawings.get(owner, id).await?)
    };
    let mut drawing = drawing.lock().await;
    let mut result = if name == "open_cadcraft" {
        json!({"content":[{"type":"text","text":format!("CADCraft drawing opened. Use drawing_id {id} for subsequent commands.")}]})
    } else {
        drawing.backend.tool(name, arguments).await?
    };
    drawing.revision += 1;
    // Export even on a refused script: preceding inputs may already have changed geometry.
    match drawing.backend.snapshot().await {
        Ok(data) => {
            result["_meta"]["cadcraft/snapshot"] = json!({"version":1,"drawing_id":id,"revision":drawing.revision,"name":"Drawing.dxf","data":data})
        }
        Err(error) => result["_meta"]["cadcraft/snapshotError"] = json!(error),
    }
    result["structuredContent"] = json!({"drawing_id":id,"revision":drawing.revision});
    bound_snapshot(&mut result);
    Ok(result)
}

async fn rpc(
    State(app): State<Arc<App>>,
    Json(request): Json<Value>,
) -> Response {
    let Some(id) = request.get("id") else {
        return StatusCode::ACCEPTED.into_response();
    };
    let result = match request["method"].as_str().unwrap_or("") {
        "initialize" => json!({"protocolVersion":"2025-06-18","capabilities":{"tools":{},"resources":{}},"serverInfo":{"name":"CADCraft","version":"0.2.0"}}),
        "ping" => json!({}),
        "tools/list" => app.tools.clone(),
        "tools/call" => match call(&app, &request["params"]).await { Ok(result) => result, Err(error) => tool_error(error) },
        "resources/list" => json!({"resources":[{"uri":URI,"name":"CADCraft viewer","mimeType":MIME}]}),
        "resources/read" if request["params"]["uri"] == URI => json!({"contents":[{"uri":URI,"mimeType":MIME,"text":app.html,"_meta":{"bionic/assets":app.manifest}}]}),
        _ => return Json(json!({"jsonrpc":"2.0","id":id,"error":{"code":-32601,"message":"Unknown method or resource"}})).into_response(),
    };
    Json(json!({"jsonrpc":"2.0","id":id,"result":result})).into_response()
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().nth(1).as_deref() == Some("--worker") {
        return backend::worker(
            std::env::args()
                .nth(2)
                .ok_or("Missing worker directory")?
                .into(),
            std::env::args()
                .nth(3)
                .ok_or("Missing worker CLI path")?
                .into(),
        );
    }
    let mut catalog = backend::Backend::start().await?;
    let tools = tool_definitions(catalog.request("tools/list", json!({})).await?);
    drop(catalog);
    let assets = std::env::var("CADCRAFT_ASSETS").unwrap_or_else(|_| "/app/assets".into());
    let mut files = Vec::new();
    let mut directory = tokio::fs::read_dir(&assets).await?;
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
    let mut html =
        tokio::fs::read_to_string(std::path::Path::new(&assets).join("index.html")).await?;
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
        tools,
        drawings: backend::Drawings::default(),
    });
    let cleanup = state.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
        loop {
            interval.tick().await;
            cleanup.drawings.reap().await;
        }
    });
    let router = Router::new()
        .route("/mcp", post(rpc))
        .route("/healthz", get(|| async { "ok" }))
        .route(
            "/",
            get(|State(app): State<Arc<App>>| async move { Html(app.html.clone()) }),
        )
        .nest_service("/assets", ServeDir::new(&assets))
        .with_state(state);
    axum::serve(
        tokio::net::TcpListener::bind(
            std::env::var("CADCRAFT_LISTEN").unwrap_or_else(|_| "0.0.0.0:8080".into()),
        )
        .await?,
        router,
    )
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app() -> App {
        App {
            html: String::new(),
            manifest: json!({}),
            tools: json!({}),
            drawings: backend::Drawings::default(),
        }
    }

    #[tokio::test]
    async fn context_cannot_be_supplied_as_tool_arguments() {
        let result = call(
            &app(),
            &json!({"name":"open_cadcraft","arguments":{"_meta":{"bionic/conversation":"forged"}}}),
        )
        .await;
        assert!(result.unwrap_err().contains("Authenticated conversation"));
    }

    #[tokio::test]
    async fn drawing_ids_do_not_cross_conversations() {
        let result = call(&app(), &json!({"name":"inspect_drawing","arguments":{"drawing_id":"another-conversation"},"_meta":{"bionic/conversation":"owner:2"}})).await;
        assert!(
            result
                .unwrap_err()
                .contains("unavailable in this conversation")
        );
    }

    #[test]
    fn upstream_tools_keep_their_schema_and_require_a_drawing() {
        let defs = tool_definitions(json!({"tools":[
            {"name":"command_line","inputSchema":{"type":"object","properties":{"text":{"type":"string"}},"required":["text"]}},
            {"name":"open","inputSchema":{}},
            {"name":"inspect_drawing","inputSchema":{"type":"object","properties":{}}}
        ]}));
        assert_eq!(defs["tools"].as_array().unwrap().len(), 3);
        assert_eq!(
            defs["tools"][1]["inputSchema"]["required"],
            json!(["text", "drawing_id"])
        );
        assert_eq!(
            defs["tools"][2]["inputSchema"]["required"],
            json!(["drawing_id"])
        );
        assert_eq!(defs["tools"][1]["_meta"]["bionic/conversationScope"], true);
    }

    #[test]
    fn large_results_keep_the_command_output_when_preview_would_exceed_client_limit() {
        let content = "x".repeat(4 * 1024 * 1024);
        let mut result = json!({"content":content,"_meta":{"cadcraft/snapshot":{"data":content}}});
        bound_snapshot(&mut result);
        assert!(result["_meta"].get("cadcraft/snapshot").is_none());
        assert!(result["_meta"]["cadcraft/snapshotError"].is_string());
        assert_eq!(result["content"].as_str().unwrap().len(), content.len());
    }
}
