use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use std::{collections::BTreeMap, sync::Mutex};

// Trusted authentication middleware inserts this extension after verifying identity.
// Never populate it directly from unsigned JWT payloads, headers, or form values.
#[derive(Clone)]
pub struct Identity {
    pub user_id: i64,
    pub team_ids: Vec<i64>,
}
pub struct Authenticated(pub Identity);
impl<S: Send + Sync> FromRequestParts<S> for Authenticated {
    type Rejection = AppError;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Identity>()
            .cloned()
            .map(Self)
            .ok_or(AppError::Unauthenticated)
    }
}
impl Identity {
    pub fn require_team(&self, team_id: i64) -> Result<(), AppError> {
        if self.team_ids.contains(&team_id) {
            Ok(())
        } else {
            Err(AppError::Forbidden)
        }
    }
}
#[derive(Debug)]
pub enum AppError {
    Unauthenticated,
    Forbidden,
    NotFound,
    Storage,
}
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::Unauthenticated => (StatusCode::UNAUTHORIZED, "Authentication required"),
            Self::Forbidden => (StatusCode::FORBIDDEN, "Access denied"),
            Self::NotFound => (StatusCode::NOT_FOUND, "Item not found"),
            Self::Storage => (StatusCode::INTERNAL_SERVER_ERROR, "Storage unavailable"),
        };
        (status, message).into_response()
    }
}
#[derive(Deserialize)]
pub struct ItemInput {
    pub name: String,
}
pub fn validate_name(name: &str) -> Result<String, &'static str> {
    let name = name.trim();
    if name.is_empty() || name.chars().count() > 100 {
        Err("Name must contain between 1 and 100 characters")
    } else {
        Ok(name.to_owned())
    }
}
#[derive(Clone)]
pub struct Item {
    pub id: i64,
    pub team_id: i64,
    pub owner_id: i64,
    pub name: String,
}

// DEMONSTRATION ADAPTER: process-local storage, no persistence or database transaction.
// Every method enforces its own authorization; replace with scoped generated SQL.
#[derive(Default)]
pub struct Store {
    inner: Mutex<Data>,
}
#[derive(Default)]
struct Data {
    last_id: i64,
    items: BTreeMap<i64, Item>,
}
impl Store {
    pub fn list(&self, identity: &Identity, team_id: i64) -> Result<Vec<Item>, AppError> {
        identity.require_team(team_id)?;
        let data = self.inner.lock().map_err(|_| AppError::Storage)?;
        Ok(data
            .items
            .values()
            .filter(|item| item.team_id == team_id)
            .cloned()
            .collect())
    }
    pub fn editable(&self, identity: &Identity, team_id: i64, id: i64) -> Result<Item, AppError> {
        identity.require_team(team_id)?;
        let data = self.inner.lock().map_err(|_| AppError::Storage)?;
        Ok(owned(&data, identity, team_id, id)?.clone())
    }
    pub fn create(&self, identity: &Identity, team_id: i64, name: String) -> Result<i64, AppError> {
        identity.require_team(team_id)?;
        let mut data = self.inner.lock().map_err(|_| AppError::Storage)?;
        let id = data.last_id.checked_add(1).ok_or(AppError::Storage)?;
        data.last_id = id;
        data.items.insert(
            id,
            Item {
                id,
                team_id,
                owner_id: identity.user_id,
                name,
            },
        );
        Ok(id)
    }
    pub fn update(
        &self,
        identity: &Identity,
        team_id: i64,
        id: i64,
        name: String,
    ) -> Result<(), AppError> {
        identity.require_team(team_id)?;
        let mut data = self.inner.lock().map_err(|_| AppError::Storage)?;
        // Authorize and mutate under one lock; no check-then-write race.
        owned(&data, identity, team_id, id)?;
        data.items.get_mut(&id).ok_or(AppError::NotFound)?.name = name;
        Ok(())
    }
    pub fn delete(&self, identity: &Identity, team_id: i64, id: i64) -> Result<(), AppError> {
        identity.require_team(team_id)?;
        let mut data = self.inner.lock().map_err(|_| AppError::Storage)?;
        owned(&data, identity, team_id, id)?;
        data.items.remove(&id).ok_or(AppError::NotFound)?;
        Ok(())
    }
}
fn owned<'a>(
    data: &'a Data,
    identity: &Identity,
    team_id: i64,
    id: i64,
) -> Result<&'a Item, AppError> {
    let item = data
        .items
        .get(&id)
        .filter(|item| item.team_id == team_id)
        .ok_or(AppError::NotFound)?;
    if item.owner_id != identity.user_id {
        return Err(AppError::Forbidden);
    }
    Ok(item)
}
