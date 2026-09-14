#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
from pathlib import Path

from openapi_spec_validator import validate


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--openapi", type=Path, required=True)
    args = parser.parse_args()
    for path in sorted(args.openapi.glob("*.openapi.json")):
        document = json.loads(path.read_text())
        validate(document)
        assert document["openapi"].startswith("3.")
        ids = [operation["operationId"] for item in document["paths"].values() for operation in item.values()]
        assert len(ids) == len(set(ids)), f"duplicate operationId in {path}"
        assert ids, f"no operations in {path}"
        print(f"{path}: {len(ids)} operations")


if __name__ == "__main__":
    main()
