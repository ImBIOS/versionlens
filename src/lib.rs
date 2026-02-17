use zed_extension_api as zed;

pub mod cache;
pub mod commands;
pub mod events;
pub mod parsers;
pub mod registry;
pub mod ui;
pub mod version;

pub use cache::FileCache;
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
