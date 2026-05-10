use reqwest::Client;
use std::time::Duration;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

pub async fn core_check_site(url: String, timeout_secs: u64) -> (String, String, String) {
    let client = Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .build()
        .unwrap_or_default();

    match client.get(&url).send().await {
        Ok(resp) => {
            let status = resp.status();
            if status.is_success() {
                ("✅".to_string(), url, status.to_string())
            } else {
                ("⚠️".to_string(), url, status.to_string())
            }
        }
        Err(_) => ("❌".to_string(), url, "Connection Failed".to_string()),
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub async fn check_site_wasm(url: String, timeout_secs: u64) -> String {
    let (icon, url, detail) = core_check_site(url, timeout_secs).await;
    format!("{} {}: {}", icon, url, detail)
}