use crate::{
    crc32::calc_crc32_file,
    dll::{download_dll, remove_dll},
    get_is_client, get_module_full_name, get_module_prefix, get_platform, print,
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

// TODO: * version is latest
// возвращает bool; если true то модуль был скачан, если false то иди нахуй
unsafe fn check_module(name: String, version: String) -> bool {
    let full_name: String = get_module_full_name(name.clone());
    let crc32: u32 = calc_crc32_file(format!("{}{}", "./garrysmod/lua/bin/", &full_name)); // * .dll can be not only in garrysmod/lua/bin

    //println!("Module:\n\t{}\n\t{}\n\t{}", name, crc32, version); // its dev print

    if crc32 == 0 {
        download_dll(&CLIENT, full_name, name, version);

        return true;
    }

    let res = CLIENT
        .get(format!(
            "http://localhost/api/packages/getCRC32?name={}&version={}&platform={}&side={}",
            name,
            version,
            get_platform(),
            if get_is_client() { "cl" } else { "sv" }
        ))
        .send()
        .expect("Couldn't send HTTP request");

    if !res.status().is_success() {
        return false;
    }

    let sv_crc32: u32;
    let res_text = res.text().unwrap();
    let str_sv_crc32: Result<u32, _> = res_text.clone().parse();

    match str_sv_crc32 {
        Ok(res) => {
            sv_crc32 = res;
        }
        Err(_) => {
            return false;
        }
    }

    if sv_crc32 != crc32 {
        download_dll(&CLIENT, full_name, name, version);

        return true;
    }

    return false;
}

unsafe extern "C-unwind" fn lua_check_module(lua: State) -> i32 {
    let name: String = lua.check_string(1).to_string();
    let version: String = lua.check_string(2).to_string();
    let is_downloading: bool = check_module(name.clone(), version);

    if is_downloading {
        let res = CLIENT
            .get(format!(
                "http://localhost/api/packages/getVersion?name={}",
                name
            ))
            .send()
            .expect("Couldn't send HTTP request");

        if res.status().is_success() {
            lua.push_string(res.text().unwrap().as_str());
        } else {
            lua.push_boolean(is_downloading);
        }
    } else {
        lua.push_boolean(is_downloading);
    }

    1
}

unsafe extern "C-unwind" fn check_packages(lua: State) -> i32 {
    let ip = lua.check_string(1).to_string();
    let prefix = get_module_prefix();

    print(lua, &prefix);

    //Выводим все это в отдельный поток, чтобы не блокировать поток Lua

    thread::spawn(move || {
        let res = CLIENT
            .get(&format!(
                "http://localhost/api/packages/getServer?serverIP={}",
                ip
            ))
            .send()
            .expect("Failed to send request");

        if !res.status().is_success() {
            return;
        }

        let json_sus: Value = from_str(res.text().unwrap().as_str()).expect("an error occured");

        for cell in json_sus.as_object().unwrap() {
            match cell {
                (key, value_) => {
                    let value: &str = value_.as_str().unwrap();
                    check_module(key.to_string(), value.to_string());
                }
            }
        }
    });

    0
}

unsafe extern "C-unwind" fn remove(lua: State) -> i32 {
    let name: String = lua.check_string(1).to_string();
    let full_name: String = get_module_full_name(name);

    remove_dll(full_name);

    0
}

pub unsafe fn initialize(lua: State) {
    let name = lua_string!("b2m");

    lua.get_global(name);

    if lua.is_nil(-1) {
        lua.pop();
        lua.new_table()
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
