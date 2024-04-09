use assets::files::{index_css_map, index_js_map, StaticFile};
use axum::extract::Path;
use axum::http::{header, HeaderValue, Response, StatusCode};
use axum::response::IntoResponse;
use tokio_util::io::ReaderStream;
use axum::body::Body;

pub async fn static_path(Path(path): Path<String>) -> impl IntoResponse {
    let path = format!("/static/{}", path);

    let data = if path == "/static/index.css.map" {
        Some(&index_css_map)
    } else if path == "/static/index.js.map" {
        Some(&index_js_map)
    } else {
        StaticFile::get(&path)
    };

    if let Some(data) = data {
        let file = match tokio::fs::File::open(data.file_name).await {
            Ok(file) => file,
            Err(_) => {
                return Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .unwrap()
            }
        };

        // convert the `AsyncRead` into a `Stream`
        let stream = ReaderStream::new(file);

        return Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_str(data.mime.as_ref()).unwrap(),
            )
            .body(Body::from_stream(stream))
            .unwrap();
    }
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::empty())
        .unwrap()
}
