-- Separate statements; $1 is always the verified user ID, $2 the selected team ID.
-- list_items: team members may read. Parameters: user_id, team_id.
SELECT i.id, i.team_id, i.owner_id, i.name FROM items i
WHERE i.team_id = $2 AND EXISTS (
    SELECT 1 FROM team_members m WHERE m.team_id = $2 AND m.user_id = $1
) ORDER BY i.id;

-- create_item: derive owner from identity. Parameters: user_id, team_id, name.
INSERT INTO items (team_id, owner_id, name)
SELECT $2, $1, btrim($3::text)
WHERE EXISTS (SELECT 1 FROM team_members m WHERE m.team_id = $2 AND m.user_id = $1)
RETURNING id, team_id, owner_id, name;

-- update_item: authorize in the mutation. Parameters: user_id, team_id, item_id, name.
UPDATE items SET name = btrim($4::text)
WHERE id = $3 AND team_id = $2 AND owner_id = $1 AND EXISTS (
    SELECT 1 FROM team_members m WHERE m.team_id = $2 AND m.user_id = $1
) RETURNING id, team_id, owner_id, name;

-- delete_item: zero rows is not success. Parameters: user_id, team_id, item_id.
DELETE FROM items WHERE id = $3 AND team_id = $2 AND owner_id = $1 AND EXISTS (
    SELECT 1 FROM team_members m WHERE m.team_id = $2 AND m.user_id = $1
) RETURNING id;
