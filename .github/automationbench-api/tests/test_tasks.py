import os

import pytest

pytest.importorskip("fastapi")

if os.environ.get("AUTOMATIONBENCH_PATH"):
    from fastapi.testclient import TestClient
    from server.app import app


@pytest.mark.skipif(not os.environ.get("AUTOMATIONBENCH_PATH"), reason="requires AutomationBench checkout")
def test_benchmark_reset_loads_named_task():
    response = TestClient(app).post("/benchmark/reset", json={"task": "simple.email_sf_contact_phone_update"})
    assert response.status_code == 200
    body = response.json()
    assert body["task"] == "simple.email_sf_contact_phone_update"
    assert "gmail" in body["allowed_services"]
    assert "salesforce" in body["allowed_services"]


@pytest.mark.skipif(not os.environ.get("AUTOMATIONBENCH_PATH"), reason="requires AutomationBench checkout")
def test_benchmark_reset_rejects_unknown_task():
    response = TestClient(app).post("/benchmark/reset", json={"task": "does.not.exist"})
    assert response.status_code == 404
