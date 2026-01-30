# QuantCalc

**Fibonacci Level Extractor for Trading** - A desktop app that captures TradingView screenshots, extracts Fibonacci levels via OCR, and calculates position sizing.

Built with **Tauri 2** (Rust backend) + **Nuxt 3** (Vue frontend).

---

## Features

- 📸 **Screen Capture** - Capture TradingView with F9 hotkey
- 🔍 **OCR Extraction** - Detects Fibonacci levels (-0.2, 0, 0.25, 0.5, 0.75, 1, 1.2)
- 📈 **Long/Short Toggle** - Auto-assigns Entry, TP, SL based on direction
- 🧮 **Position Calculator** - Calculates size, profit, loss, R:R, breakeven
- 📋 **Quick Copy** - Copy any value to clipboard
- 🔲 **Region Selection** - Custom OCR scan area
- 📌 **Pin/Auto-hide** - Slides in/out from screen edge

---

## Prerequisites

### Required Software

1. **Node.js 18+** - [Download](https://nodejs.org/)
2. **Rust** - [Install via rustup](https://rustup.rs/)
3. **Tesseract OCR** - Required for text recognition:
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

### 1. Install Dependencies

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

> Note: Capture/OCR features only work in the Tauri app, not in browser.

---

## Building

### Build for Production

```bash
npm run tauri:build
```

Output files:

- **Windows**: `src-tauri/target/release/quantcalc.exe`
- **Installer**: `src-tauri/target/release/bundle/nsis/QuantCalc_1.0.0_x64-setup.exe`

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
QuantCalc/
├── assets/css/          # Global styles
├── components/          # Vue components
│   ├── LevelsCard.vue   # Entry/TP/SL inputs
│   ├── CalculatorCard.vue
│   └── ResultsCard.vue
├── composables/         # Vue composables
│   ├── useCalculator.ts # Position calculation logic
│   ├── useConfig.ts     # Settings persistence
│   ├── useFibExtractor.ts
│   └── useAutoHide.ts
├── layouts/
├── pages/
│   └── index.vue        # Main UI
├── src-tauri/           # Rust backend
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs       # Tauri commands
│   │   ├── capture.rs   # Screen capture
│   │   ├── ocr.rs       # Tesseract OCR
│   │   └── config.rs    # Config persistence
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

## Troubleshooting

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
