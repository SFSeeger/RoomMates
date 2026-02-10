use crate::components::ErrorDisplay;
use crate::components::contexts::AuthProvider;
use crate::components::ui::dock::Dock;
use crate::components::ui::loader::Loader;
use crate::components::ui::toaster::ToastProvider;
use crate::{
    Route,
    components::ui::{Navbar, sidebar::SidebarProvider},
};
use dioxus::{fullstack::FullstackContext, prelude::*};
use dioxus_free_icons::icons::ld_icons::{LdCircleHelp, LdCircleX};

#[component]
pub fn StandardAppLayout(children: Element) -> Element {
    rsx! {
        AuthProvider {
            SidebarProvider {
                div { class: "min-h-screen flex flex-col",
                    input {
                        id: "drawer-toggle",
                        r#type: "checkbox",
                        class: "drawer-toggle",
                    }
                    Navbar {}
                    ToastProvider {
                        main { class: "grow mx-10 mt-5",
                            ErrorBoundary {
                                handle_error: |error: ErrorContext| {
                                    let http_error = FullstackContext::commit_error_status(error.error().unwrap());
                                    let error_component = match http_error.status {
                                        StatusCode::NOT_FOUND => rsx! {
                                            ErrorDisplay {
                                                title: "Page Not Found",
                                                description: "The page you are looking for does not exist.",
                                                action_text: "Return to Home",
                                                icon: LdCircleHelp,
                                                redirect_route: Route::Home {},
                                                error_context: Some(error),
                                            }
                                        },
                                        StatusCode::UNAUTHORIZED => rsx! {
                                            ErrorDisplay::<LdCircleX> {
                                                title: "Access Denied",
                                                description: "You must be logged in to access this page.",
                                                action_text: "Go to Login",
                                                redirect_route: Route::LoginPage {},
                                                error_context: Some(error),
                                            }
                                        },
                                        StatusCode::FORBIDDEN => rsx! {
                                            ErrorDisplay::<LdCircleX> {
                                                title: "Access Denied",
                                                description: "You do not have permission to access this page.",
                                                action_text: "Go to Home",
                                                redirect_route: Route::Home {},
                                                error_context: Some(error),
                                            }
                                        },
                                        _ => rsx! {
                                            ErrorDisplay {
                                                title: "An unknown error occurred",
                                                description: "Something went wrong while loading the page. Please try again later.",
                                                action_text: "Return to Home",
                                                icon: LdCircleX,
                                                redirect_route: Route::Home {},
                                                error_context: Some(error),
                                            }
                                        },
                                    };
                                    rsx! {
                                        {error_component}
                                    }
                                },
                                SuspenseBoundary {
                                    fallback: |_| {
                                        rsx! {
                                            div { class: "flex items-center justify-center gap-2",
                                                Loader {}
                                                "RoomMates is loading..."
                                            }
                                        }
                                    },
                                    Outlet::<Route> {}
                                }
                            }
                        }
                    }
                }
                Dock {}
            }
        }
    }
}
