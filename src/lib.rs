#![feature(c_unwind)]

#[macro_use]
extern crate gmod;

mod crc32;
mod dll;
mod filesafe;
mod functions;
mod printing;
mod versionchecker;

use gmod::lua::State;
use printing::print;
use std::env;

fn get_module_full_name(name: String, isclient: bool) -> String {
    format!(
        "{}_{}_{}.dll",
        get_module_prefix(isclient),
        name,
        get_platform()
    )
}

fn get_module_prefix(isclient: bool) -> String {
    format!("gm{}", if isclient { "cl" } else { "sv" })
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
