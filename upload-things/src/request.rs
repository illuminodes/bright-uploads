use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UtRequest {
    pub slug: String,
    pub action_type: String,
    pub file_name: String,
    pub file_size: u64,
}
impl Into<web_sys::RequestInit> for UtRequest {
    fn into(self) -> web_sys::RequestInit {
        let opts = web_sys::RequestInit::new();
        opts.set_method("POST");
        opts.set_mode(web_sys::RequestMode::Cors);
        opts.set_body(&web_sys::wasm_bindgen::JsValue::from_str(
            self.to_string().as_str(),
        ));
        let headers = web_sys::Headers::new().expect("Failed to build headers");
        headers
            .set("Content-Type", "application/json")
            .expect("Failed to set header");
        opts.set_headers(&headers);
        opts
    }
}
impl From<&web_sys::File> for UtRequest {
    fn from(file: &web_sys::File) -> Self {
        let slug = file
            .name()
            .to_lowercase()
            .replace(" ", "-")
            .replace(".", "-");
        Self {
            slug,
            action_type: "upload".to_string(),
            file_name: file.name(),
            file_size: file.size().abs() as u64,
        }
    }
}
impl From<web_sys::File> for UtRequest {
    fn from(file: web_sys::File) -> Self {
        let slug = file
            .name()
            .to_lowercase()
            .replace(" ", "-")
            .replace(".", "-");
        Self {
            slug,
            action_type: "upload".to_string(),
            file_name: file.name(),
            file_size: file.size().abs() as u64,
        }
    }
}
impl TryFrom<&web_sys::wasm_bindgen::JsValue> for UtRequest {
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
impl TryFrom<web_sys::wasm_bindgen::JsValue> for UtRequest {
    type Error = web_sys::wasm_bindgen::JsValue;
    fn try_from(value: web_sys::wasm_bindgen::JsValue) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl Default for UtRequest {
    fn default() -> Self {
        Self {
            slug: "".to_string(),
            action_type: "upload".to_string(),
            file_name: "".to_string(),
            file_size: 0,
        }
    }
}
impl Display for UtRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
