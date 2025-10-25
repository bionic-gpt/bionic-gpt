use crate::{
    error::ApiError,
    models::{SqlParameters, SqlResultRow},
};
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;
use indexmap::IndexMap;
use regex::Regex;
use serde_json::Value;
use std::{collections::HashSet, str::FromStr};
use time::{
    format_description::well_known::Rfc3339, macros::format_description, Date, OffsetDateTime,
    PrimitiveDateTime, Time,
};
use tokio_postgres::types::ToSql;
use tokio_postgres::{types::Type, Statement};
use uuid::Uuid;

#[derive(Debug)]
pub struct RewrittenSql {
    pub sql: String,
    pub parameter_names: Vec<String>,
}

#[derive(Debug)]
pub struct PreparedParams {
    values: Vec<Box<dyn ToSql + Sync + Send>>,
}

impl PreparedParams {
    pub fn new(values: Vec<Box<dyn ToSql + Sync + Send>>) -> Self {
        Self { values }
    }

    pub fn as_refs(&self) -> Vec<&(dyn ToSql + Sync)> {
        self.values
            .iter()
            .map(|value| value.as_ref() as &(dyn ToSql + Sync))
            .collect()
    }
}

pub fn rewrite_named_parameters(sql: &str) -> Result<RewrittenSql, ApiError> {
    #[derive(Clone)]
    enum State {
        Normal,
        SingleQuoted,
        DoubleQuoted,
        LineComment,
        BlockComment,
        DollarQuoted { tag: Option<String> },
    }

    let mut state = State::Normal;
    let mut result = String::with_capacity(sql.len());
    let mut name_to_index: IndexMap<String, usize> = IndexMap::new();
    let chars: Vec<char> = sql.chars().collect();
    let mut idx = 0usize;

    while idx < chars.len() {
        let ch = chars[idx];
        match state {
            State::Normal => {
                if ch == '\'' {
                    state = State::SingleQuoted;
                    result.push(ch);
                    idx += 1;
                    continue;
                }
                if ch == '"' {
                    state = State::DoubleQuoted;
                    result.push(ch);
                    idx += 1;
                    continue;
                }
                if ch == '-' && chars.get(idx + 1) == Some(&'-') {
                    state = State::LineComment;
                    result.push(ch);
                    result.push('-');
                    idx += 2;
                    continue;
                }
                if ch == '/' && chars.get(idx + 1) == Some(&'*') {
                    state = State::BlockComment;
                    result.push(ch);
                    result.push('*');
                    idx += 2;
                    continue;
                }
                if ch == '$' {
                    let start = idx + 1;
                    let mut end = start;
                    while end < chars.len() && chars[end].is_ascii_alphanumeric() {
                        end += 1;
                    }

                    let tag = if end > start {
                        Some(chars[start..end].iter().collect::<String>())
                    } else {
                        None
                    };

                    if chars.get(end) == Some(&'$') {
                        state = State::DollarQuoted { tag: tag.clone() };
                        result.push(ch);
                        if let Some(tag) = tag {
                            result.push_str(&tag);
                        }
                        result.push('$');
                        idx = end + 1;
                        continue;
                    }
                }

                if ch == ':' {
                    if chars.get(idx + 1) == Some(&':') {
                        // type cast :: - keep both colons
                        result.push(':');
                        result.push(':');
                        idx += 2;
                        continue;
                    }

                    let start = idx + 1;
                    let mut end = start;
                    while end < chars.len()
                        && (chars[end].is_ascii_alphanumeric() || chars[end] == '_')
                    {
                        end += 1;
                    }

                    if end == start {
                        result.push(ch);
                        idx += 1;
                        continue;
                    }

                    let name: String = chars[start..end].iter().collect();
                    if name.is_empty() {
                        result.push(ch);
                        idx += 1;
                        continue;
                    }

                    let position = if let Some(existing) = name_to_index.get(&name) {
                        *existing
                    } else {
                        let new_index = name_to_index.len() + 1;
                        name_to_index.insert(name.clone(), new_index);
                        new_index
                    };

                    result.push('$');
                    result.push_str(&position.to_string());
                    idx = end;
                    continue;
                }

                result.push(ch);
                idx += 1;
            }
            State::SingleQuoted => {
                result.push(ch);
                idx += 1;
                if ch == '\'' {
                    if chars.get(idx) == Some(&'\'') {
                        result.push('\'');
                        idx += 1;
                    } else {
                        state = State::Normal;
                    }
                }
            }
            State::DoubleQuoted => {
                result.push(ch);
                idx += 1;
                if ch == '"' {
                    if chars.get(idx) == Some(&'"') {
                        result.push('"');
                        idx += 1;
                    } else {
                        state = State::Normal;
                    }
                }
            }
            State::LineComment => {
                result.push(ch);
                idx += 1;
                if ch == '\n' {
                    state = State::Normal;
                }
            }
            State::BlockComment => {
                result.push(ch);
                idx += 1;
                if ch == '*' && chars.get(idx) == Some(&'/') {
                    result.push('/');
                    idx += 1;
                    state = State::Normal;
                }
            }
            State::DollarQuoted { ref tag } => {
                result.push(ch);
                idx += 1;
                if ch == '$' {
                    let mut matches = true;
                    if let Some(tag) = tag {
                        for (offset, tag_ch) in tag.chars().enumerate() {
                            if chars.get(idx + offset).copied() != Some(tag_ch) {
                                matches = false;
                                break;
                            }
                        }
                        if matches && chars.get(idx + tag.len()) == Some(&'$') {
                            result.push_str(tag);
                            result.push('$');
                            idx += tag.len() + 1;
                            state = State::Normal;
                        }
                    } else if chars.get(idx) == Some(&'$') {
                        result.push('$');
                        idx += 1;
                        state = State::Normal;
                    }
                }
            }
        }
    }

    Ok(RewrittenSql {
        sql: result,
        parameter_names: name_to_index.keys().cloned().collect(),
    })
}

