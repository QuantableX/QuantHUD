export interface TranscriptEntry {
  id: string;
  text: string;
  timestamp: number;
}

const TRANSCRIPT_KEY = "quanthud_transcript";
const MAX_ENTRIES = 50;

function generateId(): string {
  return Date.now().toString(36) + Math.random().toString(36).slice(2, 8);
}

// ── Shared singleton state ──
const _entries = ref<TranscriptEntry[]>([]);
const _isRecording = ref(false);
const _interimText = ref("");
const _audioLevel = ref(0);
/** Per-band frequency levels (20 bars) for visualizer - 0..1 */
const _frequencyBars = ref<number[]>(Array(20).fill(0));
const _error = ref("");
const _warning = ref("");
/** Buffer of recognized text segments accumulated during a recording session */
let _pendingTexts: string[] = [];
let _initialized = false;
let _audioCtx: AudioContext | null = null;
let _analyser: AnalyserNode | null = null;
let _micStream: MediaStream | null = null;
let _levelRaf = 0;
let _eventCleanups: Array<() => void> = [];
const NUM_VISUALIZER_BARS = 20;

async function _load() {
  try {
    if (typeof window !== "undefined" && (window as any).__TAURI__) {
      const { invoke } = await import("@tauri-apps/api/core");
      const saved = await invoke<string>("load_config");
      if (saved) {
        const config = JSON.parse(saved);
        if (config._transcriptHistory) {
          _entries.value = config._transcriptHistory;
          return;
        }
      }
    }
    const saved = localStorage.getItem(TRANSCRIPT_KEY);
    if (saved) _entries.value = JSON.parse(saved);
  } catch (e) {
    console.warn("Failed to load transcript history:", e);
  }
}

async function _save() {
  try {
    if (typeof window !== "undefined" && (window as any).__TAURI__) {
      const { invoke } = await import("@tauri-apps/api/core");
      const raw = await invoke<string>("load_config");
      const config = raw ? JSON.parse(raw) : {};
      config._transcriptHistory = _entries.value;
      await invoke("save_config", { config: JSON.stringify(config) });
      emitSync("transcript");
    } else {
      localStorage.setItem(TRANSCRIPT_KEY, JSON.stringify(_entries.value));
    }
  } catch (e) {
    console.warn("Failed to save transcript history:", e);
  }
}

