use yew::prelude::*;
use yew_router::prelude::*;

use crate::AppRoute; // Importa las rutas principales

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <nav class="bg-blue-500 text-white p-4">
            <ul class="flex space-x-4">
                <li><Link<AppRoute> to={AppRoute::Home} classes="hover:underline">{ "Home" }</Link<AppRoute>></li>
                <li><Link<AppRoute> to={AppRoute::About} classes="hover:underline">{ "About" }</Link<AppRoute>></li>
                <li><Link<AppRoute> to={AppRoute::Contact} classes="hover:underline">{ "Contact" }</Link<AppRoute>></li>
            </ul>
        </nav>
    }
}
