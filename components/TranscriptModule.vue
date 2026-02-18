<template>
  <div class="tr-module">
    <div class="tr-header">
      <span class="tr-title">
        <svg
          width="14"
          height="14"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          style="vertical-align: middle; margin-right: 4px"
        >
          <path d="M12 2a3 3 0 0 0-3 3v7a3 3 0 0 0 6 0V5a3 3 0 0 0-3-3Z" />
          <path d="M19 10v2a7 7 0 0 1-14 0v-2" />
          <line x1="12" y1="19" x2="12" y2="22" />
        </svg>
        Transcript
      </span>
      <button
        v-if="entries.length"
        class="btn btn-ghost btn-sm"
        @click="clearAll"
      >
        Clear All
      </button>
    </div>

    <!-- Record Button -->
    <button
      class="tr-record-btn"
      :class="{ recording: isRecording }"
      @click="toggleRecording"
    >
      <div
        v-if="isRecording"
        class="tr-level-bar"
        :style="{ width: Math.round(audioLevel * 100) + '%' }"
      ></div>
      <svg
        v-if="!isRecording"
        width="14"
        height="14"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
      >
        <path d="M12 2a3 3 0 0 0-3 3v7a3 3 0 0 0 6 0V5a3 3 0 0 0-3-3Z" />
        <path d="M19 10v2a7 7 0 0 1-14 0v-2" />
        <line x1="12" y1="19" x2="12" y2="22" />
      </svg>
      <span v-else class="tr-record-dot"></span>
      {{ isRecording ? "Stop Recording" : "Start Recording" }}
    </button>

    <!-- Error message -->
    <div v-if="error" class="tr-error">
      {{ error }}
    </div>

    <!-- Warning message (non-fatal, e.g. language fallback) -->
    <div v-if="warning && !error" class="tr-warning">
      {{ warning }}
    </div>

    <!-- Listening indicator while recording -->
    <div v-if="isRecording" class="tr-interim">
      <div class="tr-visualizer-container">
        <div class="tr-visualizer">
          <div
            v-for="(level, i) in frequencyBars"
            :key="i"
            class="tr-visualizer-bar"
            :style="{
              height: Math.max(8, level * 100) + '%',
              animationDelay: i * 0.03 + 's',
            }"
          ></div>
        </div>
        <span class="tr-interim-label">LISTENING...</span>
      </div>
      <p v-if="interimText" class="tr-interim-text">{{ interimText }}</p>
      <p v-else-if="!entries.length" class="tr-interim-hint">
        Speak to see transcript here.
      </p>
    </div>

    <!-- Empty state -->
    <div v-if="!entries.length && !isRecording && !error" class="tr-empty">
      <p>No transcripts yet.</p>
      <p class="tr-hint">Press the button above to start recording.</p>
    </div>

    <!-- Transcript list (containerized, copyable entries like clipboard/screenshots) -->
    <div v-if="entries.length" class="tr-list">
      <div v-for="entry in entries" :key="entry.id" class="tr-row">
        <div class="tr-content">
          <span class="tr-text">{{
            entry.text.length > 120
              ? entry.text.slice(0, 120) + "\u2026"
              : entry.text
          }}</span>
          <span class="tr-time">{{ formatTime(entry.timestamp) }}</span>
        </div>
        <div class="tr-actions">
          <button class="ctrl-btn" @click="copyEntry(entry.text)" title="Copy">
            <svg
              width="12"
              height="12"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
            >
              <path
                d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"
              />
              <rect x="8" y="2" width="8" height="4" rx="1" ry="1" />
            </svg>
          </button>
          <button
            class="ctrl-btn ctrl-del"
            @click="removeEntry(entry.id)"
            title="Delete"
          >
            &#x2715;
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const {
  entries,
  isRecording,
  interimText,
  audioLevel,
  frequencyBars,
  error,
  warning,
  toggleRecording,
  copyEntry,
  removeEntry,
  clearAll,
} = useTranscript();

function formatTime(ts: number) {
  const d = new Date(ts);
  return (
    d.toLocaleTimeString("en-US", {
      hour: "2-digit",
      minute: "2-digit",
      hour12: false,
    }) +
    " " +
    d.toLocaleDateString("en-US", { month: "short", day: "numeric" })
  );
}
</script>

