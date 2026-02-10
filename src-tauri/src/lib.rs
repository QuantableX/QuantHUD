mod capture;
mod config;

use serde::{Deserialize, Serialize};
use tauri::{
    Manager, PhysicalPosition, WebviewWindow,
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
};

/// Get the actual work-area height for a monitor by querying the OS.
/// Falls back to screen_height - 48*scale on non-Windows or on failure.
fn get_work_area_height(monitor: &tauri::Monitor) -> i32 {
    let scale_factor = monitor.scale_factor();
    let screen_height = monitor.size().height as i32;

    #[cfg(target_os = "windows")]
    {
        #[repr(C)]
        struct RECT { left: i32, top: i32, right: i32, bottom: i32 }

        extern "system" {
            fn SystemParametersInfoW(uiAction: u32, uiParam: u32, pvParam: *mut std::ffi::c_void, fWinIni: u32) -> i32;
        }

        const SPI_GETWORKAREA: u32 = 0x0030;
        let mut rect = RECT { left: 0, top: 0, right: 0, bottom: 0 };
        let ok = unsafe {
            SystemParametersInfoW(
                SPI_GETWORKAREA,
                0,
                &mut rect as *mut RECT as *mut std::ffi::c_void,
                0,
            )
        };
        if ok != 0 {
            // rect is in logical pixels for the primary monitor work area.
            // Scale to physical pixels to match monitor.size() which is physical.
            let work_h = ((rect.bottom - rect.top) as f64 * scale_factor) as i32;
            // Only use it when it looks sane (positive and smaller than full screen)
            if work_h > 0 && work_h <= screen_height {
                return work_h;
            }
        }
    }

    // Fallback: assume 48 logical-px taskbar
    screen_height - (48.0 * scale_factor) as i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaptureResult {
    pub image_base64: String,
    pub width: u32,
    pub height: u32,
}

/// Capture screen and return as base64 PNG for frontend OCR processing
#[tauri::command]
async fn capture_screen(region: Option<[i32; 4]>) -> Result<CaptureResult, String> {
    let (base64_data, width, height) = capture::capture_screen_base64(region).map_err(|e| e.to_string())?;

    Ok(CaptureResult {
        image_base64: base64_data,
        width,
        height,
    })
}

const CONTENT_WIDTH: i32 = 320; // Width of main content area
const TRIGGER_WIDTH: i32 = 20;  // Width of trigger bar
const TOTAL_WIDTH: i32 = CONTENT_WIDTH + TRIGGER_WIDTH; // 340px

// Tucking resizes window to only show trigger zone (20px)
// Showing expands window to full width (340px)
// Position parameter determines which edge the window sticks to

/// Tuck window - shrink to trigger zone width only
#[tauri::command]
async fn tuck_window(window: WebviewWindow, position: String, monitor_index: Option<usize>) -> Result<(), String> {
    let app = window.app_handle();
    let monitors = app.available_monitors().map_err(|e| e.to_string())?;
    let monitor = if let Some(idx) = monitor_index {
        monitors.into_iter().nth(idx).ok_or("Monitor index out of range")?
    } else {
        window.current_monitor()
            .map_err(|e| e.to_string())?
            .ok_or("No monitor found")?
    };

    let scale_factor = monitor.scale_factor();
    let screen_width = monitor.size().width as i32;
    let window_height = get_work_area_height(&monitor);

    // Set new size with correct height (in physical pixels)
    window.set_size(tauri::PhysicalSize::new(
        (TRIGGER_WIDTH as f64 * scale_factor) as u32,
        window_height as u32
    )).map_err(|e| e.to_string())?;

    // Reposition to correct edge (in physical pixels)
    let x = if position == "right" {
        screen_width - (TRIGGER_WIDTH as f64 * scale_factor) as i32
    } else {
        0
    };

    // Get monitor position offset
    let monitor_x = monitor.position().x;
    let monitor_y = monitor.position().y;

    window.set_position(PhysicalPosition::new(monitor_x + x, monitor_y))
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// Show window - expand to full width
#[tauri::command]
async fn show_window(window: WebviewWindow, position: String, monitor_index: Option<usize>) -> Result<(), String> {
    let app = window.app_handle();
    let monitors = app.available_monitors().map_err(|e| e.to_string())?;
    let monitor = if let Some(idx) = monitor_index {
        monitors.into_iter().nth(idx).ok_or("Monitor index out of range")?
    } else {
        window.current_monitor()
            .map_err(|e| e.to_string())?
            .ok_or("No monitor found")?
    };

    let scale_factor = monitor.scale_factor();
    let screen_width = monitor.size().width as i32;
    let window_height = get_work_area_height(&monitor);

    // Reposition FIRST so the window doesn't momentarily overflow
    // onto an adjacent monitor before the resize completes
    let x = if position == "right" {
        screen_width - (TOTAL_WIDTH as f64 * scale_factor) as i32
    } else {
        0
    };

    let monitor_x = monitor.position().x;
    let monitor_y = monitor.position().y;

    window.set_position(PhysicalPosition::new(monitor_x + x, monitor_y))
        .map_err(|e| e.to_string())?;

    // Then expand to full width
    window.set_size(tauri::PhysicalSize::new(
        (TOTAL_WIDTH as f64 * scale_factor) as u32,
        window_height as u32
    )).map_err(|e| e.to_string())?;

    Ok(())
}

/// Check if window is tucked
#[tauri::command]
async fn is_window_tucked(window: WebviewWindow) -> Result<bool, String> {
    let monitor = window.current_monitor()
        .map_err(|e| e.to_string())?
        .ok_or("No monitor found")?;
    let scale_factor = monitor.scale_factor();
    let size = window.outer_size().map_err(|e| e.to_string())?;
    Ok(size.width <= (TRIGGER_WIDTH as f64 * scale_factor) as u32)
}

/// Setup window for full height (call on startup)
#[tauri::command]
async fn setup_window_size(window: WebviewWindow, monitor_index: Option<usize>) -> Result<(), String> {
    use tauri::PhysicalSize;

    // Get monitor work area
    let app = window.app_handle();
    let monitors = app.available_monitors().map_err(|e| e.to_string())?;
    let monitor = if let Some(idx) = monitor_index {
        monitors.into_iter().nth(idx).ok_or("Monitor index out of range")?
    } else {
        window.current_monitor()
            .map_err(|e| e.to_string())?
            .ok_or("No monitor found")?
    };

    let scale_factor = monitor.scale_factor();
    let window_height = get_work_area_height(&monitor);
    let window_width = (TOTAL_WIDTH as f64 * scale_factor) as u32;

    window.set_size(PhysicalSize::new(window_width, window_height as u32))
        .map_err(|e| e.to_string())?;

    // Position window at left edge by default
    let monitor_x = monitor.position().x;
    let monitor_y = monitor.position().y;

    window.set_position(PhysicalPosition::new(monitor_x, monitor_y))
        .map_err(|e| e.to_string())?;

    window.show().map_err(|e| e.to_string())?;

    Ok(())
}

/// Set window position to left or right edge
#[tauri::command]
async fn set_window_position(window: WebviewWindow, position: String, monitor_index: Option<usize>) -> Result<(), String> {
    let app = window.app_handle();
    let monitors = app.available_monitors().map_err(|e| e.to_string())?;
    let monitor = if let Some(idx) = monitor_index {
        monitors.into_iter().nth(idx).ok_or("Monitor index out of range")?
    } else {
        window.current_monitor()
            .map_err(|e| e.to_string())?
            .ok_or("No monitor found")?
    };

    let scale_factor = monitor.scale_factor();
    let screen_width = monitor.size().width as i32;

    let x = if position == "right" {
        screen_width - (TOTAL_WIDTH as f64 * scale_factor) as i32
    } else {
        0
    };

    // Get monitor position offset
    let monitor_x = monitor.position().x;
    let monitor_y = monitor.position().y;

    window.set_position(PhysicalPosition::new(monitor_x + x, monitor_y))
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[derive(Debug, Serialize)]
pub struct MonitorInfo {
    pub index: usize,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub is_primary: bool,
}

/// Get list of available monitors
#[tauri::command]
async fn get_available_monitors(app: tauri::AppHandle) -> Result<Vec<MonitorInfo>, String> {
    let monitors = app.available_monitors()
        .map_err(|e| e.to_string())?;

    let monitor_list: Vec<MonitorInfo> = monitors
        .into_iter()
        .enumerate()
        .map(|(index, monitor)| {
            let size = monitor.size();
            let _raw_name = monitor.name()
                .map(|s| s.to_string())
                .unwrap_or_else(|| format!("Display {}", index + 1));

            // Use a simple sequential "Display N" format with resolution
            let display_name = format!("Display {} ({}×{})", index + 1, size.width, size.height);

            MonitorInfo {
                index,
                name: display_name,
                width: size.width,
                height: size.height,
                is_primary: index == 0, // First monitor is typically primary
            }
        })
        .collect();

    Ok(monitor_list)
}

#[derive(Debug, Serialize)]
pub struct CursorPosition {
    pub x: i32,
    pub y: i32,
}

#[tauri::command]
async fn get_cursor_position() -> Result<CursorPosition, String> {
    // Use Windows API to get cursor position
    #[cfg(target_os = "windows")]
    {
        use std::mem::MaybeUninit;

        #[repr(C)]
        struct POINT {
            x: i32,
            y: i32,
        }

        extern "system" {
            fn GetCursorPos(lpPoint: *mut POINT) -> i32;
        }

        let mut point = MaybeUninit::<POINT>::uninit();
        let result = unsafe { GetCursorPos(point.as_mut_ptr()) };

        if result != 0 {
            let point = unsafe { point.assume_init() };
            return Ok(CursorPosition { x: point.x, y: point.y });
        }
    }

    Err("Failed to get cursor position".to_string())
}

/// Pick the screen color at the current cursor position (Windows only).
/// Hides the overlay window first so GetPixel reads the real desktop.
#[tauri::command]
async fn pick_screen_color(app: tauri::AppHandle) -> Result<String, String> {
    // Hide the overlay from Rust so timing is deterministic
    if let Some(overlay) = app.get_webview_window("color-picker-overlay") {
        let _ = overlay.hide();
    }
    // Wait for the compositor to fully remove the overlay surface
    std::thread::sleep(std::time::Duration::from_millis(150));

    #[cfg(target_os = "windows")]
    {
        use std::mem::MaybeUninit;

        #[repr(C)]
        struct POINT { x: i32, y: i32 }

        type HDC = *mut std::ffi::c_void;
        type HWND = *mut std::ffi::c_void;
        type COLORREF = u32;

        extern "system" {
            fn GetCursorPos(lpPoint: *mut POINT) -> i32;
            fn GetDC(hWnd: HWND) -> HDC;
            fn ReleaseDC(hWnd: HWND, hDC: HDC) -> i32;
            fn GetPixel(hdc: HDC, x: i32, y: i32) -> COLORREF;
        }

        let mut pt = MaybeUninit::<POINT>::uninit();
        if unsafe { GetCursorPos(pt.as_mut_ptr()) } == 0 {
            return Err("Failed to get cursor position".to_string());
        }
        let pt = unsafe { pt.assume_init() };

        let hdc = unsafe { GetDC(std::ptr::null_mut()) };
        if hdc.is_null() {
            return Err("Failed to get screen DC".to_string());
        }

        let color = unsafe { GetPixel(hdc, pt.x, pt.y) };
        unsafe { ReleaseDC(std::ptr::null_mut(), hdc) };

        if color == 0xFFFFFFFF {
            return Err(format!("GetPixel failed at ({}, {})", pt.x, pt.y));
        }

        let r = color & 0xFF;
        let g = (color >> 8) & 0xFF;
        let b = (color >> 16) & 0xFF;
        return Ok(format!("#{:02x}{:02x}{:02x}", r, g, b));
    }

    #[cfg(not(target_os = "windows"))]
    Err("pick_screen_color is only supported on Windows".to_string())
}

#[derive(Debug, Serialize)]
pub struct OsScreenshot {
    pub path: String,
    pub filename: String,
    pub modified: u64,
}

/// List screenshots from the OS screenshots folder
#[tauri::command]
async fn list_os_screenshots() -> Result<Vec<OsScreenshot>, String> {
    let pictures = dirs::picture_dir().ok_or("Cannot find Pictures directory")?;
    let ss_dir = pictures.join("Screenshots");
    if !ss_dir.exists() {
        return Ok(vec![]);
    }
    let mut results = Vec::new();
    let entries = std::fs::read_dir(&ss_dir).map_err(|e| e.to_string())?;
    for entry in entries.flatten() {
        let path = entry.path();
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            let ext_lower = ext.to_lowercase();
            if ext_lower == "png" || ext_lower == "jpg" || ext_lower == "jpeg" || ext_lower == "bmp" {
                let filename = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                let modified = entry.metadata()
                    .and_then(|m| m.modified())
                    .map(|t| t.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs())
                    .unwrap_or(0);
                results.push(OsScreenshot {
                    path: path.to_string_lossy().to_string(),
                    filename,
                    modified,
                });
            }
        }
    }
    results.sort_by(|a, b| b.modified.cmp(&a.modified));
    if results.len() > 50 { results.truncate(50); }
    Ok(results)
}

/// Read a screenshot file and return as base64
#[tauri::command]
async fn read_screenshot_file(path: String) -> Result<String, String> {
    let data = std::fs::read(&path).map_err(|e| format!("Failed to read file: {}", e))?;
    Ok(base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &data))
}

/// Read a screenshot and return a small thumbnail as base64 PNG
#[tauri::command]
async fn read_screenshot_thumbnail(path: String, max_width: u32) -> Result<String, String> {
    let img = image::open(&path).map_err(|e| format!("Failed to open image: {}", e))?;
    let thumb = img.thumbnail(max_width, max_width);
    let mut buf = std::io::Cursor::new(Vec::new());
    thumb.write_to(&mut buf, image::ImageFormat::Png)
        .map_err(|e| format!("Failed to encode thumbnail: {}", e))?;
    Ok(base64::Engine::encode(&base64::engine::general_purpose::STANDARD, buf.into_inner()))
}

/// Open the OS screenshots folder in file explorer
#[tauri::command]
async fn open_screenshots_folder() -> Result<(), String> {
    let pictures = dirs::picture_dir().ok_or("Cannot find Pictures directory")?;
    let ss_dir = pictures.join("Screenshots");
    if !ss_dir.exists() {
        std::fs::create_dir_all(&ss_dir).map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(ss_dir.to_string_lossy().to_string())
            .spawn()
            .map_err(|e| format!("Failed to open folder: {}", e))?;
    }
    Ok(())
}

/// Copy a screenshot image to the system clipboard (actual image, not text)
#[tauri::command]
#[allow(unused_variables)]
async fn copy_screenshot_to_clipboard(path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let img = image::open(&path).map_err(|e| format!("Failed to open image: {}", e))?;
        let rgba = img.to_rgba8();
        let (w, h) = (rgba.width(), rgba.height());

        // Convert RGBA to BGRA for Windows DIB
        let mut bgra_pixels: Vec<u8> = Vec::with_capacity((w * h * 4) as usize);
        for pixel in rgba.pixels() {
            bgra_pixels.push(pixel[2]); // B
            bgra_pixels.push(pixel[1]); // G
            bgra_pixels.push(pixel[0]); // R
            bgra_pixels.push(pixel[3]); // A
        }

        // Windows DIB is bottom-up, flip rows
        let row_bytes = (w * 4) as usize;
        let mut flipped = vec![0u8; bgra_pixels.len()];
        for y in 0..h as usize {
            let src_start = y * row_bytes;
            let dst_start = ((h as usize) - 1 - y) * row_bytes;
            flipped[dst_start..dst_start + row_bytes]
                .copy_from_slice(&bgra_pixels[src_start..src_start + row_bytes]);
        }

        type HWND = *mut std::ffi::c_void;
        type HANDLE = *mut std::ffi::c_void;
        type UINT = u32;
        type BOOL = i32;

        #[repr(C)]
        #[allow(non_snake_case)]
        struct BITMAPINFOHEADER {
            biSize: u32,
            biWidth: i32,
            biHeight: i32,
            biPlanes: u16,
            biBitCount: u16,
            biCompression: u32,
            biSizeImage: u32,
            biXPelsPerMeter: i32,
            biYPelsPerMeter: i32,
            biClrUsed: u32,
            biClrImportant: u32,
        }

        const CF_DIB: UINT = 8;
        const GMEM_MOVEABLE: UINT = 0x0002;

        extern "system" {
            fn OpenClipboard(hWnd: HWND) -> BOOL;
            fn CloseClipboard() -> BOOL;
            fn EmptyClipboard() -> BOOL;
            fn SetClipboardData(uFormat: UINT, hMem: HANDLE) -> HANDLE;
            fn GlobalAlloc(uFlags: UINT, dwBytes: usize) -> HANDLE;
            fn GlobalLock(hMem: HANDLE) -> *mut u8;
            fn GlobalUnlock(hMem: HANDLE) -> BOOL;
        }

        let header_size = std::mem::size_of::<BITMAPINFOHEADER>();
        let data_size = flipped.len();
        let total_size = header_size + data_size;

        unsafe {
            if OpenClipboard(std::ptr::null_mut()) == 0 {
                return Err("Failed to open clipboard".into());
            }
            EmptyClipboard();

            let hmem = GlobalAlloc(GMEM_MOVEABLE, total_size);
            if hmem.is_null() {
                CloseClipboard();
                return Err("Failed to allocate global memory".into());
            }

            let ptr = GlobalLock(hmem);
            if ptr.is_null() {
                CloseClipboard();
                return Err("Failed to lock global memory".into());
            }

            let header = BITMAPINFOHEADER {
                biSize: header_size as u32,
                biWidth: w as i32,
                biHeight: h as i32, // positive = bottom-up
                biPlanes: 1,
                biBitCount: 32,
                biCompression: 0, // BI_RGB
                biSizeImage: data_size as u32,
                biXPelsPerMeter: 0,
                biYPelsPerMeter: 0,
                biClrUsed: 0,
                biClrImportant: 0,
            };

            std::ptr::copy_nonoverlapping(
                &header as *const BITMAPINFOHEADER as *const u8,
                ptr,
                header_size,
            );
            std::ptr::copy_nonoverlapping(
                flipped.as_ptr(),
                ptr.add(header_size),
                data_size,
            );

            GlobalUnlock(hmem);

            if SetClipboardData(CF_DIB, hmem).is_null() {
                CloseClipboard();
                return Err("Failed to set clipboard data".into());
            }

            CloseClipboard();
        }

        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    Err("copy_screenshot_to_clipboard is only supported on Windows".to_string())
}

#[tauri::command]
async fn load_config() -> Result<String, String> {
    config::load_config().map_err(|e| e.to_string())
}

#[tauri::command]
async fn save_config(config: String) -> Result<(), String> {
    config::save_config(&config).map_err(|e| e.to_string())
}

use std::sync::Mutex;
use tauri::State;

// Global state for selected region
struct RegionState(Mutex<Option<[i32; 4]>>);

// Global state for picked color
struct PickedColorState(Mutex<Option<Option<String>>>);

// Global state for screenshot preview path
struct ScreenshotPreviewState(Mutex<Option<String>>);

/// Open fullscreen transparent region selector window
#[tauri::command]
async fn open_region_selector(app: tauri::AppHandle) -> Result<(), String> {
    use tauri::{WebviewUrl, WebviewWindowBuilder};

    let window = WebviewWindowBuilder::new(&app, "region-selector", WebviewUrl::App("/region-selector".into()))
        .title("Select Region")
        .fullscreen(true)
        .decorations(false)
        .always_on_top(true)
        .skip_taskbar(true)
        .resizable(false)
        .transparent(true)
        .visible(false)
        .build()
        .map_err(|e| format!("Failed to build window: {}", e))?;

    // Show window after a tiny delay to ensure content is rendered
    let window_clone = window.clone();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(50));
        let _ = window_clone.show();
        let _ = window_clone.set_focus();
    });

    Ok(())
}

