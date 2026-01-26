use crate::{locale::Locale, CustomError, Jwt};
use axum::extract::Extension;
use axum::response::Html;
use db::{authz, queries, Pool};
use web_pages::{my_assistants, routes::prompts::MyAssistants};

pub async fn my_assistants(
    MyAssistants { team_id }: MyAssistants,
    locale: Locale,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let (rbac, _team_id_num) =
        authz::get_permissions_by_slug(&transaction, &current_user.into(), &team_id).await?;

    let prompts = queries::prompts::my_prompts()
        .bind(&transaction, &db::PromptType::Assistant)
        .all()
        .await?;

    let i18n = db::i18n::global();
    i18n.ensure_locale("en").await;
    if locale.as_str() != "en" {
        i18n.ensure_locale(locale.as_str()).await;
    }

    let html = my_assistants::page::page(team_id, rbac, prompts, locale.as_str());

    Ok(Html(html))
}
