
//---
/// TODO
#[cfg(all(not(feature = "debug"), not(feature = "verbose")))]
pub const DEFAULT_LOG_FILTER: &str = "error,slate-hud=info,bevy_slate=info,slate=info";

/// TODO
#[cfg(all(feature = "debug", not(feature = "verbose")))]
pub const DEFAULT_LOG_FILTER: &str = "info,slate-hud=debug,bevy_slate=debug,slate=debug,wgpu_core=info,wgpu_hal=info";

/// TODO
#[cfg(all(not(feature = "debug"), feature = "verbose"))]
pub const DEFAULT_LOG_FILTER: &str = "debug,slate-hud=trace,bevy_slate=trace,slate=trace,wgpu_core=error,wgpu_hal=error";
