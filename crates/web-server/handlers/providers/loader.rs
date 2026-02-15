use crate::{CustomError, Jwt};
use axum::extract::Extension;
use axum::response::Html;
use db::authz;
use db::{queries, Pool};
use web_pages::providers::upsert as provider_page;
use web_pages::routes::providers::{Edit, Index, New};

pub async fn loader(
    Index { team_id }: Index,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let (rbac, _team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;

    if !rbac.is_sys_admin {
        return Err(CustomError::Authorization);
    }

    let providers = queries::providers::providers()
        .bind(&transaction)
        .all()
        .await?;

    let html = web_pages::providers::page::page(team_id, rbac, providers);

    Ok(Html(html))
}

pub async fn new_loader(
    New { team_id }: New,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let (rbac, _team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;

    if !rbac.is_sys_admin {
        return Err(CustomError::Authorization);
    }

    let form = provider_page::ProviderForm {
        id: None,
        name: "".to_string(),
        svg_logo: "".to_string(),
        default_model_name: "".to_string(),
        default_model_display_name: "".to_string(),
        default_model_context_size: 0,
        default_model_description: "".to_string(),
        base_url: "".to_string(),
        api_key_optional: false,
        default_embeddings_model_name: "".to_string(),
        default_embeddings_model_display_name: "".to_string(),
        default_embeddings_model_context_size: 0,
        default_embeddings_model_description: "".to_string(),
        error: None,
    };

    let html = provider_page::page(team_id, rbac, form);

    Ok(Html(html))
}

pub async fn edit_loader(
    Edit { team_id, id }: Edit,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let (rbac, _team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;

    if !rbac.is_sys_admin {
        return Err(CustomError::Authorization);
    }

    let provider = queries::providers::provider()
        .bind(&transaction, &id)
        .one()
        .await?;

    let form = provider_page::ProviderForm {
        id: Some(provider.id),
        name: provider.name,
        svg_logo: provider.svg_logo,
        default_model_name: provider.default_model_name.unwrap_or_default(),
        default_model_display_name: provider.default_model_display_name.unwrap_or_default(),
        default_model_context_size: provider.default_model_context_size,
        default_model_description: provider.default_model_description,
        base_url: provider.base_url,
        api_key_optional: provider.api_key_optional,
        default_embeddings_model_name: provider.default_embeddings_model_name.unwrap_or_default(),
        default_embeddings_model_display_name: provider
            .default_embeddings_model_display_name
            .unwrap_or_default(),
        default_embeddings_model_context_size: provider
            .default_embeddings_model_context_size
            .unwrap_or_default(),
        default_embeddings_model_description: provider
            .default_embeddings_model_description
            .unwrap_or_default(),
        error: None,
    };

    let html = provider_page::page(team_id, rbac, form);

    Ok(Html(html))
}
