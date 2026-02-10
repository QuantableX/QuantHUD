// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // Single instance check using a Windows named mutex
    #[cfg(target_os = "windows")]
    {
        type HANDLE = *mut std::ffi::c_void;

        extern "system" {
            fn CreateMutexW(
                lpMutexAttributes: *mut std::ffi::c_void,
                bInitialOwner: i32,
                lpName: *const u16,
            ) -> HANDLE;
            fn GetLastError() -> u32;
            fn MessageBoxW(
                hWnd: *mut std::ffi::c_void,
                lpText: *const u16,
                lpCaption: *const u16,
                uType: u32,
            ) -> i32;
        }

        const ERROR_ALREADY_EXISTS: u32 = 183;

        let mutex_name: Vec<u16> = "Global\\QuantHUD_SingleInstance\0"
            .encode_utf16()
            .collect();

        unsafe {
            let handle = CreateMutexW(std::ptr::null_mut(), 0, mutex_name.as_ptr());
            if handle.is_null() || GetLastError() == ERROR_ALREADY_EXISTS {
                let text: Vec<u16> = "QuantHUD is already running!\0"
                    .encode_utf16()
                    .collect();
                let caption: Vec<u16> = "QuantHUD\0".encode_utf16().collect();
                MessageBoxW(
                    std::ptr::null_mut(),
                    text.as_ptr(),
                    caption.as_ptr(),
                    0x00000040, // MB_ICONINFORMATION
                );
                return;
            }
            // Handle stays open for the process lifetime (no Drop on raw pointers).
            // Windows releases it automatically when the process exits.
        }
    }

    quanthud_lib::run()
}

