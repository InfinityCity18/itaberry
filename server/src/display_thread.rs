use std::{
    collections::HashMap,
    error::Error,
    path::Path,
    sync::{Arc, Mutex, mpsc},
    thread::JoinHandle,
    time::Duration,
};

use tokio::sync::oneshot;
use tracing::{error, trace, warn};

use crate::{
    constants::ROOT_DIR,
    display::{AnyDisplay, load_config, load_displays_from_config},
    raw::{RawFile, RawImage, get_raw},
    webserver::DisplayConfigWeb,
};

pub enum DisplayCommand {
    GetDisplaysInfo(oneshot::Sender<Vec<DisplayConfigWeb>>),
    SetDisplay(i32, String),
}

pub fn display_thread(
    mut rx: tokio::sync::mpsc::Receiver<DisplayCommand>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let configs = load_config(&Path::new(&*ROOT_DIR).join("config.toml"))?;
    let configs: Vec<Arc<Mutex<DisplayConfigWeb>>> = configs
        .into_iter()
        .map(|conf| Arc::new(Mutex::new(conf.into())))
        .collect();
    let displays = load_displays_from_config(&Path::new(&*ROOT_DIR).join("config.toml"))?;
    let mut worker_channels: HashMap<i32, mpsc::Sender<String>> = HashMap::new();
    for (disp, conf) in displays.into_iter().zip(configs.iter()) {
        let arc = conf.clone();
        let (tx, rx) = mpsc::channel();
        if worker_channels.insert(disp.id(), tx).is_some() {
            warn!("Displays were assigned equal ids, overwritten display may not receive commands");
        }
        std::thread::spawn(|| worker_thread(disp, arc, rx));
    }
    loop {
        while let Some(cmd) = rx.blocking_recv() {
            match cmd {
                DisplayCommand::GetDisplaysInfo(tx) => {
                    if tx.send(clone_config(&configs)).is_err() {
                        error!("Oneshot channel for GetDisplaysInfo failed to send");
                    }
                }
                DisplayCommand::SetDisplay(id, filename) => {
                    if let Some(tx) = worker_channels.get(&id) {
                        if let Err(e) = tx.send(filename.clone()) {
                            error!("Failed to send image {} to display {id} : {e}", filename);
                        }
                    } else {
                        warn!(
                            "Tried to send image {filename} to display {id}, no such display with this id was found"
                        );
                    }
                }
            }
        }
    }
}

fn worker_thread(
    mut disp: Box<dyn AnyDisplay>,
    conf: Arc<Mutex<DisplayConfigWeb>>,
    rx: mpsc::Receiver<String>,
) {
    let mut buf = None;
    loop {
        let filename: String = if buf.is_some() {
            let tmp = buf.unwrap();
            buf = None;
            tmp
        } else {
            rx.recv().unwrap()
        };
        trace!("Worker thread with id: {} got task", disp.id());
        match get_raw(&filename, disp.size()) {
            Ok(rawfile) => {
                conf.lock().unwrap().current_image = Some(filename);
                match rawfile {
                    RawFile::Image(rawimage) => {
                        if let Err(e) = disp.display_image(&rawimage) {
                            warn!("Displaying image error : {e}");
                        }
                    }
                    RawFile::Gif(rawgif) => {
                        for (rawimage, frame_time) in rawgif.frames.iter().cycle() {
                            if let Err(e) = disp.display_image(rawimage) {
                                warn!("Displaying image (gif) error : {e}");
                            }
                            if let Ok(new_filename) = rx.try_recv() {
                                buf = Some(new_filename);
                                break;
                            }
                            std::thread::sleep(Duration::from_millis(*frame_time));
                        }
                    }
                }
            }
            Err(e) => warn!("Error in get_raw : {e}"),
        }
    }
}

fn clone_config(v: &Vec<Arc<Mutex<DisplayConfigWeb>>>) -> Vec<DisplayConfigWeb> {
    v.iter().map(|c| (*c.lock().unwrap()).clone()).collect()
}
