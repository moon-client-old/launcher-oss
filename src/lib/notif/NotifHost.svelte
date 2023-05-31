<script lang="ts">
	import {
		addNotification,
		notificationStore,
		registerCallback,
		Notification as Notif,
	} from './NotifHandler';
	import Notification from './Notification.svelte';
	import autoAnimate from '@formkit/auto-animate';
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
	use:autoAnimate={{
		duration: 300,
	}}
>
	{#each $notificationStore as notif}
		<Notification notification={notif} />
	{/each}
</div>

<style lang="scss">
	.notifs {
		position: fixed;
		top: 0;
		right: 0;
		padding: 1rem;
		z-index: 1000;
		display: flex;
		flex-direction: column-reverse;
		align-items: flex-end;
		gap: 1rem;
		width: 500px;
	}
</style>
