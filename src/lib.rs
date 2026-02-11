use std::sync::Arc;
use zed_extension_api as zed;

struct VersionLensExtension;

impl zed::Extension for VersionLensExtension {
    fn start_language_server(
        &self,
        worktree: &zed::Worktree,
        language_server_id: &zed::LanguageServerId,
        _workdir: Option<&std::path::Path>,
    ) -> Result<zed::LanguageServerStart, Box<dyn std::error::Error + Send + Sync>> {
        // VersionLens is primarily an inline overlay, not a language server
        // This is a placeholder for future LSP functionality
        Err("VersionLens does not use a traditional language server".into())
    }
}

zed::register_extension!(VersionLensExtension);
