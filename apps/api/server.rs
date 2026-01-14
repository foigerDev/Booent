use common::app_state::builder::build_app_state;

#[tokio::main]
async fn main() {
    let _app_state = build_app_state().await;

    // pass app_state to Axum / Actix
}