/// Save selected region and close selector
#[tauri::command]
async fn set_selected_region(
    app: tauri::AppHandle,
    state: State<'_, RegionState>,
    region: Option<[i32; 4]>
) -> Result<(), String> {
    *state.0.lock().unwrap() = region;

    if let Some(window) = app.get_webview_window("region-selector") {
        window.close().map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Get selected region (called by main window)
#[tauri::command]
async fn get_selected_region(state: State<'_, RegionState>) -> Result<Option<[i32; 4]>, String> {
    Ok(state.0.lock().unwrap().take())
}

/// Open transparent color picker overlay window spanning ALL monitors
#[tauri::command]
async fn open_color_picker_overlay(app: tauri::AppHandle, state: State<'_, PickedColorState>) -> Result<(), String> {
    use tauri::{WebviewUrl, WebviewWindowBuilder};

    // Clear any stale result from a previous pick
    *state.0.lock().unwrap() = None;

    // Compute bounding box of the entire virtual desktop (all monitors)
    let monitors = app.available_monitors().map_err(|e| e.to_string())?;
    let (mut min_x, mut min_y, mut max_x, mut max_y) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);
    // Track primary monitor position relative to virtual desktop origin
    let mut primary_left: i32 = 0;
    let mut primary_w: u32 = 0;
    if let Some(pm) = app.primary_monitor().map_err(|e| e.to_string())? {
        primary_left = pm.position().x;
        primary_w = pm.size().width;
    }
    for m in &monitors {
        let pos = m.position();
        let size = m.size();
        min_x = min_x.min(pos.x);
        min_y = min_y.min(pos.y);
        max_x = max_x.max(pos.x + size.width as i32);
        max_y = max_y.max(pos.y + size.height as i32);
    }
    let virt_w = (max_x - min_x) as u32;
    let virt_h = (max_y - min_y) as u32;

    // Pass primary monitor offset (relative to virtual desktop origin) as query
    // params so the overlay can center the instructions on the primary screen.
    let primary_offset_x = primary_left - min_x; // px from left edge of overlay
    let url = format!(
        "/color-picker-overlay?pmx={}&pmw={}",
        primary_offset_x, primary_w
    );

    let window = WebviewWindowBuilder::new(&app, "color-picker-overlay", WebviewUrl::App(url.into()))
        .title("Pick Color")
        .decorations(false)
        .always_on_top(true)
        .skip_taskbar(true)
        .resizable(false)
        .transparent(true)
        .visible(false)
        .build()
        .map_err(|e| format!("Failed to build window: {}", e))?;

    // Strip WS_THICKFRAME and WS_CAPTION so the DWM invisible border is
    // removed, then use SetWindowPos to place the window at exact physical
    // pixel coordinates with zero deadspace.
    #[cfg(target_os = "windows")]
    {
        use std::ffi::c_void;
        type HWND = *mut c_void;

        extern "system" {
            fn GetWindowLongW(hWnd: HWND, nIndex: i32) -> i32;
            fn SetWindowLongW(hWnd: HWND, nIndex: i32, dwNewLong: i32) -> i32;
            fn SetWindowPos(
                hWnd: HWND,
                hWndInsertAfter: HWND,
                x: i32, y: i32, cx: i32, cy: i32,
                uFlags: u32,
            ) -> i32;
        }

        const GWL_STYLE: i32 = -16;
        const WS_THICKFRAME: i32 = 0x00040000;
        const WS_CAPTION: i32 = 0x00C00000;
        const HWND_TOPMOST: isize = -1;
        const SWP_FRAMECHANGED: u32 = 0x0020;
        const SWP_NOACTIVATE: u32 = 0x0010;

        if let Ok(raw) = window.hwnd() {
            let hwnd = raw.0 as HWND;
            unsafe {
                // Remove the styles that cause the invisible DWM border
                let style = GetWindowLongW(hwnd, GWL_STYLE);
                SetWindowLongW(hwnd, GWL_STYLE, style & !WS_THICKFRAME & !WS_CAPTION);

                // Now position — SWP_FRAMECHANGED forces Windows to re-apply
                // the new style so the border is truly gone.
                SetWindowPos(
                    hwnd,
                    HWND_TOPMOST as HWND,
                    min_x, min_y,
                    virt_w as i32, virt_h as i32,
                    SWP_FRAMECHANGED | SWP_NOACTIVATE,
                );
            }
        }
    }

    // Fallback for non-Windows
    #[cfg(not(target_os = "windows"))]
    {
        window.set_size(tauri::PhysicalSize::new(virt_w, virt_h)).map_err(|e| e.to_string())?;
        window.set_position(PhysicalPosition::new(min_x, min_y)).map_err(|e| e.to_string())?;
    }

    let window_clone = window.clone();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(50));
        let _ = window_clone.show();
        let _ = window_clone.set_focus();
    });

    Ok(())
}

