use zed_extension_api as zed;

pub mod parsers;
pub mod registry;
pub mod ui;
pub mod version;

pub use ui::{Badge, BadgeStyle, DecorationManager};
pub use version::comparison::VersionComparator;

struct VersionLensExtension;

impl zed::Extension for VersionLensExtension {
    fn new() -> Self {
        Self
    }
}

zed::register_extension!(VersionLensExtension);
