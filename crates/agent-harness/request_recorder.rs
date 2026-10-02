use bytes::Bytes;
use db::{queries, Pool};
use rig::http_client::{
    DynHttpClient, HeaderMap, HttpMiddleware, Method, Result as HttpResult, Uri,
};

#[derive(Clone)]
pub(crate) struct RequestRecorder {
    pool: Pool,
    subject: String,
    chat_id: i32,
}

impl RequestRecorder {
    pub(crate) fn new(pool: Pool, subject: String, chat_id: i32) -> Self {
        Self {
            pool,
            subject,
            chat_id,
        }
    }

    pub(crate) fn transport(self) -> DynHttpClient {
        rig_reqwest::shared().with_middleware(self)
    }

    async fn persist(
        &self,
        method: &str,
        uri: &str,
        body: &str,
    ) -> Result<(), crate::errors::CustomError> {
        let mut client = self.pool.get().await?;
        let transaction = client.transaction().await?;
        db::authz::set_row_level_security_user_id(&transaction, self.subject.clone()).await?;
        queries::model_requests::insert()
            .bind(&transaction, &self.chat_id, &method, &uri, &body)
            .await?;
        transaction.commit().await?;
        Ok(())
    }
}

impl HttpMiddleware for RequestRecorder {
    fn before_request_body<'a>(
        &'a self,
        method: &'a Method,
        uri: &'a Uri,
        _headers: &'a HeaderMap,
        body: Bytes,
    ) -> rig::wasm_compat::WasmBoxedFuture<'a, HttpResult<Bytes>> {
        Box::pin(async move {
            match std::str::from_utf8(&body) {
                Ok(payload) => {
                    let uri = sanitized_uri(uri);
                    if let Err(error) = self.persist(method.as_str(), &uri, payload).await {
                        tracing::error!(
                            chat_id = self.chat_id,
                            error = %error,
                            "Failed to record outbound model request"
                        );
                    }
                }
                Err(error) => {
                    tracing::error!(
                        chat_id = self.chat_id,
                        error = %error,
                        "Outbound model request body was not valid UTF-8"
                    );
                }
            }
            Ok(body)
        })
    }
}

fn sanitized_uri(uri: &Uri) -> String {
    match (uri.scheme_str(), uri.authority()) {
        (Some(scheme), Some(authority)) => {
            let host = authority.as_str().rsplit('@').next().unwrap_or_default();
            format!("{scheme}://{host}{}", uri.path())
        }
        _ => uri.path().to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::sanitized_uri;
    use rig::http_client::Uri;

    #[test]
    fn request_uri_drops_credentials_and_query_parameters() {
        let uri: Uri = "https://user:secret@example.com/v1/chat/completions?api_key=secret"
            .parse()
            .expect("valid URI");

        assert_eq!(
            sanitized_uri(&uri),
            "https://example.com/v1/chat/completions"
        );
    }
}
