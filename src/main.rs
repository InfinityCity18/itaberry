use std::{error::Error, path::Path};

use crate::display::load_displays_from_config;

mod constants;
mod display;
mod raw;

fn main() -> Result<(), Box<dyn Error>> {
    load_displays_from_config(Path::new("config.toml"))?;
    Ok(())
}
