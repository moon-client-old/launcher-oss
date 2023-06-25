<script lang="ts">
	import { goto } from '$app/navigation';
	import {
		ArrowLeftOnRectangle,
		ChevronUpDown,
		Cog6Tooth,
		GlobeAlt,
		InformationCircle,
	} from 'svelte-hero-icons';
	import {
		Popover,
		PopoverButton,
		PopoverPanel,
	} from '@rgossiaux/svelte-headlessui';
	import { fly } from 'svelte/transition';

	import { createPopperActions } from 'svelte-popperjs';
	import { Icon } from 'svelte-hero-icons';
	import SidebarRedirectionButton from '$lib/general/SidebarRedirectionButton.svelte';
	import SidebarButton from '$lib/general/SidebarButton.svelte';
	import { get } from 'svelte/store';
	import { UserContext, userContext } from '../../stores';

	let context: UserContext = get(userContext);

	const [popperRef, popperContent] = createPopperActions();

	// Example Popper configuration
	const popperOptions = {
		position: 'bottom-end',
		strategy: 'absolute',
	};

	function log_out() {
		// Empty the user context in both local storage and writable
		userContext.update((_) => new UserContext());
		localStorage.removeItem('userContextData');
		goto('/');
	}
</script>

<div
	class="fixed top-0 flex flex-col items-center bg-slate-700/[0.25] border-r border-slate-50/[0.15] min-w-fit w-60 h-full px-2 pt-3 pb-0 shadow-xl overflow-hidden main"
>
	<div class="flex self-start items-center justify-center mb-6 mt-1.5 ml-4">
		<img
			width="25px"
			src="https://moonclient.xyz/logo.png"
			alt="branding"
		/>
		<p class="ml-3 text-lg text-gray-200">
			<b class="weight-800">Moon</b> Client
		</p>
	</div>

	<div class="flex flex-col gap-y-2 w-full">
		<SidebarRedirectionButton icon={GlobeAlt} url="/launcher"
			>Launch</SidebarRedirectionButton
		>
	</div>

	<div class="w-full gap-2 items-center mt-auto">
		<SidebarRedirectionButton
			class="mb-1"
			icon={Cog6Tooth}
			url="/launcher/settings"
			low>Settings</SidebarRedirectionButton
		>
		<SidebarRedirectionButton
			class="mb-3"
			icon={InformationCircle}
			url="/launcher/about"
			low>About</SidebarRedirectionButton
		>
		<Popover class="relative mw-15rem" let:open>
			<PopoverButton class="inline-flex w-full" use={[popperRef]}>
				<div
					class="grow border-t border-slate-600/[0.9] flex items-center space-x-2 py-3 pr-2"
				>
					<div
						class="inline-flex items-center justify-center w-9 h-9 overflow-hidden rounded-full bg-slate-600"
					>
						<span
							class="font-medium text-gray-600 dark:text-gray-300"
							>{context.username
								.substring(0, 1)
								.toUpperCase()}</span
						>
					</div>
					<div class="flex-grow text-left dark:text-white">
						<div
							class="text-white font-semibold overflow-ellipsis overflow-hidden whitespace-nowrap text-sm mw-8rem"
						>
							{context.username}
						</div>
						<div
							class="text-gray-400 font-semibold overflow-ellipsis overflow-hidden whitespace-nowrap font-0_6rem"
						>
							{context.rank}
						</div>
					</div>
					<Icon class="shrink-0 w-5 h-5" src={ChevronUpDown} />
				</div>
			</PopoverButton>
			{#if open}
				<div transition:fly|local={{ y: 5, duration: 200 }}>
					<PopoverPanel
						static
						use={[[popperContent, popperOptions]]}
						class="bg-slate-700 border border-slate-50/[0.15] rounded-lg mb-96 w-full z-10 p-2"
					>
						<SidebarButton
							class="w-full"
							icon={ArrowLeftOnRectangle}
							on:click={log_out}
							low
						>
							Log Out
						</SidebarButton>
					</PopoverPanel>
				</div>
			{/if}
		</Popover>
	</div>
</div>

<style lang="scss">
	.main {
		backdrop-filter: blur(100px);
		.weight-800 {
			font-weight: 800;
		}
		.mw-15rem {
			max-width: 15rem;
		}
		.mw-8rem {
			max-width: 8rem;
		}
		.font-0_6rem {
			font-size: 0.6rem;
		}
	}
</style>
