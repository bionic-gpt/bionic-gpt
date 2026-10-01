use crate::{HttpHeader, HttpMethod, HttpRequest, NetworkError, SandboxNetwork};
use async_trait::async_trait;
use bashkit::{HttpResponse, HttpTransport, HttpTransportError, HttpTransportRequest};
use std::sync::Arc;

pub(super) struct NetworkAdapter(pub(super) Arc<dyn SandboxNetwork>);

#[async_trait]
impl HttpTransport for NetworkAdapter {
    async fn execute(
        &self,
        request: HttpTransportRequest,
    ) -> Result<HttpResponse, HttpTransportError> {
        let method = match request.method.as_str() {
            "GET" => HttpMethod::Get,
            "POST" => HttpMethod::Post,
            "PUT" => HttpMethod::Put,
            "DELETE" => HttpMethod::Delete,
            "HEAD" => HttpMethod::Head,
            "PATCH" => HttpMethod::Patch,
            method => {
                return Err(HttpTransportError::Denied(format!(
                    "unsupported method {method}"
                )))
            }
        };
        let response = self
            .0
            .request(HttpRequest {
                method,
                url: request.url,
                headers: request
                    .headers
                    .into_iter()
                    .map(|(name, value)| HttpHeader { name, value })
                    .collect(),
                body: request.body,
                timeout: request.timeout,
                connect_timeout: request.connect_timeout,
                max_response_bytes: request.max_response_bytes as u64,
            })
            .await
            .map_err(|error| match error {
                NetworkError::Denied(message) => HttpTransportError::Denied(message),
                NetworkError::Timeout => HttpTransportError::Timeout,
                NetworkError::TooLarge(message) => HttpTransportError::TooLarge(message),
                NetworkError::Transport(message) => HttpTransportError::Transport(message),
            })?;
        Ok(HttpResponse {
            status: response.status,
            headers: response
                .headers
                .into_iter()
                .map(|header| (header.name, header.value))
                .collect(),
            body: response.body,
        })
    }
}
