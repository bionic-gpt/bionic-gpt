use crate::{BionicOpenAPI, TokenProvider};
use async_trait::async_trait;
use reqwest::{redirect::Policy, Client, Url};
use sandbox::{HttpHeader, HttpMethod, HttpRequest, HttpResponse, NetworkError, SandboxNetwork};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::net::{IpAddr, SocketAddr};
use std::path::Path;
use std::sync::Arc;

const CONNECTOR_SUFFIX: &str = ".connectors.invalid";
const DATASET_SEARCH_HOST: &str = "datasets.internal.invalid";
const SCHEDULED_TASKS_HOST: &str = "scheduled-tasks.internal.invalid";

struct ConnectorRoute {
    slug: String,
    name: String,
    base_url: String,
    document: Value,
    auth: ConnectorAuth,
    token_provider: Option<Arc<dyn TokenProvider>>,
}

enum ConnectorAuth {
    Header(String),
    Query(String),
}

impl ConnectorAuth {
    fn for_openapi(openapi: &BionicOpenAPI) -> Option<Self> {
        match openapi.get_api_key_name_and_location() {
            Some((name, location)) if location == "query" => Some(Self::Query(name)),
            Some((name, location)) if location == "header" => Some(Self::Header(name)),
            Some(_) => None,
            None => Some(Self::Header("Authorization".into())),
        }
    }

