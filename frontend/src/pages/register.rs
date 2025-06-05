use yew::prelude::*;
use gloo_net::http::Request;
use crate::types::user::{Page, RegisterRequest};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_success: Callback<()>,
    pub on_switch_page: Callback<Page>,
    pub error_message: Option<String>,
}

#[function_component(Register)]
pub fn register(props: &Props) -> Html {
    let username = use_state(String::new);
    let email = use_state(String::new);
    let password = use_state(String::new);

    let onsubmit = {
        let username = username.clone();
        let email = email.clone();
        let password = password.clone();
        let on_success = props.on_success.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let username = (*username).clone();
            let email = (*email).clone();
            let password = (*password).clone();
            let on_success = on_success.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let register_req = RegisterRequest {
                    username,
                    email,
                    password,
                };

                if let Ok(resp) = Request::post("http://localhost:8080/register")
                    .json(&register_req)
                    .unwrap()
                    .send()
                    .await
                {
                    if resp.status() == 200 {
                        on_success.emit(());
                    }
                }
            });
        })
    };

    let on_back_click = {
        let on_switch_page = props.on_switch_page.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            on_switch_page.emit(Page::Login);
        })
    };

    html! {
        <div>
            <h1 class="title">{"Register"}</h1>
            <form {onsubmit}>
                <div class="field">
                    <label class="label">{"Username"}</label>
                    <div class="control">
                        <input
                            class="input"
                            type="text"
                            placeholder="Username"
                            onchange={let username = username.clone(); Callback::from(move |e: Event| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                username.set(input.value());
                            })}
                        />
                    </div>
                </div>

                <div class="field">
                    <label class="label">{"Email"}</label>
                    <div class="control">
                        <input
                            class="input"
                            type="email"
                            placeholder="Email"
                            onchange={let email = email.clone(); Callback::from(move |e: Event| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                email.set(input.value());
                            })}
                        />
                    </div>
                </div>

                <div class="field">
                    <label class="label">{"Password"}</label>
                    <div class="control">
                        <input
                            class="input"
                            type="password"
                            placeholder="Password"
                            onchange={let password = password.clone(); Callback::from(move |e: Event| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                password.set(input.value());
                            })}
                        />
                    </div>
                </div>

                <div class="field is-grouped">
                    <div class="control">
                        <button class="button is-primary" type="submit">{"Register"}</button>
                    </div>
                    <div class="control">
                        <button
                            class="button is-link"
                            onclick={on_back_click}
                        >
                            {"Back to Login"}
                        </button>
                    </div>
                </div>
            </form>
        </div>
    }
}