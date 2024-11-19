use crate::config::Config;

use super::super::{CustomError, Jwt};
use axum::{
    extract::{Extension, Form},
    response::IntoResponse,
};
use db::authz;
use db::types::public::ChunkingStrategy;
use db::Pool;
use db::{queries, Visibility};
use serde::Deserialize;
use validator::Validate;
use web_pages::{routes::datasets::Upsert, string_to_visibility};

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

pub async fn upsert(
    Upsert { team_id }: Upsert,
    Extension(pool): Extension<Pool>,
    Extension(config): Extension<Config>,
    current_user: Jwt,
    Form(new_dataset): Form<NewDataset>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let permissions = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    // There's only 1 currently so just select it.
    let chunking_strategy = ChunkingStrategy::ByTitle;

    let mut visibility = string_to_visibility(&new_dataset.visibility);

    // Is someone trying to override the permitted form values?
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

            super::super::layout::redirect_and_snackbar(
                &web_pages::routes::datasets::Index { team_id }.to_string(),
                "Dataset Updated",
            )
        }
        (Ok(_), None) => {
            let dataset_id = queries::datasets::insert()
                .bind(
                    &transaction,
                    &team_id,
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

            super::super::layout::redirect_and_snackbar(
                &web_pages::routes::documents::Index {
                    team_id,
                    dataset_id,
                }
                .to_string(),
                "Dataset Created",
            )
        }
        (Err(_), _) => super::super::layout::redirect_and_snackbar(
            &web_pages::routes::datasets::Index { team_id }.to_string(),
            "Dataset Updated",
        ),
    }
}
