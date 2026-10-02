use crate::{CustomError, Jwt};
use axum::extract::{Extension, Query};
use axum::response::Html;
use axum::Router;
use axum_extra::routing::RouterExt;
use db::{authz, queries, Pool};
use serde::Deserialize;
use web_pages::routes::model_requests::{Index, View};

const PAGE_SIZE: usize = 50;

#[derive(Debug, Default, Deserialize)]
pub struct RequestListQuery {
    pub before_id: Option<i64>,
}

pub fn routes() -> Router {
    Router::new().typed_get(loader).typed_get(view)
}

pub async fn loader(
    Index { team_id }: Index,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Query(query): Query<RequestListQuery>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let (rbac, team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;

    authorize(&rbac)?;

    let mut requests = queries::model_requests::list()
        .bind(
            &transaction,
            &team_id_num,
            &query.before_id,
            &((PAGE_SIZE + 1) as i64),
        )
        .all()
        .await?;

    let next_before_id = if requests.len() > PAGE_SIZE {
        requests.truncate(PAGE_SIZE);
        requests.last().map(|request| request.id)
    } else {
        None
    };

    Ok(Html(web_pages::model_requests::page(
        team_id,
        rbac,
        requests,
        next_before_id,
    )))
}

pub async fn view(
    View { team_id, id }: View,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let (rbac, team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;

    authorize(&rbac)?;

    let request = queries::model_requests::detail()
        .bind(&transaction, &team_id_num, &id)
        .one()
        .await?;

    Ok(Html(web_pages::model_requests::detail_page(
        team_id, rbac, request,
    )))
}

fn authorize(rbac: &authz::Rbac) -> Result<(), CustomError> {
    if rbac.can_setup_models() || rbac.is_sys_admin {
        Ok(())
    } else {
        Err(CustomError::Authorization)
    }
}
