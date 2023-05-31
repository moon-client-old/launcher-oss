<script lang="ts">
	import { Notification, addNotification } from '$lib/notif/NotifHandler';
	import Button from '../../lib/component/Button.svelte';

	import SideBar from '../../lib/general/SideBar.svelte';

	let channels = [
		{
			name: 'Alpha',
			description:
				'The alpha channel containing the latest properly tested changes, updates are pushed here once they are ready.',
		},
	];
</script>

<SideBar />
<div class="launcher-container">
	<div class="flex flex-col gap-y-2 w-full">
		{#each channels as channel}
			<div
				class="flex items-center bg-slate-700/[0.25] border border-slate-50/[0.15] rounded-xl px-5 py-5"
			>
				<div>
					<h2 class="font-bold">{channel.name}</h2>
					<p class="text-sm text-slate-400">{channel.description}</p>
				</div>
				<div class="ml-auto pl-4">
					<Button
						class="w-24"
						full="false"
						text="Install"
						on:click={() => {
							addNotification(
								new Notification(
									'Not implemented',
									'This feature is not implemented yet.',
									'warning',
									10e3,
									true,
									[]
								)
							);
						}}
					/>
				</div>
			</div>
		{/each}
	</div>
</div>
