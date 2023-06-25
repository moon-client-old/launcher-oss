<script lang="ts">
    import {
        Notification,
        NotificationType,
        addNotification,
    } from '$lib/notification/NotificationHandler';
    import Button from '../../lib/component/Button.svelte';

    import SideBar from '../../lib/general/SideBar.svelte';
    import {fade} from "svelte/transition";
    import {UserContext, userContext} from '../../stores';
    import {get} from "svelte/store";

    let context: UserContext = get(userContext);
</script>

<SideBar/>
<div class="launcher-container" in:fade={{duration:500}}>
    <div class="w-full max-w-2xl ml-auto mr-auto">
        <div class="grid grid-cols-1 gap-5">
            {#each context.channels as channel}
                <div class="flex flex-col bg-slate-700/[0.25] border border-slate-50/[0.15] rounded-xl px-5 py-5">
                    <div>
                        <h2 class="font-bold text-xl mb-1">{channel.name}</h2>
                        <p class="text-sm text-slate-400">{channel.description}</p>
                    </div>
                    <div class="flex flex-row gap-x-2 pt-4 mt-auto">
                        <Button>Install</Button>
                        <Button>Changelog</Button>
                    </div>
                </div>
            {/each}
        </div>
    </div>
</div>
