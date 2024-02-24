<script lang="ts">
    import type {OptionButton} from "../../scripts/types";
    import {clickOutside} from "../../scripts/click-outside";

    export let buttons: OptionButton[]

    let compact: boolean = true
    let dropdownVisible: boolean

    function toggleCompactDropdown() {
        dropdownVisible = !dropdownVisible
    }
    function handleClickOutside() {
        dropdownVisible = false
    }
</script>

{#if compact}
    <button class="relative h-full cursor-pointer text-text" on:click={toggleCompactDropdown} use:clickOutside on:click_outside={handleClickOutside}>
        <i class="fa-solid fa-ellipsis-vertical"></i>
        <div class="relative flex justify-end">
            <div id="dropdown" class="fixed flex flex-col opacity-0 bg-overlay z-10 rounded-lg invisible" class:dropdownVisible={dropdownVisible}>
                {#each buttons as button}
                    <button class="option-button flex justify-center items-center p-2" on:click={button.action}>
                        <h1 class="m-0">{button.label}</h1>
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
        transition: background-color 50ms linear;
    }
    .option-button:hover {
        @apply bg-surface;
    }
    #dropdown {
        transition: visibility 0s, opacity 150ms;
    }
</style>