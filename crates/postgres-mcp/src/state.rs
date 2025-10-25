use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    inner: Arc<AppStateInner>,
}

struct AppStateInner {
    openapi_spec: &'static str,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(AppStateInner {
                openapi_spec: include_str!("../../mcp/specs/postgres.json"),
            }),
        }
    }

    pub fn openapi_spec(&self) -> &'static str {
        self.inner.openapi_spec
    }
}
