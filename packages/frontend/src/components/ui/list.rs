use dioxus::prelude::*;
#[component]
pub fn List(children: Element, header: ReadSignal<String>) -> Element {
    rsx! {
        ul { class: "list bg-base-100 rounded-box shadow-md",
            li { class: "p-4 pb-2 text-xs opacity-60 tracking-wide", {header} }
            {children}
        }
    }
}

#[component]
pub fn ListRow(children: Element, class: Option<String>) -> Element {
    let class = class.unwrap_or_default();

    rsx! {
        li { class: "list-row items-center {class}", {children} }

    }
}

#[component]
pub fn ListDetails(
    title: ReadSignal<String>,
    children: Element,
    image_url: Option<String>,
) -> Element {
    rsx! {
        ComplexListDetails {
            title: rsx! {
                h3 { {title()} }
            },
            children,
            image_url,
        }
    }
}

#[component]
pub fn ComplexListDetails(title: Element, children: Element, image_url: Option<String>) -> Element {
    rsx! {
        div {
            if image_url.is_some() {
                img { class: "size-10 rounded-box", src: image_url.unwrap() }
            }
        }

        div {
            div { {title} }
            div { class: "text-xs font-semibold opacity-60", {children} }
        }
    }
}
