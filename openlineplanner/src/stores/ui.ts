import { defineStore } from "pinia";
import type { MapPosition } from "@/types";

interface UIState {
  mapStyle: string;
  mapPosition: MapPosition;
  isLoading: boolean;
  error: string | null;
  sidebarOpen: boolean;
  theme: "light" | "dark";
}

export const useUIStore = defineStore("ui", {
  state: (): UIState => ({
    mapStyle: "mapbox://styles/mapbox/streets-v11",
    mapPosition: {
      lat: 48.2082,
      lng: 16.3738,
      zoom: 12,
    },
    isLoading: false,
    error: null,
    sidebarOpen: false,
    theme: "light",
  }),

  getters: {
    isError: (state) => state.error !== null,
    isDarkTheme: (state) => state.theme === "dark",
  },

  actions: {
    setMapStyle(style: string) {
      this.mapStyle = style;
    },

    setMapPosition(position: MapPosition) {
      this.mapPosition = position;
    },

    getMapCenter(): MapPosition {
      return this.mapPosition;
    },

    setLoading(loading: boolean) {
      this.isLoading = loading;
    },

    setError(error: string | null) {
      this.error = error;
    },

    clearError() {
      this.error = null;
    },

    toggleSidebar() {
      this.sidebarOpen = !this.sidebarOpen;
    },

    setSidebarOpen(open: boolean) {
      this.sidebarOpen = open;
    },

    toggleTheme() {
      this.theme = this.theme === "light" ? "dark" : "light";
      this.applyTheme();
    },

    setTheme(theme: "light" | "dark") {
      this.theme = theme;
      this.applyTheme();
    },

    applyTheme() {
      const root = document.documentElement;
      if (this.theme === "dark") {
        root.classList.add("dark");
      } else {
        root.classList.remove("dark");
      }
    },

    async initialize() {
      try {
        this.setLoading(true);
        this.clearError();
        
        // Load saved preferences from localStorage
        const savedTheme = localStorage.getItem("theme") as "light" | "dark";
        if (savedTheme) {
          this.setTheme(savedTheme);
        }

        const savedMapPosition = localStorage.getItem("mapPosition");
        if (savedMapPosition) {
          try {
            const position = JSON.parse(savedMapPosition) as MapPosition;
            this.setMapPosition(position);
          } catch (e) {
            console.warn("Failed to parse saved map position:", e);
          }
        }

        this.setLoading(false);
      } catch (error) {
        this.setError(error instanceof Error ? error.message : "Failed to initialize UI");
        this.setLoading(false);
      }
    },
  },
});