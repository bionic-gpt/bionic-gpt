-- migrate:up
UPDATE context.skill_files sf
SET contents = convert_to($$# Presentation Builder

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
FROM context.skills s
WHERE sf.skill_id = s.id
AND s.is_system = true
AND s.name = 'presentation-builder'
AND sf.relative_path = 'SKILL.md';


-- migrate:down
-- No-op: the old tool-based canvas path has been removed from the codebase.
