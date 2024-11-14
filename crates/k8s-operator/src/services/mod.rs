pub mod bionic;
pub mod chunking_engine;
pub mod cloudflare;
pub mod database;
pub mod deployment;
pub mod embeddings_engine;
pub mod envoy;
pub mod http_mock;
pub mod ingress;
pub mod keycloak;
pub mod keycloak_db;
pub mod llm;
pub mod llm_lite;
pub mod mailhog;
pub mod nginx;
pub mod oauth2_proxy;
pub mod observability;
pub mod pgadmin;
pub mod rag_engine;
pub mod tgi;

const BIONICGPT_IMAGE: &str = "ghcr.io/bionic-gpt/bionicgpt";
const BIONICGPT_RAG_ENGINE_IMAGE: &str = "ghcr.io/bionic-gpt/bionicgpt-rag-engine";
const BIONICGPT_DB_MIGRATIONS_IMAGE: &str = "ghcr.io/bionic-gpt/bionicgpt-db-migrations";

const ENVOYPROXY_IMAGE: &str = "envoyproxy/envoy:v1.28.0";
const KEYCLOAK_IMAGE: &str = "quay.io/keycloak/keycloak:23.0";
const OAUTH2_PROXY_IMAGE: &str = "quay.io/oauth2-proxy/oauth2-proxy:v7.5.1";
const LITE_LLM_IMAGE: &str = "ghcr.io/berriai/litellm:main-v1.10.3";
const TGI_IMAGE: &str = "ghcr.io/huggingface/text-generation-inference:1.2";
const PGADMIN_IMAGE: &str = "dpage/pgadmin4:8";
const CHUNKING_ENGINE_IMAGE: &str =
    "downloads.unstructured.io/unstructured-io/unstructured-api:4ffd8bc";
const EMBEDDINGS_ENGINE_IMAGE: &str = "ghcr.io/bionic-gpt/bionicgpt-embeddings-api:cpu-0.6";
const LLM_API_IMAGE: &str = "ghcr.io/bionic-gpt/llama-3-8b-chat:1.1.1";

// Images used for testing
const HTTP_MOCK: &str = "alexliesenfeld/httpmock:latest";
