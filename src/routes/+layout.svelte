<script lang="ts">
    import Fonts from '$lib/fonts/Fonts.svelte';
    import NotifHost from '$lib/notification/NotificationContainer.svelte';
    import {onMount} from 'svelte';
    import '../app.css';
    import {svelteMount} from '$lib/notification/NotificationHandler';

    onMount(async () => {
        svelteMount();
        while (document.getElementById('linux-font-moment') == null) {
            await new Promise((resolve) => setTimeout(resolve, 100));
        }
        const style = document.getElementById('linux-font-moment');
        if (
            navigator.userAgent.toLowerCase().includes('linux') &&
            style !== null
        )
            style.innerHTML = `/* monkey patch because webkitgtk is shit */
${[100, 200, 300, 400, 500, 600, 700, 800, 900]
                .map(
                    (weight) =>
                        `/*inter@${weight} (varweight patch)*/@font-face {font-family: 'Inter';font-style: normal;font-weight: ${weight};font-display: swap;src: url('https://fonts-cdn.nexuspipe.com/fonts/Inter-5323287c005292e89e320f96952a52f6f45e7d570baff1ae5ad41c9d38a76dd13838025ef07489d53a847b5f201b1abaf7f1ba55c385d684ed7bc3082926c7f5.woff2') format('woff2'), url('https://fonts-cdn.nexuspipe.com/fonts/Inter-5323287c005292e89e320f96952a52f6f45e7d570baff1ae5ad41c9d38a76dd13838025ef07489d53a847b5f201b1abaf7f1ba55c385d684ed7bc3082926c7f5.ttf') format('opentype');}`
                )
                .join('\n')}
`;
        else style?.remove();
    });
</script>

<svelte:head>
    <Fonts fonts={['Inter']} shouldPreload/>
    <style id="linux-font-moment"></style>
</svelte:head>

<slot/>

<NotifHost/>

<style lang="scss">
  $scrollbarBgTrack: #0f172a;
  $scrollbarFg: rgba(29, 78, 216, 0.8);
  $scrollbarBg: #1f2028;
  $scrollbarFgHover: rgba(29, 78, 216, 1);

  :global(*::-webkit-scrollbar) {
    width: 12px;
  }

  :global(*::-webkit-scrollbar-track) {
    background: $scrollbarBgTrack;
    border-radius: 8px;
    opacity: 0.3;
  }

  :global(*::-webkit-scrollbar-thumb) {
    background-color: $scrollbarFg;
    border-radius: 20px;
    border: 4px solid $scrollbarBgTrack;
    transition: background-color 0.2s;
  }

  :global(*::-webkit-scrollbar-thumb:hover) {
    background-color: $scrollbarFgHover;
  }
</style>
