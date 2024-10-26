## The Database

We use 2 main tools to manage the database

- `dbmate` For schema migrations
- `cornucopia` for generating rust code from `sql` files.

## Database Schema: postgres

### Table: model_name
| Column Name | Data Type | Nullable | Default Value |
|-------------|-----------|----------|---------------|
| column_name | data_type | is_nullable | column_default |
| name | character varying | YES | NULL |
| (1 row) |  |  | NULL |

### Table: schema_migrations
| Column Name | Data Type | Nullable | Default Value |
|-------------|-----------|----------|---------------|
| column_name | data_type | is_nullable | column_default |
| version | character varying | NO | NULL |
| (1 row) |  |  | NULL |

### Table: objects
| Column Name | Data Type | Nullable | Default Value |
|-------------|-----------|----------|---------------|
| column_name | data_type | is_nullable | column_default |
| id | integer | NO | NULL |
| team_id | integer | NO | NULL |
| object_name | character varying | NO | NULL |
| object_data | bytea | YES | NULL |
| mime_type | character varying | NO | NULL |
| file_name | character varying | NO | NULL |
| file_size | bigint | NO | NULL |
| file_hash | character varying | NO | NULL |
| created_by | integer | NO | NULL |
| created_at | timestamp with time zone | NO | now() |
| updated_at | timestamp with time zone | NO | now() |
| (11 rows) |  |  | NULL |

### Table: roles_permissions

_Maps roles to permissions. i.e. a role can have multiple permissions._

| Column Name | Data Type | Nullable | Default Value |
|-------------|-----------|----------|---------------|
| column_name | data_type | is_nullable | column_default |
| role | USER-DEFINED | NO | NULL |
| permission | USER-DEFINED | NO | NULL |
| (2 rows) |  |  | NULL |

### Table: users
| Column Name | Data Type | Nullable | Default Value |
|-------------|-----------|----------|---------------|
| column_name | data_type | is_nullable | column_default |
| id | integer | NO | NULL |
| openid_sub | character varying | NO | NULL |
| email | character varying | NO | NULL |
| first_name | character varying | YES | NULL |
| last_name | character varying | YES | NULL |
| system_admin | boolean | NO | false |
| created_at | timestamp with time zone | NO | now() |
| updated_at | timestamp with time zone | NO | now() |
| (8 rows) |  |  | NULL |

### Table: invitations

_Invitations are generated so users can join teams (teams)_

| Column Name | Data Type | Nullable | Default Value |
|-------------|-----------|----------|---------------|
| column_name | data_type | is_nullable | column_default |
| id | integer | NO | NULL |
| team_id | integer | NO | NULL |
| email | character varying | NO | NULL |
| first_name | character varying | NO | NULL |
| last_name | character varying | NO | NULL |
| roles | ARRAY | NO | NULL |
| invitation_selector | character varying | NO | NULL |
| invitation_verifier_hash | character varying | NO | NULL |
| created_at | timestamp with time zone | NO | now() |
| (9 rows) |  |  | NULL |

### Table: models
| Column Name | Data Type | Nullable | Default Value |
|-------------|-----------|----------|---------------|
| column_name | data_type | is_nullable | column_default |
| id | integer | NO | NULL |
| model_type | USER-DEFINED | NO | NULL |
| name | character varying | NO | NULL |
| base_url | character varying | NO | NULL |
| api_key | character varying | YES | NULL |
| context_size | integer | NO | NULL |
| created_at | timestamp with time zone | NO | now() |
| updated_at | timestamp with time zone | NO | now() |
| tpm_limit | integer | NO | 10000 |
| rpm_limit | integer | NO | 10000 |
| (10 rows) |  |  | NULL |

### Table: team_users

_A User can belong to multiple teams (teams)._

| Column Name | Data Type | Nullable | Default Value |
|-------------|-----------|----------|---------------|
| column_name | data_type | is_nullable | column_default |
| user_id | integer | NO | NULL |
| team_id | integer | NO | NULL |
| roles | ARRAY | NO | NULL |
| (3 rows) |  |  | NULL |

### Table: teams

_An team is created for everyone that signs up. It could also have been called teams._

| Column Name | Data Type | Nullable | Default Value |
|-------------|-----------|----------|---------------|
| column_name | data_type | is_nullable | column_default |
| id | integer | NO | NULL |
| name | character varying | YES | NULL |
| created_by_user_id | integer | NO | NULL |
| (3 rows) |  |  | NULL |

