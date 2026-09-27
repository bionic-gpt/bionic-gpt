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
use ui::support::{
    routes::{Create, Index, New},
    ItemForm,
};

pub async fn new_loader(
    New { team_id }: New,
    Authenticated(identity): Authenticated,
) -> Result<Html<String>, AppError> {
    identity.require_team(team_id)?;
    Ok(Html(ui::form::page(team_id, ItemForm::default())))
}
pub async fn action_create(
    Create { team_id }: Create,
    Authenticated(identity): Authenticated,
    State(store): State<Arc<Store>>,
    Form(input): Form<ItemInput>,
) -> Result<Response, AppError> {
    identity.require_team(team_id)?;
    let name = match validate_name(&input.name) {
        Ok(name) => name,
        Err(error) => {
            return Ok((
                StatusCode::UNPROCESSABLE_ENTITY,
                Html(ui::form::page(
                    team_id,
                    ItemForm {
                        id: None,
                        name: input.name,
                        error: Some(error.into()),
                    },
                )),
            )
                .into_response())
        }
    };
    store.create(&identity, team_id, name)?;
    Ok(Redirect::to(&Index { team_id }.to_string()).into_response())
}
