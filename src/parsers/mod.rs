pub mod types;
pub mod npm;
pub mod cargo;
pub mod go;
pub mod pyproject;
pub mod gemfile;
pub mod pubspec;

use types::Dependency;

pub trait PackageParser {
    fn parse(&self, content: &str) -> Result<Vec<Dependency>, Box<dyn std::error::Error + Send + Sync>>;
    fn supports_file(&self, filename: &str) -> bool;
}
