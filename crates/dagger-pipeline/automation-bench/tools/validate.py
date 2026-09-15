#!/usr/bin/env python3
from __future__ import annotations
import argparse
from pathlib import Path
import yaml
from generate import load_jsonc

def validate(automationbench: Path, output: Path) -> None:
    sources = sorted((automationbench / "automationbench/tools/api/schemas").glob("*.jsonc"))
    documents = sorted((output / "openapi").glob("*.yaml"))
    assert len(sources) == len(documents), f"expected {len(sources)} OpenAPI documents, found {len(documents)}"
    ids = set()
    for document_path in documents:
        document = yaml.safe_load(document_path.read_text())
        assert document["openapi"].startswith("3.")
        info = document["info"]
        assert info.get("description"), f"missing description in {document_path.name}"
        assert info.get("x-logo", {}).get("url"), f"missing logo in {document_path.name}"
        expected_server = f"http://automationbench-api:8080/api/{document_path.stem}"
        assert all(server["url"] == expected_server for server in document["servers"])
        for methods in document["paths"].values():
            for method, operation in methods.items():
                if method.lower() not in {"get", "post", "put", "patch", "delete", "options", "head"}: continue
                assert operation["operationId"] not in ids, operation["operationId"]
                ids.add(operation["operationId"])
                for parameter in operation.get("parameters", []):
                    assert parameter["in"] != "body", (
                        f"legacy body parameter in {document_path.name}: "
                        f"{operation['operationId']}"
                    )
    source_ids = {endpoint["id"] for path in sources for endpoint in load_jsonc(path).get("endpoints", [])}
    assert ids == source_ids, f"operation coverage mismatch: {source_ids - ids} / {ids - source_ids}"

if __name__ == "__main__":
    parser = argparse.ArgumentParser(); parser.add_argument("--automationbench", type=Path, required=True); parser.add_argument("--output", type=Path, required=True); args = parser.parse_args(); validate(args.automationbench, args.output); print("validated")
