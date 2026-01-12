use rweb::prelude::*;

#[component]
fn Counter() -> impl Component {
    let counter = use_signal(|| 0);

    view! {
        <p>Counter: {counter}</p>
        <button onclick={counter += 1}>+1</button>
    }
}

fn main() {
    App::new().main::<Counter>().serve();
}