async function _writeClipboard(text: string) {
  try {
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

function _addEntry(text: string) {
  const trimmed = text.trim();
  if (!trimmed) return;
  _entries.value.unshift({
    id: generateId(),
    text: trimmed,
    timestamp: Date.now(),
  });
  if (_entries.value.length > MAX_ENTRIES) _entries.value.pop();
  _save();
  _writeClipboard(trimmed);
}

/** Flush all buffered speech segments as a single entry */
function _flushPending() {
  if (_pendingTexts.length === 0) return;
  const combined = _pendingTexts.join(" ").trim();
  _pendingTexts = [];
  if (combined) {
    _addEntry(combined);
  }
}

// ── Audio level monitoring (getUserMedia + Web Audio API) ──
async function _startAudioLevel() {
  try {
    _micStream = await navigator.mediaDevices.getUserMedia({ audio: true });
    _audioCtx = new AudioContext();
    _analyser = _audioCtx.createAnalyser();
    _analyser.fftSize = 256;
    const source = _audioCtx.createMediaStreamSource(_micStream);
    source.connect(_analyser);
    const dataArray = new Uint8Array(_analyser.frequencyBinCount);
    const binCount = _analyser.frequencyBinCount;
    const bars = NUM_VISUALIZER_BARS;
    const binsPerBar = Math.max(1, Math.floor(binCount / bars));

    function tick() {
      if (!_analyser) return;
      _analyser.getByteFrequencyData(dataArray);
      let sum = 0;
      const barLevels: number[] = [];
      for (let b = 0; b < bars; b++) {
        let barSum = 0;
        const start = b * binsPerBar;
        const end = Math.min(start + binsPerBar, binCount);
        for (let i = start; i < end; i++) {
          barSum += dataArray[i];
          sum += dataArray[i];
        }
        barLevels.push(barSum / ((end - start) * 255));
      }
      _audioLevel.value = sum / (binCount * 255);
      _frequencyBars.value = barLevels;
      _levelRaf = requestAnimationFrame(tick);
    }
    tick();
  } catch (e: any) {
    console.warn("Mic level monitoring unavailable:", e);
    // Don't set error - the visualizer is optional
  }
}

function _stopAudioLevel() {
  cancelAnimationFrame(_levelRaf);
  _audioLevel.value = 0;
  _frequencyBars.value = Array(NUM_VISUALIZER_BARS).fill(0);
  if (_micStream) {
    _micStream.getTracks().forEach((t) => t.stop());
    _micStream = null;
  }
  if (_audioCtx) {
    _audioCtx.close();
    _audioCtx = null;
  }
  _analyser = null;
}

// ── Speech recognition via Tauri backend (Windows Speech API) ──
async function _startTauriRecognition(language: string) {
  const { invoke } = await import("@tauri-apps/api/core");
  const { listen } = await import("@tauri-apps/api/event");

  // Buffer speech results during recording; they are committed on stop
  const unlistenResult = await listen(
    "speech-result",
    (event: { payload?: unknown }) => {
      const raw = event.payload;
      const text =
        typeof raw === "string" ? raw : raw != null ? String(raw) : "";
      const trimmed = text.trim();
      if (trimmed) {
        // Accumulate into the pending buffer and show as interim text
        _pendingTexts.push(trimmed);
        _interimText.value = _pendingTexts.join(" ");
      }
    },
  );

  const unlistenWarning = await listen<string>("speech-warning", (event) => {
    // Non-fatal: show as warning, keep recording
    _warning.value = event.payload || "Speech recognition warning";
  });

  const unlistenError = await listen<string>("speech-error", (event) => {
    _error.value = event.payload || "Speech recognition error";
    _isRecording.value = false;
    _pendingTexts = [];
    _interimText.value = "";
    _stopAudioLevel();
    _cleanupListeners();
  });

  // speech-stopped is emitted by the backend only when the user has pressed stop
  // (SPEECH_ACTIVE was false when Completed fired). We do NOT stop recording here
  // if the user is still recording — the backend handles auto-restart internally.
  const unlistenStopped = await listen("speech-stopped", () => {
    if (_isRecording.value) {
      _isRecording.value = false;
      _stopAudioLevel();
    }
    _cleanupListeners();
  });

  _eventCleanups = [
    unlistenResult,
    unlistenWarning,
    unlistenError,
    unlistenStopped,
  ];

  // Pass the language tag to the backend.
  // "system" → pass null so Rust uses the Windows default recognizer.
  // Any other tag (e.g. "de-DE") → pass the tag string.
  const langParam = language === "system" ? null : language;
  await invoke("start_speech_recognition", { language: langParam });
}

async function _stopTauriRecognition() {
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    await invoke("stop_speech_recognition");
  } catch (e) {
    console.warn("Failed to stop speech recognition:", e);
  }
}

function _cleanupListeners() {
  _eventCleanups.forEach((fn) => fn());
  _eventCleanups = [];
}

/** Read the current speech language from config (falls back to "system") */
async function _getSpeechLanguage(): Promise<string> {
  try {
    if (typeof window !== "undefined" && (window as any).__TAURI__) {
      const { invoke } = await import("@tauri-apps/api/core");
      const saved = await invoke<string>("load_config");
      if (saved) {
        const config = JSON.parse(saved);
        if (config.speechLanguage) return config.speechLanguage;
      }
    } else {
      const saved = localStorage.getItem("quanthub_config");
      if (saved) {
        const config = JSON.parse(saved);
        if (config.speechLanguage) return config.speechLanguage;
      }
    }
  } catch (e) {
    console.warn("Failed to read speech language from config:", e);
  }
  return "system";
}

export function useTranscript() {
  if (!_initialized) {
    _initialized = true;
    _load();
    onSyncEvent("transcript", _load);
  }

  async function startRecording() {
    _error.value = "";
    _warning.value = "";
    _interimText.value = "";
    _pendingTexts = [];
    _isRecording.value = true;

    // Start audio level visualizer
    _startAudioLevel();

    // Read language preference from config
    const language = await _getSpeechLanguage();

    // Use Tauri backend for speech recognition
    try {
      await _startTauriRecognition(language);
    } catch (e: any) {
      _error.value = `Speech recognition failed: ${e.message || e}`;
      _isRecording.value = false;
      _pendingTexts = [];
      _interimText.value = "";
      _stopAudioLevel();
    }
  }

  async function stopRecording() {
    _isRecording.value = false;
    _stopAudioLevel();
    // Allow backend a moment to emit any final phrase before we stop the session
    await new Promise((r) => setTimeout(r, 400));
    await _stopTauriRecognition();
    _cleanupListeners();
    // Commit all buffered speech as a single transcript entry
    _interimText.value = "";
    _flushPending();
  }

  function toggleRecording() {
    if (_isRecording.value) {
      stopRecording();
    } else {
      startRecording();
    }
  }

  function copyEntry(text: string) {
    _writeClipboard(text);
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
    isRecording: _isRecording,
    interimText: _interimText,
    audioLevel: _audioLevel,
    frequencyBars: _frequencyBars,
    error: _error,
    warning: _warning,
    toggleRecording,
    copyEntry,
    removeEntry,
    clearAll,
  };
}
