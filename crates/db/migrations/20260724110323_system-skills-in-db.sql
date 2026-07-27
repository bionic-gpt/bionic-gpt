-- migrate:up
ALTER TABLE context.skills ADD COLUMN is_system BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE context.skills ALTER COLUMN team_id DROP NOT NULL;
ALTER TABLE context.skills ALTER COLUMN created_by DROP NOT NULL;
ALTER TABLE context.skills ADD CONSTRAINT skills_user_owned_or_system CHECK (
    is_system OR (team_id IS NOT NULL AND created_by IS NOT NULL)
);

ALTER TABLE context.skill_files ADD COLUMN contents BYTEA;
ALTER TABLE context.skill_files ALTER COLUMN object_id DROP NOT NULL;
ALTER TABLE context.skill_files ADD CONSTRAINT skill_files_single_content_source CHECK (
    num_nonnulls(object_id, contents) = 1
);

WITH skill AS (
    INSERT INTO context.skills (
        name,
        description,
        visibility,
        is_system
    )
    VALUES (
        'dataset-analysis',
        'Use assistant datasets for grounded answers with rag-search and rag-read.',
        'Company',
        true
    )
    RETURNING id
)
INSERT INTO context.skill_files (
    skill_id,
    relative_path,
    contents
)
SELECT
    id,
    'SKILL.md',
    convert_to($$# Dataset Analysis

Use this skill when the user asks a question that should be answered from the assistant's datasets.

1. Inspect `/home/user/datasets/index.json` to see available datasets.
2. Use `rag-search "query" --limit N` to find relevant chunks.
3. Read returned chunk paths with `rag-read PATH` or `cat PATH`.
4. Answer from the retrieved evidence. Say when the datasets do not contain enough information.
$$, 'UTF8')
FROM skill;

WITH skill AS (
    INSERT INTO context.skills (
        name,
        description,
        visibility,
        is_system
    )
    VALUES (
        'shell-data-workbench',
        'Inspect, filter, summarize, and transform sandbox files with shell tools.',
        'Company',
        true
    )
    RETURNING id
)
INSERT INTO context.skill_files (
    skill_id,
    relative_path,
    contents
)
SELECT
    id,
    'SKILL.md',
    convert_to($$# Shell Data Workbench

Use this skill when the user asks to inspect, filter, summarize, or transform files inside the Bashkit sandbox.

Prefer normal shell tools such as `find`, `rg`, `grep`, `awk`, `sed`, `sort`, `uniq`, `wc`, `jq`, `csv`, `yaml`, and `tomlq`.

Work inside `/home/user` unless the task requires `/tmp`. Keep intermediate files small and explain the final result, not every command.
$$, 'UTF8')
FROM skill;

WITH skill AS (
    INSERT INTO context.skills (
        name,
        description,
        visibility,
        is_system
    )
    VALUES (
        'presentation-builder',
        'Create slide decks and presentation-style visual artifacts as generated HTML canvas files.',
        'Company',
        true
    )
    RETURNING id
)
INSERT INTO context.skill_files (
    skill_id,
    relative_path,
    contents
)
SELECT
    id,
    'SKILL.md',
    convert_to($$# Presentation Builder

Use this skill when the user asks for slides, a deck, a presentation, a pitch deck, a briefing, a visual walkthrough, or an HTML canvas.

Create the artifact as a generated output file at `/home/user/output/<canvas-name>/CANVAS.md`.

The file must contain frontmatter followed by one complete static HTML document:

```markdown
---
name: canvas-name
title: Human readable title
type: text/html
---
<!doctype html>
<html>...</html>
```

1. Use a short lowercase folder name for `<canvas-name>`.
2. Build a single self-contained HTML document with inline CSS.
3. Use multiple `<section class="slide">...</section>` blocks for slide decks.
4. Keep the canvas readable, visually balanced, and sized for iframe display.
5. Do not use external scripts, external stylesheets, network calls, or multiple canvas files for one artifact.

After creating or editing the canvas, mention the generated path.
$$, 'UTF8')
FROM skill;


-- migrate:down
DELETE FROM context.skills
WHERE is_system = true
AND name IN (
    'dataset-analysis',
    'shell-data-workbench',
    'presentation-builder'
);

ALTER TABLE context.skill_files DROP CONSTRAINT skill_files_single_content_source;
ALTER TABLE context.skill_files ALTER COLUMN object_id SET NOT NULL;
ALTER TABLE context.skill_files DROP COLUMN contents;

ALTER TABLE context.skills DROP CONSTRAINT skills_user_owned_or_system;
ALTER TABLE context.skills ALTER COLUMN team_id SET NOT NULL;
ALTER TABLE context.skills ALTER COLUMN created_by SET NOT NULL;
ALTER TABLE context.skills DROP COLUMN is_system;
