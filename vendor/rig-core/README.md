# Local rig-core patch

This directory temporarily vendors `rig-core` 0.42.0 so Bionic can preserve
malformed streamed tool calls as recoverable model errors. It is pinned through
the workspace `[patch.crates-io]` entry.

Replace this directory and the Cargo patch when the upstream Rig issue is
released. Keep the malformed streamed tool-call regression test in Bionic
until that release is verified.
