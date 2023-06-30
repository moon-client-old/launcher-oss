<script lang="ts">
    import Range from '$lib/component/Range.svelte';
    import SideBar from '../../../lib/general/SideBar.svelte';
    import Button from "$lib/component/Button.svelte";
    import {invoke} from "@tauri-apps/api/tauri";
    import {addNotification, Notification, NotificationType} from "$lib/notification/NotificationHandler";
    import {fade} from "svelte/transition";

    const unknownMessage =
        'An unknown error occurred, please create an issue on GitHub';

    let memory = 2048;
    let maxMemory = 32768;
    invoke('get_max_available_memory').then(mem => maxMemory = mem / 1024 / 1.5);
    invoke('load_game_settings')
        .then(settings => memory = settings.memory)
        .catch(err => {
            let errorMessage = null;
            // Possibly find the error message in the error json
            for (const error in err) {
                errorMessage = err[error].message;
            }
            // If there is no message an unknown error occurred
            if (errorMessage === null || errorMessage === undefined) {
                errorMessage = unknownMessage;
            }
            addNotification(
                new Notification(
                    'Game Settings',
                    errorMessage,
                    NotificationType.Err,
                    5000,
                    true,
                    [
                        {
                            name: 'Copy',
                            callback: 'copy-text',
                            metadata:
                                errorMessage === unknownMessage
                                    ? `${err}`
                                    : errorMessage,
                        },
                    ]
                )
            );
        });

    function save_settings() {
        invoke('save_game_settings', {memory: memory})
            .then(() => {
                addNotification(
                    new Notification(
                        'Game Settings',
                        'Successfully saved your game settings',
                        NotificationType.Ok,
                        3e3
                    )
                );
            })
            .catch(err => {
                let errorMessage = null;
                // Possibly find the error message in the error json
                for (const error in err) {
                    errorMessage = err[error].message;
                }
                // If there is no message an unknown error occurred
                if (errorMessage === null || errorMessage === undefined) {
                    errorMessage = unknownMessage;
                }
                addNotification(
                    new Notification(
                        'Game Settings',
                        errorMessage,
                        NotificationType.Err,
                        5000,
                        true,
                        [
                            {
                                name: 'Copy',
                                callback: 'copy-text',
                                metadata:
                                    errorMessage === unknownMessage
                                        ? `${err}`
                                        : errorMessage,
                            },
                        ]
                    )
                );
            })
    }

    // Opens the settings directory
    function invokeFolderOpen() {
        invoke('open_directory_type', {"directory": "Settings"})
    }
</script>

<SideBar/>
<div class="launcher-container" in:fade={{duration:500}}>
    <div class="flex flex-row items-center ml-1 mr-1 mt-1 mb-6">
        <div class="flex flex-col">
            <p class="text-2xl font-extrabold">Settings</p>
            <p class="text-xs text-gray-300">Configure all types of properties which are used to launch your game</p>
        </div>
        <Button class="ml-auto px-4 border border-blue-500 hover:border-blue-400" full={false}
                on:click={invokeFolderOpen}>Open Launcher
            Directory
        </Button>
    </div>
    <div class="items-center bg-slate-700/[0.25] border border-slate-50/[0.15] rounded-lg px-5 py-5 shadow-lg"
         style="backdrop-filter: blur(100px)">
        <h2 class="text-xl font-bold">Game settings</h2>
        <p class="text-sm text-slate-400">All minecraft / game related settings</p>

        <div class="mt-4">
            <p class="text-sm text-slate-300">Allocated memory (in-game)</p>
            <div class="flex items-center gap-x-3 w-full">
                <Range
                        bind:value={memory}
                        min={1024}
                        max={maxMemory}
                        step={128}
                        textbox="$input$MB"
                />
            </div>
            <Button class="mt-4" on:click={save_settings}>Save changes</Button>
        </div>
    </div>
</div>
