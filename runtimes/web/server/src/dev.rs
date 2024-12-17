use std::process::Command;

pub fn build_wasm_pkg(name: &str, out_dir: &str) -> Result<(), WebDevError> {
    let output = Command::new("wasm-pack")
        .arg("build").arg(name).arg("--release")
        .arg("--target").arg("web")
        .arg("--out-name").arg("slate")
        .arg("--out-dir").arg(out_dir)
        .output()
        .expect("Failed to execute wasm-pack");
    
    if !output.status.success() {
        tracing::error!("wasm-pack failed: {}", String::from_utf8_lossy(&output.stderr));
        Err(WebDevError::BuildFailed)
    } else {
        Ok(())
    }
}

#[derive(oops::Error)]
pub enum WebDevError {
    BuildFailed,
}