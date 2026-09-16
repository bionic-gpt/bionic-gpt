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

METADATA_PATH = Path(__file__).resolve().parents[1] / "service_metadata.json"

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
        reference = value["$ref"]
        if reference.startswith("#/"):
            return {"$ref": reference}
        return {"$ref": "#/components/schemas/" + reference}
    if isinstance(value, dict) and any(
        key in value
        for key in (
            "type",
            "format",
            "description",
            "enum",
            "items",
            "additionalProperties",
            "properties",
            "required",
        )
    ):
        result = {
            k: v
            for k, v in value.items()
            if k in {
                "type",
                "format",
                "description",
                "enum",
                "items",
                "additionalProperties",
                "properties",
                "required",
            }
        }
        if "items" in result:
            result["items"] = schema_ref(result["items"])
        if "properties" in result:
            result["properties"] = {
                name: schema_for_property(name, property_schema)
                for name, property_schema in result["properties"].items()
            }
        if isinstance(result.get("additionalProperties"), dict):
            result["additionalProperties"] = schema_ref(result["additionalProperties"])
        return result
    return {"type": "object", "additionalProperties": True}


def schema_for_property(name: str, value: Any) -> dict[str, Any]:
    result = schema_ref(value)
    if result.get("type") == "string" and "format" not in result:
        inferred = annotation_schema(name, "")
        if "format" in inferred:
            result["format"] = inferred["format"]
    return result


def split_fields(value: str) -> list[str]:
    fields = []
    start = 0
    depth = 0
    for index, character in enumerate(value):
        if character in "[{(":
            depth += 1
        elif character in "]})":
            depth -= 1
        elif character == "," and depth == 0:
            fields.append(value[start:index].strip())
            start = index + 1
    fields.append(value[start:].strip())
    return [field for field in fields if field]


def annotation_schema(name: str, annotation: str) -> dict[str, Any]:
    lower = annotation.lower()
    if "binary" in lower or "file" in lower:
        return {"type": "string", "format": "binary"}
    if "date-time" in lower or "datetime" in lower or "iso 8601" in lower:
        return {"type": "string", "format": "date-time"}
    if re.search(r"\bdate\b", lower) or name.lower().endswith("date"):
        return {"type": "string", "format": "date"}
    if re.search(r"\bemail\b", lower) or "email" in name.lower():
        return {"type": "string", "format": "email"}
    if "boolean" in lower or re.search(r"\bbool\b", lower):
        return {"type": "boolean"}
    if "integer" in lower or re.search(r"\bint\b", lower):
        return {"type": "integer"}
    if "number" in lower or "float" in lower:
        return {"type": "number"}
    if "array" in lower or "list" in lower:
        return {"type": "array", "items": {"type": "string"}}
    if "object" in lower or "json" in lower:
        return {"type": "object", "additionalProperties": True}
    return {"type": "string"}


def parsed_field(name: str, detail: str, schemas: dict[str, Any]) -> dict[str, Any]:
    name = name.strip().rstrip("?")
    detail = detail.strip()
    required = "required" in detail.lower()
    nested = re.search(r"(?P<open>[\[{])(?P<body>.*)(?P<close>[\]}])", detail)
    if nested:
        nested_schema = parsed_object(nested.group("body"), schemas)
        if nested.group("open") == "[":
            result = {"type": "array", "items": nested_schema}
        else:
            result = nested_schema
    else:
        reference = re.search(r"\b([A-Z][A-Za-z0-9_]*)\b", detail)
        if reference and reference.group(1) in schemas:
            result = {"$ref": f"#/components/schemas/{reference.group(1)}"}
        else:
            result = annotation_schema(name, detail)
    result["x-source-required"] = required
    return result


def parsed_object(value: str, schemas: dict[str, Any]) -> dict[str, Any]:
    properties: dict[str, Any] = {}
    required = []
    for field in split_fields(value):
        name, separator, detail = field.partition(":")
        if not separator:
            name, separator, detail = field.partition(" (")
            detail = f"({detail}" if separator else ""
        name = name.strip().rstrip("?")
        if not name:
            continue
        property_schema = parsed_field(name, detail, schemas)
        is_required = property_schema.pop("x-source-required", False)
        properties[name] = property_schema
        if is_required:
            required.append(name)
    result: dict[str, Any] = {"type": "object", "properties": properties}
    if required:
        result["required"] = required
    return result


