use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
struct CacheEntry {
    value: String,
    timestamp: u64,
}

pub struct FileCache {
    cache_dir: PathBuf,
    ttl: Duration,
}

impl FileCache {
    pub fn new(cache_dir: PathBuf, ttl: Duration) -> Self {
        Self { cache_dir, ttl }
    }

    /// Get cached value if not expired
    pub fn get(&self, key: &str) -> Option<String> {
        let path = self.cache_path(key);
        if !path.exists() {
            return None;
        }

        let content = fs::read_to_string(&path).ok()?;
        let entry: CacheEntry = serde_json::from_str(&content).ok()?;

        let age = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .saturating_sub(entry.timestamp);

        if age > self.ttl.as_secs() {
            // Cache expired, remove it
            let _ = fs::remove_file(&path);
            return None;
        }

        Some(entry.value)
    }

    /// Set cached value with TTL
    pub fn set(&self, key: &str, value: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let path = self.cache_path(key);

        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let entry = CacheEntry {
            value: value.to_string(),
            timestamp,
        };

        let content = serde_json::to_string_pretty(&entry)?;
        fs::write(&path, content)?;

        Ok(())
    }

    /// Check if key exists and is valid
    pub fn is_valid(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    /// Clear all cached entries
    pub fn clear(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if self.cache_dir.exists() {
            fs::remove_dir_all(&self.cache_dir)?;
            fs::create_dir_all(&self.cache_dir)?;
        }
        Ok(())
    }

    /// Get cache file path for a key
    /// Key format: {registry}@{package-name}
    /// Output: {cache_dir}/{registry}/{package}.json
    fn cache_path(&self, key: &str) -> PathBuf {
        let parts: Vec<&str> = key.splitn(2, '@').collect();
        if parts.len() == 2 {
            let registry = parts[0];
            let package = parts[1];
            self.cache_dir.join(registry).join(format!("{}.json", package))
        } else {
            // Fallback for invalid key format
            self.cache_dir.join(format!("{}.json", key))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_cache_path_parsing() {
        let cache = FileCache::new(PathBuf::from("/tmp/cache"), Duration::from_secs(3600));
        let path = cache.cache_path("npm@lodash");
        assert!(path.to_string_lossy().contains("npm"));
        assert!(path.to_string_lossy().contains("lodash.json"));
    }

    #[test]
    fn test_cache_operations() {
        let temp_dir = env::temp_dir().join("versionlens_test_cache");
        let cache = FileCache::new(temp_dir.clone(), Duration::from_secs(3600));

        // Test set and get
        cache.set("test@package", "test_value").unwrap();
        assert_eq!(cache.get("test@package"), Some("test_value".to_string()));

        // Test is_valid
        assert!(cache.is_valid("test@package"));
        assert!(!cache.is_valid("nonexistent@package"));

        // Test clear
        cache.clear().unwrap();
        assert!(!cache.is_valid("test@package"));

        // Cleanup
        let _ = fs::remove_dir_all(temp_dir);
    }
}
