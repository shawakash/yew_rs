use utils::{format_volume, get_price_change_class, get_ticker_data};
use yew::prelude::*;
use yew_router::prelude::*;

mod utils;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/counter")]
    Counter,
    #[at("/about")]
    About,
    #[at("/:name")]
    Name { name: String },
    #[at("/ticker")]
    TickerPage,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component]
fn SlugPage() -> Html {
    let location = use_location().unwrap();
    let name = location.path().trim_start_matches("/").to_string();

    html! {
        <div class="text-center">
            <h1 class="text-4xl font-bold text-gray-800 mb-2">
                {format!("Dynamic Page: {}", name)}
            </h1>
            <p class="text-gray-600">
                {format!("This is a dynamic page with slug: {}", name)}
            </p>
        </div>
    }
}

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
fn About() -> Html {
    html! {
        <div class="text-center">
            <h1 class="text-4xl font-bold, text-gray-800 mb-2">{"About"}</h1>
            <p class="text-gray-600">{"This is the about page"}</p>
        </div>
    }
}

#[function_component]
fn Nav() -> Html {
    html! {
        <nav class="mb-8">
            <ul class="flex space-x-4 justify-center">
                <li>
                    <Link<Route> to={Route::Home} classes="text-blue-600 hover:text-blue-800">
                        { "Home" }
                    </Link<Route>>
                </li>
                <li>
                    <Link<Route> to={Route::Counter} classes="text-blue-600 hover:text-blue-800">
                        { "Counter" }
                    </Link<Route>>
                </li>
                <li>
                    <Link<Route> to={Route::About} classes="text-blue-600 hover:text-blue-800">
                        { "About" }
                    </Link<Route>>
                </li>
                <li>
                    <Link<Route> to={Route::TickerPage} classes="text-blue-600 hover:text-blue-800">
                        { "Crypto Ticker" }
                    </Link<Route>>
                </li>
                <li>
                    <Link<Route> to={Route::Name { name: "example".to_string() }} classes="text-blue-600 hover:text-blue-800">
                        { "Example Slug" }
                    </Link<Route>>
                </li>
            </ul>
        </nav>
    }
}

#[function_component]
fn Ticker_Page() -> Html {
    let tickers = use_state(|| Vec::new());
    let error = use_state(|| None);
    let loading = use_state(|| true);

    {
        let tickers = tickers.clone();
        let error = error.clone();
        let loading = loading.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                match get_ticker_data().await {
                    Ok(data) => {
                        tickers.set(data);
                        error.set(None);
                    }
                    Err(err) => error.set(Some(err.to_string())),
                }
                loading.set(false);
            });
            || ()
        });
    }

    html! {
        <div class="min-h-screen p-8">
            <h1 class="text-4xl font-bold mb-8 text-center">{"Crypto Market Watch"}</h1>

            if *loading {
                <div class="flex justify-center">
                    <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-white"></div>
                </div>
            } else if let Some(err) = &*error {
                <div class="text-red-500 text-center">
                    {format!("Error: {}", err)}
                </div>
            } else {
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    {
                        (*tickers).iter().map(|ticker| {
                            html! {
                                <div class="border border-gray-400 rounded-lg p-6 shadow-lg hover:shadow-xl transition-shadow">
                                    <div class="flex justify-between items-center mb-4">
                                        <h2 class="text-xl font-bold">{&ticker.symbol}</h2>
                                        <span class={get_price_change_class(&ticker.price_change_percent)}>
                                            {format!("{}%", ticker.price_change_percent)}
                                        </span>
                                    </div>
                                    <div class="space-y-2">
                                        <div class="flex justify-between">
                                            <span class="text-gray-400">{"Price:"}</span>
                                            <span class="font-medium">{format!("${}", ticker.last_price)}</span>
                                        </div>
                                        <div class="flex justify-between">
                                            <span class="text-gray-400">{"24h High:"}</span>
                                            <span class="font-medium">{format!("${}", ticker.high_price)}</span>
                                        </div>
                                        <div class="flex justify-between">
                                            <span class="text-gray-400">{"24h Low:"}</span>
                                            <span class="font-medium">{format!("${}", ticker.low_price)}</span>
                                        </div>
                                        <div class="flex justify-between">
                                            <span class="text-gray-400">{"Volume:"}</span>
                                            <span class="font-medium">{format_volume(&ticker.volume)}</span>
                                        </div>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
            }
        </div>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <div class="max-w-md w-full mx-auto p-8 space-y-8">
                <Hero />
            </div>
        },
        Route::Counter => html! {
            <div class="max-w-md w-full mx-auto p-8 space-y-8">
                <Counter />
            </div>
        },
        Route::About => html! {
            <div class="max-w-md w-full mx-auto p-8 space-y-8">
                <About />
            </div>
        },
        Route::Name { name } => html! {
            <div class="max-w-md w-full mx-auto p-8 space-y-8">
                <SlugPage />
            </div>
        },
        Route::TickerPage => html! {
            <Ticker_Page />
        },
        Route::NotFound => {
            html! { <h1 class="text-center text-lg text-slate-900 ">{ "Upps! This page sweeped over our imagination." }</h1> }
        }
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <main class="min-h-screen bg-[#fafaf7]">
                <Nav />
                <Switch<Route> render={switch} />
            </main>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
