# Office Tools Adapter

This image exposes the individual document, spreadsheet, and presentation
operations from [Mercor Intelligence Archipelago](https://github.com/Mercor-Intelligence/archipelago)
through HTTP and three generated OpenAPI documents. The workflow records the
exact upstream commit in `archipelago-commit.txt` and publishes an image tagged
with that commit.

Archipelago is Apache-2.0. The staged image preserves upstream license and
notice files. The files under `server/` and `tools/` are Bionic adapter code;
the staged `archipelago/` tree is upstream code selected by the workflow.

The adapter deliberately does not expose Archipelago MCP meta-tools or schema
tools. File parameters use ordinary multipart binary fields and edited Office
files are returned as binary HTTP responses for Bionic's existing OpenAPI/VFS
bridge.
