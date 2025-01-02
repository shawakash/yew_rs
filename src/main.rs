use yew::prelude::*;

#[function_component]
fn Counter() -> Html {
    let counter = use_state(|| 0);
    let onclick_increment = {
        let counter = counter.clone();
        move |_| {
            counter.set(*counter + 1);
        }
    };

    let onclick_decrement = {
        let counter = counter.clone();
        move |_| {
            if *counter == 0 {
                return;
            }
            counter.set(*counter - 1);
        }
    };

    html! {
        <div class="space-y-4 w-full flex flex-col items-center">
            <div class="flex justify-between w-3/5">
                <button
                    onclick={onclick_increment}
                    class="bg-[#0551cf] text-white px-4 py-2 rounded-md hover:bg-[#0442a7] transition-colors"
                >
                    { "Increment" }
                </button>
                <button
                    onclick={onclick_decrement}
                    class="bg-[#0551cf] text-white px-4 py-2 rounded-md hover:bg-[#0442a7] transition-colors"
                >
                    { "Decrement" }
                </button>
            </div>
            <p class="text-center text-2xl font-bold">{ *counter }</p>
        </div>
    }
}

#[function_component]
fn Hero() -> Html {
    html! {
        <div class="text-center">
            <h1 class="text-4xl font-bold text-gray-800 mb-2">{"Hello, Yew!"}</h1>
            <p class="text-gray-600">{" This is my first application in yew! "}</p>
        </div>
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <main class="min-h-screen bg-[#fafaf7] flex items-center justify-center">
            <div class="max-w-md w-full mx-auto p-8 space-y-8">
                <Hero />
                <Counter />
            </div>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
