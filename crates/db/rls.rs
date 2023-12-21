use crate::queries;
use crate::{Permission, Transaction};

// A helper function for setting the RLS user which is used by all the policies.
pub async fn set_row_level_security_user(
    transaction: &Transaction<'_>,
    current_user_id: String,
    current_team_id: i32,
) -> Result<Rbac, crate::TokioPostgresError> {
    let user_id = set_row_level_security_user_id(transaction, current_user_id).await?;

    let permissions = queries::users::get_permissions()
        .bind(transaction, &current_team_id)
        .all()
        .await?;

    let rbac = Rbac {
        permissions,
        user_id,
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

#[derive(Default, PartialEq)]
pub struct Rbac {
    pub permissions: Vec<Permission>,
    pub user_id: i32,
}

impl Rbac {
    pub fn can_use_api_keys(&self) -> bool {
        self.permissions.contains(&Permission::CreateApiKeys)
    }

    pub fn can_view_teams(&self) -> bool {
        self.permissions.contains(&Permission::ViewCurrentTeam)
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
}
