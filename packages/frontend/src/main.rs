use crate::components::Navbar;
use dioxus::prelude::*;
use views::Home;
mod components;
mod views;

const FAVICON: Asset = asset!("/assets/favicon.svg");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
}
fn main() {
    #[cfg(feature = "server")]
    dioxus::serve(|| async move { api::server::setup_api(App).await });

    #[cfg(not(feature = "server"))]
    dioxus::launch(App);
}
#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        Router::<Route> {}
    }
}