/// Save picked color and close overlay
#[tauri::command]
async fn set_picked_color(
    app: tauri::AppHandle,
    state: State<'_, PickedColorState>,
    color: Option<String>,
) -> Result<(), String> {
    *state.0.lock().unwrap() = Some(color);

    if let Some(window) = app.get_webview_window("color-picker-overlay") {
        window.close().map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Get picked color (called by main window)
#[tauri::command]
async fn get_picked_color(state: State<'_, PickedColorState>) -> Result<Option<String>, String> {
    let val = state.0.lock().unwrap().take();
    match val {
        Some(color) => Ok(color),
        None => Ok(None),
    }
}

/// Open fullscreen screenshot preview window
#[tauri::command]
async fn open_screenshot_preview(app: tauri::AppHandle, state: State<'_, ScreenshotPreviewState>, path: String) -> Result<(), String> {
    use tauri::{WebviewUrl, WebviewWindowBuilder};

    // Store the path so the preview window can read it
    *state.0.lock().unwrap() = Some(path);

    // Close existing preview window if any
    if let Some(existing) = app.get_webview_window("screenshot-preview") {
        let _ = existing.close();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    let window = WebviewWindowBuilder::new(&app, "screenshot-preview", WebviewUrl::App("/screenshot-preview".into()))
        .title("Screenshot Preview")
        .fullscreen(true)
        .decorations(false)
        .always_on_top(true)
        .skip_taskbar(true)
        .resizable(false)
        .transparent(true)
        .visible(false)
        .build()
        .map_err(|e| format!("Failed to build window: {}", e))?;

    let window_clone = window.clone();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(50));
        let _ = window_clone.show();
        let _ = window_clone.set_focus();
    });

    Ok(())
}