def response_schema(response: Any, schemas: dict[str, Any]) -> dict[str, Any]:
    if isinstance(response, dict):
        properties = {}
        for name, value in response.items():
            if isinstance(value, list) and len(value) == 1:
                item = value[0]
                item_schema = (
                    {"$ref": f"#/components/schemas/{item}"}
                    if isinstance(item, str) and item in schemas
                    else response_schema(item, schemas)
                )
                properties[name] = {
                    "type": "array",
                    "items": item_schema,
                }
            elif isinstance(value, str) and value in schemas:
                properties[name] = {"$ref": f"#/components/schemas/{value}"}
            elif isinstance(value, bool):
                properties[name] = {"type": "boolean"}
            elif isinstance(value, dict):
                properties[name] = response_schema(value, schemas)
            else:
                properties[name] = annotation_schema(name, str(value))
        return {"type": "object", "properties": properties}

    text = str(response or "")
    if re.search(r"\b204\b|no content|empty response", text, re.IGNORECASE):
        return {}
    if text.lstrip().startswith("{") and "}" in text:
        return parsed_object(text[text.index("{") + 1 : text.rfind("}")], schemas)
    return {"type": "object", "additionalProperties": True}


def contains_binary(
    value: Any,
    schemas: dict[str, Any] | None = None,
    seen_refs: set[str] | None = None,
) -> bool:
    if seen_refs is None:
        seen_refs = set()
    if isinstance(value, dict):
        if value.get("format") == "binary":
            return True
        reference = value.get("$ref")
        if schemas is not None and isinstance(reference, str):
            schema_name = reference.rsplit("/", 1)[-1]
            if schema_name in schemas:
                if reference in seen_refs:
                    return False
                seen_refs.add(reference)
                result = contains_binary(schemas[schema_name], schemas, seen_refs)
                seen_refs.remove(reference)
                return result
        return any(contains_binary(item, schemas, seen_refs) for item in value.values())
    if isinstance(value, list):
        return any(contains_binary(item, schemas, seen_refs) for item in value)
    return False


def request_schema(request: Any, schemas: dict[str, Any]) -> dict[str, Any]:
    if isinstance(request, dict):
        return schema_ref(request.get("schema", request))

    if isinstance(request, str):
        field_list = re.search(
            r"^\s*([A-Za-z_][A-Za-z0-9_]*)\s+fields(?:\s+to\s+update)?\s*:\s*\{(?P<fields>[^}]*)\}",
            request,
            re.IGNORECASE,
        )
        if field_list:
            schema_name = field_list.group(1)
            source_schema = schemas.get(schema_name)
            fields = []
            required = []
            for item in field_list.group("fields").split(","):
                item = item.strip()
                if not item:
                    continue
                name, _, annotations = item.partition(" (")
                name = name.strip()
                if not name:
                    continue
                fields.append(name)
                if "required" in annotations.lower():
                    required.append(name)

            if source_schema and fields:
                source_properties = source_schema.get("properties", {})
                properties = {
                    name: schema_for_property(name, source_properties[name])
                    for name in fields
                    if name in source_properties
                }
                body_schema: dict[str, Any] = {
                    "type": "object",
                    "description": request.strip(),
                    "properties": properties,
                }
                if required:
                    body_schema["required"] = [name for name in required if name in properties]
                if "additionalProperties" in source_schema:
                    additional_properties = source_schema["additionalProperties"]
                    body_schema["additionalProperties"] = (
                        additional_properties
                        if isinstance(additional_properties, bool)
                        else schema_ref(additional_properties)
                    )
                return body_schema

        match = re.match(r"^([A-Za-z_][A-Za-z0-9_]*)\s*:", request)
        if match and match.group(1) in schemas:
            return {"$ref": f"#/components/schemas/{match.group(1)}"}

        body_start = re.search(r"\{", request)
        if body_start:
            body_end = request.rfind("}")
            if body_end > body_start.start():
                body_schema = parsed_object(
                    request[body_start.end() : body_end], schemas
                )
                body_schema["description"] = request.strip()
                return body_schema

    return {
        "type": "object",
        "description": str(request).strip() if request else "JSON object body.",
        "additionalProperties": True,
    }


