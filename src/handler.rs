use axum::body::Body;
use hyper::{Request, Response, StatusCode};
use tower::{Service, ServiceExt};
use tower_http::services::{fs::ServeFileSystemResponseBody, ServeDir};

pub async fn serve_dir<F: Clone>(
    serve_dir: ServeDir<F>,
    suffixes: Vec<String>,
    request: Request<Body>,
) -> Result<Response<ServeFileSystemResponseBody>, StatusCode>
where
    ServeDir<F>: Service<Request<Body>, Response = Response<ServeFileSystemResponseBody>>,
{
    let uri = request.uri().clone();

    let mut res = serve_dir
        .clone()
        .oneshot(request)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    let mut index = 0;

    while res.status() == StatusCode::NOT_FOUND && index < suffixes.len() {
        let suffix = suffixes.get(index).ok_or(StatusCode::NOT_FOUND)?;
        let req = Request::builder()
            .uri(format!("{uri}{suffix}"))
            .body(Body::empty())
            .or(Err(StatusCode::NOT_FOUND))?;

        res = serve_dir
            .clone()
            .oneshot(req)
            .await
            .or(Err(StatusCode::NOT_FOUND))?;
        index += 1;
    }

    Ok(res)
}
