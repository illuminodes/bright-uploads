mod app_state;
mod endpoints;
mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    if let Err(error) = server::UtServer::server().await {
        tracing::error!("Server Errored Out: {}", error);
    }
    Ok(())
}
