use crate::server::auth::AuthenticationState;
use dioxus::fullstack::axum::middleware::Next;
use dioxus::fullstack::extract::Request;
use dioxus::fullstack::response::Response;
use dioxus::prelude::*;
use std::env;

pub async fn tracing_middleware(request: Request, next: Next) -> Response {
    let method = request.method().clone();
    let path = request.uri().path().to_string();
    let username = request.extensions().get::<AuthenticationState>().map_or(
        "Anonymous".to_string(),
        |state| {
            state
                .user
                .as_ref()
                .map(|user| user.email.clone())
                .unwrap_or("Anonymous".to_string())
        },
    );

    let response = next.run(request).await;

    if !env::var("ACCESS_LOG").is_ok_and(|value| value.to_lowercase() == "true") {
        return response;
    }

    let base_message = format!("{method} {path} {} - {username}", response.status());

    if response.status().is_server_error() {
        error!("{}", base_message);
    } else if response.status().is_client_error() {
        warn!("{}", base_message);
    } else {
        info!("{}", base_message);
    }

    response
}
