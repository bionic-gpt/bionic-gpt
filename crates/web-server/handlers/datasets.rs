// Consolidated datasets.rs

use axum::{
    extract::{Extension, Form},
    response::{Html, IntoResponse},
    Router,
};
use axum_extra::routing::RouterExt;
use db::authz;
use db::queries::{self, datasets, models};
use db::types::public::ChunkingStrategy;
use db::{ModelType, Pool, Visibility};
use serde::Deserialize;
use validator::Validate;
use web_pages::{
    routes::datasets::{Delete, Index, Upsert},
    string_to_visibility,
};

use crate::config::Config;
use crate::{CustomError, Jwt};

// Router setup
pub fn routes() -> Router {
    Router::new()
        .typed_get(loader)
        .typed_post(upsert_action)
        .typed_post(delete_action)
}

// Index function
pub async fn loader(
    Index { team_id }: Index,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let datasets = datasets::datasets().bind(&transaction).all().await?;

    let models = models::models()
        .bind(&transaction, &ModelType::Embeddings)
        .all()
        .await?;

    let can_set_visibility_to_company = rbac.is_sys_admin;

    let html = web_pages::datasets::index::page(
        rbac,
        team_id,
        datasets,
        models,
        can_set_visibility_to_company,
    );

    Ok(Html(html))
}

// Delete function
pub async fn delete_action(
    Delete { team_id, id }: Delete,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let _permissions = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

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

pub async fn upsert_action(
    Upsert { team_id }: Upsert,
    Extension(pool): Extension<Pool>,
    Extension(config): Extension<Config>,
    current_user: Jwt,
    Form(new_dataset): Form<NewDataset>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let permissions = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

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
