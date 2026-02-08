export interface ClockEntry {
  id: string;
  label: string;
  timezone: string;
}

export interface TimerProfile {
  id: string;
  name: string;
  durationSec: number;
}

const WORLDCLOCK_KEY = "quanthud_worldclock";

function generateId(): string {
  return Date.now().toString(36) + Math.random().toString(36).slice(2, 8);
}

const DEFAULT_CLOCKS: ClockEntry[] = [
  { id: "utc", label: "UTC", timezone: "UTC" },
  { id: "ny", label: "New York", timezone: "America/New_York" },
  { id: "london", label: "London", timezone: "Europe/London" },
  { id: "tokyo", label: "Tokyo", timezone: "Asia/Tokyo" },
];

const DEFAULT_PROFILES: TimerProfile[] = [
  { id: "default-30m", name: "30 min", durationSec: 1800 },
];

export function useWorldClock() {
  const clocks = ref<ClockEntry[]>([...DEFAULT_CLOCKS]);
  const timerProfiles = ref<TimerProfile[]>([...DEFAULT_PROFILES]);
  const activeProfileId = ref<string>(
    DEFAULT_PROFILES.length > 0 ? DEFAULT_PROFILES[0].id : "",
  );
  const timerRemainingMs = ref(0);
  const timerRunning = ref(false);
  const stopwatchElapsedMs = ref(0);
  const stopwatchRunning = ref(false);

  let timerInterval: ReturnType<typeof setInterval> | null = null;
  let stopwatchInterval: ReturnType<typeof setInterval> | null = null;
  let stopwatchStart = 0;
  let timerStart = 0;
  let timerSnapshot = 0;

  // --- Persistence ---
  async function load() {
    try {
      if (typeof window !== "undefined" && (window as any).__TAURI__) {
        const { invoke } = await import("@tauri-apps/api/core");
        const saved = await invoke<string>("load_config");
        if (saved) {
          const config = JSON.parse(saved);
          if (config._worldclock) {
            clocks.value = config._worldclock.clocks || [...DEFAULT_CLOCKS];
            timerProfiles.value = config._worldclock.profiles || [
              ...DEFAULT_PROFILES,
            ];
            return;
          }
        }
      }
      const saved = localStorage.getItem(WORLDCLOCK_KEY);
      if (saved) {
        const data = JSON.parse(saved);
        clocks.value = data.clocks || [...DEFAULT_CLOCKS];
        timerProfiles.value = data.profiles || [...DEFAULT_PROFILES];
      }
    } catch (e) {
      console.warn("Failed to load worldclock:", e);
    }
  }

  async function save() {
    try {
      const payload = { clocks: clocks.value, profiles: timerProfiles.value };
      if (typeof window !== "undefined" && (window as any).__TAURI__) {
        const { invoke } = await import("@tauri-apps/api/core");
        const raw = await invoke<string>("load_config");
        const config = raw ? JSON.parse(raw) : {};
        config._worldclock = payload;
        await invoke("save_config", { config: JSON.stringify(config) });
      } else {
        localStorage.setItem(WORLDCLOCK_KEY, JSON.stringify(payload));
      }
    } catch (e) {
      console.warn("Failed to save worldclock:", e);
    }
  }

  function addClock(label: string, timezone: string) {
    clocks.value.push({ id: generateId(), label, timezone });
    save();
  }
  function removeClock(id: string) {
    clocks.value = clocks.value.filter((c) => c.id !== id);
    save();
  }
  function addProfile(name: string, durationSec: number) {
    timerProfiles.value.push({ id: generateId(), name, durationSec });
    save();
  }
  function removeProfile(id: string) {
    timerProfiles.value = timerProfiles.value.filter((p) => p.id !== id);
    if (activeProfileId.value === id && timerProfiles.value.length > 0) {
      activeProfileId.value = timerProfiles.value[0].id;
    }
    save();
  }

  // Timer
  function startTimer() {
    if (timerRunning.value) return;
    const profile = timerProfiles.value.find(
      (p) => p.id === activeProfileId.value,
    );
    if (!profile) return;
    if (timerRemainingMs.value <= 0)
      timerRemainingMs.value = profile.durationSec * 1000;
    timerRunning.value = true;
    timerSnapshot = timerRemainingMs.value;
    timerStart = Date.now();
    timerInterval = setInterval(() => {
      const elapsed = Date.now() - timerStart;
      timerRemainingMs.value = Math.max(0, timerSnapshot - elapsed);
      if (timerRemainingMs.value <= 0) {
        stopTimer();
      }
    }, 10);
  }
  function stopTimer() {
    timerRunning.value = false;
    if (timerInterval) {
      clearInterval(timerInterval);
      timerInterval = null;
    }
  }
  function resetTimer() {
    stopTimer();
    const profile = timerProfiles.value.find(
      (p) => p.id === activeProfileId.value,
    );
    timerRemainingMs.value = profile ? profile.durationSec * 1000 : 0;
  }

  // Stopwatch
  function startStopwatch() {
    if (stopwatchRunning.value) return;
    stopwatchRunning.value = true;
    stopwatchStart = Date.now() - stopwatchElapsedMs.value;
    stopwatchInterval = setInterval(() => {
      stopwatchElapsedMs.value = Date.now() - stopwatchStart;
    }, 10);
  }
  function stopStopwatch() {
    stopwatchRunning.value = false;
    if (stopwatchInterval) {
      clearInterval(stopwatchInterval);
      stopwatchInterval = null;
    }
  }
  function resetStopwatch() {
    stopStopwatch();
    stopwatchElapsedMs.value = 0;
  }

  onMounted(() => {
    load();
  });
  onUnmounted(() => {
    stopTimer();
    stopStopwatch();
  });

  return {
    clocks,
    timerProfiles,
    activeProfileId,
    timerRemainingMs,
    timerRunning,
    stopwatchElapsedMs,
    stopwatchRunning,
    addClock,
    removeClock,
    addProfile,
    removeProfile,
    startTimer,
    stopTimer,
    resetTimer,
    startStopwatch,
    stopStopwatch,
    resetStopwatch,
    save,
  };
}
