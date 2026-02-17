<template>
  <div v-if="isOpen" class="modal-overlay" @click.self="close">
    <div class="modal">
      <header class="modal-header">
        <h2>
          <svg
            width="16"
            height="16"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            style="margin-right: 6px; vertical-align: middle"
          >
            <path
              d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"
            />
            <circle cx="12" cy="12" r="3" />
          </svg>
          Settings
        </h2>
        <button class="btn btn-icon" @click="close">
          <svg
            width="16"
            height="16"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
          >
            <path d="M18 6L6 18M6 6l12 12" />
          </svg>
        </button>
      </header>

      <div class="modal-content">
        <!-- Version + Update Check -->
        <div class="version-badge">v{{ appVersion }}</div>
        <button class="btn btn-ghost update-btn" :disabled="updateChecking" @click="checkForUpdate">
          {{ updateChecking ? 'Checking...' : 'Check for Updates' }}
        </button>
        <div v-if="updateStatus" class="update-status" :class="{ 'update-available': updateAvailable }">
          {{ updateStatus }}
        </div>

        <!-- Window Position -->
        <div class="setting-group">
          <label class="setting-label">Window Position</label>
          <select
            class="setting-select"
            :value="position"
            @change="
              setPosition(($event.target as HTMLSelectElement).value as any)
            "
          >
            <option value="left">◀ Left</option>
            <option value="right">Right ▶</option>
            <option value="dual">◀ Dual ▶</option>
          </select>
        </div>

        <!-- Color Theme -->
        <div class="setting-group">
          <label class="setting-label">Color Theme</label>
          <select
            class="setting-select"
            :value="theme"
            @change="
              setTheme(($event.target as HTMLSelectElement).value as any)
            "
          >
            <option value="light">Light</option>
            <option value="dark">Dark</option>
          </select>
        </div>

        <!-- Screenshots Folder -->
        <div class="setting-group">
          <label class="setting-label">Screenshots Folder</label>
          <div class="folder-picker">
            <div class="folder-path" :title="screenshotsFolder || defaultScreenshotsFolder || 'Pictures/Screenshots'">
              {{ screenshotsFolder || defaultScreenshotsFolder || 'Pictures/Screenshots' }}
            </div>
            <button class="btn btn-ghost folder-browse-btn" @click="browseFolder">
              Browse
            </button>
          </div>
          <button
            v-if="screenshotsFolder"
            class="btn btn-ghost btn-sm reset-btn"
            @click="setScreenshotsFolder('')"
          >
            Reset to Default
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { WindowPosition, ColorTheme } from "~/composables/useConfig";

const runtimeConfig = useRuntimeConfig();
const appVersion = runtimeConfig.public.appVersion;

const props = defineProps<{
  isOpen: boolean;
  position: WindowPosition;
  theme: ColorTheme;
  screenshotsFolder: string;
}>();

const emit = defineEmits<{
  close: [];
  "update:position": [value: WindowPosition];
  "update:theme": [value: ColorTheme];
  "update:screenshotsFolder": [value: string];
}>();

const defaultScreenshotsFolder = ref("");
const updateChecking = ref(false);
const updateStatus = ref("");
const updateAvailable = ref(false);

onMounted(async () => {
  try {
    if (typeof window !== "undefined" && (window as any).__TAURI__) {
      const { invoke } = await import("@tauri-apps/api/core");
      defaultScreenshotsFolder.value = await invoke<string>("get_default_screenshots_folder");
    }
  } catch (e) {
    console.warn("Failed to get default screenshots folder:", e);
  }
});

