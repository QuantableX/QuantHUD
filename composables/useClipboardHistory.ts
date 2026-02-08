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

export function useClipboardHistory() {
  const entries = ref<ClipboardEntry[]>([]);
  let pollInterval: ReturnType<typeof setInterval> | null = null;
  let lastClipText = "";

  async function load() {
    try {
      if (typeof window !== "undefined" && (window as any).__TAURI__) {
        const { invoke } = await import("@tauri-apps/api/core");
        const saved = await invoke<string>("load_config");
        if (saved) {
          const config = JSON.parse(saved);
          if (config._clipboardHistory) {
            entries.value = config._clipboardHistory;
            return;
          }
        }
      }
      const saved = localStorage.getItem(CLIPBOARD_KEY);
      if (saved) entries.value = JSON.parse(saved);
    } catch (e) { console.warn("Failed to load clipboard history:", e); }
  }

  async function save() {
    try {
      if (typeof window !== "undefined" && (window as any).__TAURI__) {
        const { invoke } = await import("@tauri-apps/api/core");
        const raw = await invoke<string>("load_config");
        const config = raw ? JSON.parse(raw) : {};
        config._clipboardHistory = entries.value;
        await invoke("save_config", { config: JSON.stringify(config) });
      } else {
        localStorage.setItem(CLIPBOARD_KEY, JSON.stringify(entries.value));
      }
    } catch (e) { console.warn("Failed to save clipboard history:", e); }
  }

  async function readClipboard(): Promise<string> {
    try {
      if (typeof window !== "undefined" && (window as any).__TAURI__) {
        const { readText } = await import("@tauri-apps/plugin-clipboard-manager");
        return (await readText()) || "";
      }
      return await navigator.clipboard.readText();
    } catch { return ""; }
  }

  async function writeClipboard(text: string) {
    try {
      if (typeof window !== "undefined" && (window as any).__TAURI__) {
        const { writeText } = await import("@tauri-apps/plugin-clipboard-manager");
        await writeText(text);
      } else {
        await navigator.clipboard.writeText(text);
      }
      lastClipText = text;
    } catch (e) { console.warn("Failed to write clipboard:", e); }
  }

  async function pollClipboard() {
    const text = await readClipboard();
    if (text && text !== lastClipText) {
      lastClipText = text;
      // Don't add duplicates at top
      if (entries.value.length === 0 || entries.value[0].text !== text) {
        entries.value.unshift({ id: generateId(), text, timestamp: Date.now() });
        if (entries.value.length > MAX_ENTRIES) entries.value.pop();
        save();
      }
    }
  }

  function removeEntry(id: string) {
    entries.value = entries.value.filter((e) => e.id !== id);
    save();
  }

  function clearAll() {
    entries.value = [];
    save();
  }

  function startPolling() {
    if (pollInterval) return;
    pollInterval = setInterval(pollClipboard, 1500);
  }

  function stopPolling() {
    if (pollInterval) { clearInterval(pollInterval); pollInterval = null; }
  }

  onMounted(async () => {
    await load();
    // Initialize lastClipText to current clipboard so we don't immediately add it
    lastClipText = await readClipboard();
    startPolling();
  });

  onUnmounted(() => { stopPolling(); });

  return {
    entries,
    writeClipboard,
    removeEntry,
    clearAll,
  };
}

