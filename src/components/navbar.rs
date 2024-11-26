use yew::prelude::*;
use yew_router::prelude::*;

use crate::AppRoute; // Importa tus rutas definidas

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let location = use_location(); // Obtiene la ruta actual

    let is_active = |route: &AppRoute| {
        location.as_ref().map(|loc| loc.path() == route.to_path()).unwrap_or(false)
    };

    html! {
        <nav class="bg-gray-800">
            <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
                <div class="flex h-16 items-center justify-between">
                    // Logo e ítems principales
                    <div class="flex items-center">
                        <div class="shrink-0">
                            <img class="h-8 w-auto" src="https://tailwindui.com/plus/img/logos/mark.svg?color=indigo&shade=500" alt="Your Company" />
                        </div>
                        <div class="hidden sm:ml-6 sm:block">
                            <div class="flex space-x-4">
                                <Link<AppRoute>
                                    to={AppRoute::Home}
                                    classes={classes!(
                                        "rounded-md",
                                        "px-3",
                                        "py-2",
                                        "text-sm",
                                        "font-medium",
                                        if is_active(&AppRoute::Home) {
                                            "bg-gray-900 text-white"
                                        } else {
                                            "text-gray-300 hover:bg-gray-700 hover:text-white"
                                        }
                                    )}
                                >
                                    { "Home" }
                                </Link<AppRoute>>
                                <Link<AppRoute>
                                    to={AppRoute::About}
                                    classes={classes!(
                                        "rounded-md",
                                        "px-3",
                                        "py-2",
                                        "text-sm",
                                        "font-medium",
                                        if is_active(&AppRoute::About) {
                                            "bg-gray-900 text-white"
                                        } else {
                                            "text-gray-300 hover:bg-gray-700 hover:text-white"
                                        }
                                    )}
                                >
                                    { "About" }
                                </Link<AppRoute>>
                                <Link<AppRoute>
                                    to={AppRoute::Contact}
                                    classes={classes!(
                                        "rounded-md",
                                        "px-3",
                                        "py-2",
                                        "text-sm",
                                        "font-medium",
                                        if is_active(&AppRoute::Contact) {
                                            "bg-gray-900 text-white"
                                        } else {
                                            "text-gray-300 hover:bg-gray-700 hover:text-white"
                                        }
                                    )}
                                >
                                    { "Contact" }
                                </Link<AppRoute>>
                            </div>
                        </div>
                    </div>
                    // Botones y menú del usuario
                    <div class="hidden sm:ml-6 sm:block">
                        <div class="flex items-center">
                            <button type="button" class="relative rounded-full bg-gray-800 p-1 text-gray-400 hover:text-white focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800">
                                <span class="sr-only">{ "View notifications" }</span>
                                <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M14.857 17.082a23.848 23.848 0 0 0 5.454-1.31A8.967 8.967 0 0 1 18 9.75V9A6 6 0 0 0 6 9v.75a8.967 8.967 0 0 1-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 0 1-5.714 0m5.714 0a3 3 0 1 1-5.714 0" />
                                </svg>
                            </button>
                            <div class="relative ml-3">
                                <button type="button" class="relative flex rounded-full bg-gray-800 text-sm focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800">
                                    <span class="sr-only">{ "Open user menu" }</span>
                                    <img class="h-8 w-8 rounded-full" src="https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80" alt="" />
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </nav>
    }
}
