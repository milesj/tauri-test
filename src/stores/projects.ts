import { defineStore } from "pinia";
import { shallowRef } from "vue";
import type { Project, Task, TaskType } from "@moonrepo/types";
import type { PackageJson } from "type-fest";

function parseArgs(line: string): string[] {
  let args: string[] = [];
  let quoteStack: string[] = [];
  let maxIndex = line.length - 1;
  let currentIndex = 0;
  let currentValue = "";

  while (currentIndex <= maxIndex) {
    let char = line[currentIndex];

    // Encountered a space
    if (char === " ") {
      // Not in a quote, so push a new argument
      if (quoteStack.length === 0) {
        args.push(currentValue);
        currentValue = "";
      }
      // In a quote, so append the current value
      else {
        currentValue += char;
      }
    }

    // Encountered a quote
    else if (char === '"' || char === "'") {
      currentValue += char;

      // If the quote is the same as the last quote, then we're closing the quote
      if (quoteStack[quoteStack.length - 1] === char) {
        quoteStack.pop();
      }
      // Otherwise, we're opening a quote
      else {
        quoteStack.push(char);
      }
    }

    // Encountered another value
    else {
      currentValue += char;
    }

    currentIndex += 1;
  }

  if (currentValue !== "") {
    args.push(currentValue);
  }

  return args;
}

const ENV_PREFIX = /([A-Z0-9_]+)=(.*?)/;

function createTaskFromScript(
  pkg: PackageJson,
  script: string,
  scriptCommand: string
): Task {
  let command = "";
  let args: string[] = [];
  let env: Record<string, string> = {};
  let type: TaskType = "build";
  let local = false;

  for (const arg of parseArgs(scriptCommand)) {
    // Env var
    if (ENV_PREFIX.test(arg)) {
      let [, key, value] = arg.match(ENV_PREFIX)!;
      env[key] = value;
      continue;
    }

    // Command bin
    if (command === "") {
      command = arg;
      continue;
    }

    // Argument
    if (arg === "-w" || arg === "--watch") {
      local = true;
      type = "run";
    }

    args.push(arg);
  }

  return {
    args,
    command,
    deps: [],
    env,
    id: script,
    inputs: ["**/*"],
    inputFiles: [],
    inputGlobs: [],
    inputVars: [],
    outputs: [],
    outputFiles: [],
    outputGlobs: [],
    platform: "node",
    target: `${pkg.name}:${script}`,
    type,
    // @ts-expect-error We don't need all fields
    options: {},
    // @ts-expect-error We don't need this
    metadata: {
      localOnly: local,
    },
  };
}

function createProjectFromPackage(pkg: PackageJson, pkgRoot: string): Project {
  return {
    alias: null,
    dependencies: [],
    fileGroups: {},
    id: pkg.name!,
    language: "typescript",
    platform: "node",
    root: pkgRoot.replace("package.json", ""),
    source: "",
    stack: "frontend",
    tasks: Object.fromEntries(
      Object.entries(pkg.scripts ?? {}).map(([script, commands]) => [
        script,
        createTaskFromScript(pkg, script, commands!),
      ])
    ),
    type: "library",
    config: {
      // @ts-expect-error We don't need all fields
      project: {
        description: pkg.description ?? "",
        name: pkg.name!,
        owner: typeof pkg.author === "string" ? pkg.author : null,
      },
    },
    // @ts-expect-error We don't need this
    inherited: {},
  };
}

export const useProjects = defineStore("projects", () => {
  const projects = shallowRef<Record<string, Project>>({});

  function setProjects(data: Record<string, Project>) {
    console.log("setProjects", data);

    projects.value = data;
  }

  function setProjectsFromPackages(data: Record<string, PackageJson>) {
    console.log("setProjectsFromPackages", data);

    projects.value = Object.fromEntries(
      Object.entries(data)
        .filter(([, pkg]) => pkg.name !== undefined)
        .map(([pkgPath, pkg]) => [
          pkg.name!,
          createProjectFromPackage(pkg, pkgPath),
        ])
    );
  }

  return { projects, setProjects, setProjectsFromPackages };
});
