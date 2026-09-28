use async_trait::async_trait;
use sandbox::{
    BashkitSandbox, Command, Credential, CredentialSet, OpenApiSpec, RunRequest, Sandbox,
    SandboxTool, SandboxToolDefinition, SandboxToolError, ToolCallContext, ToolExposure,
    WorkspaceSnapshot,
};
use serde_json::Value;
use std::sync::Arc;
use std::time::Duration;

struct EchoTool;

#[async_trait]
impl SandboxTool for EchoTool {
    fn definition(&self) -> SandboxToolDefinition {
        SandboxToolDefinition {
            name: "echo_callback".to_string(),
            description: "Echo a value".to_string(),
            parameters: serde_json::json!({"type": "object"}),
            exposure: ToolExposure::Python,
            credential: None,
        }
    }

    async fn call(
        &self,
        arguments: Value,
        _context: ToolCallContext<'_>,
    ) -> Result<Value, SandboxToolError> {
        Ok(arguments.get("value").cloned().unwrap_or(Value::Null))
    }
}

#[tokio::test]
async fn workspace_changes_are_reported() {
    let result = BashkitSandbox
        .run(RunRequest {
            command: Command::WriteFile {
                path: "/home/user/work/example.txt".to_string(),
                contents: b"new".to_vec(),
            },
            skills: vec![],
            openapi_specs: vec![],
            credentials: CredentialSet::default(),
            workspace: WorkspaceSnapshot::default(),
            tools: vec![],
        })
        .await
        .unwrap();
    assert_eq!(result.workspace_changes.upserted.len(), 1);
    assert_eq!(result.workspace_changes.upserted[0].contents, b"new");
}

#[tokio::test]
async fn python_calls_supplied_callback_tools() {
    let result = BashkitSandbox
        .run(RunRequest {
            command: Command::Python {
                code: "print(echo_callback(value='scheduled'))".to_string(),
                timeout: Duration::from_secs(5),
            },
            skills: vec![],
            openapi_specs: vec![],
            credentials: CredentialSet::default(),
            workspace: WorkspaceSnapshot::default(),
            tools: vec![Arc::new(EchoTool)],
        })
        .await
        .unwrap();
    assert_eq!(result.execution.stdout, "scheduled\n");
    assert_eq!(result.execution.exit_code, 0);
}

#[tokio::test]
async fn openapi_specs_are_exposed_in_the_filesystem() {
    let result = BashkitSandbox
        .run(RunRequest {
            command: Command::ReadFile {
                path: "/home/user/functions/calendar.openapi.json".to_string(),
            },
            skills: vec![],
            openapi_specs: vec![OpenApiSpec {
                name: "calendar".to_string(),
                document: serde_json::json!({"openapi": "3.0.0"}),
            }],
            credentials: CredentialSet::default(),
            workspace: WorkspaceSnapshot::default(),
            tools: vec![],
        })
        .await
        .unwrap();
    let contents = result.execution.data.unwrap();
    assert!(String::from_utf8(contents).unwrap().contains("3.0.0"));
}

#[test]
fn credentials_do_not_debug_secrets() {
    let mut credentials = CredentialSet::default();
    credentials.insert(
        "integration",
        Credential::Bearer {
            token: "secret".to_string(),
        },
    );
    let debug = format!("{credentials:?}");
    assert!(!debug.contains("secret"));
}
