# Presentation Builder

Use this skill when the user asks for slides, a deck, a presentation, a pitch deck, a briefing, a visual walkthrough, or an HTML canvas presentation.

Create reveal.js presentations as generated output files at `/home/user/output/<deck-name>/CANVAS.md`. The generated canvas must be self-contained, because it is rendered directly in the chat iframe and network access is blocked.

## Preferred Workflow

1. Pick a short lowercase `<deck-name>` using only letters, numbers, dots, underscores, or hyphens.
2. Write only the reveal slide sections to a temporary file, for example `/tmp/deck.slides.html`.
3. Build the canvas with the bundled helper:

```bash
bash /home/user/skills/presentation-builder/bin/build-reveal-canvas.sh deck-name "Human readable title" /tmp/deck.slides.html
```

The helper writes `/home/user/output/<deck-name>/CANVAS.md` with this frontmatter and a complete HTML document:

```markdown
---
name: deck-name
title: Human readable title
type: text/html
---
<!doctype html>
<html>...</html>
```

## Slide Markup

- Put each slide in a `<section>...</section>` block.
- Use nested sections only when vertical slide stacks are helpful.
- Use reveal fragments with `class="fragment"` when progressive disclosure improves the story.
- Keep slide content concise: one core idea per slide, short headings, and scannable supporting points.
- Use inline SVG, CSS shapes, tables, and semantic HTML for diagrams and data views.
- Do not reference external scripts, stylesheets, fonts, images, or network URLs.
- Do not paste the reveal.js bundle manually; use the helper script.

After creating or editing the canvas, mention the generated path.
