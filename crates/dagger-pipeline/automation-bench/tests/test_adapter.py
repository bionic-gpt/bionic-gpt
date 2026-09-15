import os
from pathlib import Path

import pytest


pytest.importorskip("fastapi")
if os.environ.get("AUTOMATIONBENCH_PATH"):
    from fastapi.testclient import TestClient
    from server.app import app


@pytest.mark.skipif(not os.environ.get("AUTOMATIONBENCH_PATH"), reason="requires AutomationBench checkout")
def test_admin_and_gmail_proxy():
    client = TestClient(app)
    assert client.get("/health").json() == {"status": "ok"}
    assert client.post("/admin/reset").status_code == 200
    response = client.get("/api/gmail/gmail/v1/users/me/messages")
    assert response.status_code == 200
    assert "messages" in response.json()
    assert client.get("/admin/world").status_code == 200
