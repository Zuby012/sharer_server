mod apis;
mod db;
mod error;
mod extractors;
mod state;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let state = state::AppState::new().await;
    let app = apis::router().with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Sharer server listening on port 3000");
    axum::serve(listener, app).await.unwrap();
}
