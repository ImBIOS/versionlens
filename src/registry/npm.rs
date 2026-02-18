use zed_extension_api::http_client::{HttpMethod, HttpRequest};

pub struct NpmClient;

impl NpmClient {
    pub fn new() -> Self {
        Self
    }

    pub fn get_latest_version(&self, package_name: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("https://registry.npmjs.org/{}", package_name);

        let request = HttpRequest::builder()
            .method(HttpMethod::Get)
            .url(&url)
            .build()
            .map_err(|e| format!("Failed to build request: {}", e))?;

        let response = request.fetch().map_err(|e| format!("HTTP request failed: {}", e))?;

        let body = String::from_utf8_lossy(&response.body);

        // Try to parse as JSON to check for errors
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
            // Check for npm registry error responses (e.g., 404)
            if let Some(error) = json.get("error").and_then(|e| e.as_str()) {
                return Err(format!("npm error: {} - {}", error, json.get("reason").map(|r| r.as_str().unwrap_or("")).unwrap_or("")).into());
            }
        }

        let json: serde_json::Value = serde_json::from_str(&body)
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;

        let version = json["dist-tags"]["latest"]
            .as_str()
            .ok_or("No latest version found")?;

        Ok(version.to_string())
    }
}

impl Default for NpmClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // These tests require Zed's runtime environment
    // They can only be run within Zed editor
    #[test]
    #[ignore]
    fn test_get_latest_version_react() {
        let client = NpmClient::new();
        let result = client.get_latest_version("react");
        assert!(result.is_ok());
        let version = result.unwrap();
        assert!(!version.is_empty());
        println!("React version: {}", version);
    }

    #[test]
    #[ignore]
    fn test_get_latest_version_lodash() {
        let client = NpmClient::new();
        let result = client.get_latest_version("lodash");
        assert!(result.is_ok());
        let version = result.unwrap();
        assert!(!version.is_empty());
        println!("Lodash version: {}", version);
    }

    #[test]
    #[ignore]
    fn test_get_latest_version_nonexistent() {
        let client = NpmClient::new();
        let result = client.get_latest_version("nonexistent-package-12345");
        assert!(result.is_err());
    }
}
