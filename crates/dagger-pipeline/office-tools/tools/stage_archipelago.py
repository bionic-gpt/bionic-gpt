#!/usr/bin/env python3
from __future__ import annotations

import argparse
import shutil
from pathlib import Path


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--archipelago", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    target = args.output / "archipelago"
    if target.exists():
        shutil.rmtree(target)
    target.mkdir(parents=True)
    for domain, server in (("documents", "docs_server"), ("spreadsheets", "sheets_server"), ("presentations", "slides_server")):
        source = args.archipelago / "mcp_servers" / domain
        destination = target / domain
        destination.mkdir(parents=True)
        for name in ("mcp_servers", "packages"):
            source_path = source / name
            if source_path.exists():
                shutil.copytree(source_path, destination / name)
    for name in ("LICENSE", "NOTICE"):
        source = args.archipelago / name
        if source.exists():
            shutil.copy2(source, args.output / name)
    (args.output / "archipelago-root.txt").write_text(str(target.resolve()) + "\n")


if __name__ == "__main__":
    main()
