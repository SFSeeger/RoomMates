use std::time::{Duration, Instant};

use dioxus::prelude::*;

use crate::ffi::{register_deep_link_tx, register_go_back_tx};

#[component]
pub fn DeepLinkHandler() -> Element {
    let navigator = use_navigator();

    use_effect(move || {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<String>();
        register_deep_link_tx(tx);

        spawn(async move {
            while rx.recv().await.is_some() {
                if let Some(url) = rx.recv().await {
                    navigator.push(url);
                }
            }
        });
    });

    rsx! {}
}

#[component]
pub fn GoBackHandler() -> Element {
    let navigator = use_navigator();

    use_effect(move || {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<()>();

        register_go_back_tx(tx);

        spawn(async move {
            let mut last_back: Option<Instant> = None;

            while rx.recv().await.is_some() {
                if navigator.can_go_back() {
                    navigator.go_back();
                    last_back = None;
                    continue;
                }

                let now = Instant::now();

                if last_back
                    .map(|t| now.duration_since(t) <= Duration::from_secs(2))
                    .unwrap_or(false)
                {
                    info!("Exiting...");
                } else {
                    last_back = Some(now);
                    info!("Spawning toast...");
                }
            }
        });
    });

    // This component does not render any UI
    rsx! {}
}
