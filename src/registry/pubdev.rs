use zed_extension_api::http_client::{HttpMethod, HttpRequest};

pub struct PubDevClient;

impl PubDevClient {
    pub fn new() -> Self {
        Self
    }

    pub fn get_latest_version(&self, package_name: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("https://pub.dev/api/packages/{}", package_name);

        let request = HttpRequest::builder()
            .method(HttpMethod::Get)
            .url(&url)
            .build()
            .map_err(|e| format!("Failed to build request: {}", e))?;

        let response = request.fetch().map_err(|e| format!("HTTP request failed: {}", e))?;

        let body = String::from_utf8_lossy(&response.body);

        // Check for error responses
        if body.contains("\"error\"") || body.contains("Package not found") {
            return Err(format!("pub.dev error: Package '{}' not found", package_name).into());
        }

        let json: serde_json::Value = serde_json::from_str(&body)
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;

        let version = json["latest"]["version"]
            .as_str()
            .ok_or("No latest version found")?;

        Ok(version.to_string())
    }
}

impl Default for PubDevClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_get_latest_version_flutter() {
        let client = PubDevClient::new();
        let result = client.get_latest_version("flutter");
        assert!(result.is_ok());
        let version = result.unwrap();
        assert!(!version.is_empty());
        println!("Flutter version: {}", version);
    }

    #[test]
    #[ignore]
    fn test_get_latest_version_provider() {
        let client = PubDevClient::new();
        let result = client.get_latest_version("provider");
        assert!(result.is_ok());
        let version = result.unwrap();
        assert!(!version.is_empty());
        println!("Provider version: {}", version);
    }

    #[test]
    #[ignore]
    fn test_get_latest_version_nonexistent() {
        let client = PubDevClient::new();
        let result = client.get_latest_version("nonexistent-package-xyz123");
        assert!(result.is_err());
    }
}
