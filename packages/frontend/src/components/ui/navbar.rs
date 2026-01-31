use crate::Route;
use crate::components::contexts::AuthState;
use crate::components::ui::sidebar::SidebarState;
use crate::components::ui::theme_controller::ThemeController;
use api::routes::users::logout;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::LdMenu;

#[component]
pub fn Navbar() -> Element {
    let mut sidebar_state = use_context::<SidebarState>();
    let auth_state = use_context::<AuthState>();
    let auth_state_clone = auth_state.clone();
    let nav = navigator();

    rsx! {
        div { class: "navbar bg-base-100 shadow-sm",
            button {
                class: "btn btn-ghost px-2 py-1 drawer-button",
                onclick: move |_| sidebar_state.visible.toggle(),
                Icon { class: "size-6", icon: LdMenu }
            }
            div { class: "flex-1",
                Link { to: Route::Home {}, class: "btn btn-ghost text-xl", "RoomMates" }
            }
            div { class: "flex-none flex gap-2 items-center",
                div { class: "hidden md:block",
                    ThemeController { id_extra: "navbar" }
                }
                if let Some(user) = auth_state.user.read().as_ref() {
                    div { class: "dropdown dropdown-end",
                        div {
                            class: "btn btn-ghost btn-circle avatar",
                            role: "button",
                            tabindex: "0",
                            div { class: "w-10 rounded-full",
                                img {
                                    alt: "Profile Picture",
                                    src: format!("https://api.dicebear.com/9.x/bottts/avif?seed={}", user.id),
                                }
                            }
                        }
                        ul {
                            class: "menu menu-sm dropdown-content bg-base-100 rounded-box z-1 mt-3 w-42 p-2 shadow",
                            tabindex: "-1",
                            li {
                                Link { to: Route::Home {}, "Profile" }
                            }
                            li {
                                Link { to: Route::Home {}, "Settings" }
                            }
                            li {
                                button {
                                    onclick: move |_| {
                                        let mut auth_state_clone = auth_state_clone.clone();
                                        async move {
                                            let _ = logout().await;
                                            auth_state_clone.logout();
                                            nav.push(Route::Home {});
                                        }
                                    },
                                    "Logout"
                                }
                            }
                        }
                    }
                } else {
                    Link {
                        to: Route::SignupView {},
                        class: "btn btn-secondary btn-sm mx-1",
                        "Sign Up"
                    }
                    Link {
                        to: Route::LoginPage {},
                        class: "btn btn-primary btn-sm mx-1",
                        "Login"
                    }
                }
            }
        }
    }
}
