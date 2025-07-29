use crate::tool::ToolInterface;
use async_trait::async_trait;
use base64::{engine::general_purpose, Engine as _};
use db::Pool;
use k8s_openapi::api::batch::v1::Job as K8sJob;
use k8s_openapi::api::core::v1::{ConfigMap, Pod};
use k8s_operator::{job, MANAGER};
use kube::api::{DeleteParams, ListParams, Patch, PatchParams};
use kube::{Api, Client};
use object_storage;
use openai_api::{BionicToolDefinition, ChatCompletionFunctionDefinition};
use rand::{distributions::Alphanumeric, Rng};
use serde::Deserialize;
use serde_json::{json, Map, Value};
use tokio::time::{sleep, Duration};
use tracing;

#[derive(Deserialize)]
struct CodeInterpreterParams {
    code: String,
    lang: String,
    args: Option<String>,
    files: Option<Vec<i32>>,
    entity_id: Option<String>,
}

/// Tool for executing short code snippets inside a sandbox
pub struct CodeInterpreterTool {
    pool: Pool,
    kube: Client,
    namespace: String,
    user_id: i32,
    team_id: i32,
}

impl CodeInterpreterTool {
    pub fn new(pool: Pool, kube: Client, namespace: String, user_id: i32, team_id: i32) -> Self {
        Self {
            pool,
            kube,
            namespace,
            user_id,
            team_id,
        }
    }
}

pub fn get_code_interpreter_tool() -> BionicToolDefinition {
    BionicToolDefinition {
        r#type: "function".to_string(),
        function: ChatCompletionFunctionDefinition {
            name: "run_code".to_string(),
            description: "Execute code inside a temporary sandbox".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "code": {"type": "string", "description": "Source code to execute"},
                    "lang": {"type": "string", "description": "Language of code", "enum": ["py", "js", "rs", "bash"]},
                    "args": {"type": "string", "description": "Optional command line arguments"},
                    "files": {"type": "array", "items": {"type": "integer"}, "description": "File IDs to mount"},
                    "entity_id": {"type": "string", "description": "Optional context identifier"}
                },
                "required": ["code", "lang"]
            }),
        },
    }
}

#[async_trait]
impl ToolInterface for CodeInterpreterTool {
    fn get_tool(&self) -> BionicToolDefinition {
        get_code_interpreter_tool()
    }

    async fn execute(&self, arguments: &str) -> Result<Value, Value> {
        let params: CodeInterpreterParams = serde_json::from_str(arguments)
            .map_err(|e| json!({"error": "Invalid arguments", "details": e.to_string()}))?;

        let supported = ["py", "js", "rs", "bash"];
        if !supported.contains(&params.lang.as_str()) {
            return Err(json!({"error": "Unsupported language"}));
        }
        let job_suffix: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect();
        let job_name = format!("code-{}", job_suffix);
        let cm_name = format!("{}-cm", job_name);

        // Gather files and create a ConfigMap
        let mut data = serde_json::Map::new();
        if let Some(ids) = params.files {
            for id in ids {
                match object_storage::get(self.pool.clone(), id).await {
                    Ok(obj) => {
                        if let (Some(bytes), Some(name)) = (obj.object_data, Some(obj.file_name)) {
                            data.insert(name, general_purpose::STANDARD.encode(bytes).into());
                        }
                    }
                    Err(e) => {
                        tracing::error!("Failed to fetch file {}: {}", id, e);
                    }
                }
            }
        }

        if !data.is_empty() {
            let cm = json!({
                "apiVersion": "v1",
                "kind": "ConfigMap",
                "metadata": { "name": cm_name, "namespace": self.namespace },
                "data": data
            });
            let cm_api: Api<ConfigMap> = Api::namespaced(self.kube.clone(), &self.namespace);
            cm_api
                .patch(
                    &cm_name,
                    &PatchParams::apply(MANAGER).force(),
                    &Patch::Apply(cm),
                )
                .await
                .map_err(
                    |e| json!({"error": "Failed to create ConfigMap", "details": e.to_string()}),
                )?;
        }

        let env = vec![
            json!({"name": "CODE", "value": general_purpose::STANDARD.encode(params.code)}),
            json!({"name": "LANG", "value": params.lang}),
            json!({"name": "ARGS", "value": params.args.unwrap_or_default()}),
        ];

        let volume_mounts = if data.is_empty() {
            vec![json!({"name": "results", "mountPath": "/results"})]
        } else {
            vec![
                json!({"name": "inputs", "mountPath": "/inputs"}),
                json!({"name": "results", "mountPath": "/results"}),
            ]
        };

        let mut volumes = vec![json!({"name": "results", "emptyDir": {}})];
        if !data.is_empty() {
            volumes.push(json!({"name": "inputs", "configMap": {"name": cm_name}}));
        }

        let spec = job::JobSpec {
            name: job_name.clone(),
            image_name: "ghcr.io/bionic-gpt/code-runner:latest".to_string(),
            env,
            command: vec!["/usr/local/bin/run-code".to_string()],
            args: vec![],
            volume_mounts,
            volumes,
        };

        job::create_job(self.kube.clone(), spec, &self.namespace)
            .await
            .map_err(|e| json!({"error": "Failed to create job", "details": e.to_string()}))?;

        let jobs_api: Api<K8sJob> = Api::namespaced(self.kube.clone(), &self.namespace);
        let pods_api: Api<Pod> = Api::namespaced(self.kube.clone(), &self.namespace);

        // Wait for job completion
        loop {
            let job = jobs_api
                .get(&job_name)
                .await
                .map_err(|e| json!({"error": "Failed to get job", "details": e.to_string()}))?;
            if let Some(status) = job.status {
                if status.succeeded.unwrap_or(0) > 0 || status.failed.unwrap_or(0) > 0 {
                    break;
                }
            }
            sleep(Duration::from_secs(1)).await;
        }

        // Get pod logs
        let lp = ListParams::default().labels(&format!("job={}", job_name));
        let pod = pods_api
            .list(&lp)
            .await
            .map_err(|e| json!({"error": "Failed to get pod", "details": e.to_string()}))?
            .items
            .into_iter()
            .next()
            .ok_or_else(|| json!({"error": "Pod not found"}))?;
        let pod_name = pod.metadata.name.clone().unwrap();
        let logs = pods_api
            .logs(&pod_name, &Default::default())
            .await
            .unwrap_or_default();

        let exit_code = pod
            .status
            .as_ref()
            .and_then(|s| s.container_statuses.as_ref())
            .and_then(|cs| cs.first())
            .and_then(|c| c.state.as_ref())
            .and_then(|s| s.terminated.as_ref())
            .map(|t| t.exit_code)
            .unwrap_or_default();

        // Cleanup
        let _ = jobs_api
            .delete(&job_name, &DeleteParams::background())
            .await;
        let _ = pods_api
            .delete(&pod_name, &DeleteParams::background())
            .await;
        if !data.is_empty() {
            let cm_api: Api<ConfigMap> = Api::namespaced(self.kube.clone(), &self.namespace);
            let _ = cm_api.delete(&cm_name, &DeleteParams::background()).await;
        }

        Ok(json!({
            "run": {"stdout": logs, "stderr": "", "code": exit_code},
            "files": []
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_schema() {
        let tool = get_code_interpreter_tool();
        assert_eq!(tool.function.name, "run_code");
    }
}
