use dioxus::prelude::*;
use doxle_core::theme::Theme;

#[component]
pub fn UserDropdown(
    theme: Signal<Theme>,
    dropdown_open: Signal<bool>,
) -> Element {
    if !dropdown_open() {
        return rsx! { };
    }
    
    let current_theme = theme();
    
    rsx! {
        div {
            style: format!("
                position: absolute;
                top: 40px;
                right: 0;
                background-color: {};
                border: 1px solid {};
                border-radius: 8px;
                min-width: 150px;
                box-shadow: 0 4px 6px rgba(0,0,0,0.1);
                z-index: 1000;
                padding: 4px 0;
            ", current_theme.bg_secondary(), current_theme.border_color()),

            // Settings button
            button {
                class: "dropdown-button",
                onclick: move |_| {
                    // Settings action (placeholder)
                    dropdown_open.set(false);
                },
                "Settings"
            }

            // Theme button
            button {
                class: "dropdown-button",
                onclick: move |_| {
                    theme.write().toggle();
                    dropdown_open.set(false);
                },
                "Theme"
            }

            // Divider
            div {
                style: format!("
                    height: 1px;
                    background-color: {};
                    margin: 4px 0;
                ", current_theme.border_color())
            }

            // About button
            button {
                class: "dropdown-button",
                onclick: move |_| {
                    // About action (placeholder)
                    dropdown_open.set(false);
                },
                "About"
            }
        }
    }
}
