use crate::config::Config;
use crate::{CustomError, Jwt};
use axum::{
    extract::{Extension, Form},
    response::IntoResponse,
};
use db::authz;
use db::queries;
use db::types::public::ChunkingStrategy;
use db::{Pool, Visibility};
use serde::Deserialize;
use validator::Validate;
use web_pages::{
    routes::datasets::{Delete, Upsert},
    string_to_visibility,
};

// Delete function
pub async fn action_delete(
    Delete { team_id, id }: Delete,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let (_permissions, _team_id_num) =
        authz::get_permissions_by_slug(&transaction, &current_user.into(), &team_id).await?;

    queries::datasets::delete().bind(&transaction, &id).await?;

    transaction.commit().await?;

    crate::layout::redirect_and_snackbar(
        &web_pages::routes::datasets::Index { team_id }.to_string(),
        "Document Deleted",
    )
}

// Upsert function
#[derive(Deserialize, Validate, Default, Debug)]
pub struct NewDataset {
    pub id: Option<i32>,
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
    #[allow(dead_code)]
    pub chunking_strategy: String,
    pub combine_under_n_chars: i32,
    pub new_after_n_chars: i32,
    pub embeddings_model_id: i32,
    pub visibility: String,
    pub multipage_sections: bool,
}

pub async fn action_upsert(
    Upsert { team_id }: Upsert,
    Extension(pool): Extension<Pool>,
    Extension(config): Extension<Config>,
    current_user: Jwt,
    Form(new_dataset): Form<NewDataset>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let (permissions, team_id_num) =
        authz::get_permissions_by_slug(&transaction, &current_user.into(), &team_id).await?;

    let chunking_strategy = ChunkingStrategy::ByTitle;

    let mut visibility = string_to_visibility(&new_dataset.visibility);

    if visibility == Visibility::Company && config.saas {
        visibility = Visibility::Team;
    }
    if visibility == Visibility::Company && !permissions.is_sys_admin {
        visibility = Visibility::Team;
    }

    match (new_dataset.validate(), new_dataset.id) {
        (Ok(_), Some(id)) => {
            queries::datasets::update()
                .bind(
                    &transaction,
                    &new_dataset.name,
                    &visibility,
                    &new_dataset.embeddings_model_id,
                    &chunking_strategy,
                    &new_dataset.combine_under_n_chars,
                    &new_dataset.new_after_n_chars,
                    &new_dataset.multipage_sections,
                    &id,
                )
                .await?;

            transaction.commit().await?;

            crate::layout::redirect_and_snackbar(
                &web_pages::routes::datasets::Index { team_id }.to_string(),
                "Dataset Updated",
            )
        }
        (Ok(_), None) => {
            let dataset_id = queries::datasets::insert()
                .bind(
                    &transaction,
                    &team_id_num,
                    &new_dataset.name,
                    &new_dataset.embeddings_model_id,
                    &chunking_strategy,
                    &new_dataset.combine_under_n_chars,
                    &new_dataset.new_after_n_chars,
                    &new_dataset.multipage_sections,
                    &visibility,
                )
                .one()
                .await?;

            transaction.commit().await?;

            crate::layout::redirect_and_snackbar(
                &web_pages::routes::documents::Index {
                    team_id,
                    dataset_id,
                }
                .to_string(),
                "Dataset Created",
            )
        }
        (Err(_), _) => crate::layout::redirect_and_snackbar(
            &web_pages::routes::datasets::Index { team_id }.to_string(),
            "Dataset Updated",
        ),
    }
}
