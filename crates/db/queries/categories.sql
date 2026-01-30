--! categories : Category()
SELECT id, name, description FROM assistants.categories;

--! insert
INSERT INTO assistants.categories (name, description)
VALUES (:name, :description)
RETURNING id;

--! update
UPDATE assistants.categories
SET name = :name,
    description = :description
WHERE id = :id;

--! delete
DELETE FROM assistants.categories WHERE id = :id;
