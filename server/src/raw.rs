mod gif;
mod image;

use std::{error::Error, path::Path};

pub use image::RawImage;
use tracing::warn;

use crate::{
    constants::{OG_DIR, ROOT_DIR},
    raw::{
        gif::{RawGif, RawGifError},
        image::RawImageError,
    },
};

pub enum RawFile {
    Gif(RawGif),
    Image(RawImage),
}

// i should be sentenced for this function, im sorry its 4am and i just want to be done with this
// so much errors, only for them to be Box<dyn Error>'ed
pub fn get_raw(filename: &str, size: (u32, u32)) -> Result<RawFile, Box<dyn Error>> {
    let path = Path::new(&*ROOT_DIR).join(OG_DIR).join(filename);
    if path.extension().ok_or_else(|| "Failed to get extension")? == "gif" {
        let filn = RawGif::raw_filename(
            path.file_prefix()
                .ok_or_else(|| "Failed to get file prefix")?
                .to_str()
                .ok_or("Failed to convert OsStr to str")?,
            size,
        );
        match RawGif::open(&filn) {
            Ok(res) => Ok(RawFile::Gif(res)),
            Err(err) => {
                warn!("Error in getting raw: {err}");
                Ok(RawFile::Gif(RawGif::open(&gif::create_resize(
                    &filn, size,
                )?)?))
            }
        }
    } else {
        let filn = RawImage::raw_filename(
            path.file_prefix()
                .ok_or_else(|| "Failed to get file prefix")?
                .to_str()
                .ok_or("Failed to convert OsStr to str")?,
            size,
        );
        match RawImage::open(&filn) {
            Ok(res) => Ok(RawFile::Image(res)),
            Err(err) => {
                warn!("Error in getting raw: {err}");
                Ok(RawFile::Image(RawImage::open(&image::create_raw(
                    &filn, size,
                )?)?))
            }
        }
    }
}