### Table: prompts
| Column Name | Data Type | Nullable | Default Value |
|-------------|-----------|----------|---------------|
| column_name | data_type | is_nullable | column_default |
| id | integer | NO | NULL |
| team_id | integer | NO | NULL |
| model_id | integer | NO | NULL |
| visibility | USER-DEFINED | NO | NULL |
| name | character varying | NO | NULL |
| max_history_items | integer | NO | NULL |
| max_chunks | integer | NO | NULL |
| max_tokens | integer | NO | NULL |
| trim_ratio | integer | NO | NULL |
| temperature | real | YES | NULL |
| system_prompt | character varying | YES | NULL |
| created_at | timestamp with time zone | NO | now() |
| updated_at | timestamp with time zone | NO | now() |
| created_by | integer | NO | NULL |
| prompt_type | USER-DEFINED | NO | 'Assistant'::prompt_type |
| description | character varying | NO | 'Please add a description'::character varying |
| disclaimer | character varying | NO | 'LLMs can make mistakes. Check important info.'::character varying |
| example1 | character varying | YES | NULL |
| example2 | character varying | YES | NULL |
| example3 | character varying | YES | NULL |
| example4 | character varying | YES | NULL |
| category_id | integer | NO | NULL |
| image_icon_object_id | integer | YES | NULL |
| (23 rows) |  |  | NULL |

### Table: documents
| Column Name | Data Type | Nullable | Default Value |
|-------------|-----------|----------|---------------|
| column_name | data_type | is_nullable | column_default |
| id | integer | NO | NULL |
| dataset_id | integer | NO | NULL |
| file_name | character varying | NO | NULL |
| file_type | character varying | YES | NULL |
| failure_reason | character varying | YES | NULL |
| content | bytea | NO | NULL |
| content_size | integer | NO | NULL |
| created_at | timestamp with time zone | NO | now() |
| updated_at | timestamp with time zone | NO | now() |
| (9 rows) |  |  | NULL |

### Table: chunks
| Column Name | Data Type | Nullable | Default Value |
|-------------|-----------|----------|---------------|
| column_name | data_type | is_nullable | column_default |
| id | integer | NO | NULL |
| document_id | integer | NO | NULL |
| text | character varying | NO | NULL |
| page_number | integer | NO | NULL |
| embeddings | USER-DEFINED | YES | NULL |
| processed | boolean | NO | false |
| created_at | timestamp with time zone | NO | now() |
| updated_at | timestamp with time zone | NO | now() |
| (8 rows) |  |  | NULL |

### Table: datasets
| Column Name | Data Type | Nullable | Default Value |
|-------------|-----------|----------|---------------|
| column_name | data_type | is_nullable | column_default |
| id | integer | NO | NULL |
| team_id | integer | NO | NULL |
| embeddings_model_id | integer | NO | NULL |
| visibility | USER-DEFINED | NO | NULL |
| name | character varying | NO | NULL |
| chunking_strategy | USER-DEFINED | NO | NULL |
| combine_under_n_chars | integer | NO | NULL |
| new_after_n_chars | integer | NO | NULL |
| multipage_sections | boolean | NO | NULL |
| created_at | timestamp with time zone | NO | now() |
| updated_at | timestamp with time zone | NO | now() |
| created_by | integer | NO | NULL |
| (12 rows) |  |  | NULL |

### Table: prompt_dataset
| Column Name | Data Type | Nullable | Default Value |
|-------------|-----------|----------|---------------|
| column_name | data_type | is_nullable | column_default |
| prompt_id | integer | NO | NULL |
| dataset_id | integer | NO | NULL |
| (2 rows) |  |  | NULL |

### Table: conversations

_Collect together the users chats a bit like a history_

| Column Name | Data Type | Nullable | Default Value |
|-------------|-----------|----------|---------------|
| column_name | data_type | is_nullable | column_default |
| id | bigint | NO | NULL |
| user_id | integer | NO | NULL |
| team_id | integer | NO | NULL |
| created_at | timestamp with time zone | NO | now() |
| prompt_id | integer | YES | NULL |
| (5 rows) |  |  | NULL |

### Table: api_keys
| Column Name | Data Type | Nullable | Default Value |
|-------------|-----------|----------|---------------|
| column_name | data_type | is_nullable | column_default |
| id | integer | NO | NULL |
| prompt_id | integer | NO | NULL |
| user_id | integer | NO | NULL |
| team_id | integer | NO | NULL |
| name | character varying | NO | NULL |
| api_key | character varying | NO | NULL |
| created_at | timestamp with time zone | NO | now() |
| (7 rows) |  |  | NULL |

### Table: document_pipelines
| Column Name | Data Type | Nullable | Default Value |
|-------------|-----------|----------|---------------|
| column_name | data_type | is_nullable | column_default |
| id | integer | NO | NULL |
| dataset_id | integer | NO | NULL |
| user_id | integer | NO | NULL |
| team_id | integer | NO | NULL |
| name | character varying | NO | NULL |
| api_key | character varying | NO | NULL |
| created_at | timestamp with time zone | NO | now() |
| updated_at | timestamp with time zone | NO | now() |
| (8 rows) |  |  | NULL |

### Table: chats

_Questions from the user and the response from the LLM_

| Column Name | Data Type | Nullable | Default Value |
|-------------|-----------|----------|---------------|
| column_name | data_type | is_nullable | column_default |
| id | integer | NO | NULL |
| conversation_id | bigint | NO | NULL |
| status | USER-DEFINED | NO | 'Pending'::chat_status |
| user_request | character varying | NO | NULL |
| prompt | character varying | NO | NULL |
| prompt_id | integer | NO | NULL |
| response | character varying | YES | NULL |
| created_at | timestamp with time zone | NO | now() |
| updated_at | timestamp with time zone | NO | now() |
| tokens_sent | integer | NO | 0 |
| tokens_received | integer | NO | 0 |
| time_taken_ms | integer | NO | 0 |
| request_embeddings | USER-DEFINED | YES | NULL |
| response_embeddings | USER-DEFINED | YES | NULL |
| (14 rows) |  |  | NULL |

