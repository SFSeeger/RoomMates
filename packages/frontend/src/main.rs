use crate::components::Navbar;
use dioxus::prelude::*;
use views::Home;
mod components;
mod views;

const TAILWIND_CSS: Asset = asset!("/assets/dist/tailwind.css");
const FAVICON: Asset = asset!("/assets/favicon.svg");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
}
fn main() {
    dioxus::launch(App);
}
#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
