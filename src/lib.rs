use zed_extension_api as zed;

pub mod parsers;
pub mod registry;
pub mod ui;
pub mod version;
pub mod events;

pub use ui::{Badge, BadgeStyle, DecorationManager};
pub use version::comparison::VersionComparator;
pub use events::{Debouncer, BufferWatcher};

struct VersionLensExtension;

impl zed::Extension for VersionLensExtension {
    fn new() -> Self {
        Self
    }
}

zed::register_extension!(VersionLensExtension);
