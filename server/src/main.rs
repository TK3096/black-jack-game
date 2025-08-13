use black_jack_game_server::infrastructure::axum_http::http_serve::start;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    start().await.expect("Failed to start the server");
}
