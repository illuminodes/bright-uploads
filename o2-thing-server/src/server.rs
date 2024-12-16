#[derive(Clone)]
pub struct UtServer {}
impl UtServer {
    pub async fn server() -> anyhow::Result<()> {
        tracing::info!("Loading server routes...");
        let listener = Self::tcp_listener().await?;
        let router = Self::router().await?;
        axum::serve(listener, router).await?;
        Ok(())
    }
    async fn router() -> anyhow::Result<axum::Router> {
        let app_state = crate::app_state::UtAppState::default();
        let cors_layer = tower_http::cors::CorsLayer::new()
            .allow_origin(tower_http::cors::Any)
            .allow_headers(vec![axum::http::HeaderName::from_static("content-type")])
            .allow_methods(vec![axum::http::Method::POST, axum::http::Method::OPTIONS]);
        let new_router = axum::Router::new()
            .route(
                upload_things::UtRoute::PresignedUrl.into(),
                axum::routing::post(crate::endpoints::UtEndpoint::handle_presigned_url_request),
            )
            .layer(cors_layer)
            .with_state(app_state);
        Ok(new_router)
    }
    async fn tcp_listener() -> anyhow::Result<tokio::net::TcpListener> {
        let tcp_address = std::env::var("TCP_ADDRESS").expect("TCP_ADDRESS has not been set");
        tracing::info!("Server started at: http://{}", &tcp_address);
        Ok(tokio::net::TcpListener::bind(&tcp_address).await?)
    }
}
