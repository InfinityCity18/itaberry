use embedded_graphics::pixelcolor::{Rgb565, raw::RawU16};
use image::{DynamicImage, GenericImageView, ImageError};
use std::{
    ffi::OsStr,
    format,
    io::Write,
    path::{Path, PathBuf},
};
use thiserror::Error;

use crate::constants::{RAW_DIR, ROOT_DIR};

pub struct RawImage {
    pub pixels: Vec<Rgb565>,
}

pub fn create_raw(
    og_img_path: &Path,
    (target_width, target_height): (u32, u32),
) -> Result<PathBuf, RawImageError> {
    let img = image::ImageReader::open(og_img_path)?.decode()?;
    let mut filename = og_img_path
        .file_name()
        .ok_or(RawImageError::Filename)?
        .to_owned();
    filename.push(OsStr::new(&format!(
        "_{}x{}.raw",
        target_width, target_height
    )));
    let output_path = Path::new(ROOT_DIR).join(RAW_DIR).join(filename);

    let resized = img.resize_to_fill(
        target_width,
        target_height,
        image::imageops::FilterType::Triangle,
    );

    let mut raw_bytes = Vec::with_capacity((target_width * target_height * 2) as usize);

    for (_x, _y, pixel) in resized.pixels() {
        let r = pixel[0];
        let g = pixel[1];
        let b = pixel[2];

        let r5 = (r >> 3) as u16;
        let g6 = (g >> 2) as u16;
        let b5 = (b >> 3) as u16;

        let rgb565 = ((r5 << 11) | (g6 << 5) | b5).to_be_bytes();

        raw_bytes.push(rgb565[0]);
        raw_bytes.push(rgb565[1]);
    }

    let mut out_file = std::fs::File::create(&output_path)?;
    out_file.write_all(&raw_bytes)?;
    Ok(output_path)
}

impl RawImage {
    fn open(path: &Path) -> Result<RawImage, RawImageError> {
        let bytes = std::fs::read(path)?;
        let pixels = bytes
            .chunks_exact(2)
            .map(|s| Rgb565::from(RawU16::new(u16::from_be_bytes([s[0], s[1]]))))
            .collect();
        Ok(RawImage { pixels })
    }
}

#[derive(Error, Debug)]
pub enum RawImageError {
    #[error("Failed with io error : {0}")]
    IO(#[from] std::io::Error),
    #[error("Failed to decode image : {0}")]
    Image(#[from] ImageError),
    #[error("Couldn't get the file name of original file")]
    Filename,
    #[error("OsStr contained invalid characters")]
    OsStr,
}
