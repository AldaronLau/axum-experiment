use std::borrow::Cow;

use axum::{
    body::Body, extract::Path, response::IntoResponse, routing, Router,
};
use http::{response::Response as HttpResponse, StatusCode};

struct Response(StatusCode, Cow<'static, [u8]>);

impl IntoResponse for Response {
    fn into_response(self) -> HttpResponse<Body> {
        let Self(status, bytes) = self;
        let res = Body::from(bytes).into_response();

        (status, res).into_response()
    }
}

async fn delete(Path(path): Path<String>) -> Response {
    Response(StatusCode::OK, format!("DELETE {path}").into_bytes().into())
}

async fn get(Path(path): Path<String>) -> Response {
    Response(StatusCode::OK, format!("GET {path}").into_bytes().into())
}

async fn head(Path(path): Path<String>) -> Response {
    Response(StatusCode::OK, format!("HEAD {path}").into_bytes().into())
}

async fn options(Path(path): Path<String>) -> Response {
    Response(StatusCode::OK, format!("OPTIONS {path}").into_bytes().into())
}

async fn patch(Path(path): Path<String>) -> Response {
    Response(StatusCode::OK, format!("PATCH {path}").into_bytes().into())
}

async fn post(Path(path): Path<String>) -> Response {
    Response(StatusCode::OK, format!("POST {path}").into_bytes().into())
}

async fn put(Path(path): Path<String>) -> Response {
    Response(StatusCode::OK, format!("PUT {path}").into_bytes().into())
}

async fn trace(Path(path): Path<String>) -> Response {
    Response(StatusCode::OK, format!("TRACE {path}").into_bytes().into())
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route(
        "/{*path}",
        routing::delete(delete)
            .get(get)
            .head(head)
            .options(options)
            .patch(patch)
            .post(post)
            .put(put)
            .trace(trace),
    );
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
