use crate::layouts::StandardAppLayout;
use dioxus::prelude::*;
use views::{
    Home, LoginPage, NotFound, SignupView,
    todo::{TodoListCreateView, TodoListListView},
};
mod components;
mod hooks;
mod layouts;
mod views;

const TAILWIND_CSS: Asset = asset!("/assets/dist/tailwind.css");
const FAVICON: Asset = asset!("/assets/favicon.svg");
pub const ICON: Asset = asset!("/assets/icon.svg");

// !! IMPORTANT: DO NOT FORMAT THIS! The formatting of the routing enum determines behaviour !!
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(StandardAppLayout)]
        #[route("/")]
        Home {},
        #[route("/login")]
        LoginPage {},
        #[route("/signup")]
        SignupView {},

        #[nest("/todo")]
            #[route("/")]
            TodoListListView {},
            #[route("/add")]
            TodoListCreateView {},
        #[end_nest]

        #[route("/:..segments")]
        NotFound { segments: Vec<String> },
}
fn main() -> Result<(), anyhow::Error> {
    info!("Starting Server");

    #[cfg(feature = "server")]
    dioxus::serve(|| async move { api::server::setup_api(App).await });

    #[cfg(not(feature = "server"))]
    dioxus::launch(App);

    #[allow(unreachable_code)]
    Ok(())
}
#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
