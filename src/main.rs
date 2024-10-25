use axum::{
    routing,
    Router,
    extract::Path,
};

async fn handler(Path(path): Path<String>) -> String {
    path
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/{*path}", routing::get(handler));
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
