<script setup lang="ts">
import { ref } from "vue";
import { provide } from "vue";
import { RouterView } from "vue-router";
import { CTX_REPOSITORY } from './context';

// Relative from the home directory
const repositoryPath = ref<string | null>(null);

provide(CTX_REPOSITORY, {
  path: repositoryPath,
  setPath(path: string) {
    repositoryPath.value = path;
  },
});
</script>

<template>
  <div class="container">
    <RouterView />
  </div>
</template>
