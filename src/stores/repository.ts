import { defineStore } from "pinia";
import { ref } from "vue";

export const useRepository = defineStore("repository", () => {
  const path = ref<string | null>(null);

  function setPath(newPath: string) {
    path.value = newPath;
  }

  return { path, setPath };
});
