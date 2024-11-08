use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::{home::Home, post_detail::PostDetail, user_profile::UserProfile};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/post/:id")]
    Post { id: String },
    #[at("/user/:id")]
    UserProfile { id: String },
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Post { id } => html! { <PostDetail id={id} /> },
        Route::UserProfile { id } => html! { <UserProfile id={id} /> },
    }
}


