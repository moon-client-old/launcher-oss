<script>
    import {invoke} from "@tauri-apps/api/tauri"
    import Button from "../lib/Button.svelte";
    import Toggle from "../lib/Toggle.svelte";

    let uid;

    function sign_in() {
        // TODO: backend implementation
        invoke("login", {
            uid: uid
        });
    }

    function on_remember_update(new_state) {
        // TODO: backend implementation
        invoke("set_remember_state", {
            state: new_state
        });
    }
</script>

<main class="flex flex-col h-full justify-center items-center">
    <div class="flex flex-row items-center mb-6">
        <img width="45px" src="https://moonclient.xyz/logo.png" alt="branding"/>
        <p class="ml-4 text-lg text-gray-200"><b style='font-weight: 800'>Moon</b> Client</p>
    </div>

    <div class="absolute radial-pricing-bg w-full h-full" style="z-index: -100"></div>
    <div class="bg-slate-700/[0.5] border border-slate-50/[0.1] w-80 max-w-md rounded-lg px-4 py-4" style="backdrop-filter: blur(100px)">
        <h2 class="font-bold text-center text-2xl">Sign in</h2>
        <h2 class="text-center text-gray-300 text-xs">to the launcher</h2>
        <label class="flex flex-col mt-4 text-sm font-semibold">
            <span class="mb-1">UID</span>
            <input class="bg-slate-800/[0.6] border border-slate-50/[0.15] border rounded-lg mt-0.5 p-2.5 transition-colors outline-0 focus:border-blue-500 focus:ring-blue-500 block"
                   required bind:value={ uid }>
        </label>

        <Toggle text="Remember me" default_state="true" on:click={ on_remember_update }/>

        <Button text="Sign in" on:click={ sign_in }></Button>
    </div>
</main>

<style>
    .radial-pricing-bg {
        background: radial-gradient(550px 550px, rgba(28, 100, 242, 0.6), rgba(30, 41, 59, 1));
        filter: blur(50px);
    }
</style>