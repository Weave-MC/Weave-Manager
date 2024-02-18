<script lang="ts">
    // this is hardcoded to only be maxed at 2
    export let columns: number = 1;
    export let items: any[];
</script>

<div class="scroll-container">
    <div class="scroll-content" class:scroll-single={columns < 2} class:scroll-multi={columns > 1}>
        {#each items as item}
            <!-- A "hack" to force styles on the scroll list entries -->
            <div class:scroll-1-element={columns < 2} class:scroll-2-element={columns > 1}>
                <slot prop={item}/>
            </div>
        {/each}
    </div>
</div>

<style>
    .scroll-container {
        @apply absolute bg-crust rounded-lg top-10 bottom-2 left-2 right-2 py-2 pl-2 pr-1;
    }
    .scroll-content {
        @apply w-full h-full overflow-y-scroll gap-2 pr-1 flex;
    }
    .scroll-single {
        @apply flex-col;
    }
    .scroll-multi {
        @apply flex-wrap flex-row;
    }
    .scroll-1-element {
        flex: 1 0 auto;
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