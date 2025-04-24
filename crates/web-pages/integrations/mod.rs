pub mod delete;
pub mod form;
pub mod index;
pub mod integration_table;
pub mod integration_type;
pub mod status_type;
use db::IntegrationType;

fn integration_type(integration_type: IntegrationType) -> String {
    match integration_type {
        IntegrationType::MCP_Server => "MCP Server".to_string(),
        IntegrationType::BuiltIn => "Built In".to_string(),
    }
}
