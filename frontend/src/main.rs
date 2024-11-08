pub mod routes;
pub mod components;

use yew::{Html, Renderer, html, function_component};
use yew_router::{Switch, BrowserRouter};
use crate::components::home::Home;

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<routes::Route> render={routes::switch} />
            </BrowserRouter>
    }
}

fn main() {
    Renderer::<App>::new().render();
}

