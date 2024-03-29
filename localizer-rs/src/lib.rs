#![allow(clippy::missing_safety_doc)]

#[macro_use]
extern crate lazy_static;

mod executable_range;
mod handlers;
mod helpers;

use anyhow::Result;
use executable_range::Range;
use mach_o_sys::dyld::uintptr_t;
use oslog::OsLogger;
use std::{
    collections::HashMap,
    ffi::CStr,
    fs::File,
    io::{BufRead, BufReader},
    os::raw::c_char,
    path::PathBuf,
    ptr,
    str::FromStr,
    sync::RwLock,
};
use uuid::Uuid;

lazy_static! {
    static ref EXECUTABLE_RANGES: RwLock<HashMap<Uuid, Range>> = RwLock::new(HashMap::new());
    static ref IMAGES_TO_TRANSLATE: RwLock<HashMap<Uuid, String>> = RwLock::new(HashMap::new());
}

#[no_mangle]
pub unsafe extern "C" fn init_localizer(to_localize_file_path: *const c_char) {
    OsLogger::new("ru.danpashin.localizer-rs")
        .level_filter(log::LevelFilter::Trace)
        .init()
        .unwrap();

    let path = CStr::from_ptr(to_localize_file_path);
    let path = PathBuf::from_str(&path.to_string_lossy()).unwrap();
    log::debug!("init_localizer() {:?}", path);

    if let Err(error) = init(path) {
        log::error!("{:?}", error);
    }
}

#[no_mangle]
pub unsafe extern "C" fn translation_file_name_for_address(address: uintptr_t) -> *const c_char {
    let images = IMAGES_TO_TRANSLATE.read().expect("Can't lock for writing");
    let file_name = EXECUTABLE_RANGES
        .read()
        .expect("Can't lock for writing")
        .iter()
        .find(|(_, range)| range.contains_address(address))
        .and_then(|(uuid, _)| images.get(uuid));

    match file_name {
        Some(file_name) => file_name.as_ptr() as *const c_char,
        None => ptr::null(),
    }
}

fn init(file_to_parse: PathBuf) -> Result<()> {
    let reader = BufReader::new(File::open(file_to_parse)?);

    let mut images = IMAGES_TO_TRANSLATE.write().expect("Can't lock for writing");
    for line in reader.lines().flatten() {
        if let Some((uuid, resource_name)) = line.split_once(':') {
            let uuid = Uuid::parse_str(uuid)?;
            let resource_name = resource_name.to_string();
            images.insert(uuid, resource_name);
        } else {
            log::warn!("Invalid line in file! {}", line);
        }
    }

    drop(images);

    handlers::init();

    Ok(())
}
