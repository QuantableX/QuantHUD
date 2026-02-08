<template>
  <div class="sidebar-wrapper">
    <!-- Burger Menu Button -->
    <button
      class="burger-btn btn btn-icon"
      :class="{ active: isOpen }"
      @click="toggleSidebar"
      title="Toggle menu"
    >
      <svg
        width="20"
        height="20"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2.5"
        stroke-linecap="round"
        stroke-linejoin="round"
      >
        <line x1="3" y1="6" x2="21" y2="6" />
        <line x1="3" y1="12" x2="21" y2="12" />
        <line x1="3" y1="18" x2="21" y2="18" />
      </svg>
    </button>

    <!-- Sidebar Panel -->
    <div class="sidebar-panel" :class="{ open: isOpen, right: isRight }">
      <div class="sidebar-header">
        <h3>Modules</h3>
        <button class="close-btn" @click="toggleSidebar">
          <svg
            width="18"
            height="18"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          >
            <line x1="18" y1="6" x2="6" y2="18" />
            <line x1="6" y1="6" x2="18" y2="18" />
          </svg>
        </button>
      </div>

      <div class="sidebar-tabs">
        <!-- Basic mode: only general modules -->
        <template v-if="displayMode === 'basic'">
          <button
            v-for="module in generalModules"
            :key="module.id"
            class="tab-btn"
            :class="{ active: activeModule === module.id }"
            @click="selectModule(module.id)"
          >
            <span class="tab-icon" v-html="module.icon"></span>
            <span class="tab-label">{{ module.label }}</span>
          </button>
        </template>

        <!-- Pro mode: sections with collapsible dividers -->
        <template v-else>
          <div
            class="section-divider"
            @click="generalCollapsed = !generalCollapsed"
          >
            <span class="divider-line"></span>
            <span class="divider-label"
              >{{ generalCollapsed ? "▶" : "▼" }} General</span
            >
            <span class="divider-line"></span>
          </div>
          <template v-if="!generalCollapsed">
            <button
              v-for="module in generalModules"
              :key="module.id"
              class="tab-btn"
              :class="{ active: activeModule === module.id }"
              @click="selectModule(module.id)"
            >
              <span class="tab-icon" v-html="module.icon"></span>
              <span class="tab-label">{{ module.label }}</span>
            </button>
          </template>

          <div
            class="section-divider"
            @click="advancedCollapsed = !advancedCollapsed"
          >
            <span class="divider-line"></span>
            <span class="divider-label"
              >{{ advancedCollapsed ? "▶" : "▼" }} Advanced</span
            >
            <span class="divider-line"></span>
          </div>
          <template v-if="!advancedCollapsed">
            <button
              v-for="module in advancedModules"
              :key="module.id"
              class="tab-btn"
              :class="{ active: activeModule === module.id }"
              @click="selectModule(module.id)"
            >
              <span class="tab-icon" v-html="module.icon"></span>
              <span class="tab-label">{{ module.label }}</span>
            </button>
          </template>
        </template>
      </div>

      <div class="sidebar-footer">
        <button
          class="tab-btn settings-tab"
          :class="{ active: activeModule === 'settings' }"
          @click="selectModule('settings')"
        >
          <span class="tab-icon">
            <svg
              width="18"
              height="18"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
            >
              <path
                d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"
              />
              <circle cx="12" cy="12" r="3" />
            </svg>
          </span>
          <span class="tab-label">Settings</span>
        </button>
      </div>
    </div>

    <!-- Overlay -->
    <div v-if="isOpen" class="sidebar-overlay" @click="toggleSidebar"></div>
  </div>
</template>

<script setup lang="ts">
import type { DisplayMode } from "~/composables/useConfig";

const isOpen = ref(false);
const generalCollapsed = ref(false);
const advancedCollapsed = ref(false);

const props = defineProps<{
  activeModule: string;
  windowPosition?: string;
  displayMode: DisplayMode;
}>();

const emit = defineEmits<{
  "update:activeModule": [moduleId: string];
}>();

const isRight = computed(() => props.windowPosition === "right");

const generalIds = ["home", "notes", "todos"];

