#!/usr/bin/env bash
set -euo pipefail

database_url="${DATABASE_URL:-${APP_DATABASE_URL:-}}"
if [[ -z "$database_url" ]]; then
  echo "DATABASE_URL or APP_DATABASE_URL is required" >&2
  exit 1
fi

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)
spec_dir="${1:-$repo_root/crates/automation-bench/specs}"

command -v psql >/dev/null || { echo "psql is required" >&2; exit 1; }
command -v python3 >/dev/null || { echo "python3 is required" >&2; exit 1; }

shopt -s nullglob
spec_files=("$spec_dir"/*.openapi.json)
if [[ ${#spec_files[@]} -eq 0 ]]; then
  echo "No OpenAPI JSON files found in $spec_dir" >&2
  exit 1
fi

tmp_dir=$(mktemp -d)
trap 'rm -rf "$tmp_dir"' EXIT

loaded=0
for spec_file in "${spec_files[@]}"; do
  sql_file="$tmp_dir/$(basename "$spec_file").sql"

  python3 - "$spec_file" >"$sql_file" <<'PY'
import json
import sys
from pathlib import Path


def sql_literal(value):
    if value is None:
        return "NULL"

    text = str(value)
    tag = "bionic_spec"
    while f"${tag}$" in text:
        tag += "_"
    return f"${tag}${text}${tag}$"


path = Path(sys.argv[1])
document = json.loads(path.read_text())
info = document.get("info") or {}
suffix = ".openapi.json"
slug = path.name[:-len(suffix)] if path.name.endswith(suffix) else path.stem

# The local AutomationBench service is deliberately authentication-free. Keep
# business parameters intact, but remove standard OpenAPI auth declarations and
# explicit access-token parameters from the imported simulator contract.
document.pop("security", None)
components = document.get("components")
if isinstance(components, dict):
    components.pop("securitySchemes", None)
    if not components:
        document.pop("components", None)

for path_item in (document.get("paths") or {}).values():
    if not isinstance(path_item, dict):
        continue
    for method, operation in path_item.items():
        if method.lower() not in {
            "get", "post", "put", "patch", "delete", "options", "head", "trace"
        } or not isinstance(operation, dict):
            continue
        operation.pop("security", None)
        operation["parameters"] = [
            parameter
            for parameter in operation.get("parameters", [])
            if not isinstance(parameter, dict)
            or parameter.get("name", "").lower()
            not in {"authorization", "access_token", "api_key", "apikey"}
        ]

document["servers"] = [{"url": f"http://automationbench-api:8080/api/{slug}"}]
title = info.get("title") or slug
description = info.get("description")
logo_url = (info.get("x-logo") or {}).get("url")
spec = json.dumps(document, ensure_ascii=False, separators=(",", ":"))

print(
    "INSERT INTO integrations.openapi_specs "
    "(slug, title, description, spec, logo_url, category, is_active, is_system)\n"
    f"VALUES ({sql_literal(slug)}, {sql_literal(title)}, "
    f"{sql_literal(description)}, {sql_literal(spec)}::jsonb, "
    f"{sql_literal(logo_url)}, 'Application', TRUE, FALSE)\n"
    "ON CONFLICT (slug) DO UPDATE SET\n"
    "title = EXCLUDED.title,\n"
    "description = EXCLUDED.description,\n"
    "spec = EXCLUDED.spec,\n"
    "logo_url = EXCLUDED.logo_url,\n"
    "category = EXCLUDED.category,\n"
    "is_active = TRUE,\n"
    "is_system = FALSE,\n"
    "updated_at = NOW();"
)
PY

  psql "$database_url" -v ON_ERROR_STOP=1 -f "$sql_file" >/dev/null
  loaded=$((loaded + 1))
done

echo "Loaded $loaded AutomationBench OpenAPI specs into integrations.openapi_specs"
