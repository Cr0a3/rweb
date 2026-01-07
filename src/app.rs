//! Implementation of the main web application

use crate::prelude::*;

/// Main webapp
#[derive(Debug, Default)]
pub struct App {
    comps: Vec<StaticComponent>,
}

impl App {
    /// Creates a new default app
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a component bundle to the app
    pub fn add(mut self, bundle: impl IntoIterator<Item = Box<dyn Component>>) -> Self {
        bundle.into_iter().for_each(
            |comp| self.comps.push(StaticComponent(comp))
        );
        self
    }

    /// Serves the app
    pub fn serve(self) {
        todo!()
    }
}