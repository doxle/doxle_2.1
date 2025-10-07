use dioxus::prelude::*;
use wgpu::Limits;
use winit::window::{WindowAttributes, Icon};
use winit::dpi::LogicalSize;

// Import from doxle_core
use doxle_core::view_mode::ViewMode;
use doxle_core::theme::Theme;

// Import UI components
mod ui;
mod platforms;
use ui::Navbar;

// CSS Styles
static STYLES: Asset = asset!("/src/styles.css");

fn main() {
    #[cfg(feature = "tracing")]
    tracing_subscriber::fmt::init();

    // Load window icon
    let icon_bytes = include_bytes!("../assets/dog-icon.png");
    let icon = load_icon(icon_bytes);

    println!("Icon loaded: {}", icon.is_some());

    let mut window = WindowAttributes::default()
        .with_title("Doxle")
        .with_decorations(true)  // Use native title bar
        .with_inner_size(LogicalSize::new(1280, 800))
        .with_min_inner_size(LogicalSize::new(1280, 800));

    // macOS unified titlebar at creation time
    #[cfg(target_os = "macos")]
    {
        use winit::platform::macos::WindowAttributesExtMacOS;
        window = window
            .with_titlebar_transparent(true)
            .with_fullsize_content_view(true)
            .with_title_hidden(true)
            .with_movable_by_window_background(true);
    }

    if let Some(icon) = icon {
        println!("Setting window icon");
        window = window.with_window_icon(Some(icon));
    } else {
        println!("Failed to load icon!");
    }

    let config: Vec<Box<dyn std::any::Any>> = vec![Box::new(limits()), Box::new(window)];
    dioxus_native::launch_cfg(app, Vec::new(), config);
}

fn limits() -> Limits {
    Limits {
        max_storage_buffers_per_shader_stage: 12,
        ..Limits::default()
    }
}

fn load_icon(bytes: &[u8]) -> Option<Icon> {
    let image = image::load_from_memory(bytes).ok()?;
    let rgba = image.to_rgba8();
    let (width, height) = rgba.dimensions();
    Icon::from_rgba(rgba.into_raw(), width, height).ok()
}

fn app() -> Element {
    // State management
    let view_mode = use_signal(|| ViewMode::TwoD);
    let theme = use_signal(|| Theme::Dark);
    let dropdown_open = use_signal(|| false);

    // Ensure macOS unified titlebar setup runs once after startup
    #[cfg(target_os = "macos")]
    {
        let mut did_setup = use_signal(|| false);
        if !did_setup() {
            platforms::setup_unified_titlebar();
            did_setup.set(true);
        }
    }

    let current_theme = theme();

    // Inject a platform class for stylesheet-based platform overrides
    let platform_class = if cfg!(target_os = "macos") { "platform-macos" } else { "" };
    
    rsx! {
        document::Stylesheet { href: STYLES }
        main {
            class: platform_class,
            style: format!("{} position: absolute; inset: 0; display: flex; flex-direction: column; background-color: var(--bg-primary); color: var(--text-primary);", current_theme.to_css_vars()),
            // Navbar component
            Navbar {
                view_mode,
                theme,
                dropdown_open,
            }

            // Canvas area - placeholder for now
            div {
                style: "width: 100%; flex: 1; background-color: var(--bg-primary); color: var(--text-secondary); display: flex; align-items: center; justify-content: center;",
                // "Canvas Area - Current Mode: {view_mode:?}"
            }
        }
    }
}
