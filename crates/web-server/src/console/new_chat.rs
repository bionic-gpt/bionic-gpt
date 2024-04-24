use super::super::{Authentication, CustomError};
use axum::{
    extract::{Extension, Path},
    response::IntoResponse,
};
use db::authz;
use db::queries::conversations;
use db::Pool;

pub async fn new_chat(
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
    Path(team_id): Path<i32>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let _permissions = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let conversation_id = conversations::create_conversation()
        .bind(&transaction, &team_id)
        .one()
        .await?;

    transaction.commit().await?;

    super::super::layout::redirect(&web_pages::routes::console::conversation_route(
        team_id,
        conversation_id,
    ))
}
