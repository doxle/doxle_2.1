use dioxus::prelude::*;
use doxle_core::view_mode::ViewMode;
use doxle_core::theme::Theme;
use super::user_dropdown::UserDropdown;

static DOG_LOGO: Asset = asset!("/assets/dog-dark.svg");
static ICON_2D: Asset = asset!("/assets/2d-dark.svg");
static ICON_3D: Asset = asset!("/assets/3d-dark.svg");

#[component]
pub fn Navbar(
    view_mode: Signal<ViewMode>,
    theme: Signal<Theme>,
    dropdown_open: Signal<bool>,
) -> Element {
    let current_theme = theme();
    // On macOS with a transparent/fullsize titlebar, leave space for the traffic lights
    let left_pad = if cfg!(target_os = "macos") { "110px" } else { "12px" };
    
    rsx! {
        nav {
            style: format!("height: 40px; width: 100%; background-color: var(--bg-primary); border-bottom: 1px solid var(--border-color); display: flex; align-items: center; justify-content: space-between; -webkit-app-region: drag; padding-left: {}; padding-right: 16px;", left_pad),
            // Left section - Dog Logo (mark as non-draggable and keep width/height)
            img {
                src: DOG_LOGO,
                style: "width: 24px; height: 24px; -webkit-app-region: no-drag;",
                alt: "Doxle"
            }

            // Center section - 2D/3D Toggle
            div {
                style: "
                    position: absolute;
                    left: 50%;
                    transform: translateX(-50%);
                    display: flex;
                    gap: 8px;
                ",

                // 2D Button
                button {
                    class: if view_mode().is_2d() { "mode-button mode-button-active" } else { "mode-button" },
                    onclick: move |_| {
                        view_mode.set(ViewMode::TwoD);
                    },
                    img {
                        src: ICON_2D,
                        style: "width: 16px; height: 16px;",
                        alt: "2D"
                    }
                }
                // 3D Button
                button {
                    class: if view_mode().is_3d() { "mode-button mode-button-active" } else { "mode-button" },
                    onclick: move |_| {
                        view_mode.set(ViewMode::ThreeD);
                    },
                    img {
                        src: ICON_3D,
                        style: "width: 16px; height: 16px;",
                        alt: "3D"
                    }
                }
            }

            // Right section - Yellow avatar "S"
            div {
                style: "position: relative;",

                // Avatar button
                button {
                    class: "avatar-button",
                    onclick: move |_| {
                        dropdown_open.set(!dropdown_open());
                    },
                    "S"
                }

                // Dropdown component
                UserDropdown {
                    theme,
                    dropdown_open,
                }
            }
        }
    }
}
