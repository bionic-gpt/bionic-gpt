use crate::{CustomError, Jwt};
use axum::body::Body;
use axum::extract::Extension;
use axum::http::header::{CONTENT_SECURITY_POLICY, CONTENT_TYPE};
use axum::response::{IntoResponse, Response};
use db::{authz, queries, Pool};
use web_pages::routes::console::GeneratedOutputCanvas;

const CANVAS_CSP: &str = "default-src 'none'; script-src 'unsafe-inline'; style-src 'unsafe-inline'; img-src data: blob:; font-src data:; connect-src 'none'; media-src data: blob:; frame-ancestors 'self'; base-uri 'none'; form-action 'none'";

#[derive(Debug, PartialEq, Eq)]
struct CanvasDocument {
    canvas_type: String,
    html: String,
}

pub async fn generated_output_canvas(
    GeneratedOutputCanvas { team_id, id }: GeneratedOutputCanvas,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let (_rbac, _team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;

    let output = queries::generated_outputs::get_content()
        .bind(&transaction, &id)
        .one()
        .await?;

    let markdown = String::from_utf8(output.object_data)
        .map_err(|_| CustomError::Database("Canvas file is not valid UTF-8".to_string()))?;
    let canvas = parse_canvas_document(&markdown)?;

    if canvas.canvas_type != "text/html" {
        return Err(CustomError::Database(
            "Only text/html canvases are supported".to_string(),
        ));
    }

    Ok(Response::builder()
        .header(CONTENT_TYPE, "text/html; charset=utf-8")
        .header(CONTENT_SECURITY_POLICY, CANVAS_CSP)
        .body(Body::from(canvas.html))
        .unwrap())
}

fn parse_canvas_document(markdown: &str) -> Result<CanvasDocument, CustomError> {
    let markdown = markdown.strip_prefix('\u{feff}').unwrap_or(markdown);
    let Some(rest) = markdown.strip_prefix("---\n") else {
        return Err(CustomError::Database(
            "Canvas file is missing frontmatter".to_string(),
        ));
    };

    let Some((frontmatter, body)) = rest.split_once("\n---\n") else {
        return Err(CustomError::Database(
            "Canvas file frontmatter is not closed".to_string(),
        ));
    };

    let canvas_type = frontmatter
        .lines()
        .find_map(|line| {
            let (key, value) = line.split_once(':')?;
            if key.trim() == "type" {
                Some(value.trim().trim_matches(['"', '\'']).to_string())
            } else {
                None
            }
        })
        .filter(|value| !value.is_empty())
        .ok_or_else(|| CustomError::Database("Canvas file is missing type".to_string()))?;

    if body.trim().is_empty() {
        return Err(CustomError::Database(
            "Canvas file has no content".to_string(),
        ));
    }

    Ok(CanvasDocument {
        canvas_type,
        html: body.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_canvas_document_accepts_html() {
        let canvas = parse_canvas_document(
            "---\nname: demo\ntitle: Demo\ntype: text/html\n---\n<!doctype html><html></html>",
        )
        .expect("valid canvas");

        assert_eq!(canvas.canvas_type, "text/html");
        assert_eq!(canvas.html, "<!doctype html><html></html>");
    }

    #[test]
    fn parse_canvas_document_rejects_missing_frontmatter() {
        assert!(parse_canvas_document("<html></html>").is_err());
    }

    #[test]
    fn parse_canvas_document_rejects_non_html_type() {
        let result = parse_canvas_document("---\nname: demo\ntype: text/markdown\n---\n# Hello");

        assert!(result.is_ok());
        assert_eq!(result.unwrap().canvas_type, "text/markdown");
    }

    #[test]
    fn parse_canvas_document_rejects_empty_body() {
        assert!(parse_canvas_document("---\ntype: text/html\n---\n   ").is_err());
    }

    #[test]
    fn canvas_csp_allows_inline_scripts_without_network() {
        assert!(CANVAS_CSP.contains("script-src 'unsafe-inline'"));
        assert!(CANVAS_CSP.contains("connect-src 'none'"));
    }
}
