<script setup lang="ts">
import { useRepository } from '#app/stores/repository';
import { invoke } from '@tauri-apps/api/tauri';
import { onMounted, ref } from 'vue'
import Failure from '../Failure.vue';

const repository = useRepository();
const loading = ref(true);
const error = ref<Error | string | null>(null);

onMounted(() => {
	invoke('load_projects', { repositoryPath: repository.path }).then((output) => {
		console.log(output);
	}).catch((e) => {
		error.value = e;
	}).finally(() => {
		loading.value = false;
	});
});


</script>

<template>
	<div class="grid">
		<aside class="col-4">
			<div class="p-4">
				<h3 class="mt-0">Projects</h3>

				<div v-if="loading">Loading...</div>

				<Failure v-if="error" :error="error" />
			</div>
		</aside>

		<main class="col-8">
			<div class="p-4">
				<slot />
			</div>
		</main>
	</div>
</template>
