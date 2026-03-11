use crate::server;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub oidc_enabled: bool,
    pub oidc_provider_name: Option<String>,
    pub signup_enabled: bool,
}

#[allow(clippy::unused_async)]
#[get("/api/app_config")]
pub async fn get_app_config() -> Result<AppConfig, ServerFnError> {
    use server::constants;
    use server::utils::{convert_env_to_bool, get_env_or, parse_env_string};

    Ok(AppConfig {
        oidc_enabled: get_env_or(constants::OIDC_ENABLED_ENV_VAR, false, convert_env_to_bool),
        oidc_provider_name: get_env_or(
            constants::OIDC_PROVIDER_NAME_ENV_VAR,
            None,
            parse_env_string,
        ),
        signup_enabled: get_env_or(constants::SIGNUP_ENABLED_ENV_VAR, true, convert_env_to_bool),
    })
}
