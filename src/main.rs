use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

use gyn::routes::routes::Route;
use crate::db::create_tables;

// Importer le module db
mod db;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");

    create_tables()?;

    let cfg = dioxus::desktop::Config::new()
        .with_custom_head(r#"<script src="https://cdn.tailwindcss.com"></script>"#.to_string())
        .with_window(dioxus::desktop::WindowBuilder::new().with_resizable(false)
        .with_inner_size(dioxus::desktop::LogicalSize::new(1300, 700)));

    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
    Ok(())
}

#[component]
fn App() -> Element {
    rsx! {
        style { "body, html {{ overflow: hidden; margin: 0; padding: 0; }}" }
        Router::<Route> {}
    }
}