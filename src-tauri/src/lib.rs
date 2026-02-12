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
const HALF_CIRCLE_HEIGHT: i32 = 48; // Height of the half-circle trigger tab (CSS 44px + border padding)

// Tucking resizes window to only show trigger zone (20px)
// Showing expands window to full width (340px)
// Position parameter determines which edge the window sticks to

/// Tuck window - shrink to trigger zone width only
/// When trigger_style is "halfcircle", the window shrinks to just the tab size
/// so clicks pass through the rest of the screen edge.
#[tauri::command]
async fn tuck_window(window: WebviewWindow, position: String, monitor_index: Option<usize>, trigger_style: Option<String>) -> Result<(), String> {
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
    let work_area_height = get_work_area_height(&monitor);

    let is_halfcircle = trigger_style.as_deref() == Some("halfcircle");

    // For half-circle mode: window is only as tall as the tab
    // For column mode: window spans the full work area height
    let window_height = if is_halfcircle {
        (HALF_CIRCLE_HEIGHT as f64 * scale_factor) as i32
    } else {
        work_area_height
    };

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

    // For half-circle: center vertically in the work area
    let y = if is_halfcircle {
        (work_area_height - window_height) / 2
    } else {
        0
    };

    window.set_position(PhysicalPosition::new(monitor_x + x, monitor_y + y))
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
        .disable_drag_drop_handler()
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

/// Show a native Win32 notification popup with OK button and chime
#[tauri::command]
async fn show_notification_popup(message: String) -> Result<(), String> {
    std::thread::spawn(move || {
        #[cfg(target_os = "windows")]
        unsafe {
            type HANDLE = *mut std::ffi::c_void;
            type HWND = *mut std::ffi::c_void;
            type HDC = *mut std::ffi::c_void;
            type HBRUSH = *mut std::ffi::c_void;
            type HFONT = *mut std::ffi::c_void;
            type HGDIOBJ = *mut std::ffi::c_void;
            type HRGN = *mut std::ffi::c_void;

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

            #[repr(C)]
            struct DRAWITEMSTRUCT {
                ctl_type: u32, ctl_id: u32, item_id: u32,
                item_action: u32, item_state: u32,
                hwnd_item: HWND, hdc: HDC, rc_item: RECT, item_data: usize,
            }

            extern "system" {
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
                fn SetWindowLongPtrW(h: HWND, index: i32, new_long: isize) -> isize;
                fn GetWindowLongPtrW(h: HWND, index: i32) -> isize;
                fn SetProcessDpiAwarenessContext(value: isize) -> i32;
            }

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
            const GWLP_USERDATA: i32 = -21;
            const POPUP_W: i32 = 340;
            const POPUP_H: i32 = 100;
            const BTN_ID: usize = 1001;
            const ODS_SELECTED: u32 = 0x0001;
            const DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2: isize = -4;

            // Store the message text as a leaked &'static str so wnd_proc can read it
            let msg_text: &'static str = Box::leak(message.into_boxed_str());

            unsafe extern "system" fn wnd_proc(hwnd: HWND, msg: u32, w: usize, l: isize) -> isize {
                match msg {
                    WM_PAINT => {
                        let user_ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *const &str;
                        let mut ps = std::mem::zeroed::<PAINTSTRUCT>();
                        let hdc = BeginPaint(hwnd, &mut ps);
                        let bg = CreateSolidBrush(0x00201a1a);
                        let rc = RECT { left: 0, top: 0, right: POPUP_W, bottom: POPUP_H };
                        FillRect(hdc, &rc, bg);
                        DeleteObject(bg as HGDIOBJ);
                        SetBkMode(hdc, 1);
                        SetTextColor(hdc, 0x00FFFFFF); // white
                        let face: Vec<u16> = "Segoe UI\0".encode_utf16().collect();
                        let font = CreateFontW(
                            -15, 0, 0, 0, 600, 0, 0, 0, 0, 0, 0, 5, 0, face.as_ptr(),
                        );
                        let old = SelectObject(hdc, font as HGDIOBJ);
                        let mut trc = RECT { left: 10, top: 10, right: POPUP_W - 10, bottom: 55 };
                        if !user_ptr.is_null() {
                            let s = &*user_ptr;
                            let text_w: Vec<u16> = s.encode_utf16().chain(std::iter::once(0)).collect();
                            DrawTextW(hdc, text_w.as_ptr(), -1, &mut trc, DT_CENTER | DT_VCENTER | DT_SINGLELINE);
                        }
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

            // Use a unique class name per invocation to avoid conflicts
            static COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let cname = format!("QuantHUDNotif{}\0", n);
            let class_name: Vec<u16> = cname.encode_utf16().collect();
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
            // DPI awareness for sharp font rendering
            SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
            let sx = GetSystemMetrics(0); // SM_CXSCREEN
            let sy = GetSystemMetrics(1); // SM_CYSCREEN
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
            // Store message text pointer in USERDATA
            let msg_ref: &'static &str = Box::leak(Box::new(msg_text));
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, msg_ref as *const &str as isize);
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
            ShowWindow(hwnd, 5);
            UpdateWindow(hwnd);
            MessageBeep(0x40); // subtle chime
            let mut msg: MSG = std::mem::zeroed();
            while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    });
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
            close_dual_window,
            show_notification_popup
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