<style scoped>
.tr-module {
  display: flex;
  flex-direction: column;
  padding: 4px 0;
  min-height: 0;
  flex: 1;
}
.tr-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 8px;
  flex-shrink: 0;
}
.tr-title {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary);
}
.tr-record-btn {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  width: 100%;
  padding: 10px 16px;
  margin-bottom: 8px;
  font-size: 13px;
  font-weight: 600;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.15s ease;
  background: var(--bg-card);
  color: var(--text-primary);
  flex-shrink: 0;
  overflow: hidden;
}
.tr-level-bar {
  position: absolute;
  left: 0;
  top: 0;
  height: 100%;
  background: rgba(255, 77, 77, 0.25);
  transition: width 0.05s linear;
  pointer-events: none;
}
.tr-record-btn:hover {
  border-color: var(--accent-blue);
  background: var(--bg-secondary);
}
.tr-record-btn.recording {
  background: var(--accent-red-dim, #5c2020);
  border-color: var(--accent-red, #ff4d4d);
  color: #ff6b6b;
}
.tr-record-btn.recording:hover {
  background: var(--accent-red, #ff4d4d);
  color: white;
}
.tr-record-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background: #ff4d4d;
  animation: pulse-dot 1s ease-in-out infinite;
}
@keyframes pulse-dot {
  0%,
  100% {
    opacity: 1;
  }
  50% {
    opacity: 0.3;
  }
}
.tr-error {
  padding: 6px 10px;
  margin-bottom: 8px;
  background: var(--accent-red-dim, #5c2020);
  border: 1px solid var(--accent-red, #ff4d4d);
  border-radius: 6px;
  font-size: 11px;
  color: #ff6b6b;
  word-break: break-word;
  flex-shrink: 0;
}
.tr-warning {
  padding: 6px 10px;
  margin-bottom: 8px;
  background: rgba(255, 180, 0, 0.12);
  border: 1px solid rgba(255, 180, 0, 0.5);
  border-radius: 6px;
  font-size: 11px;
  color: #ffb400;
  word-break: break-word;
  flex-shrink: 0;
}
.tr-interim {
  padding: 8px 10px;
  margin-bottom: 8px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  flex-shrink: 0;
}
.tr-visualizer-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
}
.tr-visualizer {
  display: flex;
  align-items: flex-end;
  justify-content: center;
  gap: 2px;
  height: 32px;
  width: 100%;
  padding: 4px 0;
}
.tr-visualizer-bar {
  width: 3px;
  min-height: 4px;
  background: var(--accent-blue, #4a9eff);
  border-radius: 2px;
  animation: visualizer-pulse 0.8s ease-in-out infinite;
  transition: height 0.1s linear;
}
@keyframes visualizer-pulse {
  0%,
  100% {
    opacity: 0.4;
  }
  50% {
    opacity: 1;
  }
}
.tr-interim-label {
  font-size: 10px;
  font-weight: 600;
  color: var(--accent-blue, #4a9eff);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  text-align: center;
}
.tr-interim-text {
  font-size: 12px;
  color: var(--text-primary);
  margin: 4px 0 0;
  word-break: break-word;
  text-align: center;
}
.tr-interim-hint {
  font-size: 11px;
  color: var(--text-secondary);
  margin: 6px 0 0;
  text-align: center;
}
.tr-empty {
  text-align: center;
  padding: 24px 0;
  color: var(--text-secondary);
  font-size: 13px;
}
.tr-hint {
  font-size: 11px;
  margin-top: 4px;
}
.tr-list {
  display: flex;
  flex-direction: column;
  gap: 4px;
  flex: 1;
  overflow-y: auto;
  min-height: 0;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  padding: 4px;
  background: var(--bg-secondary);
}
.tr-row {
  display: flex;
  align-items: flex-start;
  gap: 6px;
  padding: 6px 8px;
  background: var(--bg-card);
  border: 1px solid var(--border-color);
  border-radius: 6px;
}
.tr-content {
  flex: 1;
  min-width: 0;
}
.tr-text {
  font-size: 12px;
  color: var(--text-primary);
  word-break: break-word;
  display: block;
}
.tr-time {
  font-size: 10px;
  color: var(--text-secondary);
  margin-top: 2px;
  display: block;
}
.tr-actions {
  display: flex;
  gap: 2px;
  flex-shrink: 0;
}
.ctrl-btn {
  background: none;
  border: none;
  color: var(--text-secondary);
  cursor: pointer;
  font-size: 12px;
  width: 22px;
  height: 22px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 3px;
}
.ctrl-btn:hover {
  color: var(--text-primary);
}
.ctrl-del:hover {
  color: var(--accent-red);
}
.btn-sm {
  font-size: 12px;
  padding: 4px 10px;
}
</style>
