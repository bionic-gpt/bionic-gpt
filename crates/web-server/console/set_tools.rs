use axum::http::HeaderMap;
use axum::response::Redirect;
use axum::Form;
use axum_extra::extract::cookie::{Cookie, CookieJar};
use llm_proxy::user_config::{create_user_config_cookie, UserConfig};
use serde::Deserialize;
use web_pages::routes::console::SetTools;

#[derive(Deserialize, Debug)]
pub struct ToolsForm {
    #[serde(default)]
    tools: ToolsData,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum ToolsData {
    Single(String),
    Multiple(Vec<String>),
    None(()),
}

impl Default for ToolsData {
    fn default() -> Self {
        ToolsData::None(())
    }
}

impl Default for ToolsForm {
    fn default() -> Self {
        ToolsForm {
            tools: ToolsData::None(()),
        }
    }
}

impl ToolsForm {
    pub fn get_tools(self) -> Vec<String> {
        match self.tools {
            ToolsData::Single(tool) => vec![tool],
            ToolsData::Multiple(tools) => tools,
            ToolsData::None(()) => vec![],
        }
    }
}

pub async fn set_tools(
    SetTools {}: SetTools,
    config: UserConfig,
    jar: CookieJar,
    headers: HeaderMap,
    Form(form): Form<ToolsForm>,
) -> (CookieJar, Redirect) {
    let updated_config = UserConfig {
        default_prompt: config.default_prompt,
        enabled_tools: Some(form.get_tools()),
    };

    // Create a cookie with root path so it's accessible from any path
    let cookie = match create_user_config_cookie(&updated_config) {
        Ok(c) => c,
        Err(_) => {
            // Fallback to the old way if serialization fails
            let mut cookie = Cookie::new(
                "user_config",
                serde_json::to_string(&updated_config).unwrap(),
            );
            cookie.set_path("/"); // Set root path
            cookie
        }
    };

    let updated_jar = jar.add(cookie);

    // Get the referer header or default to the root path
    let referer = headers
        .get("referer")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("/");

    (updated_jar, Redirect::to(referer))
}
