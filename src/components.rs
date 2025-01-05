use crate::utils::{
    format_number, format_volume, get_all_tickers, get_price_change_class, get_single_ticker,
};
use crate::Route;
use log;
use web_sys::InputEvent;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn SlugPage() -> Html {
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
pub fn Counter() -> Html {
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
pub fn Hero() -> Html {
    html! {
        <div class="text-center">
            <h1 class="text-4xl font-bold text-gray-800 mb-2">{"Hello, Yew!"}</h1>
            <p class="text-gray-600">{" This is my first application in yew! "}</p>
        </div>
    }
}

#[function_component]
pub fn About() -> Html {
    html! {
        <div class="text-center">
            <h1 class="text-4xl font-bold, text-gray-800 mb-2">{"About"}</h1>
            <p class="text-gray-600">{"This is the about page"}</p>
        </div>
    }
}

#[function_component]
pub fn Nav() -> Html {
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
pub fn Ticker_Page() -> Html {
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
                match get_all_tickers().await {
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
                                            <span class="font-medium">{"$ "}{format_number(&ticker.last_price)}</span>
                                        </div>
                                        <div class="flex justify-between">
                                            <span class="text-gray-400">{"24h High:"}</span>
                                            <span class="font-medium">{"$ "}{format_number(&ticker.high_price)}</span>
                                        </div>
                                        <div class="flex justify-between">
                                            <span class="text-gray-400">{"24h Low:"}</span>
                                            <span class="font-medium">{"$ "}{format_number(&ticker.low_price)}</span>
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

#[derive(Properties, PartialEq)]
pub struct TickerProps {
    pub symbol: String,
    #[prop_or_default]
    pub onclick: Option<Callback<()>>,
}

#[function_component]
pub fn Ticker(props: &TickerProps) -> Html {
    let ticker = use_state(|| None);
    let error = use_state(|| None);
    let loading = use_state(|| true);

    {
        let ticker = ticker.clone();
        let error = error.clone();
        let loading = loading.clone();
        let symbol = props.symbol.clone();

        use_effect_with(props.symbol.clone(), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                match get_single_ticker(&symbol).await {
                    Ok(data) => {
                        ticker.set(Some(data));
                        error.set(None);
                    }
                    Err(err) => error.set(Some(err.to_string())),
                }
                loading.set(false);
            });

            || ()
        });
    }

    let onclick = props.onclick.clone();
    let onclick = Callback::from(move |_| {
        if let Some(callback) = &onclick {
            callback.emit(());
        }
    });

    html! {
        if *loading {
            <div class="flex justify-center">
                <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-gray-800"></div>
            </div>
        } else if let Some(err) = &*error {
            <div class="text-red-500 text-center p-4 bg-red-100 rounded-lg">
                {format!("Error loading {}: {}", props.symbol, err)}
            </div>
        } else if let Some(ticker_data) = &*ticker {
            <div
                onclick={onclick}
                class="border border-gray-400 rounded-lg p-6 shadow-lg hover:shadow-xl transition-shadow cursor-pointer"
            >
                <div class="flex justify-between items-center mb-4">
                    <h2 class="text-xl font-bold">{&ticker_data.symbol}</h2>
                    <span class={get_price_change_class(&ticker_data.price_change_percent)}>
                        {format!("{}%", ticker_data.price_change_percent)}
                    </span>
                </div>
                <div class="space-y-2">
                    <div class="flex justify-between">
                        <span class="text-gray-400">{"Price:"}</span>
                        <span class="font-medium">{"$ "}{format_number(&ticker_data.last_price)}</span>
                    </div>
                    <div class="flex justify-between">
                        <span class="text-gray-400">{"24h High:"}</span>
                        <span class="font-medium">{"$ "}{format_number(&ticker_data.high_price)}</span>
                    </div>
                    <div class="flex justify-between">
                        <span class="text-gray-400">{"24h Low:"}</span>
                        <span class="font-medium">{"$ "}{format_number(&ticker_data.low_price)}</span>
                    </div>
                    <div class="flex justify-between">
                        <span class="text-gray-400">{"Volume:"}</span>
                        <span class="font-medium">{format_volume(&ticker_data.volume)}</span>
                    </div>
                </div>
            </div>
        }
    }
}
