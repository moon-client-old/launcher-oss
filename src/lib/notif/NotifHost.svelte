<script lang="ts">
    import {
        addNotification,
        notificationStore,
        registerCallback,
        Notification as Notif,
    } from './NotifHandler';
    import Notification from './Notification.svelte';
    import {flip} from "svelte/animate";
    import {fade, fly} from "svelte/transition";
    // global utility callbacks
    registerCallback('dismiss-notif', () => true);
    registerCallback('copy-text', (action, notif) => {
        if (navigator.clipboard) {
            navigator.clipboard.writeText(notif.message);
            addNotification(
                new Notif('Copied!', 'Copied to Clipboard', 'success')
            );
        } else
            alert(`Your browser does not support copying to clipboard.
Message: ${notif.message}`);
        return true;
    });
</script>

<div
        class="notifs"
>
    {#each $notificationStore as notif}
        <div in:fade={{duration: 200}} out:fly={{x:100, duration: 300}}>
            <Notification notification={notif}/>
        </div>
    {/each}
</div>

<style lang="scss">
  .notifs {
    position: fixed;
    bottom: 0;
    right: 0;
    padding: 1rem;
    z-index: 1000;
    display: flex;
    flex-direction: column-reverse;
    align-items: flex-end;
    gap: 1rem;
    width: 350px;
  }
</style>
