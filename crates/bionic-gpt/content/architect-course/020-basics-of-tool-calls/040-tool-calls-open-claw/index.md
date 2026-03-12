# Open Claw Runtime Tools

Diagram of the [Open Claw](https://openclaw.ai) runtime tools, the [System Prompt](https://gist.github.com/242816/db0e828914b4d8c99de44e69aaec6042) and the Open Claw [Tool Defintions](https://gist.github.com/242816/9affbf5f3198e4e4677dd3afaf38e90d)

![Alt text](./open-claw.svg "Common Toolsets")

## Runtime Capabilities

- **Memory**: recall prior facts and context.
- **Sandbox**: safely read, write, edit, and execute code.
- **Cron**: run jobs on a schedule.
- **Skills**: packaged workflows and constraints.
- **Toolsets (OpenAPI, web, etc.)**: connect external systems.

Without a runtime, you have a chat model.
With a runtime, you have an **agent** that can act reliably over time.

## Tool Summary

| Tool name          | Category              | Required params                     | Key enums / notes                                                                     |
| ------------------ | --------------------- | ----------------------------------- | ------------------------------------------------------------------------------------- |
| `read`             | Filesystem            | `path \| file_path`                 | Read text or images, supports `offset`, `limit`                                       |
| `write`            | Filesystem            | `content`, `path \| file_path`      | Overwrites file                                                                       |
| `edit`             | Filesystem            | `path \| file_path`, `old*`, `new*` | Exact string replace; multiple alias params (`oldText`, `old_string`)                 |
| `exec`             | Shell                 | `cmd`                               | Long-running allowed; `timeout`, `pty`, `background`, `elevated`, `host`, `security`  |
| `process`          | Shell                 | `action`                            | `logs`, `write`, `keys`, `kill`, `status` (exec session control)                      |
| `browser`          | Browser automation    | `action`                            | Large dispatcher: `start`, `stop`, `open`, `navigate`, `act`, `snapshot`, `pdf`, etc. |
| `canvas`           | UI / A2UI             | `action`                            | `present`, `hide`, `eval`, `snapshot`, `push`, `reset`                                |
| `nodes`            | Device / node control | `action`                            | Pairing, notify, camera/screen/location, `run`, `invoke`                              |
| `message`          | Messaging             | `action`, `content`                 | Only `send` despite description mentioning broadcast                                  |
| `tts`              | Audio                 | `text`                              | Text-to-speech                                                                        |
| `agents_list`      | Agent mgmt            | —                                   | Lists available agents                                                                |
| `sessions_list`    | Session mgmt          | —                                   | List sessions                                                                         |
| `sessions_history` | Session mgmt          | `session_id`                        | Fetch conversation history                                                            |
| `sessions_send`    | Session mgmt          | `session_id`, `content`             | Send message to session                                                               |
| `sessions_spawn`   | Session mgmt          | `content`                           | Spawn sub-agent (one-shot or persistent)                                              |
| `subagents`        | Agent mgmt            | `action`                            | `list`, `kill`, `send`                                                                |
| `session_status`   | Session mgmt          | `session_id`                        | Inspect session; optional model override                                              |
| `web_search`       | Web                   | `query`                             | Brave Search wrapper; locale options                                                  |
| `web_fetch`        | Web                   | `url`                               | Fetch + readable extraction                                                           |
| `memory_search`    | Memory                | `query`                             | Semantic search over memory                                                           |
| `memory_get`       | Memory                | `id`                                | Retrieve memory entry                                                                 |
