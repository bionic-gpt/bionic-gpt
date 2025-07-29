use crate::error::Error;
use k8s_openapi::api::batch::v1::Job;
use kube::api::{Patch, PatchParams};
use kube::{Api, Client};
use serde_json::{json, Value};

pub struct JobSpec {
    pub name: String,
    pub image_name: String,
    pub env: Vec<Value>,
    pub command: Vec<String>,
    pub args: Vec<String>,
    pub volume_mounts: Vec<Value>,
    pub volumes: Vec<Value>,
}

pub async fn create_job(client: Client, job_spec: JobSpec, namespace: &str) -> Result<(), Error> {
    let labels = json!({ "job": job_spec.name });

    let job = json!({
        "apiVersion": "batch/v1",
        "kind": "Job",
        "metadata": {
            "name": job_spec.name,
            "namespace": namespace,
            "labels": labels,
        },
        "spec": {
            "backoffLimit": 0,
            "template": {
                "metadata": { "labels": labels },
                "spec": {
                    "restartPolicy": "Never",
                    "containers": [{
                        "name": "runner",
                        "image": job_spec.image_name,
                        "imagePullPolicy": "IfNotPresent",
                        "env": job_spec.env,
                        "volumeMounts": job_spec.volume_mounts,
                        "command": job_spec.command,
                        "args": job_spec.args
                    }],
                    "volumes": job_spec.volumes
                }
            }
        }
    });

    let api: Api<Job> = Api::namespaced(client, namespace);
    api.patch(
        &job_spec.name,
        &PatchParams::apply(crate::MANAGER).force(),
        &Patch::Apply(job),
    )
    .await?;

    Ok(())
}
