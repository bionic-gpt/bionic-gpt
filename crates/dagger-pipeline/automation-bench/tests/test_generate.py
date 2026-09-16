import json
import sys
from pathlib import Path

import yaml

sys.path.insert(0, str(Path(__file__).parents[1] / "tools"))

import generate as generate_module
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


def test_generated_specs_omit_logo_for_unknown_services(tmp_path: Path):
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
    assert "x-logo" not in document["info"]


def test_generated_specs_use_explicit_logo_url(tmp_path: Path, monkeypatch):
    metadata_path = tmp_path / "service_metadata.json"
    metadata_path.write_text(
        json.dumps(
            {
                "custom_service": {
                    "title": "Custom Service",
                    "logo_url": "https://example.com/custom.svg",
                }
            }
        )
    )
    monkeypatch.setattr(generate_module, "METADATA_PATH", metadata_path)

    source = tmp_path / "AutomationBench" / "automationbench/tools/api/schemas"
    source.mkdir(parents=True)
    (source / "custom.jsonc").write_text(
        json.dumps({"api": "custom_service", "endpoints": []})
    )

    output = tmp_path / "dist"
    generate(tmp_path / "AutomationBench", output)
    document = yaml.safe_load((output / "openapi/custom_service.yaml").read_text())

    assert document["info"]["x-logo"]["url"] == "https://example.com/custom.svg"


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


def test_generated_specs_parse_named_salesforce_update_fields(tmp_path: Path):
    source = tmp_path / "AutomationBench" / "automationbench/tools/api/schemas"
    source.mkdir(parents=True)
    (source / "salesforce.jsonc").write_text(
        json.dumps(
            {
                "api": "salesforce",
                "schemas": {
                    "Contact": {
                        "type": "object",
                        "properties": {
                            "Id": {"type": "string"},
                            "FirstName": {"type": "string"},
                            "Email": {"type": "string"},
                            "Phone": {"type": "string"},
                            "AnnualRevenue": {"type": "number"},
                        },
                    }
                },
                "endpoints": [
                    {
                        "id": "salesforce.sobjects.contact.update",
                        "method": "PATCH",
                        "path": "salesforce/services/data/v61.0/sobjects/Contact/{id}",
                        "parameters": {
                            "id": {
                                "type": "string",
                                "required": True,
                                "location": "path",
                            }
                        },
                        "request": "Contact fields to update: {FirstName, Email, Phone}",
                    }
                ],
            }
        )
    )

    output = tmp_path / "dist"
    generate(tmp_path / "AutomationBench", output)
    document = yaml.safe_load((output / "openapi/salesforce.yaml").read_text())
    operation = document["paths"]["/services/data/v61.0/sobjects/Contact/{id}"]["patch"]
    schema = operation["requestBody"]["content"]["application/json"]["schema"]

    assert list(schema["properties"]) == ["FirstName", "Email", "Phone"]
    assert schema["properties"]["Phone"] == {"type": "string"}
    assert schema["properties"]["Email"] == {"type": "string", "format": "email"}
    assert "Id" not in schema["properties"]
    assert "Contact fields to update" in schema["description"]
    assert "Phone" in operation["description"]


def test_generated_specs_document_generic_salesforce_record_updates(tmp_path: Path):
    source = tmp_path / "AutomationBench" / "automationbench/tools/api/schemas"
    source.mkdir(parents=True)
    (source / "salesforce.jsonc").write_text(
        json.dumps(
            {
                "api": "salesforce",
                "endpoints": [
                    {
                        "id": "salesforce.sobjects.record.update",
                        "method": "PATCH",
                        "path": "salesforce/services/data/v61.0/sobjects/{sObjectType}/{id}",
                        "parameters": {
                            "sObjectType": {"type": "string", "location": "path"},
                            "id": {"type": "string", "location": "path"},
                        },
                        "request": "JSON object of field updates",
                    }
                ],
            }
        )
    )

    output = tmp_path / "dist"
    generate(tmp_path / "AutomationBench", output)
    document = yaml.safe_load((output / "openapi/salesforce.yaml").read_text())
    operation = document["paths"]["/services/data/v61.0/sobjects/{sObjectType}/{id}"]["patch"]
    schema = operation["requestBody"]["content"]["application/json"]["schema"]

    assert schema["type"] == "object"
    assert schema["additionalProperties"] is True
    assert schema["description"] == "JSON object of field updates"
    assert "JSON object of field updates" in operation["description"]


