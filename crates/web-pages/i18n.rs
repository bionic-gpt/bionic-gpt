use db::Licence;

pub fn ai_assistants() -> &'static str {
    match Licence::global().default_lang.as_str() {
        "en-US" => "MCP Servers",
        _ => "AI Assistants",
    }
}

pub fn integrations() -> &'static str {
    match Licence::global().default_lang.as_str() {
        "en-US" => "MCP Servers",
        _ => "Integrations",
    }
}

pub fn integration() -> &'static str {
    match Licence::global().default_lang.as_str() {
        "en-US" => "MCP Server",
        _ => "Integration",
    }
}

pub fn prompts() -> &'static str {
    match Licence::global().default_lang.as_str() {
        "en-US" => "MCP Playground",
        _ => "Explore Assistants",
    }
}

pub fn datasets() -> &'static str {
    match Licence::global().default_lang.as_str() {
        "en-US" => "MCP RAG Servers",
        _ => "Datasets & Documents",
    }
}

pub fn assistants() -> &'static str {
    match Licence::global().default_lang.as_str() {
        "en-US" => "Playgrounds",
        _ => "Assistants",
    }
}

pub fn assistant() -> &'static str {
    match Licence::global().default_lang.as_str() {
        "en-US" => "Playground",
        _ => "Assistant",
    }
}

pub fn dataset() -> &'static str {
    match Licence::global().default_lang.as_str() {
        "en-US" => "MCP RAG Server",
        _ => "Dataset",
    }
}
