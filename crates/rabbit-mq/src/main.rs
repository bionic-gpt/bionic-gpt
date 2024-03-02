use std::env;
use std::error::Error;

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Config {
    pub rabbitmq_url: String,
    pub username: String,
    pub password: String,
    pub upload_url: String,
}

impl Config {
    pub fn new() -> Config {
        Config {
            rabbitmq_url: env::var("RABBITMQ_URL").unwrap_or_else(|_| {
                String::from("http://rabbitmq-service.rabbitmq-namespace.svc.cluster.local:15672/api/queues/%2f/bionic-github/get")
            }),
            username: env::var("USERNAME").unwrap_or_else(|_| String::from("admin")),
            password: env::var("PASSWORD").unwrap_or_else(|_| String::from("admin")),
            upload_url: env::var("UPLOAD_URL").unwrap_or_else(|_| {
                String::from("http://bionic-gpt.bionic-gpt.svc.cluster.local:7903/v1/document_upload")
            }),
        }
    }
}
impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Serialize)]
struct RabbitMQRequest {
    count: u32,
    ackmode: String,
    encoding: String,
    truncate: Option<u32>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct RabbitMQResponse {
    exchange: String,
    message_count: i32,
    routing_key: String,
    payload: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::new();
    // RabbitMQ HTTP GET request
    let messages = get_rabbitmq_messages(&config).await?;

    for message in messages {
        println!("Count: {:?}", message.message_count);

        let current_time = SystemTime::now();

        // Calculate the Unix timestamp (number of seconds since the Unix epoch)
        let unix_timestamp = current_time
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_micros();

        // Create a reqwest client
        let client = reqwest::Client::new();

        let parts: Vec<&str> = message.routing_key.split('.').collect();

        // Extract the part before the first '.'
        if let Some(api_key) = parts.first() {
            let file_part = reqwest::multipart::Part::text(message.payload)
                .file_name(format!("{}.txt", unix_timestamp))
                .mime_str("text/plain")
                .unwrap();
            let form = reqwest::multipart::Form::new().part("files", file_part);

            // Send the POST request
            let response = client
                .post(&config.upload_url)
                .header("Authorization", format!("Bearer {}", api_key))
                .multipart(form)
                .send()
                .await?;

            // Handle the response
            if response.status().is_success() {
                println!("Upload successful");
            } else {
                println!("Upload failed with status: {}", response.status());
            }
        } else {
            println!("No api_key found before the first '.'");
        }
    }

    Ok(())
}

async fn get_rabbitmq_messages(config: &Config) -> Result<Vec<RabbitMQResponse>, reqwest::Error> {
    // Create a reqwest client with basic authentication
    let client = reqwest::Client::builder().build()?;

    // Prepare the message body with the required parameters using a struct
    let message_body = RabbitMQRequest {
        count: 50,
        ackmode: "ack_requeue_false".to_string(),
        encoding: "auto".to_string(),
        truncate: Some(50000),
    };

    // RabbitMQ HTTP GET request
    let response = client
        .post(&config.rabbitmq_url)
        .basic_auth(&config.username, Some(&config.password))
        .json(&message_body)
        .send()
        .await?;

    let emitted_at = response.headers().get("emitted_at");

    dbg!(emitted_at);

    let json: Vec<RabbitMQResponse> = response.json().await?;

    Ok(json)
}
