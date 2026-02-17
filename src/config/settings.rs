use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub cache_ttl_hours: u32,
    pub inline_enabled_default: bool,
    pub enabled_registries: Vec<String>,
    pub ignore_list: Vec<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            cache_ttl_hours: 24,
            inline_enabled_default: true,
            enabled_registries: vec![
                "npm".to_string(),
                "crates.io".to_string(),
                "pypi".to_string(),
                "rubygems".to_string(),
                "pub.dev".to_string(),
                "go".to_string(),
            ],
            ignore_list: Vec::new(),
        }
    }
}

impl Settings {
    pub fn load_from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let content = fs::read_to_string(path)?;
        let settings: Settings = toml::from_str(&content)?;
        Ok(settings)
    }

    pub fn load_from_directory(dir: &Path) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let ignore_file = dir.join(".versionlens-ignore");
        if ignore_file.exists() {
            let content = fs::read_to_string(&ignore_file)?;
            let ignore_list: Vec<String> = content
                .lines()
                .filter(|line| !line.trim().starts_with('#') && !line.trim().is_empty())
                .map(|line| line.trim().to_string())
                .collect();

            let mut settings = Settings::default();
            settings.ignore_list = ignore_list;
            Ok(settings)
        } else {
            Ok(Settings::default())
        }
    }

    pub fn should_ignore(&self, package: &str) -> bool {
        self.ignore_list.iter().any(|ignored| {
            ignored == package || package.starts_with(ignored)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_default_settings() {
        let settings = Settings::default();
        assert_eq!(settings.cache_ttl_hours, 24);
        assert!(settings.inline_enabled_default);
        assert!(settings.enabled_registries.contains(&"npm".to_string()));
        assert!(settings.ignore_list.is_empty());
    }

    #[test]
    fn test_should_ignore_exact_match() {
        let mut settings = Settings::default();
        settings.ignore_list = vec!["lodash".to_string()];

        assert!(settings.should_ignore("lodash"));
        // lodash-es starts with lodash, so it should be ignored too
        assert!(settings.should_ignore("lodash-es"));
    }

    #[test]
    fn test_should_ignore_prefix_match() {
        let mut settings = Settings::default();
        settings.ignore_list = vec!["@types/".to_string()];

        assert!(settings.should_ignore("@types/node"));
        assert!(!settings.should_ignore("lodash"));
    }

    #[test]
    fn test_load_from_directory_with_ignore_file() {
        let temp_dir = TempDir::new().unwrap();
        let ignore_content = r#"
# Example ignore file
lodash
@types/node
"#;
        let mut file = fs::File::create(temp_dir.path().join(".versionlens-ignore")).unwrap();
        file.write_all(ignore_content.as_bytes()).unwrap();

        let settings = Settings::load_from_directory(temp_dir.path()).unwrap();
        assert!(settings.ignore_list.contains(&"lodash".to_string()));
        assert!(settings.ignore_list.contains(&"@types/node".to_string()));
    }

    #[test]
    fn test_load_from_directory_without_ignore_file() {
        let temp_dir = TempDir::new().unwrap();
        let settings = Settings::load_from_directory(temp_dir.path()).unwrap();
        assert!(settings.ignore_list.is_empty());
    }
}
