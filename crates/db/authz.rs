use serde::{Deserialize, Serialize};

use crate::{queries, Dataset, Prompt};
use crate::{types, Permission, Transaction};

#[derive(Serialize, Deserialize, Debug)]
pub struct Authentication {
    pub sub: String,
    pub email: String,
}

// A helper function for setting the RLS user which is used by all the policies.
pub async fn get_permissions(
    transaction: &Transaction<'_>,
    authentication: &Authentication,
    current_team_id: i32,
) -> Result<Rbac, crate::TokioPostgresError> {
    let user = queries::users::user_by_openid_sub()
        .bind(transaction, &authentication.sub)
        .one()
        .await;

    // Do we have a user with this sub?
    let (user_id, email, first_name, last_name, system_admin) = if let Ok(user) = user {
        (
            user.id,
            user.email,
            user.first_name,
            user.last_name,
            user.system_admin,
        )
    } else {
        setup_user_if_not_already_registered(transaction, authentication).await?
    };

    transaction
        .query(
            &format!("SET LOCAL row_level_security.user_id = {}", user_id),
            &[],
        )
        .await?;

    let permissions = queries::users::get_permissions()
        .bind(transaction, &current_team_id)
        .all()
        .await?;

    let rbac = Rbac {
        permissions,
        user_id,
        email,
        first_name,
        last_name,
        is_sys_admin: system_admin,
    };

    Ok(rbac)
}

pub async fn set_row_level_security_user_id(
    transaction: &Transaction<'_>,
    user_id: String,
) -> Result<i32, crate::TokioPostgresError> {
    let user = queries::users::user_by_openid_sub()
        .bind(transaction, &user_id)
        .one()
        .await?;

    transaction
        .query(
            &format!("SET LOCAL row_level_security.user_id = {}", user.id),
            &[],
        )
        .await?;

    Ok(user.id)
}

// Creates the users default prompt and anything else they need
pub async fn setup_user_if_not_already_registered(
    transaction: &Transaction<'_>,
    authentication: &Authentication,
) -> Result<(i32, String, Option<String>, Option<String>, bool), crate::TokioPostgresError> {
    let user_id = queries::users::insert()
        .bind(transaction, &authentication.sub, &authentication.email)
        .one()
        .await?;

    transaction
        .query(
            &format!("SET LOCAL row_level_security.user_id = {}", user_id),
            &[],
        )
        .await?;

    let inserted_org_id = queries::teams::insert_team()
        .bind(transaction)
        .one()
        .await?;

    let roles = vec![
        types::public::Role::TeamManager,
        types::public::Role::Collaborator,
    ];

    queries::teams::add_user_to_team()
        .bind(transaction, &user_id, &inserted_org_id, &roles)
        .await?;

    let user = queries::users::user()
        .bind(transaction, &user_id)
        .one()
        .await?;

    Ok((
        user.id,
        user.email,
        user.first_name,
        user.last_name,
        user.system_admin,
    ))
}

#[derive(Default, Clone, PartialEq)]
pub struct Rbac {
    pub permissions: Vec<Permission>,
    pub user_id: i32,
    pub email: String,
    pub is_sys_admin: bool,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

impl Rbac {
    pub fn can_use_api_keys(&self) -> bool {
        self.permissions.contains(&Permission::CreateApiKeys)
    }

    pub fn can_view_teams(&self) -> bool {
        self.permissions.contains(&Permission::ViewCurrentTeam)
    }

    pub fn can_delete_chat(&self) -> bool {
        self.permissions.contains(&Permission::DeleteChat)
    }

    pub fn can_make_invitations(&self) -> bool {
        self.permissions.contains(&Permission::InvitePeopleToTeam)
    }

    pub fn can_view_datasets(&self) -> bool {
        self.permissions.contains(&Permission::ViewDatasets)
    }

    pub fn can_manage_datasets(&self) -> bool {
        self.permissions.contains(&Permission::ManageDatasets)
    }

    pub fn can_view_prompts(&self) -> bool {
        self.permissions.contains(&Permission::ViewPrompts)
    }

    pub fn can_view_audit_trail(&self) -> bool {
        self.permissions.contains(&Permission::ViewAuditTrail)
    }

    pub fn can_setup_models(&self) -> bool {
        self.permissions.contains(&Permission::SetupModels)
    }

    pub fn can_edit_prompt(&self, prompt: &Prompt) -> bool {
        prompt.created_by == self.user_id || self.is_sys_admin
    }

    pub fn can_edit_dataset(&self, _dataset: &Dataset) -> bool {
        self.is_sys_admin
    }
}