    fn header_name(&self) -> Option<&str> {
        match self {
            Self::Header(name) => Some(name),
            Self::Query(_) => None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ConnectorSkillCatalogue {
    pub prompt_section: Option<String>,
    pub built_in_prompt_section: Option<String>,
    pub connected_prompt_section: Option<String>,
    pub files: Vec<String>,
}

pub async fn connector_skill_catalogue_for_team(
    pool: &db::Pool,
    sub: &str,
    team_id: i32,
) -> Result<ConnectorSkillCatalogue, String> {
    let mut client = pool.get().await.map_err(|error| error.to_string())?;
    let transaction = client
        .transaction()
        .await
        .map_err(|error| error.to_string())?;
    db::authz::set_row_level_security_user_id(&transaction, sub.to_string())
        .await
        .map_err(|error| error.to_string())?;
    let connected = db::queries::connections::connected_integrations()
        .bind(&transaction, &team_id)
        .all()
        .await
        .map_err(|error| error.to_string())?;
    transaction
        .commit()
        .await
        .map_err(|error| error.to_string())?;

    let mut entries = Vec::new();
    let mut used = reserved_skill_slugs();
    for integration in connected {
        let Some(document) = integration.definition else {
            continue;
        };
        let Ok(openapi) = BionicOpenAPI::new(&document) else {
            continue;
        };
        if openapi.extract_base_url().is_none() {
            continue;
        }
        if ConnectorAuth::for_openapi(&openapi).is_none() {
            continue;
        }
        entries.push((
            integration.integration_name.clone(),
            unique_slug(&integration.integration_name, &mut used),
            false,
        ));
    }
    let overrides = crate::system_tool_sources::openapi_server_overrides();
    for spec in crate::system_tool_sources::load_system_openapi_specs(pool).await? {
        let Ok(openapi) = BionicOpenAPI::new(&spec.spec) else {
            continue;
        };
        if openapi.has_api_key_security() && spec.api_key.is_none() {
            continue;
        }
        if !overrides.contains_key(&spec.slug) && openapi.extract_base_url().is_none() {
            continue;
        }
        if ConnectorAuth::for_openapi(&openapi).is_none() {
            continue;
        }
        entries.push((
            spec.title,
            unique_slug(&spec.slug, &mut used),
            spec.is_builtin,
        ));
    }
    entries.push(("Scheduled Tasks".into(), "scheduled-tasks".into(), true));
    entries.sort_by(|left, right| left.1.cmp(&right.1));

    let section = |title: &str, entries: &[&(String, String, bool)]| {
        if entries.is_empty() {
            return None;
        }
        let lines = entries
            .iter()
            .map(|(name, slug, _)| format!("- {name}: /home/user/skills/{slug}/SKILL.md"))
            .collect::<Vec<_>>()
            .join("\n");
        Some(format!("{title}:\n{lines}"))
    };
    let built_in = entries.iter().filter(|entry| entry.2).collect::<Vec<_>>();
    let connected = entries.iter().filter(|entry| !entry.2).collect::<Vec<_>>();
    let built_in_prompt_section = section("Built-in capabilities", &built_in);
    let connected_prompt_section = section("Connected integrations", &connected);
    let mut prompt = String::from("Available connector skills:\nRead the relevant SKILL.md before using curl; authentication is supplied by the runtime.");
    for value in [&built_in_prompt_section, &connected_prompt_section]
        .into_iter()
        .flatten()
    {
        prompt.push('\n');
        prompt.push_str(value);
    }
    let files = entries
        .into_iter()
        .flat_map(|(_, slug, _)| {
            let mut files = vec![format!("/home/user/skills/{slug}/SKILL.md")];
            if slug != "scheduled-tasks" {
                files.push(format!("/home/user/skills/{slug}/openapi.json"));
            }
            files
        })
        .collect();
    Ok(ConnectorSkillCatalogue {
        prompt_section: Some(prompt),
        built_in_prompt_section,
        connected_prompt_section,
        files,
    })
}

pub async fn connector_prompt_for_conversation(
    pool: &db::Pool,
    sub: &str,
    conversation_id: i64,
) -> Result<Option<String>, String> {
    Ok(
        RuntimeNetwork::load_for_conversation(pool, sub, conversation_id)
            .await?
            .prompt_section(),
    )
}

pub(crate) struct RuntimeNetwork {
    client: Client,
    routes: HashMap<String, ConnectorRoute>,
    pool: db::Pool,
    sub: String,
    conversation_id: i64,
    model_id: i32,
    scheduled_context: crate::scheduled_tasks::Context,
}

impl RuntimeNetwork {
    pub(crate) fn prompt_section(&self) -> Option<String> {
        let mut routes = self.routes.values().collect::<Vec<_>>();
        routes.sort_by(|left, right| left.slug.cmp(&right.slug));
        let mut prompt = String::from(
            "Available connector skills:\nRead the relevant SKILL.md before using curl; authentication is supplied by the runtime.\n",
        );
        for route in routes {
            prompt.push_str(&format!(
                "- {}: /home/user/skills/{}/SKILL.md\n",
                route.name, route.slug
            ));
        }
        prompt.push_str("- Dataset search: /home/user/skills/dataset-search/SKILL.md\n");
        prompt.push_str("- Scheduled tasks: /home/user/skills/scheduled-tasks/SKILL.md\n");
        prompt.truncate(prompt.trim_end().len());
        Some(prompt)
    }
    pub(crate) async fn load_for_conversation(
        pool: &db::Pool,
        sub: &str,
        conversation_id: i64,
    ) -> Result<Arc<Self>, String> {
        let mut client = pool.get().await.map_err(|error| error.to_string())?;
        let transaction = client
            .transaction()
            .await
            .map_err(|error| error.to_string())?;
        db::authz::set_row_level_security_user_id(&transaction, sub.to_string())
            .await
            .map_err(|error| error.to_string())?;
        let conversation = transaction.query_one(
            "SELECT team_id, project_id FROM llm.conversations WHERE id = $1 AND user_id = current_app_user()",
            &[&conversation_id],
        ).await.map_err(|error| error.to_string())?;
        let team_id: i32 = conversation.get(0);
        let project_id: Option<i32> = conversation.get(1);
        let model_id: i32 = transaction.query_one(
            "SELECT model_id FROM llm.chats WHERE conversation_id = $1 ORDER BY id DESC LIMIT 1",
            &[&conversation_id],
        ).await.map_err(|error| error.to_string())?.get(0);
        let connected = db::queries::connections::connected_integrations()
            .bind(&transaction, &team_id)
            .all()
            .await
            .map_err(|error| error.to_string())?;
        transaction
            .commit()
            .await
            .map_err(|error| error.to_string())?;

        let mut routes = HashMap::new();
        let mut used = reserved_skill_slugs();
        for integration in connected {
            let Some(document) = integration.definition.clone() else {
                continue;
            };
            let Ok(openapi) = BionicOpenAPI::new(&document) else {
                continue;
            };
            let Some(base_url) = openapi.extract_base_url() else {
                continue;
            };
            let Some(auth) = ConnectorAuth::for_openapi(&openapi) else {
                continue;
            };
            let slug = unique_slug(&integration.integration_name, &mut used);
            let token_provider = crate::tool_auth::token_provider_for_connected_integration(
                pool.clone(),
                sub.to_string(),
                &integration,
                &openapi,
            );
            routes.insert(
                slug.clone(),
                ConnectorRoute {
                    slug,
                    name: integration.integration_name,
                    base_url,
                    document,
                    auth,
                    token_provider,
                },
            );
        }

        let overrides = crate::system_tool_sources::openapi_server_overrides();
        for spec in crate::system_tool_sources::load_system_openapi_specs(pool).await? {
            let Ok(openapi) = BionicOpenAPI::new(&spec.spec) else {
                continue;
            };
            if openapi.has_api_key_security() && spec.api_key.is_none() {
                continue;
            }
            let Some(base_url) = overrides
                .get(&spec.slug)
                .cloned()
                .or_else(|| openapi.extract_base_url())
            else {
                continue;
            };
            let Some(auth) = ConnectorAuth::for_openapi(&openapi) else {
                continue;
            };
            let slug = unique_slug(&spec.slug, &mut used);
            let token_provider = spec.api_key.map(|token| {
                Arc::new(crate::StaticTokenProvider::new(token)) as Arc<dyn TokenProvider>
            });
            routes.insert(
                slug.clone(),
                ConnectorRoute {
                    slug,
                    name: spec.title,
                    base_url,
                    document: spec.spec,
                    auth,
                    token_provider,
                },
            );
        }

        let client = Client::builder()
            .redirect(Policy::none())
            .no_proxy()
            .build()
            .map_err(|error| error.to_string())?;
        let scheduled_context = crate::scheduled_tasks::Context {
            pool: pool.clone(),
            sub: sub.to_string(),
            conversation_id,
            model_id,
            team_id,
            project_id,
        };
        Ok(Arc::new(Self {
            client,
            routes,
            pool: pool.clone(),
            sub: sub.to_string(),
            conversation_id,
            model_id,
            scheduled_context,
        }))
    }

    pub(crate) async fn seed_skills(&self, fs: &dyn bashkit::FileSystem) -> Result<(), String> {
        for route in self.routes.values() {
            let dir = format!("/home/user/skills/{}", route.slug);
            fs.mkdir(Path::new(&dir), true)
                .await
                .map_err(|error| error.to_string())?;
            let origin = connector_origin(&route.slug);
            let skill = format!(
                "# {}\n\nUse this connector with `curl`. Authentication is supplied by the runtime; never add credentials.\n\nInspect `{dir}/openapi.json` for operations. Its server is `{origin}`. Save large or binary responses with `curl -o /home/user/output/<name> ...`.\n",
                route.name,
            );
            fs.write_file(Path::new(&format!("{dir}/SKILL.md")), skill.as_bytes())
                .await
                .map_err(|error| error.to_string())?;
            let mut document = route.document.clone();
            document["servers"] = json!([{"url": origin}]);
            let bytes = serde_json::to_vec_pretty(&document).map_err(|error| error.to_string())?;
            fs.write_file(Path::new(&format!("{dir}/openapi.json")), &bytes)
                .await
                .map_err(|error| error.to_string())?;
        }
        let dir = "/home/user/skills/dataset-search";
        fs.mkdir(Path::new(dir), true)
            .await
            .map_err(|error| error.to_string())?;
        fs.write_file(Path::new(&format!("{dir}/SKILL.md")), format!(
            "# Dataset search\n\nSearch authorized datasets with:\n\n```bash\ncurl -sG --data-urlencode 'q=<query>' --data 'limit=5' https://{DATASET_SEARCH_HOST}/search\n```\n\nResults include virtual chunk paths. Read a selected chunk with `cat <path>`.\n"
        ).as_bytes()).await.map_err(|error| error.to_string())?;
        let dir = "/home/user/skills/scheduled-tasks";
        fs.mkdir(Path::new(dir), true)
            .await
            .map_err(|error| error.to_string())?;
        fs.write_file(Path::new(&format!("{dir}/SKILL.md")), format!(
            "# Scheduled tasks\n\nManage the current user's scheduled tasks through the internal HTTP capability. Use JSON bodies and `Content-Type: application/json`.\n\n- List: `curl -s https://{SCHEDULED_TASKS_HOST}/tasks`\n- Create: `curl -s -X POST https://{SCHEDULED_TASKS_HOST}/tasks -H 'Content-Type: application/json' -d '{{\"name\":\"...\",\"prompt\":\"...\",\"cron\":\"0 8 * * *\",\"timezone\":\"Europe/London\"}}'`\n- Update: `curl -s -X PATCH https://{SCHEDULED_TASKS_HOST}/tasks/<id> -H 'Content-Type: application/json' -d '{{\"enabled\":false}}'`\n- Delete: `curl -s -X DELETE https://{SCHEDULED_TASKS_HOST}/tasks/<id>`\n"
        ).as_bytes()).await.map_err(|error| error.to_string())?;
        Ok(())
    }

    fn route_for_url(&self, url: &Url) -> Option<&ConnectorRoute> {
        let host = url.host_str()?;
        let slug = host.strip_suffix(CONNECTOR_SUFFIX)?;
        self.routes.get(slug)
    }

    async fn connector_request(
        &self,
        route: &ConnectorRoute,
        request: HttpRequest,
        parsed: &Url,
    ) -> Result<HttpResponse, NetworkError> {
        if !operation_allowed(&route.document, request.method, parsed.path()) {
            return Err(NetworkError::Denied(
                "operation is not allowed by this connector".into(),
            ));
        }
        let mut upstream = upstream_url(&route.base_url, parsed)?;
        let mut headers = sanitized_headers(
            request.headers,
            route.auth.header_name().unwrap_or_default(),
            true,
        );
        remove_injected_auth(&route.auth, &mut upstream, &mut headers);
        if let Some(provider) = &route.token_provider {
            if let Some(token) = provider.token().await {
                inject_auth(&route.auth, &mut upstream, &mut headers, token);
            }
        }
        let mut response = self
            .send(
                &self.client,
                request.method,
                upstream.clone(),
                &headers,
                request.body.clone(),
                request.timeout,
                request.max_response_bytes,
            )
            .await?;
        if response.status == 401 {
            if let Some(provider) = &route.token_provider {
                provider.force_refresh().await;
                remove_injected_auth(&route.auth, &mut upstream, &mut headers);
                if let Some(token) = provider.token().await {
                    inject_auth(&route.auth, &mut upstream, &mut headers, token);
                }
                response = self
                    .send(
                        &self.client,
                        request.method,
                        upstream,
                        &headers,
                        request.body,
                        request.timeout,
                        request.max_response_bytes,
                    )
                    .await?;
            }
        }
        rewrite_connector_redirect(&mut response, route)?;
        Ok(response)
    }

    async fn anonymous_request(
        &self,
        request: HttpRequest,
        parsed: Url,
    ) -> Result<HttpResponse, NetworkError> {
        let addresses = validate_public_destination(&parsed).await?;
        let host = parsed
            .host_str()
            .ok_or_else(|| NetworkError::Denied("URL has no host".into()))?;
        let mut client = Client::builder()
            .redirect(Policy::none())
            .no_proxy()
            .resolve_to_addrs(host, &addresses);
        if let Some(timeout) = request.connect_timeout {
            client = client.connect_timeout(timeout);
        }
        let client = client
            .build()
            .map_err(|error| NetworkError::Transport(error.to_string()))?;
        let headers = sanitized_headers(request.headers, "authorization", false);
        self.send(
            &client,
            request.method,
            parsed,
            &headers,
            request.body,
            request.timeout,
            request.max_response_bytes,
        )
        .await
    }

    async fn dataset_search(
        &self,
        request: HttpRequest,
        parsed: &Url,
    ) -> Result<HttpResponse, NetworkError> {
        if request.method != HttpMethod::Get || parsed.path() != "/search" {
            return Err(NetworkError::Denied(
                "unknown dataset-search operation".into(),
            ));
        }
        let query = parsed
            .query_pairs()
            .find(|(name, _)| name == "q")
            .map(|(_, value)| value.into_owned())
            .unwrap_or_default();
        if query.trim().is_empty() {
            return Err(NetworkError::Denied("q is required".into()));
        }
        let limit = parsed
            .query_pairs()
            .find(|(name, _)| name == "limit")
            .and_then(|(_, value)| value.parse::<i32>().ok())
            .unwrap_or(5)
            .clamp(1, 20);
        let value = crate::builtin_tools::bashkit::execute_rag_search(
            &self.pool,
            &self.sub,
            self.conversation_id,
            self.model_id,
            &query,
            limit,
        )
        .await
        .map_err(|error| NetworkError::Transport(error.to_string()))?;
        Ok(HttpResponse {
            status: 200,
            headers: vec![HttpHeader {
                name: "content-type".into(),
                value: "application/json".into(),
            }],
            body: serde_json::to_vec(&value)
                .map_err(|error| NetworkError::Transport(error.to_string()))?,
        })
    }

    async fn scheduled_tasks(
        &self,
        request: HttpRequest,
        parsed: &Url,
    ) -> Result<HttpResponse, NetworkError> {
        let segments = parsed
            .path()
            .trim_matches('/')
            .split('/')
            .collect::<Vec<_>>();
        let (operation, arguments) = match (request.method, segments.as_slice()) {
            (HttpMethod::Get, ["tasks"]) => (crate::scheduled_tasks::Operation::List, json!({})),
            (HttpMethod::Post, ["tasks"]) => (
                crate::scheduled_tasks::Operation::Create,
                parse_json_body(request.body)?,
            ),
            (HttpMethod::Patch, ["tasks", id]) => (
                crate::scheduled_tasks::Operation::Update,
                with_task_id(parse_json_body(request.body)?, id)?,
            ),
            (HttpMethod::Delete, ["tasks", id]) => (
                crate::scheduled_tasks::Operation::Delete,
                with_task_id(json!({}), id)?,
            ),
            _ => {
                return Err(NetworkError::Denied(
                    "unknown scheduled-task operation".into(),
                ))
            }
        };
        let value = crate::scheduled_tasks::execute(&self.scheduled_context, operation, arguments)
            .await
            .map_err(NetworkError::Transport)?;
        Ok(HttpResponse {
            status: 200,
            headers: vec![HttpHeader {
                name: "content-type".into(),
                value: "application/json".into(),
            }],
            body: serde_json::to_vec(&value)
                .map_err(|error| NetworkError::Transport(error.to_string()))?,
        })
    }

    #[allow(clippy::too_many_arguments)]
    async fn send(
        &self,
        client: &Client,
        method: HttpMethod,
        url: Url,
        headers: &[HttpHeader],
        body: Option<Vec<u8>>,
        timeout: std::time::Duration,
        max: u64,
    ) -> Result<HttpResponse, NetworkError> {
        let method = match method {
            HttpMethod::Get => reqwest::Method::GET,
            HttpMethod::Post => reqwest::Method::POST,
            HttpMethod::Put => reqwest::Method::PUT,
            HttpMethod::Delete => reqwest::Method::DELETE,
            HttpMethod::Head => reqwest::Method::HEAD,
            HttpMethod::Patch => reqwest::Method::PATCH,
        };
        let mut builder = client.request(method, url).timeout(timeout);
        for header in headers {
            builder = builder.header(&header.name, &header.value);
        }
        if let Some(body) = body {
            builder = builder.body(body);
        }
        let response = builder.send().await.map_err(|error| {
            if error.is_timeout() {
                NetworkError::Timeout
            } else {
                NetworkError::Transport(error.to_string())
            }
        })?;
        let status = response.status().as_u16();
        let headers = response
            .headers()
            .iter()
            .filter_map(|(name, value)| {
                value.to_str().ok().map(|value| HttpHeader {
                    name: name.to_string(),
                    value: value.to_string(),
                })
            })
            .collect();
        if response.content_length().is_some_and(|length| length > max) {
            return Err(NetworkError::TooLarge(format!("limit is {max} bytes")));
        }
        use futures_util::StreamExt;
        let mut stream = response.bytes_stream();
        let mut body = Vec::new();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|error| NetworkError::Transport(error.to_string()))?;
            if body.len() as u64 + chunk.len() as u64 > max {
                return Err(NetworkError::TooLarge(format!("limit is {max} bytes")));
            }
            body.extend_from_slice(&chunk);
        }
        Ok(HttpResponse {
            status,
            headers,
            body,
        })
    }
}

#[async_trait]
impl SandboxNetwork for RuntimeNetwork {
    async fn request(&self, request: HttpRequest) -> Result<HttpResponse, NetworkError> {
        let parsed =
            Url::parse(&request.url).map_err(|error| NetworkError::Denied(error.to_string()))?;
        if !matches!(parsed.scheme(), "http" | "https") {
            return Err(NetworkError::Denied(
                "only HTTP and HTTPS are supported".into(),
            ));
        }
        if !parsed.username().is_empty() || parsed.password().is_some() {
            return Err(NetworkError::Denied(
                "credentials in URLs are not allowed".into(),
            ));
        }
        let method = request.method;
        let host = parsed.host_str().unwrap_or_default().to_string();
        let path = parsed.path().to_string();
        let result = if parsed.host_str() == Some(DATASET_SEARCH_HOST) {
            self.dataset_search(request, &parsed).await
        } else if parsed.host_str() == Some(SCHEDULED_TASKS_HOST) {
            self.scheduled_tasks(request, &parsed).await
        } else if let Some(route) = self.route_for_url(&parsed) {
            self.connector_request(route, request, &parsed).await
        } else if parsed
            .host_str()
            .is_some_and(|host| host.ends_with(CONNECTOR_SUFFIX))
        {
            Err(NetworkError::Denied("unknown connector origin".into()))
        } else if parsed
            .host_str()
            .is_some_and(|host| host.ends_with(".internal.invalid"))
        {
            Err(NetworkError::Denied("unknown internal capability".into()))
        } else {
            self.anonymous_request(request, parsed).await
        };
        match &result {
            Ok(response) => tracing::info!(
                target: "tool_runtime::network_audit",
                ?method,
                %host,
                %path,
                status = response.status,
                "sandbox HTTP request completed"
            ),
            Err(error) => tracing::warn!(
                target: "tool_runtime::network_audit",
                ?method,
                %host,
                %path,
                error = %error,
                "sandbox HTTP request failed"
            ),
        }
        result
    }
}

fn connector_origin(slug: &str) -> String {
    format!("https://{slug}{CONNECTOR_SUFFIX}")
}

fn reserved_skill_slugs() -> HashSet<String> {
    ["dataset-search", "scheduled-tasks"]
        .into_iter()
        .map(str::to_string)
        .collect()
}

fn parse_json_body(body: Option<Vec<u8>>) -> Result<Value, NetworkError> {
    serde_json::from_slice(body.as_deref().unwrap_or_default())
        .map_err(|error| NetworkError::Denied(format!("invalid JSON body: {error}")))
}

fn with_task_id(mut value: Value, id: &str) -> Result<Value, NetworkError> {
    let id = id
        .parse::<i64>()
        .map_err(|_| NetworkError::Denied("task id must be an integer".into()))?;
    let object = value
        .as_object_mut()
        .ok_or_else(|| NetworkError::Denied("request body must be a JSON object".into()))?;
    object.insert("task_id".into(), json!(id));
    Ok(value)
}

fn remove_injected_auth(auth: &ConnectorAuth, url: &mut Url, headers: &mut Vec<HttpHeader>) {
    match auth {
        ConnectorAuth::Header(name) => {
            headers.retain(|header| !header.name.eq_ignore_ascii_case(name));
        }
        ConnectorAuth::Query(name) => {
            let pairs = url
                .query_pairs()
                .filter(|(key, _)| key != name)
                .map(|(key, value)| (key.into_owned(), value.into_owned()))
                .collect::<Vec<_>>();
            url.set_query(None);
            if !pairs.is_empty() {
                url.query_pairs_mut().extend_pairs(pairs);
            }
        }
    }
}

fn inject_auth(auth: &ConnectorAuth, url: &mut Url, headers: &mut Vec<HttpHeader>, token: String) {
    match auth {
        ConnectorAuth::Header(name) => {
            let value = if name.eq_ignore_ascii_case("authorization")
                && !token.to_ascii_lowercase().starts_with("basic ")
            {
                format!("Bearer {token}")
            } else {
                token
            };
            headers.push(HttpHeader {
                name: name.clone(),
                value,
            });
        }
        ConnectorAuth::Query(name) => {
            url.query_pairs_mut().append_pair(name, &token);
        }
    }
}

fn sanitized_headers(
    headers: Vec<HttpHeader>,
    auth_header: &str,
    connector: bool,
) -> Vec<HttpHeader> {
    headers
        .into_iter()
        .filter(|header| {
            let name = header.name.as_str();
            !name.eq_ignore_ascii_case("host")
                && !name.eq_ignore_ascii_case("proxy-authorization")
                && !name.eq_ignore_ascii_case("cookie")
                && !name.eq_ignore_ascii_case("authorization")
                && (!connector || !name.eq_ignore_ascii_case(auth_header))
        })
        .collect()
}

fn upstream_url(base: &str, request: &Url) -> Result<Url, NetworkError> {
    let value = format!(
        "{}{}{}",
        base.trim_end_matches('/'),
        request.path(),
        request
            .query()
            .map(|query| format!("?{query}"))
            .unwrap_or_default()
    );
    Url::parse(&value).map_err(|error| NetworkError::Denied(error.to_string()))
}

fn operation_allowed(document: &Value, method: HttpMethod, path: &str) -> bool {
    let method = match method {
        HttpMethod::Get => "get",
        HttpMethod::Post => "post",
        HttpMethod::Put => "put",
        HttpMethod::Delete => "delete",
        HttpMethod::Head => "head",
        HttpMethod::Patch => "patch",
    };
    document
        .get("paths")
        .and_then(Value::as_object)
        .is_some_and(|paths| {
            paths
                .iter()
                .any(|(template, item)| item.get(method).is_some() && path_matches(template, path))
        })
}

fn path_matches(template: &str, path: &str) -> bool {
    let template = template.trim_matches('/').split('/').collect::<Vec<_>>();
    let path = path.trim_matches('/').split('/').collect::<Vec<_>>();
    template.len() == path.len()
        && template.iter().zip(path).all(|(expected, actual)| {
            (expected.starts_with('{') && expected.ends_with('}')) || expected == &actual
        })
}

fn rewrite_connector_redirect(
    response: &mut HttpResponse,
    route: &ConnectorRoute,
) -> Result<(), NetworkError> {
    if !(300..400).contains(&response.status) {
        return Ok(());
    }
    let base =
        Url::parse(&route.base_url).map_err(|error| NetworkError::Transport(error.to_string()))?;
    for header in &mut response.headers {
        if header.name.eq_ignore_ascii_case("location") {
            let mut target = base
                .join(&header.value)
                .map_err(|error| NetworkError::Denied(error.to_string()))?;
            if target.scheme() != base.scheme()
                || target.host_str() != base.host_str()
                || target.port_or_known_default() != base.port_or_known_default()
            {
                return Err(NetworkError::Denied(
                    "connector redirect crossed its configured origin".into(),
                ));
            }
            if let ConnectorAuth::Query(_) = &route.auth {
                let mut ignored_headers = Vec::new();
                remove_injected_auth(&route.auth, &mut target, &mut ignored_headers);
            }
            header.value = format!(
                "{}{}{}",
                connector_origin(&route.slug),
                target.path(),
                target
                    .query()
                    .map(|query| format!("?{query}"))
                    .unwrap_or_default()
            );
        }
    }
    Ok(())
}

async fn validate_public_destination(url: &Url) -> Result<Vec<SocketAddr>, NetworkError> {
    let host = url
        .host_str()
        .ok_or_else(|| NetworkError::Denied("URL has no host".into()))?;
    let port = url
        .port_or_known_default()
        .ok_or_else(|| NetworkError::Denied("URL has no port".into()))?;
    if let Ok(ip) = host.parse::<IpAddr>() {
        if !is_public_ip(ip) {
            return Err(NetworkError::Denied("private or reserved address".into()));
        }
        return Ok(vec![SocketAddr::new(ip, port)]);
    }
    let addresses = tokio::net::lookup_host((host, port))
        .await
        .map_err(|error| NetworkError::Transport(error.to_string()))?;
    let mut found = false;
    let mut validated = Vec::new();
    for address in addresses {
        found = true;
        if !is_public_ip(address.ip()) {
            return Err(NetworkError::Denied(
                "host resolves to a private or reserved address".into(),
            ));
        }
        validated.push(address);
    }
    if !found {
        return Err(NetworkError::Denied("host did not resolve".into()));
    }
    Ok(validated)
}

fn is_public_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(ip) => {
            !(ip.is_private()
                || ip.is_loopback()
                || ip.is_link_local()
                || ip.is_broadcast()
                || ip.is_unspecified()
                || ip.is_multicast()
                || ip.octets()[0] == 0)
        }
        IpAddr::V6(ip) => {
            !(ip.is_loopback()
                || ip.is_unspecified()
                || ip.is_multicast()
                || (ip.segments()[0] & 0xfe00) == 0xfc00
                || (ip.segments()[0] & 0xffc0) == 0xfe80)
        }
    }
}

