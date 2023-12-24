use std::process::ExitCode;

use anyhow::Result;

// use axum::Router;

// use tower_http::services::ServeDir;

//---
#[tokio::main]
async fn main() -> Result<ExitCode> {
    // let app = {
    //     Router::new()
    //         // .route("/", get(|| async { "Hello, World!" }))
    //         // .nest("/asdf", ServeDir::new("./asdf"))
    //         .nest_service("/pkg", ServeDir::new("./pkg"))
    //         .nest_service("/", ServeDir::new("./public"))
    // };
    
    // axum::Server::bind(&"0.0.0.0:3000".parse()?)
    //     .serve(app.into_make_service())
    //     .await?;
    
    Ok(ExitCode::SUCCESS)
}
