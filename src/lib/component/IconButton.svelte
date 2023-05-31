<script lang="ts">
    import { Icon } from "@steeze-ui/svelte-icon";
    import type { IconSource } from "@steeze-ui/heroicons/types";
    import { createEventDispatcher } from "svelte";

    let clazz: string = "";
    export { clazz as class };

    export let src: IconSource;

    const dispatch = createEventDispatcher();
</script>

<button on:click={ () => dispatch("click") }>
    <Icon class="w-6 cursor-pointer { clazz }" src={ src } />
</button>