/// Get the screenshot preview path (called by preview window)
#[tauri::command]
async fn get_screenshot_preview_path(state: State<'_, ScreenshotPreviewState>) -> Result<Option<String>, String> {
    Ok(state.0.lock().unwrap().clone())
}

/// Close the screenshot preview window
#[tauri::command]
async fn close_screenshot_preview(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("screenshot-preview") {
        let _ = window.close();
    }
    Ok(())
}

/// Create a second window on the right edge for dual mode
#[tauri::command]
async fn create_dual_window(app: tauri::AppHandle, monitor_index: Option<usize>) -> Result<(), String> {
    use tauri::{WebviewUrl, WebviewWindowBuilder, PhysicalSize};

    // Close existing dual window if any
    if let Some(existing) = app.get_webview_window("dual-right") {
        let _ = existing.close();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    let monitors = app.available_monitors().map_err(|e| e.to_string())?;
    let monitor = if let Some(idx) = monitor_index {
        monitors.into_iter().nth(idx).ok_or("Monitor index out of range")?
    } else {
        app.primary_monitor()
            .map_err(|e| e.to_string())?
            .ok_or("No primary monitor found")?
    };

    let scale_factor = monitor.scale_factor();
    let screen_width = monitor.size().width as i32;
    let window_height = get_work_area_height(&monitor);
    let window_width = (TOTAL_WIDTH as f64 * scale_factor) as u32;

    let window = WebviewWindowBuilder::new(&app, "dual-right", WebviewUrl::App("/".into()))
        .title("QuantHUD Right")
        .inner_size(TOTAL_WIDTH as f64, 900.0)
        .resizable(false)
        .decorations(false)
        .always_on_top(true)
        .skip_taskbar(true)
        .visible(false)
        .transparent(true)
        .shadow(false)
        .build()
        .map_err(|e| format!("Failed to create dual window: {}", e))?;

    // Size to full height
    window.set_size(PhysicalSize::new(window_width, window_height as u32))
        .map_err(|e| e.to_string())?;

    // Position at right edge
    let monitor_x = monitor.position().x;
    let monitor_y = monitor.position().y;
    let x = screen_width - (TRIGGER_WIDTH as f64 * scale_factor) as i32;
    window.set_position(PhysicalPosition::new(monitor_x + x, monitor_y))
        .map_err(|e| e.to_string())?;

    // Show after positioning
    let window_clone = window.clone();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(50));
        let _ = window_clone.show();
    });

    Ok(())
}

