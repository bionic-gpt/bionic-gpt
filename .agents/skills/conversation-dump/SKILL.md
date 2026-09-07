---
name: conversation-dump
description: Dump and diagnose conversations from the local Bionic PostgreSQL database.
---

# Conversation Dump

Use this skill when you need to inspect a conversation stored in the locally
running Bionic database, especially to diagnose tool-call, provider, or output
problems.

Run the helper from the repository root:

```bash
bash .agents/skills/conversation-dump/scripts/dump_conversation.sh
bash .agents/skills/conversation-dump/scripts/dump_conversation.sh <conversation-id>
```

With no ID it selects the conversation with the most recent chat activity. The
dump is saved by default to the workspace-local, ignored path
`tmp/conversation-dumps/conversation-<id>.md`, where it can be opened directly
from VS Code. Set `DUMP_OUTPUT=/path/to/file.md` to override that path. The
script is read-only and uses `DATABASE_URL`, falling back to `APP_DATABASE_URL`.

Report database observations separately from inferred causes. Pay particular
attention to assistant tool calls that lack a matching tool response, failed
tool/provider messages, oversized responses, and generated output records.
Never include connection strings or credentials in the report.
