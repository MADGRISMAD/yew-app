use yew::prelude::*;

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="flex flex-col min-h-screen bg-gray-100">
                // Header
                <header class="bg-blue-600 text-white p-4">
                    <h1 class="text-3xl font-bold text-center">{ "My Awesome App" }</h1>
                </header>

                // Navigation
                <nav class="bg-blue-500 text-white py-2 px-4">
                    <ul class="flex justify-center space-x-4">
                        <li><a href="#" class="hover:underline">{ "Home" }</a></li>
                        <li><a href="#" class="hover:underline">{ "About" }</a></li>
                        <li><a href="#" class="hover:underline">{ "Services" }</a></li>
                        <li><a href="#" class="hover:underline">{ "Contact" }</a></li>
                    </ul>
                </nav>

                // Main Content
                <main class="flex-grow p-8">
                    <div class="max-w-4xl mx-auto">
                        <div class="bg-white rounded-lg shadow-lg p-6">
                            <h2 class="text-2xl font-bold text-gray-800 mb-4">{ "Welcome to My App" }</h2>
                            <p class="text-gray-700">
                                { "This is a sample page created with Yew and styled using Tailwind CSS. Explore the navigation links above and learn more about what we offer." }
                            </p>
                            <button class="mt-4 px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600">
                                { "Learn More" }
                            </button>
                        </div>
                    </div>
                </main>

                // Footer
                <footer class="bg-blue-600 text-white py-4 text-center">
                    <p>{ "© 2024 My Awesome App. All rights reserved." }</p>
                </footer>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
