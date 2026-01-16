//! Component implementation

use std::any::TypeId;

use crate::build::BuildCache;

/// An web node is required to implement this trait
pub trait Component: std::fmt::Debug {
    /// Renders the component (called onces)
    fn build(&self) -> BuildCache;

    /// Returns the typeid of the component
    fn typeid(&'static self) -> TypeId {
        TypeId::of::<Self>()
    }
}

#[derive(Debug)]
#[doc(hidden)]
pub struct StaticComponent(pub Box<dyn Component>, pub TypeId);

/*
/// Component
struct HelloWorld {
    counter: Signal<u32>,
}

impl Component for HelloWorld {
    fn build(&self) -> BuildCache {
        let html = div()
            .add_child(P::new().add_child("Counter: ").add_child(self.counter.use()))
            .add_child(Button::new().add_child("+1").onclick(|| { self.counter += 1; }))

        let mut cache = BuildCache::new();

        cache.set_html(html);
        cache.add_signal(self.counter);

        cache
    }
}
*/
