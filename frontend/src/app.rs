use yew::prelude::*;
use crate::types::user::{User, Page};
use crate::components::{navbar::Navbar, error_message::ErrorMessage};
use crate::pages::{login::Login, register::Register, profile::Profile};

#[function_component(App)]
pub fn app() -> Html {
    let user = use_state(|| None::<User>);
    let current_page = use_state(|| Page::Login);
    let error_message = use_state(|| None::<String>);

    let handle_login = {
        let user = user.clone();
        let current_page = current_page.clone();
        let error_message = error_message.clone();

        Callback::from(move |logged_user: User| {
            error_message.set(None);
            user.set(Some(logged_user));
            current_page.set(Page::Profile);
        })
    };

    let handle_page_switch = {
        let current_page = current_page.clone();
        let error_message = error_message.clone();

        Callback::from(move |page: Page| {
            error_message.set(None);
            current_page.set(page);
        })
    };

    let handle_register_success = {
        let current_page = current_page.clone();

        Callback::from(move |_| {
            current_page.set(Page::Login);
        })
    };

    let handle_logout = {
        let user = user.clone();
        let current_page = current_page.clone();

        Callback::from(move |_| {
            user.set(None);
            current_page.set(Page::Login);
        })
    };

    html! {
        <div>
            <Navbar />
            <div class="auth-container">
                <ErrorMessage message={(*error_message).clone()} />
                {
                    if let Some(current_user) = (*user).clone() {
                        match *current_page {
                            Page::Profile => html! {
                                <Profile
                                    user={current_user}
                                    on_logout={handle_logout}
                                />
                            },
                            _ => {
                                current_page.set(Page::Profile);
                                html! {}
                            }
                        }
                    } else {
                        match *current_page {
                            Page::Login => html! {
                                <Login
                                    on_login={handle_login}
                                    on_switch_page={handle_page_switch}
                                    error_message={(*error_message).clone()}
                                />
                            },
                            Page::Register => html! {
                                <Register
                                    on_success={handle_register_success}
                                    on_switch_page={handle_page_switch}
                                    error_message={(*error_message).clone()}
                                />
                            },
                            Page::Profile => {
                                current_page.set(Page::Login);
                                html! {}
                            }
                        }
                    }
                }
            </div>
        </div>
    }
}