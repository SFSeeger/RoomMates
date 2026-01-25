use api::routes::users::{UserInfo, get_me};
use dioxus::prelude::*;

use crate::Route;

#[derive(Clone, Debug)]
pub struct AuthState {
    pub user: Signal<Option<UserInfo>>,
}

impl AuthState {}

#[component]
pub fn AuthProvider(children: Element) -> Element {
    let mut user = use_signal(|| None);

    let user_loaded = use_server_future(|| async move { get_me().await.ok() })?;

    use_effect(move || {
        let fetched_user = user_loaded();
        user.set(fetched_user.unwrap_or(None));
    });

    use_context_provider(|| AuthState { user });

    rsx! {
        {children}
    }
}

/// Component to hide its children from unauthenticated users
///
/// #Example
/// ```ignore
/// AuthGuard {
///    div { "This content is only visible to logged in users." }
/// }
/// `````
#[component]
pub fn AuthGuard(children: Element) -> Element {
    let auth_state = use_context::<AuthState>();

    rsx! {
        if auth_state.user.read().is_some() {
            {children}
        } else {
            div { class: "flex flex-col items-center justify-center h-full",
                h1 { class: "text-2xl font-bold", "Access Denied" }
                p { "You must be logged in to access this page." }
                Link { to: Route::LoginPage {}, class: "btn btn-primary mt-4", "Go to Login" }
            }
        }
    }
}
