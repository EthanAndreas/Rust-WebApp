use yew::prelude::*;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
struct User {
    username: String,
    token: String,
}

#[derive(Clone, Serialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[function_component(App)]
fn app() -> Html {
    let user = use_state(|| None::<User>);
    let username = use_state(String::new);
    let password = use_state(String::new);

    let onsubmit = {
        let username = username.clone();
        let password = password.clone();
        let user = user.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let username = (*username).clone();
            let password = (*password).clone();
            let user = user.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let login_req = LoginRequest {
                    username: username.clone(),
                    password: password.clone(),
                };

                let resp = Request::post("http://localhost:8080/login")
                    .json(&login_req)
                    .unwrap()
                    .send()
                    .await
                    .unwrap();

                if resp.status() == 200 {
                    let user_data: User = resp.json().await.unwrap();
                    user.set(Some(user_data));
                }
            });
        })
    };

    html! {
        <div>
            <nav class="navbar is-primary">
                <div class="navbar-brand">
                    <a class="navbar-item" href="/">
                        {"Rust Web App"}
                    </a>
                </div>
            </nav>

            <div class="auth-container">
                {if user.is_none() {
                    html! {
                        <form onsubmit={onsubmit}>
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

                            <button class="button is-primary" type="submit">{"Login"}</button>
                        </form>
                    }
                } else {
                    html! {
                        <div class="box">
                            <h1 class="title">{"Welcome"}</h1>
                            <p>{format!("Logged in as: {}", user.as_ref().unwrap().username)}</p>
                            <button
                                class="button is-danger"
                                onclick={let user = user.clone(); Callback::from(move |_| user.set(None))}
                            >
                                {"Logout"}
                            </button>
                        </div>
                    }
                }}
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}