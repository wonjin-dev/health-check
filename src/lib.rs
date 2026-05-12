#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "cli")]
pub async fn core_check_site(url: String, timeout_secs: u64) -> (String, String, String) {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(timeout_secs))
        .build()
        .unwrap_or_default();

    match client.get(&url).send().await {
        Ok(resp) => {
            let status = resp.status();
            if status.is_success() {
                ("✅".to_string(), url, format!("{}", status))
            } else {
                ("⚠️".to_string(), url, format!("{}", status))
            }
        }
        Err(e) => ("❌".to_string(), url, e.to_string()),
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub async fn check_site_wasm(url: String) -> Result<JsValue, JsValue> {
    let client = reqwest::Client::new();
    match client.get(&url).send().await {
        Ok(resp) => {
            let status = resp.status().to_string();
            Ok(JsValue::from_str(&status))
        }
        Err(e) => Err(JsValue::from_str(&e.to_string())),
    }
}