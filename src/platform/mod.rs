#[cfg(windows)]
#[path="windows/mod.rs"]
pub mod platform;

#[cfg(target_os="macos")]
#[path="macos/mod.rs"]
pub mod platform;

#[cfg(target_os="linux")]
#[path="x11/mod.rs"]
pub mod platform;

pub use self::platform::*;