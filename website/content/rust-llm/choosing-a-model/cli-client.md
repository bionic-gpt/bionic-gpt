+++
title = "A Rust LLM CLI client"
description = "T"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 40
sort_by = "weight"


[extra]
toc = true
top = false
section = "rust-llm"
+++

Now we have a LLM running locally with an API we build a small CLI client as a proof of concept.

Firstly, thanks to [openai-api-rs](https://github.com/dongri/openai-api-rs) for adding [this](https://github.com/dongri/openai-api-rs/issues/18) feature to allow us to use their crate on local LLM's.

## Create a project

Run the following

`cargo new llm-cli`

then a quick sanity check

```sh
$ cargo run --bin llm-cli   
Compiling llm-cli v0.1.0 (/workspace/crates/llm-cli)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/llm-cli`
Hello, world!
```

Add the `openai-api-rs` library

```sh
$ cd llm-cli
$ cargo add openai-api-rs
    Updating crates.io index
      Adding openai-api-rs v0.1.12 to dependencies.
    Updating crates.io index
```

## OpenSSL

I generally don't install OpenSSL and use RustTLS instead. This is in an effort to keep the size of deployment containers down.

However `openai-api-rs` does require OpenSSL so we need the following.

```
sudo apt-get install -y pkg-config
```

## The Code

```rust
use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new_with_endpoint("http://llm-api:8080".to_string(), "NOKEY".to_string());
    let req = ChatCompletionRequest {
        model: "ggml-gpt4all-j".to_string(),
        messages: vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: String::from("What is Bitcoin?"),
            name: None,
            function_call: None,
        }],
        functions: None,
        function_call: None,
        temperature: None,
        top_p: None,
        n: None,
        stream: None,
        stop: None,
        max_tokens: None,
        presence_penalty: None,
        frequency_penalty: None,
        logit_bias: None,
        user: None,
    };
    let result = client.chat_completion(req).await?;
    println!("{:?}", result.choices[0].message.content);
    Ok(())
}
```

## TODO

- There's a problem decoding the response. - https://github.com/dongri/openai-api-rs/issues/20
