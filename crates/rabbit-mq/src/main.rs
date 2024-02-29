use std::error::Error;

const RABBITMQ_URL: &str = "http://host.docker.internal:25672/api/queues/bionic-gpt/get";
const USERNAME: &str = "admin";
const PASSWORD: &str = "admin";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // RabbitMQ HTTP GET request
    let response = get_rabbitmq_messages().await?;

    // Check if the request was successful (status code 200)
    if response.status().is_success() {
        // Parse the JSON response
        let json_response: serde_json::Value = response.json().await?;

        // Print or process the received messages
        println!("Received messages: {:?}", json_response);
    } else {
        // Print the error status if the request was not successful
        println!("Error: {}", response.status());
    }

    Ok(())
}

async fn get_rabbitmq_messages() -> Result<reqwest::Response, reqwest::Error> {
    // Create a reqwest client with basic authentication
    let client = reqwest::Client::builder().build()?;

    // RabbitMQ HTTP GET request
    let response = client
        .get(RABBITMQ_URL)
        .basic_auth(USERNAME, Some(PASSWORD))
        .send()
        .await?;

    Ok(response)
}