pub fn build_parameter_values(
    statement: &Statement,
    ordered_names: &[String],
    provided: &SqlParameters,
) -> Result<PreparedParams, ApiError> {
    if ordered_names.len() != statement.params().len() {
        return Err(ApiError::internal(format!(
            "parameter count mismatch: statement expects {}, but found {} placeholders",
            statement.params().len(),
            ordered_names.len()
        )));
    }

    let mut values = Vec::with_capacity(ordered_names.len());
    for (index, (name, expected_type)) in ordered_names.iter().zip(statement.params()).enumerate() {
        let value = provided
            .get(name)
            .or_else(|| provided.get(&format!("${}", index + 1)));
        let boxed = convert_parameter_value(name, expected_type, value)?;
        values.push(boxed);
    }

    Ok(PreparedParams::new(values))
}

fn convert_parameter_value(
    name: &str,
    ty: &Type,
    value: Option<&Value>,
) -> Result<Box<dyn ToSql + Sync + Send>, ApiError> {
    if value.is_none() {
        return match ty {
            &Type::BOOL => Ok(Box::new(Option::<bool>::None)),
            &Type::INT2 => Ok(Box::new(Option::<i16>::None)),
            &Type::INT4 => Ok(Box::new(Option::<i32>::None)),
            &Type::INT8 => Ok(Box::new(Option::<i64>::None)),
            &Type::FLOAT4 => Ok(Box::new(Option::<f32>::None)),
            &Type::FLOAT8 => Ok(Box::new(Option::<f64>::None)),
            &Type::NUMERIC => Ok(Box::new(Option::<f64>::None)),
            &Type::TEXT | &Type::VARCHAR | &Type::BPCHAR | &Type::NAME | &Type::UNKNOWN => {
                Ok(Box::new(Option::<String>::None))
            }
            &Type::JSON | &Type::JSONB => Ok(Box::new(Value::Null)),
            &Type::TIMESTAMP => Ok(Box::new(Option::<PrimitiveDateTime>::None)),
            &Type::TIMESTAMPTZ => Ok(Box::new(Option::<OffsetDateTime>::None)),
            &Type::DATE => Ok(Box::new(Option::<Date>::None)),
            &Type::TIME => Ok(Box::new(Option::<Time>::None)),
            &Type::UUID => Ok(Box::new(Option::<Uuid>::None)),
            &Type::BYTEA => Ok(Box::new(Option::<Vec<u8>>::None)),
            &Type::OID => Ok(Box::new(Option::<u32>::None)),
            _ => Err(ApiError::internal(format!(
                "parameter `{name}` is required"
            ))),
        };
    }

    let value = value.unwrap();
    if value.is_null() {
        return convert_parameter_value(name, ty, None);
    }

    match *ty {
        Type::BOOL => match value {
            Value::Bool(v) => Ok(Box::new(*v)),
            Value::Number(num) => Ok(Box::new(num.as_i64().unwrap_or(0) != 0)),
            Value::String(s) => Ok(Box::new(matches!(
                s.trim().to_lowercase().as_str(),
                "1" | "true" | "t" | "yes" | "y"
            ))),
            _ => Err(parameter_type_error(name, ty)),
        },
        Type::INT2 => extract_integer::<i16>(name, value, ty),
        Type::INT4 => extract_integer::<i32>(name, value, ty),
        Type::INT8 => extract_integer::<i64>(name, value, ty),
        Type::FLOAT4 => match value {
            Value::Number(num) => {
                if let Some(f) = num.as_f64() {
                    Ok(Box::new(f as f32))
                } else if let Some(i) = num.as_i64() {
                    Ok(Box::new(i as f32))
                } else if let Some(u) = num.as_u64() {
                    Ok(Box::new(u as f32))
                } else {
                    Err(parameter_type_error(name, ty))
                }
            }
            Value::String(s) => {
                let parsed = s
                    .parse::<f64>()
                    .map_err(|_| parameter_type_error(name, ty))?;
                Ok(Box::new(parsed as f32))
            }
            _ => Err(parameter_type_error(name, ty)),
        },
        Type::FLOAT8 => match value {
            Value::Number(num) => {
                if let Some(f) = num.as_f64() {
                    Ok(Box::new(f))
                } else if let Some(i) = num.as_i64() {
                    Ok(Box::new(i as f64))
                } else if let Some(u) = num.as_u64() {
                    Ok(Box::new(u as f64))
                } else {
                    Err(parameter_type_error(name, ty))
                }
            }
            Value::String(s) => {
                let parsed = s
                    .parse::<f64>()
                    .map_err(|_| parameter_type_error(name, ty))?;
                Ok(Box::new(parsed))
            }
            _ => Err(parameter_type_error(name, ty)),
        },
        Type::NUMERIC => match value {
            Value::Number(num) => {
                if let Some(f) = num.as_f64() {
                    Ok(Box::new(f))
                } else if let Some(i) = num.as_i64() {
                    Ok(Box::new(i as f64))
                } else if let Some(u) = num.as_u64() {
                    Ok(Box::new(u as f64))
                } else {
                    Err(parameter_type_error(name, ty))
                }
            }
            Value::String(s) => {
                let parsed = s
                    .parse::<f64>()
                    .map_err(|_| parameter_type_error(name, ty))?;
                Ok(Box::new(parsed))
            }
            _ => Err(parameter_type_error(name, ty)),
        },
        Type::TEXT | Type::VARCHAR | Type::BPCHAR | Type::NAME | Type::UNKNOWN => {
            Ok(Box::new(value_to_string(value)))
        }
        Type::JSON | Type::JSONB => Ok(Box::new(value.clone())),
        Type::TIMESTAMP => match value {
            Value::String(s) => Ok(Box::new(parse_timestamp(s)?)),
            _ => Err(parameter_type_error(name, ty)),
        },
        Type::TIMESTAMPTZ => match value {
            Value::String(s) => Ok(Box::new(parse_timestamptz(s)?)),
            _ => Err(parameter_type_error(name, ty)),
        },
        Type::DATE => match value {
            Value::String(s) => Ok(Box::new(parse_date(s)?)),
            _ => Err(parameter_type_error(name, ty)),
        },
        Type::TIME => match value {
            Value::String(s) => Ok(Box::new(parse_time(s)?)),
            _ => Err(parameter_type_error(name, ty)),
        },
        Type::UUID => match value {
            Value::String(s) => {
                let uuid = Uuid::parse_str(s).map_err(|_| parameter_type_error(name, ty))?;
                Ok(Box::new(uuid))
            }
            _ => Err(parameter_type_error(name, ty)),
        },
        Type::BYTEA => match value {
            Value::String(s) => {
                let decoded = BASE64_STANDARD
                    .decode(s.as_bytes())
                    .map_err(|_| parameter_type_error(name, ty))?;
                Ok(Box::new(decoded))
            }
            _ => Err(parameter_type_error(name, ty)),
        },
        Type::OID => extract_integer::<u32>(name, value, ty),
        _ => Err(ApiError::internal(format!(
            "unsupported parameter type `{}` for `{name}`",
            ty.name()
        ))),
    }
}

