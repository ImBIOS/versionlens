pub mod types;
use types::Dependency;

pub trait PackageParser {
    fn parse(&self, content: &str) -> Result<Vec<Dependency>, Box<dyn std::error::Error + Send + Sync>>;
    fn supports_file(&self, filename: &str) -> bool;
}
