use crate::types::ToolDefinition;
use rig::tool::{ToolDyn, ToolError};
use rig::wasm_compat::WasmBoxedFuture;
use serde::Deserialize;
use serde_json::{json, Value};

const MAX_HTML_BYTES: usize = 200 * 1024;

#[derive(Debug, Deserialize)]
struct RenderHtmlArgs {
    title: Option<String>,
    html: String,
}

/// A tool that asks the UI to render static HTML as an isolated canvas artifact.
pub struct HtmlCanvasTool;

impl ToolDyn for HtmlCanvasTool {
    fn name(&self) -> String {
        get_tool_definition().name
    }

    fn description(&self) -> String {
        get_tool_definition().description
    }

    fn parameters(&self) -> Value {
        get_tool_definition().parameters
    }

    fn call(&self, args: String) -> WasmBoxedFuture<'_, Result<String, ToolError>> {
        Box::pin(async move {
            let arguments: RenderHtmlArgs =
                serde_json::from_str(&args).map_err(ToolError::JsonError)?;

            let result = execute_render_html(arguments);
            serde_json::to_string(&result).map_err(ToolError::JsonError)
        })
    }
}

pub fn get_tool_definition() -> ToolDefinition {
    ToolDefinition {
        name: "render_html".to_string(),
        description: "Render a static HTML artifact for the user. Use this when a visual layout, table, report, diagram, or mockup is better shown as HTML than plain text. JavaScript is not supported; include all styling inline in the HTML.".to_string(),
        parameters: json!({
            "type": "object",
            "properties": {
                "title": {
                    "type": "string",
                    "description": "Optional short title for the rendered artifact."
                },
                "html": {
                    "type": "string",
                    "description": "A complete static HTML document or fragment. Include CSS inline. Do not include JavaScript."
                }
            },
            "required": ["html"]
        }),
    }
}

fn execute_render_html(arguments: RenderHtmlArgs) -> Value {
    let html = arguments.html.trim();
    if html.is_empty() {
        return json!({"error": "html is required"});
    }

    if arguments.html.len() > MAX_HTML_BYTES {
        return json!({
            "error": format!("html must be at most {} bytes", MAX_HTML_BYTES)
        });
    }

    json!({
        "type": "html_canvas",
        "version": 1,
        "title": arguments
            .title
            .filter(|title| !title.trim().is_empty()),
        "html": arguments.html
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_tool_definition() {
        let tool = get_tool_definition();
        assert_eq!(tool.name, "render_html");
    }

    #[test]
    fn test_renders_html_canvas_payload() {
        let result = execute_render_html(RenderHtmlArgs {
            title: Some("Preview".to_string()),
            html: "<h1>Hello</h1>".to_string(),
        });

        assert_eq!(result["type"], "html_canvas");
        assert_eq!(result["version"], 1);
        assert_eq!(result["title"], "Preview");
        assert_eq!(result["html"], "<h1>Hello</h1>");
    }

    #[test]
    fn test_rejects_empty_html() {
        let result = execute_render_html(RenderHtmlArgs {
            title: None,
            html: "   ".to_string(),
        });

        assert_eq!(result["error"], "html is required");
    }

    #[test]
    fn test_rejects_oversized_html() {
        let result = execute_render_html(RenderHtmlArgs {
            title: None,
            html: "x".repeat(MAX_HTML_BYTES + 1),
        });

        assert!(result["error"]
            .as_str()
            .unwrap_or_default()
            .contains("html must be at most"));
    }
}
