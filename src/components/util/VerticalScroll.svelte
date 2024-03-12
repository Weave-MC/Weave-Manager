<script context="module">
    import {writable} from "svelte/store"
    export const scroll = writable(null)
</script>
<script lang="ts">
    // this is hardcoded to only be maxed at 2
    import {afterUpdate, onMount} from "svelte";
    import {checkVerticalOverflow} from "../../scripts/components";

    export let columns: number = 1
    export let items: any[]

    let container: HTMLElement
    let overflowed: boolean = false

    onMount(() => {
        overflowed = checkVerticalOverflow(container)
    })

    afterUpdate(() => {
        overflowed = checkVerticalOverflow(container)
    })
</script>

<div class="absolute bg-crust rounded-lg top-10 bottom-2 left-2 right-2 py-2 pl-2 pr-1">
    <div bind:this={container} class="relative w-full h-full overflow-y-scroll gap-2 flex" class:pr-1={overflowed} class:scroll-single={columns === 1} class:scroll-multi={columns > 1} on:scroll={(e) => $scroll = e}>
        {#each items as item}
            <!-- A "hack" to force styles on the scroll list entries -->
            <div class:scroll-1-element={columns === 1} class:scroll-2-element={columns > 1}>
                <slot prop={item}/>
            </div>
        {/each}
    </div>
</div>

<style>
    .scroll-single {
        @apply flex-col;
    }
    .scroll-multi {
        @apply flex-wrap flex-row;
    }
    .scroll-1-element {
        flex: 0 0 auto;
    }
    .scroll-2-element {
        flex: 1 0 45%;
        min-height: 30%;
        max-height: 30%;
        box-sizing: border-box;
    }
    ::-webkit-scrollbar {
        @apply w-1;
    }
    ::-webkit-scrollbar-track {
        @apply bg-none;
    }
    ::-webkit-scrollbar-thumb {
        @apply bg-overlay rounded-xl relative;
    }
</style>