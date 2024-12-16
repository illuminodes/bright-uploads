use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Deserialize, Serialize)]
pub struct Metadata {
    #[serde(rename = "uploadedBy")]
    uploaded_by: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct UtRecord {
    #[serde(rename = "fileKeys")]
    pub file_keys: Vec<String>,
    pub metadata: Metadata,
    #[serde(rename = "callbackUrl")]
    pub callback_url: String,
    #[serde(rename = "callbackSlug")]
    pub callback_slug: String,
    #[serde(rename = "awaitServerData")]
    pub await_server_data: bool,
    #[serde(rename = "isDev")]
    pub is_dev: bool,
}
impl Display for UtRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
impl Default for UtRecord {
    fn default() -> Self {
        Self {
            file_keys: Vec::new(),
            metadata: Metadata {
                uploaded_by: "anonymous".to_string(),
            },
            callback_url: std::env::var("CALLBACK_DOMAIN")
                .expect("CALLBACK_DOMAIN has not been set"),
            callback_slug: "imageUploader".to_string(),
            await_server_data: false,
            is_dev: false,
        }
    }
}
impl TryFrom<&web_sys::wasm_bindgen::JsValue> for UtRecord {
    type Error = web_sys::wasm_bindgen::JsValue;
    fn try_from(value: &web_sys::wasm_bindgen::JsValue) -> Result<Self, Self::Error> {
        let js_str = web_sys::js_sys::JSON::stringify(value)?;
        let str = js_str
            .as_string()
            .ok_or_else(|| web_sys::wasm_bindgen::JsValue::from_str("Failed to stringify JSON"))?;
        let presigned_url: Self = serde_json::from_str(&str).map_err(|e| {
            web_sys::wasm_bindgen::JsValue::from_str(&format!("Failed to parse JSON: {}", e))
        })?;
        Ok(presigned_url)
    }
}
impl TryFrom<web_sys::wasm_bindgen::JsValue> for UtRecord {
    type Error = web_sys::wasm_bindgen::JsValue;
    fn try_from(value: web_sys::wasm_bindgen::JsValue) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}
