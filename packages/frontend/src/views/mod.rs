mod home;
pub use home::Home;
mod not_found;
pub use not_found::NotFound;
mod login;
pub use login::LoginPage;
mod sign_up;
pub mod todo;

pub use sign_up::SignupView;
pub mod event_views;
mod group_view;
pub use group_view::GroupView;
mod new_group;
pub use new_group::NewGroup;
