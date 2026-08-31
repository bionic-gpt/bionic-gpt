use crate::context_builder;
use crate::errors::CustomError;
use crate::jwt::Jwt;
use crate::moderation::{moderate_chat, strip_tool_data, ModerationVerdict};
use crate::user_config::UserConfig;
use db::{queries, ChatRole, ChatStatus, Pool};
use rig::completion::{CompletionRequest, Message as RigMessage};
use rig::OneOrMany;
use tool_runtime::{get_chat_tool_definitions, ToolDefinition};

pub(crate) struct RigChatRequest {
    pub(crate) model_name: String,
    pub(crate) provider_type: db::ModelProvider,
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
    _user_config: &UserConfig,
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

    let prompt = queries::models::all_models()
        .bind(&transaction)
        .all()
        .await?
        .into_iter()
        .find(|model| model.id == chat.model_id)
        .ok_or_else(|| CustomError::FaultySetup("Model configuration not found".into()))?;

    let chat_history = queries::chats::chat_history()
        .bind(
            &transaction,
            &conversation.id,
            &(prompt.max_history_items as i64),
        )
        .all()
        .await?;

    let chat_history = context_builder::convert_chat_to_messages(chat_history);

    let supports_tool_use = capabilities
        .iter()
        .any(|c| c.capability == db::ModelCapability::tool_use);

    let integration_context = if supports_tool_use {
        match tool_runtime::builtin_tools::monty::available_function_catalogue_prompt_section_for_conversation(
            pool,
            &current_user.sub,
            conversation.id,
        )
        .await
        {
            Ok(context) => context,
            Err(err) => {
                tracing::warn!("Failed to build integration prompt summary: {}", err);
                None
            }
        }
    } else {
        None
    };

    let messages = context_builder::execute_prompt(
        &transaction,
        prompt.clone(),
        Some(conversation.id),
        supports_tool_use,
        integration_context,
        chat_history,
    )
    .await?;

    queries::chats::set_chat_status()
        .bind(&transaction, &ChatStatus::InProgress, &chat_id)
        .await?;

    let tools = if supports_tool_use {
        let tools = get_chat_tool_definitions();
        tracing::debug!(
            "Sending {} tool definitions to model {}: {:?}",
            tools.len(),
            model.name,
            tools
                .iter()
                .map(|tool| tool.name.as_str())
                .collect::<Vec<_>>()
        );

        Some(tools).filter(|tool_defs: &Vec<ToolDefinition>| !tool_defs.is_empty())
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
                        &chat.model_id,
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
        model: None,
        preamble: None,
        chat_history: OneOrMany::many(messages)
            .unwrap_or_else(|_| OneOrMany::one(RigMessage::user(""))),
        documents: vec![],
        tools: tools.unwrap_or_default(),
        temperature: prompt.temperature.map(|t| t as f64),
        max_tokens: prompt.max_completion_tokens.map(|t| t as u64),
        tool_choice: None,
        additional_params: None,
        output_schema: None,
    };

    Ok(RigChatRequest {
        model_name: model.name,
        provider_type: model.provider_type,
        base_url: model.base_url,
        api_key: model.api_key,
        completion,
        model_id: model.id,
        user_id: conversation.user_id,
    })
}
