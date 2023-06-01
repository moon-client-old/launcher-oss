<script lang="ts">
    import Range from '$lib/component/Range.svelte';
    import SideBar from '../../../lib/general/SideBar.svelte';
    import Button from "$lib/component/Button.svelte";
    import {invoke} from "@tauri-apps/api/tauri";

    let memory = 2048;
    let maxMemory = 32768;
    invoke('get_max_available_memory').then(mem => maxMemory = mem / 1024 / 1.5);
</script>

<SideBar/>
<div class="launcher-container">
    <div class="items-center bg-slate-700/[0.25] border border-slate-50/[0.15] rounded-lg px-5 py-5 shadow-lg"
         style="backdrop-filter: blur(100px)">
        <h2 class="text-xl font-bold">Game settings</h2>
        <p class="text-sm text-slate-400">All minecraft / game related settings</p>

        <div class="mt-4">
            <p class="text-sm text-slate-300">Allocated memory (in-game)</p>
            <div class="flex items-center gap-x-3 w-full">
                <Range
                        bind:value={memory}
                        min={1024}
                        max={maxMemory}
                        step={128}
                        textbox="$input$MB"
                />
            </div>
            <Button class="mt-4">Save changes</Button>
        </div>
    </div>
</div>
