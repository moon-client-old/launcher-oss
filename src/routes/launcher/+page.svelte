<script lang="ts">
    import Button from '../../lib/component/Button.svelte';

    import SideBar from '../../lib/general/SideBar.svelte';
    import {fade} from "svelte/transition";
    import {UserContext, userContext} from '../../stores';
    import {get} from "svelte/store";
    import {Play} from "@steeze-ui/heroicons";
    import IconButton from "$lib/component/IconButton.svelte";
    import {ArrowPath, CircleStack, Cog6Tooth, Icon, ListBullet} from "svelte-hero-icons";

    let context: UserContext = get(userContext);

    function date(time: bigint): string {
        const date = new Date(Number(time));
        return timeDifference(new Date(), date)
    }

    function timeDifference(current: Date, previous: Date): string {
        const msPerMinute: number = 60 * 1000;
        const msPerHour: number = msPerMinute * 60;
        const msPerDay: number = msPerHour * 24;
        const msPerMonth: number = msPerDay * 30;
        const msPerYear: number = msPerDay * 365;
        const elapsed: number = current - previous;

        if (elapsed < msPerMinute) {
            const seconds: number = Math.round(elapsed / 1000);
            return seconds + 's ago';
        } else if (elapsed < msPerHour) {
            const minutes: number = Math.round(elapsed / msPerMinute);
            return minutes + 'min ago';
        } else if (elapsed < msPerDay) {
            const hours: number = Math.round(elapsed / msPerHour);
            return hours + 'h ago';
        } else if (elapsed < msPerMonth) {
            const days: number = Math.round(elapsed / msPerDay);
            return days + 'd ago';
        } else if (elapsed < msPerYear) {
            const months: number = Math.round(elapsed / msPerMonth);
            return months + 'mo ago';
        } else {
            const years: number = Math.round(elapsed / msPerYear);
            return years + 'y ago';
        }
    }

</script>

<SideBar/>
<div class="launcher-container" in:fade={{duration:500}}>
    <div class="w-full max-w-2xl ml-auto mr-auto">
        <div class="ml-1 mt-1 mb-6">
            <p class="text-2xl font-extrabold">Launch</p>
            <p class="text-xs text-gray-300">View and run all channels you have access to</p>
        </div>
        <div class="grid grid-cols-1 gap-5">
            {#each context.channels as channel}
                <div class="flex flex-col bg-slate-700/[0.25] border border-slate-50/[0.15] rounded-xl px-5 py-5">
                    <div>
                        <h2 class="font-bold text-xl mb-1.5 text-white">{channel.name}</h2>
                        <p style="font-size: 0.8rem" class="text-gray-400">{channel.description}</p>
                    </div>
                    <div class="flex flex-row gap-x-1 pt-6 justify-center items-center">
                        <Button class="text-xs px-2.5" icon={Play} small={true} full={false}>Launch</Button>
                        <div class="ml-auto flex justify-center items-center">
                            <p class="p-1 rounded-lg text-xs bg-slate-800/[0.5] flex flex-row mr-1">
                                <Icon class="w-4 mr-1.5" src={ArrowPath} solid></Icon>
                                <span>{date(channel.lastUpdated)}</span>
                            </p>
                            <p class="p-1 rounded-lg text-xs bg-slate-800/[0.5] flex flex-row mr-3">
                                <Icon class="w-4 mr-1.5" src={CircleStack} solid></Icon>
                                <span>{channel.latestVersion}</span>
                            </p>
                            <IconButton class="mr-0.5" src={ListBullet}></IconButton>
                            <IconButton src={Cog6Tooth}></IconButton>
                        </div>
                        <!--                        <Button class="text-xs px-2.5" small={true} full={false}>Changelog</Button>-->
                    </div>
                </div>
            {/each}
        </div>
    </div>
</div>
