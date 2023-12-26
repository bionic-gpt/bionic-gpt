use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Form, Path},
    response::Html,
};
use db::authz;
use db::queries;
use db::Pool;
use serde::Deserialize;
use ui_pages::audit_trail::{position_to_access_type, position_to_audit_action};

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

pub async fn filter(
    Path(team_id): Path<i32>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
    Form(filter_form): Form<Filter>,
) -> Result<Html<String>, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    let rbac = authz::get_permissions(&transaction, current_user.sub, team_id).await?;

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
            &(super::PAGE_SIZE + 1),
        )
        .all()
        .await?;

    Ok(Html(ui_pages::audit_trail::index::index(
        ui_pages::audit_trail::index::PageProps {
            team_id,
            rbac,
            team_users,
            audits,
            reset_search: false,
        },
    )))
}
