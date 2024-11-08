use yew::prelude::*;
#[derive(Properties, PartialEq, Clone)]
pub struct UserProfileProps {
    pub id: String,
}

#[function_component(UserProfile)]
pub fn user_profile(props: &UserProfileProps) -> Html {
    html! { <div>{ format!("User ID:{}", props.id) }</div> }
    }
