use super::filesystem::FsToolFiles;
use crate::{CredentialSet, SandboxTool, ToolCallContext, ToolExposure};
use async_trait::async_trait;
use bashkit::{Builtin, BuiltinContext, ExecResult, FileSystem, PythonExternalFnHandler};
use serde_json::Value;
use std::sync::Arc;

pub(super) fn python_handler(
    tools: Arc<Vec<Arc<dyn SandboxTool>>>,
    credentials: Arc<CredentialSet>,
    fs: Arc<dyn FileSystem>,
) -> PythonExternalFnHandler {
    Arc::new(move |name, args, kwargs| {
        let tools = Arc::clone(&tools);
        let credentials = Arc::clone(&credentials);
        let files = FsToolFiles(fs.clone());
        Box::pin(async move {
            let Some(tool) = tools.iter().find(|tool| tool.definition().name == name) else {
                return bashkit::ExtFunctionResult::Error(bashkit::MontyException::new(
                    bashkit::ExcType::ValueError,
                    Some(format!("unknown sandbox tool: {name}")),
                ));
            };
            let definition = tool.definition();
            let context = ToolCallContext {
                files: &files,
                credential: definition
                    .credential
                    .as_deref()
                    .and_then(|key| credentials.get(key)),
            };
            match tool.call(monty_arguments(args, kwargs), context).await {
                Ok(value) => bashkit::ExtFunctionResult::Return(json_to_monty(&value)),
                Err(err) => bashkit::ExtFunctionResult::Error(bashkit::MontyException::new(
                    bashkit::ExcType::ValueError,
                    Some(err.to_string()),
                )),
            }
        })
    })
}

pub(super) fn python_names(tools: &[Arc<dyn SandboxTool>]) -> Vec<String> {
    tools
        .iter()
        .filter(|tool| tool.definition().exposure == ToolExposure::Python)
        .map(|tool| tool.definition().name)
        .collect()
}

pub(super) struct CallbackBuiltin {
    pub(super) tool: Arc<dyn SandboxTool>,
    pub(super) credentials: Arc<CredentialSet>,
}

#[async_trait]
impl Builtin for CallbackBuiltin {
    async fn execute(&self, ctx: BuiltinContext<'_>) -> bashkit::Result<ExecResult> {
        let definition = self.tool.definition();
        let files = FsToolFiles(ctx.fs);
        let arguments = serde_json::json!({
            "args": ctx.args,
            "stdin": ctx.stdin,
        });
        let context = ToolCallContext {
            files: &files,
            credential: definition
                .credential
                .as_deref()
                .and_then(|key| self.credentials.get(key)),
        };
        match self.tool.call(arguments, context).await {
            Ok(Value::String(value)) => Ok(ExecResult::ok(format!("{value}\n"))),
            Ok(value) => Ok(ExecResult::ok(format!("{value}\n"))),
            Err(error) => Ok(ExecResult::err(format!("{error}\n"), 1)),
        }
    }

    fn llm_hint(&self) -> Option<&'static str> {
        None
    }
}

fn monty_arguments(
    args: Vec<bashkit::MontyObject>,
    kwargs: Vec<(bashkit::MontyObject, bashkit::MontyObject)>,
) -> Value {
    if kwargs.is_empty() {
        return Value::Array(args.iter().map(monty_to_json).collect());
    }
    let mut values = serde_json::Map::new();
    for (key, value) in kwargs {
        if let bashkit::MontyObject::String(key) = key {
            values.insert(key, monty_to_json(&value));
        }
    }
    Value::Object(values)
}

fn monty_to_json(value: &bashkit::MontyObject) -> Value {
    match value {
        bashkit::MontyObject::None => Value::Null,
        bashkit::MontyObject::Bool(value) => Value::Bool(*value),
        bashkit::MontyObject::Int(value) => Value::Number((*value).into()),
        bashkit::MontyObject::BigInt(value) => Value::String(value.to_string()),
        bashkit::MontyObject::Float(value) => serde_json::Number::from_f64(*value)
            .map(Value::Number)
            .unwrap_or(Value::Null),
        bashkit::MontyObject::String(value) | bashkit::MontyObject::Path(value) => {
            Value::String(value.clone())
        }
        bashkit::MontyObject::Bytes(value) => {
            Value::String(String::from_utf8_lossy(value).to_string())
        }
        bashkit::MontyObject::List(values) | bashkit::MontyObject::Tuple(values) => {
            Value::Array(values.iter().map(monty_to_json).collect())
        }
        bashkit::MontyObject::Dict(values) => Value::Object(
            values
                .into_iter()
                .filter_map(|(key, value)| match key {
                    bashkit::MontyObject::String(key) => Some((key.clone(), monty_to_json(value))),
                    _ => None,
                })
                .collect(),
        ),
        _ => Value::String(format!("{value:?}")),
    }
}

fn json_to_monty(value: &Value) -> bashkit::MontyObject {
    match value {
        Value::Null => bashkit::MontyObject::None,
        Value::Bool(value) => bashkit::MontyObject::Bool(*value),
        Value::Number(value) => value
            .as_i64()
            .map(bashkit::MontyObject::Int)
            .or_else(|| value.as_f64().map(bashkit::MontyObject::Float))
            .unwrap_or(bashkit::MontyObject::None),
        Value::String(value) => bashkit::MontyObject::String(value.clone()),
        Value::Array(values) => {
            bashkit::MontyObject::List(values.iter().map(json_to_monty).collect())
        }
        Value::Object(values) => bashkit::MontyObject::Dict(
            values
                .iter()
                .map(|(key, value)| {
                    (
                        bashkit::MontyObject::String(key.clone()),
                        json_to_monty(value),
                    )
                })
                .collect(),
        ),
    }
}
