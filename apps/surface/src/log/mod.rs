
//---
/// TODO
#[cfg(not(feature = "debug"))]
#[cfg(not(feature = "verbose"))]
pub const DEFAULT_LOG_FILTER: &str = "error,slate-surface=info,bevy_slate=info,slate=info";

/// TODO
#[cfg(feature = "debug")]
#[cfg(not(feature = "verbose"))]
pub const DEFAULT_LOG_FILTER: &str = "warn,slate_surface=trace,bevy_slate=debug,slate=debug,wgpu_core=warn,wgpu_hal=warn";

/// TODO
#[cfg(feature = "verbose")]
pub const DEFAULT_LOG_FILTER: &str = "info,slate_surface=trace,bevy_slate=trace,slate=trace,wgpu_core=error,wgpu_hal=error";