fn extract_integer<T>(
    name: &str,
    value: &Value,
    ty: &Type,
) -> Result<Box<dyn ToSql + Sync + Send>, ApiError>
where
    T: TryFrom<i64> + TryFrom<u64> + ToSql + Sync + Send + 'static,
{
    match value {
        Value::Number(num) => {
            if let Some(i) = num.as_i64() {
                let converted = T::try_from(i).map_err(|_| parameter_type_error(name, ty))?;
                Ok(Box::new(converted))
            } else if let Some(u) = num.as_u64() {
                let converted = T::try_from(u).map_err(|_| parameter_type_error(name, ty))?;
                Ok(Box::new(converted))
            } else {
                Err(parameter_type_error(name, ty))
            }
        }
        Value::String(s) => {
            let parsed = i128::from_str(s).map_err(|_| parameter_type_error(name, ty))?;
            if parsed < i64::MIN as i128 || parsed > i64::MAX as i128 {
                return Err(parameter_type_error(name, ty));
            }
            extract_integer::<T>(
                name,
                &Value::Number(serde_json::Number::from(parsed as i64)),
                ty,
            )
        }
        _ => Err(parameter_type_error(name, ty)),
    }
}

fn parameter_type_error(name: &str, ty: &Type) -> ApiError {
    ApiError::internal(format!(
        "parameter `{name}` has incompatible type for expected `{}`",
        ty.name()
    ))
}

