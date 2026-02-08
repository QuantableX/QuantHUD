# QuantHUD

**A modular desktop HUD for traders and power users** — always-on-top sidebar that slides in from the screen edge, packed with productivity tools and trading utilities.

Built with **Tauri 2** (Rust backend) + **Nuxt 3** (Vue frontend).

---

## Features

### General Modules

- 📝 **Notes** - Quick scratchpad for jotting things down
- ✅ **Todo List** - Task management with checkable items
- ⏰ **World Clock** - Track multiple time zones at a glance
- 📅 **Calendar** - Date reference and planning
- 🧮 **Calculator** - General-purpose calculator
- 🎨 **Color Picker** - Pick and copy colors from screen
- 📋 **Clipboard History** - Browse and reuse recent clipboard entries
- 📷 **Screenshots** - Capture and browse screenshot history

### Advanced Modules

- 📈 **Position Sizer** - Fibonacci-based position calculator with OCR screen capture, long/short toggle, and R:R analysis

> More advanced modules will follow.

### App Behavior

- 📌 **Pin / Auto-hide** - Slides in and out from the screen edge
- ⚙️ **Settings** - Window position (left/right), monitor selection, color themes, trigger style, activation mode, and Basic/Pro display modes

---

## Prerequisites

### Required Software

1. **Node.js 18+** - [Download](https://nodejs.org/)
2. **Rust** - [Install via rustup](https://rustup.rs/)
3. **Tesseract OCR** - Required for Position Sizer OCR:
   - **Windows**: Download installer from [UB-Mannheim](https://github.com/UB-Mannheim/tesseract/wiki)
   - Set `TESSDATA_PREFIX` env variable to Tesseract's tessdata folder
4. **Visual Studio Build Tools** (Windows) - [Download](https://visualstudio.microsoft.com/visual-cpp-build-tools/)

### Verify Installation

```bash
node --version    # Should be 18+
npm --version     # Should be 9+
rustc --version   # Should be 1.70+
cargo --version
```

---

## Setup

```bash
# Install Node dependencies
npm install

# Rust dependencies are installed automatically on first build
```

## Development

### Run in Dev Mode

```bash
npm run tauri:dev
```

This starts:

- Nuxt dev server on `http://localhost:3000`
- Tauri window pointing to the dev server

### Frontend Only (Browser)

```bash
npm run dev
```

> Note: Capture, OCR, and clipboard features only work in the Tauri app, not in browser.

---

## Building

### Build for Production

```bash
npm run tauri:build
```

Output files:

- **Windows**: `src-tauri/target/release/quanthud.exe`
- **Installer**: `src-tauri/target/release/bundle/nsis/QuantHUD_1.0.0_x64-setup.exe`

### Build Options

```bash
# Debug build (faster, larger)
npm run tauri:build -- --debug

# Specific target
npm run tauri:build -- --target x86_64-pc-windows-msvc
```

---

## Project Structure

```
QuantHUD/
├── assets/css/              # Global styles
├── components/              # Vue components
│   ├── Sidebar.vue          # Module navigation sidebar
│   ├── SettingsModal.vue    # App settings
│   ├── NotesModule.vue      # Notes scratchpad
│   ├── TodoModule.vue       # Todo list
│   ├── WorldClockModule.vue # Multi-timezone clock
│   ├── CalendarModule.vue   # Calendar
│   ├── GeneralCalcModule.vue# General calculator
│   ├── ColorPickerModule.vue# Color picker
│   ├── ClipboardHistoryModule.vue
│   ├── ScreenshotHistoryModule.vue
│   ├── LevelsCard.vue       # Entry/TP/SL inputs
│   ├── CalculatorCard.vue   # Position sizing inputs
│   └── ResultsCard.vue      # Calculation results
├── composables/             # Vue composables
│   ├── useCalculator.ts     # Position calculation logic
│   ├── useConfig.ts         # Settings persistence
│   ├── useFibExtractor.ts   # OCR Fibonacci extraction
│   ├── useAutoHide.ts       # Window auto-hide behavior
│   ├── useNotes.ts
│   ├── useTodos.ts
│   ├── useWorldClock.ts
│   ├── useCalendar.ts
│   ├── useGeneralCalc.ts
│   ├── useColorPicker.ts
│   ├── useClipboardHistory.ts
│   └── useScreenshotHistory.ts
├── pages/
│   ├── index.vue            # Main UI & module router
│   ├── region-selector.vue  # OCR region selection overlay
│   ├── screenshot-preview.vue
│   └── color-picker-overlay.vue
├── src-tauri/               # Rust backend
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs           # Tauri commands
│   │   ├── capture.rs       # Screen capture
│   │   └── config.rs        # Config persistence
│   ├── Cargo.toml
│   └── tauri.conf.json
├── nuxt.config.ts
└── package.json
```

---

## Hotkeys

| Key | Action                          |
| --- | ------------------------------- |
| F9  | Capture screen & extract levels |

---

## Code Signing (Windows Smart App Control)

Windows Smart App Control / WDAC may block unsigned debug builds with error `os error 4551`. To fix this **without** disabling any Windows security settings, sign the executable after building:

```bash
npm run tauri:sign
```

This runs `scripts/sign-dev.ps1` which:

1. Creates a local self-signed `CN=QuantHUD Dev` code-signing certificate (first run only)
2. Adds it to your user Trusted Root store (first run only)
3. Signs `src-tauri/target/debug/quanthud.exe`

To sign a **release** build instead:

```bash
powershell -ExecutionPolicy Bypass -File scripts/sign-dev.ps1 release
```

> The certificate is valid for 5 years and scoped to your user account only.

---

## Troubleshooting

### Executable Blocked (os error 4551)

- Run `npm run tauri:sign` after building to sign the exe
- See [Code Signing](#code-signing-windows-smart-app-control) above

### OCR Not Working

- Ensure Tesseract is installed and in PATH
- Set `TESSDATA_PREFIX` to the tessdata folder
- Try selecting a specific region with the 🔲 button

### Build Fails

- Run `rustup update` to update Rust
- Check Visual Studio Build Tools are installed
- Ensure Node modules are installed: `npm install`

---

## License

MIT
