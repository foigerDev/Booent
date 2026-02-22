mod app;
mod routes;
mod server;
mod app_state;
mod api_models;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
tracing_subscriber::fmt()
    .with_ansi(false)
    .init();
    server::run().await
}
