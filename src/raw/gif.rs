use embedded_graphics::pixelcolor::{Rgb565, RgbColor};
use gif::DecodingError;
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    process::{Command, ExitStatus},
};
use thiserror::Error;

use crate::constants::{RAW_DIR, ROOT_DIR};

pub struct RawGif {
    frames: Vec<(Vec<Rgb565>, u64)>,
}

pub fn create_resize(
    og_gif_path: &Path,
    (target_width, target_height): (u32, u32),
) -> Result<PathBuf, RawGifError> {
    //gifsicle --resize-fit 240x240 input.gif -o scaled_240x240.gif
    let mut filename = og_gif_path
        .file_name()
        .ok_or(RawGifError::Filename)?
        .to_owned();
    filename.push(OsStr::new(&format!(
        "_{}x{}.gif",
        target_width, target_height
    )));
    let out_path = Path::new(ROOT_DIR).join(RAW_DIR).join(filename);
    let status = Command::new("gifsicle")
        .arg("--resize-fit")
        .arg(format!("{}x{}", target_width, target_height))
        .arg(og_gif_path)
        .arg("-o")
        .arg(&out_path)
        .status()?;
    if !status.success() {
        return Err(RawGifError::Gifsicle(status));
    }
    Ok(out_path)
}

impl RawGif {
    fn open(path: &Path) -> Result<RawGif, RawGifError> {
        let f = std::fs::File::open(path)?;
        let mut options = gif::DecodeOptions::new();
        options.set_color_output(gif::ColorOutput::RGBA);

        let mut decoder = options.read_info(f)?;
        let (width, height) = (decoder.width(), decoder.height());

        let mut current_frame: Vec<Rgb565> = vec![Rgb565::BLACK; (width * height) as usize];
        let mut frames = Vec::new();

        while let Some(frame) = decoder.read_next_frame()? {
            let delay_ms = if frame.delay == 0 {
                100
            } else {
                u64::from(frame.delay) * 10
            };

            let frame_left = frame.left as usize;
            let frame_top = frame.top as usize;
            let frame_width = frame.width as usize;

            for (i, chunk) in frame.buffer.chunks_exact(4).enumerate() {
                let r = chunk[0];
                let g = chunk[1];
                let b = chunk[2];

                let r5 = r >> 3;
                let g6 = g >> 2;
                let b5 = b >> 3;

                let pixel_color = Rgb565::new(r5, g6, b5);

                let x = frame_left + (i % frame_width);
                let y = frame_top + (i / frame_width);

                current_frame[y * width as usize + x] = pixel_color;
            }

            frames.push((current_frame.clone(), delay_ms));
        }

        return Ok(RawGif { frames });
    }
}

#[derive(Error, Debug)]
pub enum RawGifError {
    #[error("Failed to open raw file : {0}")]
    IO(#[from] std::io::Error),
    #[error("Failed to read gif : {0}")]
    ReadGif(#[from] DecodingError),
    #[error("Couldn't get the file name of original file")]
    Filename,
    #[error("Gifsicle failed with status : {0}")]
    Gifsicle(ExitStatus),
}
