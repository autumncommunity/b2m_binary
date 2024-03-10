use crate::{
    filesafe::{self, create_file, remove_file},
    get_platform,
};
use reqwest::{blocking::*, header::USER_AGENT};
use serde_json::{from_str, Value};

fn get_module_crc(client: &Client, name: String, full_name: &str, version: &str) -> u32 {
    let version: String = if version == "*" {
        "latest".to_owned()
    } else {
        format!("tags/{}", version)
    };

    let url = format!(
        "https://api.github.com/repos/b2mpackages/{}/releases/{}",
        name, version
    );

    let res = client
        .get(url)
        .header(USER_AGENT, "b2m_binary")
        .send()
        .expect("err");

    if !res.status().is_success() {
        return 0;
    }

    let text: String = res.text().unwrap();
    let json: Value = from_str(&text).unwrap();
    let assets_option = json.get("assets");
    let mut crc_txt_url: &str = "";

    // damn it
    match assets_option {
        Some(assets) => {
            if let Some(assets_array) = assets.as_array() {
                for asset in assets_array {
                    if let Some(asset_object) = asset.as_object() {
                        if let Some(name) = asset_object.get("name").and_then(Value::as_str) {
                            if name == format!("{}.txt", full_name) {
                                if let Some(download_url) = asset_object
                                    .get("browser_download_url")
                                    .and_then(Value::as_str)
                                {
                                    crc_txt_url = download_url;
                                }
                            }
                        }
                    }
                }
            } else {
                println!("Failed to parse assets array");
            }
        }
        None => {
            println!("Couldn't get \"assets\" from {} release", name);
        }
    }

    if crc_txt_url.is_empty() {
        println!("Couldn't get CRC file URL.");
        return 0;
    }

    let res = client
        .get(crc_txt_url)
        .header(USER_AGENT, "b2m_binary")
        .send()
        .expect("err");

    if !res.status().is_success() {
        return 0;
    }

    let text: String = res.text().unwrap().trim().to_string();
    let result: Result<u32, _> = text.parse();

    match result {
        Ok(val) => val,
        Err(err) => {
            println!("Couldn't parse CRC to u32, got error: {}", err.to_string());
            0
        }
    }
}

pub fn download_dll(
    client: &Client,
    full_name: &str,
    name: String,
    version: &str,
    isclient: bool,
) -> bool {
    let mut res = client
        .get(format!(
            "https://autumngmod.ru/b2m/api/downloadBinary?name={}&version={}&platform={}&side={}",
            name,
            version,
            get_platform(),
            if isclient { "cl" } else { "sv" }
        ))
        .send()
        .expect("Couldn't send HTTP request");

    if !res.status().is_success() {
        println!(
            "{}",
            format!(
                "Unable to download dll {}:{} @ Error: {}",
                name,
                version,
                res.status().to_string()
            )
        );

        return false;
    }

    remove_file(full_name);

    let file = create_file(full_name);

    match file {
        Ok(mut file) => {
            let result = res.copy_to(&mut file);

            match result {
                Ok(_) => {
                    let crc32_from_github = get_module_crc(&client, name, full_name, version);
                    let downloaded_file_crc32: u32 = filesafe::calc_crc32(full_name);

                    if downloaded_file_crc32 != crc32_from_github {
                        remove_file(full_name);

                        println!(
                            "CRC32 sums are different. B2M removed {} binary because it may be dangerous.",
                            full_name
                        );

                        return false;
                    }

                    true
                }
                Err(err) => {
                    println!("Error on DLL safe: {}", err);
                    return false;
                }
            }
        }

        Err(err) => {
            println!("Error on DLL downloading: {}", err);
            return false;
        }
    }
}
