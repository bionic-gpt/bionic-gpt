#!/usr/bin/env bash
set -euo pipefail

IMAGE="${AUTOMATIONBENCH_IMAGE:-ghcr.io/bionic-gpt/automationbench-api:4a8e1061254004d9dac807054eed33fad7d1ff14}"
: "${BIONIC_DATABASE_URL:?Set BIONIC_DATABASE_URL}"
: "${BIONIC_TEAM_ID:?Set BIONIC_TEAM_ID}"
: "${BIONIC_USER_ID:?Set BIONIC_USER_ID}"

for command_name in docker psql base64; do
  command -v "$command_name" >/dev/null || { echo "$command_name is required" >&2; exit 1; }
done

tmp_dir=$(mktemp -d)
container="automationbench-import-$$"
cleanup() { docker rm "$container" >/dev/null 2>&1 || true; rm -rf "$tmp_dir"; }
trap cleanup EXIT

docker pull "$IMAGE" >/dev/null
docker create --name "$container" "$IMAGE" >/dev/null
docker cp "$container:/app/openapi" "$tmp_dir/openapi" >/dev/null

psql "$BIONIC_DATABASE_URL" -v ON_ERROR_STOP=1 <<SQL
BEGIN;
DELETE FROM integrations.integrations WHERE team_id = ${BIONIC_TEAM_ID};
COMMIT;
SQL

for spec in "$tmp_dir"/openapi/*.yaml; do
  name=$(basename "$spec" .yaml)
  encoded=$(docker run --rm -i mikefarah/yq:4 -o=json '.' < "$spec" | base64 -w0)
  psql "$BIONIC_DATABASE_URL" -v ON_ERROR_STOP=1 -v name="$name" -f - <<SQL >/dev/null
INSERT INTO integrations.integrations
  (team_id, name, definition, integration_type, visibility, created_by)
VALUES
  (${BIONIC_TEAM_ID}, :'name', convert_from(decode('$encoded','base64'),'UTF8')::jsonb,
   'OpenAPI', 'Team', ${BIONIC_USER_ID});
SQL
done

echo "Loaded $(find "$tmp_dir/openapi" -name '*.yaml' | wc -l) AutomationBench integrations for team $BIONIC_TEAM_ID"
