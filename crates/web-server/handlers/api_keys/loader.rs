use crate::{CustomError, Jwt};
use axum::extract::Extension;
use axum::response::Html;
use db::authz;
use db::Pool;
use web_pages::{api_keys, routes::api_keys::Index};

pub async fn loader(
    Index { team_id }: Index,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let (rbac, team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;

    if !rbac.can_use_api_keys() {
        return Err(CustomError::Authorization);
    }

    let page_data = super::page_data::load_api_keys_page_data(&transaction, team_id_num).await?;

    let html = api_keys::page::page(
        rbac,
        team_id,
        page_data.api_keys,
        page_data.assistants,
        page_data.models,
        page_data.token_usage_data,
        page_data.api_request_data,
        None,
    );

    Ok(Html(html))
}
