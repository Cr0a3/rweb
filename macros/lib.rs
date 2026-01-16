//! Macros of rusty react
#![warn(missing_docs)]

use proc_macro::TokenStream;

mod component_impl;
mod view_impl;

/// The `view! { ... }` macro is used for providing a simple jsx like
/// api for constructing simple views.
///
/// Example:
/// ```rust
/// view! {
///     <p>Hello World!</p>
/// }
/// ```
#[proc_macro]
pub fn view(input: TokenStream) -> TokenStream {
    view_impl::view(input)
}

/// The `#[component]` attribute is used for creating components
///
/// Example:
/// ```rust
/// #[component]
/// fn HelloWorld() -> impl Node {
///     view! {
///         <p>Hello World!</p>
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn component(_attr: TokenStream, input: TokenStream) -> TokenStream {
    component_impl::component(input)
}
