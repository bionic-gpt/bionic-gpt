use crate::user_config::UserConfig;
use axum::http::HeaderMap;
use axum::response::Redirect;
use axum::Form;
use axum_extra::extract::cookie::{Cookie, CookieJar};
use serde::Deserialize;
use web_pages::routes::console::SetTools;

#[derive(Deserialize, Default, Debug)]
pub struct ToolsForm {
    #[serde(default)]
    tools: Vec<String>, // Names of selected tools
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
        enabled_tools: Some(form.tools),
    };

    let cookie = Cookie::new(
        "user_config",
        serde_json::to_string(&updated_config).unwrap(),
    );
    let updated_jar = jar.add(cookie);

    // Get the referer header or default to the root path
    let referer = headers
        .get("referer")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("/");

    (updated_jar, Redirect::to(referer))
}
