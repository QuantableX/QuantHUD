<template>
  <div class="gc-module">
    <div class="gc-tabs">
      <button
        :class="{ active: mode === 'calculator' }"
        @click="mode = 'calculator'"
      >
        <svg
          width="14"
          height="14"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          style="vertical-align: middle; margin-right: 3px"
        >
          <rect x="4" y="2" width="16" height="20" rx="2" />
          <line x1="8" y1="6" x2="16" y2="6" />
          <line x1="8" y1="10" x2="10" y2="10" />
          <line x1="14" y1="10" x2="16" y2="10" />
          <line x1="8" y1="14" x2="10" y2="14" />
          <line x1="14" y1="14" x2="16" y2="14" />
          <line x1="8" y1="18" x2="16" y2="18" />
        </svg>
        Calculator
      </button>
      <button
        :class="{ active: mode === 'converter' }"
        @click="mode = 'converter'"
      >
        <svg
          width="14"
          height="14"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          style="vertical-align: middle; margin-right: 3px"
        >
          <polyline points="17 1 21 5 17 9" />
          <path d="M3 11V9a4 4 0 0 1 4-4h14" />
          <polyline points="7 23 3 19 7 15" />
          <path d="M21 13v2a4 4 0 0 1-4 4H3" />
        </svg>
        Converter
      </button>
    </div>
    <div v-if="mode === 'calculator'" class="gc-panel">
      <div class="calc-display">
        <div class="calc-expr">{{ expression || "&nbsp;" }}</div>
        <div class="calc-result">{{ display }}</div>
      </div>
      <div class="calc-grid">
        <button
          v-for="key in calcKeys"
          :key="key"
          :class="['calc-key', keyClass(key)]"
          @click="pressKey(key)"
        >
          {{ key }}
        </button>
      </div>
    </div>
    <div v-if="mode === 'converter'" class="gc-panel">
      <div class="conv-row">
        <select
          v-model="category"
          class="input sm"
          @change="setCategory(category)"
        >
          <option v-for="cat in categories" :key="cat" :value="cat">
            {{ cat }}
          </option>
        </select>
      </div>
      <div class="conv-fields">
        <div class="conv-field">
          <select v-model="fromUnit" class="input sm">
            <option v-for="u in currentUnits" :key="u" :value="u">
              {{ u }}
            </option>
          </select>
          <input
            v-model.number="fromValue"
            class="input"
            type="number"
            @input="convertUnits"
          />
        </div>
        <button class="btn btn-ghost swap-btn" @click="swapUnits">⇄</button>
        <div class="conv-field">
          <select v-model="toUnit" class="input sm">
            <option v-for="u in currentUnits" :key="u" :value="u">
              {{ u }}
            </option>
          </select>
          <input :value="toValue" class="input" readonly />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const {
  mode,
  display,
  expression,
  pressKey,
  category,
  fromUnit,
  toUnit,
  fromValue,
  toValue,
  convertUnits,
  swapUnits,
  setCategory,
  categories,
  currentUnits,
} = useGeneralCalc();
const calcKeys = [
  "C",
  "⌫",
  "%",
  "÷",
  "7",
  "8",
  "9",
  "×",
  "4",
  "5",
  "6",
  "-",
  "1",
  "2",
  "3",
  "+",
  "±",
  "0",
  ".",
  "=",
];
function keyClass(k: string) {
  if (["C", "⌫"].includes(k)) return "fn";
  if (["÷", "×", "-", "+", "%"].includes(k)) return "op";
  if (k === "=") return "eq";
  return "";
}
</script>

<style scoped>
.gc-module {
  padding: 4px 0;
}
.gc-tabs {
  display: flex;
  gap: 6px;
  margin-bottom: 12px;
}
.gc-tabs button {
  flex: 1;
  padding: 6px 4px;
  font-size: 11px;
  border: 1px solid var(--border-color);
  background: var(--bg-card);
  color: var(--text-secondary);
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.15s ease;
}
.gc-tabs button.active {
  background: var(--btn-primary);
  color: var(--text-primary);
  border-color: var(--btn-primary);
}
.gc-panel {
  min-height: 60px;
}
.calc-display {
  background: var(--bg-card);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 10px 12px;
  margin-bottom: 8px;
  text-align: right;
}
.calc-expr {
  font-size: 11px;
  color: var(--text-secondary);
  min-height: 16px;
}
.calc-result {
  font-size: 28px;
  font-weight: 700;
  font-family: monospace;
  color: var(--text-primary);
}
.calc-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 4px;
}
.calc-key {
  padding: 10px 0;
  font-size: 16px;
  border: 1px solid var(--border-color);
  background: var(--bg-card);
  color: var(--text-primary);
  border-radius: 6px;
  cursor: pointer;
  transition: background 0.1s;
}
.calc-key:hover {
  background: var(--bg-secondary);
}
.calc-key.fn {
  color: var(--accent-red);
}
.calc-key.op {
  color: var(--accent-blue);
}
.calc-key.eq {
  background: var(--accent-green);
  color: #fff;
  border-color: var(--accent-green);
}
.conv-row {
  margin-bottom: 8px;
}
.conv-fields {
  display: flex;
  flex-direction: column;
  gap: 8px;
  align-items: center;
}
.conv-field {
  display: flex;
  flex-direction: column;
  gap: 4px;
  width: 100%;
}
.swap-btn {
  font-size: 18px;
  padding: 2px 8px;
}
.input.sm {
  font-size: 12px;
  padding: 4px 8px;
}
</style>
