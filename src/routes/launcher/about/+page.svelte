<script lang="ts">
    import { shell } from "@tauri-apps/api";
    
    import SideBar from "../../../lib/general/SideBar.svelte";

    let date = new Date();
</script>

<SideBar />
<div class="launcher-container">
    <div class="items-center bg-slate-700/[0.25] border border-slate-50/[0.15] rounded-xl px-5 py-5">
        <h2 class="text-lg font-bold">Moon Launcher</h2>
        <p class="text-sm text-slate-400">&copy; 2018 - { date.getFullYear() } Moon Client (Do not redistribute)</p>
        <p class="text-sm text-slate-400">
            Licensed to 
            <b class="font-bold">Lennox</b>
        </p>
        <div class="space-x-2 mt-4">
            <a class="text-blue-400 hover:text-blue-500 transition" target="_blank" href="https://moonclient.xyz/terms">Terms of Service</a>
            <a class="text-blue-400 hover:text-blue-500 transition" target="_blank" href="https://moonclient.xyz/privacy">Privacy Policy</a>
        </div>
    </div>
</div>