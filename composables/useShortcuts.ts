export interface Shortcut {
  id: string;
  label: string;
  url: string;
  type: "url" | "app";
  favicon?: string | null;
}

const SHORTCUTS_KEY = "quanthud_shortcuts";

function generateId(): string {
  return Date.now().toString(36) + Math.random().toString(36).slice(2, 8);
}

export function useShortcuts() {
  const shortcuts = ref<Shortcut[]>([]);

  // --- Persistence ---

  async function loadShortcuts() {
    try {
      if (typeof window !== "undefined" && (window as any).__TAURI__) {
        const { invoke } = await import("@tauri-apps/api/core");
        const saved = await invoke<string>("load_config");
        if (saved) {
          const config = JSON.parse(saved);
          if (config._shortcuts) {
            shortcuts.value = config._shortcuts;
            return;
          }
        }
      }
      const saved = localStorage.getItem(SHORTCUTS_KEY);
      if (saved) shortcuts.value = JSON.parse(saved);
    } catch (e) {
      console.warn("Failed to load shortcuts:", e);
    }
  }

  async function saveShortcuts() {
    try {
      const data = JSON.stringify(shortcuts.value);
      if (typeof window !== "undefined" && (window as any).__TAURI__) {
        const { invoke } = await import("@tauri-apps/api/core");
        const raw = await invoke<string>("load_config");
        const config = raw ? JSON.parse(raw) : {};
        config._shortcuts = shortcuts.value;
        await invoke("save_config", { config: JSON.stringify(config) });
        emitSync("shortcuts");
      } else {
        localStorage.setItem(SHORTCUTS_KEY, data);
      }
    } catch (e) {
      console.warn("Failed to save shortcuts:", e);
    }
  }

  // --- CRUD ---

  function addShortcut(
    label = "New Shortcut",
    url = "",
    type: "url" | "app" = "url",
  ) {
    shortcuts.value.push({ id: generateId(), label, url, type, favicon: null });
    saveShortcuts();
  }

  function updateShortcut(id: string, data: Partial<Shortcut>) {
    const s = shortcuts.value.find((s) => s.id === id);
    if (s) {
      Object.assign(s, data);
      saveShortcuts();
    }
  }

  function deleteShortcut(id: string) {
    shortcuts.value = shortcuts.value.filter((s) => s.id !== id);
    saveShortcuts();
  }

  function clearAll() {
    shortcuts.value = [];
    saveShortcuts();
  }

  // --- Open ---

  async function openShortcut(shortcut: Shortcut) {
    try {
      if (typeof window !== "undefined" && (window as any).__TAURI__) {
        if (shortcut.type === "app") {
          const { invoke } = await import("@tauri-apps/api/core");
          await invoke("launch_app", { path: shortcut.url });
        } else {
          const { open } = await import("@tauri-apps/plugin-shell");
          await open(shortcut.url);
        }
      } else {
        window.open(shortcut.url, "_blank");
      }
    } catch (e) {
      console.warn("Failed to open shortcut:", e);
    }
  }

  // --- Favicon ---

  async function fetchFavicon(url: string): Promise<string | null> {
    try {
      const domain = new URL(url).hostname;
      const faviconUrl = `https://www.google.com/s2/favicons?domain=${domain}&sz=32`;
      const response = await fetch(faviconUrl);
      const blob = await response.blob();
      return new Promise((resolve) => {
        const reader = new FileReader();
        reader.onloadend = () => resolve(reader.result as string);
        reader.onerror = () => resolve(null);
        reader.readAsDataURL(blob);
      });
    } catch {
      return null;
    }
  }

  // --- File picker ---

  async function pickFile(): Promise<string | null> {
    try {
      if (typeof window !== "undefined" && (window as any).__TAURI__) {
        const { invoke } = await import("@tauri-apps/api/core");
        return await invoke<string | null>("pick_file", {
          defaultPath: null,
        });
      }
    } catch (e) {
      console.warn("Failed to pick file:", e);
    }
    return null;
  }

  // --- Lifecycle ---

  let _syncCleanup: (() => void) | null = null;

  onMounted(async () => {
    await loadShortcuts();
    _syncCleanup = await onSyncEvent("shortcuts", loadShortcuts);
  });

  onUnmounted(() => {
    _syncCleanup?.();
  });

  return {
    shortcuts,
    addShortcut,
    updateShortcut,
    deleteShortcut,
    clearAll,
    openShortcut,
    fetchFavicon,
    pickFile,
  };
}
