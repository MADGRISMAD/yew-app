use yew::prelude::*;

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <div class="bg-gray-900">
            <div class="absolute inset-x-0 top-4 -z-10 flex justify-center overflow-hidden blur-3xl" aria-hidden="true">
                <div class="aspect-[1108/632] w-[69.25rem] flex-none bg-gradient-to-r from-[#80caff] to-[#4f46e5] opacity-25"
                     style="clip-path: polygon(73.6% 51.7%, 91.7% 11.8%, 100% 46.4%, 97.4% 82.2%, 92.5% 84.9%, 75.7% 64%, 55.3% 47.5%, 46.5% 49.4%, 45% 62.9%, 50.3% 87.2%, 21.3% 64.1%, 0.1% 100%, 5.4% 51.1%, 21.4% 63.9%, 58.9% 0.2%, 73.6% 51.7%)"></div>
            </div>

            <div class="px-6 pt-14 lg:px-8">
                <div class="mx-auto max-w-2xl pt-24 text-center sm:pt-40">
                    <h1 class="text-5xl font-semibold tracking-tight text-white sm:text-7xl">{ "Contact Us" }</h1>
                    <p class="mt-8 text-lg text-gray-400 sm:text-xl">
                        { "Feel free to reach out to us for any inquiries or assistance!" }
                    </p>
                </div>
            </div>

            <div class="mt-20 px-6 lg:px-8">
                <div class="mx-auto max-w-2xl">
                    <form class="space-y-6">
                        <div>
                            <label for="name" class="block text-sm font-medium text-gray-300">{ "Name" }</label>
                            <div class="mt-2">
                                <input id="name" name="name" type="text" required={true}
                                       class="block w-full rounded-md bg-gray-800 text-gray-300 shadow-sm focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm" />
                            </div>
                        </div>
                        <div>
                            <label for="email" class="block text-sm font-medium text-gray-300">{ "Email" }</label>
                            <div class="mt-2">
                                <input id="email" name="email" type="email" required={true}
                                       class="block w-full rounded-md bg-gray-800 text-gray-300 shadow-sm focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm" />
                            </div>
                        </div>
                        <div>
                            <label for="message" class="block text-sm font-medium text-gray-300">{ "Message" }</label>
                            <div class="mt-2">
                                <textarea id="message" name="message" rows="4" required={true}
                                          class="block w-full rounded-md bg-gray-800 text-gray-300 shadow-sm focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"></textarea>
                            </div>
                        </div>
                        <div>
                            <button type="submit"
                                    class="w-full rounded-md bg-indigo-500 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-indigo-400 focus:outline-none focus:ring-2 focus:ring-indigo-500">
                                { "Send Message" }
                            </button>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    }
}
