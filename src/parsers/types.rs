// Dependency represents a single dependency entry
#[derive(Debug)]
pub struct Dependency {
    pub name: String,
    pub version_specifier: String,
    pub line_number: usize,
}

// DependencySection enum for different dependency types
pub enum DependencySection {
    Dependencies,
    DevDependencies,
    PeerDependencies,
}
