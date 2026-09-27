use super::{
    support::{AppError, Authenticated, Store},
    ui,
};
use axum::{extract::State, response::Html};
use std::sync::Arc;
use ui::support::{routes::Index, ItemView};

pub async fn loader(
    Index { team_id }: Index,
    Authenticated(identity): Authenticated,
    State(store): State<Arc<Store>>,
) -> Result<Html<String>, AppError> {
    let items = store
        .list(&identity, team_id)?
        .into_iter()
        .map(|item| ItemView {
            id: item.id,
            name: item.name,
            can_edit: item.owner_id == identity.user_id,
        })
        .collect();
    // The demonstration policy allows every authorized team member to create.
    Ok(Html(ui::list::page(team_id, items, true)))
}
