<script lang="ts">
    import OverviewSubPage from "./sub/OverviewSubPage.svelte";
    import GameSubPage from "./sub/GameSubPage.svelte";
    import ModsSubPage from "./sub/ModsSubPage.svelte";

    let selectedInnerPage = OverviewSubPage
</script>

<div id="inner-nav" class="w-full h-[10%] bg-surface rounded-lg flex flex-row justify-around items-center text-xl font-semibold">
    <button on:click={() => selectedInnerPage = OverviewSubPage}>Overview</button>
    <button on:click={() => selectedInnerPage = GameSubPage}>Game</button>
    <button on:click={() => selectedInnerPage = ModsSubPage}>Mods</button>
</div>
<div id="page" class="relative w-full h-[90%] rounded-lg">
    <svelte:component this={selectedInnerPage}/>
</div>