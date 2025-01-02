use yew::prelude::*;

#[function_component]
fn Counter() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            counter.set(*counter + 1);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <h1>{"Hello, Yew!"}</h1>
            <p>{" This is my first application in yew! "}</p>
            <br />
            <Counter />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
