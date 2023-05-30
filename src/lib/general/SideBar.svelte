<script lang="ts">
    import { goto } from "$app/navigation";
    import { RocketLaunch, Cog6Tooth, ArrowLeftOnRectangle } from "@steeze-ui/heroicons";
    
    import IconButton from "../../lib/component/IconButton.svelte";
    import RedirectButton from "../../lib/component/RedirectButton.svelte";
    import Button from "$lib/component/Button.svelte";

    function log_out() {
        goto("/");
    }
</script>

<div class="flex flex-col items-center bg-slate-700/[0.25] border border-slate-50/[0.15] min-w-fit w-3/12 h-full rounded-lg px-4 pt-6 pb-4">
    <img width="45px" src="https://moonclient.xyz/logo.png" class="mb-1 mr-1" alt="branding"/>
    <div class="flex flex-row items-center mb-6">
        <p class="text-lg text-gray-200"><b style='font-weight: 800'>Moon</b> Client</p>
    </div>

    <div class="flex flex-col gap-y-2  w-full">
        <RedirectButton icon={ RocketLaunch } text="Launch" url="/launcher" />
        <RedirectButton icon={ Cog6Tooth } text="Settings" url="/launcher/settings" />
    </div>

    <div class="flex w-full gap-2 items-center mt-auto">
        <p class="text-sm font-bold">Licensed to: </p>
        <p class="text-sm">oad (1554)</p>
        <div class="ml-auto">
            <IconButton src={ ArrowLeftOnRectangle } on:click={ log_out }/>
        </div>
    </div>
</div>