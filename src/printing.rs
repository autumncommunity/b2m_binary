use gmod::lua::State;

unsafe fn push_color(lua: State, index: i32, r: isize, g: isize, b: isize) {
    lua.new_table();

    lua.push_integer(r);
    lua.set_field(index, lua_string!("r"));

    lua.push_integer(g);
    lua.set_field(index, lua_string!("g"));

    lua.push_integer(b);
    lua.set_field(index, lua_string!("b"));
}

pub unsafe fn print(lua: State, message: &str) {
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