def test_generated_specs_parse_nested_typed_and_binary_request_fields(tmp_path: Path):
    source = tmp_path / "AutomationBench" / "automationbench/tools/api/schemas"
    source.mkdir(parents=True)
    (source / "example.jsonc").write_text(
        json.dumps(
            {
                "api": "example",
                "endpoints": [
                    {
                        "id": "example.create",
                        "method": "POST",
                        "path": "example/create",
                        "request": "Body: {email (required, string), dueDate (date), updatedAt (ISO 8601 datetime), attachment (binary)}",
                    },
                    {
                        "id": "example.optional",
                        "method": "POST",
                        "path": "example/optional",
                        "request": "Body (all optional): {inputs: [{name (required, string), value (number)}]}",
                    },
                ],
            }
        )
    )

    output = tmp_path / "dist"
    generate(tmp_path / "AutomationBench", output)
    document = yaml.safe_load((output / "openapi/example.yaml").read_text())

    create = document["paths"]["/create"]["post"]
    properties = create["requestBody"]["content"]["multipart/form-data"]["schema"]["properties"]
    assert create["requestBody"]["required"] is True
    assert properties["email"] == {"type": "string", "format": "email"}
    assert properties["dueDate"] == {"type": "string", "format": "date"}
    assert properties["updatedAt"] == {"type": "string", "format": "date-time"}
    assert properties["attachment"] == {"type": "string", "format": "binary"}

    optional = document["paths"]["/optional"]["post"]
    optional_schema = optional["requestBody"]["content"]["application/json"]["schema"]
    assert optional["requestBody"]["required"] is False
    assert optional_schema["properties"]["inputs"]["type"] == "array"
    assert optional_schema["properties"]["inputs"]["items"]["required"] == ["name"]


def test_generated_specs_use_component_refs_and_structured_responses(tmp_path: Path):
    source = tmp_path / "AutomationBench" / "automationbench/tools/api/schemas"
    source.mkdir(parents=True)
    (source / "mailchimp.jsonc").write_text(
        json.dumps(
            {
                "api": "mailchimp",
                "schemas": {
                    "List": {"type": "object", "properties": {"id": {"type": "string"}}},
                    "Member": {"type": "object", "properties": {"email": {"type": "string", "format": "email"}}},
                },
                "endpoints": [
                    {
                        "id": "mailchimp.lists.list",
                        "method": "GET",
                        "path": "3.0/lists",
                        "response": {"lists": ["List"], "total_items": "integer"},
                    },
                    {
                        "id": "mailchimp.lists.members.delete",
                        "method": "DELETE",
                        "path": "3.0/lists/{list_id}/members/{subscriber_hash}",
                        "response": "Empty response (HTTP 204)",
                    },
                ],
            }
        )
    )

    output = tmp_path / "dist"
    generate(tmp_path / "AutomationBench", output)
    document = yaml.safe_load((output / "openapi/mailchimp.yaml").read_text())

    list_operation = document["paths"]["/3.0/lists"]["get"]
    response_schema = list_operation["responses"]["200"]["content"]["application/json"]["schema"]
    assert response_schema["properties"]["lists"]["items"] == {"$ref": "#/components/schemas/List"}
    assert response_schema["properties"]["total_items"] == {"type": "integer"}

    delete_operation = document["paths"]["/3.0/lists/{list_id}/members/{subscriber_hash}"]["delete"]
    assert delete_operation["responses"] == {"204": {"description": "Empty response (HTTP 204)"}}


def test_binary_detection_handles_recursive_component_schemas():
    schemas = {
        "Node": {
            "type": "object",
            "properties": {"next": {"$ref": "#/components/schemas/Node"}},
        },
        "FileNode": {
            "type": "object",
            "properties": {
                "next": {"$ref": "#/components/schemas/FileNode"},
                "file": {"type": "string", "format": "binary"},
            },
        },
    }

    assert generate_module.contains_binary(
        {"$ref": "#/components/schemas/Node"}, schemas
    ) is False
    assert generate_module.contains_binary(
        {"$ref": "#/components/schemas/FileNode"}, schemas
    ) is True
