mod execution;
mod filesystem;
mod network;

use crate::{PythonCallArguments, PythonFunction, RunRequest, RunResult, Sandbox, SandboxError};
use async_trait::async_trait;
use bashkit::{
    Bash, ExcType, ExecutionLimits, ExtFunctionResult, FileSystem, MontyException, MontyObject,
    NetworkAllowlist, PythonExternalFnHandler, PythonLimits, SqliteLimits,
};
use execution::{command_timeout, MAX_COMMANDS, MAX_STDERR_BYTES, MAX_STDOUT_BYTES};
use filesystem::FilesystemAdapter;
use network::NetworkAdapter;
use serde_json::{Map, Number, Value};
use std::sync::Arc;

const HOME_DIR: &str = "/home/user";

#[derive(Default)]
pub struct BashkitSandbox;

#[async_trait]
impl Sandbox for BashkitSandbox {
    async fn run(&self, request: RunRequest) -> Result<RunResult, SandboxError> {
        let fs: Arc<dyn FileSystem> = Arc::new(FilesystemAdapter(request.filesystem));
        let timeout = command_timeout(&request.command);
        let functions = Arc::new(request.python_functions);
        let function_names = functions
            .iter()
            .map(|function| function.name().to_string())
            .collect::<Vec<_>>();
        let mut bash = Bash::builder()
            .fs(fs.clone())
            .username("user")
            .hostname("sandbox")
            .cwd(HOME_DIR)
            .env("BASHKIT_ALLOW_INPROCESS_PYTHON", "1")
            .env("BASHKIT_ALLOW_INPROCESS_SQLITE", "1")
            .limits(
                ExecutionLimits::new()
                    .timeout(timeout)
                    .max_commands(MAX_COMMANDS)
                    .max_stdout_bytes(MAX_STDOUT_BYTES)
                    .max_stderr_bytes(MAX_STDERR_BYTES),
            )
            .python_with_external_handler(
                PythonLimits::default().max_duration(timeout),
                function_names,
                python_handler(functions),
            )
            .sqlite_with_limits(SqliteLimits::default().max_duration(timeout))
            .network(NetworkAllowlist::allow_all())
            .http_transport(Arc::new(NetworkAdapter(request.network)))
            .build();

        let execution = execution::execute(request.command, &mut bash, fs.as_ref()).await?;
        Ok(RunResult { execution })
    }
}

fn python_handler(functions: Arc<Vec<Arc<dyn PythonFunction>>>) -> PythonExternalFnHandler {
    Arc::new(move |name, args, kwargs| {
        let functions = Arc::clone(&functions);
        Box::pin(async move {
            let Some(function) = functions.iter().find(|function| function.name() == name) else {
                return ExtFunctionResult::Error(MontyException::new(
                    ExcType::ValueError,
                    Some(format!("unknown Python function: {name}")),
                ));
            };
            let positional = match args.iter().map(monty_to_json).collect() {
                Ok(values) => values,
                Err(error) => return python_value_error(error),
            };
            let mut keyword = Map::new();
            for (key, value) in &kwargs {
                let MontyObject::String(key) = key else {
                    return python_value_error("keyword argument names must be strings".into());
                };
                let value = match monty_to_json(value) {
                    Ok(value) => value,
                    Err(error) => return python_value_error(error),
                };
                keyword.insert(key.clone(), value);
            }
            match function
                .call(PythonCallArguments {
                    positional,
                    keyword,
                })
                .await
            {
                Ok(value) => ExtFunctionResult::Return(json_to_monty(&value)),
                Err(error) => python_value_error(error.to_string()),
            }
        })
    })
}

fn python_value_error(message: String) -> ExtFunctionResult {
    ExtFunctionResult::Error(MontyException::new(ExcType::ValueError, Some(message)))
}

fn monty_to_json(value: &MontyObject) -> Result<Value, String> {
    match value {
        MontyObject::None => Ok(Value::Null),
        MontyObject::Bool(value) => Ok(Value::Bool(*value)),
        MontyObject::Int(value) => Ok(Value::Number((*value).into())),
        MontyObject::BigInt(value) => Ok(Value::String(value.to_string())),
        MontyObject::Float(value) => Number::from_f64(*value)
            .map(Value::Number)
            .ok_or_else(|| "non-finite floats are not supported".to_string()),
        MontyObject::String(value) | MontyObject::Path(value) => Ok(Value::String(value.clone())),
        MontyObject::Bytes(value) => Ok(Value::String(String::from_utf8_lossy(value).to_string())),
        MontyObject::List(values) | MontyObject::Tuple(values) => values
            .iter()
            .map(monty_to_json)
            .collect::<Result<Vec<_>, _>>()
            .map(Value::Array),
        MontyObject::Dict(values) => {
            let mut object = Map::new();
            for (key, value) in values {
                let MontyObject::String(key) = key else {
                    return Err("dictionary keys must be strings".into());
                };
                object.insert(key.clone(), monty_to_json(value)?);
            }
            Ok(Value::Object(object))
        }
        _ => Err(format!(
            "unsupported Python value type: {}",
            value.type_name()
        )),
    }
}

fn json_to_monty(value: &Value) -> MontyObject {
    match value {
        Value::Null => MontyObject::None,
        Value::Bool(value) => MontyObject::Bool(*value),
        Value::Number(value) => value
            .as_i64()
            .map(MontyObject::Int)
            .or_else(|| value.as_f64().map(MontyObject::Float))
            .unwrap_or_else(|| MontyObject::String(value.to_string())),
        Value::String(value) => MontyObject::String(value.clone()),
        Value::Array(values) => MontyObject::List(values.iter().map(json_to_monty).collect()),
        Value::Object(values) => MontyObject::Dict(
            values
                .iter()
                .map(|(key, value)| (MontyObject::String(key.clone()), json_to_monty(value)))
                .collect(),
        ),
    }
}
