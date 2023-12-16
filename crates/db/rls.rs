use crate::queries;
use crate::{Permission, Transaction};

// A helper function for setting the RLS user which is used by all the policies.
pub async fn set_row_level_security_user(
    transaction: &Transaction<'_>,
    current_user_id: i32,
) -> Result<Rbac, crate::TokioPostgresError> {
    set_row_level_security_user_id(transaction, current_user_id).await?;

    let is_sys_admin = queries::users::is_sys_admin()
        .bind(transaction, &current_user_id)
        .one()
        .await?;

    let rbac = Rbac {
        permissions: Default::default(),
        is_sys_admin,
    };

    Ok(rbac)
}

pub async fn set_row_level_security_user_id(
    transaction: &Transaction<'_>,
    user_id: i32,
) -> Result<(), crate::TokioPostgresError> {
    transaction
        .query(
            &format!("SET LOCAL row_level_security.user_id = {}", user_id),
            &[],
        )
        .await?;

    Ok(())
}

#[derive(Default, PartialEq)]
pub struct Rbac {
    pub permissions: Vec<Permission>,
    pub is_sys_admin: bool,
}

impl Rbac {
    pub fn can_use_api_keys(&self) -> bool {
        self.is_sys_admin
    }
}
