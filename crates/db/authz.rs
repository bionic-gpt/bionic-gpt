use crate::queries;
use crate::{types, DatasetConnection, Permission, Transaction, Visibility};

// A helper function for setting the RLS user which is used by all the policies.
pub async fn authorize(
    transaction: &Transaction<'_>,
    current_user_id: String,
    current_team_id: i32,
) -> Result<Rbac, crate::TokioPostgresError> {
    /***let user = queries::users::user_by_openid_sub()
        .bind(transaction, &current_user.sub)
        .one()
        .await;

    let user_id = if let Ok(user) = user {
        user.id
    } else {
        queries::users::insert()
            .bind(
                &transaction,
                &current_user.sub,
                &current_user.email,
                &current_user.given_name,
                &current_user.family_name,
            )
            .one()
            .await?
    };**/

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

// Creates the users default prompt and anything else they need
async fn _setup_user(
    transaction: &Transaction<'_>,
    current_user: i32,
) -> Result<i32, crate::TokioPostgresError> {
    let inserted_org_id = queries::teams::insert_team()
        .bind(transaction)
        .one()
        .await?;

    let roles = vec![
        types::public::Role::TeamManager,
        types::public::Role::Collaborator,
    ];

    queries::teams::add_user_to_team()
        .bind(transaction, &current_user, &inserted_org_id, &roles)
        .await?;

    let model = queries::models::get_system_model()
        .bind(transaction)
        .one()
        .await?;

    let system_prompt: Option<String> = None;

    queries::prompts::insert()
        .bind(
            transaction,
            &inserted_org_id,
            &model.id,
            &"Default (Exclude All Datasets)",
            &Visibility::Private,
            &DatasetConnection::None,
            &system_prompt,
            &3,
            &10,
            &1024,
            &100,
            &0.7,
            &0.1,
        )
        .one()
        .await?;

    Ok(inserted_org_id)
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
