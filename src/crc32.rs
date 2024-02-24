use crc32fast::Hasher;
use std::{fs::File, io::Read, path::Path};

pub fn calc_crc32(text: &[u8]) -> u32 {
    let mut hasher = Hasher::new();
    hasher.update(text);
    hasher.finalize()
}

#[allow(dead_code)]
pub fn calc_crc32_string(text: String) -> u32 {
    calc_crc32(text.as_bytes())
}

pub fn calc_crc32_file(path: String) -> u32 {
    let path_obj = Path::new(&path);
    if !path_obj.exists() || path_obj.is_dir() {
        return 0;
    }

    let mut file = File::open(path).expect("Couldn't open file");
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).expect("Couldn't read file");

    calc_crc32(&contents)
}
