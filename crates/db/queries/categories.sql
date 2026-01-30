--! categories : Category()
SELECT id, name, description FROM prompting.categories;

--! insert
INSERT INTO prompting.categories (name, description)
VALUES (:name, :description)
RETURNING id;

--! update
UPDATE prompting.categories
SET name = :name,
    description = :description
WHERE id = :id;

--! delete
DELETE FROM prompting.categories WHERE id = :id;
