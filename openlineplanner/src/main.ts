import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./App.vue";
import router from "./router";
import VueMatomo from "vue-matomo";
import { useUIStore } from "./stores/ui";

// Import global styles
import "@fontsource/poppins";
import "@fontsource/poppins/400.css";
import "@fontsource/poppins/500.css";
import "@fontsource/poppins/600.css";
import "@fontsource/poppins/700.css";

// Error handling
const handleGlobalError = (error: Error, instance: any, info: string) => {
  console.error("Global error:", error);
  console.error("Component:", instance);
  console.error("Info:", info);
  
  // You could send this to an error reporting service
  // reportError(error, instance, info);
};

const app = createApp(App);

// Global error handler
app.config.errorHandler = handleGlobalError;

// Global properties
app.config.globalProperties.$appVersion = import.meta.env.VITE_APP_VERSION || "1.0.0";

const piniaStore = createPinia();
app.use(piniaStore);
app.use(router);

// Initialize UI store
const uiStore = useUIStore();

// Matomo tracking configuration
const matomoConfig = {
  host: "https://matomo.raildeals.org/",
  siteId: 5,
  router: router,
  enableLinkTracking: true,
  requireConsent: false,
  trackInitialView: true,
  disableCookies: true,
  preInitActions: [],
  // Add performance tracking
  enableHeartBeatTimer: true,
  heartBeatTimer: 30,
};

// Only enable tracking in production
if (import.meta.env.PROD) {
  app.use(VueMatomo, matomoConfig);
}

// Mount the app
app.mount("#app");

// Export for testing
export { app, piniaStore };