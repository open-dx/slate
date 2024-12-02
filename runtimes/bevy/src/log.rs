
//---
/// TODO
#[cfg(not(any(feature = "debug", feature = "verbose")))]
pub const DEFAULT_LOG_FILTER: &str = "warn,bevy_slate_basic=info,bevy_slate=info,slate=info";

/// TODO
#[cfg(all(feature = "debug", not(feature = "verbose")))]
pub const DEFAULT_LOG_FILTER: &str = "debug,bevy_slate_basic=debug,bevy_slate=debug,slate=debug,wgpu_core=warn,wgpu_hal=warn";

/// TODO
#[cfg(all(feature = "debug", feature = "verbose"))]
pub const DEFAULT_LOG_FILTER: &str = "trace,bevy_slate_basic=trace,bevy_slate=trace,slate=trace,wgpu_core=error,wgpu_hal=error";
