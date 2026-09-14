from __future__ import annotations

import asyncio
import inspect
import json
import os
import tempfile
from pathlib import Path
from typing import Any

from fastapi import HTTPException, Request, UploadFile

from .catalog import Operation, configure_import_paths, load_operation

_CALL_LOCK = asyncio.Lock()


def _safe_name(name: str) -> str:
    candidate = Path(name).name
    if not candidate or candidate in {".", ".."} or candidate != name:
        raise HTTPException(status_code=400, detail=f"Unsafe filename: {name}")
    return candidate


def _json_or_value(value: str) -> Any:
    try:
        return json.loads(value)
    except json.JSONDecodeError:
        return value


def _set_root_on_loaded_modules(root: Path) -> None:
    os.environ["APP_FS_ROOT"] = str(root)
    os.environ["FILESYSTEM_ROOT"] = str(root)
    os.environ["APP_DOCS_ROOT"] = str(root)
    os.environ["APP_SHEETS_ROOT"] = str(root)
    os.environ["APP_SLIDES_ROOT"] = str(root)
    for module in list(__import__("sys").modules.values()):
        if module is None:
            continue
        for attribute in (
            "TARGET_AGENT_FILESYSTEM_ROOT",
            "FILESYSTEM_ROOT",
            "DOCS_ROOT",
            "SHEETS_ROOT",
            "SLIDES_ROOT",
        ):
            if hasattr(module, attribute):
                try:
                    setattr(module, attribute, str(root))
                except (AttributeError, TypeError):
                    pass


async def _form_payload(request: Request, root: Path) -> dict[str, Any]:
    form = await request.form()
    payload: dict[str, Any] = {}
    for key, value in form.multi_items():
        if isinstance(value, UploadFile):
            filename = _safe_name(value.filename or key)
            target = root / filename
            target.write_bytes(await value.read())
            payload[key] = f"/{filename}"
        else:
            payload[key] = _json_or_value(str(value))
    if "input" in payload and isinstance(payload["input"], dict):
        nested = payload.pop("input")
        nested.update(payload)
        return nested
    return payload


def _model_from_signature(function: Any) -> Any:
    parameter = next(iter(inspect.signature(function).parameters.values()), None)
    if parameter is None:
        return None
    annotation = inspect.get_annotations(function, eval_str=True).get(parameter.name)
    if annotation is None or annotation is inspect.Parameter.empty:
        return None
    return annotation


async def call_operation(operation: Operation, request: Request) -> tuple[Any, bytes | None, str | None]:
    with tempfile.TemporaryDirectory(prefix="office-tools-") as temp:
        root = Path(temp)
        upstream_root = Path(os.environ.get("ARCHIPELAGO_ROOT", "/opt/archipelago"))
        configure_import_paths(upstream_root, operation.domain)
        payload = await _form_payload(request, root)
        async with _CALL_LOCK:
            _set_root_on_loaded_modules(root)
            function, _ = load_operation(operation, upstream_root)
            _set_root_on_loaded_modules(root)
            model_type = _model_from_signature(function)
            try:
                argument = model_type.model_validate(payload) if model_type is not None else payload
                result = function(argument) if model_type is not None else function(**payload)
                if inspect.isawaitable(result):
                    result = await result
            except HTTPException:
                raise
            except Exception as exc:
                raise HTTPException(status_code=422, detail=str(exc)) from exc

        files = [path for path in root.rglob("*") if path.is_file()]
        output = files[-1] if files else None
        return result, output.read_bytes() if output else None, output.name if output else None
