--! insert_prompt_flag
INSERT INTO prompt_flags (chat_id, flag_type)
VALUES (:chat_id, :flag_type);
