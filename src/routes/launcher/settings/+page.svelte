<script lang="ts">
    import SideBar from "../../../lib/general/SideBar.svelte";

    let memory = 2048;
</script>

<SideBar />
<div class="launcher-container">
    <div class="items-center bg-slate-700/[0.25] border border-slate-50/[0.15] rounded-xl px-5 py-5">
        <h2 class="text-xl font-bold">Settings</h2>
        <p class="text-slate-400 mb-3">Minecraft related settings:</p>

        <div>
            <h3>Memory allocated to Minecraft</h3>
            <div class="flex items-center gap-x-3 w-full">
                <input class="bg-blue-700 w-full h-2 rounded-lg appearance-none cursor-pointer" type="range" min="1024" max="8192" step="512" bind:value={ memory }/>
                <p class="text-slate-400 min-w-fit">{ memory } MB</p>
            </div>
        </div>
    </div>
</div>