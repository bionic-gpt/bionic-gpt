use super::{
    support::{validate_name, AppError, Authenticated, ItemInput, Store},
    ui,
};
use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
};
use std::sync::Arc;
use ui::support::routes::{Delete, Edit, Index, Update};

pub async fn edit_loader(
    Edit { team_id, id }: Edit,
    Authenticated(identity): Authenticated,
    State(store): State<Arc<Store>>,
) -> Result<Html<String>, AppError> {
    let item = store.editable(&identity, team_id, id)?;
    Ok(Html(ui::crud::page(team_id, id, item.name, None)))
}
pub async fn action_update(
    Update { team_id, id }: Update,
    Authenticated(identity): Authenticated,
    State(store): State<Arc<Store>>,
    Form(input): Form<ItemInput>,
) -> Result<Response, AppError> {
    // Even a validation-error page must not reveal/edit an unauthorized record.
    store.editable(&identity, team_id, id)?;
    let name = match validate_name(&input.name) {
        Ok(name) => name,
        Err(error) => {
            return Ok((
                StatusCode::UNPROCESSABLE_ENTITY,
                Html(ui::crud::page(team_id, id, input.name, Some(error.into()))),
            )
                .into_response())
        }
    };
    // The store repeats authorization atomically with the mutation.
    store.update(&identity, team_id, id, name)?;
    Ok(Redirect::to(&Index { team_id }.to_string()).into_response())
}
pub async fn action_delete(
    Delete { team_id, id }: Delete,
    Authenticated(identity): Authenticated,
    State(store): State<Arc<Store>>,
) -> Result<Redirect, AppError> {
    store.delete(&identity, team_id, id)?;
    Ok(Redirect::to(&Index { team_id }.to_string()))
}
