import json
import sys
from pathlib import Path

import yaml

sys.path.insert(0, str(Path(__file__).parents[1] / "tools"))

from generate import generate


def test_generated_specs_include_service_metadata(tmp_path: Path):
    source = tmp_path / "AutomationBench" / "automationbench/tools/api/schemas"
    source.mkdir(parents=True)
    (source / "gmail.jsonc").write_text(
        json.dumps(
            {
                "api": "gmail",
                "version": "1.0.0",
                "endpoints": [
                    {
                        "id": "list_messages",
                        "method": "GET",
                        "path": "/gmail/v1/messages"
                    }
                ]
            }
        )
    )

    output = tmp_path / "dist"
    generate(tmp_path / "AutomationBench", output)
    document = yaml.safe_load((output / "openapi/gmail.yaml").read_text())

    assert document["info"]["title"] == "Gmail"
    assert document["info"]["description"]
    assert document["info"]["x-logo"]["url"].endswith("/gmail")


def test_generated_specs_use_fallback_metadata_for_unknown_services(tmp_path: Path):
    source = tmp_path / "AutomationBench" / "automationbench/tools/api/schemas"
    source.mkdir(parents=True)
    (source / "custom.jsonc").write_text(
        json.dumps(
            {
                "api": "custom_service",
                "endpoints": []
            }
        )
    )

    output = tmp_path / "dist"
    generate(tmp_path / "AutomationBench", output)
    document = yaml.safe_load((output / "openapi/custom_service.yaml").read_text())

    assert document["info"]["title"] == "Custom Service"
    assert "deterministic AutomationBench" in document["info"]["description"]
    assert document["info"]["x-logo"]["url"].endswith("/customservice")
