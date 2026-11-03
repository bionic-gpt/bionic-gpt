use crate::builtin_tools::openapi_tool_adapter::{
    HttpClient, HttpRequestBody, HttpResponse as ToolHttpResponse,
};
use crate::{BionicOpenAPI, ToolDyn, ToolError};
use async_trait::async_trait;
use base64::Engine;
use bashkit::FileSystem;
use reqwest::{Method, StatusCode, Url};
use sandbox::{
    HttpHeader, HttpMethod, HttpRequest, PythonCallArguments, PythonFunction, PythonFunctionError,
    SandboxNetwork,
};
use serde_json::{json, Map, Value};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
const MAX_RESPONSE_BYTES: u64 = 20 * 1024 * 1024;
/// Largest function list written inline in `SKILL.md`; bigger connectors get a
/// grouped overview and are searched with the discovery functions instead.
const SKILL_FULL_LIST_BYTES: usize = 6 * 1024;
const SKILL_DESCRIPTION_CHARS: usize = 1500;
const SKILL_MAX_GROUPS: usize = 80;
/// Operation files are kept near this size; optional arguments beyond it are summarised.
const OPERATION_DOC_BYTES: usize = 8 * 1024;
const OPERATION_DESCRIPTION_CHARS: usize = 2000;
const PROPERTY_DESCRIPTION_CHARS: usize = 120;
const SCHEMA_MAX_DEPTH: usize = 2;
const SCHEMA_MAX_NESTED_PROPERTIES: usize = 30;
/// Nested expansion of one argument stops after this many bytes.
const SCHEMA_BLOCK_BYTES: usize = 4 * 1024;
const NAMED_SCHEMA_DOC_BYTES: usize = 16 * 1024;
const FIND_DEFAULT_LIMIT: u64 = 20;
const FIND_MAX_LIMIT: u64 = 100;

#[derive(Clone)]
pub(crate) struct ConnectorDocument {
    pub name: String,
    pub slug: String,
    pub document: Value,
    pub is_builtin: bool,
}

#[derive(Clone)]
pub(crate) struct ConnectorOperation {
    pub function_name: String,
    pub description: String,
    pub parameters: Value,
    pub operation_id: String,
    pub method: String,
    pub path: String,
    pub tags: Vec<String>,
}

#[derive(Clone)]
pub(crate) struct ConnectorInfo {
    pub name: String,
    pub slug: String,
    pub is_builtin: bool,
    pub description: Option<String>,
    /// `components.schemas`, used to render `$ref`s by name.
    pub schemas: Arc<Value>,
    pub operations: Vec<ConnectorOperation>,
}

pub(crate) fn catalogue(documents: &[ConnectorDocument]) -> Vec<ConnectorInfo> {
    let mut documents = documents.to_vec();
    for document in &mut documents {
        normalize_parameters(&mut document.document);
    }
    catalogue_normalized(&documents)
}

/// `catalogue` for documents that have already been through `normalize_parameters`.
fn catalogue_normalized(documents: &[ConnectorDocument]) -> Vec<ConnectorInfo> {
    let mut documents = documents.iter().collect::<Vec<_>>();
    documents.sort_by(|left, right| left.slug.cmp(&right.slug));
    // Reserve every connector's discovery names before any operation is named,
    // so an operation of an earlier connector cannot take a later one's.
    let mut used = HashSet::new();
    for document in &documents {
        for kind in ["find", "describe"] {
            used.insert(discovery_function_name(&document.slug, kind));
        }
    }
    documents
        .into_iter()
        .filter_map(|document| {
            let openapi = BionicOpenAPI::new(&document.document).ok()?;
            let mut definitions = openapi.create_tool_definitions().tool_definitions;
            definitions.sort_by(|left, right| left.name.cmp(&right.name));
            let support = supported_operations(&document.document);
            let index = operation_index(&document.document);
            let schemas = Arc::new(
                document
                    .document
                    .pointer("/components/schemas")
                    .cloned()
                    .unwrap_or(Value::Null),
            );
            let operations = definitions
                .into_iter()
                .filter(|definition| match support.get(&definition.name) {
                    Some(None) => true,
                    Some(Some(reason)) => {
                        tracing::warn!(
                            connector = %document.slug,
                            operation = %definition.name,
                            %reason,
                            "connector operation is not available as a Python function"
                        );
                        false
                    }
                    None => false,
                })
                .map(|definition| {
                    let function_name =
                        operation_function_name(&document.slug, &definition.name, &mut used);
                    let (parameters, _) = expose_file_parameters(definition.parameters);
                    let (method, path, tags) =
                        index.get(&definition.name).cloned().unwrap_or_default();
                    ConnectorOperation {
                        function_name,
                        description: definition.description,
                        parameters,
                        operation_id: definition.name,
                        method,
                        path,
                        tags,
                    }
                })
                .collect();
            Some(ConnectorInfo {
                name: document.name.clone(),
                slug: document.slug.clone(),
                is_builtin: document.is_builtin,
                description: openapi
                    .get_description()
                    .filter(|value| !value.trim().is_empty()),
                schemas,
                operations,
            })
        })
        .collect()
}

const OPERATION_METHODS: [&str; 6] = ["get", "post", "put", "delete", "head", "patch"];

fn resolve_parameter_ref(components: &Value, parameter: &Value) -> Value {
    parameter
        .get("$ref")
        .and_then(Value::as_str)
        .and_then(|reference| reference.strip_prefix("#/components/parameters/"))
        .and_then(|name| components.get(name))
        .cloned()
        .unwrap_or_else(|| parameter.clone())
}

/// Copies path-level parameters into every operation of the path (operation
/// parameters win on name and location) and inlines `$ref` parameters, so the
/// rest of the pipeline only ever sees plain per-operation parameters.
/// Header parameters that OpenAPI says to ignore (`Accept`, `Content-Type`,
/// `Authorization`) or that carry the API key, which the runtime injects, are
/// removed so the model is never asked for them.
/// Idempotent. References that cannot be resolved are left in place.
fn normalize_parameters(document: &mut Value) {
    // Only the parameter components are needed while `paths` is borrowed mutably;
    // cloning the whole document doubled peak memory on multi-MB specs.
    let components = document
        .pointer("/components/parameters")
        .cloned()
        .unwrap_or(Value::Null);
    let runtime_headers = document
        .pointer("/components/securitySchemes")
        .and_then(Value::as_object)
        .into_iter()
        .flatten()
        .filter(|(_, scheme)| {
            scheme.get("type").and_then(Value::as_str) == Some("apiKey")
                && scheme.get("in").and_then(Value::as_str) == Some("header")
        })
        .filter_map(|(_, scheme)| scheme.get("name").and_then(Value::as_str))
        .map(str::to_ascii_lowercase)
        .chain(["accept", "content-type", "authorization"].map(String::from))
        .collect::<HashSet<_>>();
    let runtime_owned = |parameter: &Value| {
        parameter.get("in").and_then(Value::as_str) == Some("header")
            && parameter
                .get("name")
                .and_then(Value::as_str)
                .is_some_and(|name| runtime_headers.contains(&name.to_ascii_lowercase()))
    };
    let Some(paths) = document.get_mut("paths").and_then(Value::as_object_mut) else {
        return;
    };
    for path_item in paths.values_mut().filter_map(Value::as_object_mut) {
        let inherited = path_item
            .remove("parameters")
            .and_then(|value| value.as_array().cloned())
            .unwrap_or_default()
            .iter()
            .map(|parameter| resolve_parameter_ref(&components, parameter))
            .collect::<Vec<_>>();
        for method in OPERATION_METHODS {
            let Some(operation) = path_item.get_mut(method).and_then(Value::as_object_mut) else {
                continue;
            };
            let own = operation
                .get("parameters")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .map(|parameter| resolve_parameter_ref(&components, parameter))
                .collect::<Vec<_>>();
            if inherited.is_empty() && own.is_empty() {
                continue;
            }
            let key = |parameter: &Value| {
                (
                    parameter
                        .get("name")
                        .and_then(Value::as_str)
                        .map(str::to_string),
                    parameter
                        .get("in")
                        .and_then(Value::as_str)
                        .map(str::to_string),
                )
            };
            let mut merged = inherited
                .iter()
                .filter(|parameter| !own.iter().any(|own| key(own) == key(parameter)))
                .cloned()
                .collect::<Vec<_>>();
            merged.extend(own);
            merged.retain(|parameter| !runtime_owned(parameter));
            operation.insert("parameters".into(), Value::Array(merged));
        }
    }
}

