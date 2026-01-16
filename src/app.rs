//! Implementation of the main web application

use crate::prelude::*;

/// Main webapp
#[derive(Debug, Default)]
pub struct App {
    comps: Vec<StaticComponent>,
    metadata: Metadata,
}

impl App {
    /// Creates a new default app
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the main component
    pub fn main<C>(mut self) -> Self
    where
        C: Component + Default + 'static,
    {
        self.comps.push(StaticComponent(Box::new(C::default())));
        self
    }

    /// Serves the app
    pub fn serve(mut self) {
        // Initialize all signals

        self.comps.iter_mut().for_each(|x| x.0.init_sigs());
    }
}
