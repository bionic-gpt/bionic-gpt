---
name: office-tools
description: Regenerate the built-in Office OpenAPI specs from a selected Archipelago ref. Use when the Office container or its upstream operation surface is updated.
---

# Office tools

Use this skill only when the user asks to update or regenerate the built-in
Office tools. The generated specifications are compiled into Bionic and are
available system-wide; they are not database integrations and do not need a
migration.

## Regenerate specs

From the repository root, run:

```bash
cargo run -p dagger-pipeline -- \
  generate-office-specs \
  --archipelago-ref main
```

Pass a tag or commit instead of `main` when the Office image is built from a
specific upstream revision. The command stages the selected Archipelago
source, generates and validates the three OpenAPI documents, and writes them
to:

```text
crates/tool-runtime/system_specs/office/
```

The files are source-controlled runtime resources. Review their diff after
generation and ensure the matching Office container is built from the same
Archipelago ref.

## Verification

Check the generated documents and focused Rust code with:

```bash
git diff --check
cargo test -p tool-runtime
cargo check -p tool-runtime
```

Do not create a database migration or insert rows into the integrations tables
for these specs. Do not publish or deploy the Office image unless the user
explicitly asks for that separately.
