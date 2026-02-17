use crate::parsers::types::Dependency;
use crate::parsers::PackageParser;

pub struct GoParser;

impl PackageParser for GoParser {
    fn parse(&self, content: &str) -> Result<Vec<Dependency>, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implement with regex for go.mod
        Ok(Vec::new())
    }

    fn supports_file(&self, filename: &str) -> bool {
        filename == "go.mod"
    }
}
