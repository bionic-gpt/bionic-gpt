use rig::wasm_compat::WasmBoxedFuture;
use std::fmt;

/// Runtime-owned dynamic tool contract used by Bionic's dispatch layer.
///
/// Rig 0.42 keeps provider-facing tool contracts in `rig-core` and no longer
/// exposes the previous string-based dynamic trait. Bionic's tools still use
/// this contract because dispatch supplies runtime context outside Rig.
pub trait ToolDyn: Send + Sync {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn parameters(&self) -> serde_json::Value;
    fn call<'a>(&'a self, args: String) -> WasmBoxedFuture<'a, Result<String, ToolError>>;
}

#[derive(Debug)]
pub enum ToolError {
    JsonError(serde_json::Error),
    ToolCallError(Box<dyn std::error::Error + Send + Sync>),
}

impl fmt::Display for ToolError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::JsonError(error) => write!(formatter, "{error}"),
            Self::ToolCallError(error) => write!(formatter, "{error}"),
        }
    }
}

impl std::error::Error for ToolError {}
