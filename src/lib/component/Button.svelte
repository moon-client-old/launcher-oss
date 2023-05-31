<script lang="ts">
	import { Icon } from '@steeze-ui/svelte-icon';
	import type { IconSource } from '@steeze-ui/heroicons/types';

	type ButtonTheme = 'BLUE' | 'GREEN' | 'YELLOW' | 'RED';

	const color_translation = {
		BLUE: ['bg-blue-600', 'hover:bg-blue-500'],
		GREEN: ['bg-green-500', 'hover:bg-green-400'],
		YELLOW: ['bg-amber-500', 'hover:bg-amber-400'],
		RED: ['bg-red-700', 'hover:bg-red-600'],
	};

	import { createEventDispatcher } from 'svelte';

	let clazz: string = '';
	export { clazz as class };

	export let color: ButtonTheme = 'BLUE';
	export let icon: IconSource | undefined = undefined;
	export let full: string = 'true';

	const dispatch = createEventDispatcher<{
		click: void;
	}>();
</script>

<button
	class="{clazz} {full == 'true' && 'w-full'} {color_translation[
		color
	][0]} rounded-lg text-sm py-2.5 {color_translation[color][1]} transition"
	on:click={() => dispatch('click')}
	on:keypress={(e) => {
		if (e.key == 'Enter') dispatch('click');
	}}
>
	{#if icon}
		<Icon class="w-6 h-full mr-2" src={icon} />
	{/if}

	<slot />
</button>
