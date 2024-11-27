use yew::prelude::*;
use yew::Callback;

#[derive(Clone, PartialEq)]
struct Mesero {
    nombre: String,
    estado: String,
    turno: String,
    telefono: String,
    img_url: String,
}

#[function_component(About)]
pub fn about() -> Html {
    let meseros = use_state(|| vec![
        Mesero {
            nombre: "Juan Pérez".to_string(),
            estado: "Disponible".to_string(),
            turno: "Mañana".to_string(),
            telefono: "5551234567".to_string(),
            img_url: "https://cdn.worldvectorlogo.com/logos/hyperion-6.svg".to_string(),
        },
        Mesero {
            nombre: "María López".to_string(),
            estado: "Descansando".to_string(),
            turno: "Tarde".to_string(),
            telefono: "5557654321".to_string(),
            img_url: "https://cdn.worldvectorlogo.com/logos/hyperion-6.svg".to_string(),
        },
        Mesero {
            nombre: "Carlos Gómez".to_string(),
            estado: "Ocupado".to_string(),
            turno: "Noche".to_string(),
            telefono: "5559876543".to_string(),
            img_url: "https://cdn.worldvectorlogo.com/logos/hyperion-6.svg".to_string(),
        },
    ]);

    let eliminar_mesero = {
        let meseros = meseros.clone();
        Callback::from(move |index: usize| {
            let mut nuevos_meseros = (*meseros).clone();
            nuevos_meseros.remove(index);
            meseros.set(nuevos_meseros);
        })
    };

    let asignar_mesas = {
        let meseros = meseros.clone();
        Callback::from(move |index: usize| {
            let mut nuevos_meseros = (*meseros).clone();
            if let Some(mesero) = nuevos_meseros.get_mut(index) {
                mesero.estado = "Ocupado".to_string();
            }
            meseros.set(nuevos_meseros);
        })
    };

    let agregar_mesero = {
        let meseros = meseros.clone();
        Callback::from(move |_| {
            let mut nuevos_meseros = (*meseros).clone();
            nuevos_meseros.push(Mesero {
                nombre: "Nuevo Mesero".to_string(),
                estado: "Disponible".to_string(),
                turno: "Mañana".to_string(),
                telefono: "5550000000".to_string(),
                img_url: "https://cdn.worldvectorlogo.com/logos/hyperion-6.svg".to_string(),
            });
            meseros.set(nuevos_meseros);
        })
    };

    html! {
        <div class="flex-1 p-8 overflow-y-auto">
            <div class="flex justify-between items-center mb-8">
                <div>
                    <h1 class="text-2xl font-semibold text-gray-800">{"Gestor de Meseros y Mesas"}</h1>
                    <p class="text-sm text-gray-500">{"Administración de personal y asignación de mesas"}</p>
                </div>
                <button
                    class="bg-gray-800 text-white py-2 px-4 rounded-md hover:bg-gray-700 focus:ring focus:ring-indigo-500 shadow-sm"
                    onclick={agregar_mesero}>
                    {"Agregar Mesero"}
                </button>
            </div>

            <div>
                <div class="flex justify-between items-center mb-6">
                    <h2 class="text-xl font-semibold text-gray-800">{"Lista de Meseros"}</h2>
                </div>

                <ul role="list" class="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
                    {for meseros.iter().enumerate().map(|(index, mesero)| html! {
                        <li class="col-span-1 divide-y divide-gray-200 rounded-lg bg-white border border-gray-200 shadow-sm">
                            <div class="flex w-full items-center justify-between space-x-6 p-6">
                                <div class="flex-1 truncate">
                                    <div class="flex items-center space-x-3">
                                        <h3 class="truncate text-lg font-medium text-gray-900">{&mesero.nombre}</h3>
                                        <span class={format!(
                                            "inline-flex shrink-0 items-center rounded-full px-1.5 py-0.5 text-xs font-medium ring-1 ring-inset {}",
                                            match mesero.estado.as_str() {
                                                "Disponible" => "bg-green-50 text-green-700 ring-green-600/20",
                                                "Descansando" => "bg-gray-50 text-gray-700 ring-gray-600/20",
                                                "Ocupado" => "bg-red-50 text-red-700 ring-red-600/20",
                                                _ => "bg-gray-50 text-gray-700 ring-gray-600/20",
                                            }
                                        )}>
                                            {&mesero.estado}
                                        </span>
                                    </div>
                                    <p class="mt-1 truncate text-sm text-gray-500">{format!("Turno: {}", &mesero.turno)}</p>
                                </div>
                                <img class="h-16 w-16 shrink-0 rounded-full bg-gray-300" src={mesero.img_url.clone()} alt="Mesero" />
                            </div>
                            <div>
                                <div class="-mt-px flex divide-x divide-gray-200">
                                    <div class="flex w-0 flex-1">
                                        <button
                                            class="relative -mr-px inline-flex w-0 flex-1 items-center justify-center gap-x-3 rounded-bl-lg border border-transparent py-4 text-sm font-semibold text-gray-900 hover:bg-gray-100"
                                            onclick={asignar_mesas.reform(move |_| index)}>
                                            {"Asignar Mesas"}
                                        </button>
                                    </div>
                                    <div class="-ml-px flex w-0 flex-1">
                                        <button
                                            class="relative inline-flex w-0 flex-1 items-center justify-center gap-x-3 rounded-br-lg border border-transparent py-4 text-sm font-semibold text-gray-900 hover:text-red-500"
                                            onclick={eliminar_mesero.reform(move |_| index)}>
                                            {"Eliminar"}
                                        </button>
                                    </div>
                                </div>
                            </div>
                        </li>
                    })}
                </ul>
            </div>
        </div>
    }
}
