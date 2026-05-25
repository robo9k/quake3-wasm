use quake3_native_vm::Syscall;
use std::sync::Arc;
use std::sync::LazyLock;
use std::sync::RwLock;
use wasmtime::*;

struct State {
    store: Store<() /*Syscall*/>,
    vm_main: TypedFunc<
        (
            i32, // command: c_int
            i32, // arg0: c_int
            i32, // arg1: c_int
            i32, // arg2: c_int
            i32, // arg3: c_int
            i32, // arg4: c_int
            i32, // arg5: c_int
            i32, // arg6: c_int
            i32, // arg7: c_int
            i32, // arg8: c_int
            i32, // arg9: c_int
            i32, // arg10: c_int
            i32, // arg11: c_int
        ),
        i32, // intptr_t
    >,
}

static _VM_IMPL: LazyLock<Arc<RwLock<Option<State>>>> =
    LazyLock::new(|| Arc::new(RwLock::new(None)));

pub fn dll_entry(/*syscall: Syscall*/) -> wasmtime::Result<()> {
    let engine = Engine::default();

    let module = Module::from_file(&engine, "c.wat")?;

    let mut linker = Linker::new(&engine);
    linker.func_wrap("env", "syscall", |arg: i32, args: i32| -> i32 {
        // TODO: depending on `arg`, get arguments from instance memory, convert references
        println!("arg={}", arg);
        println!("args={}", args);
        0
    })?;

    let mut store: Store<() /*Syscall*/> = Store::new(&engine, () /*syscall*/);

    let instance = linker.instantiate(&mut store, &module)?;

    let vm_main = instance.get_typed_func::<(
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
    ), i32>(&mut store, "vmMain")?;

    vm_main.call(&mut store, (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0))?;

    let mut VM_IMPL = _VM_IMPL.write().unwrap();
    *VM_IMPL = Some(State { store, vm_main });

    //println!("executable path: {:?}", process_path::get_executable_path());
    //println!("dylib path: {:?}", process_path::get_dylib_path());

    Ok(())
}