const allModules = [
  {
    id: "home",
    label: "Home",
    icon: '<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/><polyline points="9 22 9 12 15 12 15 22"/></svg>',
  },
  {
    id: "notes",
    label: "Notes",
    icon: '<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="16" y1="13" x2="8" y2="13"/><line x1="16" y1="17" x2="8" y2="17"/><polyline points="10 9 9 9 8 9"/></svg>',
  },
  {
    id: "todos",
    label: "Todo List",
    icon: '<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M9 11l3 3L22 4"/><path d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11"/></svg>',
  },
  {
    id: "calculator",
    label: "Calculator",
    icon: '<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="4" y="2" width="16" height="20" rx="2"/><line x1="8" y1="6" x2="16" y2="6"/><line x1="8" y1="10" x2="16" y2="10"/><line x1="8" y1="14" x2="16" y2="14"/></svg>',
  },
  {
    id: "placeholder2",
    label: "Placeholder 2",
    icon: '<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="16"/><line x1="8" y1="12" x2="16" y2="12"/></svg>',
  },
  {
    id: "placeholder3",
    label: "Placeholder 3",
    icon: '<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polygon points="12 2 2 7 12 12 22 7 12 2"/><polyline points="2 17 12 22 22 17"/><polyline points="2 12 12 17 22 12"/></svg>',
  },
  {
    id: "placeholder4",
    label: "Placeholder 4",
    icon: '<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/></svg>',
  },
  {
    id: "placeholder5",
    label: "Placeholder 5",
    icon: '<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 2L2 7l10 5 10-5-10-5z"/><path d="M2 17l10 5 10-5M2 12l10 5 10-5"/></svg>',
  },
  {
    id: "placeholder6",
    label: "Placeholder 6",
    icon: '<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><path d="M8 14s1.5 2 4 2 4-2 4-2"/><line x1="9" y1="9" x2="9.01" y2="9"/><line x1="15" y1="9" x2="15.01" y2="9"/></svg>',
  },
  {
    id: "placeholder7",
    label: "Placeholder 7",
    icon: '<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M20.59 13.41l-7.17 7.17a2 2 0 0 1-2.83 0L2 12V2h10l8.59 8.59a2 2 0 0 1 0 2.82z"/><line x1="7" y1="7" x2="7.01" y2="7"/></svg>',
  },
];

const generalModules = computed(() =>
  allModules.filter((m) => generalIds.includes(m.id)),
);
const advancedModules = computed(() =>
  allModules.filter((m) => !generalIds.includes(m.id)),
);

function toggleSidebar() {
  isOpen.value = !isOpen.value;
}

function selectModule(moduleId: string) {
  emit("update:activeModule", moduleId);
  isOpen.value = false;
}
</script>

<style scoped>
.sidebar-wrapper {
  display: inline-block;
}

.burger-btn {
  background: var(--btn-primary);
}

.burger-btn:hover {
  background: var(--btn-primary-hover);
}

.burger-btn.active {
  background: var(--accent-blue);
}

.sidebar-panel {
  position: fixed;
  top: 0;
  left: -280px;
  width: 280px;
  height: 100vh;
  background: var(--bg-secondary);
  border-right: 1px solid var(--border-color);
  transition:
    left 0.3s ease,
    right 0.3s ease;
  z-index: 1002;
  display: flex;
  flex-direction: column;
}

.sidebar-panel.right {
  left: auto;
  right: -280px;
  border-right: none;
  border-left: 1px solid var(--border-color);
}

.sidebar-panel.open {
  left: 0;
}

.sidebar-panel.right.open {
  left: auto;
  right: 0;
}

.sidebar-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px;
  border-bottom: 1px solid var(--border-color);
}

.sidebar-header h3 {
  font-size: 18px;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0;
}

.close-btn {
  width: 32px;
  height: 32px;
  padding: 0;
  background: transparent;
  border: none;
  color: var(--text-secondary);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 6px;
  transition: all 0.15s ease;
}

.close-btn:hover {
  background: var(--bg-card);
  color: var(--text-primary);
}

.sidebar-tabs {
  flex: 1;
  padding: 12px;
  overflow-y: auto;
}

.sidebar-footer {
  padding: 12px;
  background: var(--bg-primary);
}

.tab-btn {
  width: 100%;
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  margin-bottom: 8px;
  background: var(--bg-card);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.15s ease;
  text-align: left;
}

.settings-tab {
  margin-bottom: 0;
}

.tab-btn:hover {
  background: var(--input-bg);
  border-color: var(--accent-blue);
  color: var(--text-primary);
}

.tab-btn.active {
  background: var(--accent-blue);
  border-color: var(--accent-blue);
  color: white;
}

.tab-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.tab-label {
  font-size: 14px;
  font-weight: 500;
}

.section-divider {
  display: flex;
  align-items: center;
  gap: 8px;
  margin: 12px 0 8px;
  cursor: pointer;
  user-select: none;
}

.divider-line {
  flex: 1;
  height: 1px;
  background: var(--border-color);
}

.divider-label {
  font-size: 11px;
  font-weight: 600;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  white-space: nowrap;
}

.section-divider:hover .divider-label {
  color: var(--text-primary);
}

.section-divider:hover .divider-line {
  background: var(--text-secondary);
}

.sidebar-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: rgba(0, 0, 0, 0.5);
  z-index: 1000;
}
</style>
