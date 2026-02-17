use zed_extension_api::http_client::{HttpMethod, HttpRequest};

pub struct CratesClient;

impl CratesClient {
    pub fn new() -> Self {
        Self
    }

    pub fn get_latest_version(&self, crate_name: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("https://crates.io/api/v1/crates/{}", crate_name);

        let request = HttpRequest::builder()
            .method(HttpMethod::Get)
            .url(&url)
            .build()
            .map_err(|e| format!("Failed to build request: {}", e))?;

        let response = request.fetch().map_err(|e| format!("HTTP request failed: {}", e))?;

        let body = String::from_utf8_lossy(&response.body);

        let json: serde_json::Value = serde_json::from_str(&body)
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;

        // Check for errors
        if let Some(errors) = json.get("errors") {
            let error_msg = errors.as_array()
                .and_then(|arr| arr.first())
                .and_then(|e| e.get("detail"))
                .and_then(|d| d.as_str())
                .unwrap_or("Unknown error");
            return Err(format!("crates.io error: {}", error_msg).into());
        }

        let version = json["crate"]["max_version"]
            .as_str()
            .ok_or("No max_version found")?;

        Ok(version.to_string())
    }
}

impl Default for CratesClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_get_latest_version_serde() {
        let client = CratesClient::new();
        let result = client.get_latest_version("serde");
        assert!(result.is_ok());
        let version = result.unwrap();
        assert!(!version.is_empty());
        println!("Serde version: {}", version);
    }

    #[test]
    #[ignore]
    fn test_get_latest_version_tokio() {
        let client = CratesClient::new();
        let result = client.get_latest_version("tokio");
        assert!(result.is_ok());
        let version = result.unwrap();
        assert!(!version.is_empty());
        println!("Tokio version: {}", version);
    }

    #[test]
    #[ignore]
    fn test_get_latest_version_nonexistent() {
        let client = CratesClient::new();
        let result = client.get_latest_version("nonexistent-crate-xyz");
        assert!(result.is_err());
    }
}
