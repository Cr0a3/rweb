//! Rusty react

#![warn(missing_docs)]

pub mod app;
pub mod component;

/// Reexport of the most commonly used types
pub mod prelude {
    pub use crate::app::*;
    pub use crate::component::*;
}