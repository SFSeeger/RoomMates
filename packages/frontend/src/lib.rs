use dioxus::CapturedError;
use dioxus::fullstack::RequestError;
use dioxus::prelude::*;
use serde::Deserialize;

// This is an extremely dirty way to retrieve the actual error message from the server. For some reason, the Server functions return `ServerFnError::Request(RequestError::Status {message, ..})`
// where the message is a stringified JSON of the actual error.
// Example of the message:  {"message":"error running server function: Cannot invite owner! (details: None)","code":400,"data":{"ServerError":{"message":"Cannot invite owner!","code":400}}}
#[derive(Deserialize, Debug)]
struct ServerError {
    message: String,
    #[serde(default)]
    data: Option<ServerFnError>,
}

/// Converts a captured error into a user-friendly message.
/// It checks for specific error types and returns appropriate messages based on the error content.
/// If the error type is not recognized, it returns a generic error message.
///
/// # Example
/// ```
/// # use dioxus::{prelude::ServerFnError, CapturedError};
/// # use roommates::message_from_captured_error;
/// # let error: CapturedError = anyhow::anyhow!(ServerFnError::ServerError { message: "Internal Server Error".to_string(), code: 500, details: None }).into();
/// let message = message_from_captured_error(&error);
/// // This message can be used to display a user-friendly error message in the UI.
/// assert_eq!(
///     message,
///     "Internal Server Error".to_string()
/// )
/// ```
#[must_use]
pub fn message_from_captured_error(error: &CapturedError) -> String {
    if let Some(err) = error.downcast_ref::<ServerFnError>() {
        return match err {
            ServerFnError::ServerError { message, .. } => message.clone(),
            ServerFnError::Request(RequestError::Status(message, _)) => serde_json::from_str::<
                ServerError,
            >(message)
            .map(|server_error| {
                if let Some(ServerFnError::ServerError { message, .. }) = server_error.data {
                    message
                } else {
                    server_error.message
                }
            })
            .unwrap_or(message.clone()),
            _ => "An unknown error occurred".to_string(),
        };
    }

    if let Some(err) = error.downcast_ref::<StatusCode>() {
        return format!("An error occurred with status code: {err}");
    }

    if let Some(err) = error.downcast_ref::<HttpError>() {
        return err.message.clone().unwrap_or(format!(
            "An error occurred with status code: {}",
            err.status
        ));
    }

    "An unknown error occurred".to_string()
}

/// The number of days since the first weekday of current date
/// From dioxus-calendar's source code:
pub fn days_since(date: time::Date, weekday: time::Weekday) -> i64 {
    let lhs = date.replace_day(1).unwrap().weekday() as i64;
    let rhs = weekday as i64;
    if lhs < rhs { 7 + lhs - rhs } else { lhs - rhs }
}

/// Helper function, used to check if an event occurs on a specific day.
/// It checks if the event matches the date and returns a boolean.
pub fn is_event_on_day(event: &entity::event::Model, date: time::Date) -> bool {
    let weekday: time::Weekday = event.weekday.into();
    (!event.reoccurring && event.date == date) || (event.reoccurring && weekday == date.weekday())
}

/// Helper function, used to check if an [`entity::event::FullEvent`] occurs on a specific day.
/// It checks if the event matches the date and returns a boolean.
pub fn is_full_event_on_day(event: &entity::event::FullEvent, date: time::Date) -> bool {
    let weekday: time::Weekday = event.weekday.into();
    (!event.reoccurring && event.date == date) || (event.reoccurring && weekday == date.weekday())
}
