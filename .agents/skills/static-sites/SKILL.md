---
name: static-sites
description: Create or modify Bionic's generated marketing site, documentation, blog, course pages, and static assets. Use for content, layouts, summaries, Tailwind, or ssg_whiz changes under crates/bionic-gpt.
---

# Static Sites

The current static site is `crates/bionic-gpt`; the old `static-website` and
`deploy-mcp` directories do not exist and must not be referenced as active
projects.

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
DO_NOT_RUN_SERVER=1 cargo run -p bionic-gpt
cargo build -p bionic-gpt
```

Inspect generated HTML for metadata, links, and asset paths. Run
`git diff --check`. Do not treat generated `dist/` output as a source change
unless the repository explicitly tracks it.
