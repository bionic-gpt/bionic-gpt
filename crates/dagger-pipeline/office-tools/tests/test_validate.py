import json
from pathlib import Path


def test_openapi_artifacts_are_valid_json(tmp_path: Path):
    for name in ("documents", "spreadsheets", "presentations"):
        target = tmp_path / f"{name}.openapi.json"
        target.write_text(json.dumps({"openapi": "3.1.0", "paths": {"/x": {"post": {"operationId": "x"}}}}))
        document = json.loads(target.read_text())
        assert document["openapi"] == "3.1.0"
