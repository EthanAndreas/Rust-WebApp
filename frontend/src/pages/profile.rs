use yew::prelude::*;
use crate::types::user::User;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub user: User,
    pub on_logout: Callback<()>,
}

#[function_component(Profile)]
pub fn profile(props: &Props) -> Html {
    html! {
        <div class="box">
            <h1 class="title">{"Profile"}</h1>
            <div class="content">
                <p><strong>{"Username: "}</strong>{props.user.username.clone()}</p>
            </div>
            <div class="field">
                <div class="control">
                    <button
                        class="button is-danger"
                        onclick={let on_logout = props.on_logout.clone();
                            Callback::from(move |_| on_logout.emit(()))}
                    >
                        {"Logout"}
                    </button>
                </div>
            </div>
        </div>
    }
}