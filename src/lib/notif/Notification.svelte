<script lang="ts">
	import markdownIt from 'markdown-it/';
	import {
		callbacks,
		registerCallback,
		type Notification,
	} from './NotifHandler';
	import { onMount } from 'svelte';
	const md = markdownIt({
		breaks: true,
		linkify: true,
		typographer: true,
		html: false,
	});
	export let notification: Notification;
	let firstFramePassed = false;
	onMount(() => {
		setTimeout(() => {
			firstFramePassed = true;
		}, 0);
	});
	$: actions = [
		...(notification.actions ?? []),
		...(notification.dismissable
			? [{ name: 'Dismiss', callback: 'dismiss-notif' }]
			: []),
	];
	const generateFallbackCallback = (callback: string) => () => {
		console.warn(`Callback ${callback} not found`);
		return true;
	};
</script>

<div
	class="notif {notification.type} {actions.length > 2
		? 'btnbottom'
		: 'btnleft'}"
	style="flex-direction: {actions.length > 2 ? 'column' : 'row'}"
>
	<div class="notif-content">
		{#if notification.title}
			<div class="notif-title">{notification.title}</div>
		{/if}
		<div class="notif-body">
			{#if notification.message}
				{@html md.render(notification.message)}
			{/if}
		</div>
	</div>
	{#if actions.length > 0}
		<div class="notif-actions">
			{#each actions as action}
				<button
					class="notif-action"
					on:click={() => {
						if (action.callback) {
							const result = (
								callbacks[action.callback] ??
								generateFallbackCallback(action.callback)
							)(action, notification);
							if (result) notification.dismiss();
						}
					}}
				>
					{action.name}
				</button>
			{/each}
		</div>
	{/if}
</div>

<style lang="scss">
	@use 'sass:color';
	$errorAccent: #ff7777ff;
	$infoAccent: #77c9ff;
	$successAccent: #77ff77ff;
	$warningAccent: #ffff77ff;
	.notif {
		display: flex;
		background: var(--background, #fff2);
		backdrop-filter: blur(10px);
		border-radius: 0.5rem;
		width: 100%;
		border: 1px solid var(--accent);
		position: relative;
		z-index: 2;
		&.error {
			--accent: #{$errorAccent};
			--background: #{color.adjust($errorAccent, $alpha: -0.7)};
		}
		&.info {
			--accent: #{$infoAccent};
			--background: #{color.adjust($infoAccent, $alpha: -0.7)};
		}
		&.success {
			--accent: #{$successAccent};
			--background: #{color.adjust($successAccent, $alpha: -0.7)};
		}
		&.warning {
			--accent: #{$warningAccent};
			--background: #{color.adjust($warningAccent, $alpha: -0.7)};
		}
		&::before,
		&::after {
			content: '';
			display: block;
			height: 100%;
			width: 100%;
			position: absolute;
			top: 0;
			left: 0;
			background: var(--accent);
			opacity: 0.2;
			filter: blur(32px);
			pointer-events: none;
		}
		&::before {
			z-index: 1;
		}
		&::after {
			z-index: 3;
		}
		.notif-content {
			padding: 1rem;
			.notif-title {
				font-weight: 400;
				font-size: 1.2rem;
			}
			.notif-body {
				font-size: 0.9rem;
				margin-top: 0.5rem;
			}
		}
		&.btnleft {
			.notif-actions {
				margin-left: auto;
				display: flex;
				flex-direction: column;
				justify-content: center;
				> .notif-action {
					border-left: 1px solid #fff2;
					&:first-child {
						border-bottom: 1px solid #fff2;
						border-top-right-radius: 0.5rem;
					}
					&:last-child {
						border-bottom-right-radius: 0.5rem;
					}
				}
			}
		}
		&.btnbottom {
			flex-direction: column;
			.notif-content {
				padding: 1rem 1rem 1rem 1rem;
			}
			.notif-actions {
				display: flex;
				flex-direction: row;
				justify-content: center;
				> .notif-action {
					border-top: 1px solid #fff2;
					&:not(:first-child) {
						border-left: 1px solid #fff2;
					}
					&:first-child {
						border-bottom-left-radius: 0.5rem;
					}
					&:last-child {
						border-bottom-right-radius: 0.5rem;
					}
				}
			}
		}
		.notif-actions {
			> .notif-action {
				flex: 50%;
				display: flex;
				justify-content: center;
				align-items: center;
				padding: 0.5rem;
			}
		}
	}
</style>
