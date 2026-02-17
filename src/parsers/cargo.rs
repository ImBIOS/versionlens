use crate::parsers::types::Dependency;
use crate::parsers::PackageParser;

pub struct CargoParser;

impl PackageParser for CargoParser {
    fn parse(&self, content: &str) -> Result<Vec<Dependency>, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implement with regex for Cargo.toml
        Ok(Vec::new())
    }

    fn supports_file(&self, filename: &str) -> bool {
        filename == "Cargo.toml"
    }
}
