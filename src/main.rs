use yew::prelude::*;
use yew_router::prelude::*;

mod components; // Importa componentes
mod pages;      // Importa páginas

use components::navbar::Navbar;
use components::footer::Footer;
use pages::home::Home;
use pages::about::About;
use pages::contact::Contact;

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
            <div class="min-h-screen flex flex-col">
                <Navbar /> // Navbar reusable
                <main class="flex-grow">
                <Switch<AppRoute> render={switch} />
                </main>
                <Footer /> // Footer reusable
            </div>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
