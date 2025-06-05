use yew::prelude::*;
use gloo_net::http::Request;
use crate::types::user::{User, Page, LoginRequest};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_login: Callback<User>,
    pub on_switch_page: Callback<Page>,
    pub error_message: Option<String>,
}

#[function_component(Login)]
pub fn login(props: &Props) -> Html {
    let username = use_state(String::new);
    let password = use_state(String::new);

    let onsubmit = {
        let username = username.clone();
        let password = password.clone();
        let on_login = props.on_login.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let username = (*username).clone();
            let password = (*password).clone();
            let on_login = on_login.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let login_req = LoginRequest {
                    username: username.clone(),
                    password: password.clone(),
                };

                if let Ok(resp) = Request::post("http://localhost:8080/login")
                    .json(&login_req)
                    .unwrap()
                    .send()
                    .await
                {
                    if resp.status() == 200 {
                        if let Ok(user_data) = resp.json::<User>().await {
                            on_login.emit(user_data);
                        }
                    }
                }
            });
        })
    };

    let onclick_register = {
        let on_switch_page = props.on_switch_page.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            on_switch_page.emit(Page::Register);
        })
    };

    html! {
        <div>
            <h1 class="title">{"Login"}</h1>
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
                        <button class="button is-primary" type="submit">{"Login"}</button>
                    </div>
                    <div class="control">
                        <button
                            class="button is-link"
                            onclick={onclick_register}
                        >
                            {"Register"}
                        </button>
                    </div>
                </div>
            </form>
        </div>
    }
}