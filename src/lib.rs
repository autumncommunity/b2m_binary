#![feature(c_unwind)]

use std::env;

#[macro_use] extern crate gmod;

mod functions;

// todo: make print clearly

unsafe fn push_color(lua: gmod::lua::State, index: i32, r: isize, g: isize, b: isize) {
    lua.new_table();

    lua.push_integer(r);
    lua.set_field(index, lua_string!("r"));

    lua.push_integer(g);
    lua.set_field(index, lua_string!("g"));

    lua.push_integer(b);
    lua.set_field(index, lua_string!("b"));
}

unsafe fn print(lua: gmod::lua::State, message: &str) {
    lua.get_global(lua_string!("MsgC"));

    if lua.is_nil(-1) {
        return
    }

    push_color(lua, -2, 255, 165, 0);

    lua.push_string("[B2M] ");

    push_color(lua, -2, 255, 255, 255);

    lua.push_string(format!("{}\n", message).as_str());
    lua.call(4, 0);

    lua.pop_n(4);
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
unsafe fn gmod13_open(lua: gmod::lua::State) -> i32 {
    //print(lua, "b2m by autumncommunity");
    //print(lua, "Join us! https://discord.gg/HspPfVkHGh");

    println!("yep144");


    functions::initialize(lua);

    println!("suss");


    0
}

#[gmod13_close]
fn gmod13_close(_lua: gmod::lua::State) -> i32 {
    println!("Goodbye from binary module!");
    0
}
