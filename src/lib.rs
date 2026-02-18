use zed_extension_api as zed;

mod cache;
mod version;

struct VersionLensExtension;

impl zed::Extension for VersionLensExtension {
    fn new() -> Self {
        Self
    }
}

zed::register_extension!(VersionLensExtension);
