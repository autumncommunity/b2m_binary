use crate::{get_is_client, get_platform};
use reqwest::blocking::*;
use std::{
    fs::{self, File},
    path::Path,
};

// посути, скачивание можно вывести в отдельный поток
// но эта идея говно, потому что велик шанс того
// что когда игрок будет заходить на сервер, и паралелльно
// он будет скачивать модули, то когда клиент начнёт
// запускать Lua файлы, модуль не будет скачан, что может
// привести к ошибкам
pub fn download_dll(client: &Client, full_name: String, name: String, version: String) {
    let mut res = client
        .get(format!(
            "http://localhost/downloadBinary?name={}&version={}&platform={}&side={}",
            name,
            version,
            get_platform(),
            if get_is_client() { "cl" } else { "sv" }
        ))
        .send()
        .expect("Couldn't send HTTP request");

    if !res.status().is_success() {
        println!("{}", format!("Unable to download dll {}:{}", name, version));

        return;
    }

    remove_dll(full_name.clone());

    let mut file =
        File::create(format!("./garrysmod/lua/bin/{}", full_name)).expect("Couldn't create file");
    let _ = res.copy_to(&mut file);
}

pub fn remove_dll(name: String) {
    let full_path = format!("./garrysmod/lua/bin/{}", name);
    let path = Path::new(&full_path);

    if path.exists() && path.is_file() {
        let _ = fs::remove_file(path);
    }
}
