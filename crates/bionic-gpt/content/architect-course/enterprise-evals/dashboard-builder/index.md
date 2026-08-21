# Dashboard Generation

<div class="not-prose my-6 flex flex-wrap gap-3">
  <a class="btn btn-primary" href="dashboard-builder.zip" download>Download Dashboard Builder Skill</a>
  <a class="btn btn-outline" href="SKILL.md" download>Download SKILL.md</a>
  <a class="btn btn-outline" href="bin/render_dashboard.py" download>Download renderer</a>
</div>

Download the ZIP, then open **Skills** in Bionic and upload it. The ZIP contains
the skill instructions and the bundled renderer in the folder structure Bionic
expects.

A dashboard is a compact answer to a structured-data question. The goal is not
to turn every dataset into a chart. The goal is to choose a few views that make
the important metrics, comparisons, trends, and risks easy to understand.

## The workflow

This eval tests whether the model can turn structured data into a useful,
grounded dashboard artifact. It checks skill discovery, data analysis, widget
selection, JSON generation, and canvas rendering without invented values.

When a user asks to visualize, compare, monitor, summarize, or explore
structured data, the model can discover the `Dashboard Builder` skill in the
virtual filesystem and read:

```text
/home/user/skills/dashboard-builder/SKILL.md
```

The skill defines a small JSON dashboard DSL. The model writes:

```text
/home/user/output/canvas/dashboard.json
```

It then runs the bundled renderer:

```bash
python3 /home/user/skills/dashboard-builder/bin/render_dashboard.py
```

The renderer validates the JSON and creates:

```text
/home/user/output/canvas/CANVAS.md
```

The JSON remains available as the editable source, while `CANVAS.md` is the
self-contained HTML artifact that Bionic displays in the conversation.

## Choosing widgets

Use the smallest useful set of widgets:

| Question | Widget |
| --- | --- |
| What is the headline number? | `metric` |
| How are categories different? | `bar` |
| How is a value changing over time? | `line` |
| How is a total composed? | `pie` |
| Which individual records matter? | `table` |
| What needs attention? | `alert` |

A good dashboard usually contains three to seven widgets. It should communicate
conclusions rather than reproduce every row in the source data.

## Example prompt

```text
Analyse the attached quarterly sales data. Create a focused dashboard showing
total revenue, the monthly trend, revenue by region, the largest customers, and
any material margin risk. Use only values derived from the data.
```

The resulting dashboard should make the evidence visible and explain important
limitations. It must not invent values or imply more precision than the source
data supports.

## The important boundary

The model creates JSON, not a custom application. The skill and renderer provide
the repeatable presentation method; the runtime provides Bashkit, the virtual
filesystem, generated-output persistence, and the chat canvas.
