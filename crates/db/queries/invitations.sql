--: Invitation()
--: InviteSummary()

--! insert_invitation
INSERT INTO 
    auth.invitations (
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

--! get_invitation : Invitation
SELECT
    id, 
    team_id, 
    email, 
    first_name, 
    last_name, 
    invitation_selector, 
    invitation_verifier_hash,
    roles,
    created_at
FROM 
    auth.invitations 
WHERE
    invitation_selector = :invitation_selector;

--! get_invitation_by_id : Invitation
SELECT
    id, 
    team_id, 
    email, 
    first_name, 
    last_name, 
    invitation_selector, 
    invitation_verifier_hash,
    roles,
    created_at
FROM 
    auth.invitations 
WHERE
    id = :invite_id;

--! delete_invitation
DELETE FROM
    auth.invitations
WHERE
    email = :email
AND
    team_id = :team_id;

--! delete
DELETE FROM
    auth.invitations
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
    auth.invitations 
WHERE team_id = :team_id;

--! get_by_user : InviteSummary
SELECT  
    id, 
    email,
    first_name, 
    last_name, 
    COALESCE((SELECT name FROM tenancy.teams t WHERE t.id = team_id), 'Name Not Set') AS team_name,
    team_id,
    (SELECT email FROM auth.users u WHERE u.id IN (SELECT created_by_user_id FROM tenancy.teams t WHERE t.id = team_id)) AS created_by,
    created_at  
FROM 
    auth.invitations 
WHERE 
    email in (SELECT email from auth.users WHERE id = current_app_user());