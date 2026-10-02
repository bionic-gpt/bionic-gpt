--: ModelRequestSummary()
--: ModelRequestDetail()

--! insert
INSERT INTO llm.requests (chat_id, method, uri, body)
VALUES (:chat_id, :method, :uri, encrypt_text(:body));

--! list(before_id?) : ModelRequestSummary
SELECT
    r.id,
    r.chat_id,
    ch.conversation_id,
    m.name AS model_name,
    m.provider_type,
    u.email,
    ch.status,
    r.method,
    r.uri,
    trim(both '"' from to_json(r.created_at)::text) AS created_at
FROM llm.requests r
JOIN llm.chats ch ON ch.id = r.chat_id
JOIN llm.conversations c ON c.id = ch.conversation_id
JOIN iam.users u ON u.id = c.user_id
JOIN model_registry.models m ON m.id = ch.model_id
WHERE c.team_id = :team_id
  AND r.id < COALESCE(:before_id, 9223372036854775807)
ORDER BY r.id DESC
LIMIT :limit;

--! detail : ModelRequestDetail
SELECT
    r.id,
    r.chat_id,
    ch.conversation_id,
    m.name AS model_name,
    m.provider_type,
    u.email,
    ch.status,
    r.method,
    r.uri,
    decrypt_text(r.body) AS body,
    trim(both '"' from to_json(r.created_at)::text) AS created_at
FROM llm.requests r
JOIN llm.chats ch ON ch.id = r.chat_id
JOIN llm.conversations c ON c.id = ch.conversation_id
JOIN iam.users u ON u.id = c.user_id
JOIN model_registry.models m ON m.id = ch.model_id
WHERE c.team_id = :team_id
  AND r.id = :request_id;
