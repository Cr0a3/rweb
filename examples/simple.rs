use rweb::prelude::*;

#[component]
fn index() -> impl Node {
    rsx! { 
        <p>Hello World!</p>
    }
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn main() {
    App::new()
        .add(index)
        .serve();
}