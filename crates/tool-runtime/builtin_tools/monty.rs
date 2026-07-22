use crate::types::ToolDefinition;
use monty::MontyRun;
use monty_types::{CompileOptions, LimitedTracker, PrintWriter, ResourceLimits};
use rig::tool::{ToolDyn, ToolError};
use rig::wasm_compat::WasmBoxedFuture;
use serde::Deserialize;
use serde_json::{json, Value};
use std::time::{Duration, Instant};

const DEFAULT_TIMEOUT_MS: u64 = 5_000;
const MAX_TIMEOUT_MS: u64 = 30_000;
const DEFAULT_MAX_MEMORY_BYTES: usize = 64 * 1024 * 1024;
const DEFAULT_MAX_ALLOCATIONS: usize = 1_000_000;

#[derive(Debug, Deserialize)]
struct RunPythonArgs {
    code: String,
    timeout_ms: Option<u64>,
}

/// A tool that runs hermetic Python snippets in Monty.
pub struct MontyTool;

impl ToolDyn for MontyTool {
    fn name(&self) -> String {
        get_tool_definition().name
    }

    fn description(&self) -> String {
        get_tool_definition().description
    }

    fn parameters(&self) -> Value {
        get_tool_definition().parameters
    }

    fn call(&self, args: String) -> WasmBoxedFuture<'_, Result<String, ToolError>> {
        Box::pin(async move {
            let arguments: RunPythonArgs =
                serde_json::from_str(&args).map_err(ToolError::JsonError)?;

            let result = tokio::task::spawn_blocking(move || execute_run_python(arguments))
                .await
                .map_err(|err| {
                    ToolError::ToolCallError(Box::new(std::io::Error::other(err.to_string())))
                })?;

            serde_json::to_string(&result).map_err(ToolError::JsonError)
        })
    }
}

pub fn get_tool_definition() -> ToolDefinition {
    ToolDefinition {
        name: "run_python".to_string(),
        description: "Run a short, hermetic Python snippet with Monty. Use this for calculations, data shaping, and small programs. The sandbox has no access to the host filesystem, environment variables, network, third-party Python packages, or Bionic tools. Return values and print output are captured.".to_string(),
        parameters: json!({
            "type": "object",
            "properties": {
                "code": {
                    "type": "string",
                    "description": "Python source code to execute."
                },
                "timeout_ms": {
                    "type": "integer",
                    "minimum": 100,
                    "maximum": MAX_TIMEOUT_MS,
                    "description": "Optional execution timeout in milliseconds. Defaults to 5000."
                }
            },
            "required": ["code"]
        }),
    }
}

fn execute_run_python(arguments: RunPythonArgs) -> Value {
    if arguments.code.trim().is_empty() {
        return json!({"error": "code is required"});
    }

    let timeout = arguments
        .timeout_ms
        .unwrap_or(DEFAULT_TIMEOUT_MS)
        .clamp(100, MAX_TIMEOUT_MS);

    let runner = match MontyRun::new(arguments.code, "tool.py", vec![], CompileOptions::default()) {
        Ok(runner) => runner,
        Err(err) => return json!({"error": err.to_string()}),
    };

    let started = Instant::now();
    let mut stdout = String::new();

    let limits = ResourceLimits::new()
        .max_duration(Duration::from_millis(timeout))
        .max_memory(DEFAULT_MAX_MEMORY_BYTES)
        .max_allocations(DEFAULT_MAX_ALLOCATIONS);

    let result = match runner.run(
        vec![],
        LimitedTracker::new(limits),
        PrintWriter::collect_string(&mut stdout),
    ) {
        Ok(result) => result,
        Err(err) => {
            return json!({
                "stdout": stdout,
                "stderr": "",
                "error": err.to_string(),
                "duration_ms": started.elapsed().as_millis()
            });
        }
    };

    json!({
        "stdout": stdout,
        "stderr": "",
        "result": serde_json::to_value(&result).unwrap_or_else(|_| json!({"repr": result.to_string()})),
        "repr": result.to_string(),
        "duration_ms": started.elapsed().as_millis()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_tool_definition() {
        let tool = get_tool_definition();
        assert_eq!(tool.name, "run_python");
    }

    #[test]
    fn test_rejects_empty_code() {
        let result = execute_run_python(RunPythonArgs {
            code: "   ".to_string(),
            timeout_ms: None,
        });
        assert_eq!(result["error"], "code is required");
    }
}
