use crate::Route;
use crate::components::ui::toaster::ToastProvider;
use crate::components::ui::{Footer, Navbar, SidebarProvider, loader::Loader};
use crate::views::NotFound;
use dioxus::{fullstack::FullstackContext, prelude::*};
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::LdCircleX;

#[component]
pub fn StandardAppLayout(children: Element) -> Element {
    rsx! {
        SidebarProvider {
            div { class: "min-h-screen flex flex-col",
                input {
                    id: "drawer-toggle",
                    r#type: "checkbox",
                    class: "drawer-toggle",
                }
                Navbar {}
                main { class: "grow mx-10 mt-5",
                    ToastProvider {
                        SuspenseBoundary {
                            fallback: |_| {
                                rsx! {
                                    div { class: "flex items-center justify-center gap-2",
                                        Loader {}
                                        "RoomMates is loading..."
                                    }
                                }
                            },
                            ErrorBoundary {
                                handle_error: |error: ErrorContext| {
                                    let http_error = FullstackContext::commit_error_status(error.error().unwrap());
                                    let error_component = match http_error.status {
                                        StatusCode::NOT_FOUND => rsx! {
                                            NotFound { segments: vec![] }
                                        },
                                        _ => rsx! {
                                            div { class: "flex flex-col items-center gap-4 justify-center h-full text-error",
                                                Icon { class: "size-30", icon: LdCircleX }
                                                h1 { class: "text-2xl font-bold text-center", "An unknown error occurred" }
                                                Link { class: "btn btn-lg btn-outline", to: Route::Home {}, "Return to start" }
                                            }
                                        },
                                    };
                                    rsx! {
                                        {error_component}
                                    }
                                },
                                Outlet::<Route> {}
                            }
                        }
                    }
                }
            }
            Footer {}
        }
    }
}
