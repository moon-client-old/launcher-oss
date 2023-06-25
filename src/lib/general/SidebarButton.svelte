<script lang="ts">
	import type { IconSource } from 'svelte-hero-icons';
	import { Icon } from 'svelte-hero-icons';
	import { createEventDispatcher } from 'svelte';

	let clazz: string = '';
	export { clazz as class };

	export let text: string = 'Button';
	export let icon: IconSource | undefined;
	export let low: boolean;

	function generateClasses(low: boolean): string {
		let classList = '';
		console.log(low);
		if (!low) {
			classList += ' text-white';
		}
		return classList.trim();
	}

	const dispatch = createEventDispatcher();
</script>

<button
	class="flex items-center text-sm font-medium text-neutral-400 rounded-lg text-center px-3 py-2.5 hover:bg-slate-800/[0.4] hover:text-white transition {generateClasses(
		low
	)} {clazz}"
	on:click={() => dispatch('click')}
>
	{#if icon}
		<Icon
			style="width: 20px; height: 20px"
			class="h-full mr-2 text-neutral-400 transition-all"
			src={icon}
			solid
		/>
	{/if}
	<slot><em>{text}</em></slot>
</button>
