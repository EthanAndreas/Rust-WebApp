use yew::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <nav class="navbar is-primary">
            <div class="navbar-brand">
                <a class="navbar-item" href="/">
                    {"Rust Web App"}
                </a>
            </div>
        </nav>
    }
}