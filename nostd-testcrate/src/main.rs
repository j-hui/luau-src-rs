#![no_std]
#![no_main]

extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate panic_halt;

use panic_halt as _;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

use core::ffi::{c_char, c_int, c_long, c_void};

// #[repr(C)]
// #[allow(non_snake_case, non_camel_case_types)]
// pub struct lua_CompileOptions {
//     optimizationLevel: c_int,
//     debugLevel: c_int,
//     coverageLevel: c_int,
//     vectorLib: *const c_char,
//     vectorCtor: *const c_char,
//     mutableGlobals: *const *const c_char,
// }

extern "C" {
    // pub fn free(ptr: *mut c_void);

    pub fn luaL_newstate() -> *mut c_void;
    pub fn lua_close(state: *mut c_void);
    // pub fn luaL_openlibs(state: *mut c_void);
    pub fn lua_getfield(state: *mut c_void, index: c_int, k: *const c_char) -> c_int;
    pub fn lua_tolstring(state: *mut c_void, index: c_int, len: *mut c_long) -> *const c_char;
    pub fn lua_call(state: *mut c_void, nargs: c_int, nresults: c_int);

    pub fn lua_pushinteger(state: *mut c_void, n: c_int);
    pub fn lua_tointegerx(state: *mut c_void, index: c_int, isnum: *mut c_int) -> c_int;

    // pub fn luau_compile(
    //     source: *const c_char,
    //     size: usize,
    //     options: *mut lua_CompileOptions,
    //     outsize: *mut usize,
    // ) -> *mut c_char;
    pub fn luau_load(
        state: *mut c_void,
        chunkname: *const c_char,
        data: *const c_char,
        size: usize,
        env: c_int,
    ) -> c_int;
}

pub unsafe fn lua_getglobal(state: *mut c_void, k: *const c_char) {
    lua_getfield(state, -102002 /* LUA_GLOBALSINDEX */, k);
}

fn luau_works() {
    use core::{ptr, slice};
    unsafe {
        hprintln!("About to new state!");
        let state = luaL_newstate();
        hprintln!("Created new state!");
        // assert!(state != ptr::null_mut());

        /*
        let version = {
            lua_getglobal(state, "_VERSION\0".as_ptr().cast());
            let mut len: c_long = 0;
            let version_ptr = lua_tolstring(state, -1, &mut len);
            slice::from_raw_parts(version_ptr as *const u8, len as usize)
        };

        hprintln!("Version is {:?}", version);
        */

        /*
        let code = "local a, b = ... return a + b\0";
        let mut bytecode_size = 0;
        let bytecode = luau_compile(
            code.as_ptr().cast(),
            code.len() - 1,
            ptr::null_mut(),
            &mut bytecode_size,
        );
        let result = luau_load(state, "sum\0".as_ptr().cast(), bytecode, bytecode_size, 0);
        assert_eq!(result, 0);
        free(bytecode.cast());
        */

        // Call the loaded function
        // lua_pushinteger(state, 123);
        // lua_pushinteger(state, 321);
        // lua_call(state, 2, 1);
        // let i = lua_tointegerx(state, -1, ptr::null_mut());
        // if i != 444 {
        //     hprintln!("the integer was not correct, it was {} not 444", i);
        //     debug::exit(debug::EXIT_FAILURE);
        // }

        lua_close(state);
        hprintln!("closed state!");
    }
}

#[entry]
fn main() -> ! {
    hprintln!("Hello, world!");

    luau_works();

    hprintln!("about to exit!");
    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}
