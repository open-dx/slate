use std::process::ExitCode;

use anyhow::Result;

pub fn main() -> Result<ExitCode> {
    println!("cargo::rerun-if-changed=../src/main.rs");
    println!("cargo::rerun-if-changed=../src/relay.rs");
    println!("cargo::rerun-if-changed=../src/surface.rs");
    
    let project_dir = std::env::current_dir()?;
    
    watch_dir(project_dir.parent().and_then(|p| p.parent()).expect("watch dir"));
    
    if let Ok(profile) = std::env::var("PROFILE") {
        if profile == "release" {
            println!("cargo:rustc-cdylib-link-arg=-mwindows");
        }
    }
    
    Ok(ExitCode::SUCCESS)
}

fn watch_dir(dir: &std::path::Path) {
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir).expect("Failed to read directory") {
            let entry = entry.expect("Failed to read entry");
            let path = entry.path();

            if path.is_dir() {
                watch_dir(&path);
            } else if let Some(ext) = path.extension() {
                if ext == "rs" {
                    println!("cargo:rerun-if-changed={}", path.display());
                }
            }
        }
    }
}
