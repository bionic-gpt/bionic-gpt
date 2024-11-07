use axum::{
    http::HeaderMap,
    response::{Html, IntoResponse},
    Router,
};
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

    // Close the HTML tags
    html.push_str("</table></body></html>");

    // Return the HTML response
    Html(html)
}

// Helper function to escape HTML special characters
fn html_escape(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
