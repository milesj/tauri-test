import { defineStore } from "pinia";
import { ref } from "vue";
import type { NodePackageManager } from "@moonrepo/types";

export const useRepository = defineStore("repository", () => {
  const path = ref<string | null>(null);
  const packageManager = ref<NodePackageManager>("npm");

  function setPath(newPath: string) {
    console.log("setRepositoryPath", newPath);
    path.value = newPath;
  }

  function setPackageManager(pm: NodePackageManager) {
    console.log("setPackageManager", pm);
    packageManager.value = pm;
  }

  return { path, packageManager, setPath, setPackageManager };
});
