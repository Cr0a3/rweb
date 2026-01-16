//! Signals/States

use std::{
    any::TypeId,
    ops::{Add, Deref, DerefMut},
};

/// Creates a state (state) which can be changed,
/// resulting in a rerender of the child components
/// Example:
/// ```rust
/// #[component]
/// fn Counter() -> impl Component {
///     let counter = use_signal(|| 0);
///
///     view! {
///         <p>Counter: {counter}</p>
///         <button onclick={counter += 1}>+1</button>
///     }
/// }
/// ```
//[NOTE] This function is actually derived by the macro
pub fn use_signal<T: 'static + Sized>(default: fn() -> T) -> Signal<T> {
    Signal {
        default,
        value: None,
        ty: TypeId::of::<T>(),
    }
}

/// A signal/state
pub struct Signal<T> {
    default: fn() -> T,
    pub(crate) value: Option<T>,
    pub(crate) ty: TypeId,
}

impl<T> Signal<T> {
    /// Initialises the signal
    pub fn init(&mut self) {
        self.value = Some((self.default)());
    }
}

impl<T> Deref for Signal<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match &self.value {
            Some(v) => v,
            None => panic!("Tried to use an unitialized signal."),
        }
    }
}

impl<T> DerefMut for Signal<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match &mut self.value {
            Some(v) => v,
            None => panic!("Tried to use an unitialized signal."),
        }
    }
}
