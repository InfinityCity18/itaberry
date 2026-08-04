use crate::display::Display;

use super::anydisplay::AnyDisplay;
use linux_embedded_hal::SpidevDevice;
use mipidsi::{
    Builder,
    interface::SpiInterface,
    models::{GC9A01, ST7789},
};
use rppal::{gpio::Gpio, hal::Delay};
use serde::{Deserialize, Serialize};
use spidev::{SpiModeFlags, Spidev, SpidevOptions};
use thiserror::Error;

const SPI_CLOCK_SPEED_HZ: u32 = 60_000_000;
const BUFFER_SIZE: usize = 4096;

impl DisplayConfig {
    pub fn build_display(self, id: i32) -> Result<Box<dyn AnyDisplay>, BuildDisplayError> {
        let gpio = Gpio::new()?;
        let mut delay = Delay::new();
        let spidev_device = SpidevDevice(create_spidev(&self)?);

        let dc = gpio.get(self.dc_pin)?.into_output();
        let rst = gpio.get(self.reset_pin)?.into_output();
        let buffer: &'static mut [u8] = Box::leak(Box::new([0; BUFFER_SIZE]));

        let interface = SpiInterface::new(spidev_device, dc, buffer);

        match self.model {
            DisplayModelConfig::Gc9a01 => {
                let displ = Builder::new(GC9A01, interface)
                    .color_order(self.color_order)
                    .display_offset(self.display_offset.0, self.display_offset.1)
                    .display_size(self.display_size.0, self.display_size.1)
                    .invert_colors(self.inversion)
                    .orientation(self.orientation)
                    .reset_pin(rst)
                    .init(&mut delay)
                    .map_err(|e| BuildDisplayError::Init(format!("{:?}", e)))?;
                let displayo = Display { id, disp: displ };
                Ok(Box::new(displayo))
            }
            DisplayModelConfig::St7789 => {
                let displ = Builder::new(ST7789, interface)
                    .color_order(self.color_order)
                    .display_offset(self.display_offset.0, self.display_offset.1)
                    .display_size(self.display_size.0, self.display_size.1)
                    .invert_colors(self.inversion)
                    .orientation(self.orientation)
                    .reset_pin(rst)
                    .init(&mut delay)
                    .map_err(|e| BuildDisplayError::Init(format!("{:?}", e)))?;
                let displayo = Display { id, disp: displ };
                Ok(Box::new(displayo))
            }
        }
    }
}

fn create_spidev(conf: &DisplayConfig) -> Result<Spidev, SpidevError> {
    let mut spi = Spidev::open(format!(
        "/dev/spidev{}.{}",
        conf.controller_id, conf.chipselect_id
    ))?;
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(SPI_CLOCK_SPEED_HZ)
        .mode(SpiModeFlags::SPI_MODE_0)
        .build();
    spi.configure(&options)?;
    Ok(spi)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DisplayConfigRoot {
    #[serde(rename = "DisplayConfig")]
    pub displayconfig: Vec<DisplayConfig>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DisplayConfig {
    pub id: i32,
    model: DisplayModelConfig,
    reset_pin: u8,
    dc_pin: u8,
    controller_id: u8,
    chipselect_id: u8,
    display_size: (u16, u16),
    display_offset: (u16, u16),
    #[serde(with = "ColorInversionDef")]
    inversion: mipidsi::options::ColorInversion,
    #[serde(with = "ColorOrderDef")]
    color_order: mipidsi::options::ColorOrder,
    #[serde(with = "OrientationDef")]
    orientation: mipidsi::options::Orientation,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "mipidsi::options::ColorInversion")]
pub enum ColorInversionDef {
    Normal,
    Inverted,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "mipidsi::options::ColorOrder")]
pub enum ColorOrderDef {
    Rgb,
    Bgr,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "mipidsi::options::Orientation")]
pub struct OrientationDef {
    #[serde(with = "RotationDef")]
    rotation: mipidsi::options::Rotation,
    mirrored: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(remote = "mipidsi::options::Rotation")]
pub enum RotationDef {
    Deg0,
    Deg90,
    Deg180,
    Deg270,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum DisplayModelConfig {
    Gc9a01,
    St7789,
}

#[derive(Debug, Error)]
pub enum BuildDisplayError {
    #[error(transparent)]
    Spidev(#[from] SpidevError),
    #[error(transparent)]
    Gpio(#[from] rppal::gpio::Error),
    #[error("Init failed with {0}")]
    Init(String),
}

#[derive(Debug, Error)]
#[error(transparent)]

pub struct SpidevError(#[from] std::io::Error);
