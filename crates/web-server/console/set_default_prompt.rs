use axum::http::HeaderMap;
use axum::response::Redirect;
use axum::Form;
use axum_extra::extract::cookie::{Cookie, CookieJar};
use llm_proxy::user_config::{create_user_config_cookie, UserConfig};
use serde::Deserialize;
use web_pages::routes::console::SetPrompt;

#[derive(Deserialize, Default, Debug)]
pub struct IdForm {
    id: i32,
}

pub async fn set_default_prompt(
    SetPrompt {}: SetPrompt,
    config: UserConfig,
    jar: CookieJar,
    headers: HeaderMap,
    Form(form): Form<IdForm>,
) -> (CookieJar, Redirect) {
    let updated_config = UserConfig {
        default_prompt: Some(form.id),
        enabled_tools: config.enabled_tools,
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
