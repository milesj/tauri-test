<script setup lang="ts">
import { useProjects } from '#app/stores/projects';
import { ref } from 'vue';
import { watch } from 'vue';
import { useRoute } from 'vue-router';
import type { Project, Task } from "@moonrepo/types";

const route = useRoute();
const projectsStore = useProjects();
const project = ref<Project | null>(null);

watch(() => route.params.name, (name) => {
	project.value = projectsStore.projects[String(name)];
}, { immediate: true });

function getTaskCommand(task: Task): string {
	let line = task.command;

	if (task.args.length > 0) {
		line += ' ';
		line += task.args[0];
	}

	return line;
}
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
			</DataTable>
		</div>
	</ProjectsLayout>
</template>
