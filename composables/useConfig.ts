import type { CalculatorInputs } from "./useCalculator";

export type WindowPosition = "left" | "right";
export type ColorTheme = "default" | "monochrome";
export type TriggerStyle = "column" | "halfcircle";
export type ActivationMode = "hover" | "click";
export type DisplayMode = "basic" | "pro";

export interface AppConfig {
  scanRegion: [number, number, number, number] | null;
  calcSettings: Partial<CalculatorInputs>;
  windowPosition: WindowPosition;
  colorTheme: ColorTheme;
  triggerStyle: TriggerStyle;
  activationMode: ActivationMode;
  monitorIndex: number;
  displayMode: DisplayMode;
}

const CONFIG_KEY = "quanthub_config";

export function useConfig() {
  const config = ref<AppConfig>({
    scanRegion: null,
    calcSettings: {},
    windowPosition: "left",
    colorTheme: "default",
    triggerStyle: "halfcircle",
    activationMode: "hover",
    monitorIndex: 0,
    displayMode: "basic",
  });

  async function loadConfig() {
    try {
      // Try Tauri store first
      if (window.__TAURI__) {
        const { invoke } = await import("@tauri-apps/api/core");
        const saved = await invoke<string>("load_config");
        if (saved) {
          config.value = JSON.parse(saved);
        }
      } else {
        // Fallback to localStorage for dev
        const saved = localStorage.getItem(CONFIG_KEY);
        if (saved) {
          config.value = JSON.parse(saved);
        }
      }
    } catch (e) {
      console.warn("Failed to load config:", e);
    }
  }

  async function saveConfig() {
    try {
      const data = JSON.stringify(config.value);
      if (window.__TAURI__) {
        const { invoke } = await import("@tauri-apps/api/core");
        await invoke("save_config", { config: data });
      } else {
        localStorage.setItem(CONFIG_KEY, data);
      }
    } catch (e) {
      console.warn("Failed to save config:", e);
    }
  }

  function setScanRegion(region: [number, number, number, number] | null) {
    config.value.scanRegion = region;
    saveConfig();
  }

  function setCalcSettings(settings: Partial<CalculatorInputs>) {
    config.value.calcSettings = { ...config.value.calcSettings, ...settings };
    saveConfig();
  }

  function setWindowPosition(position: WindowPosition) {
    config.value.windowPosition = position;
    saveConfig();
  }

  function setColorTheme(theme: ColorTheme) {
    config.value.colorTheme = theme;
    saveConfig();
  }

  function setTriggerStyle(style: TriggerStyle) {
    config.value.triggerStyle = style;
    saveConfig();
  }

  function setActivationMode(mode: ActivationMode) {
    config.value.activationMode = mode;
    saveConfig();
  }

  function setMonitorIndex(index: number) {
    config.value.monitorIndex = index;
    saveConfig();
  }

  function setDisplayMode(mode: DisplayMode) {
    config.value.displayMode = mode;
    saveConfig();
  }

  // Load on init
  onMounted(() => {
    loadConfig();
  });

  return {
    config,
    loadConfig,
    saveConfig,
    setScanRegion,
    setCalcSettings,
    setWindowPosition,
    setColorTheme,
    setTriggerStyle,
    setActivationMode,
    setMonitorIndex,
    setDisplayMode,
  };
}
