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

<div id="inner-nav" class="w-full h-[10%] bg-surface rounded-lg px-16">
    <div id="content" class="relative w-full h-full flex flex-row justify-between items-center text-xl font-semibold">
        <button class="w-1/5 h-full" on:click={() => selectPage("overview")}>Overview</button>
        <button class="w-1/5 h-full" on:click={() => selectPage("game")}>Game</button>
        <button class="w-1/5 h-full" on:click={() => selectPage("mods")}>Mods</button>

        <span id="glider" class="absolute bottom-0 rounded-t-lg h-1.5 w-1/5 bg-accent {selectedClass}"/>
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
        left: 40%;
    }
    .mods {
        left: 80%;
    }
</style>