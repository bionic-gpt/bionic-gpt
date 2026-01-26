use crate::{CustomError, Jwt};
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
pub struct DeleteInviteForm {
    pub team_id: String,
    pub invite_id: i32,
}

pub async fn delete(
    DeleteInvite { team_id }: DeleteInvite,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Form(delete_invite): Form<DeleteInviteForm>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let (permissions, team_id_num) =
        authz::get_permissions_by_slug(&transaction, &current_user.into(), &delete_invite.team_id)
            .await?;

    if permissions.can_make_invitations() {
        queries::invitations::delete()
            .bind(&transaction, &delete_invite.invite_id, &team_id_num)
            .await?;
    }

    transaction.commit().await?;

    crate::layout::redirect_and_snackbar(&Index { team_id }.to_string(), "Invite Deleted")
}
