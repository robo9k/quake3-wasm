```terminal
$ cargo build
$ wasm-tools demangle target/wasm32-unknown-unknown/debug/quake3_wasm.wasm -o quake3_wasm.wasm
$ wasm-tools print quake3_wasm.wasm -o quake3_wasm.wat
```

## NOTEs

- .qvm does not have `dllEntry` since `syscall` fn is directly mapped via `*-syscalls.asm`
  to a negative .. pointer?  
  "Syscall" is the only QVM bytecode CALL that would need special Wasm transpiler handling anyway
- `quake3-native-vm::native_vm!` (which does have `dllEntry`) does currently depend on
  - `core::ffi::c_int`, `libc::intptr_t`
  - `alloc::sync::Arc`, `std::sync::RwLock`
  - `once_cell::sync::Lazy`

  However other than the FFI types those all seem to compile fine with `wasm32-unknown-unknown`
- `dllEntry` from Rust .wasm
  ```wat
  (type (;3;) (func (param i32)))
  ```
- `vmMain` from Rust .wasm
  ```wat
  (type (;10;) (func (param i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32) (result i32)))
  ```
- Wasm does not have C varargs  
  https://github.com/WebAssembly/tool-conventions/blob/main/BasicCABI.md#:~:text=Varargs  
  https://wingolog.org/archives/2023/03/20/a-world-to-win-webassembly-for-the-rest-of-us#:~:text=Scheme%20to%20Wasm%3A%20Varargs
- Quake 3 .map files could be used as debug info  
  https://github.com/WebAssembly/tool-conventions/blob/main/Debugging.md  
  https://github.com/robo9k/quake3-qvm/blob/master/assets/ioq3/baseq3/vm/qagame.map
- There is a limited Wasm feature subset that even embedded runtimes might implement  
  https://github.com/WebAssembly/tool-conventions/blob/main/Lime.md  
  Which might make sense for a "Quake 3 QVM" .. target? Remember the Dreamcast


## TODOs

- Build .wasm from C  
  https://github.com/robo9k/quake3-qvm/blob/master/assets/mod-syscall.c
- Build .wasm from simpler (unsafe) Rust  
  https://github.com/robo9k/q3hi.rs
- Build `quake3-native-vm::native_vm!` with panic=abort
- Build hand-made .wat Hello World  
  Export one memories (data, LIT, BSS)  
  Export functions `dllEntry`, `vmMain`  
  Use a `global` / `table` for syscall fn pointer?
- Build a playground CLI with Wasmtime  
  https://docs.wasmtime.dev/examples-hello-world.html
- Eventually build a native Quake 3 module akin to https://github.com/robo9k/q3py/  
  Which needs something similar to the hardcoded `VMA()` in e.g. `SV_GameSystemCalls` to convert pointer args into Wasm memories pointers  
  The other way with `VM_Call()` does not pass pointers
- Build a .qvm to .wasm `qvm2wasm` .. transpiler?
- Ideally the native module would work both for .wasm built directly from C / Rust and `qvm2wasm` alike
