from __future__ import annotations

import json
import os
import sys
from pathlib import Path
from threading import Lock

from fastapi import FastAPI, Request
from fastapi.responses import JSONResponse, Response
from fastapi.staticfiles import StaticFiles

automationbench_path = Path(os.environ.get("AUTOMATIONBENCH_PATH", "/opt/automationbench"))
if str(automationbench_path) not in sys.path:
    sys.path.insert(0, str(automationbench_path))

from automationbench.schema.world import WorldState  # noqa: E402
from automationbench.tools.api.fetch import api_fetch  # noqa: E402

app = FastAPI(title="AutomationBench API adapter", version="1.0.0")
openapi_dir = Path(os.environ.get("AUTOMATIONBENCH_OPENAPI", "/app/openapi"))
if openapi_dir.exists():
    app.mount("/openapi", StaticFiles(directory=openapi_dir), name="generated-openapi")
world = WorldState()
world_lock = Lock()


def _world_json() -> dict:
    return world.model_dump(mode="json")


def _load_routes() -> dict[str, dict[str, str]]:
    path = Path(os.environ.get("AUTOMATIONBENCH_ROUTES", "/app/routes.json"))
    if not path.exists():
        return {}
    return {item["name"]: item for item in json.loads(path.read_text()).get("services", [])}


@app.get("/health")
def health() -> dict[str, str]:
    return {"status": "ok"}


@app.post("/admin/reset")
def reset() -> dict:
    global world
    with world_lock:
        world = WorldState()
        return _world_json()


@app.post("/admin/world")
async def replace_world(request: Request) -> dict:
    payload = await request.json()
    initial_state = payload.get("initial_state", payload) if isinstance(payload, dict) else {}
    if not isinstance(initial_state, dict):
        return JSONResponse({"error": "initial_state must be an object"}, status_code=422)
    # Treat explicit null top-level services like omitted services so callers can
    # pass partially populated AutomationBench fixtures.
    initial_state = {key: value for key, value in initial_state.items() if value is not None}
    global world
    try:
        replacement = WorldState(**initial_state)
    except Exception as exc:
        return JSONResponse({"error": str(exc)}, status_code=422)
    with world_lock:
        world = replacement
        return _world_json()


@app.get("/admin/world")
def get_world() -> dict:
    with world_lock:
        return _world_json()


@app.api_route("/api/{service}/{path:path}", methods=["GET", "POST", "PUT", "PATCH", "DELETE"])
async def api_proxy(service: str, path: str, request: Request):
    route = _load_routes().get(service)
    if route is None:
        return JSONResponse({"error": {"code": 404, "message": f"Unknown service: {service}"}}, status_code=404)
    prefix = route.get("prefix", "")
    internal_path = f"{prefix}{path}".lstrip("/")
    real_path = internal_path.removeprefix(prefix).lstrip("/")
    url = route["base_url"].rstrip("/") + "/" + real_path
    query: dict[str, object] = {}
    for key, value in request.query_params.multi_items():
        if key in query:
            query[key] = [query[key], value] if not isinstance(query[key], list) else query[key] + [value]
        else:
            query[key] = value
    raw_body = await request.body()
    with world_lock:
        result = api_fetch(world, request.method, url, json.dumps(query) if query else None, raw_body.decode() if raw_body else None)
    try:
        payload = json.loads(result)
    except json.JSONDecodeError:
        return Response(result, media_type="application/json")
    error = payload.get("error") if isinstance(payload, dict) else None
    status = int(error.get("code", 200)) if isinstance(error, dict) and str(error.get("code", "")).isdigit() else 200
    return JSONResponse(payload, status_code=status)
