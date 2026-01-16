#![no_main]

use rweb::prelude::*;

#[component]
fn Counter() -> impl Component {
    let counter = use_signal(|| 0);

    view! {
        <p>Counter: {counter}</p>
        <button onclick={counter += 1}>+1</button>
    }
}

#[wasm_bindgen::prelude::wasm_bindgen]
#[no_mangle]
pub fn main() {
    App::new()
        .set_metadata(Metadata {
            title: Some("Example counter".to_owned()),
            ..Default::default()
        })
        .main::<Counter>()
        .serve()
        .expect("Failed to run");
}
