<script setup lang="ts">
import { ref, watch } from 'vue';
import type { Task } from '@moonrepo/types';
import { Command, type Child } from '@tauri-apps/api/shell';
import { useRepository } from '#app/stores/repository';
// import { parse, type ParsedSpan } from 'ansicolor';
import { Terminal } from '@xterm/xterm';
import { FitAddon } from '@xterm/addon-fit';
import { Unicode11Addon } from '@xterm/addon-unicode11';
import { onMounted } from 'vue';

interface Props {
	task: Task;
	cwd?: string;
}

const props = defineProps<Props>();
const repositoryStore = useRepository();

const childProcess = ref<Child | null>(null);
// const input = ref<string>('');
const output = ref<string>('');
const duration = ref<number>(0);
const durationInterval = ref<number | null>(null);
const status = ref<'idle' | 'running' | 'passed' | 'failed'>('idle');

const terminal = new Terminal();
terminal.options.allowProposedApi = true; // unicode
terminal.options.cursorStyle = 'block';
terminal.options.theme = {};

const fitAddon = new FitAddon();
const unicode11Addon = new Unicode11Addon();
terminal.loadAddon(unicode11Addon);
terminal.loadAddon(fitAddon);
terminal.unicode.activeVersion = '11';



// const FG_COLORS: Record<string, string> = {
// 	default: 'text-color',
// 	white: 'text-white',
// 	black: 'text-gray-400',
// 	red: 'text-red-600',
// 	green: 'text-green-600',
// 	yellow: 'text-yellow-600',
// 	blue: 'text-blue-600',
// 	magenta: 'text-pink-600',
// 	cyan: 'text-cyan-600',
// 	darkGray:  'text-gray-600',
// 	lightGray:  'text-gray-200',
// 	lightRed: 'text-red-300',
// 	lightGreen: 'text-green-300',
// 	lightYellow: 'text-yellow-300',
// 	lightBlue:  'text-blue-300',
// 	lightMagenta:  'text-pink-300',
// 	lightCyan: 'text-cyan-300',
// };

// const BG_COLORS: Record<string, string> = {
// 	bgDefault: '',
// 	bgWhite: 'bg-white',
// 	bgBlack: 'bg-gray-800',
// 	bgRed: 'bg-red-600',
// 	bgGreen: 'bg-green-600',
// 	bgYellow: 'bg-yellow-600',
// 	bgBlue: 'bg-blue-600',
// 	bgMagenta: 'bg-pink-600',
// 	bgCyan: 'bg-cyan-600',
// 	bgDarkGray: 'bg-gray-900',
// 	bgLightGray: 'bg-gray-700',
// 	bgLightRed: 'bg-red-400',
// 	bgLightGreen: 'bg-green-400',
// 	bgLightYellow: 'bg-yellow-400',
// 	bgLightBlue: 'bg-blue-400',
// 	bgLightMagenta: 'bg-pink-400',
// 	bgLightCyan: 'bg-cyan-400',
// };

// async function sendInput(event: KeyboardEvent) {
// 	console.log(event);

// 	if (childProcess.value) {
// 		await childProcess.value.write(input.value);
// 	}

// 	 input.value = '';
// }

// function colorizeOutput(span: ParsedSpan) {
// 	let classes: string[] = [];

// 	if (span.bold) {
// 		classes.push('text-bold')
// 	}

// 	if (span.italic) {
// 		classes.push('font-italic');
// 	}

// 	if (span.bgColor?.name && BG_COLORS[span.bgColor.name]) {
// 		classes.push(BG_COLORS[span.bgColor.name]);
// 	}

// 	if (span.color?.name && FG_COLORS[span.color.name]) {
// 		classes.push(FG_COLORS[span.color.name]);
// 	}

// 	return classes.join(' ');
// }

function updateOutput(line: string) {
	console.log(line);
	if (line === '') {
		return;
	} else if (line === '\n') {
		// output.value += '<br/>';
		terminal.write('\n');
		return;
	}

	terminal.writeln(line.trim());

	// for (const span of parse(line).spans) {
	// 	let text = span.text.replace('\n', '<br/>');

	// 	output.value += span.css
	// 		? `<span class="${colorizeOutput(span)}">${text}</span>`
	// 		: text;
	// }
}

function resetInterval() {
	if (durationInterval.value) {
		clearInterval(durationInterval.value);
	}
}

function resetState() {
	output.value = '';
	duration.value = 0;
	status.value = 'idle';
	terminal.clear();
	resetInterval();
}

async function runCommand() {
	resetState();

	let command = new Command(repositoryStore.packageManager, ['run', props.task.id], {
		cwd: props.cwd,
		env: {
			// 3 = 16m colors
			FORCE_COLOR: '3',
			CLICOLOR_FORCE: '3',
			COLORTERM: 'truecolor',
		},
	});

	command.on('close', result => {
		if (result.code === 0) {
			status.value = 'passed';
		} else {
			status.value = 'failed';
		}

		resetInterval();
	});

	command.on('error', error => {
		console.error(error);

		status.value = 'failed';

		resetInterval();
	});

	command.stdout.on('data', updateOutput);

	command.stderr.on('data', updateOutput);

	durationInterval.value = setInterval(() => {
		duration.value += 100;
	}, 100);

	setTimeout(() => {
		document.getElementById('terminal')?.scrollIntoView();
		terminal.focus();
	}, 100);

	status.value = 'running';

	childProcess.value = await command.spawn();
}

async function killCommand() {
	if (childProcess.value) {
		await childProcess.value.kill();

		status.value = 'failed';
		childProcess.value = null;

		resetInterval();
	}
}

function getElapsed() {
	return Math.max(duration.value / 1000, 0);
}

function getScript() {
	return (props.task.metadata as unknown as { originalScript: string }).originalScript;
}

onMounted(() => {
	terminal.onData(text => {
		// if (terminal.textarea) {
		// 	terminal.textarea.value += text;
		// }
		if (childProcess.value) {
			childProcess.value.write(text);
		}
	});
	terminal.open(document.getElementById('terminal')!);
	fitAddon.fit();
});

watch(() => props.task, runCommand, { immediate: true });
</script>

<template>
	<div>
		<div v-if="status === 'running'" style="float:right">
			<Button label="Stop" @click="killCommand" />
		</div>

		<h2>{{ task.id }}</h2>

		<p v-if="status === 'running'">Running <span class="text-primary">{{ getScript() }}</span> for {{ getElapsed() }}s</p>


		<p v-if="status === 'passed' || status === 'failed'">Ran <span class="text-primary">{{ getScript() }}</span> in {{ getElapsed() }}s</p>

		<!-- <div id="output-console" v-html="output" />

		<InputText class="w-full" size="small" type="text" v-model.trim="input" @keyup="sendInput" /> -->

		<div id="terminal" />
	</div>
</template>
