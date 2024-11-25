use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div class="p-4">
            <h1 class="text-3xl font-bold">{ "About Us" }</h1>
            <p class="mt-2">{ "Learn more about our awesome application." }</p>
        </div>
    }
}
