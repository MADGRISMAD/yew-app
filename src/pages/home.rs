use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="flex-1 p-8 overflow-y-auto">
            // Encabezado
            <div class="flex justify-between items-center mb-8">
                <div>
                    <h1 class="text-2xl font-semibold text-gray-800">{"Gourmet Delicatessen"}</h1>
                    <p class="text-sm text-gray-500">{"Martes, 2 de Febrero 2023"}</p>
                </div>
                <div class="relative">
                    <span class="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400">{"🔍"}</span>
                    <input
                        class="w-64 pl-10 bg-white border border-gray-300 rounded-md shadow-sm focus:ring focus:ring-indigo-500"
                        placeholder="Buscar productos gourmet..."
                    />
                </div>
            </div>

            // Categorías y vista principal
            <div>
                // Categorías
                <div class="mb-8">
                    <div class="flex space-x-4 mb-6 overflow-x-auto pb-2">
                        {for vec!["Todos", "Carnes", "Pescados", "Quesos", "Embutidos", "Vinos", "Delicatessen"].iter().map(|categoria| html! {
                            <button class="bg-gray-800 text-white px-4 py-2 rounded-md hover:bg-gray-700">{categoria}</button>
                        })}
                    </div>
                    <div class="flex justify-between items-center">
                        <h2 class="text-xl font-semibold text-gray-800">{"Seleccione Productos"}</h2>
                        <button class="text-gray-600 border border-gray-300 px-4 py-2 rounded-md hover:bg-gray-100">
                            {"Para Llevar"} <span class="ml-2">{"▼"}</span>
                        </button>
                    </div>
                </div>

                // Productos
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    {for vec![
                        ("Filete de Res Premium", "Corte fino de res, madurado 28 días", "€29.99", 20),
                        ("Costillas de Cordero", "Costillas de cordero de Nueva Zelanda", "€34.99", 15),
                        ("Salmón Ahumado", "Salmón noruego ahumado en frío", "€24.99", 25)
                    ].iter().map(|(nombre, descripcion, precio, disponible)| html! {
                        <div class="bg-white border border-gray-200 rounded-md shadow-sm p-4">
                            <div class="h-32 bg-gray-200 rounded-md mb-4 flex items-center justify-center">
                                <span class="text-gray-500">{"Imagen"}</span>
                            </div>
                            <h3 class="font-semibold mb-2 text-gray-800">{nombre}</h3>
                            <p class="text-sm text-gray-600 mb-2">{descripcion}</p>
                            <div class="flex justify-between items-center">
                                <p class="text-gray-800 font-semibold">{precio}</p>
                                <p class="text-sm text-gray-500">{format!("{} disponibles", disponible)}</p>
                            </div>
                        </div>
                    })}
                </div>
            </div>
        </div>
    }
}
