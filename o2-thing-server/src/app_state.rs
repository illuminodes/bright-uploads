#[derive(Clone)]
pub struct UtAppState {
    client: reqwest::Client,
    api_key: String,
    app_id: String,
}
impl UtAppState {
    pub async fn register_url(
        &self,
        form: upload_things::UtRecord,
    ) -> anyhow::Result<reqwest::Response> {
        let region: &str = upload_things::UploadRegion::UsWestSeattle.alias();
        let url = format!("https://{}.ingest.uploadthing.com/route-metadata", region);
        let post_request = self.client.post(&url).body(form.to_string());
        post_request.send().await.map_err(|e| anyhow::anyhow!(e))
    }
    pub fn sign_url(
        &self,
        payload: upload_things::UtRequest,
    ) -> anyhow::Result<upload_things::UtPreSignedUrl> {
        let mut unsigned = upload_things::UtPreSignedUrl::default();
        unsigned.generate_file_key(self.app_id.clone())?;
        unsigned.presigned_url(payload, self.api_key.clone(), self.app_id.clone())?;
        Ok(unsigned)
    }
}
impl Default for UtAppState {
    fn default() -> Self {
        let mut header_map = axum::http::HeaderMap::new();
        header_map.insert(
            reqwest::header::CONTENT_TYPE,
            axum::http::HeaderValue::from_static("application/json"),
        );
        let api_key = std::env::var("API_KEY").expect("API_KEY has not been set");
        header_map.insert(
            "x-uploadthing-api-key",
            axum::http::HeaderValue::from_str(&api_key).expect("API_KEY has not been set"),
        );
        Self {
            client: reqwest::Client::builder()
                .default_headers(header_map)
                .build()
                .expect("Failed to build reqwest client"),
            api_key,
            app_id: std::env::var("MY_APP_ID").expect("MY_APP_ID has not been set"),
        }
    }
}
