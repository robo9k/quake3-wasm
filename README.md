```terminal
$ cargo build --release

$ wasm-tools demangle target/wasm32-unknown-unknown/release/native.wasm -o native.wasm
$ wasm-tools print native.wasm -o native.wat

$ wasm-tools demangle target/wasm32-unknown-unknown/release/naive.wasm -o naive.wasm
$ wasm-tools print naive.wasm -o naive.wat

$ clang \
   --target=wasm32 \
   -fvisibility=hidden \
   -O3 \
   -flto \
   -nostdlib \
   -Wl,--no-entry \
   -Wl,--export-dynamic \
   -Wl,--lto-O3 \
   -o c.wasm \
   c/{g_main,g_syscalls}.c
$ wasm-tools print c.wasm -o c.wat
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
  C and Rust agree on `syscall` fn being:
  ```wat
  (type (;0;) (func (param i32 i32) (result i32)))
  ```
  varargs are passed as a i31/pointer to (stack) array  
  length is _not_ passed and has to be deduced from args  
  https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=d7c6f5b641e45a410577d6c15c56a6fb
  ```rust
  unsafe extern "C" { fn foo(...); }

  #[unsafe(no_mangle)]
  pub extern "C" fn bar() {
      unsafe { foo(1, 2, 3, 4); }
  }
  ```
  ```wat
  (type (;0;) (func (param i32)))
  (import "env" "foo" (func $foo (;0;) (type 0)))
  (func $bar (;1;) (type 1)
    (local i32)

    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.set 0
    local.get 0
    global.set $__stack_pointer

    local.get 0
    i32.const 4
    i32.store offset=12

    local.get 0
    i32.const 3
    i32.store offset=8

    local.get 0
    i32.const 2
    i32.store offset=4

    local.get 0
    i32.const 1
    i32.store

    local.get 0

    call $foo

    local.get 0
    i32.const 16
    i32.add
    global.set $__stack_pointer
    return
  )
  ```
  https://doc.rust-lang.org/beta/unstable-book/language-features/c-variadic.html  
  Luckily we do not need to export a vararg fn, just pass-through a pointer  
  We need to handle each syscall, see VMA() note further below
- C/Rust do not properly use Wasm `funcref` for syscall pointer, plain i32 does not work for host functions
- Quake 3 .map files could be used as debug info  
  https://github.com/WebAssembly/tool-conventions/blob/main/Debugging.md  
  https://github.com/robo9k/quake3-qvm/blob/master/assets/ioq3/baseq3/vm/qagame.map
- There is a limited Wasm feature subset that even embedded runtimes might implement  
  https://github.com/WebAssembly/tool-conventions/blob/main/Lime.md  
  Which might make sense for a "Quake 3 QVM" .. target? Remember the Dreamcast


## TODOs

- [x] Build .wasm from C  
      https://github.com/robo9k/quake3-qvm/blob/master/assets/mod-syscall.c
- [x] Build .wasm from simpler (unsafe) Rust  
      https://github.com/robo9k/q3hi.rs
- [x] Build with panic=abort
- [x] Build with `CStr` and no allocation
- [x] Build as `no_std`
- [x] Build with --release
- [ ] Build hand-made .wat Hello World  
      Export one memories (data, LIT, BSS)  
      Export functions `dllEntry`, `vmMain`  
      Use a `global` / `table` for syscall fn pointer?
- [ ] Build a playground CLI with Wasmtime  
      https://docs.wasmtime.dev/examples-hello-world.html
- [ ] Eventually build a native Quake 3 module akin to https://github.com/robo9k/q3py/  
      Which needs something similar to the hardcoded `VMA()` in e.g. `SV_GameSystemCalls` to convert pointer args into Wasm memories pointers  
      The other way with `VM_Call()` does not pass pointers
- [ ] Build a .qvm to .wasm `qvm2wasm` .. transpiler?
- [ ] Ideally the native module would work both for .wasm built directly from C / Rust and `qvm2wasm` alike
