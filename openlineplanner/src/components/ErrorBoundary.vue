<template>
  <div v-if="error" class="error-boundary">
    <h2>Something went wrong</h2>
    <p>{{ error.message }}</p>
    <button @click="resetError">Try again</button>
  </div>
  <slot v-else />
</template>

<script setup lang="ts">
import { ref, onErrorCaptured } from "vue";

interface ErrorState {
  message: string;
  stack?: string;
}

const error = ref<ErrorState | null>(null);

onErrorCaptured((err: Error, instance: any, info: string) => {
  console.error("Error captured by boundary:", err);
  console.error("Component:", instance);
  console.error("Info:", info);
  
  error.value = {
    message: err.message || "An unexpected error occurred",
    stack: err.stack,
  };
  
  // Return false to prevent the error from propagating
  return false;
});

const resetError = () => {
  error.value = null;
};
</script>

<style scoped>
.error-boundary {
  padding: 2rem;
  text-align: center;
  background-color: #fef2f2;
  border: 1px solid #fecaca;
  border-radius: 0.5rem;
  margin: 1rem;
}

.error-boundary h2 {
  color: #dc2626;
  margin-bottom: 1rem;
}

.error-boundary p {
  color: #7f1d1d;
  margin-bottom: 1rem;
}

.error-boundary button {
  background-color: #dc2626;
  color: white;
  border: none;
  padding: 0.5rem 1rem;
  border-radius: 0.25rem;
  cursor: pointer;
  transition: background-color 0.2s;
}

.error-boundary button:hover {
  background-color: #b91c1c;
}
</style>