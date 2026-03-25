use dioxus::prelude::*;
use std::sync::OnceLock;
use tokio::sync::mpsc::UnboundedSender;

pub mod components;

static DEEP_LINK_TX: OnceLock<UnboundedSender<String>> = OnceLock::new();
static GO_BACK_TX: OnceLock<UnboundedSender<()>> = OnceLock::new();

#[cfg(feature = "mobile")]
mod android {
    use super::{handle_deep_link, handle_go_back};
    use dioxus::prelude::debug;
    use jni::EnvUnowned;
    use jni::jni_mangle;
    use jni::objects::JClass;
    use jni::objects::JString;

    #[jni_mangle("dev.dioxus.main.MainActivity")]
    pub extern "system" fn native_handle_deep_link<'caller>(
        _unowned_env: EnvUnowned<'caller>,
        _class: JClass<'caller>,
        url: JString<'caller>,
    ) {
        debug!("Hello from nativeHandleDeepLink!");
        let url: String = url.to_string();

        debug!("Received deep link: {}", url);

        handle_deep_link(&url);
    }

    #[jni_mangle("dev.dioxus.main.MainActivity")]
    pub extern "system" fn native_handle_go_back<'caller>(
        _unowned_env: EnvUnowned<'caller>,
        _class: JClass<'caller>,
    ) {
        debug!("Hello from nativeHandleGoBack!");

        handle_go_back();
    }
}

pub fn register_deep_link_tx(tx: UnboundedSender<String>) {
    let _ = DEEP_LINK_TX.set(tx);
}
pub fn register_go_back_tx(tx: UnboundedSender<()>) {
    let _ = GO_BACK_TX.set(tx);
}

#[allow(dead_code)]
fn handle_deep_link(url: &str) {
    let url = url.strip_prefix("roommates://").unwrap_or(&url).to_string();
    debug!("Handling deep link in handle_deep_link: {}", url);
    if let Some(tx) = DEEP_LINK_TX.get() {
        if let Err(e) = tx.send(url) {
            debug!("Failed to send deep link URL through channel: {}", e);
        }
    } else {
        debug!("Deep link channel not initialized");
    }
}

#[allow(dead_code)]
fn handle_go_back() {
    if let Some(tx) = GO_BACK_TX.get() {
        if let Err(e) = tx.send(()) {
            debug!("Failed to send go back signal through channel: {}", e);
        }
    } else {
        debug!("Go back channel not initialized");
    }
}