fn supported_operations(document: &Value) -> HashMap<String, Option<String>> {
    let mut operations = HashMap::new();
    let Some(paths) = document.get("paths").and_then(Value::as_object) else {
        return operations;
    };
    for path_item in paths.values().filter_map(Value::as_object) {
        for method in OPERATION_METHODS {
            let Some(operation) = path_item.get(method).and_then(Value::as_object) else {
                continue;
            };
            let Some(operation_id) = operation.get("operationId").and_then(Value::as_str) else {
                continue;
            };
            let reason = if operation
                .get("parameters")
                .and_then(Value::as_array)
                .is_some_and(|parameters| {
                    parameters.iter().any(|parameter| {
                        parameter.get("$ref").is_some()
                            || parameter.get("in").and_then(Value::as_str) == Some("cookie")
                    })
                }) {
                Some("unresolved and cookie parameters are not supported".to_string())
            } else {
                operation
                    .get("requestBody")
                    .and_then(|body| body.get("content"))
                    .and_then(Value::as_object)
                    .filter(|content| {
                        !content.contains_key("application/json")
                            && !content.contains_key("multipart/form-data")
                            && !content.contains_key("application/x-www-form-urlencoded")
                    })
                    .map(|_| "request body content type is not supported".to_string())
            };
            operations.insert(operation_id.to_string(), reason);
        }
    }
    operations
}

/// (method, path, tags) for every operation that has an operationId.
fn operation_index(document: &Value) -> HashMap<String, (String, String, Vec<String>)> {
    let mut index = HashMap::new();
    let Some(paths) = document.get("paths").and_then(Value::as_object) else {
        return index;
    };
    for (path, path_item) in paths {
        for method in ["get", "post", "put", "delete", "head", "patch"] {
            let Some(operation) = path_item.get(method).and_then(Value::as_object) else {
                continue;
            };
            let Some(operation_id) = operation.get("operationId").and_then(Value::as_str) else {
                continue;
            };
            let tags = operation
                .get("tags")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect();
            index.insert(
                operation_id.to_string(),
                (method.to_ascii_uppercase(), path.clone(), tags),
            );
        }
    }
    index
}

fn discovery_function_name(slug: &str, kind: &str) -> String {
    format!("{}_{kind}", sanitize_identifier(slug))
}

fn operation_group(operation: &ConnectorOperation) -> String {
    operation
        .tags
        .first()
        .cloned()
        .or_else(|| {
            operation
                .path
                .split('/')
                .find(|segment| !segment.is_empty() && !segment.starts_with('{'))
                .map(str::to_string)
        })
        .unwrap_or_else(|| "other".into())
}

fn truncate_text(value: &str, max_characters: usize) -> String {
    let value = value.trim();
    if value.chars().count() <= max_characters {
        return value.to_string();
    }
    let mut text = value.chars().take(max_characters).collect::<String>();
    if let Some(last_space) = text.rfind(char::is_whitespace) {
        text.truncate(last_space);
    }
    text.push('…');
    text
}

pub(crate) fn skill_markdown(connector: &ConnectorInfo) -> String {
    let find = discovery_function_name(&connector.slug, "find");
    let describe = discovery_function_name(&connector.slug, "describe");
    let mut markdown = format!(
        "# {}\n\nUse this connector from `run_python`. Its functions are supplied by Bionic at runtime; do not import a module or add credentials. Functions accept keyword arguments or one positional dictionary.\n\nRead an operation file under `/home/user/skills/{}/operations/` when you need its exact arguments.\n\n",
        connector.name, connector.slug
    );
    if let Some(description) = &connector.description {
        markdown.push_str(&format!(
            "## About\n\n{}\n\n",
            truncate_text(description, SKILL_DESCRIPTION_CHARS)
        ));
    }
    let list = connector
        .operations
        .iter()
        .map(|operation| {
            format!(
                "- `{}` — {} ([details](operations/{}.md))\n",
                operation.function_name,
                compact_summary(&operation.description, 180),
                operation.function_name
            )
        })
        .collect::<String>();
    if list.len() <= SKILL_FULL_LIST_BYTES {
        markdown.push_str("## Functions\n\n");
        markdown.push_str(&list);
        markdown.push_str(&format!(
            "\nSearch these with `{find}(\"keyword\")`; `{describe}(\"<function>\")` returns a function's arguments.\n"
        ));
        return markdown;
    }
    let example = connector
        .operations
        .first()
        .map(|operation| operation.function_name.as_str())
        .unwrap_or("<function>");
    markdown.push_str(&format!(
        "## Finding functions\n\nThis connector has {} functions, too many to list. Search them, then read the arguments of the one you need:\n\n```python\nprint({find}(\"keyword\"))\nprint({describe}(\"{example}\"))\n```\n\n`{find}` takes `query` (all words must match the function name, path, tag or description) and optional `limit` (default {FIND_DEFAULT_LIMIT}, max {FIND_MAX_LIMIT}). The same details are in `/home/user/skills/{}/operations/<function>.md`.\n\n## Groups\n\n",
        connector.operations.len(),
        connector.slug
    ));
    let mut groups = std::collections::BTreeMap::<String, usize>::new();
    for operation in &connector.operations {
        *groups.entry(operation_group(operation)).or_default() += 1;
    }
    for (group, count) in groups.iter().take(SKILL_MAX_GROUPS) {
        markdown.push_str(&format!("- `{group}`: {count}\n"));
    }
    if groups.len() > SKILL_MAX_GROUPS {
        markdown.push_str(&format!(
            "- and {} more groups\n",
            groups.len() - SKILL_MAX_GROUPS
        ));
    }
    markdown
}

fn compact_summary(value: &str, max_characters: usize) -> String {
    let value = value.split_whitespace().collect::<Vec<_>>().join(" ");
    if value.chars().count() <= max_characters {
        return value;
    }
    let mut summary = value.chars().take(max_characters).collect::<String>();
    if let Some(last_space) = summary.rfind(' ') {
        summary.truncate(last_space);
    }
    summary.push('…');
    summary
}

pub(crate) fn operation_markdown(
    connector: &ConnectorInfo,
    operation: &ConnectorOperation,
) -> String {
    let describe = discovery_function_name(&connector.slug, "describe");
    let mut markdown = format!(
        "# {} — `{}`\n\n{}\n\nOpenAPI operation: `{}` ({} {})\n\n## Arguments\n\n",
        connector.name,
        operation.function_name,
        truncate_text(&operation.description, OPERATION_DESCRIPTION_CHARS),
        operation.operation_id,
        operation.method,
        operation.path
    );
    let (arguments, omitted) = render_capped(
        &connector.schemas,
        &operation.parameters,
        OPERATION_DOC_BYTES,
    );
    if arguments.is_empty() {
        markdown.push_str("This function takes no arguments.\n");
    } else {
        markdown.push_str(&arguments);
        if omitted > 0 {
            markdown.push_str(&format!(
                "\n[{omitted} more optional arguments not shown to keep this file short]\n"
            ));
        }
        if connector
            .schemas
            .as_object()
            .is_some_and(|map| !map.is_empty())
        {
            markdown.push_str(&format!(
                "\nA type shown by name (for example `Ticket`) can be expanded with `{describe}(\"Ticket\")`.\n"
            ));
        }
    }
    markdown.push_str(&format!(
        "\n## Example\n\n```python\nresult = {}({})\nprint(result)\n```\n",
        operation.function_name,
        example_arguments(&operation.parameters),
    ));
    markdown
}

fn component_name(schema: &Value) -> Option<&str> {
    schema
        .get("$ref")?
        .as_str()?
        .strip_prefix("#/components/schemas/")
}

fn resolve_schema<'a>(schemas: &'a Value, schema: &'a Value) -> &'a Value {
    let mut current = schema;
    for _ in 0..8 {
        match component_name(current).and_then(|name| schemas.get(name)) {
            Some(next) => current = next,
            None => break,
        }
    }
    current
}

/// Properties and required names of an object schema, following `$ref` and `allOf`.
fn object_properties<'a>(
    schemas: &'a Value,
    schema: &'a Value,
    allof_depth: usize,
) -> (Vec<(&'a str, &'a Value)>, HashSet<&'a str>) {
    let schema = resolve_schema(schemas, schema);
    let mut properties = Vec::new();
    let mut required = HashSet::new();
    if let Some(map) = schema.get("properties").and_then(Value::as_object) {
        properties.extend(map.iter().map(|(name, value)| (name.as_str(), value)));
    }
    if let Some(names) = schema.get("required").and_then(Value::as_array) {
        required.extend(names.iter().filter_map(Value::as_str));
    }
    if allof_depth > 0 {
        for part in schema
            .get("allOf")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            let (inner, inner_required) = object_properties(schemas, part, allof_depth - 1);
            properties.extend(inner);
            required.extend(inner_required);
        }
    }
    (properties, required)
}

