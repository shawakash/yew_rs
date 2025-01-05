use components::{About, Counter, Hero, Nav, SlugPage, Ticker, Ticker_Page};
use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod utils;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
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
    #[at("/ticker/:symbol")]
    Ticker { symbol: String },
    #[not_found]
    #[at("/404")]
    NotFound,
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
        Route::Ticker { symbol } => html! {
            <Ticker symbol={symbol} />
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
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
