use rweb::prelude::*;

#[component]
fn index() -> impl Node {
    rsx! { 
        <p>Hello World!</p>
    }
}

fn main() {
    App::new()
        .add(index)
        .serve();
}