fn schema_label(schema: &Value, depth: usize) -> String {
    if let Some(name) = component_name(schema) {
        return name.to_string();
    }
    if depth > 3 {
        return "value".into();
    }
    for key in ["oneOf", "anyOf"] {
        if let Some(variants) = schema.get(key).and_then(Value::as_array) {
            let mut labels = variants
                .iter()
                .take(4)
                .map(|variant| schema_label(variant, depth + 1))
                .collect::<Vec<_>>()
                .join(" | ");
            if variants.len() > 4 {
                labels.push_str(" | …");
            }
            return labels;
        }
    }
    if let Some(values) = schema.get("enum").and_then(Value::as_array) {
        let mut shown = values
            .iter()
            .take(8)
            .map(|value| match value.as_str() {
                Some(text) => text.to_string(),
                None => value.to_string(),
            })
            .collect::<Vec<_>>()
            .join("|");
        if values.len() > 8 {
            shown.push_str("|…");
        }
        return format!("enum({shown})");
    }
    match schema.get("type").and_then(Value::as_str) {
        Some("array") => format!(
            "list[{}]",
            schema
                .get("items")
                .map(|items| schema_label(items, depth + 1))
                .unwrap_or_else(|| "value".into())
        ),
        Some("string") => match schema.get("format").and_then(Value::as_str) {
            Some(format) => format!("string({format})"),
            None => "string".into(),
        },
        Some(other) => other.to_string(),
        None if schema.get("properties").is_some() || schema.get("allOf").is_some() => {
            "object".into()
        }
        None => "value".into(),
    }
}

fn render_property(
    schemas: &Value,
    name: &str,
    schema: &Value,
    required: bool,
    depth: usize,
    path: &mut Vec<String>,
    out: &mut Vec<String>,
) {
    let description = schema
        .get("description")
        .and_then(Value::as_str)
        .or_else(|| {
            resolve_schema(schemas, schema)
                .get("description")
                .and_then(Value::as_str)
        })
        .map(|text| compact_summary(text, PROPERTY_DESCRIPTION_CHARS))
        .filter(|text| !text.is_empty());
    out.push(format!(
        "{}- `{name}` ({}{}){}",
        "  ".repeat(depth),
        schema_label(schema, 0),
        if required { ", required" } else { "" },
        description
            .map(|text| format!(": {text}"))
            .unwrap_or_default()
    ));
    if depth >= SCHEMA_MAX_DEPTH {
        return;
    }
    let target = if schema.get("type").and_then(Value::as_str) == Some("array") {
        schema.get("items").unwrap_or(schema)
    } else {
        schema
    };
    let named = component_name(target).map(str::to_string);
    if named.as_ref().is_some_and(|name| path.contains(name)) {
        return;
    }
    let (mut properties, nested_required) = object_properties(schemas, target, 3);
    if properties.is_empty() {
        return;
    }
    if let Some(name) = &named {
        path.push(name.clone());
    }
    let total = properties.len();
    properties.sort_by_key(|(name, _)| !nested_required.contains(name));
    let mut truncated = total > SCHEMA_MAX_NESTED_PROPERTIES;
    for (name, property) in properties.into_iter().take(SCHEMA_MAX_NESTED_PROPERTIES) {
        if out.iter().map(String::len).sum::<usize>() > SCHEMA_BLOCK_BYTES {
            truncated = true;
            break;
        }
        render_property(
            schemas,
            name,
            property,
            nested_required.contains(name),
            depth + 1,
            path,
            out,
        );
    }
    if truncated {
        out.push(format!("{}- … more", "  ".repeat(depth + 1)));
    }
    if named.is_some() {
        path.pop();
    }
}

/// Renders the properties of `schema` as an indented list, required ones first.
/// Required properties are always shown; optional ones stop once the text
/// reaches `cap` bytes. Returns the text and the number of properties left out.
fn render_capped(schemas: &Value, schema: &Value, cap: usize) -> (String, usize) {
    let (mut properties, required) = object_properties(schemas, schema, 3);
    properties.sort_by_key(|(name, _)| !required.contains(name));
    let mut text = String::new();
    let mut omitted = 0;
    for (name, property) in properties {
        let is_required = required.contains(name);
        if !is_required && text.len() >= cap {
            omitted += 1;
            continue;
        }
        let mut lines = Vec::new();
        render_property(
            schemas,
            name,
            property,
            is_required,
            0,
            &mut Vec::new(),
            &mut lines,
        );
        for line in lines {
            text.push_str(&line);
            text.push('\n');
        }
    }
    (text, omitted)
}

/// Markdown for one named component schema, for `<slug>_describe("Name")`.
fn named_schema_markdown(connector: &ConnectorInfo, requested: &str) -> Option<String> {
    let map = connector.schemas.as_object()?;
    let (name, schema) = map.get_key_value(requested).or_else(|| {
        map.iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(requested))
    })?;
    let mut markdown = format!("# {} — type `{name}`\n\n", connector.name);
    if let Some(description) = resolve_schema(&connector.schemas, schema)
        .get("description")
        .and_then(Value::as_str)
    {
        markdown.push_str(&format!(
            "{}\n\n",
            truncate_text(description, OPERATION_DESCRIPTION_CHARS)
        ));
    }
    let (fields, omitted) = render_capped(&connector.schemas, schema, NAMED_SCHEMA_DOC_BYTES);
    if fields.is_empty() {
        markdown.push_str(&format!("`{name}` is {}.\n", schema_label(schema, 0)));
    } else {
        markdown.push_str("## Fields\n\n");
        markdown.push_str(&fields);
        if omitted > 0 {
            markdown.push_str(&format!("\n[{omitted} more optional fields not shown]\n"));
        }
    }
    Some(markdown)
}

pub(crate) fn build_python_functions(
    documents: &[ConnectorDocument],
    network: Arc<dyn SandboxNetwork>,
    filesystem: Arc<dyn FileSystem>,
) -> Vec<Arc<dyn PythonFunction>> {
    let mut documents = documents.to_vec();
    for document in &mut documents {
        normalize_parameters(&mut document.document);
    }
    let documents = documents.as_slice();
    let catalogue = catalogue_normalized(documents);
    let client: Arc<dyn HttpClient> = Arc::new(MediatedHttpClient { network });
    let mut functions = Vec::new();
    for (document, connector) in documents_for_catalogue(documents, &catalogue) {
        let Ok(openapi) = BionicOpenAPI::new(&document.document) else {
            continue;
        };
        let tools = openapi.create_tools_with_http_client(
            &format!("https://{}.connectors.invalid", connector.slug),
            client.clone(),
        );
        let mut tools = tools
            .into_iter()
            .map(|tool| (tool.name(), tool))
            .collect::<Vec<_>>();
        tools.sort_by(|left, right| left.0.cmp(&right.0));
        if !connector.operations.is_empty() {
            let shared = Arc::new(connector.clone());
            for kind in [DiscoveryKind::Find, DiscoveryKind::Describe] {
                functions.push(Arc::new(ConnectorDiscoveryFunction {
                    name: discovery_function_name(&connector.slug, kind.suffix()),
                    kind,
                    connector: shared.clone(),
                }) as Arc<dyn PythonFunction>);
            }
        }
        for operation in &connector.operations {
            let Some((_, tool)) = tools
                .iter()
                .find(|(operation_id, _)| operation_id == &operation.operation_id)
            else {
                continue;
            };
            let (_, file_parameters) = expose_file_parameters(tool.parameters());
            functions.push(Arc::new(ConnectorPythonFunction {
                name: operation.function_name.clone(),
                tool: tool.clone(),
                file_parameters,
                filesystem: filesystem.clone(),
                output_sequence: AtomicUsize::new(0),
            }) as Arc<dyn PythonFunction>);
        }
    }
    functions.sort_by(|left, right| left.name().cmp(right.name()));
    functions
}

fn documents_for_catalogue<'a>(
    documents: &'a [ConnectorDocument],
    catalogue: &'a [ConnectorInfo],
) -> Vec<(&'a ConnectorDocument, &'a ConnectorInfo)> {
    catalogue
        .iter()
        .filter_map(|connector| {
            documents
                .iter()
                .find(|document| document.slug == connector.slug)
                .map(|document| (document, connector))
        })
        .collect()
}

#[derive(Clone, Copy)]
enum DiscoveryKind {
    Find,
    Describe,
}

impl DiscoveryKind {
    fn suffix(self) -> &'static str {
        match self {
            Self::Find => "find",
            Self::Describe => "describe",
        }
    }
}

/// `<slug>_find` and `<slug>_describe`: let the model search a large connector
/// instead of reading a function list that does not fit its context.
struct ConnectorDiscoveryFunction {
    name: String,
    kind: DiscoveryKind,
    connector: Arc<ConnectorInfo>,
}

#[async_trait]
impl PythonFunction for ConnectorDiscoveryFunction {
    fn name(&self) -> &str {
        &self.name
    }

