use std::{env::home_dir, path::PathBuf, sync::LazyLock};

pub static ROOT_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| home_dir().unwrap().join(".local/share/itaberry"));
pub const RAW_DIR: &str = "raw";
pub const OG_DIR: &str = "og";
pub const SERVE_ADDR: &str = "0.0.0.0:5901";
pub const MAX_SIZE_LIMIT_100MB: usize = 1 << 27; // 2^27 ~ 134 MB
pub const SPI_CLOCK_SPEED_HZ: u32 = 60_000_000;
pub const BUFFER_SIZE: usize = 4096;
