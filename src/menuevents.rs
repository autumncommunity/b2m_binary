use gmod::lua::State;
struct _IGameEventManager2 {}

struct CreateInterfaceFn {}

// TODO: make this

pub unsafe fn initialize(_lua: State) {
    let (engine, engine_path): (gmod::libloading::Library, &'static str) =
        open_library!("engine").expect("Failed to open engine.dll!");

    //let interface: gmod::libloading::Symbol<CreateInterfaceFn> = engine
    //    .get(b"CreateInterface")
    //    .expect("Couldn't get CreateInterface");
    //
    //println!("CreateInterface -> {:?}", interface);
}
