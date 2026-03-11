use api::routes::app_config::{AppConfig, get_app_config};
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Eq)]
pub struct AppConfigContext {
    inner: AppConfig,
}

impl AppConfigContext {
    pub fn new(inner: AppConfig) -> Self {
        Self { inner }
    }
}

#[component]
pub fn AppConfigProvider(children: Element) -> Element {
    let app_config = use_loader(get_app_config)?;

    use_context_provider(|| AppConfigContext::new(app_config.cloned()));

    rsx! {
        {children}
    }
}

pub fn use_app_config() -> AppConfig {
    try_use_context::<AppConfigContext>()
        .expect("use_app_settings can only be used inside AppConfigContext")
        .inner
}
