-- migrate:up
ALTER TABLE prompts ADD COLUMN description VARCHAR NOT NULL DEFAULT 'Please add a description';
ALTER TABLE prompts ADD COLUMN disclaimer VARCHAR NOT NULL DEFAULT 'LLMs can make mistakes. Check important info.';
ALTER TABLE prompts ADD COLUMN example1 VARCHAR;
ALTER TABLE prompts ADD COLUMN example2 VARCHAR;
ALTER TABLE prompts ADD COLUMN example3 VARCHAR;
ALTER TABLE prompts ADD COLUMN example4 VARCHAR;

-- migrate:down
ALTER TABLE prompts DROP COLUMN description;
ALTER TABLE prompts DROP COLUMN disclaimer;
ALTER TABLE prompts DROP COLUMN example1;
ALTER TABLE prompts DROP COLUMN example2;
ALTER TABLE prompts DROP COLUMN example3;
ALTER TABLE prompts DROP COLUMN example4;


