#![feature(c_unwind)]

#[macro_use]
extern crate gmod;

mod crc32;
mod dll;
mod functions;

use gmod::lua::State;
use lazy_static::lazy_static;
use std::{env, sync::Mutex};

lazy_static! {
    static ref IS_CLIENT: Mutex<bool> = Mutex::new(false);
}

// todo: make print clearly

unsafe fn push_color(lua: State, index: i32, r: isize, g: isize, b: isize) {
    lua.new_table();

    lua.push_integer(r);
    lua.set_field(index, lua_string!("r"));

    lua.push_integer(g);
    lua.set_field(index, lua_string!("g"));

    lua.push_integer(b);
    lua.set_field(index, lua_string!("b"));
}

unsafe fn print(lua: State, message: &str) {
    lua.get_global(lua_string!("MsgC"));

    if lua.is_nil(-1) {
        return;
    }

    push_color(lua, -2, 255, 165, 0);

    lua.push_string("[B2M] ");

    push_color(lua, -2, 255, 255, 255);

    lua.push_string(format!("{}\n", message).as_str());
    lua.call(4, 0);
}

unsafe fn get_module_full_name(name: String) -> String {
    format!("{}_{}_{}.dll", get_module_prefix(), name, get_platform())
}

unsafe fn get_module_prefix() -> String {
    "gm".to_owned()
        + (if *IS_CLIENT.lock().unwrap() {
            "cl"
        } else {
            "sv"
        })
}

fn get_platform() -> &'static str {
    let is_x86_64 = gmod::is_x86_64();

    match (env::consts::OS, is_x86_64) {
        ("windows", true) => "win64",
        ("windows", false) => "win32",
        ("linux", true) => "linux64",
        ("linux", false) => "linux",
        ("macos", _) => "osx",
        _ => "unknown",
    }
}

#[gmod13_open]
unsafe fn gmod13_open(lua: State) -> i32 {
    print(lua, "B2M by autumncommunity");
    print(lua, "Join us! https://discord.gg/HspPfVkHGh");

    functions::initialize(lua);

    0
}

#[gmod13_close]
fn gmod13_close(_lua: State) -> i32 {
    0
}
