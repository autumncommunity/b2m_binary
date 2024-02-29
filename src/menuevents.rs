use gmod::lua::State;
use libc::{c_char, c_int, c_void};
use std::{ffi::CString, ptr::null};

#[allow(non_snake_case)]
trait IGameEventManager2 {
    fn Reset(&self);
}

unsafe extern "C-unwind" fn listen(_lua: State) -> i32 {
    0
}

unsafe fn initialize_lua(lua: State) {
    let string_gameevent: *const i8 = lua_string!("gameevent");
    let string_listen: *const i8 = lua_string!("Listen");

    lua.get_global(string_gameevent);

    if lua.is_nil(-1) {
        lua.pop();
        lua.new_table();
    }

    lua.push_function(listen);
    lua.set_field(-2, string_listen);

    lua.set_global(string_gameevent);
}

pub unsafe fn initialize(lua: State) {
    let (engine, _engine_path): (gmod::libloading::Library, &'static str) =
        open_library!("engine").expect("Failed to open engine.dll!");

    // пушим таблицу gameevent в луа

    initialize_lua(lua);

    // получаем функцию CreateInterfaceFn factory
    let interface: gmod::libloading::Symbol<
        unsafe extern "C" fn(*const c_char, *const c_int) -> *mut dyn IGameEventManager2,
    > = engine
        .get(b"CreateInterface")
        .expect("Couldn't get CreateInterface"); // возвращает адрес как символ

    // Принтим адрес interface
    println!("Interface address: {:p}", &interface);

    // Обьявляем class_name of GAMEEVENTSMANAGER002
    let class_name: CString = CString::new("GAMEEVENTSMANAGER002").unwrap();

    // Гетмаем event_manager
    let event_manager_ptr = interface(class_name.as_ptr(), null());

    // Принтим адрес event_manager
    println!("Event Manager address: {:p}", &event_manager_ptr); // принтим адрес
    println!("Event Manager is null: {}", event_manager_ptr.is_null());

    let event_manager: Box<dyn IGameEventManager2> = unsafe { Box::from_raw(event_manager_ptr) };

    event_manager.Reset();
}