/// Close the dual-right window if it exists
#[tauri::command]
async fn close_dual_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("dual-right") {
        window.close().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .manage(RegionState(Mutex::new(None)))
        .manage(PickedColorState(Mutex::new(None)))
        .manage(ScreenshotPreviewState(Mutex::new(None)))
        .setup(|app| {
            let show_item = MenuItem::with_id(app, "show", "Show / Hide", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_item, &quit_item])?;

            TrayIconBuilder::new()
                .icon(app.default_window_icon().cloned().unwrap())
                .menu(&menu)
                .tooltip("QuantHUD")
                .on_menu_event(|app, event| {
                    match event.id.as_ref() {
                        "show" => {
                            if let Some(window) = app.get_webview_window("main") {
                                if window.is_visible().unwrap_or(false) {
                                    let _ = window.hide();
                                } else {
                                    let _ = window.show();
                                    let _ = window.set_focus();
                                }
                            }
                        }
                        "quit" => {
                            app.exit(0);
                        }
                        _ => {}
                    }
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            capture_screen,
            get_cursor_position,
            get_available_monitors,
            load_config,
            save_config,
            tuck_window,
            show_window,
            is_window_tucked,
            setup_window_size,
            set_window_position,
            open_region_selector,
            set_selected_region,
            get_selected_region,
            pick_screen_color,
            list_os_screenshots,
            read_screenshot_file,
            read_screenshot_thumbnail,
            open_screenshots_folder,
            copy_screenshot_to_clipboard,
            open_color_picker_overlay,
            set_picked_color,
            get_picked_color,
            open_screenshot_preview,
            get_screenshot_preview_path,
            close_screenshot_preview,
            create_dual_window,
            close_dual_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

