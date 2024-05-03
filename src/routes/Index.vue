<script setup lang="ts">
import * as dialog from '@tauri-apps/api/dialog';
import * as fs from '@tauri-apps/api/fs';
import * as path from '@tauri-apps/api/path';
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { useRepository } from '#app/stores/repository';

const router = useRouter();
const repository = useRepository();
const error = ref<Error | string | null>(null);

async function browse() {
	const homeDir = await path.homeDir(); // Has a trailing slash
	const selectedDir = await dialog.open({
		defaultPath: homeDir,
		directory: true,
		recursive: true, // Adds scope permissions to nested files
	});

	if (typeof selectedDir !== 'string' || !selectedDir) {
		throw new Error('Please open a directory to continue');
	}

	if (!(await fs.exists(`${selectedDir}/.git`, { dir: fs.Dir.Home }))) {
		throw new Error('Directory must be a Git repository (.git not found)');
	}

	repository.setPath(selectedDir);
	router.push('/dashboard');
}

function onBrowse() {
	browse().catch((cause) => {
		error.value = cause.message;
	});
}
</script>

<template>
	<div class="flex align-items-center justify-content-center h-screen w-full">
		<div class="text-center">
			<h2>Open a repository to get started!</h2>

			<Button @click="onBrowse">Browse</Button>

			<Failure v-if="error" class="mt-4" :error="error" />
		</div>
	</div>
</template>
