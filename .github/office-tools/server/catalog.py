from __future__ import annotations

import importlib
import inspect
import os
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any


@dataclass(frozen=True)
class Operation:
    domain: str
    name: str
    module: str
    function: str
    binary_response: bool = False


OPERATIONS: tuple[Operation, ...] = (
    Operation("documents", "create_document", "tools.create_document", "create_document", True),
    Operation("documents", "delete_document", "tools.delete_document", "delete_document"),
    Operation("documents", "get_document_overview", "tools.get_document_overview", "get_document_overview"),
    Operation("documents", "read_document_content", "tools.read_document_content", "read_document_content"),
    Operation("documents", "read_image", "tools.read_image", "read_image", True),
    Operation("documents", "add_content_text", "tools.add_content_text", "add_content_text", True),
    Operation("documents", "edit_content_text", "tools.edit_content_text", "edit_content_text", True),
    Operation("documents", "delete_content_text", "tools.delete_content_text", "delete_content_text", True),
    Operation("documents", "add_image", "tools.add_image", "add_image", True),
    Operation("documents", "modify_image", "tools.modify_image", "modify_image", True),
    Operation("documents", "apply_formatting", "tools.apply_formatting", "apply_formatting", True),
    Operation("documents", "header_footer", "tools.header_footer", "header_footer", True),
    Operation("documents", "page_margins", "tools.page_margins", "page_margins", True),
    Operation("documents", "page_orientation", "tools.page_orientation", "page_orientation", True),
    Operation("documents", "comments", "tools.comments", "comments", True),
    Operation("spreadsheets", "create_spreadsheet", "tools.create_spreadsheet", "create_spreadsheet", True),
    Operation("spreadsheets", "delete_spreadsheet", "tools.delete_spreadsheet", "delete_spreadsheet"),
    Operation("spreadsheets", "read_tab", "tools.read_tab", "read_tab"),
    Operation("spreadsheets", "read_csv", "tools.read_csv", "read_csv"),
    Operation("spreadsheets", "list_tabs_in_spreadsheet", "tools.list_tabs_in_spreadsheet", "list_tabs_in_spreadsheet"),
    Operation("spreadsheets", "add_tab", "tools.add_tab", "add_tab", True),
    Operation("spreadsheets", "delete_tab", "tools.delete_tab", "delete_tab", True),
    Operation("spreadsheets", "edit_spreadsheet", "tools.edit_spreadsheet", "edit_spreadsheet", True),
    Operation("spreadsheets", "add_content_text", "tools.add_content_text", "add_content_text", True),
    Operation("spreadsheets", "delete_content_cell", "tools.delete_content_cell", "delete_content_cell", True),
    Operation("spreadsheets", "create_chart", "tools.create_chart", "create_chart", True),
    Operation("spreadsheets", "filter_tab", "tools.filter_tab", "filter_tab", True),
    Operation("presentations", "create_deck", "tools.create_slides", "create_deck", True),
    Operation("presentations", "delete_deck", "tools.delete_slides", "delete_deck"),
    Operation("presentations", "add_slide", "tools.add_slide", "add_slide", True),
    Operation("presentations", "edit_slides", "tools.edit_slides", "edit_slides", True),
    Operation("presentations", "add_image", "tools.add_image", "add_image", True),
    Operation("presentations", "modify_image", "tools.modify_image", "modify_image", True),
    Operation("presentations", "insert_chart", "tools.insert_chart", "insert_chart", True),
    Operation("presentations", "insert_table", "tools.insert_table", "insert_table", True),
    Operation("presentations", "add_shape", "tools.add_shape", "add_shape", True),
    Operation("presentations", "read_slides", "tools.read_slides", "read_slides"),
    Operation("presentations", "read_completedeck", "tools.read_completedeck", "read_completedeck"),
    Operation("presentations", "read_individualslide", "tools.read_individualslide", "read_individualslide"),
    Operation("presentations", "read_image", "tools.read_image", "read_image", True),
)


def configure_import_paths(root: Path, domain: str | None = None) -> None:
    staged_root = str(root.resolve())
    sys.path[:] = [path for path in sys.path if not str(path).startswith(staged_root)]
    domains = (("documents", "docs_server"), ("spreadsheets", "sheets_server"), ("presentations", "slides_server"))
    if domain:
        domains = tuple(item for item in domains if item[0] == domain)
    for domain_name, server_name in domains:
        server = root / domain_name / "mcp_servers" / server_name
        package_root = root / domain_name / "packages"
        package_paths = (package_root, package_root / "mcp_schema", package_root / "mercor-mcp-shared" / "packages")
        for path in (server, server / "tools", server / "utils", server / "models", *package_paths):
            if path.exists() and str(path) not in sys.path:
                sys.path.insert(0, str(path))
    os.environ.setdefault("APP_FS_ROOT", "/tmp/office-workspace")


def load_operation(operation: Operation, root: Path) -> tuple[Any, Any]:
    configure_import_paths(root, operation.domain)
    for module_name in list(sys.modules):
        if module_name in {"tools", "utils", "models", "mcp_schema"} or module_name.startswith(("tools.", "utils.", "models.", "mcp_schema.")):
            del sys.modules[module_name]
    module = importlib.import_module(operation.module)
    function = getattr(module, operation.function)
    signature = inspect.signature(function)
    parameter = next(iter(signature.parameters.values()), None)
    model = None
    if parameter is not None and parameter.annotation not in (inspect.Parameter.empty, Any):
        annotation = parameter.annotation
        model = getattr(annotation, "model_validate", None)
    return function, parameter.annotation if parameter is not None else None


def operation_by_name(domain: str, name: str) -> Operation:
    for operation in OPERATIONS:
        if operation.domain == domain and operation.name == name:
            return operation
    raise KeyError(f"Unknown {domain} operation: {name}")
