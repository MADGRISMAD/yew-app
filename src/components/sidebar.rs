use yew::prelude::*;
use yew_router::prelude::*;

use crate::AppRoute;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let location = use_location();

    let is_active = |route: &AppRoute| {
        location
            .as_ref()
            .map(|loc| loc.path() == route.to_path())
            .unwrap_or(false)
    };

    html! {
        <div class={classes!("flex", "grow", "flex-col", "gap-y-5", "overflow-y-auto", "bg-white", "px-6")}>
            <div class={classes!("flex", "h-16", "shrink-0", "items-center")}>
                <img class={classes!("h-8", "w-auto")} src="https://tailwindui.com/plus/img/logos/mark.svg?color=indigo&shade=500" alt="Your Company" />
            </div>
            <nav class={classes!("flex", "flex-1", "flex-col")}>
                <ul role="list" class={classes!("flex", "flex-1", "flex-col", "gap-y-7")}>
                    <li>
                        <ul role="list" class={classes!("-mx-2", "space-y-1")}>
                            <li>
                                <Link<AppRoute>
                                    to={AppRoute::Home}
                                    classes={classes!(
                                        "group", "flex", "justify-center", "rounded-md", "p-2",
                                        if is_active(&AppRoute::Home) {
                                            "bg-gray-100 text-gray-900"
                                        } else {
                                            "text-gray-400 hover:bg-gray-100 hover:text-gray-900"
                                        }
                                    )}
                                >
                                    <svg class={classes!("w-6", "h-6")} fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="m2.25 12 8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25" />
                                    </svg>
                                </Link<AppRoute>>
                            </li>
                            <li>
                                <Link<AppRoute>
                                    to={AppRoute::About}
                                    classes={classes!(
                                        "group", "flex", "justify-center", "rounded-md", "p-2",
                                        if is_active(&AppRoute::About) {
                                            "bg-gray-100 text-gray-900"
                                        } else {
                                            "text-gray-400 hover:bg-gray-100 hover:text-gray-900"
                                        }
                                    )}
                                >
                                    <svg class={classes!("w-6", "h-6")} fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M15 19.128a9.38 9.38 0 0 0 2.625.372 9.337 9.337 0 0 0 4.121-.952 4.125 4.125 0 0 0-7.533-2.493M15 19.128v-.003c0-1.113-.285-2.16-.786-3.07M15 19.128v.106A12.318 12.318 0 0 1 8.624 21c-2.331 0-4.512-.645-6.374-1.766l-.001-.109a6.375 6.375 0 0 1 11.964-3.07M12 6.375a3.375 3.375 0 1 1-6.75 0 3.375 3.375 0 0 1 6.75 0Zm8.25 2.25a2.625 2.625 0 1 1-5.25 0 2.625 2.625 0 0 1 5.25 0Z" />
                                    </svg>
                                </Link<AppRoute>>
                            </li>
                        </ul>
                    </li>
                </ul>
            </nav>
        </div>
    }
}
