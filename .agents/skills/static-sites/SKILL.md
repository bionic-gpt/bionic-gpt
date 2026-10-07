---
name: static-sites
description: Create or modify Bionic's generated marketing site, documentation, blog, course pages, and static assets. Use for content, layouts, summaries, Tailwind, or ssg_whiz changes under crates/bionic-gpt.
---

# Static Sites

The current static site is `crates/bionic-gpt`

## Content and Rendering

- Marketing pages are under `crates/bionic-gpt/content/pages`.
- Blog content is under `crates/bionic-gpt/content/blog` and is registered through `src/blog_summary.rs`.
- Documentation is under `content/docs` and is registered through `src/docs_summary.rs`.
- Course content is under `content/architect-course` and is registered through `src/architect_course_summary.rs`.
- Shared layouts and site configuration are under `crates/bionic-gpt/src`.
- `crates/bionic-gpt/build.rs` generates course page source where required before `ssg_whiz` renders the site.

Preserve the existing `ssg_whiz` summary and layout conventions. Keep visible
images, Open Graph images, metadata, canonical links, navigation, and footer
behavior distinct where the code supports those concepts.

## Marketing Visual System

Marketing typography is defined in `crates/bionic-gpt/input.css`. HTML heading
levels provide the standard type hierarchy: `h1` is the page or hero heading,
`h2` is a section heading, and `h3` is a feature, card, or subsection heading.
The site inherits Tailwind's `--default-font-family`, currently its system sans
stack. Eyebrow text uses the semantic `.site-eyebrow` class.

The homepage use-case river has named art-direction treatments for its intro
and feature headings: `.use-case-river__heading--intro` and
`.use-case-river__heading--copy`. These styles live alongside the global
heading rules in the marketing stylesheet.

Shared CSS owns recurring typography. DaisyUI provides themed controls such as
buttons and semantic colors such as `base-content` and `primary`. Component
Tailwind classes describe composition and geometry, including grids, flex
layouts, spacing, alignment, and image sizing.

## Blog Articles

For a new blog article, read [references/new-blog-article.md](references/new-blog-article.md) before editing. It defines the content, summary metadata, hero-image, and Open Graph workflow.

Use the bundled Open Graph helper to create the social-preview image from the article hero:

```bash
cargo run --quiet --manifest-path .agents/skills/static-sites/scripts/prepare-open-graph/Cargo.toml -- <hero-image> <article-folder>/open-graph.jpg
```

The helper creates an exact 1200x630 JPEG and adapts quality until the file is below the site's 490,000-byte limit.

## Assets and Commands

Static application assets are maintained in `crates/web-assets`; its Tailwind
input is `crates/web-assets/input.css`, and its TypeScript entry is
`crates/web-assets/index.ts`. The site-specific Tailwind input is in
`crates/bionic-gpt/input.css`.

`crates/web-assets/build.rs` uses `cache-busters` to generate typed references
for files in `dist/` and `images/`; application code should use those generated
references rather than hard-coded hashed asset paths.

Verified development recipes are:

```bash
just ws
just wts
```

The `ws` recipe watches `crates/bionic-gpt/content` and `src`; `wts` watches
the site's Tailwind input. Inspect `Justfile` before using other recipes.

## Verification

```bash
(cd crates/bionic-gpt && DO_NOT_RUN_SERVER=1 cargo run -p bionic-gpt)
cargo build -p bionic-gpt
```

Run the site generator from `crates/bionic-gpt` so its `dist/` output lands in
the site crate's ignored build directory, not the workspace root. Inspect the
generated HTML for metadata, links, and asset paths. Run `git diff --check`.
