use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub oidc_enabled: bool,
    pub oidc_provider_name: Option<String>,
    pub signup_enabled: bool,
}

fn convert_env_to_bool(value: &str) -> bool {
    match value.to_lowercase().trim() {
        "" | "true" | "yes" | "1" => true,
        "false" | "no" | "0" | &_ => false,
    }
}

fn parse_env_string(value: &str) -> Option<String> {
    let value = value.trim();
    if value.is_empty() {
        None
    } else {
        Some(value.to_owned())
    }
}

fn get_env_or<T, F: FnOnce(&str) -> T>(env_key: &str, default: T, conversion: F) -> T {
    env::var(env_key).map_or(default, |v| conversion(&v))
}

#[allow(clippy::unused_async)]
#[get("/api/app_config")]
pub async fn get_app_config() -> Result<AppConfig, ServerFnError> {
    Ok(AppConfig {
        oidc_enabled: get_env_or("OIDC_ENABLED", false, convert_env_to_bool),
        oidc_provider_name: get_env_or("OIDC_PROVIDER_NAME", None, parse_env_string),
        signup_enabled: get_env_or("SIGNUP_ENABLED", true, convert_env_to_bool),
    })
}
