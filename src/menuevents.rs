use gmod::{detour::Function, lua::State};
use libc::{c_char, c_int};
use std::{ffi::CString, ptr::null};

// TODO: make this

trait IGameEventManager2 {
    fn load_events_from_file(&self, filename: &str) -> i32;
    fn Reset(&self);
    fn add_listener(
        &self,
        listener: &dyn IGameEventListener2,
        name: &str,
        b_server_side: bool,
    ) -> bool;

    fn find_listener(&self, listener: &dyn IGameEventListener2, name: &str) -> bool;
    fn remove_listener(&self, listener: &dyn IGameEventListener2);
    fn create_event(&self, name: &str, b_force: bool) -> Option<Box<dyn IGameEvent>>;
    fn fire_event(&self, event: Box<dyn IGameEvent>, b_dont_broadcast: bool) -> bool;
    fn fire_event_client_side(&self, event: Box<dyn IGameEvent>) -> bool;
    fn duplicate_event(&self, event: Box<dyn IGameEvent>) -> Box<dyn IGameEvent>;
    fn free_event(&self, event: Box<dyn IGameEvent>);
    fn serialize_event(&self, event: Box<dyn IGameEvent>, buf: &mut bf_write) -> bool;
    fn unserialize_event(&self, buf: &mut bf_read) -> Box<dyn IGameEvent>; // create new KeyValues, must be deleted
}

impl dyn IGameEventManager2 {
    fn Reset(&self) {}
}

//impl Debug for dyn IGameEventListener2 {}

trait IGameEventListener2 {}

trait IGameEvent {}

#[allow(non_camel_case_types)]
#[allow(dead_code)]
struct bf_write {}

#[allow(non_camel_case_types)]
#[allow(dead_code)]
struct bf_read {}

pub unsafe fn initialize(_lua: State) {
    let (engine, _engine_path): (gmod::libloading::Library, &'static str) =
        open_library!("engine").expect("Failed to open engine.dll!");

    let interface: gmod::libloading::Symbol<
        unsafe extern "C" fn(*const c_char, *const c_int) -> *mut dyn IGameEventManager2,
    > = engine
        .get(b"CreateInterface")
        .expect("Couldn't get CreateInterface"); // возвращает адрес как символ

    println!("Interface: {:?}", interface);

    let arg_string: CString = CString::new("GAMEEVENTSMANAGER002").unwrap();

    if !interface.to_ptr().is_null() {
        let res: *mut dyn IGameEventManager2 = interface(arg_string.as_ptr(), null());
        println!("{:p}", &res);

        let event_manager = &mut *res;

        //IGameEventManager2::Reset(event_manager);
    }
}
