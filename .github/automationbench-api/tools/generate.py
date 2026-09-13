#!/usr/bin/env python3
"""Generate permissive OpenAPI 3.1 documents from AutomationBench JSONC schemas."""
from __future__ import annotations

import argparse
import json
import re
import shutil
from pathlib import Path
from typing import Any

import yaml

INTERNAL_PREFIX = {
    "gmail": "", "google_calendar": "", "google_sheets": "sheets/",
    "google_ads": "googleads/v19/", "airtable": "airtable/v0/", "asana": "asana/1.0/",
    "buffer": "buffer/1/", "calendly": "calendly/", "canva": "canva/rest/v1/",
    "openai": "openai/v1/", "confluence": "confluence/wiki/", "docusign": "docusign/",
    "facebook_conversions": "facebook/conversions/v25/", "facebook_lead_ads": "facebook/lead_ads/v25/",
    "facebook_pages": "facebook/v25/", "google_drive": "", "linkedin_ads": "linkedin/ads/rest/",
    "linkedin_conversions": "linkedin/conversions/rest/", "freshdesk": "freshdesk/", "gorgias": "gorgias/",
    "helpcrunch": "helpcrunch/v1/", "helpscout": "helpscout/v2/", "hiver": "hiver/v1/",
    "hubspot": "hubspot/", "instagram": "instagram/v25/", "intercom": "intercom/",
    "jira": "jira/", "linkedin": "linkedin/v2/", "mailchimp": "mailchimp/", "monday": "monday/v2/",
    "notion": "notion/v1/", "pipefy": "pipefy/v1/", "quickbooks": "quickbooks/",
    "reamaze": "reamaze/v1/", "recruitee": "recruitee/v1/", "salesforce": "salesforce/",
    "slack": "slack/", "trello": "trello/1/", "twilio": "twilio/2010-04-01/", "twitter": "twitter/2/",
    "wave": "wave/", "xero": "xero/", "zendesk": "zendesk/", "zoho_desk": "zoho/v1/",
    "zoom": "zoom/v2/", "basecamp3": "basecamp3/", "bamboohr": "bamboohr/", "chatgpt": "openai/v1/",
}

def load_jsonc(path: Path) -> dict[str, Any]:
    text = "\n".join(line for line in path.read_text().splitlines() if not line.lstrip().startswith("//"))
    return json.loads(text)

def schema_ref(value: Any) -> dict[str, Any]:
    if isinstance(value, dict) and "$ref" in value:
        return {"$ref": "#/components/schemas/" + value["$ref"]}
    if isinstance(value, dict) and value.get("type") in {"object", "array", "string", "integer", "number", "boolean"}:
        result = {k: v for k, v in value.items() if k in {"type", "format", "description", "enum", "items", "additionalProperties"}}
        if "items" in result:
            result["items"] = schema_ref(result["items"])
        return result
    return {"type": "object", "additionalProperties": True}

def path_for(service: str, endpoint_path: str) -> str:
    prefix = INTERNAL_PREFIX.get(service, "")
    relative = endpoint_path.removeprefix(prefix)
    return "/api/" + service + "/" + relative.lstrip("/")

def operation(endpoint: dict[str, Any]) -> dict[str, Any]:
    result: dict[str, Any] = {
        "operationId": endpoint["id"],
        "summary": endpoint.get("description", endpoint["id"]),
        "description": endpoint.get("description", ""),
        "responses": {"200": {"description": endpoint.get("response", "Successful response"), "content": {"application/json": {"schema": {"type": "object", "additionalProperties": True}}}}},
    }
    parameters = []
    for name, info in (endpoint.get("parameters") or {}).items():
        info = info if isinstance(info, dict) else {}
        parameter = {"name": name, "in": info.get("location", "query"), "required": bool(info.get("required", False)), "schema": schema_ref(info),}
        if info.get("description"): parameter["description"] = info["description"]
        parameters.append(parameter)
    if parameters: result["parameters"] = parameters
    request = endpoint.get("request") or endpoint.get("requestBody")
    if request:
        result["requestBody"] = {"required": True, "content": {"application/json": {"schema": {"type": "object", "additionalProperties": True}}}}
    return result

def generate(automationbench: Path, output: Path) -> list[Path]:
    schemas_dir = automationbench / "automationbench/tools/api/schemas"
    openapi_dir = output / "openapi"
    openapi_dir.mkdir(parents=True, exist_ok=True)
    for stale in openapi_dir.glob("*.yaml"):
        stale.unlink()
    generated = []
    route_services = []
    schema_data = []
    for source in sorted(schemas_dir.glob("*.jsonc")):
        data = load_jsonc(source)
        schema_data.append(data)
        service = data["api"]
        document: dict[str, Any] = {"openapi": "3.1.0", "info": {"title": service, "version": str(data.get("version", "1.0.0"))}, "servers": [{"url": f"http://automationbench-api:8080/api/{service}"}], "paths": {}, "components": {"schemas": {}}}
        for name, definition in (data.get("schemas") or {}).items(): document["components"]["schemas"][name] = schema_ref(definition)
        used_routes: set[tuple[str, str]] = set()
        aliases: dict[str, str] = {}
        for endpoint in data.get("endpoints", []):
            route = "/" + path_for(service, endpoint["path"]).split("/", 3)[3]
            key = (endpoint["method"].lower(), route)
            if key in used_routes:
                alias = route.rstrip("/") + "/_operation/" + endpoint["id"]
                aliases[alias.lstrip("/")] = route.lstrip("/")
                route = alias
            used_routes.add((endpoint["method"].lower(), route))
            document["paths"].setdefault(route, {})[endpoint["method"].lower()] = operation(endpoint)
        target = openapi_dir / f"{service}.yaml"
        target.write_text(yaml.safe_dump(document, sort_keys=False, allow_unicode=True))
        generated.append(target)
        route_services.append({"name": service, "base_url": data.get("baseUrl", ""), "prefix": INTERNAL_PREFIX.get(service, ""), "aliases": aliases})
    (output / "routes.json").write_text(json.dumps({"services": route_services}, indent=2, sort_keys=True) + "\n")
    # Keep the generated directory self-contained for Docker builds.
    adapter_root = Path(__file__).resolve().parents[1]
    shutil.copytree(adapter_root / "server", output / "server", dirs_exist_ok=True)
    shutil.copy2(adapter_root / "requirements.txt", output / "requirements.txt")
    shutil.copy2(adapter_root / "Dockerfile", output / "Dockerfile")
    return generated

if __name__ == "__main__":
    parser = argparse.ArgumentParser(); parser.add_argument("--automationbench", type=Path, required=True); parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args(); generate(args.automationbench, args.output)
