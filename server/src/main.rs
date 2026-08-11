use std::{error::Error, path::Path};

use crate::display::load_displays_from_config;

mod constants;
mod display;
mod raw;

fn main() -> Result<(), Box<dyn Error>> {
    let mut displays = load_displays_from_config(Path::new("config.toml"))?;
    displays.iter_mut().next().unwrap().display_test_image()?;
    Ok(())
}
