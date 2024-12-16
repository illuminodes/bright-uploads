enum UploadParams {
    Expires(u128),
    Identifier(String),
    FileName(String),
    FileSize(u64),
    Slug(String),
    Signature(String),
}
impl Into<(String, String)> for UploadParams {
    fn into(self) -> (String, String) {
        match self {
            Self::Expires(value) => ("expires".to_string(), value.to_string()),
            Self::Identifier(value) => ("x-ut-identifier".to_string(), value),
            Self::FileName(value) => ("x-ut-file-name".to_string(), value),
            Self::FileSize(value) => ("x-ut-file-size".to_string(), value.to_string()),
            Self::Slug(value) => ("x-ut-slug".to_string(), value),
            Self::Signature(value) => ("signature".to_string(), format!("hmac-sha256={}", value)),
        }
    }
}

const ONE_HOUR_DELAY_MILLIS: u128 = 3600 * 1000;
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct UtPreSignedUrl {
    pub url: String,
    pub file_key: String,
    pub expires: u128,
    #[serde(skip)]
    pub region: crate::UploadRegion,
}
impl Default for UtPreSignedUrl {
    fn default() -> Self {
        let expires = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("No time in platform")
            .as_millis()
            + ONE_HOUR_DELAY_MILLIS;
        Self {
            url: "".to_string(),
            file_key: "".to_string(),
            expires,
            region: crate::UploadRegion::UsWestSeattle,
        }
    }
}
impl UtPreSignedUrl {
    pub fn try_into_request(
        self,
        file_data: web_sys::FormData,
    ) -> Result<web_sys::Request, web_sys::wasm_bindgen::JsValue> {
        let init = web_sys::RequestInit::new();
        init.set_method("PUT");
        init.set_mode(web_sys::RequestMode::Cors);
        init.set_body(&file_data.into());
        let request = web_sys::Request::new_with_str_and_init(&self.url, &init)?;
        Ok(request)
    }
    pub fn presigned_url(
        &mut self,
        request: crate::UtRequest,
        api_key: String,
        app_id: String,
    ) -> anyhow::Result<()> {
        let query_params = vec![
            UploadParams::Expires(self.expires).into(),
            UploadParams::Identifier(app_id).into(),
            UploadParams::FileName(request.file_name).into(),
            UploadParams::FileSize(request.file_size).into(),
            UploadParams::Slug(request.slug).into(),
        ];
        let mut url = self.new_url(&query_params)?;
        self.url = url.to_string();
        let signature = self.generate_signature(api_key)?;
        let (query, signature) = UploadParams::Signature(signature).into();
        url.query_pairs_mut().append_pair(&query, &signature);
        self.url = url.to_string();
        Ok(())
    }
    fn new_url(&self, query_params: &[(String, String)]) -> anyhow::Result<url::Url> {
        // Generate the upload URL (example format)
        let mut url = url::Url::parse(&format!(
            "https://{}.ingest.uploadthing.com/{}",
            self.region.alias(),
            self.file_key
        ))?;
        for (key, value) in query_params {
            url.query_pairs_mut().append_pair(key, value);
        }
        Ok(url)
    }
    fn generate_signature(&self, api_key: String) -> anyhow::Result<String> {
        type HmacSha256 = hmac::Hmac<sha2::Sha256>;
        use hmac::Mac;
        let mut mac = HmacSha256::new_from_slice(api_key.as_bytes())?;
        mac.update(self.url.to_string().as_bytes());
        let result = mac.finalize();
        let signature_bytes = result.into_bytes();
        Ok(signature_bytes
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<String>())
    }
    pub fn generate_file_key(&mut self, app_id: String) -> anyhow::Result<()> {
        use base64::prelude::*;
        let app_hash = Self::djb2_hash(&app_id);
        let alphabet: Vec<char> = Self::shuffle(sqids::DEFAULT_ALPHABET, &app_id)
            .chars()
            .collect();
        let sqids = sqids::Sqids::builder()
            .alphabet(alphabet)
            .min_length(12)
            .build()?;
        let encoded_app_id = sqids.encode(&vec![app_hash.abs() as u64])?;
        let file_seed = uuid::Uuid::new_v4().to_string();
        let encoded_file_seed = BASE64_URL_SAFE.encode(file_seed.as_bytes());
        self.file_key = format!("{}{}", encoded_app_id, encoded_file_seed);
        Ok(())
    }
    fn djb2_hash(s: &str) -> i32 {
        let mut h: i64 = 5381;
        for &byte in s.as_bytes().iter().rev() {
            h = (h * 33) ^ (byte as i64);
            // Simulate 32-bit integer overflow
            h &= 0xFFFFFFFF;
        }
        // Convert to signed 32-bit integer with the same bit manipulation
        h = (h & 0xBFFFFFFF) | ((h >> 1) & 0x40000000);

        if h >= 0x80000000 {
            h -= 0x100000000;
        }
        h as i32
    }
    fn shuffle(input: &str, seed: &str) -> String {
        let mut chars: Vec<char> = input.chars().collect();
        let seed_num = Self::djb2_hash(seed);
        for i in 0..chars.len() {
            let j = ((seed_num % (i as i32 + 1)) + i as i32) as usize % chars.len();
            let temp = chars[i];
            chars[i] = chars[j];
            chars[j] = temp;
        }
        chars.iter().collect()
    }
}
impl TryFrom<&web_sys::wasm_bindgen::JsValue> for UtPreSignedUrl {
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
impl TryFrom<web_sys::wasm_bindgen::JsValue> for UtPreSignedUrl {
    type Error = web_sys::wasm_bindgen::JsValue;
    fn try_from(value: web_sys::wasm_bindgen::JsValue) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

#[cfg(test)]
#[test]
fn shuffle() {
    let input = sqids::DEFAULT_ALPHABET;
    let seed = "73bwh5z2wi";
    let shuffled = UtPreSignedUrl::shuffle(input, &seed);
    let expected = "Ha7cdM3yek9jLh6lb85oPwNAgKrIztFfXqnxismQGW2UTvuJOS4CZpRB10VDYE";
    assert_eq!(shuffled, expected);
}
#[cfg(test)]
#[test]
fn hash() {
    let seed = "73bwh5z2wi";
    let expected = "gL3R2N9BwZXI";
    let app_hash = UtPreSignedUrl::djb2_hash(&seed);
    let sqids = sqids::Sqids::builder()
        .alphabet(
            UtPreSignedUrl::shuffle(sqids::DEFAULT_ALPHABET, &seed)
                .chars()
                .collect(),
        )
        .min_length(12)
        .build()
        .expect("Failed to build sqids");
    let encoded_app_id = sqids
        .encode(&vec![app_hash.abs() as u64])
        .expect("Failed to encode");
    assert_eq!(encoded_app_id, expected);
}
