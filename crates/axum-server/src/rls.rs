use crate::authentication::Authentication;
use crate::errors::CustomError;
use db::queries;
use db::Transaction;

// A helper function for setting the RLS user which is used by all the policies.
pub async fn set_row_level_security_user(
    transaction: &Transaction<'_>,
    current_user: &Authentication,
) -> Result<bool, CustomError> {
    set_row_level_security_user_id(transaction, current_user.user_id).await?;

    let is_admin = queries::users::is_sys_admin()
        .bind(transaction, &current_user.user_id)
        .one()
        .await?;

    Ok(is_admin)
}

pub async fn set_row_level_security_user_id(
    transaction: &Transaction<'_>,
    user_id: i32,
) -> Result<(), CustomError> {
    transaction
        .query(
            &format!("SET LOCAL row_level_security.user_id = {}", user_id),
            &[],
        )
        .await?;

    Ok(())
}
