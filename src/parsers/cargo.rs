use crate::parsers::types::Dependency;
use crate::parsers::PackageParser;
use toml::Value;

pub struct CargoParser;

impl CargoParser {
    pub fn new() -> Self {
        Self
    }
}

impl PackageParser for CargoParser {
    fn parse(&self, content: &str) -> Result<Vec<Dependency>, Box<dyn std::error::Error + Send + Sync>> {
        let mut deps = Vec::new();

        // Parse TOML content
        let toml_value: Value = content.parse()?;

        // Helper function to extract dependencies from a table
        fn extract_deps(value: &Value, section_name: &str, deps: &mut Vec<Dependency>, content: &str) {
            if let Some(table) = value.get(section_name).and_then(|v| v.as_table()) {
                for (name, ver) in table {
                    let version_spec = match ver {
                        Value::String(s) => s.as_str().to_string(),
                        Value::Table(t) => {
                            // Handle table form: name = { version = "x.y.z" }
                            t.get("version")
                                .and_then(|v| v.as_str())
                                .unwrap_or("*")
                                .to_string()
                        }
                        _ => "*".to_string(),
                    };

                    // Find line number by searching for the dependency name
                    let line_number = content
                        .lines()
                        .enumerate()
                        .find(|(_, line)| line.contains(&format!("{} = ", name)))
                        .map(|(i, _)| i + 1)
                        .unwrap_or(1);

                    deps.push(Dependency {
                        name: name.clone(),
                        version_specifier: version_spec,
                        line_number,
                    });
                }
            }
        }

        // Extract dependencies from all sections
        extract_deps(&toml_value, "dependencies", &mut deps, content);
        extract_deps(&toml_value, "dev-dependencies", &mut deps, content);
        extract_deps(&toml_value, "build-dependencies", &mut deps, content);

        Ok(deps)
    }

    fn supports_file(&self, filename: &str) -> bool {
        filename == "Cargo.toml"
    }
}

impl Default for CargoParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cargo_toml() {
        let parser = CargoParser::new();
        let content = r#"
[package]
name = "test"
version = "0.1.0"

[dependencies]
serde = "1.0"
tokio = { version = "1.0", features = ["full"] }

[dev-dependencies]
mockall = "0.11"
"#;

        let result = parser.parse(content);
        assert!(result.is_ok());
        let deps = result.unwrap();
        assert!(deps.len() >= 3);

        // Check that we found serde
        assert!(deps.iter().any(|d| d.name == "serde"));
        assert!(deps.iter().any(|d| d.name == "tokio"));
        assert!(deps.iter().any(|d| d.name == "mockall"));
    }

    #[test]
    fn test_supports_file() {
        let parser = CargoParser::new();
        assert!(parser.supports_file("Cargo.toml"));
        assert!(!parser.supports_file("Cargo.lock"));
        assert!(!parser.supports_file("go.mod"));
    }
}
