#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
pub use macos::setup_unified_titlebar;

#[cfg(not(target_os = "macos"))]
pub fn setup_unified_titlebar() {}