    async fn call(&self, arguments: PythonCallArguments) -> Result<Value, PythonFunctionError> {
        let (key, usage) = match self.kind {
            DiscoveryKind::Find => ("query", "a query string"),
            DiscoveryKind::Describe => ("function", "a function name"),
        };
        let mut arguments = arguments;
        // Allow the natural positional form: find("tickets").
        let positional = match arguments.positional.as_slice() {
            [Value::String(value)] => Some(value.clone()),
            _ => None,
        };
        if let Some(value) = positional {
            arguments.positional.clear();
            arguments.keyword.insert(key.into(), Value::String(value));
        }
        let arguments = merge_arguments(arguments)?;
        let Some(value) = arguments.get(key).and_then(Value::as_str) else {
            return Err(PythonFunctionError(format!(
                "{} requires {usage}",
                self.name
            )));
        };
        match self.kind {
            DiscoveryKind::Find => {
                let limit = arguments
                    .get("limit")
                    .and_then(Value::as_u64)
                    .unwrap_or(FIND_DEFAULT_LIMIT)
                    .clamp(1, FIND_MAX_LIMIT) as usize;
                Ok(Value::String(find_operations(
                    &self.connector,
                    value,
                    limit,
                )))
            }
            DiscoveryKind::Describe => {
                if let Some(operation) = self.connector.operations.iter().find(|operation| {
                    operation.function_name == value || operation.operation_id == value
                }) {
                    return Ok(Value::String(operation_markdown(
                        &self.connector,
                        operation,
                    )));
                }
                named_schema_markdown(&self.connector, value)
                    .map(Value::String)
                    .ok_or_else(|| {
                        PythonFunctionError(format!(
                            "unknown function or type {value:?}; search with {}",
                            discovery_function_name(&self.connector.slug, "find")
                        ))
                    })
            }
        }
    }
}

fn find_operations(connector: &ConnectorInfo, query: &str, limit: usize) -> String {
    let terms = query
        .split_whitespace()
        .map(str::to_lowercase)
        .collect::<Vec<_>>();
    if terms.is_empty() {
        return "Give at least one search word.".into();
    }
    let matches = connector
        .operations
        .iter()
        .filter(|operation| {
            let haystack = format!(
                "{} {} {} {} {} {}",
                operation.function_name,
                operation.operation_id,
                operation.method,
                operation.path,
                operation.tags.join(" "),
                operation.description
            )
            .to_lowercase();
            terms.iter().all(|term| haystack.contains(term))
        })
        .collect::<Vec<_>>();
    if matches.is_empty() {
        return format!("No functions match {query:?}. Try fewer or different words.");
    }
    let mut output = matches
        .iter()
        .take(limit)
        .map(|operation| {
            format!(
                "{}  {} {}  {}\n",
                operation.function_name,
                operation.method,
                operation.path,
                compact_summary(&operation.description, 100)
            )
        })
        .collect::<String>();
    if matches.len() > limit {
        output.push_str(&format!(
            "[{} more matches; add words to narrow the query or raise limit]\n",
            matches.len() - limit
        ));
    }
    output
}

struct ConnectorPythonFunction {
    name: String,
    tool: Arc<dyn ToolDyn>,
    file_parameters: Vec<FileParameterMapping>,
    filesystem: Arc<dyn FileSystem>,
    output_sequence: AtomicUsize,
}

#[async_trait]
impl PythonFunction for ConnectorPythonFunction {
    fn name(&self) -> &str {
        &self.name
    }

    async fn call(&self, arguments: PythonCallArguments) -> Result<Value, PythonFunctionError> {
        let mut arguments = merge_arguments(arguments)?;
        materialize_file_parameters(
            &mut arguments,
            &self.file_parameters,
            self.filesystem.as_ref(),
        )
        .await?;
        let result = self
            .tool
            .call(arguments.to_string())
            .await
            .map_err(|error| PythonFunctionError(tool_error(error)))?;
        let value: Value = serde_json::from_str(&result).unwrap_or(Value::String(result));
        let sequence = self.output_sequence.fetch_add(1, Ordering::Relaxed);
        persist_binary_response(&self.name, sequence, value, self.filesystem.as_ref()).await
    }
}

fn merge_arguments(arguments: PythonCallArguments) -> Result<Value, PythonFunctionError> {
    if arguments.positional.len() > 1 {
        return Err(PythonFunctionError(
            "connector functions accept at most one positional dictionary".into(),
        ));
    }
    let mut object = match arguments.positional.into_iter().next() {
        Some(Value::Object(object)) => object,
        Some(_) => {
            return Err(PythonFunctionError(
                "the positional connector argument must be a dictionary".into(),
            ))
        }
        None => Map::new(),
    };
    object.extend(arguments.keyword);
    Ok(Value::Object(object))
}

#[derive(Clone)]
struct FileParameterMapping {
    api_parameter: String,
    path_parameter: String,
    multiple: bool,
    required: bool,
}

fn expose_file_parameters(mut parameters: Value) -> (Value, Vec<FileParameterMapping>) {
    let required_parameters = parameters
        .get("required")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_string)
        .collect::<HashSet<_>>();
    let Some(properties) = parameters
        .get_mut("properties")
        .and_then(Value::as_object_mut)
    else {
        return (parameters, Vec::new());
    };
    let file_parameters = properties
        .iter()
        .filter_map(|(name, schema)| {
            let multiple = schema.get("type").and_then(Value::as_str) == Some("array");
            let format = if multiple {
                schema
                    .get("items")
                    .and_then(|items| items.get("format"))
                    .and_then(Value::as_str)
            } else {
                schema.get("format").and_then(Value::as_str)
            };
            matches!(format, Some("byte" | "binary")).then_some((name.clone(), multiple))
        })
        .collect::<Vec<_>>();
    let mut mappings = Vec::new();
    for (index, (api_parameter, multiple)) in file_parameters.into_iter().enumerate() {
        let path_parameter = match (index, multiple) {
            (0, false) => "file_path".to_string(),
            (0, true) => "file_paths".to_string(),
            (_, false) => format!("{api_parameter}_file_path"),
            (_, true) => format!("{api_parameter}_file_paths"),
        };
        properties.remove(&api_parameter);
        properties.insert(
            path_parameter.clone(),
            if multiple {
                json!({"type":"array","items":{"type":"string"},"description":"VFS paths under /home/user/attachments, /home/user/work, or /home/user/output."})
            } else {
                json!({"type":"string","description":"A VFS path under /home/user/attachments, /home/user/work, or /home/user/output."})
            },
        );
        mappings.push(FileParameterMapping {
            required: required_parameters.contains(&api_parameter),
            api_parameter,
            path_parameter,
            multiple,
        });
    }
    if let Some(required) = parameters.get_mut("required").and_then(Value::as_array_mut) {
        for required_name in required {
            if let Some(mapping) = mappings
                .iter()
                .find(|mapping| required_name.as_str() == Some(&mapping.api_parameter))
            {
                *required_name = Value::String(mapping.path_parameter.clone());
            }
        }
    }
    (parameters, mappings)
}

async fn materialize_file_parameters(
    arguments: &mut Value,
    mappings: &[FileParameterMapping],
    filesystem: &dyn FileSystem,
) -> Result<(), PythonFunctionError> {
    for mapping in mappings {
        let Some(raw_value) = arguments.get(&mapping.path_parameter) else {
            if mapping.required {
                return Err(PythonFunctionError(format!(
                    "{} is required",
                    mapping.path_parameter
                )));
            }
            continue;
        };
        let values = if mapping.multiple {
            raw_value
                .as_array()
                .ok_or_else(|| {
                    PythonFunctionError(format!(
                        "{} must be a list of VFS paths",
                        mapping.path_parameter
                    ))
                })?
                .iter()
                .map(|value| {
                    value.as_str().map(str::to_string).ok_or_else(|| {
                        PythonFunctionError(format!(
                            "{} entries must be strings",
                            mapping.path_parameter
                        ))
                    })
                })
                .collect::<Result<Vec<_>, _>>()?
        } else {
            vec![raw_value
                .as_str()
                .ok_or_else(|| {
                    PythonFunctionError(format!("{} must be a VFS path", mapping.path_parameter))
                })?
                .to_string()]
        };
        let mut files = Vec::new();
        for path in values {
            if !allowed_file_path(&path) {
                return Err(PythonFunctionError(format!(
                    "{path} must be under /home/user/attachments, /home/user/work, or /home/user/output"
                )));
            }
            let bytes = filesystem
                .read_file(Path::new(&path))
                .await
                .map_err(|error| PythonFunctionError(format!("failed to read {path}: {error}")))?;
            let filename = Path::new(&path)
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| PythonFunctionError(format!("invalid file path: {path}")))?;
            files.push(json!({
                "__bionic_file": true,
                "filename": filename,
                "content_base64": base64::engine::general_purpose::STANDARD.encode(bytes),
            }));
        }
        arguments[&mapping.api_parameter] = if mapping.multiple {
            Value::Array(files)
        } else {
            files.into_iter().next().unwrap_or(Value::Null)
        };
        arguments
            .as_object_mut()
            .expect("connector arguments are an object")
            .remove(&mapping.path_parameter);
    }
    Ok(())
}

fn allowed_file_path(path: &str) -> bool {
    [
        "/home/user/attachments/",
        "/home/user/work/",
        "/home/user/output/",
    ]
    .iter()
    .any(|prefix| path.starts_with(prefix))
}

