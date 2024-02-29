use std::env;
use std::error::Error;

use serde::{Deserialize, Serialize};

pub struct Config {
    pub rabbitmq_url: String,
    pub username: String,
    pub password: String,
}

impl Config {
    pub fn new() -> Config {
        Config {
            rabbitmq_url: env::var("RABBITMQ_URL").unwrap_or_else(|_| {
                String::from("http://rabbitmq-service.rabbitmq-namespace.svc.cluster.local:15672/api/queues/%2f/bionic-github/get")
            }),
            username: env::var("USERNAME").unwrap_or_else(|_| String::from("admin")),
            password: env::var("PASSWORD").unwrap_or_else(|_| String::from("admin")),
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

#[derive(Debug, Deserialize)]
struct RabbitMQResponse {
    exchange: String,
    message_count: i32,
    payload: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::new();
    // RabbitMQ HTTP GET request
    let messages = get_rabbitmq_messages(&config).await?;

    for message in messages {
        println!("Exchnage: {:?}", message.exchange);
        println!("Count: {:?}", message.message_count);
        println!("Payload: {:?}", message.payload);
    }

    Ok(())
}

async fn get_rabbitmq_messages(config: &Config) -> Result<Vec<RabbitMQResponse>, reqwest::Error> {
    // Create a reqwest client with basic authentication
    let client = reqwest::Client::builder().build()?;

    // Prepare the message body with the required parameters using a struct
    let message_body = RabbitMQRequest {
        count: 1,
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

    let json: Vec<RabbitMQResponse> = response.json().await?;

    Ok(json)
}
