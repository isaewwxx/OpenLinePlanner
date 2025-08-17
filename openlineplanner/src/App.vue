<template>
  <div id="app">
    <ErrorBoundary>
      <WelcomeOverlay />
      <WelcomeTour />
      <PageHeader />
      <RouterView />
      <PageFooter />
    </ErrorBoundary>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from "vue";
import { RouterView } from "vue-router";
import WelcomeTour from "./components/WelcomeTour.vue";
import WelcomeOverlay from "./components/WelcomeOverlay.vue";
import PageHeader from "@/components/PageHeader.vue";
import PageFooter from "@/components/PageFooter.vue";
import ErrorBoundary from "@/components/ErrorBoundary.vue";
import { useUIStore } from "./stores/ui";

// Initialize stores
const uiStore = useUIStore();

// Initialize app on mount
onMounted(() => {
  try {
    uiStore.getMapCenter();
  } catch (error) {
    console.error("Failed to initialize map center:", error);
  }
});
</script>

<style lang="scss">
#app {
  height: 100%;
  display: flex;
  flex-direction: column;
  max-height: 100%;
  font-family: "Poppins", sans-serif;
}

// Global error styles
.error-boundary {
  padding: 2rem;
  text-align: center;
  background-color: #fef2f2;
  border: 1px solid #fecaca;
  border-radius: 0.5rem;
  margin: 1rem;
  
  h2 {
    color: #dc2626;
    margin-bottom: 1rem;
  }
  
  p {
    color: #7f1d1d;
    margin-bottom: 1rem;
  }
  
  button {
    background-color: #dc2626;
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 0.25rem;
    cursor: pointer;
    
    &:hover {
      background-color: #b91c1c;
    }
  }
}
</style>
