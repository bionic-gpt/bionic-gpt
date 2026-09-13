#!/usr/bin/env bash
set -euo pipefail

IMAGE="${AUTOMATIONBENCH_IMAGE:-ghcr.io/bionic-gpt/automationbench-api:4a8e1061254004d9dac807054eed33fad7d1ff14}"
: "${BIONIC_DATABASE_URL:?Set BIONIC_DATABASE_URL}"
: "${BIONIC_TEAM_ID:?Set BIONIC_TEAM_ID}"
: "${BIONIC_USER_ID:?Set BIONIC_USER_ID}"

for command_name in docker psql yq; do
  command -v "$command_name" >/dev/null || { echo "$command_name is required" >&2; exit 1; }
done

tmp_dir=$(mktemp -d)
container="automationbench-import-$$"
cleanup() { docker rm "$container" >/dev/null 2>&1 || true; rm -rf "$tmp_dir"; }
trap cleanup EXIT

docker pull "$IMAGE" >/dev/null
docker create --name "$container" "$IMAGE" >/dev/null
docker cp "$container:/app/openapi" "$tmp_dir/openapi" >/dev/null

for spec in "$tmp_dir"/openapi/*.yaml; do
  name=$(basename "$spec" .yaml)
  definition=$(yq -o=json '.' "$spec")
  psql "$BIONIC_DATABASE_URL" -v ON_ERROR_STOP=1 \
    -v team_id="$BIONIC_TEAM_ID" -v user_id="$BIONIC_USER_ID" \
    -v name="$name" -v definition="$definition" <<'SQL'
WITH updated AS (
  UPDATE integrations.integrations
  SET definition = :'definition'::jsonb, visibility = 'Team', updated_at = NOW()
  WHERE team_id = :'team_id' AND name = :'name'
  RETURNING id
)
INSERT INTO integrations.integrations
  (team_id, name, definition, integration_type, visibility, created_by)
SELECT :'team_id', :'name', :'definition'::jsonb, 'OpenAPI', 'Team', :'user_id'
WHERE NOT EXISTS (SELECT 1 FROM updated);
SQL
done

echo "AutomationBench integrations loaded for team $BIONIC_TEAM_ID"
