use crate::app::build_app;
use std::net::SocketAddr;

pub async fn run() -> Result<(), std::io::Error> {
    let app_state = common::app_state::builder::build_app_state().await;
    let app = build_app(app_state);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("Server running on http://{}", addr);

    axum_server::Server::bind(addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
