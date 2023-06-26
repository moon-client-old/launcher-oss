<script lang="ts">
    import Button from '../../lib/component/Button.svelte';

    import SideBar from '../../lib/general/SideBar.svelte';
    import {fade} from "svelte/transition";
    import {Channel, UserContext, userContext, Version} from '../../stores';
    import {get} from "svelte/store";
    import {Play} from "@steeze-ui/heroicons";
    import IconButton from "$lib/component/IconButton.svelte";
    import {
        ArrowPath,
        Bookmark,
        Check,
        ChevronUpDown,
        CircleStack,
        Cog6Tooth,
        Icon,
        ListBullet
    } from "svelte-hero-icons";
    import {
        Dialog,
        DialogOverlay,
        DialogTitle,
        DialogDescription, Transition, TransitionChild,
        Listbox,
        ListboxButton,
        ListboxOptions,
        ListboxOption,
    } from "@rgossiaux/svelte-headlessui";
    import ChannelCard from "./ChannelCard.svelte";
    import Toggle from "$lib/component/Toggle.svelte";

    const people = [
        {id: 1, name: "Durward Reynolds", unavailable: false},
        {id: 2, name: "Kenton Towne", unavailable: false},
        {id: 3, name: "Therese Wunsch", unavailable: false},
        {id: 4, name: "Benedict Kessler", unavailable: true},
        {id: 5, name: "Katelyn Rohan", unavailable: false},
    ];
    let selectedPerson = people[0];

    let isOpen = true;
    let settings = null;
    let selected = null;
    let settingsContext = null;
    let selectedVersionId = null;
    let changelog = null;

    class SettingsContext {
        selectedId: string
        selectedVersion: Version

        from(channel: Channel) {
            selectedVersionId = channel.latestVersion;
            this.selectedId = channel.latestVersion;
            channel.versions.forEach((version) => {
                if (this.selectedId == version.id) {
                    this.selectedVersion = version;
                }
            });
        }

        fromSpecific(channel: Channel, id: string) {
            selectedVersionId = id;
            this.selectedId = id;
            channel.versions.forEach((version) => {
                if (this.selectedId == version.id) {
                    this.selectedVersion = version;
                }
            });
        }
    }

    function computeNewContext(event: CustomEvent) {
        settingsContext = new SettingsContext();
        settingsContext.fromSpecific(selected, event.detail);
    }

    let context: UserContext = get(userContext);

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
                <ChannelCard channel={channel} on:settings={function() {
                  settings = channel;
                  selected = channel;
                  settingsContext = new SettingsContext();
                  settingsContext.from(channel);
                }}></ChannelCard>
            {/each}
        </div>
    </div>


</div>
<Transition appear show={settings != null}>
    <Dialog
            as="div"
            class="fixed inset-0 z-10 overflow-y-auto w-full"
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
                    <p class="text-white text-xs text-gray-400 mt-1">Configure channel specific properties for {selected.name}</p>
                    <div class="flex flex-col gap-y-2.5 mt-2">
                        <Listbox class="z-50" bind:value={selectedVersionId} on:change={computeNewContext}>
                            <div class="relative mt-1">
                                <ListboxButton
                                        class="w-full py-2 pl-3 pr-10 text-left bg-slate-900/[0.5] rounded-lg shadow-md cursor-default focus:outline-none focus-visible:ring-2 focus-visible:ring-opacity-75 focus-visible:ring-white focus-visible:ring-offset-orange-300 focus-visible:ring-offset-2 focus-visible:border-indigo-500 sm:text-sm">
                                    <span class="block text-gray-300 truncate">{settingsContext.selectedVersion.name}</span>
                                    <span class="absolute inset-y-0 right-0 flex items-center pr-2 pointer-events-none">
                                        <Icon class="w-5 h-5 text-gray-300" src={ChevronUpDown} aria-hidden="true" />
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
                                                    class={({ active }) =>`cursor-default select-none relative py-2 pl-10 pr-4 ${active ? "text-blue-400 bg-blue-600/[0.2]" : "text-gray-300"}`}
                                                    value={version.id}
                                                    let:selected
                                            >
                                                <span class={`block truncate ${selected ? "font-medium" : "font-normal"}`}>{version.name}</span>
                                                {#if selected}
                                                    <span class="absolute inset-y-0 left-0 flex items-center pl-3 text-blue-400">
                                                        <Icon class="w-5 h-5" aria-hidden="true" src={Check} />
                                                    </span>
                                                {/if}
                                            </ListboxOption>
                                        {/each}
                                    </ListboxOptions>
                                </Transition>
                            </div>
                        </Listbox>
                        <Toggle checked={true}>Always use latest version</Toggle>
                    </div>
                    <div class="mt-4 flex flex-row gap-x-2">
                        <Button class="px-2 text-xs" small={true} full={false} on:click={() => settings = null}>Save
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
