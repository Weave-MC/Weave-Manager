<script lang="ts">
    export let title: string
    export let options: string[]
    export let selection: HTMLSelectElement
</script>

<div class="relative text-text flex flex-col justify-center items-center w-full gap-2">
    <label for="selection">{title}</label>
    <select bind:this={selection} name="selection" class={$$props.class}>
        {#each options as option}
            <option value={option}>{option}</option>
        {/each}
    </select>
</div>

<style>
    select {
        border: none;
        box-shadow: none;
    }
    select.decorated option:hover {
        box-shadow: 0 0 10px 100px #1882A8 inset !important;
    }
</style>