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
        <!-- Version -->
        <div class="version-badge">v{{ appVersion }}</div>

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
}>();

const emit = defineEmits<{
  close: [];
  "update:position": [value: WindowPosition];
  "update:theme": [value: ColorTheme];
}>();

function close() {
  emit("close");
}

function setPosition(pos: WindowPosition) {
  emit("update:position", pos);
}

function setTheme(t: ColorTheme) {
  emit("update:theme", t);
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
}

.version-badge {
  text-align: center;
  font-size: 12px;
  color: var(--text-secondary);
  margin-bottom: 12px;
  padding: 4px 8px;
  background: var(--input-bg);
  border-radius: 4px;
  display: inline-block;
  width: 100%;
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
</style>
