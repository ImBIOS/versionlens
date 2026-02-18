use crate::parsers::types::Dependency;
use crate::parsers::PackageParser;
use toml::Value;

pub struct PyProjectParser;

impl PyProjectParser {
    pub fn new() -> Self {
        Self
    }
}

impl PackageParser for PyProjectParser {
    fn parse(&self, content: &str) -> Result<Vec<Dependency>, Box<dyn std::error::Error + Send + Sync>> {
        let mut deps = Vec::new();

        // Parse TOML content
        let toml_value: Value = content.parse()?;

        // Helper to find line number
        fn find_line_number(content: &str, name: &str) -> usize {
            content
                .lines()
                .enumerate()
                .find(|(_, line)| line.contains(name))
                .map(|(i, _)| i + 1)
                .unwrap_or(1)
        }

        // Get the project section (PEP 621 standard)
        if let Some(project) = toml_value.get("project").and_then(|v| v.as_table()) {
            // Extract dependencies - handle both table and array formats

            // Try table format first: dependencies = { package = "version" }
            if let Some(deps_table) = project.get("dependencies").and_then(|v| v.as_table()) {
                for (name, ver) in deps_table {
                    let version_spec = match ver {
                        Value::String(s) => s.as_str().to_string(),
                        Value::Table(t) => {
                            t.get("version")
                                .and_then(|v| v.as_str())
                                .unwrap_or("*")
                                .to_string()
                        }
                        _ => "*".to_string(),
                    };

                    deps.push(Dependency {
                        name: name.clone(),
                        version_specifier: version_spec,
                        line_number: find_line_number(content, name),
                    });
                }
            }

            // Also handle array format: dependencies = ["package>=1.0", "package2"]
            if let Some(deps_array) = project.get("dependencies").and_then(|v| v.as_array()) {
                for item in deps_array {
                    if let Some(dep_str) = item.as_str() {
                        // Parse "package>=version" format
                        let parts: Vec<&str> = dep_str.splitn(2, |c| c == '>' || c == '<' || c == '=' || c == '~' || c == '^').collect();
                        if parts.len() >= 2 {
                            let name = parts[0].trim();
                            let version = dep_str.trim_start_matches(name).trim();
                            deps.push(Dependency {
                                name: name.to_string(),
                                version_specifier: version.to_string(),
                                line_number: find_line_number(content, name),
                            });
                        } else if parts.len() == 1 {
                            // Just package name, no version
                            deps.push(Dependency {
                                name: dep_str.to_string(),
                                version_specifier: "*".to_string(),
                                line_number: find_line_number(content, dep_str),
                            });
                        }
                    }
                }
            }

            // Extract optional-dependencies
            if let Some(opt_deps) = project.get("optional-dependencies").and_then(|v| v.as_table()) {
                for (group_name, table) in opt_deps {
                    // Handle table format
                    if let Some(deps_table) = table.as_table() {
                        for (name, ver) in deps_table {
                            let version_spec = match ver {
                                Value::String(s) => s.as_str().to_string(),
                                Value::Table(t) => {
                                    t.get("version")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("*")
                                        .to_string()
                                }
                                _ => "*".to_string(),
                            };

                            deps.push(Dependency {
                                name: name.clone(),
                                version_specifier: format!("{} [{}]", version_spec, group_name),
                                line_number: find_line_number(content, name),
                            });
                        }
                    }
                    // Handle array format
                    else if let Some(deps_array) = table.as_array() {
                        for item in deps_array {
                            if let Some(dep_str) = item.as_str() {
                                let parts: Vec<&str> = dep_str.splitn(2, |c| c == '>' || c == '<' || c == '=' || c == '~' || c == '^').collect();
                                if parts.len() >= 2 {
                                    let name = parts[0].trim();
                                    let version = dep_str.trim_start_matches(name).trim();
                                    deps.push(Dependency {
                                        name: name.to_string(),
                                        version_specifier: format!("{} [{}]", version, group_name),
                                        line_number: find_line_number(content, name),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        // Also check for legacy [tool.poetry] section
        if let Some(poetry) = toml_value.get("tool").and_then(|v| v.get("poetry")).and_then(|v| v.as_table()) {
            // Handle table format
            if let Some(deps_table) = poetry.get("dependencies").and_then(|v| v.as_table()) {
                for (name, ver) in deps_table {
                    let version_spec = match ver {
                        Value::String(s) => s.as_str().to_string(),
                        Value::Table(t) => {
                            t.get("version")
                                .and_then(|v| v.as_str())
                                .unwrap_or("*")
                                .to_string()
                        }
                        _ => "*".to_string(),
                    };

                    // Only add if not already present
                    if !deps.iter().any(|d| d.name == *name) {
                        deps.push(Dependency {
                            name: name.clone(),
                            version_specifier: version_spec,
                            line_number: find_line_number(content, name),
                        });
                    }
                }
            }
            // Handle array format for poetry
            else if let Some(deps_array) = poetry.get("dependencies").and_then(|v| v.as_array()) {
                for item in deps_array {
                    if let Some(dep_str) = item.as_str() {
                        let parts: Vec<&str> = dep_str.splitn(2, |c| c == '>' || c == '<' || c == '=' || c == '~' || c == '^').collect();
                        if parts.len() >= 2 {
                            let name = parts[0].trim();
                            let version = dep_str.trim_start_matches(name).trim();
                            if !deps.iter().any(|d| d.name == name) {
                                deps.push(Dependency {
                                    name: name.to_string(),
                                    version_specifier: version.to_string(),
                                    line_number: find_line_number(content, name),
                                });
                            }
                        }
                    }
                }
            }
        }

        // Check for legacy [dependencies] section at root level
        if let Some(deps_table) = toml_value.get("dependencies").and_then(|v| v.as_table()) {
            for (name, ver) in deps_table {
                let version_spec = match ver {
                    Value::String(s) => s.as_str().to_string(),
                    Value::Table(t) => {
                        t.get("version")
                            .and_then(|v| v.as_str())
                            .unwrap_or("*")
                            .to_string()
                    }
                    _ => "*".to_string(),
                };

                deps.push(Dependency {
                    name: name.clone(),
                    version_specifier: version_spec,
                    line_number: find_line_number(content, name),
                });
            }
        }

        Ok(deps)
    }

    fn supports_file(&self, filename: &str) -> bool {
        filename == "pyproject.toml"
    }
}

impl Default for PyProjectParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pyproject_toml() {
        let parser = PyProjectParser::new();
        let content = r#"
[project]
name = "myapp"
version = "0.1.0"
dependencies = [
    "requests>=2.28",
    "flask>=2.0",
]

[project.optional-dependencies]
dev = [
    "pytest>=7.0",
    "black>=23.0",
]

[build-system]
requires = ["setuptools>=61.0"]
"#;

        let result = parser.parse(content);
        assert!(result.is_ok());
        let deps = result.unwrap();

        assert!(deps.len() >= 4, "Expected >= 4 deps, got {}", deps.len());
        assert!(deps.iter().any(|d| d.name == "requests"));
        assert!(deps.iter().any(|d| d.name == "flask"));
        assert!(deps.iter().any(|d| d.name == "pytest"));
        assert!(deps.iter().any(|d| d.name == "black"));
    }

    #[test]
    fn test_supports_file() {
        let parser = PyProjectParser::new();
        assert!(parser.supports_file("pyproject.toml"));
        assert!(!parser.supports_file("Cargo.toml"));
    }
}
