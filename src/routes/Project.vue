<script setup lang="ts">
import { useProjects } from '#app/stores/projects';
import { ref } from 'vue';
import { watch } from 'vue';
import { useRoute } from 'vue-router';
import type { Project, Task } from "@moonrepo/types";
import { useRepository } from '#app/stores/repository';
import { Command } from '@tauri-apps/api/shell';

const route = useRoute();
const repositoryStore = useRepository();
const projectsStore = useProjects();
const project = ref<Project | null>(null);
const activeTask = ref<Task | null>(null);
const taskOutput = ref<string>('');


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

async function runTaskCommand(task: Task) {
	activeTask.value = task;
	taskOutput.value = '';

	let command = new Command(repositoryStore.packageManager, ['run', task.id], {
		cwd: project.value?.root,
		env: {
			// 3 = 16m colors
			FORCE_COLOR: '3',
			CLICOLOR_FORCE: '3',
		},
	});
	command.stdout.on('data', line => {
		taskOutput.value += line;
	});
	command.stderr.on('data', line => {
		taskOutput.value += line;
	});

	setTimeout(() => {
		document.getElementById('active-task')?.scrollIntoView();
	}, 100);

	await command.spawn();
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
				<Column>
					<template #body="slotProps">
						<Button label="Run" @click="runTaskCommand(slotProps.data)" />
					</template>
				</Column>
			</DataTable>

			<div v-if="activeTask" class="mt-6" id="active-task">
				<h2>{{ activeTask.id }}</h2>

				<p>Running </p>

				<pre>{{ taskOutput }}</pre>
			</div>
		</div>
	</ProjectsLayout>
</template>
