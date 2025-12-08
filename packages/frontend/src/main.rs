use crate::layouts::StandardAppLayout;
use dioxus::prelude::*;
use views::Home;
mod components;
mod layouts;
mod views;

const TAILWIND_CSS: Asset = asset!("/assets/dist/tailwind.css");
const FAVICON: Asset = asset!("/assets/favicon.svg");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(StandardAppLayout)]
    #[route("/")]
    Home {},
}
fn main() -> Result<(), anyhow::Error> {
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
