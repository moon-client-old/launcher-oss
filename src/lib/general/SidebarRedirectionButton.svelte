<script lang="ts">
    import {page} from "$app/stores";
    import type {IconSource} from "@steeze-ui/heroicons/types";
    import {Icon} from "@steeze-ui/svelte-icon";

    let clazz: string = "";
    export {clazz as class};

    export let text: string = "Button";
    export let icon: IconSource | undefined;
    export let url: string;
    export let low: boolean;

    function generateClasses(url: string, low: boolean): string {
        let classList = ""
        const onSameUrl = url == $page.route.id;
        if (url == $page.route.id) {
            classList += "bg-slate-800/[0.7]"
        }
        console.log(low)
        if (!low || onSameUrl) {
            classList += " text-white"
        }
        return classList.trim()
    }
</script>

<a href={ url }
   class="flex items-center text-sm font-medium text-neutral-400 rounded-lg text-center px-3 py-2.5 hover:bg-slate-800/[0.4] hover:text-white transition { generateClasses(url, low) } { clazz }">
    {#if icon}
        <Icon style="width: 20px; height: 20px"
              class="h-full mr-2 text-neutral-400 { $page.route.id == url ? 'text-white' : '' } transition-all"
              src={ icon }/>
    {/if}
    { text }
</a>
