--: Invitation()
--: InviteSummary()

--! insert_invitation
INSERT INTO 
    iam.invitations (
        team_id, 
        email, 
        first_name, 
        last_name, 
        invitation_selector, 
        invitation_verifier_hash, 
        roles)
    VALUES(
        :team_id, 
        :email, 
        :first_name, 
        :last_name, 
        :invitation_selector, 
        :invitation_verifier_hash, 
        :roles);

-- Claim exactly one invitation atomically. Callers must validate and create
-- membership in the same transaction so failures restore the invitation.
--! consume_invitation : Invitation
DELETE FROM iam.invitations
WHERE invitation_selector = :invitation_selector
AND email = (SELECT email FROM iam.users WHERE id = current_app_user())
RETURNING id, team_id, email, first_name, last_name,
    invitation_selector, invitation_verifier_hash, roles, created_at;

--! consume_invitation_by_id : Invitation
DELETE FROM iam.invitations
WHERE id = :invite_id
AND email = (SELECT email FROM iam.users WHERE id = current_app_user())
RETURNING id, team_id, email, first_name, last_name,
    invitation_selector, invitation_verifier_hash, roles, created_at;

--! delete
DELETE FROM
    iam.invitations
WHERE
    id = :invite_id
AND
    team_id = :team_id;

--! get_all : Invitation
SELECT  
    id, 
    email,
    first_name, 
    last_name, 
    invitation_selector, 
    invitation_verifier_hash,
    team_id,
    roles,
    created_at  
FROM 
    iam.invitations 
WHERE team_id = :team_id;

--! get_by_user : InviteSummary
SELECT  
    id, 
    email,
    first_name, 
    last_name, 
    COALESCE((SELECT name FROM iam.teams t WHERE t.id = team_id), 'Name Not Set') AS team_name,
    team_id,
    (SELECT email FROM iam.users u WHERE u.id IN (SELECT created_by_user_id FROM iam.teams t WHERE t.id = team_id)) AS created_by,
    created_at  
FROM 
    iam.invitations 
WHERE 
    email in (SELECT email from iam.users WHERE id = current_app_user());