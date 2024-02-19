// b2m.HandleServer("0.0.0.0:27015")

use std::thread;
use reqwest::blocking::*;
use lazy_static::lazy_static;
use serde_json::{from_str, Value};

lazy_static! {
    static ref CLIENT: Client = Client::new();
}

unsafe extern "C-unwind" fn handle_server(lua: gmod::lua::State) -> i32 {
    let ip = lua.check_string(1).to_string();

    println!("yeppy");

    //Выводим все это в отдельный поток, чтобы не блокировать поток Lua
    thread::spawn(move || {
        let res = CLIENT.get(&format!("https://zetaproduct.ru/api/version?ip={}", ip))
            .send()
            .expect("Failed to send request");

        if res.status().is_success() {
            let json_sus: Value = from_str(res.text().unwrap().as_str()).expect("an error occured");

            println!("{:?}", json_sus);
        }
    });

    0
}

pub unsafe fn initialize(lua: gmod::lua::State) {
    println!("yep11");

    let name = lua_string!("b2m");

    lua.get_global(name);

    if lua.is_nil(-1) {
        lua.pop();
        lua.new_table()
    }

    lua.push_function(handle_server);
    lua.set_field(-2, lua_string!("HandleServer"));

    lua.set_global(name);

    println!("yep12");
}