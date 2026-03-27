mod api_models;
mod app;
mod app_state;
mod routes;
mod server;
mod utils;
mod middleware;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt().with_ansi(false).init();
    server::run().await
}
