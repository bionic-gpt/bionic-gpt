use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Path},
    response::IntoResponse,
};
use db::queries::conversations;
use db::Pool;

pub async fn new_chat(
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
    Path(team_id): Path<i32>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    let conversation_id = conversations::create_conversation()
        .bind(&transaction, &team_id)
        .one()
        .await?;

    transaction.commit().await?;

    crate::layout::redirect(&ui_components::routes::console::conversation_route(
        team_id,
        conversation_id,
    ))
}
