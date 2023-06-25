<script lang="ts">
    import { Icon } from "svelte-hero-icons";
    import type { IconSource } from "svelte-hero-icons";
    import { createEventDispatcher } from "svelte";

    let clazz: string = "";
    export { clazz as class };

    export let src: IconSource;

    const dispatch = createEventDispatcher();
</script>

<button class="hover:bg-slate-900/[0.5] rounded-full p-1 transition" on:click={ () => dispatch("click") }>
    <Icon class="w-5 cursor-pointer { clazz }" src={ src } solid />
</button>
