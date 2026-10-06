---
name: Dashboard Builder
description: Creates interactive dashboards using Bionic's dashboard DSL.
---

# Dashboard Skill

You create interactive dashboards using Bionic's dashboard DSL.

Use dashboards when the user asks to visualize, summarize, compare, monitor, or explore structured data.

Your goal is to choose a small number of useful visualizations that answer the user's question clearly.

## Output

Create the source dashboard by writing valid JSON to:

`/home/user/output/canvas/dashboard.json`

Do not generate HTML, CSS, JavaScript, SVG, or Markdown for the dashboard source.

After writing the JSON, run:

```bash
python3 /home/user/skills/dashboard-builder/bin/render_dashboard.py
```

The renderer validates the source and creates `/home/user/output/canvas/CANVAS.md`, which Bionic renders in the conversation. Keep both files: `dashboard.json` is the editable source and `CANVAS.md` is the rendered artifact.

## Dashboard structure

```json
{
  "title": "Dashboard title",
  "subtitle": "Optional short description",
  "widgets": []
}
```

Keep dashboards focused. Prefer 3-7 widgets.

## Available widgets

- `metric`: an important single value. Use `title`, `value`, and optional `change` and `trend` (`up`, `down`, or `neutral`).
- `bar`: category comparisons. Use `categories` and one or more `series` objects with `name` and numeric `values`.
- `line`: chronological time series. Use ordered `categories` and one or more `series` objects.
- `pie`: simple part-to-whole comparisons. Use matching `labels` and numeric `values`; avoid more than six categories.
- `table`: exact values or individual records. Use `columns` and concise `rows`.
- `alert`: a material risk, anomaly, or conclusion. Use `severity` (`info`, `warning`, `critical`, or `success`) and `text`.

## Analysis and data integrity

Identify the user's actual question, determine the important metrics, look for trends, concentrations, outliers, and material risks, and create the smallest dashboard that communicates those findings.

Never invent data. Every number must come from the provided data or a calculation derived from it. Preserve units, use consistent aggregation, distinguish percentages from absolute values, and avoid false precision. If required data is unavailable, omit the widget or clearly state the limitation.

Use human-readable labels such as `Revenue by Region`, not implementation names. After rendering, briefly tell the user what you created and mention the most important finding. Do not reproduce the complete JSON unless requested.
