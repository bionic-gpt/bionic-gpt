// Populate VerifiedIdentity only after authentication middleware verifies the request.
#[derive(Clone)]
pub struct VerifiedIdentity {
    pub user_id: i64,
    pub team_ids: Vec<i64>,
}
pub struct ItemScope {
    pub team_id: i64,
    pub owner_id: i64,
}
#[derive(Debug, PartialEq)]
pub struct AccessDenied;
pub fn require_team(identity: &VerifiedIdentity, team_id: i64) -> Result<(), AccessDenied> {
    if identity.team_ids.contains(&team_id) {
        Ok(())
    } else {
        Err(AccessDenied)
    }
}
pub fn require_write(
    identity: &VerifiedIdentity,
    selected_team: i64,
    item: &ItemScope,
) -> Result<(), AccessDenied> {
    require_team(identity, selected_team)?;
    if item.team_id == selected_team && item.owner_id == identity.user_id {
        Ok(())
    } else {
        Err(AccessDenied)
    }
}
