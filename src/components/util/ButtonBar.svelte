<script lang="ts">
    import type {OptionButton} from "../../scripts/types";
    import {clickOutside} from "../../scripts/components";
    import {scroll} from "./VerticalScroll.svelte";
    import {settings} from "../../scripts/stores";

    export let buttons: OptionButton[]

    let dropdown: HTMLElement
    let cozyButtons: HTMLElement

    let compact: boolean = $settings.compact_buttons
    let dropdownVisible: boolean

    function toggleCompactDropdown() {
        dropdownVisible = !dropdownVisible
    }
    function handleClickOutside() {
        dropdownVisible = false
    }

    $: if($scroll) onScroll($scroll)
    const onScroll = (e) => {
        if (compact) {
            if (dropdown) {
                const parent = dropdown.parentElement
                if (parent) {
                    dropdown.style.top = parent.getBoundingClientRect().top + "px"
                    dropdownVisible = false
                }
            }
        } else {
            if (cozyButtons) {
                const tooltips = cozyButtons.getElementsByClassName("tooltip")

                for (let i = 0; i < tooltips.length; i++) {
                    const tooltip = tooltips[i] as HTMLElement
                    if (tooltip) {
                        const parent = tooltip.parentElement
                        if (parent) {
                            tooltip.style.top = parent.getBoundingClientRect().top + "px"
                        }
                    }
                }
            }
        }
    }
</script>

{#if compact}
    <button class="cursor-pointer text-text w-4" on:click={toggleCompactDropdown} use:clickOutside on:click_outside={handleClickOutside}>
        <i class="fa-solid fa-ellipsis-vertical"></i>
        <div bind:this={dropdown} class="fixed flex justify-end z-10">
            <div id="dropdown" class="absolute bg-overlay rounded-lg flex flex-col whitespace-nowrap max-w-[10rem] overflow-clip invisible opacity-0" class:dropdown-visible={dropdownVisible}>
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
    <div bind:this={cozyButtons} class="relative flex flex-row h-full {$$props.class}">
        {#each buttons as button}
            <button class="cozy-button h-full bg-overlay rounded aspect-square" on:click={button.action}>
                <i class={button.icon}></i>
            </button>
            <div class="tooltip fixed flex justify-end mt-5 z-10">
                <div class="absolute bg-overlay rounded-lg flex flex-col justify-center items-center drop-shadow-md max-w-[10rem] whitespace-nowrap p-2">
                    <h1>{button.label}</h1>
                </div>
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
    .dropdown-visible {
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
        transition: visibility 150ms, opacity 150ms;
    }
</style>