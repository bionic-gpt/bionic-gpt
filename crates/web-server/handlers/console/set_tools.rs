use axum::body::Bytes;
use axum::http::HeaderMap;
use axum::response::Redirect;
use axum_extra::extract::cookie::{Cookie, CookieJar};
use llm_proxy::user_config::{create_user_config_cookie, UserConfig};
use web_pages::routes::console::SetTools;

pub async fn set_tools(
    SetTools {}: SetTools,
    config: UserConfig,
    jar: CookieJar,
    headers: HeaderMap,
    body: Bytes,
) -> (CookieJar, Redirect) {
    // parse into a Vec of (key, value) pairs
    let pairs: Vec<(String, String)> =
        serde_urlencoded::from_bytes(&body).expect("invalid form data");

    // collect all the "tools" values into a Vec<String>
    let tools: Vec<String> = pairs
        .into_iter()
        .filter(|(k, _)| k == "tools")
        .map(|(_, v)| v)
        .collect();

    let updated_config = UserConfig {
        default_prompt: config.default_prompt,
        enabled_tools: Some(tools),
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
