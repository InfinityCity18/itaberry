mod anydisplay;
mod displaybuild;

use displaybuild::DisplayConfig;
use mipidsi::{
    interface::{InterfacePixelFormat, SpiInterface},
    models::{GC9A01, Model, ST7789},
};
use rppal::gpio;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::display::{anydisplay::AnyDisplay, displaybuild::BuildDisplayError};

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

pub fn load_displays_from_config(path: &str) -> Result<Vec<Box<dyn AnyDisplay>>, DisplayLoadError> {
    let file_content_string = std::fs::read_to_string(path)?;
    let display_configs: Vec<DisplayConfig> = toml::from_str(&file_content_string)?;
    let displays: Vec<Box<dyn AnyDisplay>> = display_configs
        .into_iter()
        .map(|conf| {
            let id = conf.id;
            conf.build_display(id)
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(displays)
}

#[derive(Error, Debug)]
enum DisplayLoadError {
    #[error("Failed to open config file")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse toml config")]
    TomlParse(#[from] toml::de::Error),
    #[error(transparent)]
    BuildDisplay(#[from] BuildDisplayError),
}
