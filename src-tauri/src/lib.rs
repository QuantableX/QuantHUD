mod capture;
mod config;

use serde::{Deserialize, Serialize};
use tauri::{Manager, PhysicalPosition, WebviewWindow};

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
async fn tuck_window(window: WebviewWindow, position: String) -> Result<(), String> {
    let monitor = window.current_monitor()
        .map_err(|e| e.to_string())?
        .ok_or("No monitor found")?;
    let screen_width = monitor.size().width as i32;
    let current_size = window.outer_size().map_err(|e| e.to_string())?;

    // Set new width
    window.set_size(tauri::PhysicalSize::new(TRIGGER_WIDTH as u32, current_size.height))
        .map_err(|e| e.to_string())?;

    // Reposition to correct edge
    let x = if position == "right" {
        screen_width - TRIGGER_WIDTH
    } else {
        0
    };
    window.set_position(PhysicalPosition::new(x, 0))
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// Show window - expand to full width
#[tauri::command]
async fn show_window(window: WebviewWindow, position: String) -> Result<(), String> {
    let monitor = window.current_monitor()
        .map_err(|e| e.to_string())?
        .ok_or("No monitor found")?;
    let screen_width = monitor.size().width as i32;
    let current_size = window.outer_size().map_err(|e| e.to_string())?;

    // Set new width
    window.set_size(tauri::PhysicalSize::new(TOTAL_WIDTH as u32, current_size.height))
        .map_err(|e| e.to_string())?;

    // Reposition to correct edge
    let x = if position == "right" {
        screen_width - TOTAL_WIDTH
    } else {
        0
    };
    window.set_position(PhysicalPosition::new(x, 0))
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// Check if window is tucked
#[tauri::command]
async fn is_window_tucked(window: WebviewWindow) -> Result<bool, String> {
    let size = window.outer_size().map_err(|e| e.to_string())?;
    Ok(size.width <= TRIGGER_WIDTH as u32)
}

/// Setup window for full height (call on startup)
#[tauri::command]
async fn setup_window_size(window: WebviewWindow) -> Result<(), String> {
    use tauri::PhysicalSize;

    // Get monitor work area
    let monitor = window.current_monitor()
        .map_err(|e| e.to_string())?
        .ok_or("No monitor found")?;

    let screen_height = monitor.size().height;
    let taskbar_height = 40; // Windows 11 taskbar

    let window_height = screen_height - taskbar_height;
    let window_width = TOTAL_WIDTH as u32;

    window.set_size(PhysicalSize::new(window_width, window_height))
        .map_err(|e| e.to_string())?;

    // Position window at left edge by default
    window.set_position(PhysicalPosition::new(0, 0))
        .map_err(|e| e.to_string())?;

    window.show().map_err(|e| e.to_string())?;

    Ok(())
}

/// Set window position to left or right edge
#[tauri::command]
async fn set_window_position(window: WebviewWindow, position: String) -> Result<(), String> {
    let monitor = window.current_monitor()
        .map_err(|e| e.to_string())?
        .ok_or("No monitor found")?;

    let screen_width = monitor.size().width as i32;

    let x = if position == "right" {
        screen_width - TOTAL_WIDTH
    } else {
        0
    };

    window.set_position(PhysicalPosition::new(x, 0))
        .map_err(|e| e.to_string())?;

    Ok(())
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

/// Open fullscreen transparent region selector window
#[tauri::command]
async fn open_region_selector(app: tauri::AppHandle) -> Result<(), String> {
    use tauri::{WebviewUrl, WebviewWindowBuilder};

    // Get primary monitor size
    let monitors = app.available_monitors().map_err(|e| e.to_string())?;
    let monitor = monitors.into_iter().next().ok_or("No monitor found")?;
    let size = monitor.size();

    let _window = WebviewWindowBuilder::new(&app, "region-selector", WebviewUrl::App("/region-selector".into()))
        .title("Select Region")
        .inner_size(size.width as f64, size.height as f64)
        .position(0.0, 0.0)
        .decorations(false)
        .always_on_top(true)
        .skip_taskbar(true)
        .focused(true)
        .build()
        .map_err(|e| format!("Failed to build window: {}", e))?;

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .manage(RegionState(Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![
            capture_screen,
            get_cursor_position,
            load_config,
            save_config,
            tuck_window,
            show_window,
            is_window_tucked,
            setup_window_size,
            set_window_position,
            open_region_selector,
            set_selected_region,
            get_selected_region
        ])

        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

