export interface ClipboardEntry {
  id: string;
  text: string;
  timestamp: number;
}

const CLIPBOARD_KEY = "quanthud_clipboard";
const MAX_ENTRIES = 50;

function generateId(): string {
  return Date.now().toString(36) + Math.random().toString(36).slice(2, 8);
}

// ── Shared singleton state so polling survives component unmount ──
const _entries = ref<ClipboardEntry[]>([]);
let _pollInterval: ReturnType<typeof setInterval> | null = null;
let _lastClipText = "";
let _initialized = false;

async function _load() {
  try {
    if (typeof window !== "undefined" && (window as any).__TAURI__) {
      const { invoke } = await import("@tauri-apps/api/core");
      const saved = await invoke<string>("load_config");
      if (saved) {
        const config = JSON.parse(saved);
        if (config._clipboardHistory) {
          _entries.value = config._clipboardHistory;
          return;
        }
      }
    }
    const saved = localStorage.getItem(CLIPBOARD_KEY);
    if (saved) _entries.value = JSON.parse(saved);
  } catch (e) {
    console.warn("Failed to load clipboard history:", e);
  }
}

async function _save() {
  try {
    if (typeof window !== "undefined" && (window as any).__TAURI__) {
      const { invoke } = await import("@tauri-apps/api/core");
      const raw = await invoke<string>("load_config");
      const config = raw ? JSON.parse(raw) : {};
      config._clipboardHistory = _entries.value;
      await invoke("save_config", { config: JSON.stringify(config) });
      emitSync("clipboard");
    } else {
      localStorage.setItem(CLIPBOARD_KEY, JSON.stringify(_entries.value));
    }
  } catch (e) {
    console.warn("Failed to save clipboard history:", e);
  }
}

async function _readClipboard(): Promise<string> {
  try {
    if (typeof window !== "undefined" && (window as any).__TAURI__) {
      const { readText } = await import("@tauri-apps/plugin-clipboard-manager");
      return (await readText()) || "";
    }
    return await navigator.clipboard.readText();
  } catch {
    return "";
  }
}

async function _pollClipboard() {
  const text = await _readClipboard();
  if (text && text !== _lastClipText) {
    _lastClipText = text;
    if (_entries.value.length === 0 || _entries.value[0].text !== text) {
      _entries.value.unshift({ id: generateId(), text, timestamp: Date.now() });
      if (_entries.value.length > MAX_ENTRIES) _entries.value.pop();
      _save();
    }
  }
}

let _syncCleanup: (() => void) | null = null;

/** Call once at app startup (e.g. in index.vue onMounted) */
export async function initClipboardPolling() {
  if (_initialized) return;
  _initialized = true;
  await _load();
  _lastClipText = await _readClipboard();
  if (!_pollInterval) {
    _pollInterval = setInterval(_pollClipboard, 1500);
  }
  _syncCleanup = await onSyncEvent("clipboard", _load);
}

/** Call on app teardown if needed */
export function disposeClipboardPolling() {
  if (_pollInterval) {
    clearInterval(_pollInterval);
    _pollInterval = null;
  }
  _syncCleanup?.();
  _syncCleanup = null;
  _initialized = false;
}

export function useClipboardHistory() {
  async function writeClipboard(text: string) {
    try {
      // Set _lastClipText BEFORE writing so the poll won't re-detect it as new
      _lastClipText = text;
      if (typeof window !== "undefined" && (window as any).__TAURI__) {
        const { writeText } =
          await import("@tauri-apps/plugin-clipboard-manager");
        await writeText(text);
      } else {
        await navigator.clipboard.writeText(text);
      }
    } catch (e) {
      console.warn("Failed to write clipboard:", e);
    }
  }

  function removeEntry(id: string) {
    _entries.value = _entries.value.filter((e) => e.id !== id);
    _save();
  }

  function clearAll() {
    _entries.value = [];
    _save();
  }

  return {
    entries: _entries,
    writeClipboard,
    removeEntry,
    clearAll,
  };
}
