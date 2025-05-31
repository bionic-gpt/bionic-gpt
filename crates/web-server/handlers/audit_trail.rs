use crate::{CustomError, Jwt};
use axum::{extract::Extension, response::Html};
use db::authz;
use db::queries;
use db::Pool;

use axum::extract::Form;
use axum::Router;
use axum_extra::routing::RouterExt;
use serde::Deserialize;
use web_pages::{
    audit_trail::{position_to_access_type, position_to_audit_action},
    routes::audit_trail::Index,
};

pub const PAGE_SIZE: i64 = 10;

pub fn routes() -> Router {
    Router::new().typed_get(loader).typed_post(filter_action)
}

pub async fn loader(
    Index { team_id }: Index,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    if !rbac.can_view_audit_trail() {
        return Err(CustomError::Authorization);
    }

    let team_users = queries::teams::get_users()
        .bind(&transaction, &team_id)
        .all()
        .await?;

    let audits = queries::audit_trail::audit()
        .bind(&transaction, &None, &None, &None, &None, &(PAGE_SIZE + 1))
        .all()
        .await?;

    let html = web_pages::audit_trail::index::page(team_users, audits, team_id, rbac, true);

    Ok(Html(html))
}

#[derive(Deserialize, Default, Debug)]
pub struct Filter {
    pub id: i32,
    pub user: i32,
    pub access_type: usize,
    pub action: usize,
}

impl Filter {
    pub fn get_id(&self) -> Option<i32> {
        match self.id {
            0 => None,
            n => Some(n),
        }
    }

    pub fn get_user(&self) -> Option<i32> {
        match self.user {
            0 => None,
            n => Some(n),
        }
    }

    pub fn convert_to_access_type(&self) -> Option<db::AuditAccessType> {
        if self.access_type == 0 {
            None
        } else {
            Some(position_to_access_type(self.access_type - 1))
        }
    }

    pub fn convert_to_action(&self) -> Option<db::AuditAction> {
        if self.action == 0 {
            None
        } else {
            Some(position_to_audit_action(self.action - 1))
        }
    }
}

pub async fn filter_action(
    Index { team_id }: Index,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Form(filter_form): Form<Filter>,
) -> Result<Html<String>, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    if !rbac.can_view_audit_trail() {
        return Err(CustomError::Authorization);
    }

    let team_users = queries::teams::get_users()
        .bind(&transaction, &team_id)
        .all()
        .await?;

    let audits = queries::audit_trail::audit()
        .bind(
            &transaction,
            &filter_form.get_id(),
            &filter_form.convert_to_action(),
            &filter_form.convert_to_access_type(),
            &filter_form.get_user(),
            &(PAGE_SIZE + 1),
        )
        .all()
        .await?;

    let html = web_pages::audit_trail::index::page(team_users, audits, team_id, rbac, false);

    Ok(Html(html))
}
