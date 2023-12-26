use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Form},
    response::IntoResponse,
};
use db::authz;
use db::queries;
use db::Pool;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct DeleteMember {
    pub team_id: i32,
    pub user_id: i32,
}

pub async fn delete(
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
    Form(delete_member): Form<DeleteMember>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let _permissions =
        authz::get_permissions(&transaction, current_user.into(), delete_member.team_id).await?;

    queries::teams::remove_user()
        .bind(&transaction, &delete_member.user_id, &delete_member.team_id)
        .await?;

    transaction.commit().await?;

    crate::layout::redirect_and_snackbar("/app/team", "User Removed")
}
