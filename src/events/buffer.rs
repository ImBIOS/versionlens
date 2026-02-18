use std::collections::HashMap;
use std::sync::Mutex;

use crate::parsers::PackageParser;
use crate::parsers::npm::NpmParser;
use crate::parsers::cargo::CargoParser;
use crate::parsers::go::GoParser;
use crate::parsers::pyproject::PyProjectParser;
use crate::parsers::gemfile::GemfileParser;
use crate::parsers::pubspec::PubspecParser;
use crate::parsers::types::Dependency;
use crate::registry::NpmClient;
use crate::ui::DecorationManager;
use crate::ui::badges::{Badge, BadgeStyle};
use crate::version::comparison::VersionComparator;
use super::debouncer::Debouncer;

/// BufferWatcher monitors buffer changes and updates version badges
///
/// This is the main integration point that ties together:
/// - Package parsers (detecting and parsing dependency files)
/// - Registry clients (fetching latest versions)
/// - Version comparators (checking if updates are available)
/// - UI decorations (displaying badges in the editor)
///
/// Note: This implementation uses string-based file paths for compatibility
/// with Zed's extension API. The actual Zed buffer integration happens in the
/// extension's callback handlers which translate Zed types to these interfaces.
pub struct BufferWatcher {
    /// Parser instances for different package ecosystems
    parsers: Vec<Box<dyn PackageParser>>,
    /// Registry client for fetching latest versions
    npm_client: NpmClient,
    /// Version comparator for semantic version comparison
    version_comparator: VersionComparator,
    /// Decoration manager for displaying badges
    decoration_manager: DecorationManager,
    /// Debouncer to prevent excessive API calls
    debouncer: Debouncer,
    /// Cache of latest versions to avoid redundant API calls
    version_cache: Mutex<HashMap<String, String>>,
    /// Supported file extensions mapped to parser indices
    supported_files: HashMap<String, usize>,
}

impl BufferWatcher {
    /// Create a new BufferWatcher
    pub fn new() -> Self {
        let mut parsers: Vec<Box<dyn PackageParser>> = Vec::new();
        let mut supported_files = HashMap::new();

        // Register NPM parser
        let npm_idx = parsers.len();
        supported_files.insert("package.json".to_string(), npm_idx);
        parsers.push(Box::new(NpmParser));

        // Register Cargo parser
        let cargo_idx = parsers.len();
        supported_files.insert("Cargo.toml".to_string(), cargo_idx);
        parsers.push(Box::new(CargoParser));

        // Register Go parser
        let go_idx = parsers.len();
        supported_files.insert("go.mod".to_string(), go_idx);
        parsers.push(Box::new(GoParser));

        // Register PyProject parser
        let pyproject_idx = parsers.len();
        supported_files.insert("pyproject.toml".to_string(), pyproject_idx);
        parsers.push(Box::new(PyProjectParser));

        // Register Gemfile parser
        let gemfile_idx = parsers.len();
        supported_files.insert("Gemfile".to_string(), gemfile_idx);
        parsers.push(Box::new(GemfileParser));

        // Register Pubspec parser
        let pubspec_idx = parsers.len();
        supported_files.insert("pubspec.yaml".to_string(), pubspec_idx);
        supported_files.insert("pubspec.yml".to_string(), pubspec_idx);
        parsers.push(Box::new(PubspecParser));

        Self {
            parsers,
            npm_client: NpmClient::new(),
            version_comparator: VersionComparator::new(),
            decoration_manager: DecorationManager::new(),
            debouncer: Debouncer::new(500), // 500ms debounce
            version_cache: Mutex::new(HashMap::new()),
            supported_files,
        }
    }

