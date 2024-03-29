use anyhow::{bail, Result};
use kube::{
    api::{ApiResource, DynamicObject, GroupVersionKind, Patch, PatchParams},
    discovery::{ApiCapabilities, Scope},
    Api, Client, Discovery, ResourceExt,
};
use tracing::{info, trace, warn};

pub async fn apply(client: &Client, yaml: &str, namespace: Option<&str>) -> Result<()> {
    let ssapply = PatchParams::apply("kubectl-light").force();
    let discovery = Discovery::new(client.clone()).run().await?;
    for doc in multidoc_deserialize(yaml)? {
        let obj: DynamicObject = serde_yaml::from_value(doc)?;
        let namespace = obj.metadata.namespace.as_deref().or(namespace);
        let gvk = if let Some(tm) = &obj.types {
            GroupVersionKind::try_from(tm)?
        } else {
            bail!("cannot apply object without valid TypeMeta {:?}", obj);
        };
        let name = obj.name_any();
        if let Some((ar, caps)) = discovery.resolve_gvk(&gvk) {
            let api = dynamic_api(ar, caps, client.clone(), namespace, false);
            trace!("Applying {}: \n{}", gvk.kind, serde_yaml::to_string(&obj)?);
            let data: serde_json::Value = serde_json::to_value(&obj)?;
            let _r = api.patch(&name, &ssapply, &Patch::Apply(data)).await?;
            info!("applied {} {}", gvk.kind, name);
        } else {
            warn!("Cannot apply document for unknown {:?}", gvk);
        }
    }
    Ok(())
}

pub fn multidoc_deserialize(data: &str) -> Result<Vec<serde_yaml::Value>> {
    use serde::Deserialize;
    let mut docs = vec![];
    for de in serde_yaml::Deserializer::from_str(data) {
        docs.push(serde_yaml::Value::deserialize(de)?);
    }
    Ok(docs)
}

fn dynamic_api(
    ar: ApiResource,
    caps: ApiCapabilities,
    client: Client,
    ns: Option<&str>,
    all: bool,
) -> Api<DynamicObject> {
    if caps.scope == Scope::Cluster || all {
        Api::all_with(client, &ar)
    } else if let Some(namespace) = ns {
        Api::namespaced_with(client, namespace, &ar)
    } else {
        Api::default_namespaced_with(client, &ar)
    }
}
