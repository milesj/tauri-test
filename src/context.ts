import { InjectionKey, Ref } from "vue";

export const CTX_REPOSITORY = Symbol("repository") as InjectionKey<{
  path: Ref<string | null>;
  setPath: (path: string) => void;
}>;
