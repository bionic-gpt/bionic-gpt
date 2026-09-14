from fastapi.testclient import TestClient


def test_health(monkeypatch):
    monkeypatch.setenv("ARCHIPELAGO_ROOT", "/tmp/archipelago")
    from server.main import app

    assert TestClient(app).get("/health").json() == {"status": "ok"}
