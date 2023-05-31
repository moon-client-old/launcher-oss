<script lang="ts">
    import Fonts from "$lib/Fonts/Fonts.svelte";
    import "../app.css";
</script>

<svelte:head>
    <Fonts fonts={["Inter"]} shouldPreload />
</svelte:head>

<slot />
