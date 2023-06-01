<script lang="ts">
    import {invoke} from '@tauri-apps/api/tauri';
    import {goto} from '$app/navigation';

    import Button from '../lib/component/Button.svelte';
    import Toggle from '../lib/component/Toggle.svelte';
    import {userContext, UserContext} from '../stores';
    import {addNotification, Notification, NotificationType,} from '$lib/notification/NotificationHandler';
    import {fade} from "svelte/transition";

    let uid: string = '';
    let lastUid: string = '';
    let rememberMe = true;

    // force number
    $: {
        const _uid = uid.trim();
        if (_uid === '' || _uid === '0x' || _uid === '0b') {
            lastUid = uid;
        } else if (isNaN(Number(_uid))) {
            uid = lastUid;
        } else {
            lastUid = uid;
        }
    }
    const unknownMessage =
        'An unknown error occurred, please create an issue on GitHub';

    // Load the login settings on startup
    invoke('load_login_settings').then(data => {
        // Only load uid if it's not custom
        if (data.uid != -1) {
            uid = lastUid = data.uid.toString();
        }
        rememberMe = data.remember_me;
        // Automatically log-in when remember me is selected
        if (rememberMe) {
            sign_in();
        }
    });

    const sign_in = async () => {
        uid = Number(uid.trim()).toString(); // convert to decimal
        if (uid === 'NaN')
            return addNotification(
                new Notification(
                    'Invalid UID',
                    'Please enter a valid UID',
                    NotificationType.Err,
                    3e3
                )
            );
        await invoke('login', {
            uid: uid,
            rememberMe: rememberMe
        })
            .then(async (response) => {
                // get user data
                let userData = new UserContext();
                userData.serialize(response as object);
                userContext.update((_) => userData);
                localStorage.setItem(
                    'userContextData',
                    JSON.stringify(response)
                );
                addNotification(
                    new Notification(
                        'Logged in!',
                        'Logged in successfully',
                        NotificationType.Ok,
                        3e3
                    )
                );
                goto('/launcher');
            })
            .catch((err) => {
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
                        'Login Error',
                        errorMessage,
                        NotificationType.Err,
                        100000,
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
    };
</script>

<div in:fade|local={{duration: 300}} class="flex flex-col h-full justify-center items-center">
    <img
            width="45px"
            src="https://moonclient.xyz/logo.png"
            class="mb-1 mr-1"
            alt="branding"
    />
    <div class="flex flex-row items-center mb-6">
        <p class="text-lg text-gray-200">
            <b style="font-weight: 800">Moon</b> Client
        </p>
    </div>

    <div
            class="bg-slate-700/[0.25] border border-slate-50/[0.15] w-80 max-w-md rounded-lg px-4 py-4"
            style="backdrop-filter: blur(100px)"
    >
        <h2 class="font-bold text-center text-2xl">Sign in</h2>
        <h2 class="text-center text-gray-300 text-xs">to the launcher</h2>

        <div class="space-y-2">
            <label class="flex flex-col mt-4 text-sm font-semibold">
                <span class="mb-1">UID</span>
                <input
                        class="bg-slate-800/[0.6] border-slate-50/[0.15] border rounded-lg mt-0.5 p-2.5 transition-colors outline-0 focus:border-blue-500 focus:ring-blue-500 block"
                        required
                        bind:value={uid}
                />
            </label>

            <Toggle bind:checked={rememberMe}>Remember me</Toggle>

            <Button on:click={sign_in}>Sign in</Button>
        </div>
    </div>
</div>
