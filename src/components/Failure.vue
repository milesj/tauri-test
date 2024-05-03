<script setup lang="ts">
interface Props {
	error?: Error | string | null;
}

const props = defineProps<Props>();

function message() {
	if (props.error instanceof Error) {
		return props.error.message;
	}

	return props.error || '';
}
</script>

<template>
	<Message v-if="props.error" :closable="false" severity="error">{{ message() }}</Message>
</template>
