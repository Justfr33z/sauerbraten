mod sauerbraten;
mod module;
mod utils;

use crate::module::manager::ModuleManager;
use winapi::{
    um::{
        winnt,
        wincon,
        handleapi,
        consoleapi,
        libloaderapi,
        processthreadsapi,
    },
    ctypes,
    _core::ptr,
    shared::minwindef,
};

#[allow(non_snake_case)]
#[no_mangle]
pub extern "stdcall" fn DllMain(hinst_dll: minwindef::HINSTANCE, fdw_reason: u32, _: minwindef::LPVOID) -> i32 {
    match fdw_reason {
        winnt::DLL_PROCESS_ATTACH => {
            unsafe {
                libloaderapi::DisableThreadLibraryCalls(hinst_dll);

                handleapi::CloseHandle(
                    processthreadsapi::CreateThread(
                        ptr::null_mut(),
                        0,
                        Some(entry_point),
                        hinst_dll as *mut ctypes::c_void,
                        0,
                        ptr::null_mut()
                    )
                );
            }
        }
        winnt::DLL_THREAD_ATTACH => {}
        winnt::DLL_THREAD_DETACH => {}
        winnt::DLL_PROCESS_DETACH => {}
        _ => {}
    };

    true as i32
}

unsafe extern "system" fn entry_point(hinst_dll: *mut ctypes::c_void) -> u32 {
    consoleapi::AllocConsole();

    let mut manager = ModuleManager {
        modules: vec![]
    };

    manager.start();

    wincon::FreeConsole();
    libloaderapi::FreeLibraryAndExitThread(hinst_dll as *mut minwindef::HINSTANCE__, 0);
    0
}