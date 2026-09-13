from __future__ import annotations

import json
import os
import sys
import importlib
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
from automationbench.runner import compute_allowed_services, strip_none_values  # noqa: E402

app = FastAPI(title="AutomationBench API adapter", version="1.0.0")
openapi_dir = Path(os.environ.get("AUTOMATIONBENCH_OPENAPI", "/app/openapi"))
if openapi_dir.exists():
    app.mount("/openapi", StaticFiles(directory=openapi_dir), name="generated-openapi")
world = WorldState()
world_lock = Lock()
_tasks: dict[str, dict] | None = None


def _world_json() -> dict:
    return world.model_dump(mode="json")


def _load_routes() -> dict[str, dict[str, str]]:
    path = Path(os.environ.get("AUTOMATIONBENCH_ROUTES", "/app/routes.json"))
    if not path.exists():
        return {}
    return {item["name"]: item for item in json.loads(path.read_text()).get("services", [])}


def _load_tasks() -> dict[str, dict]:
    global _tasks
    if _tasks is not None:
        return _tasks
    tasks: dict[str, dict] = {}
    for domain in ("simple", "sales", "marketing", "operations", "support", "finance", "hr"):
        module = importlib.import_module(f"automationbench.domains.{domain}.tasks")
        dataset = module.get_simple_dataset() if domain == "simple" else module.__dict__[f"get_{domain}_dataset"]()
        for row in dataset:
            info = row.get("info", {})
            if isinstance(info, str):
                info = json.loads(info)
            name = info.get("task_name") or row.get("task")
            if name:
                tasks[name] = {"prompt": row.get("prompt", []), "info": info}
    _tasks = tasks
    return tasks


@app.get("/health")
def health() -> dict[str, str]:
    return {"status": "ok"}


@app.post("/admin/reset")
def reset() -> dict:
    global world
    with world_lock:
        world = WorldState()
        return _world_json()


@app.post("/benchmark/reset")
async def benchmark_reset(request: Request):
    payload = await request.json()
    task_name = payload.get("task") if isinstance(payload, dict) else None
    if not isinstance(task_name, str) or not task_name:
        return JSONResponse({"error": "task is required"}, status_code=400)
    task = _load_tasks().get(task_name)
    if task is None:
        return JSONResponse({"error": {"code": 404, "message": f"Unknown AutomationBench task: {task_name}"}}, status_code=404)
    info = task["info"]
    initial_state = strip_none_values(info.get("initial_state", {}))
    assertions = info.get("assertions", [])
    zapier_tools = info.get("zapier_tools", [])
    allowed_services = compute_allowed_services(initial_state, assertions, zapier_tools)
    replacement = WorldState(**initial_state)
    replacement.meta.allowed_services = allowed_services
    global world
    with world_lock:
        world = replacement
        return {"task": task_name, "allowed_services": allowed_services, "world": _world_json()}


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
    path = route.get("aliases", {}).get(path, path)
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
