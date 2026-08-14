use std::{error::Error, path::Path};

use tracing::Level;

use crate::{
    constants::{OG_DIR, RAW_DIR, ROOT_DIR},
    display::{AnyDisplay, FakeDisplay, load_displays_from_config},
};

mod constants;
mod display;
mod display_thread;
mod raw;
mod webserver;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    create_dirs().await?;
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();
    let mut displays: Vec<Box<FakeDisplay>> =
        (1..5).map(|id| Box::new(FakeDisplay { id })).collect();
    webserver::webserver_main().await?;
    Ok(())
}

async fn create_dirs() -> Result<(), Box<dyn Error>> {
    tokio::fs::create_dir_all(Path::new(&*ROOT_DIR).join(RAW_DIR)).await?;
    tokio::fs::create_dir_all(Path::new(&*ROOT_DIR).join(OG_DIR)).await?;
    Ok(())
}