fn unique_slug(value: &str, used: &mut HashSet<String>) -> String {
    let base = slugify(value);
    let mut candidate = base.clone();
    let mut suffix = 2;
    while !used.insert(candidate.clone()) {
        candidate = format!("{base}-{suffix}");
        suffix += 1;
    }
    candidate
}

fn slugify(value: &str) -> String {
    let mut slug = String::new();
    let mut separator = false;
    for character in value.chars().flat_map(char::to_lowercase) {
        if character.is_ascii_alphanumeric() {
            slug.push(character);
            separator = false;
        } else if !separator && !slug.is_empty() {
            slug.push('-');
            separator = true;
        }
    }
    while slug.ends_with('-') {
        slug.pop();
    }
    if slug.is_empty() {
        "connector".into()
    } else {
        slug
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn openapi_paths_match_templated_segments_only() {
        assert!(path_matches(
            "/gmail/v1/users/{userId}/messages",
            "/gmail/v1/users/me/messages"
        ));
        assert!(!path_matches(
            "/gmail/v1/users/{userId}/messages",
            "/gmail/v1/users/me/settings"
        ));
    }

    #[test]
    fn gmail_oauth_route_uses_documented_operation_and_bearer_header() {
        let document = json!({
            "paths": {
                "/gmail/v1/users/{userId}/messages": {"get": {"operationId": "listMessages"}}
            }
        });
        assert!(operation_allowed(
            &document,
            HttpMethod::Get,
            "/gmail/v1/users/me/messages"
        ));
        let mut url =
            Url::parse("https://gmail.googleapis.com/gmail/v1/users/me/messages").unwrap();
        let mut headers = Vec::new();
        inject_auth(
            &ConnectorAuth::Header("Authorization".into()),
            &mut url,
            &mut headers,
            "oauth-token".into(),
        );
        assert_eq!(
            headers,
            vec![HttpHeader {
                name: "Authorization".into(),
                value: "Bearer oauth-token".into(),
            }]
        );
    }

    #[test]
    fn private_addresses_are_not_public() {
        assert!(!is_public_ip("127.0.0.1".parse().unwrap()));
        assert!(!is_public_ip("169.254.169.254".parse().unwrap()));
        assert!(!is_public_ip("::1".parse().unwrap()));
        assert!(is_public_ip("8.8.8.8".parse().unwrap()));
    }

    #[test]
    fn caller_credentials_are_removed_before_routing() {
        let headers = sanitized_headers(
            vec![
                HttpHeader {
                    name: "Authorization".into(),
                    value: "stolen".into(),
                },
                HttpHeader {
                    name: "X-API-Key".into(),
                    value: "stolen".into(),
                },
                HttpHeader {
                    name: "Cookie".into(),
                    value: "session=stolen".into(),
                },
                HttpHeader {
                    name: "Accept".into(),
                    value: "application/json".into(),
                },
            ],
            "X-API-Key",
            true,
        );
        assert_eq!(
            headers,
            vec![HttpHeader {
                name: "Accept".into(),
                value: "application/json".into()
            }]
        );
    }

    #[test]
    fn connector_redirects_stay_on_the_virtual_origin() {
        let route = ConnectorRoute {
            slug: "gmail".into(),
            name: "Gmail".into(),
            base_url: "https://gmail.googleapis.com".into(),
            document: json!({}),
            auth: ConnectorAuth::Header("Authorization".into()),
            token_provider: None,
        };
        let mut response = HttpResponse {
            status: 302,
            headers: vec![HttpHeader {
                name: "Location".into(),
                value: "/gmail/v1/users/me/messages?page=2".into(),
            }],
            body: Vec::new(),
        };
        rewrite_connector_redirect(&mut response, &route).unwrap();
        assert_eq!(
            response.headers[0].value,
            "https://gmail.connectors.invalid/gmail/v1/users/me/messages?page=2"
        );

        response.headers[0].value = "https://evil.example/steal".into();
        assert!(matches!(
            rewrite_connector_redirect(&mut response, &route),
            Err(NetworkError::Denied(_))
        ));
    }

    #[test]
    fn internal_task_route_owns_the_path_task_id() {
        let value = with_task_id(json!({"enabled": false, "task_id": 999}), "42").unwrap();
        assert_eq!(value["task_id"], 42);
        assert_eq!(value["enabled"], false);
        assert!(with_task_id(json!({}), "not-an-id").is_err());
    }

    #[test]
    fn query_api_keys_replace_caller_values_and_do_not_leak_in_redirects() {
        let auth = ConnectorAuth::Query("key".into());
        let mut url = Url::parse("https://api.example.test/items?key=caller&page=2").unwrap();
        let mut headers = Vec::new();
        remove_injected_auth(&auth, &mut url, &mut headers);
        inject_auth(&auth, &mut url, &mut headers, "runtime-secret".into());
        assert_eq!(
            url.as_str(),
            "https://api.example.test/items?page=2&key=runtime-secret"
        );

        let route = ConnectorRoute {
            slug: "example".into(),
            name: "Example".into(),
            base_url: "https://api.example.test".into(),
            document: json!({}),
            auth,
            token_provider: None,
        };
        let mut response = HttpResponse {
            status: 302,
            headers: vec![HttpHeader {
                name: "Location".into(),
                value: "https://api.example.test/next?key=runtime-secret&page=3".into(),
            }],
            body: Vec::new(),
        };
        rewrite_connector_redirect(&mut response, &route).unwrap();
        assert_eq!(
            response.headers[0].value,
            "https://example.connectors.invalid/next?page=3"
        );
    }
}
