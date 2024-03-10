use std::{
    fs::{self, File},
    path::Path,
};

use crate::crc32;

static DLL_PLACE: &str = "garrysmod\\lua\\bin";
static UNSAFE_PATTERNS: [&'static str; 2] = ["../", "..\\"];

fn is_path_unsafe(input: &str) -> bool {
    for i in UNSAFE_PATTERNS {
        if input.contains(i) {
            return true;
        }
    }

    false
}

pub fn exists(file: &str) -> bool {
    if is_path_unsafe(file) {
        return false;
    }

    return Path::new(file).exists();
}

pub fn is_file(file: &str) -> bool {
    if is_path_unsafe(file) {
        return false;
    }

    return Path::new(file).is_file();
}

pub fn remove_file(file: &str) -> bool {
    let path = format!("{}\\{}", DLL_PLACE, file);

    if is_path_unsafe(file) || !exists(&path) || !is_file(&path) {
        return false;
    }

    let file = fs::remove_file(path);

    match file {
        Ok(_) => return true,
        Err(_) => return false,
    }
}

pub fn calc_crc32(file: &str) -> u32 {
    let path = format!("{}\\{}", DLL_PLACE, file);

    if is_path_unsafe(file) || !exists(&path) || !is_file(&path) {
        return 0;
    }

    crc32::calc_crc32_file(path)
}

pub fn create_file(file: &str) -> Result<File, &'static str> {
    let path = format!("{}\\{}", DLL_PLACE, file);

    if is_path_unsafe(file) {
        return Err("Path using unsafe symbols");
    }

    let file = File::create_new(path);

    match file {
        Ok(file) => Ok(file),
        Err(_) => Err("Cannot create file!"),
    }
}
