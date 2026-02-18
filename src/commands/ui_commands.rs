/// UI commands for the VersionLens extension.
///
/// These commands handle user interface interactions like toggling
/// inline badges and opening package registry pages.

use crate::state::AppState;

pub struct UiCommands;

impl UiCommands {
    /// "VersionLens: Toggle Inline" - Enable/disable inline badges
    ///
    /// Toggles the display of version information inline in the editor.
    /// This can use Zed settings or in-memory state.
    ///
    /// Returns a message indicating the new state.
    pub fn toggle_inline(state: &AppState) -> String {
        let new_state = state.toggle_inline();
        if new_state {
            "Inline badges enabled".to_string()
        } else {
            "Inline badges disabled".to_string()
        }
    }

    /// "VersionLens: Open Registry" - Open package page in browser
    ///
    /// Opens the package registry page for the package at cursor position.
    /// Returns the URL that should be opened.
    ///
    /// Supported registries:
    /// - npm: https://www.npmjs.com/package/{name}
    /// - crates.io: https://crates.io/crates/{name}
    /// - PyPI: https://pypi.org/project/{name}
    /// - RubyGems: https://rubygems.org/gems/{name}
    /// - pub.dev: https://pub.dev/packages/{name}
    /// - go: https://pkg.go.dev/{name}
    pub fn open_registry(package_name: &str, registry: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let url = match registry {
            "npm" => format!("https://www.npmjs.com/package/{}", package_name),
            "crates.io" => format!("https://crates.io/crates/{}", package_name),
            "pypi" => format!("https://pypi.org/project/{}", package_name),
            "rubygems" => format!("https://rubygems.org/gems/{}", package_name),
            "pub.dev" => format!("https://pub.dev/packages/{}", package_name),
            "go" => format!("https://pkg.go.dev/{}", package_name),
            _ => return Err(format!("Unknown registry: {}", registry).into()),
        };

        // Note: Opening the URL in the browser requires Zed's open_url command.
        // The extension returns the URL, and the caller (via Zed's command system)
        // is responsible for opening it. In Zed, you can use:
        //   zed::open_url(&url)
        // However, this may not be available in all contexts, so we return the URL
        // for the command palette handler to manage.

        Ok(url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::AppState;

    #[test]
    fn test_toggle_inline_enabled() {
        let state = AppState::new();
        assert!(state.is_inline_enabled());

        let result = UiCommands::toggle_inline(&state);
        assert_eq!(result, "Inline badges disabled");
        assert!(!state.is_inline_enabled());
    }

    #[test]
    fn test_toggle_inline_disabled() {
        let state = AppState::new();
        state.toggle_inline(); // Disable first

        let result = UiCommands::toggle_inline(&state);
        assert_eq!(result, "Inline badges enabled");
        assert!(state.is_inline_enabled());
    }

    #[test]
    fn test_open_registry_npm() {
        let result = UiCommands::open_registry("lodash", "npm");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://www.npmjs.com/package/lodash");
    }

    #[test]
    fn test_open_registry_crates() {
        let result = UiCommands::open_registry("serde", "crates.io");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://crates.io/crates/serde");
    }

    #[test]
    fn test_open_registry_pypi() {
        let result = UiCommands::open_registry("requests", "pypi");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://pypi.org/project/requests");
    }

    #[test]
    fn test_open_registry_unknown() {
        let result = UiCommands::open_registry("some-package", "unknown-registry");
        assert!(result.is_err());
    }
}
