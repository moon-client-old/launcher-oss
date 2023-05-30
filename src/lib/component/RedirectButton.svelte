<script lang="ts">
    import { page } from "$app/stores";
    import type { IconSource } from "@steeze-ui/heroicons/types";
    import { Icon } from "@steeze-ui/svelte-icon";

    let clazz: string = "";
    export { clazz as class };

    export let text: string = "Button";
    export let icon: IconSource | undefined;
    export let url: string;
</script>

<a href={ url } class="flex items-center { $page.route.id == url && "bg-slate-600/[0.5]"} rounded-lg text-sm text-center pl-4 py-2.5 hover:bg-slate-500 transition { clazz }" >
    {#if icon}
        <Icon class="w-6 h-full mr-2" src={ icon } />
    {/if}

    { text }
</a>
