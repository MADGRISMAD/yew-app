use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub label: String,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    html! {
        <button
            class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
            onclick={props.onclick.clone()}
        >
            { &props.label }
        </button>
    }
}
