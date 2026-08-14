use crate::display::FakeDisplay;
use crate::raw::RawImage;

use super::Display;
use embedded_graphics::Drawable;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::{OriginDimensions, Point, Size};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::primitives::Rectangle;
use mipidsi::interface::InterfacePixelFormat;
use mipidsi::models::Model;
use mockall::predicate::*;
use mockall::*;
use thiserror::Error;

pub trait AnyDisplay {
    fn id(&self) -> i32;
    fn size(&self) -> (u32, u32);
    fn display_test_image(&mut self) -> Result<(), DisplayError>;
    fn clear_display(&mut self, color: Rgb565) -> Result<(), DisplayError>;
    fn display_image(&mut self, img: &RawImage) -> Result<(), DisplayError>;
}

impl AnyDisplay for FakeDisplay {
    fn id(&self) -> i32 {
        self.id
    }

    fn size(&self) -> (u32, u32) {
        (240, 240)
    }

    fn display_test_image(&mut self) -> Result<(), DisplayError> {
        tracing::debug!("Test image display with id: {}", self.id);
        Ok(())
    }

    fn clear_display(&mut self, color: Rgb565) -> Result<(), DisplayError> {
        tracing::debug!("Cleared display with id: {}", self.id);
        Ok(())
    }

    fn display_image(&mut self, img: &RawImage) -> Result<(), DisplayError> {
        tracing::debug!("Displaying image on display id: {}", self.id);
        Ok(())
    }
}

impl<'a, MODEL> AnyDisplay for Display<'a, MODEL>
where
    MODEL: Model<ColorFormat = Rgb565>,
    MODEL::ColorFormat: InterfacePixelFormat<u8>,
{
    fn id(&self) -> i32 {
        self.id
    }

    fn size(&self) -> (u32, u32) {
        (self.disp.size().width, self.disp.size().height)
    }

    fn display_test_image(&mut self) -> Result<(), DisplayError> {
        mipidsi::TestImage::new()
            .draw(&mut self.disp)
            .map_err(|e| DisplayError::DrawError(format!("{:?}", e)))?;
        Ok(())
    }

    fn clear_display(&mut self, color: Rgb565) -> Result<(), DisplayError> {
        self.disp
            .clear(color)
            .map_err(|e| DisplayError::DrawError(format!("{:?}", e)))?;
        Ok(())
    }

    fn display_image(&mut self, img: &RawImage) -> Result<(), DisplayError> {
        let area = Rectangle::new(
            Point::zero(),
            Size::new(self.disp.size().width, self.disp.size().height),
        );
        self.disp
            .fill_contiguous(&area, img.pixels.clone())
            .map_err(|e| DisplayError::DrawError(format!("{:?}", e)))?;
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum DisplayError {
    #[error("Drawing failed : {0}")]
    DrawError(String),
}
