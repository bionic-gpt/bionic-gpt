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


def test_generated_specs_preserve_nested_request_schemas(tmp_path: Path):
    source = tmp_path / "AutomationBench" / "automationbench/tools/api/schemas"
    source.mkdir(parents=True)
    (source / "gmail.jsonc").write_text(
        json.dumps(
            {
                "api": "gmail",
                "schemas": {
                    "Message": {
                        "type": "object",
                        "properties": {
                            "raw": {"type": "string"},
                            "payload": {
                                "$ref": "MessagePart"
                            },
                        },
                    },
                    "MessagePart": {
                        "type": "object",
                        "properties": {
                            "headers": {
                                "type": "array",
                                "items": {"$ref": "MessagePartHeader"},
                            }
                        },
                    },
                    "MessagePartHeader": {
                        "type": "object",
                        "properties": {
                            "name": {"type": "string"},
                            "value": {"type": "string"},
                        },
                    },
                    "Draft": {
                        "type": "object",
                        "properties": {"message": {"$ref": "Message"}},
                    },
                },
                "endpoints": [
                    {
                        "id": "gmail.users.drafts.create",
                        "method": "POST",
                        "path": "gmail/v1/users/{userId}/drafts",
                        "parameters": {
                            "userId": {
                                "type": "string",
                                "required": True,
                                "location": "path",
                            }
                        },
                        "request": "Draft: {message: Message with payload headers/body}",
                    }
                ],
            }
        )
    )

    output = tmp_path / "dist"
    generate(tmp_path / "AutomationBench", output)
    document = yaml.safe_load((output / "openapi/gmail.yaml").read_text())
    operation = document["paths"]["/gmail/v1/users/{userId}/drafts"]["post"]

    assert operation["parameters"][0]["name"] == "userId"
    assert operation["requestBody"]["content"]["application/json"]["schema"] == {
        "$ref": "#/components/schemas/Draft"
    }
    assert document["components"]["schemas"]["Draft"]["properties"]["message"] == {
        "$ref": "#/components/schemas/Message"
    }
    assert document["components"]["schemas"]["Message"]["properties"]["payload"] == {
        "$ref": "#/components/schemas/MessagePart"
    }
    assert document["components"]["schemas"]["MessagePart"]["properties"]["headers"]["items"] == {
        "$ref": "#/components/schemas/MessagePartHeader"
    }


def test_generated_specs_convert_body_parameters_to_request_body(tmp_path: Path):
    source = tmp_path / "AutomationBench" / "automationbench/tools/api/schemas"
    source.mkdir(parents=True)
    (source / "slack.jsonc").write_text(
        json.dumps(
            {
                "api": "slack",
                "endpoints": [
                    {
                        "id": "slack.messages.update",
                        "method": "POST",
                        "path": "api/chat.update",
                        "parameters": {
                            "channel": {
                                "type": "string",
                                "description": "Channel containing the message.",
                                "required": True,
                                "location": "body",
                            },
                            "ts": {
                                "type": "string",
                                "description": "Timestamp of the message.",
                                "required": True,
                                "location": "body",
                            },
                            "token": {
                                "type": "string",
                                "required": True,
                                "location": "query",
                            },
                        },
                    }
                ],
            }
        )
    )

    output = tmp_path / "dist"
    generate(tmp_path / "AutomationBench", output)
    document = yaml.safe_load((output / "openapi/slack.yaml").read_text())
    operation = document["paths"]["/api/chat.update"]["post"]

    assert operation["parameters"] == [
        {
            "name": "token",
            "in": "query",
            "required": True,
            "schema": {"type": "string"},
        }
    ]
    assert operation["requestBody"]["content"]["application/json"]["schema"] == {
        "type": "object",
        "properties": {
            "channel": {
                "type": "string",
                "description": "Channel containing the message.",
            },
            "ts": {
                "type": "string",
                "description": "Timestamp of the message.",
            },
        },
        "required": ["channel", "ts"],
    }
