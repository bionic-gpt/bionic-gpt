use reqwest::{Client, Error};
use serde_json::{json, Value};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Create the client
    let client = Client::new();

    // Define the API endpoint
    let url = "https://generativelanguage.googleapis.com/v1beta/chat/completions";

    // Retrieve the API token from the environment variable
    let token = env::var("API_KEY").expect("API_KEY environment variable must be set");

    // Define the JSON payload
    let payload = json!({
        "messages": [
            { "role": "user", "content": "Say hi" }
        ],
        "model": "gemini-1.5-flash"
    });

    // Send the request and handle the response
    let response = client
        .post(url)
        .header("Authorization", &format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await
        .unwrap();

    if response.status().is_success() {
        let body: Value = response.json().await?;
        println!("Response body: {}", body);
    } else {
        eprintln!("Failed with status: {}", response.status());
        let error_body = response.text().await?;
        eprintln!("Error body: {}", error_body);
    }

    // Set up the HTTP request
    let response = ureq::post(url)
        .set("Authorization", &format!("Bearer {}", token))
        .set("Content-Type", "application/json")
        .send_json(payload);

    // Handle the response
    match response {
        Ok(res) if res.status() == 200 => {
            let body: serde_json::Value = res.into_json().unwrap();
            println!("Response body: {}", body);
        }
        Ok(res) => {
            eprintln!("Failed with status: {}", res.status());
            eprintln!("Error body: {}", res.into_string().unwrap());
        }
        Err(err) => {
            eprintln!("Request failed: {}", err);
        }
    }

    Ok(())
}
