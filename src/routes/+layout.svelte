<script lang="ts">
	import Fonts from '$lib/Fonts/Fonts.svelte';
	import '../app.css';
</script>

<svelte:head>
	<Fonts fonts={['Inter']} shouldPreload />
</svelte:head>

<slot />

<style lang="scss">
	$scrollbarBgTrack: #1f2028;
	$scrollbarFg: rgba(29, 78, 216, 0.8);
	$scrollbarBg: #1f2028;
	$scrollbarFgHover: rgba(29, 78, 216, 1);

	:global(*::-webkit-scrollbar) {
		width: 12px;
	}

	:global(*::-webkit-scrollbar-track) {
		background: $scrollbarBgTrack;
		opacity: 0.3;
	}

	:global(*::-webkit-scrollbar-thumb) {
		background-color: $scrollbarFg;
		border-radius: 20px;
		border: 3px solid $scrollbarBg;
		transition: background-color 0.2s;
	}

	:global(*::-webkit-scrollbar-thumb:hover) {
		background-color: $scrollbarFgHover;
	}
</style>
