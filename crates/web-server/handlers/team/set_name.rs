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
use web_pages::routes::team::SetName as SetNameRoute;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct SetName {
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
}

pub async fn set_name(
    SetNameRoute { team_id }: SetNameRoute,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Form(set_name): Form<SetName>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let _permissions = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    queries::teams::set_name()
        .bind(&transaction, &set_name.name, &team_id)
        .await?;

    transaction.commit().await?;

    crate::layout::redirect_and_snackbar(
        &web_pages::routes::team::Index { team_id }.to_string(),
        "Team Name Updated",
    )
}
