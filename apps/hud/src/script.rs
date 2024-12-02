use serde::Serialize;
use serde::Deserialize;

use bevy::prelude::*;
use bevy::asset::AssetLoader;
use bevy::asset::LoadContext;
use bevy::asset::AsyncReadExt;
use bevy::asset::io::Reader;
use bevy::reflect::TypePath;

//---
#[derive(Asset, TypePath, Default, Debug)]
pub struct Script {
    pub bytes: Vec<u8>,
}

impl From<Vec<u8>> for Script {
    fn from(bytes: Vec<u8>) -> Self {
        Script {
            bytes,
        }
    }
}

#[derive(Debug)]
pub enum ScriptError {
    IoError(std::io::Error),
}

impl core::error::Error for ScriptError {
    //..
}

impl core::fmt::Display for ScriptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("TODO")
    }
}

impl From<std::io::Error> for ScriptError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ScriptSettings;

//---
/// The ScriptLoader is responsible for keeping track of 
#[derive(Default, Debug)]
pub struct ScriptLoader {
    // compiler: TypeScriptLoader,
}

impl AssetLoader for ScriptLoader {
    type Asset = Script;
    type Settings = ScriptSettings;
    type Error = ScriptError;
    
    async fn load<'loader>(&'loader self, reader: &'loader mut Reader<'_>, _settings: &'loader ScriptSettings, _ctx: &'loader mut LoadContext<'_>) -> Result<Self::Asset, Self::Error> {
        info!("Loading Script...");
        
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        
        let script = Script::from(bytes.to_owned());
        // ctx.set_default_asset(LoadedAsset::new_with_dependencies(script, None));
        
        Ok(script)
    }
    
    fn extensions(&self) -> &[&str] {
        &["ethos", "ts", "tsx"]
    }
}
