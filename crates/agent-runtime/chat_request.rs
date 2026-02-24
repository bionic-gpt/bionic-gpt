use crate::context_builder;
use crate::errors::CustomError;
use crate::jwt::Jwt;
use crate::moderation::{moderate_chat, strip_tool_data, ModerationVerdict};
use crate::user_config::UserConfig;
use db::{queries, ChatRole, ChatStatus, Pool};
use rig::completion::{CompletionRequest, Message as RigMessage};
use rig::OneOrMany;
use tool_runtime::{
    get_chat_tools_user_selected_with_system_openapi, get_tools, ToolDefinition, ToolScope,
};

pub(crate) struct RigChatRequest {
    pub(crate) model_name: String,
    pub(crate) base_url: String,
    pub(crate) api_key: Option<String>,
    pub(crate) completion: CompletionRequest,
    pub(crate) model_id: i32,
    pub(crate) user_id: i32,
}

/// Builds the model request payload and marks the chat as in-progress.
pub(crate) async fn create_request(
    pool: &Pool,
    current_user: &Jwt,
    chat_id: i32,
    user_config: &UserConfig,
) -> Result<RigChatRequest, CustomError> {
    let mut db_client = pool.get().await?;
    let transaction = db_client.transaction().await?;
    db::authz::set_row_level_security_user_id(&transaction, current_user.sub.to_string()).await?;

    let model = queries::models::model_host_by_chat_id()
        .bind(&transaction, &chat_id)
        .one()
        .await?;

    let capabilities = queries::capabilities::get_model_capabilities()
        .bind(&transaction, &model.id)
        .all()
        .await?;

    let chat = queries::chats::chat()
        .bind(&transaction, &chat_id)
        .one()
        .await?;

    let conversation = queries::conversations::get_conversation_from_chat()
        .bind(&transaction, &chat_id)
        .one()
        .await?;

    let attachment_count = queries::conversations::count_attachments()
        .bind(&transaction, &conversation.id)
        .one()
        .await?;

    let prompt = queries::prompts::prompt()
        .bind(&transaction, &chat.prompt_id, &conversation.team_id)
        .one()
        .await?;

    let chat_history = queries::chats::chat_history()
        .bind(
            &transaction,
            &conversation.id,
            &(prompt.max_history_items as i64),
        )
        .all()
        .await?;

    let chat_history = context_builder::convert_chat_to_messages(chat_history);

    let messages = context_builder::execute_prompt(
        &transaction,
        prompt.clone(),
        Some(conversation.id),
        chat_history,
    )
    .await?;

    queries::chats::set_chat_status()
        .bind(&transaction, &ChatStatus::InProgress, &chat_id)
        .await?;

    let tools = if capabilities
        .iter()
        .any(|c| c.capability == db::ModelCapability::tool_use)
    {
        let mut all_tools = get_chat_tools_user_selected_with_system_openapi(
            pool,
            user_config.enabled_tools.as_ref(),
        )
        .await;

        if attachment_count > 0 {
            all_tools.extend(get_tools(ToolScope::DocumentIntelligence));
        }

        if let Ok(integration_tools) =
            context_builder::get_prompt_integration_tools(&transaction, prompt.id).await
        {
            all_tools.extend(integration_tools);
        }

        Some(dedupe_tools_by_name(all_tools))
            .filter(|tool_defs: &Vec<ToolDefinition>| !tool_defs.is_empty())
    } else {
        None
    };

    if capabilities
        .iter()
        .any(|c| c.capability == db::ModelCapability::Guarded)
    {
        let guard_model = queries::models::models()
            .bind(&transaction, &db::ModelType::Guard)
            .one()
            .await?;

        let sanitized = strip_tool_data(&messages);
        match moderate_chat(
            &guard_model.base_url,
            guard_model.api_key.as_deref(),
            &guard_model.name,
            sanitized,
        )
        .await
        {
            Ok(ModerationVerdict::Safe) => {}
            Ok(ModerationVerdict::Unsafe(code)) => {
                queries::chats::new_chat()
                    .bind(
                        &transaction,
                        &conversation.id,
                        &chat.prompt_id,
                        &None::<String>,
                        &None::<String>,
                        &"Your question violated our guidelines",
                        &ChatRole::Assistant,
                        &ChatStatus::Error,
                    )
                    .one()
                    .await?;
                queries::prompt_flags::insert_prompt_flag()
                    .bind(&transaction, &chat_id, &code)
                    .await?;
                transaction.commit().await?;
                return Err(CustomError::FaultySetup("Moderation failed".into()));
            }
            Err(status) => {
                transaction.commit().await?;
                return Err(CustomError::FaultySetup(format!(
                    "Moderation failed: {status}"
                )));
            }
        }
    }

    transaction.commit().await?;

    let completion = CompletionRequest {
        preamble: None,
        chat_history: OneOrMany::many(messages)
            .unwrap_or_else(|_| OneOrMany::one(RigMessage::user(""))),
        documents: vec![],
        tools: tools.unwrap_or_default(),
        temperature: prompt.temperature.map(|t| t as f64),
        max_tokens: prompt.max_completion_tokens.map(|t| t as u64),
        tool_choice: None,
        additional_params: None,
    };

    Ok(RigChatRequest {
        model_name: model.name,
        base_url: model.base_url,
        api_key: model.api_key,
        completion,
        model_id: model.id,
        user_id: conversation.user_id,
    })
}

fn dedupe_tools_by_name(tools: Vec<ToolDefinition>) -> Vec<ToolDefinition> {
    let mut deduped = Vec::new();
    let mut names = std::collections::HashSet::new();
    for tool in tools.into_iter().rev() {
        if names.insert(tool.name.clone()) {
            deduped.push(tool);
        }
    }
    deduped.reverse();
    deduped
}