def parameter_schema(info: dict[str, Any]) -> dict[str, Any]:
    raw_schema = info.get("schema")
    if raw_schema is None:
        raw_schema = {
            key: value
            for key, value in info.items()
            if key not in {"description", "location", "required"}
        }
    return schema_ref(raw_schema)

def path_for(service: str, endpoint_path: str) -> str:
    prefix = INTERNAL_PREFIX.get(service, "")
    relative = endpoint_path.removeprefix(prefix)
    return "/api/" + service + "/" + relative.lstrip("/")

def service_metadata(service: str) -> dict[str, str]:
    metadata = json.loads(METADATA_PATH.read_text()).get(service, {})
    title = metadata.get("title", service.replace("_", " ").title())
    result = {
        "title": title,
        "description": metadata.get(
            "description",
            f"Simulated {title} API for deterministic AutomationBench workflow evaluation.",
        ),
    }
    logo_url = metadata.get("logo_url")
    if logo_url is None and metadata.get("logo_slug"):
        logo_url = f"https://cdn.simpleicons.org/{metadata['logo_slug']}"
    if logo_url:
        result["logo_url"] = logo_url
    return result

def operation(endpoint: dict[str, Any], schemas: dict[str, Any]) -> dict[str, Any]:
    response = endpoint.get("response", "Successful response")
    response_description = (
        response if isinstance(response, str) else "Structured response"
    )
    response_status = "204" if response_schema(response, schemas) == {} else "200"
    responses: dict[str, Any] = {
        response_status: {
            "description": response_description or "Successful response",
        }
    }
    if response_status != "204":
        responses[response_status]["content"] = {
            "application/json": {"schema": response_schema(response, schemas)}
        }
    result: dict[str, Any] = {
        "operationId": endpoint["id"],
        "summary": endpoint.get("description", endpoint["id"]),
        "description": endpoint.get("description", ""),
        "responses": responses,
    }
    parameters = []
    body_parameters = []
    for name, info in (endpoint.get("parameters") or {}).items():
        info = info if isinstance(info, dict) else {}
        location = info.get("location", "query")
        if location == "body":
            body_parameters.append((name, info))
            continue
        parameter = {
            "name": name,
            "in": location,
            "required": bool(info.get("required", False)),
            "schema": parameter_schema(info),
        }
        if info.get("description"): parameter["description"] = info["description"]
        parameters.append(parameter)
    if parameters: result["parameters"] = parameters
    request = endpoint.get("request") or endpoint.get("requestBody")
    if request or body_parameters:
        if request:
            body_schema = request_schema(request, schemas)
        else:
            properties = {}
            required = []
            for name, info in body_parameters:
                property_schema = parameter_schema(info)
                if info.get("description"):
                    property_schema["description"] = info["description"]
                properties[name] = property_schema
                if info.get("required", False):
                    required.append(name)
            body_schema = {"type": "object", "properties": properties}
            if required:
                body_schema["required"] = required
        body_description = body_schema.get("description")
        if body_description and body_description not in result["description"]:
            result["description"] = (
                f"{result['description']} Request body: {body_description}."
            ).strip()
        request_required = True
        if isinstance(request, str) and re.search(r"\ball optional\b", request, re.IGNORECASE):
            request_required = False
        elif body_parameters:
            request_required = any(info.get("required", False) for _, info in body_parameters)
        elif isinstance(request, dict):
            request_required = bool(request.get("required", True))
        content_type = (
            "multipart/form-data"
            if contains_binary(body_schema, schemas)
            else "application/json"
        )
        result["requestBody"] = {
            "required": request_required,
            "content": {
                content_type: {"schema": body_schema}
            },
        }
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
        metadata = service_metadata(service)
        info: dict[str, Any] = {
            "title": metadata["title"],
            "version": str(data.get("version", "1.0.0")),
            "description": metadata["description"],
        }
        if metadata.get("logo_url"):
            info["x-logo"] = {"url": metadata["logo_url"]}
        document: dict[str, Any] = {
            "openapi": "3.1.0",
            "info": info,
            "servers": [{"url": f"http://automationbench-api:8080/api/{service}"}],
            "paths": {},
            "components": {"schemas": {}},
        }
        schemas = data.get("schemas") or {}
        for name, definition in schemas.items(): document["components"]["schemas"][name] = schema_ref(definition)
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
            document["paths"].setdefault(route, {})[endpoint["method"].lower()] = operation(endpoint, schemas)
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
