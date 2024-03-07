<script lang="ts">
    import {createEventDispatcher} from "svelte";
    import type {SelectionOption} from "../../scripts/types";

    export let title: string
    export let options: SelectionOption[]
    export let selected: SelectionOption

    const dispatch = createEventDispatcher<{select: {value: SelectionOption}}>()

    let open: boolean = false
    function select(option: SelectionOption) {
        selected = option
        open = false
        dispatch("select", {value: option})
    }
</script>

<div class="relative text-text flex flex-col justify-center items-center w-full gap-2">
    <h1>{title}</h1>
    <button id="selection-head" class="{$$props.class}" on:click={() => open = !open}>
        {selected.name}
    </button>
    <div id="selection-body" class="absolute w-full top-[115%] py-2 pl-2 pr-1 {open ? 'visible' : 'hidden'} {$$props.class}">
        <div class="relative flex flex-col overflow-y-scroll pr-1 max-h-[10rem]">
            {#each options as option}
                <button class="option py-1" on:click={() => select(option)}>{option.name}</button>
            {/each}
        </div>
    </div>
</div>

<style>
    .option {
        transition: background-color 50ms;
    }
    .option:hover {
        @apply bg-base;
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