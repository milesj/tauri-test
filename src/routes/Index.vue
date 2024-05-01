<script setup lang="ts">
import * as dialog from '@tauri-apps/api/dialog';
// import * as fs from '@tauri-apps/api/fs';
import * as path from '@tauri-apps/api/path';
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { useRepository } from '#app/stores/repository';

const router = useRouter();
const repository = useRepository();
const error = ref<string | null>(null);

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

	// We can't use absolute paths when checking for existence,
	// so update the path to be relative from the home directory
	// @see https://tauri.app/v1/api/js/fs#security
	const dir = selectedDir === homeDir || homeDir.startsWith(selectedDir)
		? '.'
		: selectedDir.replace(homeDir, '');

	// TODO fix this check!
	// if (!(await fs.exists(`${selectedDir}/.git`, { dir: fs.Dir.Home }))) {
	// 	throw new Error('Directory must be a Git repository (.git not found)');
	// }

	console.log(selectedDir, dir);

	repository.setPath(dir);
	router.push('/dashboard');
}

function onBrowse() {
	browse().catch((cause) => {
		error.value = String(cause);
	});
}
</script>

<template>
	<div class="flex align-items-center justify-content-center h-screen w-full">
		<div class="text-center">
			<h2>Open a repository to get started!</h2>

			<Button @click="onBrowse">Browse</Button>

			<Message v-if="error" :closable="false" class="mt-4" severity="error">{{ error }}</Message>
		</div>
	</div>
</template>
