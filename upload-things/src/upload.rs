#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq, Hash)]
pub struct UtUpload {
    pub url: String,
    #[serde(rename = "appUrl")]
    pub app_url: String,
    #[serde(rename = "fileHash")]
    pub file_hash: String,
}
impl TryFrom<&web_sys::wasm_bindgen::JsValue> for UtUpload {
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
impl TryFrom<web_sys::wasm_bindgen::JsValue> for UtUpload {
    type Error = web_sys::wasm_bindgen::JsValue;
    fn try_from(value: web_sys::wasm_bindgen::JsValue) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}