async function checkForUpdate() {
  updateChecking.value = true;
  updateStatus.value = "";
  updateAvailable.value = false;
  try {
    const res = await fetch(
      "https://api.github.com/repos/QuantableX/QuantHUD/releases/latest"
    );
    if (!res.ok) throw new Error(`GitHub API returned ${res.status}`);
    const data = await res.json();
    const latestVersion = (data.tag_name || "").replace(/^v/, "");
    const currentVersion = appVersion as string;
    if (latestVersion && latestVersion !== currentVersion) {
      updateAvailable.value = true;
      updateStatus.value = `Update available: v${latestVersion}`;
      // Open the release page
      if (typeof window !== "undefined" && (window as any).__TAURI__) {
        const { open } = await import("@tauri-apps/plugin-shell");
        await open(data.html_url);
      }
    } else {
      updateStatus.value = "You are on the latest version.";
    }
  } catch (e: any) {
    updateStatus.value = "Failed to check for updates.";
    console.warn("Update check failed:", e);
  } finally {
    updateChecking.value = false;
  }
}

async function browseFolder() {
  try {
    if (typeof window !== "undefined" && (window as any).__TAURI__) {
      const { invoke } = await import("@tauri-apps/api/core");
      const currentPath = props.screenshotsFolder || defaultScreenshotsFolder.value || "";
      const selected = await invoke<string | null>("pick_folder", { defaultPath: currentPath || null });
      if (selected) {
        setScreenshotsFolder(selected);
      }
    }
  } catch (e) {
    console.warn("Failed to pick folder:", e);
  }
}

function close() {
  emit("close");
}

function setPosition(pos: WindowPosition) {
  emit("update:position", pos);
}

function setTheme(t: ColorTheme) {
  emit("update:theme", t);
}

function setScreenshotsFolder(folder: string) {
  emit("update:screenshotsFolder", folder);
}
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal {
  background: var(--bg-card);
  border: 1px solid var(--border-color);
  border-radius: 12px;
  width: 280px;
  max-height: 80vh;
  overflow: hidden;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  border-bottom: 1px solid var(--border-color);
}

.modal-header h2 {
  font-size: 16px;
  font-weight: 600;
}

.modal-content {
  padding: 16px;
  overflow-y: auto;
  max-height: calc(80vh - 50px);
}

.version-badge {
  text-align: center;
  font-size: 12px;
  color: var(--text-secondary);
  margin-bottom: 4px;
  padding: 6px 12px;
  background: var(--input-bg);
  border-radius: 6px;
  display: block;
}

.update-btn {
  width: 100%;
  padding: 6px 12px;
  font-size: 11px;
  margin-bottom: 4px;
}

.update-status {
  font-size: 11px;
  color: var(--text-secondary);
  text-align: center;
  padding: 4px 0;
  margin-bottom: 12px;
}

.update-status.update-available {
  color: var(--accent-blue, #4a9eff);
  font-weight: 600;
}

.folder-picker {
  display: flex;
  gap: 6px;
  align-items: center;
}

.folder-path {
  flex: 1;
  min-width: 0;
  padding: 8px 10px;
  background: var(--input-bg);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  color: var(--text-primary);
  font-size: 11px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.folder-browse-btn {
  flex-shrink: 0;
  padding: 8px 12px;
  font-size: 11px;
}

.setting-group {
  margin-bottom: 16px;
}

.setting-label {
  display: block;
  font-size: 13px;
  color: var(--text-secondary);
  margin-bottom: 8px;
}

.setting-select {
  width: 100%;
  padding: 10px 12px;
  background: var(--input-bg);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  color: var(--text-primary);
  font-size: 13px;
  cursor: pointer;
  transition: all 0.15s ease;
  box-sizing: border-box;
}

.setting-select:hover {
  border-color: var(--accent-blue);
  background: var(--bg-secondary);
}

.setting-select:focus {
  outline: none;
  border-color: var(--accent-blue);
  background: var(--bg-secondary);
}

.setting-select option {
  background: var(--bg-secondary);
  color: var(--text-primary);
}

.reset-btn {
  margin-top: 4px;
  font-size: 11px;
}

.btn-sm {
  font-size: 12px;
  padding: 4px 10px;
}
</style>
