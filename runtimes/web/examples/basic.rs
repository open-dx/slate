use std::process::ExitCode;

use anyhow::Result;

use tokio::net::TcpListener;

use tower_http::services::ServeDir;

use axum::Router;

//---
#[tokio::main]
async fn main() -> Result<ExitCode> {
    slate::log::init("trace");
    
    web_slate_server::dev::build_wasm_pkg("./runtimes/web", "./dist/public/pkg")?;
    
    let public_dir = ServeDir::new("./runtimes/web/dist/public")
        .append_index_html_on_directories(true);
    
    let app = Router::new()
        .fallback_service(axum::routing::get_service(public_dir));
    
    let addr = ("localhost", 9999);
    
    tracing::info!("Server running at http://{}:{} ..", addr.0, addr.1);
    axum::serve(TcpListener::bind(addr).await?, app).await?;
    
    Ok(ExitCode::SUCCESS)
}
