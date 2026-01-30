export function useAutoHide(isPinned: Ref<boolean>) {
  const isHidden = ref(false);
  const windowWidth = 340;

  async function slideIn() {
    if (isHidden.value || isPinned.value) return;

    if (window.__TAURI__) {
      try {
        const { getCurrentWindow } = await import("@tauri-apps/api/window");
        const win = getCurrentWindow();
        const position = await win.outerPosition();
        await win.setPosition({
          x: -windowWidth - 10,
          y: position.y,
          type: "Physical",
        });
        isHidden.value = true;
      } catch (e) {
        console.warn("slideIn failed:", e);
      }
    }
  }

  async function slideOut() {
    if (!isHidden.value) return;

    if (window.__TAURI__) {
      try {
        const { getCurrentWindow } = await import("@tauri-apps/api/window");
        const win = getCurrentWindow();
        const position = await win.outerPosition();
        await win.setPosition({ x: 0, y: position.y, type: "Physical" });
        await win.setFocus();
        isHidden.value = false;
      } catch (e) {
        console.warn("slideOut failed:", e);
      }
    }
  }

  // Track mouse position for auto-hide
  let checkInterval: ReturnType<typeof setInterval> | null = null;

  async function startTracking() {
    if (checkInterval) return;

    checkInterval = setInterval(async () => {
      if (!window.__TAURI__ || isPinned.value) return;

      try {
        const { getCurrentWindow } = await import("@tauri-apps/api/window");
        const win = getCurrentWindow();
        const pos = await win.outerPosition();
        const size = await win.outerSize();

        // Get cursor position via Tauri
        const { invoke } = await import("@tauri-apps/api/core");
        const cursor = await invoke<{ x: number; y: number }>(
          "get_cursor_position",
        ).catch(() => null);

        if (!cursor) return;

        const inWindow =
          cursor.x >= pos.x &&
          cursor.x <= pos.x + size.width &&
          cursor.y >= pos.y &&
          cursor.y <= pos.y + size.height;

        const nearEdge = cursor.x <= 20;

        if (isHidden.value && nearEdge) {
          slideOut();
        } else if (!isHidden.value && !inWindow) {
          slideIn();
        }
      } catch (e) {
        // Ignore
      }
    }, 100);
  }

  function stopTracking() {
    if (checkInterval) {
      clearInterval(checkInterval);
      checkInterval = null;
    }
  }

  onMounted(() => {
    startTracking();
  });

  onUnmounted(() => {
    stopTracking();
  });

  return {
    isHidden,
    slideIn,
    slideOut,
    startTracking,
    stopTracking,
  };
}
