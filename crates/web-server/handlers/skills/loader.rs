use crate::{config::Config, locale::Locale, CustomError, Jwt};
use axum::{extract::Extension, response::Html};
use db::authz;
use db::queries::skills;
use db::Pool;
use web_pages::routes::skills::Index;

pub async fn loader(
    Index { team_id }: Index,
    locale: Locale,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Extension(config): Extension<Config>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let (rbac, _team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;

    let skills = skills::skills().bind(&transaction).all().await?;
    let can_set_visibility_to_company = rbac.is_sys_admin && !config.saas;

    let html = web_pages::skills::page::page(
        rbac,
        team_id,
        skills,
        can_set_visibility_to_company,
        locale.as_str(),
    );

    Ok(Html(html))
}
