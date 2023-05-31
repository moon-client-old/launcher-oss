<script lang="ts">
    import {get} from "svelte/store";

    import SideBar from "../../../lib/general/SideBar.svelte";
    import {UserContext, userContext} from "../../../stores";

    let context: UserContext = get(userContext);
    let date = new Date();
</script>

<SideBar/>
<div class="launcher-container">
    <div class="items-center bg-slate-700/[0.25] border border-slate-50/[0.15] rounded-lg px-5 py-5 shadow-lg"
         style="backdrop-filter: blur(100px)">
        <h2 class="text-xl font-bold mb-2">Moon Launcher</h2>
        <p class="text-sm text-slate-400">&copy; 2018 - { date.getFullYear() } Moon Client (Licensed under GPL-3.0)</p>
        <p class="text-sm text-slate-400">
            Licensed to <b class="font-bold">{ context.username }</b>
        </p>
        <div class="space-x-2 mt-1">
            <a class="text-blue-400 hover:text-blue-500 transition duration-100 text-sm" target="_blank"
               href="https://github.com/moon-client/launcher-oss">Source Code</a>
            <a class="text-blue-400 hover:text-blue-500 transition duration-100 text-sm" target="_blank"
               href="https://moonclient.xyz/terms">Terms
                of Service</a>
            <a class="text-blue-400 hover:text-blue-500 transition duration-100 text-sm" target="_blank"
               href="https://moonclient.xyz/privacy">Privacy Policy</a>
        </div>
    </div>
</div>