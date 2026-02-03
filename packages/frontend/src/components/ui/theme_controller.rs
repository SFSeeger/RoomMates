use std::fmt;

use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons::LdPalette};

use crate::components::ui::button::Button;

#[derive(Copy, Clone, PartialEq, Default)]
enum Theme {
    #[default]
    Default,
    DefaultDark,
    Warm,
    Pastel,
    Frappe,
}

impl Theme {
    fn value(&self) -> &'static str {
        match self {
            Theme::Default => "roommatesdefault",
            Theme::DefaultDark => "roommatesdark",
            Theme::Warm => "roommateswarm",
            Theme::Pastel => "roommatespastel",
            Theme::Frappe => "frappe",
        }
    }

    fn from_string(value: &str) -> Self {
        match value {
            "roommatesdefault" => Theme::Default,
            "roommatesdark" => Theme::DefaultDark,
            "roommateswarm" => Theme::Warm,
            "roommatespastel" => Theme::Pastel,
            "frappe" => Theme::Frappe,
            _ => Theme::Default,
        }
    }
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Theme::Default => "Light",
            Theme::DefaultDark => "Dark",
            Theme::Warm => "Warm",
            Theme::Pastel => "Pastel",
            Theme::Frappe => "Catppuccino",
        };
        write!(f, "{name}")
    }
}

#[component]
pub fn ThemeController(#[props(default)] dropdown_top: bool, id_extra: String) -> Element {
    let mut current_theme = use_signal(|| Theme::Default);
    use_effect(move || {
        spawn(async move {
            let mut eval = document::eval(
                r#"
                    const darkModeMql = window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)');
                    const defaultTheme = darkModeMql && darkModeMql.matches ? "roommatesdark" : "roommatesdefault";
                    dioxus.send(localStorage.getItem("theme") ?? defaultTheme);
                "#,
            );
            let theme: String = eval.recv().await.unwrap();
            current_theme.set(Theme::from_string(&theme));
        });
    });
    let on_theme_change = move |theme: Theme| {
        current_theme.set(theme);
        document::eval(&format!(
            r#"localStorage.setItem("theme", "{}");"#,
            theme.value(),
        ));
    };
    rsx! {
        Button {
            class: "w-full btn-sm",
            popovertarget: "popover-theme-{id_extra}",
            style: "anchor-name:--anchor-theme-{id_extra}",
            Icon { icon: LdPalette, class: "size-4" }
            "Theme"
        }
        ul {
            class: if dropdown_top { "dropdown-top" },
            class: "dropdown dropdown-end menu w-full md:w-52 bg-base-300 rounded-box p-2 shadow-2xl",
            popover: "auto",
            id: "popover-theme-{id_extra}",
            style: "position-anchor:--anchor-theme-{id_extra}",

            ThemeControllerEntry {
                theme: Theme::Default,
                current_theme,
                on_theme_change,
            }
            ThemeControllerEntry {
                theme: Theme::DefaultDark,
                current_theme,
                on_theme_change,
            }
            ThemeControllerEntry { theme: Theme::Warm, current_theme, on_theme_change }
            ThemeControllerEntry {
                theme: Theme::Pastel,
                current_theme,
                on_theme_change,
            }
            ThemeControllerEntry {
                theme: Theme::Frappe,
                current_theme,
                on_theme_change,
            }
        }
    }
}

#[component]
fn ThemeControllerEntry(
    theme: Theme,
    current_theme: ReadSignal<Theme>,
    on_theme_change: EventHandler<Theme>,
) -> Element {
    rsx! {
        li {
            input {
                r#type: "radio",
                name: "theme-dropdown",
                class: "theme-controller w-full btn btn-sm btn-block btn-ghost justify-start",
                aria_label: theme.to_string(),
                value: theme.value(),
                checked: *current_theme.read() == theme,
                onclick: move |_| { on_theme_change.call(theme) },
            }
        }
    }
}
