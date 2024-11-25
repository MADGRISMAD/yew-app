use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="bg-blue-600 text-white p-4 text-center">
            { "© 2024 My SPA Application. All rights reserved." }
        </footer>
    }
}
