use std::env;

pub fn convert_env_to_bool(value: &str) -> bool {
    match value.to_lowercase().trim() {
        "" | "true" | "yes" | "1" => true,
        "false" | "no" | "0" | &_ => false,
    }
}

pub fn parse_env_string(value: &str) -> Option<String> {
    let value = value.trim();
    if value.is_empty() {
        None
    } else {
        Some(value.to_owned())
    }
}

pub fn get_env_or<T, F: FnOnce(&str) -> T>(env_key: &str, default: T, conversion: F) -> T {
    env::var(env_key).map_or(default, |v| conversion(&v))
}
