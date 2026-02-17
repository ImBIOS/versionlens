use crate::parsers::types::Dependency;
use crate::parsers::PackageParser;

pub struct PyProjectParser;

impl PackageParser for PyProjectParser {
    fn parse(&self, content: &str) -> Result<Vec<Dependency>, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implement with regex for pyproject.toml
        Ok(Vec::new())
    }

    fn supports_file(&self, filename: &str) -> bool {
        filename == "pyproject.toml"
    }
}
