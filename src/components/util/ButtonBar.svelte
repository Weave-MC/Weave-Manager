<script lang="ts">
    import type {OptionButton} from "../../scripts/types";
    import {clickOutside} from "../../scripts/click-outside";
    import {afterUpdate, beforeUpdate, onMount, tick} from "svelte";

    export let buttons: OptionButton[]
    export let name: string

    let dropdown: HTMLElement

    let compact: boolean = true
    let dropdownVisible: boolean

    function toggleCompactDropdown() {
        dropdownVisible = !dropdownVisible
    }
    function handleClickOutside() {
        dropdownVisible = false
    }

    // TODO this is terrible, I have no idea what I should do instead though
    function moveToParent() {
        const parent = dropdown.parentElement
        dropdown.style.top = parent.getBoundingClientRect().top + "px"
    }

    afterUpdate(async () => {
        moveToParent()
    })
</script>

{#if compact}
    <button class="cursor-pointer text-text w-4" on:click={toggleCompactDropdown} use:clickOutside on:click_outside={handleClickOutside}>
        <i class="fa-solid fa-ellipsis-vertical"></i>
        <div bind:this={dropdown} class="fixed flex justify-end z-10">
            <div id="dropdown" class="absolute bg-overlay rounded-lg flex flex-col whitespace-nowrap max-w-[10rem] overflow-clip invisible opacity-0" class:dropdownVisible={dropdownVisible}>
                {#each buttons as button}
                    <button class="option-button flex justify-center items-center p-2" on:click={button.action}>
                        <h1>{button.label}</h1>
                        <slot prop={button}/>
                    </button>
                {/each}
            </div>
        </div>
    </button>
{:else}
    <div class="relative flex flex-row h-full {$$props.class}">
        {#each buttons as button}
            <button class="cozy-button h-full bg-overlay rounded aspect-square" on:click={button.action}>
                <i class={button.icon}></i>
            </button>
            <div class="tooltip absolute p-2 bg-overlay z-10 rounded-lg flex justify-center items-center drop-shadow-md max-w-[10rem] whitespace-nowrap top-[100%] right-0 mt-2">
                <h1>{button.label}</h1>
            </div>
        {/each}
    </div>
{/if}

<style>
    .tooltip {
        visibility: hidden;
        opacity: 0;
        transition: all 50ms;
    }
    .cozy-button:hover + .tooltip {
        opacity: 100%;
        visibility: visible;
    }
    .dropdownVisible {
        visibility: visible;
        opacity: 100%;
    }
    .option-button {
        transition: background-color 25ms linear;
    }
    .option-button:hover {
        @apply bg-surface;
    }
    #dropdown {
        transition: visibility 150s, opacity 150ms;
    }
</style>