mod anydisplay;
mod displaybuild;

use std::path::Path;

use displaybuild::DisplayConfig;
use mipidsi::{
    interface::{InterfacePixelFormat, SpiInterface},
    models::Model,
};
use rppal::gpio;
use thiserror::Error;

pub use crate::display::{
    anydisplay::AnyDisplay,
    displaybuild::{BuildDisplayError, DisplayConfigRoot},
};

pub struct FakeDisplay {
    pub id: i32,
}

struct Display<'a, MODEL>
where
    MODEL: Model,
    MODEL::ColorFormat: InterfacePixelFormat<u8>,
{
    pub id: i32,
    pub disp: mipidsi::Display<
        SpiInterface<'a, linux_embedded_hal::SpidevDevice, gpio::OutputPin>,
        MODEL,
        gpio::OutputPin,
    >,
}

pub fn load_displays_from_config(
    path: &Path,
) -> Result<Vec<Box<dyn AnyDisplay + Send + Sync>>, DisplayLoadError> {
    let file_content_string = std::fs::read_to_string(path)?;
    let display_configs: DisplayConfigRoot = toml::from_str(&file_content_string)?;
    let displays: Vec<Box<dyn AnyDisplay + Send + Sync>> = display_configs
        .displayconfig
        .into_iter()
        .map(|conf| {
            let id = conf.id;
            conf.build_display(id)
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(displays)
}

pub fn load_config(path: &Path) -> Result<Vec<DisplayConfig>, DisplayLoadError> {
    let file_content_string = std::fs::read_to_string(path)?;
    let display_configs: DisplayConfigRoot = toml::from_str(&file_content_string)?;
    Ok(display_configs.displayconfig)
}

#[derive(Error, Debug)]
pub enum DisplayLoadError {
    #[error("Failed to open config file")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse toml config : {0}")]
    TomlParse(#[from] toml::de::Error),
    #[error(transparent)]
    BuildDisplay(#[from] BuildDisplayError),
}