    /// Handle buffer change event
    ///
    /// Called when a buffer is opened or modified. This is the main entry point
    /// for processing file changes.
    ///
    /// # Arguments
    /// * `file_path` - The path to the file that changed
    /// * `content` - The content of the buffer
    pub fn on_buffer_change(&mut self, file_path: &str, content: &str) {
        // Extract filename from path
        let filename = std::path::Path::new(file_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");

        // Check if this is a supported package file
        if !self.is_package_file(filename) {
            return;
        }

        // Debounce to avoid excessive processing
        if !self.debouncer.should_proceed(file_path) {
            return;
        }

        // Parse dependencies
        let dependencies = match self.parse_dependencies(filename, content) {
            Ok(deps) => deps,
            Err(e) => {
                eprintln!("Failed to parse dependencies: {}", e);
                return;
            }
        };

        // Process each dependency
        let mut badges = Vec::new();

        for dep in dependencies {
            // Try to get cached version first
            let latest_version = match self.get_cached_or_fetch_version(&dep.name) {
                Ok(version) => version,
                Err(e) => {
                    eprintln!("Failed to get latest version for {}: {}", dep.name, e);
                    continue;
                }
            };

            // Compare versions
            let comparison = self.version_comparator.compare(&dep.version_specifier, &latest_version);

            // Create badge based on comparison
            let badge = self.create_badge_from_comparison(&dep.name, &latest_version, dep.line_number, &comparison);
            badges.push(badge);
        }

        // Update decorations
        self.decoration_manager.update_badges(badges, file_path);
    }

    /// Handle buffer change from a file path string
    ///
    /// This is a convenience method that reads the file from disk.
    /// Use this when you have a file path and want to process it.
    ///
    /// # Arguments
    /// * `file_path` - Path to the file to process
    pub fn on_file_change(&mut self, file_path: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Read file content
        let content = std::fs::read_to_string(file_path)?;
        self.on_buffer_change(file_path, &content);
        Ok(())
    }

    /// Check if a filename is a supported package file
    pub fn is_package_file(&self, filename: &str) -> bool {
        self.supported_files.contains_key(filename)
    }

    /// Parse dependencies from buffer content using the appropriate parser
    fn parse_dependencies(&self, filename: &str, content: &str) -> Result<Vec<Dependency>, Box<dyn std::error::Error + Send + Sync>> {
        // Find the parser for this file type
        let parser_idx = self.supported_files.get(filename);

        if let Some(&idx) = parser_idx {
            if let Some(parser) = self.parsers.get(idx) {
                return parser.parse(content);
            }
        }

        Err("No parser found for file".into())
    }

    /// Get version from cache or fetch from registry
    fn get_cached_or_fetch_version(&self, package_name: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // Check cache first
        {
            let cache = self.version_cache.lock().unwrap();
            if let Some(version) = cache.get(package_name) {
                return Ok(version.clone());
            }
        }

        // Fetch from registry (only npm for now)
        let version = self.npm_client.get_latest_version(package_name)?;

        // Update cache
        {
            let mut cache = self.version_cache.lock().unwrap();
            cache.insert(package_name.to_string(), version.clone());
        }

        Ok(version)
    }

    /// Create a badge from a version comparison result
    fn create_badge_from_comparison(
        &self,
        package_name: &str,
        latest_version: &str,
        line_number: usize,
        comparison: &crate::version::comparison::VersionComparison,
    ) -> Badge {
        let style = match comparison {
            crate::version::comparison::VersionComparison::UpToDate { .. } => BadgeStyle::UpToDate,
            crate::version::comparison::VersionComparison::Outdated { diff, .. } => {
                match diff {
                    crate::version::comparison::VersionDiff::Major => BadgeStyle::MajorDiff,
                    _ => BadgeStyle::Outdated,
                }
            }
            crate::version::comparison::VersionComparison::Error(_) => BadgeStyle::Outdated,
        };

        let text = comparison.display_text();

        Badge::new(text, style, line_number, package_name.to_string())
    }

    /// Clear badges for a buffer
    pub fn clear_buffer(&mut self, file_path: &str) {
        self.decoration_manager.clear_badges(file_path);
        self.debouncer.reset(file_path);
    }

    /// Clear all badges and caches
    pub fn clear_all(&mut self) {
        self.decoration_manager.clear_all();
        self.debouncer.clear();
        self.version_cache.lock().unwrap().clear();
    }

    /// Get decoration manager reference for external access
    pub fn decoration_manager(&self) -> &DecorationManager {
        &self.decoration_manager
    }

    /// Get mutable decoration manager reference
    pub fn decoration_manager_mut(&mut self) -> &mut DecorationManager {
        &mut self.decoration_manager
    }

    /// Check if a buffer has active badges
    pub fn has_active_badges(&self, file_path: &str) -> bool {
        self.decoration_manager.has_badges(file_path)
    }

    /// Get badge count for a buffer
    pub fn badge_count(&self, file_path: &str) -> usize {
        self.decoration_manager.badge_count(file_path)
    }

    /// Get cached version for a package (for testing)
    #[allow(dead_code)]
    pub fn get_cached_version(&self, package_name: &str) -> Option<String> {
        let cache = self.version_cache.lock().unwrap();
        cache.get(package_name).cloned()
    }

    /// Manually refresh versions for a buffer
    pub fn refresh_buffer(&mut self, file_path: &str, content: &str) {
        // Clear cache for this buffer's file to force refresh
        self.debouncer.reset(file_path);

        // Reprocess
        self.on_buffer_change(file_path, content);
    }

    /// Process a single dependency and return its badge
    ///
    /// This is useful for testing individual dependencies.
    pub fn process_dependency(&mut self, package_name: &str, current_version: &str, line_number: usize) -> Result<Badge, Box<dyn std::error::Error + Send + Sync>> {
        // Get latest version
        let latest_version = self.get_cached_or_fetch_version(package_name)?;

        // Compare versions
        let comparison = self.version_comparator.compare(current_version, &latest_version);

        // Create badge
        Ok(self.create_badge_from_comparison(package_name, &latest_version, line_number, &comparison))
    }

    /// Get all supported file types
    pub fn supported_file_types(&self) -> Vec<&'static str> {
        vec![
            "package.json",
            "Cargo.toml",
            "go.mod",
            "pyproject.toml",
            "Gemfile",
            "pubspec.yaml",
            "pubspec.yml",
        ]
    }

