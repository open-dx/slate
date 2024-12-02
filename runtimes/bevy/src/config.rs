use bevy::color::{Color as BevyColor, Srgba};


pub struct BevySlateConfig {
    pub bevy_defaults: bool,
    pub log_filter: String,
    pub asset_dir: String,
    pub clear_color: BevyColor,
}

impl Default for BevySlateConfig {
    fn default() -> Self {
        BevySlateConfig {
            bevy_defaults: true,
            log_filter: String::default(),
            asset_dir: String::default(),
            clear_color: BevyColor::hsla(0., 0., 0., 0.),
        }
    }
}

impl BevySlateConfig {
    pub fn with_bevy_defaults(mut self, bevy_defaults: bool) -> Self {
        self.bevy_defaults = bevy_defaults;
        self // etc..
    }
    
    pub fn with_log_filter<S: Into<String>>(mut self, log_filter: S) -> Self {
        self.log_filter = log_filter.into();
        self // etc..
    }
    
    pub fn with_asset_dir<S: Into<String>>(mut self, asset_dir: S) -> Self {
        self.asset_dir = asset_dir.into();
        self // etc..
    }
    
    pub fn with_clear_color(mut self, color: BevyColor) -> Self {
        self.clear_color = color;
        self // etc..
    }
}
