use axum::response::Redirect;
use axum::Form;
use axum_extra::extract::cookie::{Cookie, CookieJar};
use serde::Deserialize;
use web_pages::routes::console::SetPrompt;

use crate::user_config::UserConfig;

#[derive(Deserialize, Default, Debug)]
pub struct IdForm {
    id: i32,
    redirect: String,
}

pub async fn set_default_prompt(
    SetPrompt {}: SetPrompt,
    _config: UserConfig,
    jar: CookieJar,
    Form(form): Form<IdForm>,
) -> (CookieJar, Redirect) {
    let updated_config = UserConfig {
        default_prompt: Some(form.id),
    };

    let cookie = Cookie::new(
        "user_config",
        serde_json::to_string(&updated_config).unwrap(),
    );
    let updated_jar = jar.add(cookie);

    (updated_jar, Redirect::to(&form.redirect))
}