fn value_to_string(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Array(arr) => serde_json::to_string(arr).unwrap_or_default(),
        Value::Object(map) => serde_json::to_string(map).unwrap_or_default(),
        Value::Null => "null".into(),
    }
}

fn parse_timestamp(input: &str) -> Result<PrimitiveDateTime, ApiError> {
    if let Ok(pdt) = PrimitiveDateTime::parse(
        input,
        &format_description!(
            "[year]-[month]-[day] [hour]:[minute]:[second][optional [.[subsecond digits:6]]]"
        ),
    ) {
        return Ok(pdt);
    }

    let odt = OffsetDateTime::parse(input, &Rfc3339)
        .map_err(|_| ApiError::internal("invalid timestamp"))?;
    Ok(PrimitiveDateTime::new(odt.date(), odt.time()))
}

fn parse_timestamptz(input: &str) -> Result<OffsetDateTime, ApiError> {
    OffsetDateTime::parse(input, &Rfc3339)
        .map_err(|_| ApiError::internal("invalid timestamp with time zone"))
}

fn parse_date(input: &str) -> Result<Date, ApiError> {
    Date::parse(input, &format_description!("[year]-[month]-[day]"))
        .map_err(|_| ApiError::internal("invalid date"))
}

fn parse_time(input: &str) -> Result<Time, ApiError> {
    Time::parse(
        input,
        &format_description!("[hour]:[minute]:[second][optional [.[subsecond digits:6]]]"),
    )
    .map_err(|_| ApiError::internal("invalid time"))
}

