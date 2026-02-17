use crate::parsers::types::Dependency;
use crate::parsers::PackageParser;

pub struct GemfileParser;

impl PackageParser for GemfileParser {
    fn parse(&self, content: &str) -> Result<Vec<Dependency>, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implement with regex for Gemfile
        Ok(Vec::new())
    }

    fn supports_file(&self, filename: &str) -> bool {
        filename == "Gemfile"
    }
}
