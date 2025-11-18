# Understanding Tool Calls

Tool calls let an assistant hand off specific pieces of work to deterministic code, so every capability has a predictable interface. This page explains when to create a tool versus a text-only response and how to encode arguments for reliability.

We cover the request lifecycle, how structured responses are marshalled in Rust, and what telemetry you can collect to monitor usage.
