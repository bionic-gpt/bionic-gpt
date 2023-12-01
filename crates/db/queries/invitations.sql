--: Invitation()

--! insert_invitation
INSERT INTO 
    invitations (
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
    invitations 
WHERE
    invitation_selector = :invitation_selector;

--! delete_invitation
DELETE FROM
    invitations
WHERE
    email = :email
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
    invitations 
WHERE team_id = :team_id;