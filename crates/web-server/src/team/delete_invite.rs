use super::super::{Authentication, CustomError};
use axum::{
    extract::{Extension, Form},
    response::IntoResponse,
};
use db::authz;
use db::queries;
use db::Pool;
use serde::Deserialize;
use validator::Validate;
use web_pages::routes::team::{DeleteInvite, Index};

#[derive(Deserialize, Validate, Default, Debug)]
pub struct DeleteMember {
    pub team_id: i32,
    pub user_id: i32,
}

pub async fn delete(
    DeleteInvite { team_id }: DeleteInvite,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
    Form(delete_member): Form<DeleteMember>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let _permissions =
        authz::get_permissions(&transaction, &current_user.into(), delete_member.team_id).await?;

    queries::teams::remove_user()
        .bind(&transaction, &delete_member.user_id, &delete_member.team_id)
        .await?;

    transaction.commit().await?;

    super::super::layout::redirect_and_snackbar(&Index { team_id }.to_string(), "User Removed")
}
