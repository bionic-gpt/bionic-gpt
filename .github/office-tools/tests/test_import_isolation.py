import os
from pathlib import Path

import pytest

from server.catalog import load_operation, operation_by_name


ROOT = Path(os.environ["ARCHIPELAGO_ROOT"]) if os.environ.get("ARCHIPELAGO_ROOT") else None
pytestmark = pytest.mark.skipif(ROOT is None or not ROOT.exists(), reason="staged Archipelago source is unavailable")


def schema_origin() -> str:
    import mcp_schema

    return str(Path(mcp_schema.__file__).resolve())


def test_domain_schema_packages_are_isolated():
    assert ROOT is not None
    load_operation(operation_by_name("documents", "create_document"), ROOT)
    document_schema = schema_origin()
    assert "/documents/" in document_schema

    load_operation(operation_by_name("spreadsheets", "create_spreadsheet"), ROOT)
    spreadsheet_schema = schema_origin()
    assert "/spreadsheets/" in spreadsheet_schema

    load_operation(operation_by_name("presentations", "create_deck"), ROOT)
    presentation_schema = schema_origin()
    assert "/presentations/" in presentation_schema

    assert len({document_schema, spreadsheet_schema, presentation_schema}) == 3
