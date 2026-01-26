use crate::{CustomError, Jwt};
use axum::{
    extract::{Extension, Form},
    response::IntoResponse,
};
use db::authz;
use db::{queries, Pool};
use serde::Deserialize;
use validator::Validate;
use web_pages::routes::categories::{Delete, Upsert};

#[derive(Deserialize, Validate, Default, Debug)]
pub struct CategoryForm {
    pub id: Option<i32>,
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
    pub description: String,
}

pub async fn action_upsert(
    Upsert { team_id }: Upsert,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Form(form): Form<CategoryForm>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let (rbac, _team_id_num) =
        authz::get_permissions_by_slug(&transaction, &current_user.into(), &team_id).await?;

    if !rbac.is_sys_admin {
        return Err(CustomError::Authorization);
    }

    match (form.validate(), form.id) {
        (Ok(_), Some(id)) => {
            queries::categories::update()
                .bind(&transaction, &form.name, &form.description, &id)
                .await?;
            transaction.commit().await?;
            Ok(crate::layout::redirect_and_snackbar(
                &web_pages::routes::categories::Index { team_id }.to_string(),
                "Category Updated",
            )
            .into_response())
        }
        (Ok(_), None) => {
            queries::categories::insert()
                .bind(&transaction, &form.name, &form.description)
                .one()
                .await?;
            transaction.commit().await?;
            Ok(crate::layout::redirect_and_snackbar(
                &web_pages::routes::categories::Index { team_id }.to_string(),
                "Category Created",
            )
            .into_response())
        }
        (Err(_), _) => Ok(crate::layout::redirect_and_snackbar(
            &web_pages::routes::categories::Index { team_id }.to_string(),
            "Category Validation Error",
        )
        .into_response()),
    }
}

pub async fn action_delete(
    Delete { team_id, id }: Delete,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let (rbac, _team_id_num) =
        authz::get_permissions_by_slug(&transaction, &current_user.into(), &team_id).await?;

    if !rbac.is_sys_admin {
        return Err(CustomError::Authorization);
    }

    queries::categories::delete()
        .bind(&transaction, &id)
        .await?;
    transaction.commit().await?;

    crate::layout::redirect_and_snackbar(
        &web_pages::routes::categories::Index { team_id }.to_string(),
        "Category Deleted",
    )
}
