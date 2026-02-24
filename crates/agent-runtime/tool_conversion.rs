use openai_api::{BionicToolDefinition, ChatCompletionFunctionDefinition, ToolCall};
use tool_runtime::ToolDefinition;

pub fn to_openai_tool_definition(def: ToolDefinition) -> BionicToolDefinition {
    BionicToolDefinition {
        r#type: def.r#type,
        function: ChatCompletionFunctionDefinition {
            name: def.function.name,
            description: def.function.description,
            parameters: def.function.parameters,
        },
    }
}

pub fn to_openai_tool_definitions(defs: Vec<ToolDefinition>) -> Vec<BionicToolDefinition> {
    defs.into_iter().map(to_openai_tool_definition).collect()
}

pub fn to_tool_runtime_tool_call(call: ToolCall) -> tool_runtime::ToolCall {
    tool_runtime::ToolCall {
        id: call.id,
        index: call.index,
        r#type: call.r#type,
        function: tool_runtime::ToolCallFunction {
            name: call.function.name,
            arguments: call.function.arguments,
        },
    }
}

pub fn to_tool_runtime_tool_calls(calls: Vec<ToolCall>) -> Vec<tool_runtime::ToolCall> {
    calls.into_iter().map(to_tool_runtime_tool_call).collect()
}
