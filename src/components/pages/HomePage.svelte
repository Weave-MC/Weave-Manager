<script lang="ts">
    import OverviewSubPage from "./sub/OverviewSubPage.svelte";
    import GameSubPage from "./sub/GameSubPage.svelte";
    import ModsSubPage from "./sub/ModsSubPage.svelte";

    let selectedInnerPage = OverviewSubPage
    let selectedClass = "overview"

    function selectPage(page: any) {
        selectedClass = page
        switch (page) {
            case "overview": {
                selectedInnerPage = OverviewSubPage
                break
            }
            case "game": {
                selectedInnerPage = GameSubPage
                break
            }
            case "mods": {
                selectedInnerPage = ModsSubPage
                break
            }
        }
    }
</script>

<div id="inner-nav" class="w-full h-12 bg-surface rounded-lg px-16">
    <div id="content" class="relative w-full h-full flex flex-row justify-between items-center text-xl font-semibold">
        <button class="w-1/4 h-full" on:click={() => selectPage("overview")}>Overview</button>
        <button class="w-1/4 h-full" on:click={() => selectPage("game")}>Game</button>
        <button class="w-1/4 h-full" on:click={() => selectPage("mods")}>Mods</button>

        <div id="glider" class="absolute bottom-0 w-1/4 {selectedClass} flex justify-center items-center">
            <span class="rounded-t-lg h-[0.25rem] w-28 bg-accent"/>
        </div>
    </div>
</div>
<div id="page" class="relative w-full h-[90%] rounded-lg">
    <svelte:component this={selectedInnerPage}/>
</div>

<style>
    #glider {
        transition: left 250ms;
    }
    .overview {
        left: 0;
    }
    .game {
        left: 37.5%;
    }
    .mods {
        left: 75%;
    }
</style>