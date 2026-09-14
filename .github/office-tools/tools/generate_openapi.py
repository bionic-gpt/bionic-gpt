#!/usr/bin/env python3
from __future__ import annotations

import argparse
import inspect
import json
import os
import sys
from pathlib import Path

from pydantic import TypeAdapter

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from server.catalog import OPERATIONS, configure_import_paths, load_operation  # noqa: E402


def schema_for(operation, root: Path) -> dict:
    function, _ = load_operation(operation, root)
    parameters = list(inspect.signature(function).parameters.values())
    annotations = inspect.get_annotations(function, eval_str=True)
    if not parameters:
        return {"type": "object", "additionalProperties": True}
    annotation = annotations.get(parameters[0].name)
    if len(parameters) == 1 and hasattr(annotation, "model_json_schema"):
        schema = annotation.model_json_schema(ref_template="#/components/schemas/{model}")
    else:
        properties = {}
        required = []
        for parameter in parameters:
            annotation = annotations.get(parameter.name, object)
            properties[parameter.name] = TypeAdapter(annotation).json_schema()
            if parameter.default is inspect.Parameter.empty:
                required.append(parameter.name)
        schema = {"type": "object", "properties": properties}
        if required:
            schema["required"] = required
    return rewrite_file_fields(schema)


def rewrite_file_fields(value):
    if isinstance(value, dict):
        result = {key: rewrite_file_fields(item) for key, item in value.items()}
        properties = result.get("properties", {})
        for name, schema in properties.items():
            if name.endswith("_path") and name in {"file_path", "image_path", "spreadsheet_path", "presentation_path"}:
                schema.clear()
                schema.update({"type": "string", "format": "binary", "description": "Office file upload"})
        return result
    if isinstance(value, list):
        return [rewrite_file_fields(item) for item in value]
    return value


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--staged", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    sys.path.insert(0, str(args.staged))
    os.environ["ARCHIPELAGO_ROOT"] = str(args.staged / "archipelago")
    configure_import_paths(Path(os.environ["ARCHIPELAGO_ROOT"]))
    args.output.mkdir(parents=True, exist_ok=True)
    for domain in ("documents", "spreadsheets", "presentations"):
        document = {
            "openapi": "3.1.0",
            "info": {"title": f"Bionic {domain.title()} Office API", "version": "1.0.0"},
            "servers": [{"url": "http://office-tools:8080"}],
            "paths": {},
            "components": {"schemas": {}},
        }
        for operation in (item for item in OPERATIONS if item.domain == domain):
            schema = schema_for(operation, Path(os.environ["ARCHIPELAGO_ROOT"]))
            document["paths"][f"/{domain}/{operation.name}"] = {
                "post": {
                    "operationId": operation.name,
                    "summary": operation.name.replace("_", " ").capitalize(),
                    "description": inspect.getdoc(load_operation(operation, Path(os.environ["ARCHIPELAGO_ROOT"]))[0]) or f"Mercor Archipelago {operation.name} operation.",
                    "requestBody": {"required": True, "content": {"multipart/form-data": {"schema": schema}}},
                    "responses": {
                        "200": {"description": "Successful response", "content": {"application/octet-stream" if operation.binary_response else "application/json": {"schema": {"type": "string", "format": "binary"} if operation.binary_response else {"type": "object", "additionalProperties": True}}}},
                        "422": {"description": "Invalid operation input"},
                    },
                }
            }
        (args.output / f"{domain}.openapi.json").write_text(json.dumps(document, indent=2) + "\n")


if __name__ == "__main__":
    main()
