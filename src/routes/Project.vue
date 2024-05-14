<script setup lang="ts">
import { useProjects } from '#app/stores/projects';
import { onMounted, ref } from 'vue';
import { watch } from 'vue';
import { useRoute } from 'vue-router';
import type { Project, Task } from "@moonrepo/types";
import { listen } from '@tauri-apps/api/event';

const route = useRoute();
const projectsStore = useProjects();
const project = ref<Project | null>(null);
const activeTask = ref<Task | null>(null);

watch(() => route.params.id, (id) => {
	console.log(id, projectsStore.projects);
	project.value = projectsStore.projects[String(id)];
}, { immediate: true });

function getTaskCommand(task: Task): string {
	let line = task.command;

	if (task.args.length > 0) {
		line += ' ';
		line += task.args[0];
	}

	return line;
}

onMounted(() => {
	listen('hello', (event) => {
		console.log(event);
	})
});
</script>

<template>
	<ProjectsLayout>
		<div v-if="project">
			<h1 class="mt-0">{{ project.id }}</h1>

			<p>{{ project.config.project?.description }}</p>

			<DataTable :value="Object.values(project.tasks)" stripedRows>
				<Column field="id" header="Task" sortable></Column>
				<Column field="command" header="Command" sortable>
					<template #body="slotProps">
						{{ getTaskCommand(slotProps.data) }}
					</template>
				</Column>
				<Column field="platform" header="Platform" sortable></Column>
				<Column field="type" header="Type" sortable></Column>
				<Column>
					<template #body="slotProps">
						<Button label="Run" @click="activeTask = slotProps.data" />
					</template>
				</Column>
			</DataTable>

			<div v-if="activeTask" class="mt-6">
				<RunningTask :task="activeTask" :cwd="project.root" />
			</div>
		</div>
	</ProjectsLayout>
</template>
