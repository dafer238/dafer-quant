use anyhow::Result;
use hyper::Server;
use tracing::info;

mod config;
mod frontend;
//mod models;
mod server;
//mod services;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    info!("Starting application...");

    // Load configuration
    let config = config::app_config::AppConfig::load()?;
    info!("Configuration loaded successfully");

    // Initialize server
    let _app = server::create_app(config.clone()).await?;

    println!("aksjghf");
    // // Mount Leptos application
    // let leptos_options = leptos::LeptosOptions {
    //     output_name: "my-app".into(),
    //     site_root: "src/frontend".into(),
    //     site_pkg_dir: "pkg".into(),
    //     //style_file: Some("src/frontend/styles/main.css".into()),
    //     //..Default::default()
    // };
    //
    //leptos_axum::generate_route_list(|| leptos_options.clone());
    // // Start the server
    // let addr = ([0, 0, 0, 0], config.port).into();
    // info!("Server listening on http://{}", addr);
    //
    // Server::bind(&addr)
    //     .serve(app.into_make_service())
    //     .await?;

    Ok(())
}
