mod home;
pub use home::Home;
mod not_found;
pub use not_found::NotFound;
mod login;
pub use login::LoginPage;
mod sign_up;
pub mod todo;

pub use sign_up::SignupView;
mod event_views;
pub use event_views::AddEventView;
pub use event_views::EditEventView;
pub use event_views::ListEventView;
