use zed_extension_api::http_client::{HttpMethod, HttpRequest};

pub struct RubyGemsClient;

impl RubyGemsClient {
    pub fn new() -> Self {
        Self
    }

    pub fn get_latest_version(&self, gem_name: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("https://rubygems.org/api/v1/gems/{}.json", gem_name);

        let request = HttpRequest::builder()
            .method(HttpMethod::Get)
            .url(&url)
            .build()
            .map_err(|e| format!("Failed to build request: {}", e))?;

        let response = request.fetch().map_err(|e| format!("HTTP request failed: {}", e))?;

        let body = String::from_utf8_lossy(&response.body);

        // Check for error responses (404, etc.)
        if body.contains("\"error\"") {
            let json: serde_json::Value = serde_json::from_str(&body)
                .map_err(|e| format!("Failed to parse JSON: {}", e))?;
            let error_msg = json.get("error")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown error");
            return Err(format!("RubyGems error: {}", error_msg).into());
        }

        let json: serde_json::Value = serde_json::from_str(&body)
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;

        let version = json["version"]
            .as_str()
            .ok_or("No version found")?;

        Ok(version.to_string())
    }
}

impl Default for RubyGemsClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_get_latest_version_rails() {
        let client = RubyGemsClient::new();
        let result = client.get_latest_version("rails");
        assert!(result.is_ok());
        let version = result.unwrap();
        assert!(!version.is_empty());
        println!("Rails version: {}", version);
    }

    #[test]
    #[ignore]
    fn test_get_latest_version_rake() {
        let client = RubyGemsClient::new();
        let result = client.get_latest_version("rake");
        assert!(result.is_ok());
        let version = result.unwrap();
        assert!(!version.is_empty());
        println!("Rake version: {}", version);
    }

    #[test]
    #[ignore]
    fn test_get_latest_version_nonexistent() {
        let client = RubyGemsClient::new();
        let result = client.get_latest_version("nonexistent-gem-xyz123");
        assert!(result.is_err());
    }
}
