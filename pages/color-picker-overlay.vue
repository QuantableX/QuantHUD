<template>
  <div class="overlay" @click="pickAtCursor">
    <div class="instructions">Click anywhere to pick the color at that point • ESC to cancel</div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";

async function pickAtCursor() {
  try {
    const hex = await invoke<string>("pick_screen_color");
    await invoke("set_picked_color", { color: hex });
  } catch (e) {
    console.warn("Failed to pick color:", e);
    await invoke("set_picked_color", { color: null });
  }
}

async function cancel() {
  await invoke("set_picked_color", { color: null });
}

function handleKeydown(e: KeyboardEvent) {
  if (e.key === "Escape") {
    e.preventDefault();
    cancel();
  }
}

onMounted(() => {
  window.addEventListener("keydown", handleKeydown);
});

onUnmounted(() => {
  window.removeEventListener("keydown", handleKeydown);
});
</script>

<style>
html,
body,
#__nuxt {
  background: rgba(0, 0, 0, 0.01) !important;
  margin: 0;
  padding: 0;
  overflow: hidden;
  width: 100vw;
  height: 100vh;
}
</style>

<style scoped>
.overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: transparent;
  cursor: crosshair;
}

.instructions {
  position: absolute;
  top: 20px;
  left: 50%;
  transform: translateX(-50%);
  background: rgba(0, 0, 0, 0.7);
  color: white;
  padding: 8px 16px;
  border-radius: 6px;
  font-size: 13px;
  pointer-events: none;
  user-select: none;
}
</style>

