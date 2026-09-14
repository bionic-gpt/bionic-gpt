from server.catalog import OPERATIONS


def test_catalog_exposes_individual_upstream_operations():
    by_domain = {}
    for operation in OPERATIONS:
        by_domain.setdefault(operation.domain, []).append(operation.name)

    assert len(by_domain["documents"]) == 15
    assert len(by_domain["spreadsheets"]) == 12
    assert len(by_domain["presentations"]) == 13
    assert "docs_schema" not in by_domain["documents"]
    assert "sheets_schema" not in by_domain["spreadsheets"]
    assert "slides_schema" not in by_domain["presentations"]
