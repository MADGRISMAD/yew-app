use yew::prelude::*;
use yew_router::prelude::*;

mod components; // Importa componentes
mod pages; // Importa páginas

use components::rsidebar::Rsidebar;
use components::sidebar::Sidebar; // Componente Sidebar izquierda // Componente Sidebar derecha

use pages::about::About;
use pages::contact::Contact;
use pages::home::Home;

#[derive(Routable, PartialEq, Clone, Debug)]
enum AppRoute {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/contact")]
    Contact,
}

fn switch(routes: AppRoute) -> Html {
    match routes {
        AppRoute::Home => html! { <Home /> },
        AppRoute::About => html! { <About /> },
        AppRoute::Contact => html! { <Contact /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
            <BrowserRouter>
                <div class="h-full bg-white flex">
                    // Sidebar izquierda
                    <div class="hidden lg:fixed lg:inset-y-0 lg:z-50 lg:flex lg:w-20 lg:flex-col bg-white border-r border-gray-200">
                        <Sidebar />
                    </div>

                    // Contenido principal
                    <main class="flex-1 lg:pl-20 lg:pr-80">
                        <div class="w-full h-full">
                            <Switch<AppRoute> render={switch} />
                        </div>
                    </main>

                    // Sidebar derecha
                    <div class="block lg:fixed lg:inset-y-0 lg:right-0 lg:z-50 lg:w-80 bg-white border-l border-gray-200 p-6 overflow-y-auto">
                            <Rsidebar />
                    </div>


                </div>
            </BrowserRouter>
        }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
