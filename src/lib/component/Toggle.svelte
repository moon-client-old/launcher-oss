<script lang="ts">
    import { createEventDispatcher } from "svelte";

    let clazz: string | undefined;
    export { clazz as class };

    export let text: string;
    export let default_state: string;

    const dispatch = createEventDispatcher();
    let state = default_state == "true";

    function on_click() {
        state = !state;
        dispatch("click", state);
    }
</script>

<label class="relative inline-flex items-center cursor-pointer { clazz }">
    <input type="checkbox" class="sr-only peer" bind:checked={ state } on:click={ on_click }>
    <div class="w-11 h-7 transition rounded-full bg-slate-800/[0.6] peer peer-checked:bg-blue-600 after:content-[''] after:absolute after:top-1 after:left-[5px] after:bg-gray-200 after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:after:drop-shadow peer-checked:after:translate-x-3.5"></div>
    <span class="ml-3 text-sm font-medium text-gray-900 dark:text-gray-300">{ text }</span>
</label>