async fn persist_binary_response(
    function_name: &str,
    sequence: usize,
    mut value: Value,
    filesystem: &dyn FileSystem,
) -> Result<Value, PythonFunctionError> {
    if value.get("__bionic_binary") != Some(&Value::Bool(true)) {
        return Ok(value);
    }
    let encoded = value
        .get("content_base64")
        .and_then(Value::as_str)
        .ok_or_else(|| PythonFunctionError("binary response is missing content".into()))?;
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .map_err(|error| PythonFunctionError(format!("invalid binary response: {error}")))?;
    let content_type = value
        .get("content_type")
        .and_then(Value::as_str)
        .unwrap_or("application/octet-stream");
    let extension = if content_type == "application/pdf" {
        "pdf"
    } else {
        "bin"
    };
    let directory = format!("/home/user/output/{function_name}");
    let filename = if sequence == 0 {
        format!("output.{extension}")
    } else {
        format!("output-{}.{extension}", sequence + 1)
    };
    let path = format!("{directory}/{filename}");
    filesystem
        .mkdir(Path::new(&directory), true)
        .await
        .map_err(|error| PythonFunctionError(error.to_string()))?;
    filesystem
        .write_file(Path::new(&path), &bytes)
        .await
        .map_err(|error| PythonFunctionError(error.to_string()))?;
    if let Some(object) = value.as_object_mut() {
        object.remove("content_base64");
        object.insert("path".into(), Value::String(path));
        object.insert("size".into(), json!(bytes.len()));
    }
    Ok(value)
}

fn tool_error(error: ToolError) -> String {
    match error {
        ToolError::JsonError(error) => error.to_string(),
        ToolError::ToolCallError(error) => error.to_string(),
    }
}

struct MediatedHttpClient {
    network: Arc<dyn SandboxNetwork>,
}

#[async_trait]
impl HttpClient for MediatedHttpClient {
    async fn send(
        &self,
        method: Method,
        url: Url,
        headers: Vec<(String, String)>,
        body: Option<HttpRequestBody>,
    ) -> Result<ToolHttpResponse, String> {
        let method = method_to_sandbox(method)?;
        let mut headers = headers
            .into_iter()
            .map(|(name, value)| HttpHeader { name, value })
            .collect::<Vec<_>>();
        let body = match body {
            Some(HttpRequestBody::Json(value)) => {
                headers.push(HttpHeader {
                    name: "Content-Type".into(),
                    value: "application/json".into(),
                });
                Some(serde_json::to_vec(&value).map_err(|error| error.to_string())?)
            }
            Some(HttpRequestBody::Form(fields)) => {
                headers.push(HttpHeader {
                    name: "Content-Type".into(),
                    value: "application/x-www-form-urlencoded".into(),
                });
                Some(encode_form(&fields).into_bytes())
            }
            Some(HttpRequestBody::Multipart { fields, files }) => {
                let boundary = multipart_boundary(&fields, &files);
                headers.push(HttpHeader {
                    name: "Content-Type".into(),
                    value: format!("multipart/form-data; boundary={boundary}"),
                });
                Some(encode_multipart(&boundary, fields, files))
            }
            None => None,
        };
        let response = self
            .network
            .request(HttpRequest {
                method,
                url: url.to_string(),
                headers,
                body,
                timeout: REQUEST_TIMEOUT,
                connect_timeout: Some(Duration::from_secs(10)),
                max_response_bytes: MAX_RESPONSE_BYTES,
            })
            .await
            .map_err(|error| error.to_string())?;
        let content_type = response
            .headers
            .iter()
            .find(|header| header.name.eq_ignore_ascii_case("content-type"))
            .map(|header| header.value.clone());
        Ok(ToolHttpResponse {
            status: StatusCode::from_u16(response.status).map_err(|error| error.to_string())?,
            body: response.body,
            content_type,
        })
    }
}

fn method_to_sandbox(method: Method) -> Result<HttpMethod, String> {
    match method {
        Method::GET => Ok(HttpMethod::Get),
        Method::POST => Ok(HttpMethod::Post),
        Method::PUT => Ok(HttpMethod::Put),
        Method::DELETE => Ok(HttpMethod::Delete),
        Method::HEAD => Ok(HttpMethod::Head),
        Method::PATCH => Ok(HttpMethod::Patch),
        _ => Err(format!("unsupported connector HTTP method: {method}")),
    }
}

fn encode_form(fields: &[(String, String)]) -> String {
    let mut url = Url::parse("http://form.invalid/").expect("static URL");
    url.query_pairs_mut().extend_pairs(fields);
    url.query().unwrap_or_default().to_string()
}

fn encode_multipart(
    boundary: &str,
    fields: Vec<(String, String)>,
    files: Vec<(String, String, Vec<u8>)>,
) -> Vec<u8> {
    let mut body = Vec::new();
    for (name, value) in fields {
        let name = multipart_quoted(&name);
        body.extend_from_slice(format!("--{boundary}\r\nContent-Disposition: form-data; name=\"{name}\"\r\n\r\n{value}\r\n").as_bytes());
    }
    for (name, filename, bytes) in files {
        let name = multipart_quoted(&name);
        let filename = multipart_quoted(&filename);
        body.extend_from_slice(format!("--{boundary}\r\nContent-Disposition: form-data; name=\"{name}\"; filename=\"{filename}\"\r\nContent-Type: application/octet-stream\r\n\r\n").as_bytes());
        body.extend_from_slice(&bytes);
        body.extend_from_slice(b"\r\n");
    }
    body.extend_from_slice(format!("--{boundary}--\r\n").as_bytes());
    body
}

fn multipart_boundary(fields: &[(String, String)], files: &[(String, String, Vec<u8>)]) -> String {
    let mut contents = Vec::new();
    for (name, value) in fields {
        contents.extend_from_slice(name.as_bytes());
        contents.extend_from_slice(value.as_bytes());
    }
    for (name, filename, bytes) in files {
        contents.extend_from_slice(name.as_bytes());
        contents.extend_from_slice(filename.as_bytes());
        contents.extend_from_slice(bytes);
    }
    let digest = md5::compute(contents);
    format!("bionic-{digest:x}")
}

fn multipart_quoted(value: &str) -> String {
    value
        .replace('\r', "%0D")
        .replace('\n', "%0A")
        .replace('"', "%22")
}

fn operation_function_name(slug: &str, operation_id: &str, used: &mut HashSet<String>) -> String {
    let prefix = sanitize_identifier(slug);
    let operation = sanitize_identifier(operation_id);
    let operation = operation
        .strip_prefix(&format!("{prefix}_"))
        .unwrap_or(&operation);
    unique_identifier(&format!("{prefix}_{operation}"), used)
}

fn unique_identifier(value: &str, used: &mut HashSet<String>) -> String {
    let base = sanitize_identifier(value);
    let mut candidate = base.clone();
    let mut suffix = 2;
    while !used.insert(candidate.clone()) {
        candidate = format!("{base}_{suffix}");
        suffix += 1;
    }
    candidate
}

fn sanitize_identifier(value: &str) -> String {
    let mut output = String::new();
    let characters = value.chars().collect::<Vec<_>>();
    for (index, character) in characters.iter().copied().enumerate() {
        if character.is_ascii_alphanumeric() || character == '_' {
            if character.is_ascii_uppercase()
                && !output.ends_with('_')
                && index > 0
                && (characters[index - 1].is_ascii_lowercase()
                    || characters
                        .get(index + 1)
                        .is_some_and(|next| next.is_ascii_lowercase()))
            {
                output.push('_');
            }
            output.push(character.to_ascii_lowercase());
        } else if !output.ends_with('_') {
            output.push('_');
        }
    }
    let mut output = output.trim_matches('_').to_string();
    if output.is_empty() {
        output = "function".into();
    }
    if output.starts_with(|character: char| character.is_ascii_digit())
        || is_python_keyword(&output)
    {
        output.insert(0, '_');
    }
    output
}

fn is_python_keyword(value: &str) -> bool {
    matches!(
        value,
        "false"
            | "none"
            | "true"
            | "and"
            | "as"
            | "assert"
            | "async"
            | "await"
            | "break"
            | "class"
            | "continue"
            | "def"
            | "del"
            | "elif"
            | "else"
            | "except"
            | "finally"
            | "for"
            | "from"
            | "global"
            | "if"
            | "import"
            | "in"
            | "is"
            | "lambda"
            | "nonlocal"
            | "not"
            | "or"
            | "pass"
            | "raise"
            | "return"
            | "try"
            | "while"
            | "with"
            | "yield"
    )
}

