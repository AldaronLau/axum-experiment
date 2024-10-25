use axum::{extract::Path, routing, Router};

async fn delete(Path(path): Path<String>) -> String {
    format!("DELETE {path}")
}

async fn get(Path(path): Path<String>) -> String {
    format!("GET {path}")
}

async fn head(Path(path): Path<String>) -> String {
    format!("HEAD {path}")
}

async fn options(Path(path): Path<String>) -> String {
    format!("OPTIONS {path}")
}

async fn patch(Path(path): Path<String>) -> String {
    format!("PATCH {path}")
}

async fn post(Path(path): Path<String>) -> String {
    format!("POST {path}")
}

async fn put(Path(path): Path<String>) -> String {
    format!("PUT {path}")
}

async fn trace(Path(path): Path<String>) -> String {
    format!("TRACE {path}")
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
