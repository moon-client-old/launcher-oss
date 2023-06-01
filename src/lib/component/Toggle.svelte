<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	let clazz: string = '';
	export { clazz as class };

	const dispatch = createEventDispatcher();
	export let checked: boolean = false;

	const on_click = () => {
		checked = !checked;
		dispatch('click', checked);
	};
</script>

<label class="relative inline-flex items-center cursor-pointer {clazz}">
	<input
		type="checkbox"
		class="sr-only peer"
		bind:checked
		on:click={on_click}
	/>
	<div
		class="w-11 h-7 transition rounded-full bg-slate-800/[0.6] peer peer-checked:bg-blue-600 after:content-[''] after:absolute after:top-1 after:left-[5px] after:bg-gray-200 after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:after:drop-shadow peer-checked:after:translate-x-3.5"
	/>
	<span class="ml-3 text-sm font-medium text-gray-300 dark:text-gray-300"
		><slot>Toggle</slot></span
	>
</label>
