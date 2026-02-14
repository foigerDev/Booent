mod app;
mod routes;
mod server;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    server::run().await
}
