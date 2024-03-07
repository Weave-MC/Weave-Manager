<script lang="ts">
    import {createEventDispatcher} from "svelte";

    export let enabled: boolean
    export let title: string
    export let description: string

    const dispatch = createEventDispatcher<{toggle: {state: boolean}}>()

    function toggle() {
        enabled = !enabled
        dispatch("toggle", {state: enabled})
    }
</script>

<div id="checkbox-setting" class="w-full flex flex-row justify-between items-center">
    <div class="flex flex-col gap-1 max-w-[26rem]">
        <h1 id="title" class="text-lg">{title}</h1>
        <h1 id="description">{description}</h1>
    </div>

    <button id="checkbox" class="h-8 aspect-square border-overlay border-2 flex justify-center items-center text-xl rounded" on:click={toggle}>
        {#if enabled}
            <i class="fa-solid fa-check"></i>
        {/if}
    </button>
</div>
