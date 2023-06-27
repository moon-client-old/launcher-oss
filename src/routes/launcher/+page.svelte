<script lang="ts">
    import Button from '../../lib/component/Button.svelte';
    import SideBar from '../../lib/general/SideBar.svelte';
    import {fade} from "svelte/transition";
    import {Channel, UserContext, userContext, Version} from '../../stores';
    import {get, writable, type Writable} from "svelte/store";
    import {Check, ChevronUpDown, Icon} from "svelte-hero-icons";
    import {
        Dialog,
        DialogOverlay,
        DialogTitle,
        Listbox,
        ListboxButton,
        ListboxOption,
        ListboxOptions,
        Transition,
        TransitionChild,
    } from "@rgossiaux/svelte-headlessui";
    import ChannelCard from "./ChannelCard.svelte";
    import Toggle from "$lib/component/Toggle.svelte";
    import {invoke} from "@tauri-apps/api/tauri";

    let context: UserContext = get(userContext);

    // Settings dialog
    let settings = null;
    let selected = null;
    let settingsContext = null;

    // Changelog dialog
    let displayChangelog: boolean = false;
    let changelogChannel = null;

    let channelVersionMap: Map<Channel, Writable<ChannelContext>> = new Map<Channel, Writable<ChannelContext>>();

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

    // Holds information about a channels selected version
    export class ChannelContext {
        // @ts-ignore
        version: Version | undefined
        requiresLatest: boolean
    }

    // Holds information about a currently running setting edit session
    class SettingsContext {
        selectedId: string
        selectedVersion: Version
        channel: Channel
        requiresLatest: boolean = true

        from(channel: Channel, context: ChannelContext) {
            this.channel = channel;
            this.selectedId = context.version.id;
            this.requiresLatest = context.requiresLatest;
            // Find a suitable channel version
            channel.versions.forEach((version) => {
                if (this.selectedId === version.id) {
                    this.selectedVersion = version;
                }
            });
        }

        fromSpecific(channel: Channel, id: string, requiresLatest: boolean) {
            this.channel = channel;
            this.selectedId = id;
            this.requiresLatest = requiresLatest;
            // Find a suitable channel version
            channel.versions.forEach((version) => {
                if (this.selectedId === version.id) {
                    this.selectedVersion = version;
                }
            });
        }
    }

    // Finds the channel context of a channel
    async function findContextOf(channel: Channel): Promise<Writable<ChannelContext> | undefined> {
        let version = channelVersionMap.get(channel);
        // If version is null we set one first
        if (version == undefined) {
            await invoke('load_selection_settings_for', {channel: channel.name})
                .then(obj => {
                    let preferred = obj.preferred_version;
                    // We might want the latest version instead
                    if (obj.requires_latest) {
                        preferred = channel.latestVersion;
                    }
                    let version = channel.versions.find(version => {
                        return version.id == preferred;
                    });
                    const channelContext = new ChannelContext();
                    channelContext.version = version;
                    channelContext.requiresLatest = obj.requires_latest;
                    channelVersionMap.set(channel, writable(channelContext));
                });
        }
        return channelVersionMap.get(channel);
    }

    // Updates the selection based on the new selected value
    function computeNewContext(event: CustomEvent) {
        let oldRequiresLatest = settingsContext.requiresLatest;
        settingsContext = new SettingsContext();
        settingsContext.fromSpecific(selected, event.detail, oldRequiresLatest);
    }

    // Saves the current settings using the currently visible settings context
    async function saveCurrentSettings() {
        await invoke('save_selection_settings_for', {
            channel: settingsContext.channel.name,
            version: settingsContext.selectedVersion.id,
            alwaysLatest: settingsContext.requiresLatest
        });
        let context = await findContextOf(settingsContext.channel);
        // Update version if not undefined
        if (context != undefined) {
            context.update((ctx) => {
                ctx.version = settingsContext.selectedVersion;
                ctx.requiresLatest = settingsContext.requiresLatest;
                // Use latest if wanted
                if (settingsContext.requiresLatest) {
                    // Find the suitable channel version (latest)
                    settingsContext.channel.versions.forEach((version) => {
                        if (version.id === settingsContext.channel.latestVersion) {
                            ctx.version = version;
                        }
                    });
                }
                return ctx;
            });
        }
    }

</script>

