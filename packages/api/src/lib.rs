use dioxus::prelude::*;

pub mod routes;
#[cfg(feature = "server")]
pub mod server;

#[post("/api/echo")]
pub async fn echo(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}
