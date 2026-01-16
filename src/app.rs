//! Implementation of the main web application

use std::{any::TypeId, collections::HashMap};

use anyhow::bail;

use crate::prelude::*;

/// Main webapp
#[derive(Debug, Default)]
pub struct App {
    comps: Vec<StaticComponent>,
    main: Option<StaticComponent>,
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
        self.main = Some(StaticComponent(Box::new(C::default()), TypeId::of::<C>()));
        self
    }

    /// Registers a usable component
    pub fn register<C>(mut self) -> Self
    where
        C: Component + Default + 'static,
    {
        self.comps
            .push(StaticComponent(Box::new(C::default()), TypeId::of::<C>()));
        self
    }

    /// Updates the metadata
    pub fn set_metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = metadata;
        self
    }

    /// Serves the app
    pub fn serve(self) -> anyhow::Result<()> {
        let Some(main) = self.main else {
            bail!(
                "You need to set the main component `App::new().main::<MainComp>()` to serve the app"
            );
        };

        // Initialize all signals

        let window = web_sys::window().expect("Failed to retrive window");
        let doc = window.document().expect("Failed to retrive document");

        self.metadata.apply(&doc)?;

        let mut comps = HashMap::new();
        self.comps.iter().for_each(|comp| {
            let build = comp.0.build();
            comps.insert(comp.1, build);
        });

        let mut main = main.0.build();
        main.infer(comps);
        main.attach(&doc, "body");

        // ToDo: Listen for callbacks

        Ok(())
    }
}