pub fn row_to_json(row: &tokio_postgres::Row) -> Result<SqlResultRow, ApiError> {
    let mut values = Vec::with_capacity(row.len());

    for (idx, column) in row.columns().iter().enumerate() {
        let ty = column.type_();

        let cell = match *ty {
            Type::BOOL => row
                .try_get::<_, Option<bool>>(idx)
                .map(|opt| opt.map(Value::Bool))?,
            Type::INT2 => row
                .try_get::<_, Option<i16>>(idx)
                .map(|opt| opt.map(|v| Value::Number(serde_json::Number::from(v as i64))))?,
            Type::INT4 => row
                .try_get::<_, Option<i32>>(idx)
                .map(|opt| opt.map(|v| Value::Number(serde_json::Number::from(v as i64))))?,
            Type::INT8 => row
                .try_get::<_, Option<i64>>(idx)
                .map(|opt| opt.map(|v| Value::Number(serde_json::Number::from(v))))?,
            Type::FLOAT4 => row.try_get::<_, Option<f32>>(idx).map(|opt| {
                opt.and_then(|v| serde_json::Number::from_f64(v as f64).map(Value::Number))
            })?,
            Type::FLOAT8 => row
                .try_get::<_, Option<f64>>(idx)
                .map(|opt| opt.and_then(|v| serde_json::Number::from_f64(v).map(Value::Number)))?,
            Type::NUMERIC => row.try_get::<_, Option<f64>>(idx).map(|opt| {
                opt.and_then(|v| serde_json::Number::from_f64(v).map(Value::Number))
                    .or_else(|| opt.map(|v| Value::String(v.to_string())))
            })?,
            Type::TEXT | Type::VARCHAR | Type::BPCHAR | Type::NAME => row
                .try_get::<_, Option<String>>(idx)
                .map(|opt| opt.map(Value::String))?,
            Type::JSON | Type::JSONB => row.try_get::<_, Option<Value>>(idx)?,
            Type::TIMESTAMP => row
                .try_get::<_, Option<PrimitiveDateTime>>(idx)
                .map(|opt| {
                    opt.map(|value| {
                        let odt = value.assume_utc();
                        let formatted = odt.format(&Rfc3339).unwrap_or_else(|_| odt.to_string());
                        Value::String(formatted)
                    })
                })?,
            Type::TIMESTAMPTZ => row.try_get::<_, Option<OffsetDateTime>>(idx).map(|opt| {
                opt.map(|value| {
                    let formatted = value.format(&Rfc3339).unwrap_or_else(|_| value.to_string());
                    Value::String(formatted)
                })
            })?,
            Type::DATE => row
                .try_get::<_, Option<Date>>(idx)
                .map(|opt| opt.map(|value| Value::String(value.to_string())))?,
            Type::TIME => row
                .try_get::<_, Option<Time>>(idx)
                .map(|opt| opt.map(|value| Value::String(value.to_string())))?,
            Type::UUID => row
                .try_get::<_, Option<Uuid>>(idx)
                .map(|opt| opt.map(|value| Value::String(value.to_string())))?,
            Type::BYTEA => row
                .try_get::<_, Option<Vec<u8>>>(idx)
                .map(|opt| opt.map(|value| Value::String(BASE64_STANDARD.encode(value))))?,
            Type::OID => row
                .try_get::<_, Option<u32>>(idx)
                .map(|opt| opt.map(|v| Value::Number(serde_json::Number::from(v as u64))))?,
            _ => row
                .try_get::<_, Option<String>>(idx)
                .map(|opt| opt.map(Value::String))?,
        };

        values.push(cell.unwrap_or(Value::Null));
    }

    Ok(values)
}

pub fn extract_filter_columns(filter: &str) -> Vec<String> {
    // Capture column names from patterns like "schema"."table"."column" or "alias"."column"
    // and de-duplicate results.
    static QUALIFIED: once_cell::sync::Lazy<Regex> = once_cell::sync::Lazy::new(|| {
        Regex::new(r#""(?:(?:[^"]+)"\.)?(?:(?:[^"]+)"\.)?"(?P<column>[^"]+)""#).unwrap()
    });

    let mut seen = HashSet::new();
    let mut columns = Vec::new();
    for caps in QUALIFIED.captures_iter(filter) {
        if let Some(col) = caps.name("column") {
            let name = col.as_str().to_string();
            if seen.insert(name.clone()) {
                columns.push(name);
            }
        }
    }
    columns
}
