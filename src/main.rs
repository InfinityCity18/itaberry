use embedded_graphics::pixelcolor::raw::RawU16;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
use gif::DecodeOptions;
use linux_embedded_hal::SpidevDevice;
use mipidsi::TestImage;
use mipidsi::options::Orientation;
use mipidsi::{Builder, models::ILI9486Rgb666}; // Provides the builder for Display
use rppal::gpio::{self, Gpio};
use rppal::hal::Delay;
use rppal::spi::{Bus, Spi};
use spidev::{SpiModeFlags, Spidev, SpidevOptions}; // Provides the required color type
use tinybmp::Bmp;

mod constants;
mod display;
mod raw;

fn main() {}
