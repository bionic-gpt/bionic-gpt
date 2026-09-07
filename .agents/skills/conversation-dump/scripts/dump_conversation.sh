#!/usr/bin/env bash
set -euo pipefail

conversation_id="${1:-}"
database_url="${DATABASE_URL:-${APP_DATABASE_URL:-}}"

if [[ -z "$database_url" ]]; then
  echo "DATABASE_URL or APP_DATABASE_URL is required" >&2
  exit 1
fi

if [[ -n "$conversation_id" && ! "$conversation_id" =~ ^[0-9]+$ ]]; then
  echo "conversation ID must be a positive integer" >&2
  exit 1
fi

id_clause="TRUE"
if [[ -n "$conversation_id" ]]; then
  id_clause="c.id = $conversation_id"
fi

sql=$(cat <<'SQL'
WITH selected AS (
  SELECT c.id
  FROM llm.conversations c
  JOIN llm.chats ch ON ch.conversation_id = c.id
  WHERE $id_clause
  GROUP BY c.id
  ORDER BY MAX(ch.created_at) DESC
  LIMIT 1
), conversation_data AS (
  SELECT c.id, c.created_at, MAX(ch.created_at) AS last_activity,
    COALESCE(json_agg(json_build_object(
      'id', ch.id, 'role', ch.role::text, 'status', ch.status::text,
      'created_at', ch.created_at, 'tool_call_id', ch.tool_call_id,
      'tool_calls', ch.tool_calls, 'content', decrypt_text(ch.content)
    ) ORDER BY ch.id) FILTER (WHERE ch.id IS NOT NULL), '[]'::json) AS chats,
    COALESCE((SELECT json_agg(json_build_object(
      'id', go.id, 'path', go.path, 'file_name', go.file_name,
      'mime_type', go.mime_type, 'size', go.file_size, 'created_at', go.created_at
    ) ORDER BY go.path) FROM llm.generated_outputs go WHERE go.conversation_id = c.id), '[]'::json) AS outputs
  FROM selected s JOIN llm.conversations c ON c.id = s.id
  LEFT JOIN llm.chats ch ON ch.conversation_id = c.id
  GROUP BY c.id, c.created_at
)
SELECT row_to_json(conversation_data) FROM conversation_data;
SQL
)
sql="${sql//\$id_clause/$id_clause}"

json=$(psql "$database_url" --no-psqlrc --tuples-only --no-align -c "$sql")

if [[ -z "$json" ]]; then
  echo "No matching conversation found" >&2
  exit 1
fi

resolved_id=$(jq -r '.id' <<< "$json")

markdown=$(jq -r '
  def clip: if . == null then "" elif (tostring|length) > 12000 then (tostring[0:12000] + "… [truncated]") else tostring end;
  "# Conversation \(.id)\n\nCreated: \(.created_at)\nLast activity: \(.last_activity)\n\n## Messages\n" +
  ([.chats[] | "\n### \(.id) — \(.role) — \(.status) — \(.created_at)\n" +
    (if .tool_call_id then "Tool call ID: `\(.tool_call_id)`\n" else "" end) +
    (if .tool_calls then "Tool calls:\n```json\n\(.tool_calls|clip)\n```\n" else "" end) +
    (if (.content // "") != "" then "Content:\n\n\(.content|clip)\n" else "" end)] | join("\n")) +
  "\n## Generated outputs\n" +
  (if (.outputs|length) == 0 then "\nNone recorded.\n" else ([.outputs[] | "\n- `\(.path)` — \(.file_name), \(.mime_type), \(.size) bytes (id \(.id))"] | join("")) end)
' <<< "$json")

printf '%s\n' "$markdown"
repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../../" && pwd)
output_path="${DUMP_OUTPUT:-$repo_root/tmp/conversation-dumps/conversation-${resolved_id}.md}"
mkdir -p "$(dirname "$output_path")"
printf '%s\n' "$markdown" > "$output_path"
printf 'Saved dump to %s\n' "$output_path" >&2
