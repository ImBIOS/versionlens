use zed_extension_api as zed;

pub mod parsers;
pub mod registry;

struct VersionLensExtension;

impl zed::Extension for VersionLensExtension {
    fn new() -> Self {
        Self
    }
}

zed::register_extension!(VersionLensExtension);
