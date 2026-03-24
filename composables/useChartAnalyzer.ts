export interface ChartAnalysisResult {
  market_phase: string;
  schematic: string;
  wyckoff_phase: string;
  events: string[];
  current_transition: string;
  bias: string;
}

export interface AnalysisHistoryEntry {
  id: string;
  timestamp: number;
  result: ChartAnalysisResult;
  imageBase64?: string;
}

const HISTORY_KEY = "quanthud_analysis_history";
const MAX_HISTORY = 30;

export function useChartAnalyzer() {
  const isAnalyzing = ref(false);
  const status = ref("Ready");
  const result = ref<ChartAnalysisResult | null>(null);
  const rawResponse = ref("");
  const capturedImage = ref("");
  const history = ref<AnalysisHistoryEntry[]>([]);

  function loadHistory() {
    try {
      const saved = localStorage.getItem(HISTORY_KEY);
      if (saved) history.value = JSON.parse(saved);
    } catch {
      history.value = [];
    }
  }

  function saveHistory() {
    try {
      localStorage.setItem(HISTORY_KEY, JSON.stringify(history.value));
    } catch {
      /* ignore */
    }
  }

  function addToHistory(res: ChartAnalysisResult, imageBase64?: string) {
    const entry: AnalysisHistoryEntry = {
      id: Date.now().toString(36),
      timestamp: Date.now(),
      result: res,
      imageBase64,
    };
    history.value.unshift(entry);
    if (history.value.length > MAX_HISTORY) {
      history.value = history.value.slice(0, MAX_HISTORY);
    }
    saveHistory();
  }

  function deleteHistoryEntry(id: string) {
    history.value = history.value.filter((e) => e.id !== id);
    saveHistory();
  }

  function clearHistory() {
    history.value = [];
    saveHistory();
  }

  async function captureAndAnalyze(
    region: [number, number, number, number] | null,
    provider: string,
    baseUrl: string,
    model: string,
    analysisTypes: string[],
    windowConfig: {
      position: string;
      monitorIndex: number;
      triggerStyle: string;
    },
  ) {
    if (isAnalyzing.value) return;

    isAnalyzing.value = true;
    status.value = "Capturing...";
    result.value = null;
    rawResponse.value = "";

    try {
      const isTauri =
        typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
      if (!isTauri) {
        status.value = "Only works in Tauri";
        return;
      }

      const { invoke } = await import("@tauri-apps/api/core");

      // Hide HUD so it doesn't appear in the screenshot
      await invoke("tuck_window", {
        position: windowConfig.position,
        monitorIndex: windowConfig.monitorIndex,
        triggerStyle: windowConfig.triggerStyle,
      });
      await new Promise((r) => setTimeout(r, 250));

      // Capture screenshot
      const capture = await invoke<{
        image_base64: string;
        width: number;
        height: number;
      }>("capture_screen", {
        region,
        defaultCrop: false,
      });

      // Show HUD again immediately after capture
      await invoke("show_window", {
        position: windowConfig.position,
        monitorIndex: windowConfig.monitorIndex,
      });

      capturedImage.value = capture.image_base64;
      status.value = "Analyzing chart...";

      // Send to local AI for analysis
      const response = await invoke<string>("analyze_chart", {
        imageBase64: capture.image_base64,
        analysisTypes,
        provider,
        baseUrl,
        model,
      });

      rawResponse.value = response;

      // Parse JSON from response
      try {
        let jsonStr = response;
        const fenceMatch = jsonStr.match(/```(?:json)?\s*([\s\S]*?)```/);
        if (fenceMatch) {
          jsonStr = fenceMatch[1];
        }
        const jsonMatch = jsonStr.match(/\{[\s\S]*\}/);
        if (jsonMatch) {
          const parsed = JSON.parse(jsonMatch[0]);
          result.value = parsed;
          addToHistory(parsed, capturedImage.value || undefined);
          status.value = "Analysis complete";
        } else {
          status.value = "Could not parse response";
        }
      } catch {
        status.value = "Could not parse response";
      }
    } catch (e: any) {
      const msg = e.message || String(e);
      status.value = msg.length > 80 ? msg.slice(0, 80) + "..." : msg;
      // Try to show window on error too
      try {
        const { invoke } = await import("@tauri-apps/api/core");
        await invoke("show_window", {
          position: windowConfig.position,
          monitorIndex: windowConfig.monitorIndex,
        });
      } catch {
        /* ignore */
      }
    } finally {
      isAnalyzing.value = false;
    }
  }

  function clearResults() {
    result.value = null;
    rawResponse.value = "";
    capturedImage.value = "";
    status.value = "Ready";
  }

  // Load history on creation
  loadHistory();

  return {
    isAnalyzing,
    status,
    result,
    rawResponse,
    capturedImage,
    history,
    captureAndAnalyze,
    clearResults,
    deleteHistoryEntry,
    clearHistory,
  };
}
