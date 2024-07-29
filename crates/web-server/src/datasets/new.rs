use super::super::{Authentication, CustomError};
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
use web_pages::routes::datasets::New;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct NewDataset {
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

pub async fn new(
    New { team_id }: New,
    Extension(pool): Extension<Pool>,
    current_user: Authentication,
    Form(new_dataset): Form<NewDataset>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let _permissions = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    // There's only 1 currently so just select it.
    let chunking_strategy = ChunkingStrategy::ByTitle;

    let visibility = if new_dataset.visibility == "Private" {
        Visibility::Private
    } else {
        Visibility::Team
    };

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
