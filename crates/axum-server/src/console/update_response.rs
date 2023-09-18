use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Form, Path},
    response::IntoResponse,
};
use db::queries::chats;
use db::Pool;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct Chat {
    pub response: String,
    pub chat_id: i32,
}

pub async fn update_response(
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
    Path(team_id): Path<i32>,
    Form(message): Form<Chat>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    if message.validate().is_ok() {
        chats::update_chat()
            .bind(&transaction, &message.response, &message.chat_id)
            .await?;
    }

    transaction.commit().await?;

    crate::layout::redirect(&ui_components::routes::console::index_route(team_id))
}
