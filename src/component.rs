//! Component implementation

/// An web node is required to implement this trait
pub trait Component: std::fmt::Debug {

}

impl Component for () {

}

#[derive(Debug)]
#[doc(hidden)]
pub struct StaticComponent(pub Box<dyn Component>);