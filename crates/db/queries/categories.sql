--! categories : Category()
SELECT id, name, description FROM categories;

--! insert
INSERT INTO categories (name, description)
VALUES (:name, :description)
RETURNING id;

--! update
UPDATE categories
SET name = :name,
    description = :description
WHERE id = :id;

--! delete
DELETE FROM categories WHERE id = :id;
