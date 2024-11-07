use axum::{
    http::HeaderMap,
    response::{Html, IntoResponse},
    Router,
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use serde_json::Value;
use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt::init();

    // Build our application with a fallback handler to catch all routes
    let app = Router::new().fallback(handler);

    // Get the port from the environment variable, default to 3000 if not set
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a valid u16 number");

    // Define the address to listen on (0.0.0.0)
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::debug!("Listening on {}", addr);
    println!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

// The handler function that processes all incoming requests
async fn handler(headers: HeaderMap) -> impl IntoResponse {
    // Start building the HTML response
    let mut html = String::from(
        "<html>
            <head>
                <title>Request Headers</title>
                <style>
                    body { font-family: Arial, sans-serif; }
                    table { border-collapse: collapse; width: 50%; }
                    th, td { border: 1px solid #ddd; padding: 8px; }
                    th { background-color: #f2f2f2; }
                    pre { background-color: #f8f8f8; padding: 10px; border: 1px solid #ddd; }
                </style>
            </head>
            <body>
                <h1>Request Headers</h1>
                <table>
                    <tr><th>Name</th><th>Value</th></tr>",
    );

    // Iterate over the headers and append them to the HTML table
    for (key, value) in headers.iter() {
        let name = key.as_str();
        let value = value.to_str().unwrap_or("<invalid UTF-8>");
        html.push_str(&format!(
            "<tr><td>{}</td><td>{}</td></tr>",
            html_escape(name),
            html_escape(value)
        ));
    }

    html.push_str("</table>");

    // Check for x-forwarded-access-token header and parse JWT
    if let Some(token_value) = headers.get("x-forwarded-access-token") {
        if let Ok(token_str) = token_value.to_str() {
            html.push_str("<h2>Parsed JWT (x-forwarded-access-token)</h2>");
            match parse_jwt(token_str) {
                Ok(parsed_jwt_html) => {
                    html.push_str(&parsed_jwt_html);
                }
                Err(e) => {
                    html.push_str(&format!(
                        "<p>Error parsing JWT: {}</p>",
                        html_escape(&e.to_string())
                    ));
                }
            }
        }
    } else {
        html.push_str("<h2>JWT Not Found</h2>");
    }

    // Close the HTML tags
    html.push_str("</body></html>");

    // Return the HTML response
    Html(html)
}

// Function to parse JWT and return HTML representation
fn parse_jwt(token: &str) -> Result<String, Box<dyn std::error::Error>> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err("Invalid JWT format".into());
    }

    let header_b64 = parts[0];
    let payload_b64 = parts[1];
    // The signature is the third part, but we won't decode it in this example

    let header_json = base64_url_decode(header_b64)?;
    let payload_json = base64_url_decode(payload_b64)?;

    let header_value: Value = serde_json::from_str(&header_json)?;
    let payload_value: Value = serde_json::from_str(&payload_json)?;

    let mut html = String::new();

    html.push_str("<h3>Header</h3>");
    html.push_str("<pre>");
    html.push_str(&html_escape(&serde_json::to_string_pretty(&header_value)?));
    html.push_str("</pre>");

    html.push_str("<h3>Payload</h3>");
    html.push_str("<pre>");
    html.push_str(&html_escape(&serde_json::to_string_pretty(&payload_value)?));
    html.push_str("</pre>");

    Ok(html)
}

// Helper function to base64url decode a JWT part
fn base64_url_decode(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let decoded_bytes = URL_SAFE_NO_PAD.decode(input)?;
    let decoded_str = String::from_utf8(decoded_bytes)?;
    Ok(decoded_str)
}

// Helper function to escape HTML special characters
fn html_escape(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
