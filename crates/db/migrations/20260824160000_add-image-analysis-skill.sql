-- migrate:up

WITH skill AS (
    INSERT INTO context.skills (
        name,
        description,
        visibility,
        is_system
    )
    VALUES (
        'image-analysis',
        'Use image evidence to produce domain-aware answers that distinguish observations, uncertainty, and recommended follow-up checks.',
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
    convert_to($$# Image Analysis

Use this skill when the user's task depends on information contained in an
uploaded image, photograph, diagram, screenshot, equipment image, or rendered
document.

## Workflow

1. Identify the relevant image in `/home/user/attachments`.
2. List `/home/user/functions` and read the relevant image-analysis function
   documentation before calling it.
3. Send the image reference and a short task describing what needs to be
   extracted or assessed.
4. Treat the returned analysis as evidence, not as the final answer.
5. Write a useful answer for the user's domain and task.

## Evidence discipline

Separate the answer into:

- directly observed details;
- uncertain interpretations, preserving the tool's confidence;
- recommended follow-up checks or actions.

Never invent text, measurements, labels, equipment states, defects, or
diagnoses that are not supported by the image-analysis result. If the analysis
says that something cannot be read or determined reliably, say so rather than
guessing. Do not present an inference as a direct observation.

Use the user's domain context to explain why the observations matter, while
keeping the level of certainty explicit.
$$, 'UTF8')
FROM skill;

-- migrate:down

DELETE FROM context.skills
WHERE is_system = true
AND name = 'image-analysis';
