use zed_extension_api::http_client::{HttpMethod, HttpRequest};

pub struct GoClient;

impl GoClient {
    pub fn new() -> Self {
        Self
    }

    /// Fetches the latest version of a Go module from the Go proxy.
    /// The proxy.golang.org API returns a list of versions, we take the last one (latest).
    pub fn get_latest_version(&self, module_name: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // Clean up the module name - remove any version suffix if present
        let clean_name = module_name.split('@').next().unwrap_or(module_name);

        let url = format!("https://proxy.golang.org/{}/@v/list", clean_name);

        let request = HttpRequest::builder()
            .method(HttpMethod::Get)
            .url(&url)
            .build()
            .map_err(|e| format!("Failed to build request: {}", e))?;

        let response = request.fetch().map_err(|e| format!("HTTP request failed: {}", e))?;

        let body = String::from_utf8_lossy(&response.body);

        // The response is a list of versions, one per line
        // Take the last line as it's typically the latest version
        let versions: Vec<&str> = body.lines().collect();

        if versions.is_empty() {
            return Err("No versions found for module".into());
        }

        // Get the last version (usually the latest)
        let latest = versions.last().ok_or("No versions found")?;

        // Remove any go.mod suffix if present (sometimes versions have .mod suffix)
        let version = latest.trim_end_matches(".mod");

        Ok(version.to_string())
    }
}

impl Default for GoClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_get_latest_version_golang_org_x_text() {
        let client = GoClient::new();
        let result = client.get_latest_version("golang.org/x/text");
        assert!(result.is_ok());
        let version = result.unwrap();
        assert!(!version.is_empty());
        println!("golang.org/x/text version: {}", version);
    }

    #[test]
    #[ignore]
    fn test_get_latest_version_github_com_sirupsen_logrus() {
        let client = GoClient::new();
        let result = client.get_latest_version("github.com/sirupsen/logrus");
        assert!(result.is_ok());
        let version = result.unwrap();
        assert!(!version.is_empty());
        println!("sirupsen/logrus version: {}", version);
    }

    #[test]
    #[ignore]
    fn test_get_latest_version_nonexistent() {
        let client = GoClient::new();
        let result = client.get_latest_version("github.com/nonexistent/module/xyz");
        assert!(result.is_err());
    }
}
