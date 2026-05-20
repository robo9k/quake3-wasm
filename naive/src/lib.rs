use std::{cell::UnsafeCell, ffi::CString};

use i32 as c_int;
use isize as intptr_t;

/// syscallptr from g_syscalls.c
pub type Syscall = extern "C" fn(arg: intptr_t, ...) -> intptr_t;

pub struct SyncCell<T>(UnsafeCell<T>);

unsafe impl<T> Sync for SyncCell<T> {}

impl<T> SyncCell<T> {
    pub const fn new(v: T) -> Self {
        Self(UnsafeCell::new(v))
    }

    pub unsafe fn set(&self, v: T) {
        unsafe { *self.0.get() = v };
    }
}

impl<T: Copy> SyncCell<T> {
    pub unsafe fn get(&self) -> T {
        unsafe { *self.0.get() }
    }
}

static SYSCALL: SyncCell<Option<Syscall>> = SyncCell::new(None);

/// See [ioquake3's `game/g_public.h`](https://github.com/ioquake/ioq3/blob/master/code/game/g_public.h)
const G_ERROR: intptr_t = 1;
const GAME_INIT: c_int = 0;
const GAME_SHUTDOWN: c_int = 1;

/// vmMain() from g_main.c
#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn vmMain(
    command: c_int,
    _arg0: c_int,
    _arg1: c_int,
    _arg2: c_int,
    _arg3: c_int,
    _arg4: c_int,
    _arg5: c_int,
    _arg6: c_int,
    _arg7: c_int,
    _arg8: c_int,
    _arg9: c_int,
    _arg10: c_int,
    _arg11: c_int,
) -> intptr_t {
    match command {
        GAME_INIT => {
            let msg = CString::new("Hello, World!").unwrap();
            unsafe {
                let syscall = SYSCALL.get().unwrap();
                syscall(G_ERROR, msg.as_ptr());
            }
            unreachable!()
        }
        GAME_SHUTDOWN => {
            // Just return a dummy value here for clean shutdown
            0
        }
        _ => panic!("Game command not implemented"),
    }
}

/// dllEntry() from g_syscalls.c
#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn dllEntry(syscall: Syscall) {
    unsafe {
        SYSCALL.set(Some(syscall));
    }
}