<SideBar/>
<div class="launcher-container" in:fade={{duration:500}}>
    <div class="w-full max-w-2xl ml-auto mr-auto">
        <div class="flex flex-row items-center ml-1 mr-1 mt-1 mb-6">
            <div class="flex flex-col">
                <p class="text-2xl font-extrabold">Launch</p>
                <p class="text-xs text-gray-300">View and run all channels you have access to</p>
            </div>
            <Button class="ml-auto px-4 border border-blue-500 hover:border-blue-400" full={false}>Open Minecraft
                Directory
            </Button>
        </div>
        <div class="grid grid-cols-1 gap-5">
            {#each context.channels as channel}
                {#await findContextOf(channel)}
                {:then context}
                    <ChannelCard channel={channel} writableContext={context}
                                 on:settings={
                            function() {
                                // Handle opening of settings dialog
                                let channelContext = get(context);
                                settings = channel;
                                selected = channel;
                                settingsContext = new SettingsContext();
                                settingsContext.from(channel, channelContext);
                            }
                        }
                                 on:changelog={
                            function() {
                                // Handle opening of changelog dialog
                                displayChangelog = true;
                                changelogChannel = channel;
                            }
                        }
                    ></ChannelCard>
                {:catch error}
                    <p>{error}</p>
                {/await}
            {/each}
        </div>
    </div>
</div>

<!-- Settings Dialog -->
<Transition appear show={settings != null}>
    <Dialog
            as="div"
            class="fixed inset-0 z-10 overflow-y-auto w-full shadow-xl"
            on:close={() => settings = null}
    >
        <div class="min-h-screen px-4 text-center">
            <TransitionChild
                    enter="ease-out duration-300"
                    enterFrom="opacity-0"
                    enterTo="opacity-100"
                    leave="ease-in duration-200"
                    leaveFrom="opacity-100"
                    leaveTo="opacity-0"
            >
                <DialogOverlay class="fixed inset-0 bg-slate-900/[0.25]"/>
            </TransitionChild>

            <TransitionChild
                    enter="ease-out duration-200"
                    enterFrom="scale-95"
                    enterTo="scale-100"
                    leave="ease-in duration-100"
                    leaveFrom="scale-100"
                    leaveTo="scale-95"
            >
                <!-- This element is to trick the selected rendering engine (depends on os, webview 2 on windows for example) into centering the modal contents -->
                <span class="inline-block h-screen align-middle" aria-hidden="true">&#8203;</span>
                <div class="inline-block max-w-md p-6 my-8 text-left align-middle transition-all transform bg-slate-700/[0.25] border border-slate-50/[0.15] rounded-2xl shadow-xl"
                     style="min-width: 600px; backdrop-filter: blur(50px)">
                    <DialogTitle as="h3" class="text-2xl font-bold leading-6 text-white">
                        Settings
                    </DialogTitle>
                    <p class="text-white text-xs text-gray-400 mt-1">Configure channel specific properties
                        for {selected.name}</p>
                    <div class="flex flex-col gap-y-2.5 mt-2">
                        <Listbox class="z-50" bind:value={settingsContext.selectedId} on:change={computeNewContext}>
                            <div class="relative mt-1">
                                <ListboxButton
                                        class="w-full py-2 pl-3 pr-10 text-left bg-slate-900/[0.5] rounded-lg shadow-md cursor-default focus:outline-none focus-visible:ring-2 focus-visible:ring-opacity-75 focus-visible:ring-white focus-visible:ring-offset-orange-300 focus-visible:ring-offset-2 focus-visible:border-indigo-500 sm:text-sm">
                                    <span class="block text-gray-300 truncate">{settingsContext.selectedVersion.name}</span>
                                    <span class="absolute inset-y-0 right-0 flex items-center pr-2 pointer-events-none">
                                        <Icon class="w-5 h-5 text-gray-300" src={ChevronUpDown} aria-hidden="true"/>
                                    </span>
                                </ListboxButton>
                                <Transition
                                        leave="transition ease-in duration-100"
                                        enter="transition ease-in duration-100"
                                        enterFrom="opacity-0"
                                        enterTo="opacity-100"
                                        leaveFrom="opacity-100"
                                        leaveTo="opacity-0"
                                >
                                    <ListboxOptions
                                            class="absolute z-50 w-full py-1 mt-1 overflow-auto text-base bg-slate-900 rounded-md shadow-lg max-h-60 ring-1 ring-black ring-opacity-5 focus:outline-none sm:text-sm">
                                        {#each selected.versions as version}
                                            <ListboxOption
                                                    class={({ active }) =>`transition rounded-md cursor-default select-none relative py-2 pl-10 pr-4 ${active ? "text-blue-400 bg-blue-600/[0.2]" : "text-gray-300"}`}
                                                    value={version.id}
                                                    let:selected
                                            >
                                                <span class={`block truncate ${selected ? "font-medium" : "font-normal"}`}>{version.name}</span>
                                                {#if selected}
                                                    <span class="absolute inset-y-0 left-0 flex items-center pl-3 text-blue-400">
                                                        <Icon class="w-5 h-5" aria-hidden="true" src={Check}/>
                                                    </span>
                                                {/if}
                                            </ListboxOption>
                                        {/each}
                                    </ListboxOptions>
                                </Transition>
                            </div>
                        </Listbox>
                        <Toggle bind:checked={settingsContext.requiresLatest}>Always use latest version</Toggle>
                    </div>
                    <div class="mt-4 flex flex-row gap-x-2">
                        <Button class="px-2 text-xs" small={true} full={false} on:click={() => {
                            saveCurrentSettings();
                            settings = null;
                        }}>
                            Save
                        </Button>
                        <Button class="px-2 text-xs" small={true} full={false} on:click={() => settings = null}
                                color="RED">Close
                        </Button>
                    </div>
                </div>
            </TransitionChild>
        </div>
    </Dialog>
</Transition>

<!-- Changelog Dialog -->
<Transition appear show={displayChangelog}>
    <Dialog
            as="div"
            class="fixed inset-0 z-10 overflow-y-auto w-full shadow-xl"
            on:close={() => displayChangelog = false}
    >
        <div class="min-h-screen px-4 text-center">
            <TransitionChild
                    enter="ease-out duration-300"
                    enterFrom="opacity-0"
                    enterTo="opacity-100"
                    leave="ease-in duration-200"
                    leaveFrom="opacity-100"
                    leaveTo="opacity-0"
            >
                <DialogOverlay class="fixed inset-0 bg-slate-900/[0.25]"/>
            </TransitionChild>

            <TransitionChild
                    enter="ease-out duration-200"
                    enterFrom="scale-95"
                    enterTo="scale-100"
                    leave="ease-in duration-100"
                    leaveFrom="scale-100"
                    leaveTo="scale-95"
            >
                <!-- This element is to trick the selected rendering engine (depends on os, webview 2 on windows for example) into centering the modal contents -->
                <span class="inline-block h-screen align-middle" aria-hidden="true">&#8203;</span>
                <div class="inline-block max-w-md p-6 my-8 text-left align-middle transition-all transform bg-slate-700/[0.25] border border-slate-50/[0.15] rounded-2xl shadow-xl"
                     style="min-width: 600px; backdrop-filter: blur(50px)">
                    <DialogTitle as="h3" class="text-2xl font-bold leading-6 text-white">
                        Changelog
                    </DialogTitle>
                    <p class="text-white text-xs text-gray-400 mt-1">View all recent changes made to this channel</p>
                    <div class="flex flex-col overflow-y-scroll max-h-96">
                        {#each {length: changelogChannel.versions.length} as _, index}
                            {@const reverseIndex = changelogChannel.versions.length - 1 - index}
                            {@const version = changelogChannel.versions[reverseIndex]}

                            <span class="mt-2"></span>
                            <p class="font-bold text-white mb-0" style="font-size: 1.2rem">{version.name}</p>
                            <p class="text-xs text-gray-400">was released {formatReleaseDate(version.releasedAt)}</p>
                            <span class="mt-1"></span>
                            <p style="white-space: pre-line; font-size: 0.8rem" class="text-slate-300 max-h-screen">
                                {version.changelog}
                            </p>
                        {/each}
                    </div>
                    <div class="mt-4 flex flex-row gap-x-2">
                        <Button class="px-2 text-xs" small={true} full={false}
                                on:click={() => displayChangelog = false}>
                            Close
                        </Button>
                    </div>
                </div>
            </TransitionChild>
        </div>
    </Dialog>
</Transition>

