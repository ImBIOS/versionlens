use crate::parsers::types::Dependency;
use crate::parsers::PackageParser;

pub struct PubspecParser;

impl PackageParser for PubspecParser {
    fn parse(&self, content: &str) -> Result<Vec<Dependency>, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implement with regex for pubspec.yaml
        Ok(Vec::new())
    }

    fn supports_file(&self, filename: &str) -> bool {
        filename == "pubspec.yaml"
    }
}
