use zed_extension_api::http_client::{HttpMethod, HttpRequest};

pub struct PyPIClient;

impl PyPIClient {
    pub fn new() -> Self {
        Self
    }

    pub fn get_latest_version(&self, package_name: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("https://pypi.org/pypi/{}/json", package_name);

        let request = HttpRequest::builder()
            .method(HttpMethod::Get)
            .url(&url)
            .build()
            .map_err(|e| format!("Failed to build request: {}", e))?;

        let response = request.fetch().map_err(|e| format!("HTTP request failed: {}", e))?;

        let body = String::from_utf8_lossy(&response.body);

        let json: serde_json::Value = serde_json::from_str(&body)
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;

        // Check for PyPI error responses
        if json.get("error").is_some() {
            let error_msg = json.get("error")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown error");
            return Err(format!("PyPI error: {}", error_msg).into());
        }

        let version = json["info"]["version"]
            .as_str()
            .ok_or("No version found")?;

        Ok(version.to_string())
    }
}

impl Default for PyPIClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_get_latest_version_requests() {
        let client = PyPIClient::new();
        let result = client.get_latest_version("requests");
        assert!(result.is_ok());
        let version = result.unwrap();
        assert!(!version.is_empty());
        println!("Requests version: {}", version);
    }

    #[test]
    #[ignore]
    fn test_get_latest_version_flask() {
        let client = PyPIClient::new();
        let result = client.get_latest_version("flask");
        assert!(result.is_ok());
        let version = result.unwrap();
        assert!(!version.is_empty());
        println!("Flask version: {}", version);
    }

    #[test]
    #[ignore]
    fn test_get_latest_version_nonexistent() {
        let client = PyPIClient::new();
        let result = client.get_latest_version("nonexistent-package-xyz123");
        assert!(result.is_err());
    }
}
