use yew::prelude::*;

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <div class="p-4">
            <h1 class="text-3xl font-bold">{ "Contact Us" }</h1>
            <p class="mt-2">{ "Feel free to reach out to us anytime!" }</p>
        </div>
    }
}
