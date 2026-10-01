use async_trait::async_trait;
use std::fmt;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Head,
    Patch,
}

#[derive(Clone, PartialEq, Eq)]
pub struct HttpHeader {
    pub name: String,
    pub value: String,
}

impl fmt::Debug for HttpHeader {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HttpHeader")
            .field("name", &self.name)
            .field("value", &"[REDACTED]")
            .finish()
    }
}

#[derive(Clone)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub url: String,
    pub headers: Vec<HttpHeader>,
    pub body: Option<Vec<u8>>,
    pub timeout: Duration,
    pub connect_timeout: Option<Duration>,
    pub max_response_bytes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: Vec<HttpHeader>,
    pub body: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkError {
    Denied(String),
    Timeout,
    TooLarge(String),
    Transport(String),
}

impl fmt::Display for NetworkError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Denied(message) => write!(formatter, "access denied: {message}"),
            Self::Timeout => formatter.write_str("operation timed out"),
            Self::TooLarge(message) => write!(formatter, "response too large: {message}"),
            Self::Transport(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for NetworkError {}

#[async_trait]
pub trait SandboxNetwork: Send + Sync {
    async fn request(&self, request: HttpRequest) -> Result<HttpResponse, NetworkError>;
}

#[derive(Debug, Default)]
pub struct DenyNetwork;

#[async_trait]
impl SandboxNetwork for DenyNetwork {
    async fn request(&self, request: HttpRequest) -> Result<HttpResponse, NetworkError> {
        Err(NetworkError::Denied(format!(
            "network access is not enabled for {}",
            request.url
        )))
    }
}
