use yew::prelude::*;

#[function_component(Rsidebar)]
pub fn rsidebar() -> Html {
    html! {
        <div class="w-full max-w-full space-y-4">
            <h2 class="text-xl font-semibold mb-4 text-gray-800">{"Pedido #34562"}</h2>
            <div class="flex gap-2 mb-6">
                <button 
                    class="bg-gray-800 text-white rounded-md px-4 py-2"
                >
                    {"Para Llevar"}
                </button>
                <button 
                    class="text-gray-600 border border-gray-400 rounded-md px-4 py-2"
                >
                    {"Envío a Domicilio"}
                </button>
            </div>
            <div class="space-y-4 mb-8">
                <div class="flex items-center">
                    <img src="https://via.placeholder.com/48" alt="Producto 1" class="w-12 h-12 rounded-lg object-cover" />
                    <div class="ml-3 flex-1">
                        <p class="text-sm font-medium text-gray-800">{"Producto 1"}</p>
                        <p class="text-xs text-gray-500">{"€ 29.99"}</p>
                    </div>
                    <span class="text-sm font-medium ml-4">{"€ 29.99"}</span>
                </div>
            </div>
            <div class="space-y-4">
                <div class="flex justify-between text-sm">
                    <span class="text-gray-600">{"Subtotal"}</span>
                    <span class="font-medium">{"€ 69.97"}</span>
                </div>
                <div class="flex justify-between text-sm">
                    <span class="text-gray-600">{"IVA (21%)"}</span>
                    <span class="font-medium">{"€ 14.69"}</span>
                </div>
                <hr class="border-gray-300" />
                <div class="flex justify-between text-base font-semibold">
                    <span>{"Total"}</span>
                    <span>{"€ 84.66"}</span>
                </div>
            </div>
            <button 
                class="w-full mt-6 bg-gray-800 hover:bg-gray-700 text-white py-2 rounded-md"
            >
                {"Finalizar Pedido"}
            </button>
        </div>
    }
}
