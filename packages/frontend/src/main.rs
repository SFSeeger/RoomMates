#![windows_subsystem = "windows"]

use crate::layouts::StandardAppLayout;
use crate::views::event_views::DateQueryParam;
use dioxus::prelude::*;
use roommates::OptionalIntQueryParam;
use views::{
    Home, LoginPage, NotFound, Profile, SignupView,
    event_views::{AddEventView, EditEventView, EventCalendarView, ListEventView},
    groups::{EditGroup, GroupView},
    invitation_views::{ListInviteView, SendInvite},
    todo::{TodoListCreateView, TodoListListView, TodosGroupView},
};

mod components;
mod hooks;
mod layouts;
mod views;

const TAILWIND_CSS: Asset = asset!("/assets/dist/tailwind.css");
const FAVICON: Asset = asset!("/assets/favicon.ico");
pub const ICON: Asset = asset!("/assets/icon.svg");

// !! IMPORTANT: DO NOT FORMAT THIS! The formatting of the routing enum determines behaviour !!
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(StandardAppLayout)]
        #[route("/")]
        Home {},
        #[route("/login")]
        LoginPage {},
        #[route("/signup")]
        SignupView {},

        #[nest("/todo")]
            #[route("/")]
            TodoListListView {},
            #[route("/add")]
            TodoListCreateView {},
            #[nest("/:todo_list_id")]
                #[route("/")]
                TodosGroupView {todo_list_id: i32},
            #[end_nest]
        #[end_nest]

        #[nest("/event")]
            #[route("/")]
            EventCalendarView {},
            #[route("/list?:date")]
            ListEventView {date: DateQueryParam},
            #[route("/:event_id/edit?:group_id")]
            EditEventView {event_id: i32, group_id: OptionalIntQueryParam},
            #[route("/add?:group_id")]
            AddEventView {group_id: OptionalIntQueryParam},
       #[end_nest]

       #[nest("/invitations")]
            #[route("/")]
            ListInviteView {},
            #[route("/:invite_id/send_invite")]
            SendInvite {invite_id : i32},
         #[end_nest]

        #[nest("/groups")]
            #[route("/")]
            GroupView {},
            #[route("/:group_id/edit")]
            EditGroup {group_id: i32},
        #[end_nest]

        #[route("/profile")]
        Profile {},

        #[route("/:..segments")]
        NotFound { segments: Vec<String> },
}
fn main() {
    #[cfg(feature = "server")]
    dioxus::serve(|| async move { api::server::setup_api(App).await });

    #[cfg(not(debug_assertions))]
    #[cfg(all(not(feature = "server"), any(feature = "desktop", feature = "mobile")))]
    dioxus_fullstack::set_server_url(env!(
        "SERVER_URL",
        "SERVER_URL environment variable must be set when compiling"
    ));
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);
}
#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
