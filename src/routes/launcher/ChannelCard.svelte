<script lang="ts">
    import {Play} from "@steeze-ui/heroicons";
    import {ArrowPath, CircleStack, Cog6Tooth, Icon, ListBullet} from "svelte-hero-icons";
    import IconButton from "$lib/component/IconButton.svelte";
    import Button from "$lib/component/Button.svelte";
    import type {Channel} from "../../stores";
    import {Version} from "../../stores";
    import {createEventDispatcher} from "svelte";
    import type {type Writable} from "svelte/store";
    import {get} from "svelte/store";

    class ChannelContext {
        // @ts-ignore
        version: Version
    }

    // Formats the release date of a version
    function formatReleaseDate(time: bigint): string {
        const date = new Date(Number(time));
        return formatTimeDifference(new Date(), date)
    }

    // Formats the difference between two given dates
    function formatTimeDifference(current: Date, previous: Date): string {
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

    export let channel: Channel;

    // Handle the writable context passed to the card
    export let writableContext: Writable<ChannelContext>;
    let context = get(writableContext);
    writableContext.subscribe(newContext => {
        context = newContext;
    });

    const dispatch = createEventDispatcher<{
        settings: void;
        changelog: void;
    }>();
</script>

<div class="flex flex-col bg-slate-700/[0.25] border border-slate-50/[0.15] rounded-xl px-5 py-5">
    <div>
        <h2 class="font-bold text-xl mb-2 text-white">{channel.name}</h2>
        <p style="font-size: 0.785rem" class="text-slate-400">{channel.description}</p>
    </div>
    <span class="border-b border-slate-50/[0.15] rounded-2xl mb-4 mt-0.5 pt-3.5"></span>
    <div class="flex flex-row gap-x-1 mt-1 justify-center items-center">
        <Button class="text-xs px-2.5 font-semibold" icon={Play} small={true} full={false}>
            Launch
        </Button>
        <p class="p-1 px-1.5 rounded-lg text-xs bg-slate-800/[0.5] border border-slate-200/[0.3] flex flex-row items-center justify-center ml-1 mr-1">
            <Icon class="w-4 mr-1.5 text-gray-300" src={ArrowPath} solid></Icon>
            <span>{formatReleaseDate(context.version.releasedAt)}</span>
        </p>
        <p class="p-1 px-1.5 rounded-lg text-xs bg-slate-800/[0.5] border border-slate-200/[0.3] items-center justify-center flex flex-row">
            <Icon class="w-4 mr-1.5 text-gray-300" src={CircleStack} solid></Icon>
            <span>{context.version.name}</span>
        </p>
        <div class="ml-auto flex justify-center items-center">
            <IconButton src={ListBullet} on:click={() => dispatch("changelog")}></IconButton>
            <span class="mr-0.5"></span>
            <IconButton src={Cog6Tooth} on:click={() => dispatch("settings")}></IconButton>
        </div>
    </div>
</div>