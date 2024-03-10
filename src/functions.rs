use crate::{
    crc32::calc_crc32_file, dll::download_dll, filesafe::remove_file, get_module_full_name,
    get_platform, print, versionchecker,
};
use gmod::lua::State;
use lazy_static::lazy_static;
use reqwest::blocking::*;
use serde_json::{from_str, Value};
use std::thread;

lazy_static! {
    static ref CLIENT: Client = Client::new();
}

unsafe extern "C-unwind" fn lua_print(lua: State) -> i32 {
    let msg = lua.check_string(1).to_string();
    print(lua, &msg);

    0
}

// Проверяет модуль.
//      Если модуль не скачан - то он скачивает его, и возвращает true
//      Если скачать не получилось - то он возвращает false
fn check_module(name: String, version: &str, isclient: bool) -> bool {
    let full_name: String = get_module_full_name(name.clone(), isclient);
    let crc32: u32 = calc_crc32_file(format!("{}\\{}", "garrysmod\\lua\\bin", &full_name));

    if crc32 == 0 {
        return download_dll(&CLIENT, &full_name, name, version, isclient);
    }

    let res = CLIENT
        .get(format!(
            "https://autumngmod.ru/b2m/api/packages/getCRC32?name={}&version={}&platform={}&side={}",
            name,
            version,
            get_platform(),
            if isclient { "cl" } else { "sv" }
        ))
        .send()
        .expect("Couldn't send HTTP request");

    if !res.status().is_success() {
        return false;
    }

    let sv_crc32: u32;
    let str_sv_crc32: Result<u32, _> = res.text().unwrap().parse();

    match str_sv_crc32 {
        Ok(result) => {
            sv_crc32 = result;
        }
        Err(_) => {
            return false;
        }
    }

    if sv_crc32 != crc32 {
        return download_dll(&CLIENT, &full_name, name, version, isclient);
    }

    return true;
}

// возвращает либо последнюю версию, либо false если не (получилось получить) версию
unsafe extern "C-unwind" fn lua_check_module(lua: State) -> i32 {
    let name: String = lua.check_string(1).to_string();
    let version_: String = lua.check_string(2).to_string();
    let version: &str = version_.trim();
    let isclient: bool = lua.check_boolean(3);
    let is_downloaded: bool = check_module(name.clone(), version, isclient);

    if version == "*" {
        let res = CLIENT
            .get(format!(
                "https://autumngmod.ru/b2m/api/packages/getVersion?name={}",
                name
            ))
            .send()
            .expect("Couldn't send HTTP request");

        let status = res.status();
        let text = res.text().unwrap();
        let latest_version = text.as_str();

        if status.is_success() && is_downloaded {
            lua.push_string(latest_version);
        } else {
            lua.push_boolean(false);
        }
    } else if is_downloaded {
        lua.push_string(version)
    } else {
        lua.push_boolean(false);
    }

    1
}

// for menu-side (downloads packages from server)
unsafe extern "C-unwind" fn check_packages(lua: State) -> i32 {
    let ip = lua.check_string(1).to_string();

    thread::spawn(move || {
        let res = CLIENT
            .get(&format!(
                "https://autumngmod.ru/b2m/api/packages/getServer?serverIP={}",
                ip
            ))
            .send()
            .expect("Failed to send request");

        let status = res.status();
        let text = res.text();
        let txt = text.unwrap();

        if !status.is_success() {
            return;
        }

        let json_sus: Value = from_str(txt.as_str()).expect("an error occured");

        for cell in json_sus.as_object().unwrap() {
            match cell {
                (key, value_) => {
                    let value: &str = value_.as_str().unwrap();
                    check_module(key.to_string(), value, true);
                }
            }
        }
    });

    0
}

// b2m.Remove(name, isclient)
unsafe extern "C-unwind" fn remove(lua: State) -> i32 {
    let name: String = lua.check_string(1).to_string();
    let isclient: bool = lua.check_boolean(2);
    let full_name: String = get_module_full_name(name, isclient);

    remove_file(&full_name);

    0
}

pub unsafe fn initialize(lua: State) {
    let name = lua_string!("b2m");

    lua.get_global(name);

    if lua.is_nil(-1) {
        lua.pop();
        lua.new_table()
    }

    // check the version

    let is_new: bool = versionchecker::is_newest(lua, &CLIENT);

    if !is_new {
        return;
    }

    lua.new_table();
    lua.push_value(-2);
    lua.set_field(-2, lua_string!("Packages"));

    lua.new_table();
    lua.push_value(-2);
    lua.set_field(-2, lua_string!("DB"));

    lua.push_function(lua_print);
    lua.set_field(-2, lua_string!("Print"));

    lua.push_function(remove);
    lua.set_field(-2, lua_string!("Remove"));

    lua.push_function(check_packages);
    lua.set_field(-2, lua_string!("CheckPackages"));

    lua.push_function(lua_check_module);
    lua.set_field(-2, lua_string!("CheckModule"));

    lua.push_string(env!("CARGO_PKG_VERSION"));
    lua.set_field(-2, lua_string!("Version"));

    lua.set_global(name);
}
