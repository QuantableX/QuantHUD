<template>
  <div
    class="app-wrapper"
    :class="{
      tucked: isTucked,
      'position-right': windowPosition === 'right',
      'trigger-halfcircle': triggerStyle === 'halfcircle',
      'trigger-column': triggerStyle === 'column',
    }"
    @mouseenter="onMouseEnter"
    @mouseleave="onMouseLeave"
  >
    <!-- Trigger zone (left for right-position) -->
    <div
      v-if="windowPosition === 'right'"
      class="trigger-zone trigger-left"
      @mouseenter="triggerStyle === 'column' ? onMouseEnter() : undefined"
    >
      <div class="trigger-tab" @mouseenter="onMouseEnter">
        <span class="trigger-arrow">{{ isTucked ? "◀" : "▶" }}</span>
      </div>
    </div>

    <!-- Main content area -->
    <div class="main-container">
      <!-- Scrollable Content -->
      <div class="scroll-content">
        <!-- Header with Burger and Pin -->
        <header class="header">
          <!-- Left side: Burger when left, Pin when right -->
          <Sidebar
            v-if="windowPosition === 'left'"
            :active-module="activeModule"
            :window-position="windowPosition"
            @update:active-module="activeModule = $event"
          />
          <button
            v-else
            class="btn btn-icon pin-btn"
            :class="{ active: isPinned }"
            @click="togglePin"
            title="Pin window (keeps it visible)"
          >
            <svg
              width="18"
              height="18"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2.5"
              stroke-linecap="round"
              stroke-linejoin="round"
            >
              <line x1="12" y1="17" x2="12" y2="22" />
              <path
                d="M5 17h14v-1.76a2 2 0 0 0-1.11-1.79l-1.78-.9A2 2 0 0 1 15 10.76V6h1a2 2 0 0 0 0-4H8a2 2 0 0 0 0 4h1v4.76a2 2 0 0 1-1.11 1.79l-1.78.9A2 2 0 0 0 5 15.24Z"
              />
            </svg>
          </button>

          <h1
            class="title"
            @click="activeModule = 'home'"
            style="cursor: pointer"
          >
            𝐐𝐮𝐚𝐧𝐭𝐇𝐔𝐃
          </h1>

          <!-- Right side: Pin when left, Burger when right -->
          <button
            v-if="windowPosition === 'left'"
            class="btn btn-icon pin-btn"
            :class="{ active: isPinned }"
            @click="togglePin"
            title="Pin window (keeps it visible)"
          >
            <svg
              width="18"
              height="18"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2.5"
              stroke-linecap="round"
              stroke-linejoin="round"
            >
              <line x1="12" y1="17" x2="12" y2="22" />
              <path
                d="M5 17h14v-1.76a2 2 0 0 0-1.11-1.79l-1.78-.9A2 2 0 0 1 15 10.76V6h1a2 2 0 0 0 0-4H8a2 2 0 0 0 0 4h1v4.76a2 2 0 0 1-1.11 1.79l-1.78.9A2 2 0 0 0 5 15.24Z"
              />
            </svg>
          </button>
          <Sidebar
            v-else
            :active-module="activeModule"
            :window-position="windowPosition"
            @update:active-module="activeModule = $event"
          />
        </header>

        <!-- Home Module -->
        <div v-if="activeModule === 'home'" class="module-content">
          <div class="home-hub">
            <div class="hub-grid">
              <button
                v-for="mod in homeModules"
                :key="mod.id"
                class="hub-card"
                @click="activeModule = mod.id"
              >
                <span class="hub-icon" v-html="mod.icon"></span>
                <span class="hub-label">{{ mod.label }}</span>
              </button>
            </div>
          </div>
        </div>

        <!-- Notes Module -->
        <div v-else-if="activeModule === 'notes'" class="module-content">
          <NotesModule />
        </div>

        <!-- Calculator Module -->
        <div v-else-if="activeModule === 'calculator'">
          <!-- Capture Controls -->
          <div class="capture-row">
            <button
              class="btn btn-primary capture-btn"
              @click="handleCapture"
              :disabled="isProcessing"
            >
              <svg
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                style="margin-right: 6px"
              >
                <path
                  d="M23 19a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h4l2-3h6l2 3h4a2 2 0 0 1 2 2z"
                />
                <circle cx="12" cy="13" r="4" />
              </svg>
              Capture (F9)
            </button>
            <button
              class="btn btn-icon region-btn"
              :class="{ active: scanRegion }"
              @click="toggleRegion"
              title="Select scan region"
            >
              <svg
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
              >
                <rect x="3" y="3" width="7" height="7" />
                <rect x="14" y="3" width="7" height="7" />
                <rect x="3" y="14" width="7" height="7" />
                <rect x="14" y="14" width="7" height="7" />
              </svg>
            </button>
          </div>

          <!-- Status -->
          <div class="status">{{ status }}</div>

          <!-- Long/Short Toggle -->
          <div class="direction-toggle">
            <button
              class="btn direction-btn long"
              :class="{ active: isLong }"
              @click="setDirection(true)"
            >
              ▲ LONG ▲
            </button>
            <button class="btn btn-icon btn-red" @click="handleClear">
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
            <button
              class="btn direction-btn short"
              :class="{ active: !isLong }"
              @click="setDirection(false)"
            >
              ▼ SHORT ▼
            </button>
          </div>

          <!-- Levels Card -->
          <LevelsCard
            :is-long="isLong"
            :levels="levels"
            @update:levels="updateLevels"
            @copy="copyToClipboard"
          />

          <!-- Calculator Card -->
          <CalculatorCard
            :inputs="inputs"
            @update:inputs="updateInputs"
            @calculate="calculate"
          />

          <!-- Results Card -->
          <ResultsCard
            :results="results"
            :leverage="inputs.leverage"
            :error="error"
            @copy="copyToClipboard"
          />
        </div>

        <!-- Settings Module -->
        <div v-else-if="activeModule === 'settings'" class="module-content">
          <div class="card">
            <!-- Version -->
            <div class="version-badge">v{{ appVersion }}</div>

            <!-- Monitor Selection -->
            <div class="setting-group">
              <label class="setting-label">Monitor</label>
              <select
                class="monitor-select"
                :value="config.monitorIndex"
                @change="handleMonitorChange($event)"
              >
                <option
                  v-for="monitor in availableMonitors"
                  :key="monitor.index"
                  :value="monitor.index"
                >
                  {{ monitor.name }}
                </option>
              </select>
            </div>

            <!-- Window Position -->
            <div class="setting-group">
              <label class="setting-label">Window Position</label>
              <div class="setting-options">
                <button
                  class="btn option-btn"
                  :class="{ active: config.windowPosition === 'left' }"
                  @click="handlePositionChange('left')"
                >
                  ◀ Left
                </button>
                <button
                  class="btn option-btn"
                  :class="{ active: config.windowPosition === 'right' }"
                  @click="handlePositionChange('right')"
                >
                  Right ▶
                </button>
              </div>
            </div>

            <!-- Color Theme -->
            <div class="setting-group">
              <label class="setting-label">Color Theme</label>
              <div class="setting-options">
                <button
                  class="btn option-btn"
                  :class="{ active: config.colorTheme === 'default' }"
                  @click="handleThemeChange('default')"
                >
                  <svg
                    width="14"
                    height="14"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    style="margin-right: 4px; vertical-align: middle"
                  >
                    <circle cx="12" cy="12" r="10" />
                    <path d="M12 2a10 10 0 0 0 0 20" />
                  </svg>
                  Default
                </button>
                <button
                  class="btn option-btn"
                  :class="{ active: config.colorTheme === 'monochrome' }"
                  @click="handleThemeChange('monochrome')"
                >
                  <svg
                    width="14"
                    height="14"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    style="margin-right: 4px; vertical-align: middle"
                  >
                    <rect x="3" y="3" width="18" height="18" rx="2" />
                  </svg>
                  Monochrome
                </button>
              </div>
            </div>

            <!-- Trigger Style -->
            <div class="setting-group">
              <label class="setting-label">Trigger Style</label>
              <div class="setting-options">
                <button
                  class="btn option-btn"
                  :class="{
                    active:
                      config.triggerStyle === 'halfcircle' ||
                      !config.triggerStyle,
                  }"
                  @click="handleTriggerStyleChange('halfcircle')"
                >
                  ◗ Half Circle
                </button>
                <button
                  class="btn option-btn"
                  :class="{ active: config.triggerStyle === 'column' }"
                  @click="handleTriggerStyleChange('column')"
                >
                  ▮ Column
                </button>
              </div>
            </div>
          </div>
        </div>

        <!-- Todo Module -->
        <div v-else-if="activeModule === 'todos'" class="module-content">
          <TodoModule />
        </div>

        <!-- Placeholder Modules -->
        <div
          v-else-if="
            [
              'placeholder2',
              'placeholder3',
              'placeholder4',
              'placeholder5',
              'placeholder6',
              'placeholder7',
            ].includes(activeModule)
          "
          class="module-content"
        >
          <div class="card">
            <h2 style="margin-bottom: 16px">📦 {{ activeModule }}</h2>
            <p style="color: var(--text-secondary); text-align: center">
              Coming soon...
            </p>
          </div>
        </div>

        <!-- Fallback for unknown modules -->
        <div v-else class="module-content">
          <div class="card">
            <h2 style="margin-bottom: 16px">{{ activeModule }}</h2>
            <p style="color: var(--text-secondary); text-align: center">
              This module is coming soon.
            </p>
          </div>
        </div>
      </div>
    </div>

    <!-- Trigger zone (right for left-position) -->
    <div
      v-if="windowPosition === 'left'"
      class="trigger-zone trigger-right"
      @mouseenter="triggerStyle === 'column' ? onMouseEnter() : undefined"
    >
      <div class="trigger-tab" @mouseenter="onMouseEnter">
        <span class="trigger-arrow">{{ isTucked ? "▶" : "◀" }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const runtimeConfig = useRuntimeConfig();
const appVersion = runtimeConfig.public.appVersion;

const { inputs, levels, isLong, results, error, calculate, clear } =
  useCalculator();
const {
  fibPrices,
  isProcessing,
  status,
  scanRegion,
  captureAndExtract,
  getLevelPrices,
  clearFibPrices,
} = useFibExtractor();
const {
  config,
  loadConfig,
  setCalcSettings,
  setScanRegion,
  setWindowPosition,
  setColorTheme,
  setTriggerStyle,
  setMonitorIndex,
} = useConfig();

const isPinned = ref(false);
const isTucked = ref(true);
const activeModule = ref("home");

const homeModules = [
  {
    id: "notes",
    label: "Notes",
    icon: '<svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="16" y1="13" x2="8" y2="13"/><line x1="16" y1="17" x2="8" y2="17"/></svg>',
  },
  {
    id: "todos",
    label: "Todo List",
    icon: '<svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M9 11l3 3L22 4"/><path d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11"/></svg>',
  },
  {
    id: "calculator",
    label: "Calculator",
    icon: '<svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="4" y="2" width="16" height="20" rx="2"/><line x1="8" y1="6" x2="16" y2="6"/><line x1="8" y1="10" x2="16" y2="10"/><line x1="8" y1="14" x2="16" y2="14"/></svg>',
  },
  {
    id: "placeholder2",
    label: "Placeholder 2",
    icon: '<svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="16"/><line x1="8" y1="12" x2="16" y2="12"/></svg>',
  },
  {
    id: "placeholder3",
    label: "Placeholder 3",
    icon: '<svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polygon points="12 2 2 7 12 12 22 7 12 2"/><polyline points="2 17 12 22 22 17"/><polyline points="2 12 12 17 22 12"/></svg>',
  },
  {
    id: "placeholder4",
    label: "Placeholder 4",
    icon: '<svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/></svg>',
  },
  {
    id: "placeholder5",
    label: "Placeholder 5",
    icon: '<svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 2L2 7l10 5 10-5-10-5z"/><path d="M2 17l10 5 10-5M2 12l10 5 10-5"/></svg>',
  },
  {
    id: "placeholder6",
    label: "Placeholder 6",
    icon: '<svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><path d="M8 14s1.5 2 4 2 4-2 4-2"/><line x1="9" y1="9" x2="9.01" y2="9"/><line x1="15" y1="9" x2="15.01" y2="9"/></svg>',
  },
  {
    id: "placeholder7",
    label: "Placeholder 7",
    icon: '<svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M20.59 13.41l-7.17 7.17a2 2 0 0 1-2.83 0L2 12V2h10l8.59 8.59a2 2 0 0 1 0 2.82z"/><line x1="7" y1="7" x2="7.01" y2="7"/></svg>',
  },
];
const windowPosition = computed(() => config.value.windowPosition || "left");
const triggerStyle = computed(() => config.value.triggerStyle || "halfcircle");
const availableMonitors = ref<
  Array<{
    index: number;
    name: string;
    width: number;
    height: number;
    is_primary: boolean;
  }>
>([]);
let invoke: any = null;

// Check if running in Tauri
const isTauri =
  typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

// Load saved settings on mount
onMounted(async () => {
  await loadConfig();
  if (config.value.scanRegion) {
    scanRegion.value = config.value.scanRegion;
    status.value = `Region: ${config.value.scanRegion[2]}x${config.value.scanRegion[3]}`;
  }
  applyTheme();
  if (isTauri) {
    const core = await import("@tauri-apps/api/core");
    invoke = core.invoke;

    // Load available monitors
    try {
      availableMonitors.value = await invoke("get_available_monitors");
    } catch (e) {
      console.warn("Failed to load monitors:", e);
      availableMonitors.value = [
        {
          index: 0,
          name: "Primary Monitor",
          width: 1920,
          height: 1080,
          is_primary: true,
        },
      ];
    }

    await invoke("setup_window_size", {
      monitorIndex: config.value.monitorIndex,
    });
    // Start tucked
    await invoke("tuck_window", {
      position: config.value.windowPosition || "left",
      monitorIndex: config.value.monitorIndex,
    });
    try {
      const { register } = await import("@tauri-apps/plugin-global-shortcut");
      await register("F9", handleCapture);
    } catch (e) {
      console.warn("Failed to register hotkey:", e);
    }
  }
});

async function onMouseEnter() {
  if (isTauri && invoke) {
    await invoke("show_window", {
      position: windowPosition.value,
      monitorIndex: config.value.monitorIndex,
    });
  }
  isTucked.value = false;
}

async function onMouseLeave() {
  if (isPinned.value) return;
  if (isTauri && invoke) {
    await invoke("tuck_window", {
      position: windowPosition.value,
      monitorIndex: config.value.monitorIndex,
    });
  }
  isTucked.value = true;
}

async function togglePin() {
  isPinned.value = !isPinned.value;
  if (isPinned.value && isTauri && invoke) {
    await invoke("show_window", {
      position: windowPosition.value,
      monitorIndex: config.value.monitorIndex,
    });
    isTucked.value = false;
  }
}

async function handleCapture() {
  await captureAndExtract();
  // Update levels from extracted fib prices
  const { entry, tp, sl } = getLevelPrices(isLong.value);
  if (entry) levels.entry = entry;
  if (tp) levels.tp = tp;
  if (sl) levels.sl = sl;
}

async function toggleRegion() {
  if (scanRegion.value) {
    scanRegion.value = null;
    setScanRegion(null);
    status.value = "Region cleared";
  } else {
    await selectRegion();
  }
}

async function selectRegion() {
  if (!isTauri || !invoke) return;

  // Open the region selector window
  await invoke("open_region_selector");

  // Poll for result (window closes after selection)
  const checkResult = async () => {
    const region = await invoke<[number, number, number, number] | null>(
      "get_selected_region",
    );
    if (region) {
      scanRegion.value = region;
      setScanRegion(region);
      status.value = `Region: ${region[2]}x${region[3]}`;
    } else {
      // Check if selector window still exists
      const { WebviewWindow } = await import("@tauri-apps/api/webviewWindow");
      const selectorWindow = await WebviewWindow.getByLabel("region-selector");
      if (selectorWindow) {
        // Still selecting, check again
        setTimeout(checkResult, 100);
      } else {
        status.value = "Region selection cancelled";
      }
    }
  };

  // Start polling after a short delay
  setTimeout(checkResult, 200);
}

function setDirection(long: boolean) {
  isLong.value = long;
  // Refresh levels if we have fib prices
  if (Object.keys(fibPrices.value).length > 0) {
    const { entry, tp, sl } = getLevelPrices(long);
    if (entry) levels.entry = entry;
    if (tp) levels.tp = tp;
    if (sl) levels.sl = sl;
  }
}

function handleClear() {
  clear();
  clearFibPrices();
}

function updateLevels(newLevels: typeof levels) {
  Object.assign(levels, newLevels);
}

function updateInputs(newInputs: typeof inputs) {
  Object.assign(inputs, newInputs);
  setCalcSettings(newInputs);
}

async function copyToClipboard(value: string) {
  try {
    if (window.__TAURI__) {
      const { writeText } =
        await import("@tauri-apps/plugin-clipboard-manager");
      await writeText(value);
    } else {
      await navigator.clipboard.writeText(value);
    }
  } catch (e) {
    console.warn("Copy failed:", e);
  }
}

async function handlePositionChange(position: "left" | "right") {
  setWindowPosition(position);
  if (isTauri && invoke) {
    await invoke("set_window_position", {
      position,
      monitorIndex: config.value.monitorIndex,
    });
  }
}

async function handleMonitorChange(event: Event) {
  const target = event.target as HTMLSelectElement;
  const monitorIndex = parseInt(target.value);
  setMonitorIndex(monitorIndex);

  if (isTauri && invoke) {
    const wasTucked = isTucked.value;

    // Force tuck before switching to prevent white flash
    if (!wasTucked) {
      await invoke("tuck_window", {
        position: windowPosition.value,
        monitorIndex: config.value.monitorIndex, // Use current monitor for tuck
      });
      isTucked.value = true;
    }

    // Reposition window to the new monitor (already tucked)
    await invoke("setup_window_size", { monitorIndex });
    await invoke("tuck_window", {
      position: windowPosition.value,
      monitorIndex,
    });

    // If it wasn't tucked before, show it again on the new monitor
    if (!wasTucked) {
      await invoke("show_window", {
        position: windowPosition.value,
        monitorIndex,
      });
      isTucked.value = false;
    }
  }
}

function handleThemeChange(theme: "default" | "monochrome") {
  setColorTheme(theme);
  applyTheme();
}

function handleTriggerStyleChange(style: "column" | "halfcircle") {
  setTriggerStyle(style);
}

function applyTheme() {
  if (config.value.colorTheme) {
    document.documentElement.setAttribute(
      "data-theme",
      config.value.colorTheme,
    );
  }
}
</script>

<style scoped>
.app-wrapper {
  height: 100vh;
  width: 340px;
  display: flex;
  flex-direction: row;
  overflow: hidden;
}

/* Tucked state - window shrinks, hide main content */
.app-wrapper.tucked {
  pointer-events: none;
}
.app-wrapper.tucked .main-container {
  display: none;
}

.main-container {
  width: 320px;
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  background: var(--bg-primary);
}

/* Trigger zone: no layout space, just a positioning anchor */
.trigger-zone {
  position: relative;
  width: 20px;
  flex-shrink: 0;
  pointer-events: none;
}

/* Half-circle tab: absolutely positioned so only it is visible */
.trigger-tab {
  position: absolute;
  top: 50%;
  transform: translateY(-50%);
  width: 20px;
  height: 44px;
  background: var(--bg-primary);
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: background 0.15s ease;
  border: 1px solid var(--border-color);
  pointer-events: auto;
}

/* Right-side tab: flat left edge, rounded right edge */
.trigger-right .trigger-tab {
  right: 0;
  border-radius: 0 22px 22px 0;
  border-left: none;
}

/* Left-side tab: rounded left edge, flat right edge */
.trigger-left .trigger-tab {
  left: 0;
  border-radius: 22px 0 0 22px;
  border-right: none;
}

.trigger-tab:hover {
  background: var(--bg-secondary);
}

.trigger-arrow {
  color: var(--text-secondary);
  font-size: 12px;
}

.trigger-tab:hover .trigger-arrow {
  color: var(--text-primary);
}

/* Column trigger style overrides */
.trigger-column.tucked {
  pointer-events: auto;
}

.trigger-column .trigger-zone {
  pointer-events: auto;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
}

.trigger-column .trigger-zone.trigger-right {
  border-left: none;
}

.trigger-column .trigger-zone.trigger-left {
  border-right: none;
}

.trigger-column .trigger-tab {
  position: static;
  transform: none;
  width: 100%;
  height: 100%;
  border: none;
  border-radius: 0;
  background: transparent;
}

.trigger-column .trigger-tab:hover {
  background: var(--bg-secondary);
}

.scroll-content {
  flex: 1;
  overflow-y: auto;
  padding: 8px 12px;
}

.header {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 6px 0;
}

.header-left,
.header-right {
  display: flex;
  align-items: center;
  gap: 4px;
}

.pin-btn.active {
  background: var(--accent-green-dim);
}

.title {
  font-size: 22px;
  font-weight: 900;
  color: white;
  letter-spacing: 1px;
  text-shadow: 0 0 1px rgba(255, 255, 255, 0.5);
}

.capture-row {
  display: flex;
  justify-content: center;
  gap: 8px;
  margin: 6px 0;
}

.capture-btn {
  min-width: 140px;
}

.region-btn.active {
  background: var(--accent-green-dim);
  color: white;
}

.status {
  text-align: center;
  font-size: 13px;
  color: var(--text-secondary);
  margin: 3px 0;
}

.direction-toggle {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 8px;
  margin: 6px 0;
}

.direction-btn {
  width: 120px;
  font-weight: 700;
}

.direction-btn.long.active {
  background: var(--accent-green-dim);
  color: white;
}

.direction-btn.short.active {
  background: var(--accent-red-dim);
  color: white;
}

.direction-btn:not(.active) {
  background: #444;
  color: var(--text-secondary);
}

.module-content {
  padding: 8px 0;
}

.module-content .card {
  margin: 8px 0;
}

.module-content h2 {
  font-size: 18px;
  font-weight: 700;
  color: var(--text-primary);
  text-align: center;
}

.version-badge {
  text-align: center;
  font-size: 12px;
  color: var(--text-secondary);
  margin-bottom: 16px;
  padding: 6px 12px;
  background: var(--input-bg);
  border-radius: 6px;
  display: block;
}

.setting-group {
  margin-bottom: 16px;
}

.setting-label {
  display: block;
  font-size: 13px;
  color: var(--text-secondary);
  margin-bottom: 8px;
  font-weight: 500;
}

.setting-options {
  display: flex;
  gap: 8px;
}

.option-btn {
  flex: 1;
  background: var(--input-bg);
  border: 1px solid var(--border-color);
  color: var(--text-secondary);
  font-size: 13px;
}

.option-btn.active {
  background: var(--accent-blue);
  border-color: var(--accent-blue);
  color: white;
}

.option-btn:hover:not(.active) {
  background: var(--bg-secondary);
  border-color: var(--accent-blue);
}

.monitor-select {
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

.monitor-select:hover {
  border-color: var(--accent-blue);
  background: var(--bg-secondary);
}

.monitor-select:focus {
  outline: none;
  border-color: var(--accent-blue);
  background: var(--bg-secondary);
}

.monitor-select option {
  background: var(--bg-secondary);
  color: var(--text-primary);
}

/* Home Hub */
.home-hub {
  padding: 4px 0;
}

.hub-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 10px;
}

.hub-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 10px;
  background: var(--bg-card);
  border: 1px solid var(--border-color);
  border-radius: 10px;
  padding: 20px 12px;
  cursor: pointer;
  transition: all 0.15s ease;
  color: var(--text-secondary);
}

.hub-card:hover {
  border-color: var(--accent-blue);
  color: var(--text-primary);
  background: var(--bg-secondary);
}

.hub-icon {
  display: flex;
  align-items: center;
  justify-content: center;
}

.hub-label {
  font-size: 13px;
  font-weight: 600;
  letter-spacing: 0.3px;
}
</style>