### Table: audit_trail_api_generation

_Capture API Text Generation_

| Column Name | Data Type | Nullable | Default Value |
|-------------|-----------|----------|---------------|
| column_name | data_type | is_nullable | column_default |
| id | integer | NO | NULL |
| audit_id | integer | NO | NULL |
| api_key_id | integer | NO | NULL |
| tokens_sent | integer | NO | NULL |
| tokens_received | integer | NO | NULL |
| time_taken | integer | NO | NULL |
| (6 rows) |  |  | NULL |

### Table: audit_trail

_Log all accesses to the system_

| Column Name | Data Type | Nullable | Default Value |
|-------------|-----------|----------|---------------|
| column_name | data_type | is_nullable | column_default |
| id | integer | NO | NULL |
| user_id | integer | NO | NULL |
| team_id | integer | YES | NULL |
| access_type | USER-DEFINED | NO | NULL |
| action | USER-DEFINED | NO | NULL |
| created_at | timestamp with time zone | NO | now() |
| (6 rows) |  |  | NULL |

### Table: audit_trail_text_generation

_For text generation we capture extra information_

| Column Name | Data Type | Nullable | Default Value |
|-------------|-----------|----------|---------------|
| column_name | data_type | is_nullable | column_default |
| id | integer | NO | NULL |
| audit_id | integer | NO | NULL |
| chat_id | integer | NO | NULL |
| tokens_sent | integer | NO | NULL |
| tokens_received | integer | NO | NULL |
| time_taken | integer | NO | NULL |
| (6 rows) |  |  | NULL |

### Table: chunks_chats

_For each chat, track the chunks used as part of the prompt._

| Column Name | Data Type | Nullable | Default Value |
|-------------|-----------|----------|---------------|
| column_name | data_type | is_nullable | column_default |
| chunk_id | integer | NO | NULL |
| chat_id | integer | NO | NULL |
| (2 rows) |  |  | NULL |

### Table: barricade_users
| Column Name | Data Type | Nullable | Default Value |
|-------------|-----------|----------|---------------|
| column_name | data_type | is_nullable | column_default |
| id | integer | NO | nextval('barricade_users_id_seq'::regclass) |
| email | character varying | NO | NULL |
| hashed_password | character varying | NO | NULL |
| reset_password_selector | character varying | YES | NULL |
| reset_password_validator_hash | character varying | YES | NULL |
| created_at | timestamp without time zone | NO | now() |
| updated_at | timestamp without time zone | NO | now() |
| (7 rows) |  |  | NULL |

### Table: api_chats

_Capture API Text Generation_

| Column Name | Data Type | Nullable | Default Value |
|-------------|-----------|----------|---------------|
| column_name | data_type | is_nullable | column_default |
| id | integer | NO | NULL |
| api_key_id | integer | NO | NULL |
| prompt | character varying | NO | NULL |
| response | character varying | YES | NULL |
| status | USER-DEFINED | NO | 'Pending'::chat_status |
| tokens_sent | integer | NO | NULL |
| tokens_received | integer | NO | 0 |
| time_taken_ms | integer | NO | 0 |
| created_at | timestamp with time zone | NO | now() |
| updated_at | timestamp with time zone | NO | now() |
| (10 rows) |  |  | NULL |

### Table: sessions
| Column Name | Data Type | Nullable | Default Value |
|-------------|-----------|----------|---------------|
| column_name | data_type | is_nullable | column_default |
| id | integer | NO | nextval('sessions_id_seq'::regclass) |
| session_verifier | character varying | NO | NULL |
| user_id | integer | NO | NULL |
| otp_code_encrypted | character varying | NO | NULL |
| otp_code_attempts | integer | NO | 0 |
| otp_code_confirmed | boolean | NO | false |
| otp_code_sent | boolean | NO | false |
| created_at | timestamp without time zone | NO | now() |
| (8 rows) |  |  | NULL |

### Table: rate_limits
| Column Name | Data Type | Nullable | Default Value |
|-------------|-----------|----------|---------------|
| column_name | data_type | is_nullable | column_default |
| id | integer | NO | NULL |
| api_key_id | integer | YES | NULL |
| tpm_limit | integer | NO | NULL |
| rpm_limit | integer | NO | NULL |
| created_at | timestamp with time zone | NO | now() |
| (5 rows) |  |  | NULL |

### Table: categories
| Column Name | Data Type | Nullable | Default Value |
|-------------|-----------|----------|---------------|
| column_name | data_type | is_nullable | column_default |
| id | integer | NO | NULL |
| name | character varying | NO | NULL |
| description | text | YES | NULL |
| (3 rows) |  |  | NULL |

