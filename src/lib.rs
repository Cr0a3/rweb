//! Rusty react

#![warn(missing_docs)]

pub mod app;
pub mod build;
pub mod component;
pub mod signal;
pub mod wasm;

/// Reexport of the most commonly used types
pub mod prelude {
    pub use crate::app::*;
    pub use crate::component::*;
    pub use crate::signal::*;
    pub use macros::*;
    pub use crate::wasm::Metadata;
}