fn example_arguments(parameters: &Value) -> String {
    let required = parameters
        .get("required")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .collect::<Vec<_>>();
    if required.iter().all(|name| is_python_identifier(name)) {
        required
            .into_iter()
            .map(|name| format!("{name}=..."))
            .collect::<Vec<_>>()
            .join(", ")
    } else {
        format!(
            "{{{}}}",
            required
                .into_iter()
                .map(|name| format!("{name:?}: ..."))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

fn is_python_identifier(value: &str) -> bool {
    let mut characters = value.chars();
    characters
        .next()
        .is_some_and(|first| first == '_' || first.is_ascii_alphabetic())
        && characters.all(|character| character == '_' || character.is_ascii_alphanumeric())
        && !is_python_keyword(&value.to_ascii_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;
    use bashkit::InMemoryFs;
    use std::sync::Mutex;

    #[derive(Default)]
    struct RecordingNetwork(Mutex<Vec<HttpRequest>>);

    #[async_trait]
    impl SandboxNetwork for RecordingNetwork {
        async fn request(
            &self,
            request: HttpRequest,
        ) -> Result<sandbox::HttpResponse, sandbox::NetworkError> {
            self.0.lock().unwrap().push(request);
            Ok(sandbox::HttpResponse {
                status: 200,
                headers: vec![HttpHeader {
                    name: "Content-Type".into(),
                    value: "application/json".into(),
                }],
                body: br#"{"records":[{"Id":"001"}]}"#.to_vec(),
            })
        }
    }

    #[test]
    fn operation_names_strip_duplicate_connector_prefix() {
        let mut used = HashSet::new();
        assert_eq!(
            operation_function_name("salesforce", "salesforce.sobjects.lead.create", &mut used),
            "salesforce_sobjects_lead_create"
        );
        assert_eq!(
            operation_function_name("salesforce", "salesforce.query", &mut used),
            "salesforce_query"
        );
        assert_eq!(
            operation_function_name("typst", "compileDocument", &mut used),
            "typst_compile_document"
        );
    }

    #[test]
    fn file_parameters_are_exposed_as_vfs_paths() {
        let (parameters, mappings) = expose_file_parameters(json!({
            "type":"object",
            "properties":{"file":{"type":"string","format":"binary"}},
            "required":["file"]
        }));
        assert_eq!(parameters["required"], json!(["file_path"]));
        assert!(parameters["properties"].get("file").is_none());
        assert_eq!(mappings[0].path_parameter, "file_path");
        assert!(mappings[0].required);
    }

    #[tokio::test]
    async fn optional_file_parameters_may_be_omitted() {
        let (_, mappings) = expose_file_parameters(json!({
            "type":"object",
            "properties":{"file":{"type":"string","format":"binary"}}
        }));
        let mut arguments = json!({});
        materialize_file_parameters(&mut arguments, &mappings, &InMemoryFs::new())
            .await
            .unwrap();
        assert_eq!(arguments, json!({}));
    }

    #[test]
    fn examples_use_a_dictionary_for_non_identifier_parameters() {
        assert_eq!(
            example_arguments(&json!({"required":["filter[name]"]})),
            r#"{"filter[name]": ...}"#
        );
    }

    #[tokio::test]
    async fn generated_function_uses_the_mediated_virtual_origin() {
        let document = salesforce_document();
        let network = Arc::new(RecordingNetwork::default());
        let functions =
            build_python_functions(&[document], network.clone(), Arc::new(InMemoryFs::new()));
        let function = functions
            .iter()
            .find(|function| function.name() == "salesforce_query")
            .unwrap();
        let result = function
            .call(PythonCallArguments {
                positional: Vec::new(),
                keyword: Map::from_iter([("q".into(), Value::String("SELECT Id".into()))]),
            })
            .await
            .unwrap();
        assert_eq!(result["records"][0]["Id"], "001");
        let requests = network.0.lock().unwrap();
        assert_eq!(
            requests[0].url,
            "https://salesforce.connectors.invalid/query?q=SELECT+Id"
        );
    }

    #[test]
    fn generated_skill_links_to_operation_docs_without_exposing_openapi() {
        let connectors = catalogue(&[salesforce_document()]);
        let skill = skill_markdown(&connectors[0]);
        assert!(skill.contains("`salesforce_query`"));
        assert!(skill.contains("operations/salesforce_query.md"));
        assert!(!skill.contains("openapi.json"));
    }

    #[test]
    fn header_parameters_are_exposed_as_arguments() {
        let mut document = salesforce_document();
        document.document["paths"]["/query"]["get"]["parameters"] = json!([{
            "name":"X-Custom",
            "in":"header",
            "schema":{"type":"string"}
        }]);
        let connectors = catalogue(&[document]);
        assert_eq!(connectors[0].operations.len(), 1);
        assert!(connectors[0].operations[0].parameters["properties"]
            .get("X-Custom")
            .is_some());
    }

    #[test]
    fn unsupported_cookie_parameter_operations_are_not_exposed() {
        let mut document = salesforce_document();
        document.document["paths"]["/query"]["get"]["parameters"] = json!([{
            "name":"session",
            "in":"cookie",
            "schema":{"type":"string"}
        }]);
        assert!(catalogue(&[document])[0].operations.is_empty());
    }

    fn header_document() -> ConnectorDocument {
        let mut document = salesforce_document();
        document.document["components"] = json!({"securitySchemes":{
            "key":{"type":"apiKey","in":"header","name":"X-API-Key"}
        }});
        document.document["paths"]["/query"]["get"]["parameters"] = json!([
            {"name":"q","in":"query","schema":{"type":"string"}},
            {"name":"clientId","in":"header","required":true,"schema":{"type":"string"}},
            {"name":"Api-Version","in":"header","schema":{"type":"string","default":"2022-06-28"}},
            {"name":"X-API-Key","in":"header","required":true,"schema":{"type":"string"}},
            {"name":"Accept","in":"header","schema":{"type":"string"}},
            {"name":"Authorization","in":"header","schema":{"type":"string"}}
        ]);
        document
    }

    #[test]
    fn runtime_owned_headers_are_not_exposed() {
        let connectors = catalogue(&[header_document()]);
        let properties = connectors[0].operations[0].parameters["properties"]
            .as_object()
            .unwrap();
        let mut names = properties.keys().map(String::as_str).collect::<Vec<_>>();
        names.sort();
        assert_eq!(names, ["Api-Version", "clientId", "q"]);
    }

    #[tokio::test]
    async fn header_arguments_are_sent_as_headers_not_query_or_body() {
        let network = Arc::new(RecordingNetwork::default());
        let functions = build_python_functions(
            &[header_document()],
            network.clone(),
            Arc::new(InMemoryFs::new()),
        );
        functions
            .iter()
            .find(|function| function.name() == "salesforce_query")
            .unwrap()
            .call(PythonCallArguments {
                positional: Vec::new(),
                keyword: Map::from_iter([
                    ("q".into(), json!("a")),
                    ("clientId".into(), json!("guid-1")),
                ]),
            })
            .await
            .unwrap();
        let requests = network.0.lock().unwrap();
        assert_eq!(
            requests[0].url,
            "https://salesforce.connectors.invalid/query?q=a"
        );
        assert!(requests[0].body.is_none());
        let header = |name: &str| {
            requests[0]
                .headers
                .iter()
                .find(|header| header.name == name)
                .map(|header| header.value.as_str())
        };
        assert_eq!(header("clientId"), Some("guid-1"));
        // Not supplied, so the schema default is used.
        assert_eq!(header("Api-Version"), Some("2022-06-28"));
    }

    #[test]
    fn every_curated_json_spec_gets_a_skill_within_budget() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../bionic-gpt/content/docs/integrations/specs");
        let mut checked = 0;
        let mut unparsed = Vec::new();
        for entry in std::fs::read_dir(&dir).unwrap() {
            let path = entry.unwrap().path();
            // Connectors are stored as JSON documents; nothing in the runtime
            // loads the YAML specs in this folder, so they are out of scope.
            if path.extension().and_then(|value| value.to_str()) != Some("json") {
                continue;
            }
            let document = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
            let slug = path.file_stem().unwrap().to_string_lossy().to_string();
            let connectors = catalogue(&[ConnectorDocument {
                name: slug.clone(),
                slug: slug.clone(),
                is_builtin: false,
                document,
            }]);
            let Some(connector) = connectors.first() else {
                unparsed.push(slug);
                continue;
            };
            let skill = skill_markdown(connector);
            assert!(
                skill.len() < SKILL_FULL_LIST_BYTES + SKILL_DESCRIPTION_CHARS * 4 + 2048,
                "{slug}: SKILL.md is {} bytes",
                skill.len()
            );
            checked += 1;
        }
        assert!(
            unparsed.is_empty(),
            "specs that did not parse: {unparsed:?}"
        );
        assert!(checked > 0, "no JSON specs found in {}", dir.display());
    }

    fn ticket_document() -> ConnectorDocument {
        ConnectorDocument {
            name: "Tickets".into(),
            slug: "tickets".into(),
            is_builtin: false,
            document: json!({
                "openapi":"3.0.0",
                "info":{"title":"Tickets","version":"1"},
                "servers":[{"url":"https://upstream.example"}],
                "paths":{"/tickets":{"post":{
                    "operationId":"createTicket",
                    "requestBody":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/Ticket"}}}},
                    "responses":{"200":{"description":"ok"}}
                }}},
                "components":{"schemas":{
                    "Ticket":{
                        "type":"object",
                        "required":["summary","company"],
                        "properties":{
                            "note":{"type":"string","description":"Free text"},
                            "status":{"type":"string","enum":["open","closed"]},
                            "created":{"type":"string","format":"date-time"},
                            "summary":{"type":"string","description":"One line summary"},
                            "company":{"$ref":"#/components/schemas/Company"},
                            "children":{"type":"array","items":{"$ref":"#/components/schemas/Ticket"}}
                        }
                    },
                    "Company":{
                        "allOf":[
                            {"$ref":"#/components/schemas/Base"},
                            {"type":"object","required":["name"],"properties":{"name":{"type":"string"}}}
                        ]
                    },
                    "Base":{"type":"object","properties":{"id":{"type":"integer"}}}
                }}
            }),
        }
    }

    fn ticket_markdown() -> String {
        let connectors = catalogue(&[ticket_document()]);
        operation_markdown(&connectors[0], &connectors[0].operations[0])
    }

    #[test]
    fn operation_files_render_nested_refs_by_name_and_expand_them() {
        let markdown = ticket_markdown();
        assert!(markdown.contains("(POST /tickets)"), "{markdown}");
        assert!(markdown.contains("- `summary` (string, required): One line summary"));
        assert!(markdown.contains("- `company` (Company, required)"));
        // Company is expanded one level, with allOf merged and required first.
        assert!(
            markdown.contains("  - `name` (string, required)"),
            "{markdown}"
        );
        assert!(markdown.contains("  - `id` (integer)"));
        assert!(markdown.contains("(enum(open|closed))"));
        assert!(markdown.contains("(string(date-time))"));
        assert!(markdown.contains("(list[Ticket])"));
        assert!(!markdown.contains("$ref"));
        assert!(!markdown.contains("Parameter schema"));
        assert!(markdown.contains("tickets_describe(\"Ticket\")"));
    }

    #[test]
    fn required_arguments_are_listed_before_optional_ones() {
        let markdown = ticket_markdown();
        let summary = markdown.find("- `summary`").unwrap();
        let company = markdown.find("- `company`").unwrap();
        let note = markdown.find("- `note`").unwrap();
        assert!(summary < note && company < note);
    }

    #[test]
    fn self_referencing_types_do_not_recurse_forever() {
        let markdown = ticket_markdown();
        let children = markdown.find("- `children`").unwrap();
        // children -> Ticket is expanded once, and its own children stop there.
        assert!(markdown[children..].matches("`children`").count() <= 2);
    }

    #[test]
    fn operation_files_stay_near_the_size_cap_but_keep_required_arguments() {
        let mut properties = Map::new();
        for index in 0..400 {
            properties.insert(
                format!("optional{index:03}"),
                json!({"type":"string","description":"x".repeat(100)}),
            );
        }
        properties.insert("must".into(), json!({"type":"string"}));
        let mut document = ticket_document();
        document.document["components"]["schemas"]["Ticket"] =
            json!({"type":"object","required":["must"],"properties":properties});
        let connectors = catalogue(&[document]);
        let markdown = operation_markdown(&connectors[0], &connectors[0].operations[0]);
        assert!(
            markdown.len() < OPERATION_DOC_BYTES + 2048,
            "{}",
            markdown.len()
        );
        assert!(markdown.contains("- `must` (string, required)"));
        assert!(markdown.contains("more optional arguments not shown"));
    }

    #[tokio::test]
    async fn describe_expands_a_named_type() {
        let functions = build_python_functions(
            &[ticket_document()],
            Arc::new(RecordingNetwork::default()),
            Arc::new(InMemoryFs::new()),
        );
        let describe = functions
            .iter()
            .find(|function| function.name() == "tickets_describe")
            .unwrap();
        let result = describe
            .call(PythonCallArguments {
                positional: vec![json!("company")],
                keyword: Map::new(),
            })
            .await
            .unwrap();
        let text = result.as_str().unwrap();
        assert!(text.contains("type `Company`"), "{text}");
        assert!(text.contains("- `name` (string, required)"));
        let unknown = describe
            .call(PythonCallArguments {
                positional: vec![json!("Nothing")],
                keyword: Map::new(),
            })
            .await;
        assert!(unknown.unwrap_err().0.contains("tickets_find"));
    }

    #[test]
    fn every_curated_json_spec_has_bounded_operation_files() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../bionic-gpt/content/docs/integrations/specs");
        let mut largest = (0, String::new());
        for entry in std::fs::read_dir(&dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().and_then(|value| value.to_str()) != Some("json") {
                continue;
            }
            let document = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
            let slug = path.file_stem().unwrap().to_string_lossy().to_string();
            for connector in catalogue(&[ConnectorDocument {
                name: slug.clone(),
                slug: slug.clone(),
                is_builtin: false,
                document,
            }]) {
                for operation in &connector.operations {
                    let size = operation_markdown(&connector, operation).len();
                    if size > largest.0 {
                        largest = (size, format!("{slug}/{}", operation.operation_id));
                    }
                }
            }
        }
        eprintln!(
            "largest operation file: {} bytes ({})",
            largest.0, largest.1
        );
        assert!(largest.0 < 16 * 1024, "{} bytes: {}", largest.0, largest.1);
    }

    fn parameter_document() -> ConnectorDocument {
        ConnectorDocument {
            name: "Items".into(),
            slug: "items".into(),
            is_builtin: false,
            document: json!({
                "openapi":"3.0.0",
                "info":{"title":"Items","version":"1"},
                "servers":[{"url":"https://upstream.example"}],
                "paths":{
                    "/items/{id}":{
                        "parameters":[
                            {"name":"id","in":"path","required":true,"schema":{"type":"integer"}},
                            {"$ref":"#/components/parameters/Verbose"}
                        ],
                        "get":{"operationId":"getItem","responses":{"200":{"description":"ok"}}},
                        "delete":{
                            "operationId":"deleteItem",
                            "parameters":[{"name":"verbose","in":"query","schema":{"type":"string"}}],
                            "responses":{"200":{"description":"ok"}}
                        }
                    },
                    "/charges":{"post":{
                        "operationId":"createCharge",
                        "requestBody":{"content":{"application/x-www-form-urlencoded":{"schema":{
                            "type":"object",
                            "required":["customer"],
                            "properties":{
                                "customer":{"type":"string"},
                                "metadata":{"type":"object"},
                                "items":{"type":"array","items":{"type":"object"}}
                            }
                        }}}},
                        "responses":{"200":{"description":"ok"}}
                    }}
                },
                "components":{"parameters":{
                    "Verbose":{"name":"verbose","in":"query","schema":{"type":"boolean"}}
                }}
            }),
        }
    }

    #[test]
    fn path_level_and_referenced_parameters_are_inherited_by_operations() {
        let connectors = catalogue(&[parameter_document()]);
        let names = connectors[0]
            .operations
            .iter()
            .map(|operation| operation.operation_id.as_str())
            .collect::<Vec<_>>();
        assert_eq!(names, ["createCharge", "deleteItem", "getItem"]);
        let get = connectors[0]
            .operations
            .iter()
            .find(|operation| operation.operation_id == "getItem")
            .unwrap();
        assert_eq!(get.parameters["required"], json!(["id"]));
        assert_eq!(get.parameters["properties"]["verbose"]["type"], "boolean");
        // The operation's own parameter overrides the path-level reference.
        let delete = connectors[0]
            .operations
            .iter()
            .find(|operation| operation.operation_id == "deleteItem")
            .unwrap();
        assert_eq!(delete.parameters["properties"]["verbose"]["type"], "string");
    }

    #[test]
    fn operation_names_never_take_another_connectors_discovery_names() {
        let document = |slug: &str, operation_id: &str| ConnectorDocument {
            name: slug.into(),
            slug: slug.into(),
            is_builtin: false,
            document: json!({
                "openapi":"3.0.0",
                "info":{"title":slug,"version":"1"},
                "paths":{"/x":{"get":{
                    "operationId":operation_id,
                    "responses":{"200":{"description":"ok"}}
                }}}
            }),
        };
        // "a" sorts first, and its operation would become "a_b_find".
        let connectors = catalogue(&[document("a_b", "list"), document("a", "b_find")]);
        let names = connectors
            .iter()
            .flat_map(|connector| &connector.operations)
            .map(|operation| operation.function_name.as_str())
            .collect::<Vec<_>>();
        assert!(!names.contains(&"a_b_find"), "{names:?}");
        assert!(!names.contains(&"a_b_describe"), "{names:?}");
    }

    #[test]
    fn normalizing_parameters_twice_changes_nothing() {
        let mut once = parameter_document().document;
        normalize_parameters(&mut once);
        let mut twice = once.clone();
        normalize_parameters(&mut twice);
        assert_eq!(once, twice);
        assert!(once["paths"]["/items/{id}"].get("parameters").is_none());
    }

    #[tokio::test]
    async fn path_level_and_referenced_parameters_are_sent() {
        let network = Arc::new(RecordingNetwork::default());
        let functions = build_python_functions(
            &[parameter_document()],
            network.clone(),
            Arc::new(InMemoryFs::new()),
        );
        functions
            .iter()
            .find(|function| function.name() == "items_get_item")
            .unwrap()
            .call(PythonCallArguments {
                positional: Vec::new(),
                keyword: Map::from_iter([("id".into(), json!(5)), ("verbose".into(), json!(true))]),
            })
            .await
            .unwrap();
        assert_eq!(
            network.0.lock().unwrap()[0].url,
            "https://items.connectors.invalid/items/5?verbose=true"
        );
    }

    #[tokio::test]
    async fn form_urlencoded_bodies_are_flattened_and_sent() {
        let network = Arc::new(RecordingNetwork::default());
        let functions = build_python_functions(
            &[parameter_document()],
            network.clone(),
            Arc::new(InMemoryFs::new()),
        );
        functions
            .iter()
            .find(|function| function.name() == "items_create_charge")
            .unwrap()
            .call(PythonCallArguments {
                positional: Vec::new(),
                keyword: Map::from_iter([
                    ("customer".into(), json!("cus 1")),
                    ("metadata".into(), json!({"k": "v&w"})),
                    ("items".into(), json!([{"price": "p"}])),
                ]),
            })
            .await
            .unwrap();
        let requests = network.0.lock().unwrap();
        assert!(requests[0].headers.iter().any(|header| {
            header.name == "Content-Type" && header.value == "application/x-www-form-urlencoded"
        }));
        let body = String::from_utf8(requests[0].body.clone().unwrap()).unwrap();
        let mut pairs = Url::parse(&format!("http://x/?{body}"))
            .unwrap()
            .query_pairs()
            .map(|(name, value)| (name.to_string(), value.to_string()))
            .collect::<Vec<_>>();
        pairs.sort();
        assert_eq!(
            pairs,
            [
                ("customer".to_string(), "cus 1".to_string()),
                ("items[0][price]".to_string(), "p".to_string()),
                ("metadata[k]".to_string(), "v&w".to_string()),
            ]
        );
    }

    #[test]
    fn curated_specs_expose_operations_with_path_level_parameters_and_forms() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../bionic-gpt/content/docs/integrations/specs");
        let (mut total, mut exposed) = (0, 0);
        let mut by_spec = std::collections::BTreeMap::new();
        for entry in std::fs::read_dir(&dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().and_then(|value| value.to_str()) != Some("json") {
                continue;
            }
            let mut document: Value =
                serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
            normalize_parameters(&mut document);
            let support = supported_operations(&document);
            let supported = support.values().filter(|reason| reason.is_none()).count();
            total += support.len();
            exposed += supported;
            by_spec.insert(
                path.file_stem().unwrap().to_string_lossy().to_string(),
                (supported, support.len()),
            );
        }
        eprintln!("exposed {exposed} of {total}: {by_spec:?}");
        // Google and Stripe were almost entirely dropped before; only cookie parameters and unusual body types are still unsupported.
        for (spec, minimum) in [
            ("google-gmail", 73),
            ("google-drive", 63),
            ("stripe", 600),
            ("zendesk", 600),
        ] {
            assert!(by_spec[spec].0 >= minimum, "{spec}: {:?}", by_spec[spec]);
        }
    }

    fn large_document(operations: usize) -> ConnectorDocument {
        let mut paths = Map::new();
        for index in 0..operations {
            let tag = format!("group{}", index % 5);
            paths.insert(
                format!("/{tag}/items{index}"),
                json!({"get":{
                    "operationId": format!("listItems{index}"),
                    "tags":[tag],
                    "summary": format!("List the widgets of kind {index}, a reasonably long summary to take up space in the list"),
                    "responses":{"200":{"description":"ok"}}
                }}),
            );
        }
        ConnectorDocument {
            name: "Big".into(),
            slug: "big".into(),
            is_builtin: false,
            document: json!({
                "openapi":"3.0.0",
                "info":{"title":"Big","version":"1","description":"Big API. Use group filters."},
                "servers":[{"url":"https://upstream.example"}],
                "paths": paths
            }),
        }
    }

    #[test]
    fn small_connectors_keep_the_full_function_list() {
        let connectors = catalogue(&[salesforce_document()]);
        let skill = skill_markdown(&connectors[0]);
        assert!(skill.contains("- `salesforce_query`"));
        assert!(skill.contains("salesforce_find(\"keyword\")"));
        assert!(!skill.contains("## Groups"));
    }

    #[test]
    fn large_connectors_get_a_grouped_overview_under_budget() {
        let connectors = catalogue(&[large_document(400)]);
        let skill = skill_markdown(&connectors[0]);
        assert!(skill.len() < SKILL_FULL_LIST_BYTES, "{} bytes", skill.len());
        assert!(skill.contains("400 functions"));
        assert!(skill.contains("big_find(\"keyword\")"));
        assert!(skill.contains("- `group0`: 80"));
        assert!(!skill.contains("big_list_items7`"));
        assert!(skill.contains("Big API. Use group filters."));
    }

    #[test]
    fn long_info_descriptions_are_truncated_in_the_skill() {
        let mut document = salesforce_document();
        document.document["info"]["description"] = json!("word ".repeat(1000));
        let connectors = catalogue(&[document]);
        let skill = skill_markdown(&connectors[0]);
        assert!(skill.contains("## About"));
        assert!(skill.len() < SKILL_DESCRIPTION_CHARS + 1500);
        assert!(skill.contains('…'));
    }

    #[test]
    fn groups_fall_back_to_the_first_path_segment_without_tags() {
        let mut document = large_document(300);
        for path_item in document.document["paths"]
            .as_object_mut()
            .unwrap()
            .values_mut()
        {
            path_item["get"].as_object_mut().unwrap().remove("tags");
        }
        let connectors = catalogue(&[document]);
        let skill = skill_markdown(&connectors[0]);
        assert!(skill.contains("- `group0`: 60"));
    }

    #[test]
    fn operations_cannot_take_the_discovery_function_names() {
        let mut document = salesforce_document();
        document.document["paths"]["/find"] = json!({"get":{
            "operationId":"find",
            "responses":{"200":{"description":"ok"}}
        }});
        let connectors = catalogue(&[document]);
        let names = connectors[0]
            .operations
            .iter()
            .map(|operation| operation.function_name.as_str())
            .collect::<Vec<_>>();
        assert!(!names.contains(&"salesforce_find"), "{names:?}");
        assert!(names.contains(&"salesforce_find_2"), "{names:?}");
    }

    async fn call_discovery(
        name: &str,
        arguments: PythonCallArguments,
    ) -> Result<Value, PythonFunctionError> {
        let functions = build_python_functions(
            &[large_document(30)],
            Arc::new(RecordingNetwork::default()),
            Arc::new(InMemoryFs::new()),
        );
        functions
            .iter()
            .find(|function| function.name() == name)
            .unwrap()
            .call(arguments)
            .await
    }

    #[tokio::test]
    async fn find_matches_all_words_and_caps_results() {
        let result = call_discovery(
            "big_find",
            PythonCallArguments {
                positional: vec![json!("group1 widgets")],
                keyword: Map::from_iter([("limit".into(), json!(2))]),
            },
        )
        .await
        .unwrap();
        let text = result.as_str().unwrap();
        assert_eq!(
            text.lines().filter(|line| line.starts_with("big_")).count(),
            2
        );
        assert!(text.contains("GET /group1/items"));
        assert!(text.contains("[4 more matches"));
        assert!(!text.contains("group2"));
    }

    #[tokio::test]
    async fn find_reports_no_matches_and_missing_query() {
        let none = call_discovery(
            "big_find",
            PythonCallArguments {
                positional: Vec::new(),
                keyword: Map::from_iter([("query".into(), json!("zzz"))]),
            },
        )
        .await
        .unwrap();
        assert!(none.as_str().unwrap().starts_with("No functions match"));
        let missing = call_discovery(
            "big_find",
            PythonCallArguments {
                positional: Vec::new(),
                keyword: Map::new(),
            },
        )
        .await;
        assert!(missing.unwrap_err().0.contains("requires a query string"));
    }

    #[tokio::test]
    async fn describe_returns_the_operation_markdown() {
        let result = call_discovery(
            "big_describe",
            PythonCallArguments {
                positional: Vec::new(),
                keyword: Map::from_iter([("function".into(), json!("big_list_items3"))]),
            },
        )
        .await
        .unwrap();
        assert!(result
            .as_str()
            .unwrap()
            .contains("OpenAPI operation: `listItems3`"));
        let unknown = call_discovery(
            "big_describe",
            PythonCallArguments {
                positional: vec![json!("nope")],
                keyword: Map::new(),
            },
        )
        .await;
        assert!(unknown.unwrap_err().0.contains("big_find"));
    }

    fn salesforce_document() -> ConnectorDocument {
        ConnectorDocument {
            name: "Salesforce".into(),
            slug: "salesforce".into(),
            is_builtin: false,
            document: json!({
                "openapi":"3.0.0",
                "info":{"title":"Salesforce","version":"1"},
                "servers":[{"url":"https://upstream.example"}],
                "paths":{"/query":{"get":{
                    "operationId":"salesforce.query",
                    "parameters":[{"name":"q","in":"query","required":true,"schema":{"type":"string"}}],
                    "responses":{"200":{"description":"ok"}}
                }}}
            }),
        }
    }
}
