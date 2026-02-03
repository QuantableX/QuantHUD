import pkg from "./package.json";

export default defineNuxtConfig({
  ssr: false,
  devtools: { enabled: false },
  telemetry: false,

  runtimeConfig: {
    public: {
      appVersion: pkg.version,
    },
  },

  app: {
    head: {
      title: "QuantHUD",
      meta: [
        { charset: "utf-8" },
        { name: "viewport", content: "width=device-width, initial-scale=1" },
      ],
    },
  },

  css: ["~/assets/css/main.css"],

  vite: {
    clearScreen: false,
    server: {
      strictPort: true,
    },
    envPrefix: ["VITE_", "TAURI_"],
  },

  compatibilityDate: "2024-01-01",
});
