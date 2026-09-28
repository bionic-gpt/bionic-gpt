//! Model-tool to sandbox dispatch.
//!
//! This is intentionally independent of database and authorization concerns:
//! the agent harness supplies a fully resolved `SandboxInputs` for every call.

use crate::ToolCall;
use sandbox::{
    Command, CredentialSet, OpenApiSpec, RunRequest, RunResult, Sandbox, SandboxError, SandboxFile,
    SandboxTool, WorkspaceSnapshot,
};
use serde::Deserialize;
use std::sync::Arc;
use std::time::Duration;

const DEFAULT_TIMEOUT_MS: u64 = 5_000;
const MAX_TIMEOUT_MS: u64 = 30_000;

pub struct SandboxInputs {
    pub skills: Vec<SandboxFile>,
    pub openapi_specs: Vec<OpenApiSpec>,
    pub credentials: CredentialSet,
    pub workspace: WorkspaceSnapshot,
    pub tools: Vec<Arc<dyn SandboxTool>>,
}

pub async fn dispatch_to_sandbox(
    sandbox: &dyn Sandbox,
    call: &ToolCall,
    inputs: SandboxInputs,
) -> Result<RunResult, SandboxError> {
    let command = command_for_call(call)?;
    sandbox
        .run(RunRequest {
            command,
            skills: inputs.skills,
            openapi_specs: inputs.openapi_specs,
            credentials: inputs.credentials,
            workspace: inputs.workspace,
            tools: inputs.tools,
        })
        .await
}

fn command_for_call(call: &ToolCall) -> Result<Command, SandboxError> {
    let arguments = call.function.arguments.clone();
    match call.function.name.as_str() {
        "run_bash" => {
            let arguments: RunBashArgs = serde_json::from_value(arguments).map_err(json_error)?;
            if arguments.commands.trim().is_empty() {
                return Err(SandboxError("commands is required".to_string()));
            }
            let timeout_ms = arguments
                .timeout_ms
                .unwrap_or(DEFAULT_TIMEOUT_MS)
                .clamp(100, MAX_TIMEOUT_MS);
            Ok(Command::Shell {
                script: arguments.commands,
                timeout: Duration::from_millis(timeout_ms),
            })
        }
        "run_python" => {
            let arguments: PythonArgs = serde_json::from_value(arguments).map_err(json_error)?;
            Ok(Command::Python {
                code: arguments.code,
                timeout: Duration::from_millis(MAX_TIMEOUT_MS),
            })
        }
        "read_file" => {
            let arguments: ReadArgs = serde_json::from_value(arguments).map_err(json_error)?;
            Ok(Command::ReadFile {
                path: arguments.path,
            })
        }
        "write_file" => {
            let arguments: WriteArgs = serde_json::from_value(arguments).map_err(json_error)?;
            Ok(Command::WriteFile {
                path: arguments.path,
                contents: arguments.content.into_bytes(),
            })
        }
        "edit_file" => {
            let arguments: EditArgs = serde_json::from_value(arguments).map_err(json_error)?;
            Ok(Command::EditFile {
                path: arguments.path,
                find: arguments.find.into_bytes(),
                replace: arguments.replace.into_bytes(),
            })
        }
        name => Err(SandboxError(format!("unknown sandbox tool: {name}"))),
    }
}

fn json_error(error: serde_json::Error) -> SandboxError {
    SandboxError(format!("invalid tool arguments: {error}"))
}

#[derive(Deserialize)]
struct RunBashArgs {
    commands: String,
    timeout_ms: Option<u64>,
}

#[derive(Deserialize)]
struct PythonArgs {
    code: String,
}

#[derive(Deserialize)]
struct ReadArgs {
    path: String,
}

#[derive(Deserialize)]
struct WriteArgs {
    path: String,
    content: String,
}

#[derive(Deserialize)]
struct EditArgs {
    path: String,
    find: String,
    replace: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ToolCallFunction;
    use rig::message::ToolCallId;
    use serde_json::json;

    #[test]
    fn maps_model_tools_to_provider_neutral_commands() {
        let call = ToolCall::new(
            ToolCallId::new_or_mint("call-1"),
            ToolCallFunction::new(
                "run_bash".to_string(),
                json!({"commands": "pwd", "timeout_ms": 750}),
            ),
        );
        let command = command_for_call(&call).unwrap();
        assert!(matches!(
            command,
            Command::Shell { script, timeout }
                if script == "pwd" && timeout == Duration::from_millis(750)
        ));
    }

    #[test]
    fn rejects_unknown_model_tools() {
        let call = ToolCall::new(
            ToolCallId::new_or_mint("call-1"),
            ToolCallFunction::new("not_a_tool".to_string(), json!({})),
        );
        assert!(command_for_call(&call).is_err());
    }
}
