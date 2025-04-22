use openai::Credentials;
use openai::chat::{
    ChatCompletion, ChatCompletionDelta, ChatCompletionFunctionDefinition, ChatCompletionMessage,
    ChatCompletionMessageRole, ToolCall,
};
use serde_json::json;
use std::io::{Write, stdin, stdout};
use tokio::sync::mpsc::{Receiver, error::TryRecvError};

#[tokio::main]
async fn main() {
    let credentials = Credentials::from_env();

    let mut messages = vec![ChatCompletionMessage {
        role: ChatCompletionMessageRole::System,
        content: Some("You can call the get_current_time tool.".to_string()),
        ..Default::default()
    }];

    // Define the tool
    let tools = vec![ChatCompletionFunctionDefinition {
        name: "get_current_time".to_string(),
        description: Some("Returns the current UTC time.".to_string()),
        parameters: Some(json!({ "type": "object", "properties": {}, "required": [] })),
    }];

    loop {
        print!("User: ");
        stdout().flush().unwrap();

        let mut user_input = String::new();
        stdin().read_line(&mut user_input).unwrap();
        let user_input = user_input.trim();

        if user_input.is_empty() {
            continue;
        }

        messages.push(ChatCompletionMessage {
            role: ChatCompletionMessageRole::User,
            content: Some(user_input.to_string()),
            ..Default::default()
        });

        // Request a streaming response with tool support
        let chat_stream = ChatCompletionDelta::builder("gpt-4", messages.clone())
            .credentials(credentials.clone())
            .functions(tools.clone())
            .function_call("auto")
            .create_stream()
            .await
            .unwrap();

        let (completion, maybe_tool_call) = listen_for_tokens(chat_stream).await;
        messages.push(completion.choices[0].message.clone());

        if let Some(tool_call) = maybe_tool_call {
            // Simulate tool execution
            let now = chrono::Utc::now().to_rfc3339();
            let tool_result = ChatCompletionMessage {
                role: ChatCompletionMessageRole::Tool,
                tool_call_id: Some(tool_call.id.clone()),
                content: Some(format!("{{ \"current_time\": \"{}\" }}", now)),
                ..Default::default()
            };
            messages.push(tool_result);

            // Ask GPT to continue with tool result
            let final_response = ChatCompletion::builder("gpt-4", messages.clone())
                .credentials(credentials.clone())
                .create()
                .await
                .unwrap();

            if let Some(reply) = final_response
                .choices
                .first()
                .and_then(|c| c.message.content.clone())
            {
                println!("\nAssistant: {}", reply);
                messages.push(final_response.choices[0].message.clone());
            }
        }
    }
}

async fn listen_for_tokens(
    mut stream: Receiver<ChatCompletionDelta>,
) -> (ChatCompletion, Option<ToolCall>) {
    let mut merged: Option<ChatCompletionDelta> = None;
    let mut tool_call: Option<ToolCall> = None;

    loop {
        match stream.try_recv() {
            Ok(delta) => {
                let choice = &delta.choices[0];

                if let Some(content) = &choice.delta.content {
                    print!(">> {}", content);
                    stdout().flush().unwrap();
                }

                if let Some(calls) = &choice.delta.tool_calls {
                    print!(">> {:?}", calls);
                    if let Some(tc) = calls.first() {
                        tool_call = Some(tc.clone());
                    }
                }

                match merged.as_mut() {
                    Some(c) => c.merge(delta).unwrap(),
                    None => merged = Some(delta),
                }
            }
            Err(TryRecvError::Empty) => {
                println!("Empty");
                tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            }
            Err(TryRecvError::Disconnected) => {
                println!("Disconnect");
                break;
            }
        }
    }

    println!();
    (merged.unwrap().into(), tool_call)
}
