<script lang="ts">
    import Selection from "../Selection.svelte";
    import type {SelectionOption} from "../../../scripts/types";

    export let title: string
    export let description: string
    export let options: SelectionOption[]
    export let value: SelectionOption
</script>

<div id="selection-content" class="w-full flex flex-row justify-between items-center">
    <div class="flex flex-col gap-1 max-w-[20rem]">
        <h1 id="title" class="text-lg">{title}</h1>
        <h1 id="description">{description}</h1>
    </div>
    <div id="selection-wrapper">
        <Selection class="bg-surface w-[8rem] p-2 rounded" bind:selected={value} title="" options={options} on:select/>
    </div>
</div>