use axum::response::IntoResponse;

pub struct UtEndpoint {}

impl UtEndpoint {
    pub async fn handle_presigned_url_request(
        state: axum::extract::State<crate::app_state::UtAppState>,
        axum::Json(payload): axum::Json<upload_things::UtRequest>,
    ) -> axum::response::Response {
        tracing::info!("New upload request: {:?}", payload);
        let presigned_url = match state.sign_url(payload) {
            Ok(presigned_url) => {
                tracing::info!("Presigned URL Generated");
                presigned_url
            }
            Err(e) => {
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
                    .into_response();
            }
        };
        let form = upload_things::UtRecord {
            file_keys: vec![presigned_url.file_key.clone()],
            ..Default::default()
        };
        let post_request = state.register_url(form).await;
        match post_request {
            Ok(_) => {
                tracing::info!("Registered Upload Response");
                (
                    [(reqwest::header::CONTENT_TYPE, "application/json")],
                    axum::Json(presigned_url),
                )
                    .into_response()
            }
            Err(e) => {
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
                    .into_response();
            }
        }
    }
}
