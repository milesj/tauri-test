<script setup lang="ts">
import { useRepository } from '#app/stores/repository';
import { invoke } from '@tauri-apps/api/tauri';
import { onMounted, ref } from 'vue'
import Failure from '../Failure.vue';
import { useProjects } from '#app/stores/projects';
import type { PackageJson } from "type-fest";
import { MenuItem } from 'primevue/menuitem';
import type { Project } from "@moonrepo/types";
import { useRouter } from 'vue-router';

const router = useRouter();
const repository = useRepository();
const projectsStore = useProjects();
const loading = ref(true);
const error = ref<Error | string | null>(null);

onMounted(() => {
	invoke<{
		root: string;
		rootPackage: PackageJson;
		packages: Record<string, PackageJson> | null,
		workspaces: string[] | null;
	}>('load_projects', { repositoryPath: repository.path }).then((result) => {
		if (result.packages) {
			projectsStore.setProjectsFromPackages(result.packages);
		} else {
			projectsStore.setProjectsFromPackages({
				[result.root]: result.rootPackage,
			});
		}
	}).catch((e) => {
		error.value = e;
	}).finally(() => {
		loading.value = false;
	});
});

function getMenuItems(): MenuItem[] {
	const items =  Object.values(projectsStore.projects).map((project: Project) => ({
		label: project.id,
		command() {
			console.log(`/project/${project.id}`);
			router.push(`/project/${project.id}`);
		},
	}));

	items.sort((a, d) => a.label.localeCompare(d.label));

	return items;
}
</script>

<template>
	<div class="grid">
		<aside class="col-4">
			<div class="p-4">
				<h3 class="mt-0">Projects</h3>

				<div v-if="loading" class="mt-4">Loading...</div>

				<div v-if="error" class="mt-4">
					<Failure :error="error" />
				</div>

				<div v-if="!loading && !error" class="mt-4">
					<Menu :model="getMenuItems()" />
				</div>
			</div>
		</aside>

		<main class="col-8">
			<div class="p-4">
				<slot />
			</div>
		</main>
	</div>
</template>
