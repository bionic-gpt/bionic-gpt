from __future__ import annotations

import json
import mimetypes
import os
from pathlib import Path

from fastapi import FastAPI, Request
from fastapi.responses import JSONResponse, Response
from fastapi.staticfiles import StaticFiles

from .catalog import OPERATIONS
from .common import call_operation

app = FastAPI(title="Bionic Office Tools", version="1.0.0")
openapi_dir = Path(os.environ.get("OFFICE_OPENAPI_ROOT", "/app/openapi"))
if openapi_dir.exists():
    app.mount("/openapi", StaticFiles(directory=openapi_dir), name="office-openapi")


@app.get("/health")
def health() -> dict[str, str]:
    return {"status": "ok"}


def _json_result(value):
    if hasattr(value, "model_dump"):
        return value.model_dump(mode="json")
    if isinstance(value, str):
        try:
            return json.loads(value)
        except json.JSONDecodeError:
            return {"result": value}
    return value


async def _invoke(operation, request: Request):
    result, output_bytes, output_name = await call_operation(operation, request)
    if operation.name == "read_image" and hasattr(result, "data"):
        image_format = getattr(result, "format", "jpeg")
        return Response(result.data, media_type=f"image/{image_format}")
    if operation.binary_response and output_bytes and output_name:
        content_type = mimetypes.guess_type(output_name)[0] or "application/octet-stream"
        return Response(
            output_bytes,
            media_type=content_type,
            headers={"Content-Disposition": f'attachment; filename="{output_name}"'},
        )
    return JSONResponse(_json_result(result))


def _register() -> None:
    for operation in OPERATIONS:
        async def handler(request: Request, operation=operation):
            return await _invoke(operation, request)

        handler.__name__ = operation.name
        app.add_api_route(
            f"/{operation.domain}/{operation.name}",
            handler,
            methods=["POST"],
            name=operation.name,
            operation_id=operation.name,
        )


_register()
