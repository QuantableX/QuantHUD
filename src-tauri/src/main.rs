// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // Single instance check using a Windows named mutex
    #[cfg(target_os = "windows")]
    {
        type HANDLE = *mut std::ffi::c_void;
        type HWND = *mut std::ffi::c_void;
        type HDC = *mut std::ffi::c_void;
        type HBRUSH = *mut std::ffi::c_void;
        type HFONT = *mut std::ffi::c_void;
        type HGDIOBJ = *mut std::ffi::c_void;

        #[repr(C)]
        struct RECT { left: i32, top: i32, right: i32, bottom: i32 }

        #[repr(C)]
        struct PAINTSTRUCT {
            hdc: HDC,
            f_erase: i32,
            rc_paint: RECT,
            f_restore: i32,
            f_inc_update: i32,
            rgb_reserved: [u8; 32],
        }

        #[repr(C)]
        struct WNDCLASSW {
            style: u32,
            lpfn_wnd_proc: unsafe extern "system" fn(HWND, u32, usize, isize) -> isize,
            cb_cls_extra: i32,
            cb_wnd_extra: i32,
            h_instance: HANDLE,
            h_icon: HANDLE,
            h_cursor: HANDLE,
            hbr_background: HBRUSH,
            lpsz_menu_name: *const u16,
            lpsz_class_name: *const u16,
        }

        #[repr(C)]
        struct MSG {
            hwnd: HWND,
            message: u32,
            w_param: usize,
            l_param: isize,
            time: u32,
            pt_x: i32,
            pt_y: i32,
        }

        type HRGN = *mut std::ffi::c_void;

        #[repr(C)]
        struct DRAWITEMSTRUCT {
            ctl_type: u32, ctl_id: u32, item_id: u32,
            item_action: u32, item_state: u32,
            hwnd_item: HWND, hdc: HDC, rc_item: RECT, item_data: usize,
        }

        extern "system" {
            fn CreateMutexW(a: *mut std::ffi::c_void, b: i32, c: *const u16) -> HANDLE;
            fn GetLastError() -> u32;
            fn GetSystemMetrics(index: i32) -> i32;
            fn RegisterClassW(wc: *const WNDCLASSW) -> u16;
            fn CreateWindowExW(
                ex: u32, class: *const u16, title: *const u16, style: u32,
                x: i32, y: i32, w: i32, h: i32,
                parent: HWND, menu: HANDLE, inst: HANDLE, param: *mut std::ffi::c_void,
            ) -> HWND;
            fn ShowWindow(h: HWND, cmd: i32) -> i32;
            fn UpdateWindow(h: HWND) -> i32;
            fn DestroyWindow(h: HWND) -> i32;
            fn PostQuitMessage(code: i32);
            fn DefWindowProcW(h: HWND, msg: u32, w: usize, l: isize) -> isize;
            fn GetMessageW(msg: *mut MSG, h: HWND, min: u32, max: u32) -> i32;
            fn TranslateMessage(msg: *const MSG) -> i32;
            fn DispatchMessageW(msg: *const MSG) -> isize;
            fn BeginPaint(h: HWND, ps: *mut PAINTSTRUCT) -> HDC;
            fn EndPaint(h: HWND, ps: *const PAINTSTRUCT) -> i32;
            fn CreateSolidBrush(color: u32) -> HBRUSH;
            fn FillRect(hdc: HDC, rc: *const RECT, brush: HBRUSH) -> i32;
            fn DeleteObject(obj: HGDIOBJ) -> i32;
            fn SetTextColor(hdc: HDC, color: u32) -> u32;
            fn SetBkMode(hdc: HDC, mode: i32) -> i32;
            fn CreateFontW(
                h: i32, w: i32, esc: i32, ori: i32, weight: i32,
                italic: u32, underline: u32, strikeout: u32,
                charset: u32, out_prec: u32, clip_prec: u32,
                quality: u32, pitch: u32, face: *const u16,
            ) -> HFONT;
            fn SelectObject(hdc: HDC, obj: HGDIOBJ) -> HGDIOBJ;
            fn DrawTextW(hdc: HDC, text: *const u16, len: i32, rc: *mut RECT, fmt: u32) -> i32;
            fn CreateRoundRectRgn(x1: i32, y1: i32, x2: i32, y2: i32, cx: i32, cy: i32) -> HRGN;
            fn SetWindowRgn(h: HWND, hrgn: HRGN, redraw: i32) -> i32;
            fn FillRgn(hdc: HDC, hrgn: HRGN, hbr: HBRUSH) -> i32;
            fn MessageBeep(utype: u32) -> i32;
            fn SetProcessDpiAwarenessContext(value: isize) -> i32;
        }

        const ERROR_ALREADY_EXISTS: u32 = 183;
        const WM_PAINT: u32 = 0x000F;
        const WM_COMMAND: u32 = 0x0111;
        const WM_DESTROY: u32 = 0x0002;
        const WM_DRAWITEM: u32 = 0x002B;
        const WS_POPUP: u32 = 0x8000_0000;
        const WS_VISIBLE: u32 = 0x1000_0000;
        const WS_CHILD: u32 = 0x4000_0000;
        const BS_OWNERDRAW: u32 = 0x0000_000B;
        const WS_EX_TOPMOST: u32 = 0x0008;
        const WS_EX_TOOLWINDOW: u32 = 0x0000_0080;
        const DT_CENTER: u32 = 0x01;
        const DT_VCENTER: u32 = 0x04;
        const DT_SINGLELINE: u32 = 0x20;
        const SM_CXSCREEN: i32 = 0;
        const SM_CYSCREEN: i32 = 1;
        const DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2: isize = -4;
        const POPUP_W: i32 = 340;
        const POPUP_H: i32 = 100;
        const BTN_ID: usize = 1001;
        const ODS_SELECTED: u32 = 0x0001;

        unsafe extern "system" fn wnd_proc(hwnd: HWND, msg: u32, w: usize, l: isize) -> isize {
            match msg {
                WM_PAINT => {
                    let mut ps = std::mem::zeroed::<PAINTSTRUCT>();
                    let hdc = BeginPaint(hwnd, &mut ps);
                    let bg = CreateSolidBrush(0x00201a1a); // dark bg (BGR)
                    let rc = RECT { left: 0, top: 0, right: POPUP_W, bottom: POPUP_H };
                    FillRect(hdc, &rc, bg);
                    DeleteObject(bg as HGDIOBJ);
                    SetBkMode(hdc, 1); // TRANSPARENT
                    SetTextColor(hdc, 0x00FFFFFF); // white (BGR)
                    let face: Vec<u16> = "Segoe UI\0".encode_utf16().collect();
                    let font = CreateFontW(
                        -15, 0, 0, 0, 600, 0, 0, 0, 0, 0, 0, 5, 0, face.as_ptr(),
                    );
                    let old = SelectObject(hdc, font as HGDIOBJ);
                    let text: Vec<u16> = "QuantHUD is already running\0".encode_utf16().collect();
                    let mut trc = RECT { left: 10, top: 10, right: POPUP_W - 10, bottom: 55 };
                    DrawTextW(hdc, text.as_ptr(), -1, &mut trc, DT_CENTER | DT_VCENTER | DT_SINGLELINE);
                    SelectObject(hdc, old);
                    DeleteObject(font as HGDIOBJ);
                    EndPaint(hwnd, &ps);
                    0
                }
                WM_DRAWITEM => {
                    let dis = &*(l as *const DRAWITEMSTRUCT);
                    let hdc = dis.hdc;
                    let rc = &dis.rc_item;
                    // First clear entire button rect with popup bg to eliminate default border
                    let clear_br = CreateSolidBrush(0x00201a1a);
                    let clear_rc = RECT { left: rc.left, top: rc.top, right: rc.right, bottom: rc.bottom };
                    FillRect(hdc, &clear_rc, clear_br);
                    DeleteObject(clear_br as HGDIOBJ);
                    // Draw rounded button on top
                    let bg_color = if dis.item_state & ODS_SELECTED != 0 { 0x00555555 } else { 0x00444444 };
                    let bg = CreateSolidBrush(bg_color);
                    let rgn = CreateRoundRectRgn(rc.left, rc.top, rc.right, rc.bottom, 10, 10);
                    FillRgn(hdc, rgn, bg);
                    DeleteObject(rgn as HGDIOBJ);
                    DeleteObject(bg as HGDIOBJ);
                    // Button text
                    SetBkMode(hdc, 1);
                    SetTextColor(hdc, 0x00FFFFFF);
                    let face: Vec<u16> = "Segoe UI\0".encode_utf16().collect();
                    let font = CreateFontW(-13, 0, 0, 0, 600, 0, 0, 0, 0, 0, 0, 5, 0, face.as_ptr());
                    let old = SelectObject(hdc, font as HGDIOBJ);
                    let text: Vec<u16> = "OK\0".encode_utf16().collect();
                    let mut trc = RECT { left: rc.left, top: rc.top, right: rc.right, bottom: rc.bottom };
                    DrawTextW(hdc, text.as_ptr(), -1, &mut trc, DT_CENTER | DT_VCENTER | DT_SINGLELINE);
                    SelectObject(hdc, old);
                    DeleteObject(font as HGDIOBJ);
                    1
                }
                WM_COMMAND => {
                    if (w & 0xFFFF) == BTN_ID {
                        DestroyWindow(hwnd);
                    }
                    0
                }
                WM_DESTROY => {
                    PostQuitMessage(0);
                    0
                }
                _ => DefWindowProcW(hwnd, msg, w, l),
            }
        }

        let mutex_name: Vec<u16> = "Global\\QuantHUD_SingleInstance\0"
            .encode_utf16()
            .collect();

        unsafe {
            let handle = CreateMutexW(std::ptr::null_mut(), 0, mutex_name.as_ptr());
            if handle.is_null() || GetLastError() == ERROR_ALREADY_EXISTS {
                // DPI awareness for sharp font rendering
                SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
                // Show popup with OK button — user must dismiss
                let class_name: Vec<u16> = "QuantHUDPopup\0".encode_utf16().collect();
                let wc = WNDCLASSW {
                    style: 0,
                    lpfn_wnd_proc: wnd_proc,
                    cb_cls_extra: 0,
                    cb_wnd_extra: 0,
                    h_instance: std::ptr::null_mut(),
                    h_icon: std::ptr::null_mut(),
                    h_cursor: std::ptr::null_mut(),
                    hbr_background: std::ptr::null_mut(),
                    lpsz_menu_name: std::ptr::null(),
                    lpsz_class_name: class_name.as_ptr(),
                };
                RegisterClassW(&wc);
                let sx = GetSystemMetrics(SM_CXSCREEN);
                let sy = GetSystemMetrics(SM_CYSCREEN);
                let title: Vec<u16> = "\0".encode_utf16().collect();
                let hwnd = CreateWindowExW(
                    WS_EX_TOPMOST | WS_EX_TOOLWINDOW,
                    class_name.as_ptr(),
                    title.as_ptr(),
                    WS_POPUP,
                    sx - POPUP_W - 16, sy - POPUP_H - 60,
                    POPUP_W, POPUP_H,
                    std::ptr::null_mut(), std::ptr::null_mut(),
                    std::ptr::null_mut(), std::ptr::null_mut(),
                );
                // Rounded corners
                let rgn = CreateRoundRectRgn(0, 0, POPUP_W, POPUP_H, 16, 16);
                SetWindowRgn(hwnd, rgn, 1);
                // Create OK button
                let btn_class: Vec<u16> = "BUTTON\0".encode_utf16().collect();
                let btn_text: Vec<u16> = "OK\0".encode_utf16().collect();
                let btn_w = 70;
                let btn_h = 28;
                let btn_x = (POPUP_W - btn_w) / 2;
                let btn_y = POPUP_H - btn_h - 12;
                CreateWindowExW(
                    0,
                    btn_class.as_ptr(),
                    btn_text.as_ptr(),
                    WS_CHILD | WS_VISIBLE | BS_OWNERDRAW,
                    btn_x, btn_y, btn_w, btn_h,
                    hwnd, BTN_ID as *mut std::ffi::c_void,
                    std::ptr::null_mut(), std::ptr::null_mut(),
                );
                ShowWindow(hwnd, 5); // SW_SHOW
                UpdateWindow(hwnd);
                MessageBeep(0x40); // MB_ICONASTERISK — subtle chime
                let mut msg: MSG = std::mem::zeroed();
                while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
                    TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }
                return;
            }
            // Handle stays open for the process lifetime (no Drop on raw pointers).
            // Windows releases it automatically when the process exits.
        }
    }

    quanthud_lib::run()
}

