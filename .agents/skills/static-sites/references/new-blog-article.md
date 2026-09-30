# New Blog Article

Use this workflow when adding a post under `crates/bionic-gpt/content/blog/`.

## Inputs and Defaults

- Require an article title.
- Derive the slug from the title when none is supplied.
- Default the publish date to the current date.
- Infer author metadata from established blog conventions unless the user specifies it.

## Workflow

1. Inspect an existing article and its `PageSummary` in `crates/bionic-gpt/src/blog_summary.rs`.
2. Create `crates/bionic-gpt/content/blog/<slug>/index.md`, following the local article style and the user's supplied content.
3. Keep the supplied hero image as the visible article image.
4. Generate a distinct `open-graph.jpg` with the helper documented in `SKILL.md`. Do not stretch or regenerate the artwork.
5. Add the `PageSummary` in descending date order and reference the hero and Open Graph assets separately.
6. Validate the image dimensions and byte size, build the site, and inspect the rendered metadata.

## PageSummary Metadata

Match the complete local `PageSummary` shape, including:

- `date`
- `title`
- `description`
- `folder`
- `markdown`
- `image`
- `open_graph_image`
- `author`
- `author_image`

Use these paths consistently:

```rust
folder: "blog/<slug>/",
markdown: include_str!("../content/blog/<slug>/index.md"),
image: Some("/blog/<slug>/<hero-image>"),
open_graph_image: Some("/blog/<slug>/open-graph.jpg"),
```

The visible hero preserves the intended article presentation. The Open Graph asset must be a center-cropped 1200x630 JPEG below 490,000 bytes for broad social-preview compatibility, including WhatsApp.

Keep an initial draft simple and easy to revise. Do not invent or expand content beyond the user's request.
