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
