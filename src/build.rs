//! View construction

use std::{any::TypeId, collections::HashMap};

use web_sys::Document;

/// Cache of a build
#[derive(Debug, Default)]
pub struct BuildCache {}

impl BuildCache {
    /// Constructs a new empty buildcache
    pub fn new() -> Self {
        Self::default()
    }

    /// Infers the children components
    pub fn infer(&mut self, _comps: HashMap<TypeId, BuildCache>) {
        todo!()
    }

    /// Attaches the build to the wasm document
    pub fn attach(&self, _doc: &Document, _parent: &str) {
        todo!()
    }
}
