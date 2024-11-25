use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="p-4">
            <h1 class="text-3xl font-bold">{ "Welcome to the Home Page!" }</h1>
            <p class="mt-2">{ "This is the main landing page of the SPA." }</p>
            <p>{ "Navigate through the menu to explore other pages." }</p>
        </div>
    }
}
