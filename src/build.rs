//! View construction

/// Cache of a build
#[derive(Debug, Default)]
pub struct BuildCache {}

impl BuildCache {
    /// Constructs a new empty buildcache
    pub fn new() -> Self {
        Self::default()
    }
}
