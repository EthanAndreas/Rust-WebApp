use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub message: Option<String>,
}

#[function_component(ErrorMessage)]
pub fn error_message(props: &Props) -> Html {
    if let Some(error) = &props.message {
        html! {
            <div class="notification is-danger">
                {error}
            </div>
        }
    } else {
        html! {}
    }
}