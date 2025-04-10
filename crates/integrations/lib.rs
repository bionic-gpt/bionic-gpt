//! Integrations crate
//!
//! This crate provides integration with external services and tools.

pub mod function_executor;
pub mod function_tools;
pub mod tool_call_handler;
pub mod weather;

// Re-export key types for convenience
pub use function_executor::execute_tool_call;
pub use function_tools::get_tools;
pub use tool_call_handler::handle_tool_call;
pub use weather::{execute_weather_function, get_weather_tool};
