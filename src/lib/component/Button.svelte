<script lang="ts">

    type ButtonTheme = 'BLUE' | 'GREEN' | 'YELLOW' | 'RED';

    const color_translation = {
        BLUE: ['bg-blue-600', 'hover:bg-blue-500'],
        GREEN: ['bg-green-500', 'hover:bg-green-400'],
        YELLOW: ['bg-amber-500', 'hover:bg-amber-400'],
        RED: ['bg-red-700', 'hover:bg-red-600'],
    };

    import {createEventDispatcher} from 'svelte';
    import {Icon, type IconSource} from "svelte-hero-icons";

    let clazz: string = '';
    export {clazz as class};

    export let color: ButtonTheme = 'BLUE';
    export let icon: IconSource | undefined = undefined;
    export let full: string = 'true';
    export let small: boolean = false;

    let padding: string = small ? "py-1.5" : "py-2.5";
    let iconSize: string = small ? "w-4" : "w-5";

    const dispatch = createEventDispatcher<{
        click: void;
    }>();
</script>

<button
        class="{clazz} {full === 'true' && 'w-full'} {color_translation[color][0]} flex items-center justify-center rounded-lg text-sm {padding} {color_translation[color][1]} transition"
        on:click={() => dispatch('click')}
        on:keypress={(e) => {
		    if (e.key === 'Enter') dispatch('click');
	    }}
>
    {#if icon}
        <Icon class="{iconSize} {small ? 'mr-1' : 'mr-2'}" src={icon} solid mini/>
    {/if}

    <slot>Button</slot>
</button>
