use yew::prelude::*;
#[derive(Properties, PartialEq, Clone)]
pub struct PostDetailProps {
    pub id: String,
}

#[function_component(PostDetail)]
pub fn post_detail(props: &PostDetailProps) -> Html {
    html! { <div>{ format! ("Post ID:{}", props.id) }</div> }
    }