    /// Get the parser for a specific file type
    #[allow(dead_code)]
    pub fn get_parser_for_file(&self, filename: &str) -> Option<&dyn PackageParser> {
        self.supported_files.get(filename).and_then(|&idx| self.parsers.get(idx).map(|p| p.as_ref()))
    }
}

impl Default for BufferWatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_watcher_creation() {
        let watcher = BufferWatcher::new();
        assert!(watcher.is_package_file("package.json"));
        assert!(watcher.is_package_file("Cargo.toml"));
        assert!(!watcher.is_package_file("README.md"));
    }

    #[test]
    fn test_is_package_file() {
        let watcher = BufferWatcher::new();

        assert!(watcher.is_package_file("package.json"));
        assert!(watcher.is_package_file("Cargo.toml"));
        assert!(watcher.is_package_file("go.mod"));
        assert!(watcher.is_package_file("pyproject.toml"));
        assert!(watcher.is_package_file("Gemfile"));
        assert!(watcher.is_package_file("pubspec.yaml"));

        assert!(!watcher.is_package_file("package-lock.json"));
        assert!(!watcher.is_package_file("Cargo.lock"));
        assert!(!watcher.is_package_file("random.txt"));
    }

    #[test]
    fn test_clear_all() {
        let mut watcher = BufferWatcher::new();

        // Add some cache entries
        watcher.version_cache.lock().unwrap().insert("test".to_string(), "1.0.0".to_string());

        // Clear all
        watcher.clear_all();

        // Cache should be empty
        let cache = watcher.version_cache.lock().unwrap();
        assert!(cache.is_empty());
    }

    #[test]
    fn test_supported_file_types() {
        let watcher = BufferWatcher::new();
        let files = watcher.supported_file_types();

        assert!(files.contains(&"package.json"));
        assert!(files.contains(&"Cargo.toml"));
        assert!(files.contains(&"go.mod"));
    }

    #[test]
    fn test_get_parser_for_file() {
        let watcher = BufferWatcher::new();

        let parser = watcher.get_parser_for_file("package.json");
        assert!(parser.is_some());

        let parser = watcher.get_parser_for_file("unknown.json");
        assert!(parser.is_none());
    }
}
